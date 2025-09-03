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

use std::alloc::{Layout, handle_alloc_error};
use std::any::TypeId;
use std::mem::MaybeUninit;
use std::ptr::NonNull;

use aleph_any::{AnyArc, declare_interfaces};
use aleph_rhi_api::*;
use allocator_api2::alloc::Allocator;
use blink_alloc::BlinkAlloc;
use windows::utils::{CPUDescriptorHandle, GPUDescriptorHandle};

use crate::device::Device;
use crate::internal::descriptor_chunk::DescriptorChunk;
use crate::internal::parameter_block::ParameterBlock;
use crate::internal::parameter_block_pool::ParameterBlockPool;
use crate::parameter_block_layout::ParameterBlockLayout;

pub struct DescriptorPool {
    pub(crate) _device: AnyArc<Device>,
    pub(crate) _layout: AnyArc<ParameterBlockLayout>,

    /// The base address of the arena this pool allocates resource descriptors from
    pub(crate) resource_arena: Option<DescriptorChunk>,

    /// Object pool allocator that descriptor set objects are allocated from.
    pub(crate) set_pool: ParameterBlockPool,

    /// A bump arena used to allocate the backing buffers for the sampler and dynamic cb arrays
    /// inside the set objects.
    pub(crate) set_array_pool: BlinkAlloc,

    /// The bump state for the descriptor pool. Used to bump allocate descriptor blocks from the
    /// resource arena.
    pub(crate) descriptor_bump_index: u32,
}

declare_interfaces!(DescriptorPool, [IDescriptorPool]);

impl IGetPlatformInterface for DescriptorPool {
    unsafe fn __query_platform_interface(&self, _target: TypeId, _out: *mut ()) -> Option<()> {
        None
    }
}

impl DescriptorPool {
    fn get_optional_handles_for_index(
        &self,
        index: u32,
    ) -> (Option<CPUDescriptorHandle>, Option<GPUDescriptorHandle>) {
        if let Some(arena) = self.resource_arena.as_ref() {
            let (cpu, gpu) = arena.get_handles_for_index(index);
            (Some(cpu), Some(gpu))
        } else {
            (None, None)
        }
    }
}

impl IDescriptorPool for DescriptorPool {
    fn allocate_block(&mut self) -> Result<ParameterBlockHandle, DescriptorAllocateError> {
        let mut set = MaybeUninit::uninit();
        let num_from_free_list = self
            .set_pool
            .allocate_blocks(std::slice::from_mut(&mut set))
            .ok_or(DescriptorAllocateError::OutOfPoolMemory)?;

        // Safety: allocate_sets is requried to intialize this so this is safe
        let set = unsafe { set.assume_init() };

        // If we got a set from the free list then we can just return it immediately without
        // allocating the internal arrays as they're already allocated.
        if num_from_free_list == 1 {
            return Ok(set);
        }

        // Increment
        let set_index = self.descriptor_bump_index;
        self.descriptor_bump_index += 1;

        unsafe {
            // Set pool is required to have an initialized object
            let set_ptr = ParameterBlock::ptr_from_handle(set).as_mut();

            set_ptr._layout = NonNull::from(self._layout.as_ref());

            let num_samplers = self._layout.compiled.samplers.num_samplers() as usize;
            let n = num_samplers;
            if n != 0 {
                let layout = Layout::array::<Option<GPUDescriptorHandle>>(n).unwrap();
                let result = self.set_array_pool.allocate_zeroed(layout);
                let result = match result {
                    Ok(v) => v,
                    Err(_) => handle_alloc_error(layout),
                };
                let slice = std::slice::from_raw_parts(
                    result.cast::<Option<GPUDescriptorHandle>>().as_ptr(),
                    n,
                );
                set_ptr.samplers = NonNull::from(slice);
            }

            let (resource_handle_cpu, resource_handle_gpu) =
                self.get_optional_handles_for_index(set_index);
            set_ptr.resource_handle_cpu = resource_handle_cpu;
            set_ptr.resource_handle_gpu = resource_handle_gpu;
        }

        Ok(set)
    }

    fn allocate_blocks(
        &mut self,
        num_sets: usize,
    ) -> Result<Box<[ParameterBlockHandle]>, DescriptorAllocateError> {
        let mut sets = vec![MaybeUninit::uninit(); num_sets];
        let num_from_free_list = self
            .set_pool
            .allocate_blocks(&mut sets)
            .ok_or(DescriptorAllocateError::OutOfPoolMemory)?;

        let num_samplers = self._layout.compiled.samplers.num_samplers() as usize;

        unsafe {
            let uninitialized_sets = &mut sets[num_from_free_list..];

            let n = num_samplers * uninitialized_sets.len();
            let samplers = if n != 0 {
                let layout = Layout::array::<Option<GPUDescriptorHandle>>(n).unwrap();
                let result = self.set_array_pool.allocate_zeroed(layout);
                let result = match result {
                    Ok(v) => v,
                    Err(_) => handle_alloc_error(layout),
                };
                std::slice::from_raw_parts(result.cast::<Option<GPUDescriptorHandle>>().as_ptr(), n)
            } else {
                &[]
            };

            for (i, v) in uninitialized_sets.iter_mut().enumerate() {
                let v = v.assume_init();
                let v = ParameterBlock::ptr_from_handle(v).as_mut();
                v._layout = NonNull::from(self._layout.as_ref());

                if num_samplers != 0 {
                    let start = i * num_samplers;
                    let end = start + num_samplers;
                    let samplers = &samplers[start..end];
                    v.samplers = NonNull::from(samplers);
                }
            }
        }

        debug_assert_eq!(sets.len(), sets.capacity());
        debug_assert_eq!(sets.len(), num_sets);
        unsafe { Ok(std::mem::transmute(sets.into_boxed_slice())) }
    }

    unsafe fn free(&mut self, sets: &[ParameterBlockHandle]) {
        self.set_pool.free_blocks(sets);
    }

    unsafe fn reset(&mut self) {
        unsafe {
            self.set_pool.reset_pool();
            self.set_array_pool.reset_unchecked();
            self.descriptor_bump_index = 0;
        }
    }
}

impl Drop for DescriptorPool {
    fn drop(&mut self) {
        if let Some(arena) = self.resource_arena.as_ref() {
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
