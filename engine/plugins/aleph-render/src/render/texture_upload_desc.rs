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

use crate::render::BufferUploadSource;

/// This describes the size and format of a texture upload payload.
///
/// # Important
///
/// This only describes the size of a texture mip/array slice. This does not encode the target mip
/// level or array slice as that is handled separately. This allows for the same staging data to be
/// used as the source for multiple destination resources.
#[derive(Clone, Eq, PartialEq, Debug, Default, Hash)]
pub struct TextureMipUploadDesc {
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

impl TextureMipUploadDesc {
    pub const fn new(width: u32, height: u32, depth: u32, format: Format) -> Self {
        Self {
            width,
            height,
            depth,
            format,
        }
    }

    /// Computes the number of bytes needed to store the texture slice described by this
    /// [TextureMipUploadDesc] in a format compatible with being uploaded using a buffer to texture
    /// copy.
    pub const fn size_requirement(&self) -> usize {
        debug_assert!(self.width > 0);
        debug_assert!(self.height > 0);
        debug_assert!(self.depth > 0);

        // Width must be aligned to the row pitch
        let aligned_width = self.aligned_width();

        // Use the padded width and bytes-per-texel to calculate the required size
        let texel_count = aligned_width * self.height * self.depth;
        let bytes = self.format.bytes_per_element() * texel_count;

        bytes as usize
    }

    /// Returns the width of the texture expanded to the width required to satisfy row pitch
    /// requirements needed for uploading the texture with a buffer to texture copy.
    pub const fn aligned_width(&self) -> u32 {
        // Width must be aligned to the row pitch
        let aligned_width = self
            .width
            .next_multiple_of(self.format.buffer_to_texture_copy_row_pitch());

        aligned_width
    }

    pub const fn extent(&self) -> Extent3D {
        Extent3D::new(self.width, self.height, self.depth)
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
    /// The inner [`BufferUploadSource`] that we abstract over to create a [`TextureUploadSource`].
    pub(crate) source: BufferUploadSource,

    /// A description of the texture data we will store in this upload memory block. The size of the
    /// staging memory block will have been derived from this description.
    pub(crate) desc: TextureMipUploadDesc,
}

// TODO: Block Formats like DXT/BCn

impl TextureUploadSource {
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
    pub unsafe fn new(
        buffer: AnyArc<dyn IBuffer>,
        desc: TextureMipUploadDesc,
        offset: u64,
        data: NonNull<[u8]>,
    ) -> Self {
        let required_size = desc.size_requirement() as usize;

        #[cfg(debug_assertions)]
        {
            debug_assert!(desc.width > 0, "Width must be > 0");
            debug_assert!(desc.height > 0, "Height must be > 0");
            debug_assert!(desc.depth > 0, "Depth must be > 0");

            debug_assert!(
                data.len() >= required_size,
                "data.len() is {} but must be >= {}",
                data.len(),
                required_size
            );

            debug_assert_eq!(
                offset % 512,
                0,
                "Offset must be aligned to 512 bytes within the buffer"
            );
        }

        let source = BufferUploadSource::new(buffer, offset, data);
        Self { source, desc }
    }

    /// Constructs a new owned [TextureUploadSource] for the given texture upload description.
    ///
    /// # Safety
    ///
    /// See [TextureUploadSource::new] for more information.
    ///
    /// This utility is safer to use than [TextureUploadSource::new] as it guarantees all the buffer
    /// size and ownership constraints by construction. The only relevant requirements are to
    /// ensure the [TextureMipUploadDesc] encodes a valid non-zero-sized texture. That is:
    ///
    /// - 'desc.width', 'desc.height', and 'desc.depth' must all be at least 1. No zero-sized
    ///   textures.
    pub unsafe fn new_owned(
        device: &dyn IDevice,
        desc: TextureMipUploadDesc,
    ) -> Result<Self, BufferCreateError> {
        let size_requirement = desc.size_requirement();
        let buffer = device.create_buffer(&BufferDesc {
            size: size_requirement as u64,
            cpu_access: CpuAccessMode::Write,
            usage: ResourceUsageFlags::COPY_SOURCE,
            name: None,
        })?;

        let ptr = buffer.map().unwrap();
        let data = NonNull::slice_from_raw_parts(ptr, size_requirement as usize);
        let out = Self::new(buffer, desc, 0, data);

        Ok(out)
    }

