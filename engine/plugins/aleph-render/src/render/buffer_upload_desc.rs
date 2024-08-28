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

use aleph_rhi_api::*;
use interfaces::any::AnyArc;

/// A data source for a buffer upload request. Represents an annotated block of upload memory that
/// pairs the memory block with a description of the buffer that it contains.
///
/// This can be combined in the upload manager with a target buffer to create an upload request.
///
/// This struct abstracts access to a chunk of upload memory so that it can be safely used to upload
/// buffer data to the GPU.
///
/// # Performance Warning
///
/// It is likely for the underlying upload memory to be mapped as write-combined or uncached. This
/// will make reads from the upload memory very expensive as well as make random writes expensive.
///
/// It is highly recommended to only write to this memory once, sequentially.
pub struct BufferUploadSource {
    /// A tracking handle for the buffer that the upload data is going to be stored in. This may be
    /// an owned buffer (the request is the only holder of the refcount) or a shared buffer where
    /// the data is a suballocated region from this buffer.
    ///
    /// Importantly the underlying mapped buffer memory will never be accessed through this
    /// reference. This is simply stored to keep the buffer alive.
    pub(crate) buffer: AnyArc<dyn IBuffer>,

    /// An offset in bytes from the start of the buffer for where the upload data starts. This
    /// is needed as the upload commands use an offset+len and not a raw ptr+len pair.
    pub(crate) offset: u64,

    /// A pointer to a slice of memory for where the upload data should be written into for the
    /// specific image mip that is being uploaded by this request. This will be appropriately sized
    /// and aligned for the texture's upload parameters.
    ///
    /// This memory is considered owned by this [TextureUploadSource] object but must be stored as
    /// a pointer as a reference would be self-referential. It is up to the upload system to ensure
    /// that we never hand out the same region to multiple upload requests.
    ///
    /// This slice will be mapped in an upload heap which is likely to be write-combined or uncached
    /// memory. Reads to this region may be very expensive, and random writes may also be slow.
    pub(crate) data: NonNull<[u8]>,
}

impl BufferUploadSource {
    /// Constructs a new upload source from the given parameters. Includes some debug validation to
    /// try and detect mistakes.
    ///
    /// This function is intended to be used when sub-allocating upload payloads from a larger
    /// staging buffer allocation.
    ///
    /// # Safety
    ///
    /// There are a bunch of requirements for safely implementing this system.
    ///
    /// - 'data' takes ownership of the underlying buffer
    /// - 'data' must point to memory in the mapped range of 'buffer'
    /// - 'data' must be sized for the texture described by 'desc'
    /// - 'offset' must be aligned to 512 bytes within the buffer
    /// - 'data.len()' combined with 'offset' must not overrun the end of the buffer
    /// - 'desc.width', 'desc.height', and 'desc.depth' must all be at least 1. No zero-sized
    ///   textures.
    ///
    /// There are a bunch of debug asserts for these which are only enabled on debug builds, check
    /// those to see all the requirements. Do not violate these requirements as they will not be
    /// checked in a release build.
    pub unsafe fn new(buffer: AnyArc<dyn IBuffer>, offset: u64, data: NonNull<[u8]>) -> Self {
        #[cfg(debug_assertions)]
        {
            debug_assert!(data.len() > 0, "len must be > 0");

            let buffer_desc = buffer.desc_ref();
            debug_assert!(
                buffer_desc.cpu_access == CpuAccessMode::Write,
                "'data' must be from an upload heap"
            );

            debug_assert!(
                buffer_desc.size >= offset,
                "'offset' ({}) is outside of the buffer (size {})",
                offset,
                buffer_desc.size
            );

            let bytes_after_offset = buffer_desc.size - offset;
            debug_assert!(
                bytes_after_offset >= data.len() as u64,
                "'buffer' is too small. [{}+{}] overruns the upload buffer (size {})",
                offset,
                data.len(),
                buffer_desc.size
            );
        }

        Self {
            buffer,
            offset,
            data,
        }
    }

