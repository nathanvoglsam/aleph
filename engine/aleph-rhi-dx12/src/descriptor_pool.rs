//
//
// This file is a part of Aleph
//
// https://github.com/nathanvoglsam/aleph
//
// MIT License
//
// Copyright (c) 2020 Aleph Engine
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

use std::any::TypeId;
use std::mem::MaybeUninit;
use std::ptr::NonNull;

use aleph_any::{AnyArc, declare_interfaces};
use aleph_rhi_api::*;
use aleph_rhi_impl_utils::parameter_block_pool::ParameterBlockPool;

use crate::descriptor_arena::LinearBlockFactory;
use crate::device::Device;
use crate::internal::descriptor_chunk::DescriptorChunk;
use crate::parameter_block_layout::ParameterBlockLayout;

pub struct DescriptorPool {
    pub(crate) _device: AnyArc<Device>,
    pub(crate) _layout: AnyArc<ParameterBlockLayout>,
    pub(crate) resource_arena: Option<DescriptorChunk>,
    pub(crate) pool: ParameterBlockPool<LinearBlockFactory>,
}

declare_interfaces!(DescriptorPool, [IDescriptorPool]);

impl IGetPlatformInterface for DescriptorPool {
    unsafe fn __query_platform_interface(&self, _target: TypeId, _out: *mut ()) -> Option<()> {
        None
    }
}

impl IDescriptorPool for DescriptorPool {
    fn allocate_block(&mut self) -> Result<ParameterBlockHandle, DescriptorAllocateError> {
        let mut blocks: [MaybeUninit<_>; 1] = [MaybeUninit::uninit(); 1];
        self.pool
            .allocate_blocks((self.resource_arena.as_ref(), &self._layout), &mut blocks)?;

        unsafe {
            let block = blocks[0].assume_init();
            Ok(block)
        }
    }

    fn allocate_blocks(
        &mut self,
        num_blocks: usize,
    ) -> Result<Box<[ParameterBlockHandle]>, DescriptorAllocateError> {
        let mut blocks = Box::new_uninit_slice(num_blocks);
        self.pool
            .allocate_blocks((self.resource_arena.as_ref(), &self._layout), &mut blocks)?;

        let blocks = Box::leak(blocks);
        let blocks = NonNull::from(blocks);
        let blocks =
            NonNull::slice_from_raw_parts(blocks.cast::<ParameterBlockHandle>(), blocks.len());
        unsafe { Ok(Box::from_raw(blocks.as_ptr())) }
    }

    unsafe fn free(&mut self, blocks: &[ParameterBlockHandle]) {
        self.pool.free_blocks(blocks)
    }

    unsafe fn reset(&mut self) {
        unsafe {
            self.pool.reset_pool();
        }
    }
}

impl Drop for DescriptorPool {
    fn drop(&mut self) {
        if let Some(arena) = &self.resource_arena {
            // Safety:
            // It's not possible to use the DescriptorPool, and thus the Arena, again as we're in
            // the drop implementation.
            //
            // We can't prevent user's further up the callstack from trying to use descriptors from
            // the pool (and arena) after calling this. This is reflected in all APIs that use them
            // being unsafe. We still leave preventing user-after-free to the caller.
            unsafe {
                arena.release_allocation_to_heap(self._device.descriptor_heaps.gpu_view_heap());
            }
        }
    }
}
