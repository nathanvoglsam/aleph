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

use std::ptr::NonNull;

use aleph_interfaces::any::AnyArc;
use aleph_rhi_api::*;

use crate::{
    AllocationResult, IUploadAllocator, RawDeviceAllocationResult, RingBuffer, SubAllocatorResult,
    UploadBumpAllocator,
};

/// A wrapper over [RingBuffer] that allows allocating blocks from a device visible uniform buffer.
///
/// This is intended to be used for allocating one-time use uniform buffers. The buffer will be
/// allocated from an upload heap, so will exist as host-coherent device visible memory. (i.e) Host
/// memory mapped into the device's address space. Uncached device reads will hit host memory so
/// it is expected that these buffers are only read once (into cache) and then never again.
///
/// Uniform buffers that will be read multiple times throughout the frame (i.e in different passes)
/// should _not_ be allocated from a ring buffer, and instead should go through the standard staging
/// buffer route to ensure we don't generate excess traffic to host memory.
///
/// This utility works because the first access of a uniform buffer in an upload heap will pull it
/// into cache (GL2 on AMD) and future reads will be served from cache until the pages get evicted.
/// If the buffer is only accessed by a single draw or collection of draws then it's very likely
/// to stay in cache and never be evicted until after the final use. This would mean we only hit
/// host memory once for the initial 'upload' meaning we get the same access performance as a staged
/// uniform buffer.
///
/// If buffers get evicted from cache we start losing performance so be aware.
pub struct UploadRingBuffer {
    buffer: AnyArc<dyn IBuffer>,
    base_host_address: NonNull<u8>,
    state: RingBuffer,
}

impl IUploadAllocator for UploadRingBuffer {
    /// Allocate the given number of bytes from the ring buffer.
    ///
    /// See [RingBuffer::allocate] for more in-depth information on the algorithm.
    #[inline]
    fn allocate(&self, size: usize) -> RawDeviceAllocationResult {
        let allocation = self.state.allocate(size);
        self.convert_result(allocation)
    }

    /// Allocate the number of bytes from the ring buffer, accounting for the requested alignment.
    ///
    /// See [RingBuffer::allocate_aligned] for more in-depth information.
    #[inline]
    fn allocate_aligned(&self, size: usize, align: usize) -> RawDeviceAllocationResult {
        let allocation = self.state.allocate_aligned(size, align);
        debug_assert!(allocation.offset & (align - 1) == 0);
        self.convert_result(allocation)
    }
}

impl UploadRingBuffer {
    /// Constructs a [UploadRingBuffer] with the given capacity and name, allocating the buffer
    /// from the provided device.
    pub fn new_uniform_buffer(
        device: &dyn IDevice,
        capacity: usize,
        name: Option<&str>,
    ) -> Option<Self> {
        if let Some(state) = RingBuffer::new(capacity) {
            let buffer = device
                .create_buffer(&BufferDesc {
                    size: capacity as u64,
                    cpu_access: CpuAccessMode::Write,
                    usage: ResourceUsageFlags::CONSTANT_BUFFER,
                    name,
                })
                .ok()?;
            let base_host_address = buffer.map().ok()?;
            Some(Self {
                buffer,
                base_host_address,
                state,
            })
        } else {
            None
        }
    }

    /// A utility for creating a bump allocator backed by a region of the ring buffer. Useful for
    /// creating a sub-allocator from the ring buffer that can be sent to other threads for parallel
    /// command recording.
    pub fn allocate_aligned_bump_allocator(
        &self,
        size: usize,
        align: usize,
    ) -> SubAllocatorResult<UploadBumpAllocator> {
        let allocation = self.state.allocate_aligned(size, align);
        debug_assert!(allocation.offset & (align - 1) == 0);

        // Safety: all preconditions are met implicitly here. The allocate functions can't give us
        //         a bad region and the ring buffer is already guaranteed to have a valid base
        //         block.
        let allocator = unsafe {
            UploadBumpAllocator::new_from_block(
                self.buffer.as_ref(),
                self.base_host_address,
                allocation.offset,
                size,
            )
            .unwrap()
        };

        SubAllocatorResult {
            allocator,
            allocated: allocation.allocated,
        }
    }

    /// Free the given number of bytes from the ring buffer.
    ///
    /// # Safety
    ///
    /// It is the caller's responsibility to ensure that the bytes being freed are not in use both
    /// on the host and on the device.
    #[inline]
    pub unsafe fn free(&mut self, size: usize) {
        self.state.free(size)
    }

    /// Free all bytes from the ring buffer, leaving the head in place.
    ///
    /// # Safety
    ///
    /// It is the caller's responsibility to ensure that the bytes being freed are not in use both
    /// on the host and on the device.
    #[inline]
    pub unsafe fn clear(&mut self) {
        self.state.clear()
    }

    /// The total capacity the bump allocator can allocate for
    pub const fn capacity(&self) -> usize {
        self.state.capacity()
    }

    /// The current number of bytes allocated from the allocator
    #[inline]
    pub fn size(&self) -> usize {
        self.state.size()
    }

    /// The number of bytes remaining that can still be allocated from the allocator
    #[inline]
    pub fn size_remaining(&self) -> usize {
        self.state.size_remaining()
    }

    /// Get the buffer that this is allocating from
    #[inline]
    pub fn buffer(&self) -> &dyn IBuffer {
        self.buffer.as_ref()
    }

    /// Internal function for convertin an allocation result to our own [RawDeviceAllocationResult]
    #[inline]
    fn convert_result(&self, v: AllocationResult) -> RawDeviceAllocationResult {
        // Safety: This is safe because 'size' is guaranteed to be less than 'isize::MAX' at this
        //         point (checked inside RingBuffer::allocate). Assuming 'base_host_address' is
        //         placed correctly it is thus not possible for this addition to overflow the
        //         allocated object _or_ overflow the pointer.
        let host_address = unsafe {
            let addr = self.base_host_address.as_ptr().add(v.offset);
            NonNull::new_unchecked(addr)
        };

        RawDeviceAllocationResult {
            device_offset: v.offset,
            result: host_address,
            allocated: v.allocated,
        }
    }
}