    /// Constructs a new owned [BufferUploadSource] for the given texture upload description.
    ///
    /// # Safety
    ///
    /// See [BufferUploadSource::new] for more information.
    ///
    /// This utility is safer to use than [BufferUploadSource::new] as it guarantees all the buffer
    /// size and ownership constraints by construction. The only relevant requirements are to
    /// ensure the [TextureMipUploadDesc] encodes a valid non-zero-sized texture. That is:
    ///
    /// - 'desc.width', 'desc.height', and 'desc.depth' must all be at least 1. No zero-sized
    ///   textures.
    pub unsafe fn new_owned(device: &dyn IDevice, len: usize) -> Result<Self, BufferCreateError> {
        let buffer = device.create_buffer(&BufferDesc {
            size: len as u64,
            cpu_access: CpuAccessMode::Write,
            usage: ResourceUsageFlags::COPY_SOURCE,
            name: None,
        })?;

        let ptr = buffer.map().unwrap();

        let out = Self::new(buffer, 0, NonNull::slice_from_raw_parts(ptr, len));
        Ok(out)
    }

    /// Creates a [`BufferCopyRegion`] that describes a copy out of the staging buffer into some
    /// destination buffer starting at `dst_offset`.
    ///
    /// This will encode a copy of all bytes in the source buffer region (`self.offset` +
    /// `self.data.len()`) into the destination at the given offset.
    pub const fn get_copy_region(&self, dst_offset: u64) -> BufferCopyRegion {
        BufferCopyRegion {
            src_offset: self.offset,
            dst_offset,
            size: self.data.len() as u64,
        }
    }

    /// Calls [`IBuffer::unmap`] the internal buffer.
    ///
    /// # Safety
    ///
    /// It is the caller's responsibility to ensure this even makes sense to call.
    /// A [`BufferUploadSource`] is not guaranteed to be the 'owner' of the buffer. The internal
    /// buffer could be shared between multiple upload source instances as a staging sub-allocation
    /// scheme and so unmapping this buffer could unmap the address range underneath other callers.
    ///
    /// In general it's only correct to call this on [`BufferUploadSource`] instances that were
    /// constructed via the [`BufferUploadSource::new_owned`] function.
    #[inline(always)]
    pub unsafe fn unmap(&self) {
        self.buffer.unmap();
    }

    /// Returns a handle to the [`IBuffer`] object that backs our staging buffer.
    ///
    /// # Warning
    ///
    /// Do not attempt to map or access the buffer's mapped memory. An upload source is considered
    /// a mutable borrow to some region of that mapped buffer, so you're almost certainly going to
    /// cause UB via mutable aliasing.
    ///
    /// It does require an unsafe block to do this, but it's not obvious you can't do this
    /// specifically because of the abstraction this upload source provides
    #[inline(always)]
    pub fn buffer(&self) -> &dyn IBuffer {
        self.buffer.as_ref()
    }

    /// Get the upload block as a raw pointer.
    ///
    /// # Performance Warning
    ///
    /// The upload memory may be write-combined or uncached memory as it will be mapped for upload
    /// to the GPU. Reads should be treated as *very* expensive for these mapped regions.
    ///
    /// It is recommended to use write-only accessors to prevent accidental reads. It is also
    /// heavily recommended to only perform sequential writes to these regions.
    pub const fn data_ptr(&self) -> NonNull<[u8]> {
        self.data
    }

    /// Get the upload block as a slice.
    ///
    /// # Performance Warning
    ///
    /// The upload memory may be write-combined or uncached memory as it will be mapped for upload
    /// to the GPU. Reads should be treated as *very* expensive for these mapped regions.
    ///
    /// It is recommended to use write-only accessors to prevent accidental reads. It is also
    /// heavily recommended to only perform sequential writes to these regions.
    ///
    /// This is provided as it is technically valid usage but should be handled with care. Avoid
    /// reading from this slice.
    #[inline(always)]
    pub fn data_mut(&mut self) -> &mut [u8] {
        // Safety: It is guaranteed by the implementation that this should be uniquely owned by the
        //         request and valid for access as long as the upload request object is available.
        unsafe { self.data.as_mut() }
    }
}
