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

use aleph_rhi_api::*;
use windows::utils::{CPUDescriptorHandle, GPUDescriptorHandle};

use crate::internal::descriptor_heap::{DescriptorAllocation, DescriptorHeap};

/// An internal data-structure used as the backing storage for an object-pool based descriptor pool
/// allocator.
///
/// [DescriptorArena] will manage an allocation and provide utilities for handling a block of memory
/// from an ID3D12DescriptorHeap that has space for 'num_sets' descriptor sets which each contain
/// 'num_descriptors_per_set'.
///
/// This includes indexing logic, virtual address calculation, and calling the underlying heap
/// allocator to get the arena's memory.
pub struct DescriptorArena {
    /// The allocation handle of the descriptor arena
    pub allocation: DescriptorAllocation,

    /// The descriptor increment for the descriptor type this arena allocates for
    pub descriptor_increment: u32,

    /// The total number of descriptor sets this arena has space for
    pub num_sets: u32,

    /// The number of descriptors per set for this arena
    pub num_descriptors_per_set: u32,

    /// The CPU handle to the start of the descriptor arena
    pub cpu_base: CPUDescriptorHandle,

    /// The GPU handle to the start of the descriptor arena
    pub gpu_base: GPUDescriptorHandle,
}

impl DescriptorArena {
    /// Constructs a new arena from the given heap that has enough space for 'num_sets' that have
    /// 'num_descriptors_per_set'
    pub fn new(
        heap: &DescriptorHeap,
        num_sets: u32,
        num_descriptors_per_set: u32,
    ) -> Result<Option<Self>, DescriptorPoolCreateError> {
        if num_sets * num_descriptors_per_set == 0 {
            return Ok(None);
        }

        // The total number of descriptors needed from the heap to store 'num_sets'
        let num_descriptors = num_descriptors_per_set * num_sets;

        let allocation = heap
            .allocate(num_descriptors)
            .ok_or(DescriptorPoolCreateError::OutOfMemory)?;
        let cpu_base = heap.allocation_to_cpu_handle(allocation).unwrap();
        let gpu_base = heap.allocation_to_gpu_handle(allocation).unwrap();

        Ok(Some(DescriptorArena {
            allocation,
            descriptor_increment: heap.descriptor_increment(),
            num_sets,
            num_descriptors_per_set,
            cpu_base,
            gpu_base,
        }))
    }

    /// Gets the CPU and GPU handle for the given set index in this arena.
    pub fn get_handles_for_set_index(
        &self,
        set_index: u32,
    ) -> (CPUDescriptorHandle, GPUDescriptorHandle) {
        debug_assert!(
            set_index < self.num_sets,
            "Requested a set out of the arena's bounds"
        );

        let offset = set_index as usize
            * self.num_descriptors_per_set as usize
            * self.descriptor_increment as usize;
        let cpu = self.cpu_base.add(offset);
        let gpu = self.gpu_base.add(offset as u64);
        (cpu, gpu)
    }

    /// This function is used for returning the underlying descriptor heap allocation back to the
    /// heap.
    ///
    /// # Safety
    ///
    /// The arena doesn't know which heap it was allocated from. It is the caller's responsibility
    /// to ensure that the arena is released to the correct heap.
    ///
    /// It is invalid to use any descriptors from this arena once they are released back to the
    /// heap. It is the caller's responsibility to ensure that all descriptors allocated from this
    /// arena are not used after the arena is released.
    ///
    /// It is invalid to allocate new descriptors from the arena after releasing the backing
    /// allocation. It is the caller's responsibility to ensure that the arena is no longer used
    /// after calling this function.
    pub unsafe fn release_allocation_to_heap(&self, heap: &DescriptorHeap) {
        heap.release(self.allocation);
    }
}
