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
use interfaces::any::AnyArc;
use std::ptr::NonNull;

/// This describes the size and format of a texture upload payload.
///
/// # Important
///
/// This only describes the size of a texture mip/array slice. This does not encode the target mip
/// level or array slice as that is handled separately. This allows for the same staging data to be
/// used as the source for multiple destination resources.
#[derive(Clone, Eq, PartialEq, Debug, Default, Hash)]
pub struct TextureUploadDesc {
    /// The width of the texture. Row pitch is handled internally, this should be the logical width
    /// not physical width.
    pub width: u32,

    /// The height of the texture
    pub height: u32,

    /// The depth of the texture
    pub depth: u32,

    /// The pixel format of the texture
    pub format: Format,
}

impl TextureUploadDesc {
    /// Computes the number of bytes needed to store the texture slice described by this
    /// [TextureUploadDesc] in a format compatible with being uploaded using a buffer to texture
    /// copy.
    pub const fn size_requirement(&self) -> u32 {
        debug_assert!(self.width > 0);
        debug_assert!(self.height > 0);
        debug_assert!(self.depth > 0);

        // Width must be aligned to the row pitch
        let aligned_width = self
            .width
            .next_multiple_of(self.format.buffer_to_texture_copy_row_pitch());

        // Use the padded width and bytes-per-texel to calculate the required size
        let texel_count = aligned_width * self.height * self.depth;
        let bytes = self.format.bytes_per_element() * texel_count;

        bytes
    }
}

/// A data source for a texture upload request. Represents an annotated block of upload memory that
/// pairs the memory block with a description of the texture slice that it contains.
///
/// This can be combined in the upload manager with a target texture + mip level and array slice to
/// create an upload request.
///
/// This struct abstracts access to a chunk of upload memory so that it can be safely used to upload
/// texture data to the GPU.
///
/// # Performance Warning
///
/// It is likely for the underlying upload memory to be mapped as write-combined or uncached. This
/// will make reads from the upload memory very expensive as well as make random writes expensive.
///
/// It is highly recommended to only write to this memory once, sequentially.
pub struct TextureUploadSource {
    /// A tracking handle for the buffer that the upload data is going to be stored in. This may be
    /// an owned buffer (the request is the only holder of the refcount) or a shared buffer where
    /// the data is a suballocated region from this buffer.
    ///
    /// Importantly the underlying mapped buffer memory will never be accessed through this
    /// reference. This is simply stored to keep the buffer alive.
    pub(crate) buffer: AnyArc<dyn IBuffer>,

    /// A description of the texture data we will store in this upload memory block. The size of the
    /// staging memory block will have been derived from this description.
    pub(crate) desc: TextureUploadDesc,

    /// An offset in bytes from the start of the buffer for where the upload data starts. This
    /// is needed as the upload commands use an offset+len and not a raw ptr+len pair. We use [u32]
    /// because that is the largest width we can send to a platform API for a buffer offset.
    pub(crate) offset: u32,

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

impl TextureUploadSource {
    /// Constructs a new upload source from the given parameters. Includes some debug validation to
    /// try and detect mistakes.
    ///
    /// # Safety
    ///
    /// There are a bunch of requirements for safely implementing this system.
    ///
    /// - 'desc' should describe a valid, non-zero sized texture.
    /// - 'data' takes ownership of the underlying buffer
    /// - 'data' must point to memory in the mapped range of 'buffer'
    /// - 'data' must be sized for the texture described by 'desc'
    /// - 'offset' must be aligned to 512 bytes within the buffer
    /// - 'data.len()' combined with 'offset' must not overrun the end of the buffer
    ///
    /// There are a bunch of debug asserts for these which are only enabled on debug builds, check
    /// those to see all the requirements.
    pub(crate) unsafe fn new(
        buffer: AnyArc<dyn IBuffer>,
        desc: TextureUploadDesc,
        offset: u32,
        data: NonNull<[u8]>,
    ) -> Self {
        #[cfg(debug_assertions)]
        {
            debug_assert!(desc.width > 0, "Width must be > 0");
            debug_assert!(desc.height > 0, "Height must be > 0");
            debug_assert!(desc.depth > 0, "Depth must be > 0");

            let required_size = desc.size_requirement() as usize;
            let actual_size = data.len();
            debug_assert!(
                required_size >= actual_size,
                "data.len() is {} but must be >= {}",
                actual_size,
                required_size
            );

            debug_assert_eq!(
                offset % 512,
                0,
                "Offset must be aligned to 512 bytes within the buffer"
            );

            let buffer_desc = buffer.desc_ref();
            debug_assert!(
                buffer_desc.cpu_access == CpuAccessMode::Write,
                "'data' must be from an upload heap"
            );

            debug_assert!(
                buffer_desc.size >= offset as u64,
                "'offset' ({}) is outside of the buffer (size {})",
                offset,
                buffer_desc.size
            );

            let bytes_after_offset = buffer_desc.size - offset as u64;
            debug_assert!(
                bytes_after_offset >= required_size as u64,
                "'buffer' is to small. [{}+{}] overruns the upload buffer (size {})",
                offset,
                data.len(),
                buffer_desc.size
            );
        }

        Self {
            buffer,
            desc,
            offset,
            data,
        }
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
    /// This is provided as it is technically valid usage but should be handled with care.
    pub fn data_ref(&self) -> &[u8] {
        // Safety: It is guaranteed by the implementation that this should be uniquely owned by the
        //         request and valid for access as long as the upload request object is available.
        unsafe { self.data.as_ref() }
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
    /// This is provided as it is technically valid usage but should be handled with care.
    pub fn data_mut(&mut self) -> &mut [u8] {
        // Safety: It is guaranteed by the implementation that this should be uniquely owned by the
        //         request and valid for access as long as the upload request object is available.
        unsafe { self.data.as_mut() }
    }
}