    /// Discard any texture specific info and decay this [`TextureUploadSource`] into the underlying
    /// [`BufferUploadSource`].
    ///
    /// Useful for unifying the types for deferred deletion when all you care about is the rhi
    /// buffer handle.
    #[inline]
    pub fn into_buffer_source(self) -> BufferUploadSource {
        let TextureUploadSource { source, .. } = self;
        source
    }

    /// Calls [`IBuffer::unmap`] the internal buffer.
    ///
    /// # Safety
    ///
    /// See [`BufferUploadSource::unmap`] for more info.
    #[inline(always)]
    pub unsafe fn unmap(&self) {
        self.source.unmap();
    }

    /// Returns a handle to the [`IBuffer`] object that backs our staging buffer.
    ///
    /// # Warning
    ///
    /// See [`BufferUploadSource::buffer`] for more info.
    #[inline(always)]
    pub fn buffer(&self) -> &dyn IBuffer {
        self.source.buffer()
    }

    /// Constructs a [BufferToTextureCopyRegion] that encodes a valid copy command to copy from the
    /// source buffer into the destination texture at the given mip and array layer.
    ///
    /// # Info
    ///
    /// We make some assumptions.
    /// - We only allow uploading entire mip levels and/or array layers so the origin is always
    ///   `(0, 0)` and the extent is assumed to cover the entire subresource.
    pub const fn get_copy_region(
        &self,
        mip_level: u32,
        array_layer: u32,
        aspect: TextureCopyAspect,
    ) -> BufferToTextureCopyRegion {
        BufferToTextureCopyRegion {
            src: ImageDataLayout {
                offset: self.source.offset,
                row_pitch: self.desc.aligned_width(),
                extent: self.desc.extent(),
            },
            dst: TextureCopyInfo {
                mip_level,
                array_layer,
                aspect,
                origin: UOffset3D::new(0, 0, 0),
                extent: self.desc.extent(),
            },
        }
    }

    /// Gets a slice over the requested row, including padding texels needed for satisfying row
    /// pitch.
    ///
    /// # Safety
    ///
    /// The input parameter is not bounds checked and my produce a slice that is out of bounds of
    /// the upload buffer slice we're sub-slicing from. It is the caller's responsibility to ensure
    /// that the buffer is valid for the requested row.
    pub unsafe fn row_ptr(&self, row: u32) -> NonNull<[u8]> {
        // Calculate the offset to the start of the 'row'th row.
        let aligned_width = self.desc.aligned_width() as usize;
        let row_offset = aligned_width * row as usize;

        // We check anyway on debug builds because we can.
        #[cfg(debug_assertions)]
        {
            let row_count = self.desc.height * self.desc.depth;
            debug_assert!(
                row < row_count,
                "Row '{row}' out of bounds of '{row_count}'"
            );
        }

        // Make our sub-slice with the new offset+len pair.
        let base_ptr = self.source.data.cast::<u8>().as_ptr();
        let ptr = base_ptr.add(row_offset);
        let ptr = NonNull::new(ptr as _).unwrap_unchecked();
        NonNull::slice_from_raw_parts(ptr, aligned_width)
    }

    /// Get the upload block as a raw pointer.
    ///
    /// See [`BufferUploadSource::data_ptr`] for more info.
    pub const fn data_ptr(&self) -> NonNull<[u8]> {
        self.source.data
    }

    /// Get the upload block as a slice.
    ///
    /// See [`BufferUploadSource::data_mut`] for more info.
    #[inline(always)]
    pub fn data_mut(&mut self) -> &mut [u8] {
        // Safety: It is guaranteed by the implementation that this should be uniquely owned by the
        //         request and valid for access as long as the upload request object is available.
        unsafe { self.source.data.as_mut() }
    }
}

impl Into<BufferUploadSource> for TextureUploadSource {
    fn into(self) -> BufferUploadSource {
        self.into_buffer_source()
    }
}
