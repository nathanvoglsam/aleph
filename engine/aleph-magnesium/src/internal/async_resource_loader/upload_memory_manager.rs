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

use std::cell::RefCell;
use std::ptr::NonNull;

use aleph_alloc::instrumentation::IAllocationCategory;
use aleph_alloc::offset_allocator::{Allocation, OffsetAllocator};

use crate::async_resource_loader::AllocateRangeError;
use crate::internal::async_resource_loader::AsyncResourceLoader;

pub struct UploadMemoryManager {
    /// The upload memory we sub-allocate from.
    pub buffer: rhi::BufferHandle,

    /// The base address of the mapped upload memory block.
    base_addr: MakeSend,

    /// Memory allocator that we use to sub-allocate ranges from 'upload_memory'.
    allocator: RefCell<OffsetAllocator>,
}

impl UploadMemoryManager {
    pub fn new(device: &dyn rhi::IDevice, size: u32) -> Option<Self> {
        let buffer = device
            .create_buffer(&rhi::BufferDesc {
                size: size as u64,
                cpu_access: rhi::CpuAccessMode::Write,
                usage: rhi::ResourceUsageFlags::COPY_SOURCE,
                name: rhi::obj_name_opt!("UploadMemory"),
            })
            .ok()?;

        let allocator = AsyncResourceLoader::with(|| OffsetAllocator::new(size, 512));
        let allocator = RefCell::new(allocator);

        let base_addr = MakeSend(device.map_buffer(&buffer).ok()?);

        Some(Self {
            buffer,
            base_addr,
            allocator,
        })
    }

    pub fn allocate_upload_range(
        &self,
        size: u32,
    ) -> Result<(Allocation, NonNull<[u8]>), AllocateRangeError> {
        // Attempt to create a sub-allocated region from our upload-buffer
        let allocation = self.allocator.borrow_mut().allocate(size);
        if allocation.is_fail() {
            return Err(AllocateRangeError::NotEnoughUploadMemory);
        }

        let data = unsafe { self.base_addr.0.add(allocation.offset as usize) };
        let data = NonNull::slice_from_raw_parts(data, size as usize);

        Ok((allocation, data))
    }

    pub fn allocate_upload_range_aligned(
        &self,
        size: u32,
        align: u32,
    ) -> Result<((Allocation, u32), NonNull<[u8]>), AllocateRangeError> {
        // Attempt to create a sub-allocated region from our upload-buffer
        let (allocation, aligned_offset) =
            self.allocator.borrow_mut().allocate_aligned(size, align);
        if allocation.is_fail() {
            return Err(AllocateRangeError::NotEnoughUploadMemory);
        }

        let data = unsafe { self.base_addr.0.add(aligned_offset as usize) };
        let data = NonNull::slice_from_raw_parts(data, size as usize);

        Ok(((allocation, aligned_offset), data))
    }

    pub fn free_upload_range(&self, allocation: Allocation) {
        self.allocator.borrow_mut().free(allocation);
    }
}

struct MakeSend(NonNull<u8>);
unsafe impl Send for MakeSend {}
