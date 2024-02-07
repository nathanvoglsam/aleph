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

use std::num::{NonZeroU64, NonZeroUsize};

use aleph_rhi_impl_utils::offset_allocator;
use aleph_rhi_impl_utils::offset_allocator::OffsetAllocator;
use parking_lot::Mutex;
use windows::core::CanInto;
use windows::utils::{CPUDescriptorHandle, GPUDescriptorHandle};
use windows::Win32::Graphics::Direct3D12::*;

pub struct DescriptorHeap {
    /// The device the heap belongs to
    device: ID3D12Device,

    /// The descriptor heap we're allocating into
    heap: ID3D12DescriptorHeap,

    /// The descriptor type of the descriptor heap
    r#type: D3D12_DESCRIPTOR_HEAP_TYPE,

    /// The increment size for the descriptor type this heap stores
    descriptor_increment: u32,

    /// The descriptor handle for the start of the heap on the CPU
    start_cpu_handle: D3D12_CPU_DESCRIPTOR_HANDLE,

    /// The descriptor handle for the start of the heap on the GPU
    start_gpu_handle: D3D12_GPU_DESCRIPTOR_HANDLE,

    /// The mutable state wrapped in a mutex for safe multi-threaded access
    state: Mutex<OffsetAllocator>,
}

impl DescriptorHeap {
    /// Creates a new [DescriptorHeap] based on the provided settings
    pub fn new(
        device: &impl CanInto<ID3D12Device>,
        r#type: D3D12_DESCRIPTOR_HEAP_TYPE,
        num_descriptors: u32,
        gpu_visible: bool,
    ) -> windows::core::Result<Self> {
        let device = device.can_into();

        if num_descriptors == 0 {
            // A descriptor heap with 0 size makes no sense
            panic!("Creating a DescriptorHeap with num_descriptors = 0 is an error");
        }

        // Round 'num_descriptors' up to the next multiple of '32' so that it neatly maps and fits
        // with our bitset in 'flags'
        let num_descriptors = match num_descriptors % 32 {
            0 => num_descriptors,
            r => num_descriptors + (32 - r),
        };

        if num_descriptors > 1_073_741_824 {
            // This prevents numerical error in the allocate code which converts a u32 to an i32.
            // This value 'num_descriptors' sets the upper bound of what will undergo the u32->i32
            // conversion. By keeping the upper bound below i32::MAX then the conversion will
            // always be safe.
            //
            // No D3D12 implementation supports this many descriptors in a heap anyway
            panic!("Creating a DescriptorHeap with num_descriptors > 2^30 is an error");
        }

        // Create the actual descriptor heap based on the requested settings
        let heap = unsafe {
            let mut flags = D3D12_DESCRIPTOR_HEAP_FLAGS::default();

            if gpu_visible {
                flags |= D3D12_DESCRIPTOR_HEAP_FLAG_SHADER_VISIBLE
            }

            let desc = D3D12_DESCRIPTOR_HEAP_DESC {
                Type: r#type,
                NumDescriptors: num_descriptors,
                Flags: flags,
                NodeMask: 0,
            };
            device.CreateDescriptorHeap::<ID3D12DescriptorHeap>(&desc)?
        };

        // Query (to cache) the descriptor increment size
        let descriptor_size = unsafe { device.GetDescriptorHandleIncrementSize(r#type) };

        // Query (to cache) the handle for the start of the heap on the CPU
        let start_cpu_handle = unsafe { heap.GetCPUDescriptorHandleForHeapStart() };

        // Query (to cache) the handle for the start of the heap on the GPU
        let start_gpu_handle = unsafe {
            if gpu_visible {
                heap.GetGPUDescriptorHandleForHeapStart()
            } else {
                D3D12_GPU_DESCRIPTOR_HANDLE::default()
            }
        };

        let state = OffsetAllocator::new(num_descriptors, OffsetAllocator::DEFAULT_MAX_ALLOCS);
        let out = Self {
            device: device.clone(),
            heap,
            r#type,
            descriptor_increment: descriptor_size,
            start_cpu_handle,
            start_gpu_handle,
            state: Mutex::new(state),
        };
        Ok(out)
    }

    /// Allocates a contiguous range of 'num_descriptors'
    #[allow(unused)]
    pub fn allocate(&self, num_descriptors: u32) -> Option<DescriptorAllocation> {
        if num_descriptors == 0 {
            return None;
        }

        let allocation = self.state.lock().allocate(num_descriptors);
        if !allocation.is_fail() {
            Some(DescriptorAllocation(allocation))
        } else {
            None
        }
    }

    /// Release 'num_descriptors' descriptors, starting from 'id'
    #[allow(unused)]
    pub fn release(&self, allocation: DescriptorAllocation) {
        self.state.lock().free(allocation.0)
    }

    /// Converts the given 'id' into the [CPUDescriptorHandle] it represents
    #[allow(unused)]
    pub fn allocation_to_cpu_handle(&self, id: DescriptorAllocation) -> Option<CPUDescriptorHandle> {
        let size = self.descriptor_increment as usize;
        let id = id.0.offset as usize;
        let base = self.start_cpu_handle.ptr;
        let ptr = base + (id * size);
        NonZeroUsize::new(ptr).map(CPUDescriptorHandle::from)
    }

    /// Converts the given 'id' into the [GPUDescriptorHandle] it represents
    #[allow(unused)]
    pub fn allocation_to_gpu_handle(&self, id: DescriptorAllocation) -> Option<GPUDescriptorHandle> {
        let size = self.descriptor_increment as u64;
        let id = id.0.offset as u64;
        let base = self.start_gpu_handle.ptr;
        let ptr = base + (id * size);
        NonZeroU64::new(ptr).map(GPUDescriptorHandle::from)
    }

    /// Returns the descriptor increment of the descriptor type this heap allocates
    #[allow(unused)]
    pub const fn descriptor_increment(&self) -> u32 {
        self.descriptor_increment
    }

    /// Returns the [ID3D12DescriptorHeap] that this object encapsulates
    #[allow(unused)]
    pub const fn heap(&self) -> &ID3D12DescriptorHeap {
        &self.heap
    }

    /// Returns the [D3D12_DESCRIPTOR_HEAP_TYPE] that this heap stores descriptors for
    #[allow(unused)]
    pub const fn heap_type(&self) -> D3D12_DESCRIPTOR_HEAP_TYPE {
        self.r#type
    }

    /// Returns the [ID3D12Device] that this object wraps the heap for
    #[allow(unused)]
    pub const fn device(&self) -> &ID3D12Device {
        &self.device
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(transparent)]
pub struct DescriptorAllocation(offset_allocator::Allocation);
