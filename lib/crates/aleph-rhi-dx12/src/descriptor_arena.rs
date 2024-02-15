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
use std::cell::Cell;

use aleph_any::{declare_interfaces, AnyArc};
use aleph_rhi_api::*;
use bumpalo::Bump;
use windows::utils::{CPUDescriptorHandle, GPUDescriptorHandle};

use crate::descriptor_set_layout::DescriptorSetLayout;
use crate::device::Device;
use crate::internal::descriptor_chunk::DescriptorChunk;
use crate::internal::descriptor_set::DescriptorSet;
use crate::internal::unwrap;

pub struct DescriptorArena {
    pub(crate) _device: AnyArc<Device>,

    /// The base address of the arena this pool allocates resource descriptors from
    pub(crate) resource_arena: DescriptorChunk,

    /// Bump allocator that descriptor set objects are allocated from
    pub(crate) descriptor_set_pool: Cell<Option<Bump>>,

    /// The bump state for the descriptor pool. Used to bump allocate descriptor blocks from the
    /// resource arena.
    pub(crate) descriptor_bump_index: Cell<u32>,

    /// The number of descriptor set objects currently allocated from the arena.
    pub(crate) num_sets: Cell<u32>,

    /// The maximum number of descriptor sets that can be allocated from the pool
    pub(crate) set_capacity: u32,
}

declare_interfaces!(DescriptorArena, [IDescriptorArena]);

impl IGetPlatformInterface for DescriptorArena {
    unsafe fn __query_platform_interface(&self, _target: TypeId, _out: *mut ()) -> Option<()> {
        None
    }
}

impl DescriptorArena {
    fn get_optional_handles_for_index(
        &self,
        layout: &DescriptorSetLayout,
        index: u32,
    ) -> (Option<CPUDescriptorHandle>, Option<GPUDescriptorHandle>) {
        if layout.resource_num != 0 {
            let (cpu, gpu) = self.resource_arena.get_handles_for_index(index);
            (Some(cpu), Some(gpu))
        } else {
            (None, None)
        }
    }
}

impl IDescriptorArena for DescriptorArena {
    fn allocate_set(
        &self,
        layout: &dyn IDescriptorSetLayout,
    ) -> Result<DescriptorSetHandle, DescriptorPoolAllocateError> {
        let layout = unwrap::descriptor_set_layout(layout);
        self.allocate_set_internal(layout)
    }

    fn allocate_sets(
        &self,
        layout: &dyn IDescriptorSetLayout,
        num_sets: usize,
    ) -> Result<Vec<DescriptorSetHandle>, DescriptorPoolAllocateError> {
        let layout = unwrap::descriptor_set_layout(layout);
        let mut sets = Vec::with_capacity(num_sets);
        for _ in 0..num_sets {
            sets.push(self.allocate_set_internal(layout)?);
        }
        Ok(sets)
    }

    unsafe fn free(&self, _sets: &[DescriptorSetHandle]) {
        // Intentionally unimplemented
    }

    unsafe fn reset(&self) {
        self.descriptor_bump_index.set(0);
        self.num_sets.set(0);

        let mut pool = self.descriptor_set_pool.take().unwrap();
        pool.reset();
        self.descriptor_set_pool.set(Some(pool));
    }
}

impl DescriptorArena {
    /// Internal version of [IDescriptorArena::allocate_set] that takes an unwrapped set layout
    /// so we don't repeatedly unwrap the same object in a loop when calling
    /// [IDescriptorArena::allocate_sets].
    fn allocate_set_internal(
        &self,
        layout: &DescriptorSetLayout,
    ) -> Result<DescriptorSetHandle, DescriptorPoolAllocateError> {
        if self.num_sets.get() == self.set_capacity {
            return Err(DescriptorPoolAllocateError::OutOfMemory);
        }

        if self.descriptor_bump_index.get() + layout.resource_num
            > self.resource_arena.num_descriptors
        {
            return Err(DescriptorPoolAllocateError::OutOfPoolMemory);
        }

        // Bump allocate the required number of descriptors from the set
        let set_index = self.descriptor_bump_index.get();
        self.descriptor_bump_index
            .set(self.descriptor_bump_index.get() + layout.resource_num);

        let (resource_handle_cpu, resource_handle_gpu) =
            self.get_optional_handles_for_index(layout, set_index);

        let handle = {
            let pool = self.descriptor_set_pool.take().unwrap();
            let handle = DescriptorSet::heap_allocate(
                &&pool,
                layout,
                layout.dynamic_constant_buffers.len(),
                layout.sampler_tables.len(),
                resource_handle_cpu,
                resource_handle_gpu,
            );
            self.descriptor_set_pool.set(Some(pool));
            handle
        };

        Ok(handle)
    }
}

impl Drop for DescriptorArena {
    fn drop(&mut self) {
        // Safety:
        // It's not possible to use the DescriptorArena, and thus the Chunk, again as we're in
        // the drop implementation.
        //
        // We can't prevent user's further up the callstack from trying to use descriptors from
        // the pool (and arena) after calling this. This is reflected in all APIs that use them
        // being unsafe. We still leave preventing user-after-free to the caller.
        unsafe {
            self.resource_arena
                .release_allocation_to_heap(self._device.descriptor_heaps.gpu_view_heap());
        }
    }
}
