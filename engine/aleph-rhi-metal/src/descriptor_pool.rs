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
use std::sync::Arc;

use aleph_alloc::instrumentation::system;
use aleph_rhi_api::*;
use aleph_rhi_impl_utils::parameter_block_pool::ParameterBlockPool;
use blink_alloc::BlinkAlloc;

use crate::descriptor_arena::LinearBlockFactory;
use crate::device::Device;
use crate::internal::memory_block::MemoryBlock;
use crate::internal::unwrap;
use crate::parameter_block_layout::ParameterBlockLayout;

pub struct DescriptorPool {
    pub(crate) _device: Arc<Device>,
    pub(crate) _layout: Arc<ParameterBlockLayout>,
    pub(crate) block: MemoryBlock,
    pub(crate) pool: ParameterBlockPool<LinearBlockFactory>,
}

impl IGetPlatformInterface for DescriptorPool {
    unsafe fn __query_platform_interface(&self, _target: TypeId, _out: *mut ()) -> Option<()> {
        None
    }
}

impl IDescriptorPool for DescriptorPool {
    fn allocate_block(&mut self) -> Result<ParameterBlockHandle, DescriptorAllocateError> {
        let mut blocks: [MaybeUninit<_>; 1] = [MaybeUninit::uninit(); 1];
        self.pool
            .allocate_blocks((&self.block, &self._layout), &mut blocks)?;

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
            .allocate_blocks((&self.block, &self._layout), &mut blocks)?;

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

impl DescriptorPool {
    pub(crate) fn create(
        device: &Device,
        desc: &DescriptorPoolDesc,
    ) -> Result<Box<dyn IDescriptorPool>, DescriptorPoolCreateError> {
        let layout = unwrap::parameter_block_layout(desc.layout);

        let len = layout.compiled.num_arguments * desc.num_blocks as usize;
        let block = MemoryBlock::new(device, len).ok_or(DescriptorPoolCreateError::Platform)?;

        let factory = LinearBlockFactory {
            next_resource_index: 0,
            arena: BlinkAlloc::new_in(system()),
        };
        let pool = ParameterBlockPool::new(factory, desc.num_blocks as usize);

        let pool: Box<dyn IDescriptorPool> = Box::new(DescriptorPool {
            _device: device.this.upgrade().unwrap(),
            _layout: layout.this.upgrade().unwrap(),
            block,
            pool,
        });

        Ok(pool)
    }
}
