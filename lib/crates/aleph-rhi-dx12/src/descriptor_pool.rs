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

use aleph_any::{declare_interfaces, AnyArc};
use aleph_rhi_api::*;
use bumpalo::Bump;
use windows::utils::{CPUDescriptorHandle, GPUDescriptorHandle};

use crate::descriptor_set_layout::DescriptorSetLayout;
use crate::device::Device;
use crate::internal::descriptor_chunk::DescriptorChunk;
use crate::internal::descriptor_set::DescriptorSet;

pub struct DescriptorPool {
    pub(crate) _device: AnyArc<Device>,
    pub(crate) _layout: AnyArc<DescriptorSetLayout>,

    /// The base address of the arena this pool allocates resource descriptors from
    pub(crate) resource_arena: Option<DescriptorChunk>,

    /// Bump allocator that descriptor set objects are allocated from
    pub(crate) descriptor_set_pool: Bump,

    /// The bump state for the descriptor pool. Used to bump allocate descriptor blocks from the
    /// resource arena.
    pub(crate) descriptor_bump_index: u32,

    /// The maximum number of descriptor sets that can be allocated from the pool
    pub(crate) set_capacity: u32,

    /// List of free handles
    pub(crate) free_list: Vec<DescriptorSetHandle>,
}

declare_interfaces!(DescriptorPool, [IDescriptorPool]);

impl IGetPlatformInterface for DescriptorPool {
    unsafe fn __query_platform_interface(&self, _target: TypeId, _out: *mut ()) -> Option<()> {
        None
    }
}

impl DescriptorPool {
    /// Checks if there is space to allocate a new set in the descriptor pool.
    ///
    /// # Warning
    ///
    /// This function assumes it is being called immediately prior to trying to allocate a set. As
    /// such it returns an OOM error instead of a simple bool.
    fn check_oom(&self) -> Result<(), DescriptorPoolAllocateError> {
        if self.free_list.len() == self.set_capacity as usize {
            Err(DescriptorPoolAllocateError::OutOfMemory)
        } else {
            Ok(())
        }
    }

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
    fn allocate_set(&mut self) -> Result<DescriptorSetHandle, DescriptorPoolAllocateError> {
        // First try and grab something from the free list
        if let Some(handle) = self.free_list.pop() {
            return Ok(handle);
        }

        // We don't need to check OOM unless we're trying to allocate a new set object
        self.check_oom()?;

        // Increment
        let set_index = self.descriptor_bump_index;
        self.descriptor_bump_index += 1;

        let (resource_handle_cpu, resource_handle_gpu) =
            self.get_optional_handles_for_index(set_index);

        let handle = {
            DescriptorSet::heap_allocate(
                &&self.descriptor_set_pool,
                self._layout.as_ref(),
                self._layout.dynamic_constant_buffers.len(),
                self._layout.sampler_tables.len(),
                resource_handle_cpu,
                resource_handle_gpu,
            )
        };

        Ok(handle)
    }

    fn allocate_sets(
        &mut self,
        num_sets: usize,
    ) -> Result<Vec<DescriptorSetHandle>, DescriptorPoolAllocateError> {
        let mut sets = Vec::with_capacity(num_sets);
        for _ in 0..num_sets {
            sets.push(self.allocate_set()?);
        }
        Ok(sets)
    }

    unsafe fn free(&mut self, sets: &[DescriptorSetHandle]) {
        for set in sets {
            self.free_list.push(set.clone());
        }
    }

    unsafe fn reset(&mut self) {
        self.free_list.clear();
        self.descriptor_set_pool.reset();
        self.descriptor_bump_index = 0;
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
