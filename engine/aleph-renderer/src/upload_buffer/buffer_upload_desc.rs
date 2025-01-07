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

use std::num::NonZero;
use std::ptr::NonNull;

use aleph_device_allocators::{IUploadAllocator, UploadBumpAllocator};
use aleph_rhi_api::*;
use smallbox::space::S8;
use smallbox::{smallbox, SmallBox};

use crate::{IUploadBuffer, SharedUploadBuffer};

/// This struct describes a buffer object for storage in a buffer pool. This is a simplifcation
/// over [`BufferDesc`] that omits various options.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct BufferObjectDesc {
    /// The size of the buffer in bytes. This can _not_ be zero, see [`BufferObjectDesc`] for
    /// info.
    pub size: NonZero<u64>,

    /// The set of supported usage flags the resource should be created with
    pub usage: ResourceUsageFlags,
}

impl BufferObjectDesc {
    pub const fn new() -> Self {
        Self {
            size: NonZero::new(1).unwrap(),
            usage: ResourceUsageFlags::empty(),
        }
    }

    pub const fn size(&mut self, size: u64) -> &mut Self {
        self.size = NonZero::new(size).unwrap();
        self
    }

    pub const fn usage(&mut self, usage: ResourceUsageFlags) -> &mut Self {
        self.usage = usage;
        self
    }
}

pub struct BufferUploadDesc {
    /// The buffer that our buffer data has been populated in ready for upload to the GPU.
    pub buffer: SmallBox<dyn IUploadBuffer, S8>,

    /// Offset into [`Self::buffer`] for where the buffer data starts.
    pub offset: usize,
}

impl BufferUploadDesc {
    /// Constructs a new owned [`BufferUploadDesc`] for the given buffer upload description.
    pub fn new_owned(
        device: &dyn IDevice,
        desc: &BufferObjectDesc,
    ) -> Result<Self, BufferCreateError> {
        let buffer = unsafe {
            let buffer = device.create_buffer(&BufferDesc {
                size: desc.size.get(),
                cpu_access: CpuAccessMode::Write,
                usage: ResourceUsageFlags::COPY_SOURCE,
                name: None,
            })?;

            let ptr = buffer.map().unwrap();
            let data = NonNull::slice_from_raw_parts(ptr, desc.size.get() as usize);
            SharedUploadBuffer::new(buffer, 0, data)
        };

        let out = Self {
            buffer: smallbox!(buffer),
            offset: 0,
        };
        Ok(out)
    }

    /// Constructs a new [`BufferUploadDesc`] for the given buffer upload description by allocating
    /// a block from the given bump arena.
    pub fn new_in_bump_arena(
        bump: &UploadBumpAllocator,
        desc: &BufferObjectDesc,
    ) -> Result<Self, BufferCreateError> {
        assert!(bump.usage().contains(ResourceUsageFlags::COPY_SOURCE));

        let buffer = unsafe {
            let block = bump
                .allocate_aligned(desc.size.get() as usize, 512)
                .ok_or(BufferCreateError::OutOfMemory)?;
            let data = NonNull::slice_from_raw_parts(block.result, desc.size.get() as usize);
            SharedUploadBuffer::new(bump.buffer().upgrade(), block.device_offset as u64, data)
        };

        let out = Self {
            buffer: smallbox!(buffer),
            offset: 0,
        };
        Ok(out)
    }

    pub(crate) fn get_copy_region(
        &self,
        desc: &BufferObjectDesc,
        dst_offset: u64,
    ) -> BufferCopyRegion {
        BufferCopyRegion {
            src_offset: self.buffer.device_offset() + self.offset as u64,
            dst_offset,
            size: desc.size.get(),
        }
    }
}
