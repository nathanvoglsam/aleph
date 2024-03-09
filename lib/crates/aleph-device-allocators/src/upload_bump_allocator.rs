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

use crate::{AllocationResult, BumpAllocator, IUploadAllocator, RawDeviceAllocationResult};

pub struct UploadBumpAllocator {
    /// The buffer object we're allocating from.
    buffer: AnyArc<dyn IBuffer>,

    /// The base address in the host's address space of the block inside 'buffer' we're allocating
    /// from.
    base_host_address: NonNull<u8>,

    /// The base offset of the block we're allocating from, used for patching our device offsets for
    /// the block we're allocating from.
    base_device_offset: usize,

    /// The bump allocator state. The allocator's brain.
    state: BumpAllocator,
}

impl IUploadAllocator for UploadBumpAllocator {
    /// Allocate the given number of bytes from the buffer.
    ///
    /// See [BumpAllocator::allocate] for more in-depth information on the algorithm.
    #[inline]
    fn allocate(&self, size: usize) -> RawDeviceAllocationResult {
        let allocation = self.state.allocate(size);
        self.convert_result(allocation)
    }

    /// Allocate the number of bytes from the buffer, accounting for the requested alignment.
    ///
    /// See [BumpAllocator::allocate_aligned] for more in-depth information.
    #[inline]
    fn allocate_aligned(&self, size: usize, align: usize) -> RawDeviceAllocationResult {
        let allocation = self.state.allocate_aligned(size, align);
        debug_assert!(allocation.offset & (align - 1) == 0);
        self.convert_result(allocation)
    }
}

impl UploadBumpAllocator {
    /// Constructs a [UploadBumpAllocator] with the given capacity and name, allocating the buffer
    /// from the provided device.
    pub fn new_uniform_buffer(
        device: &dyn IDevice,
        capacity: usize,
        name: Option<&str>,
    ) -> Option<Self> {
        if let Some(state) = BumpAllocator::new(capacity) {
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
                base_device_offset: 0,
                state,
            })
        } else {
            None
        }
    }

    /// Constructs a new [UploadBumpAllocator] with the given capacity from the description of a
    /// memory block.
    ///
    /// `buffer`: The buffer object this allocator is allocating from
    /// `base_address`: The base address of the buffer in the host's address space (mapped)
    /// `offset`: Offset, in bytes, from the base address where the buffer starts
    /// `capacity`: The size, in bytes, of the block we wish to sub-allocate from
    ///
    /// This function, instead of constructing a new buffer, will instead take the given buffer,
    /// offset and capacity and construct a 'sub-allocator' that will allocate from the given block
    /// of memory.
    ///
    /// # Safety
    ///
    /// It is the caller's responsibility to ensure the following:
    /// - 'base_host_address' is the base address of the 'buffer' as maped into the host address
    ///   space.
    /// - The allocation at 'base_host_address' is valid for `offset + capacity` bytes such that any
    ///   sub-allocations from within the block could never overrun the end of the allocated block.
    pub unsafe fn new_from_block(
        buffer: &dyn IBuffer,
        base_host_address: NonNull<u8>,
        offset: usize,
        capacity: usize,
    ) -> Option<Self> {
        if let Some(state) = BumpAllocator::new(capacity) {
            // Safety: It is the caller's responsibility to ensure that the block starting at
            //         'offset' is good for 'capacity' bytes. If this requirement is met correctly
            //         then the offset can't overflow or escape the bounds of the allocated object.
            let offset_address = base_host_address.as_ptr().add(offset);
            let base_host_address = NonNull::new_unchecked(offset_address);
            Some(Self {
                buffer: buffer.upgrade(),
                base_host_address,
                base_device_offset: offset,
                state,
            })
        } else {
            None
        }
    }

    /// Free all bytes from the bump allocator.
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
        //         point (checked inside BumpAllocator::allocate). Assuming 'base_host_address' is
        //         placed correctly it is thus not possible for this addition to overflow the
        //         allocated object _or_ overflow the pointer.
        let host_address = unsafe {
            let addr = self.base_host_address.as_ptr().add(v.offset);
            NonNull::new_unchecked(addr)
        };

        RawDeviceAllocationResult {
            device_offset: v.offset + self.base_device_offset,
            result: host_address,
            allocated: v.allocated,
        }
    }
}
