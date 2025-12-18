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

use std::ops::Range;
use std::ptr::NonNull;

use aleph_device_allocators::{IUploadAllocator, UploadBumpAllocator};
use arrayvec::ArrayVec;
use smallbox::SmallBox;
use smallbox::space::S8;

use crate::resource::texture::physical::PhysicalTextureDesc;
use crate::resource::texture::simple::SimpleTextureDesc;
use crate::resource::texture::single::SingleTextureDesc;
use crate::resource_loader::upload_buffer::{IUploadBuffer, UploadBuffer};

/// A data source paired with a list of offsets that encodes where each mip in a set of levels
/// starts in that buffer.
///
/// The 'data' field will contain a list of n offsets into 'buffer' that describe where each mip
/// in a range (also encoded by 'data') of mip levels. No meaning is ascribed to these levels by
/// a [`MipUploadDesc`] on its own. Only by pairing with a texture description of some variety
/// is it possible to interpret the data store in an instance of this struct.
///
/// Simple cases will often create an instance of this struct directly with a texture desc,
/// immediately pairing them. However, more advanced data driven use cases may require separating
/// the raw texture data from the specific description of that texture.
///
/// Whole mip levels must be provided when using this type. Partial or streaming uploads are not
/// supported by this interface.
pub struct MipUploadDesc {
    /// The alignment, in bytes, we are aligning our row_stride to for each level.
    pub(crate) stride_align: u32,

    /// The buffer that our image levels have been populated in ready for upload to the GPU.
    pub buffer: SmallBox<dyn IUploadBuffer, S8>,

    /// List of offsets into 'buffer' where a given mip level's data can be found.
    pub data: LevelOffsets,
}

impl MipUploadDesc {
    /// Constructs a new instance from the given device and desc by allocating a whole new upload
    /// buffer with space for the requested mips of the described texture.
    ///
    /// Takes a 'stride_align' value that specifies the alignment, in bytes, that each row of our
    /// upload data will be aligned to. The 'stride_align' value will be taken as
    /// `max(stride_align, 1)`.
    pub fn new_owned<T: SingleTextureDesc>(
        device: &dyn rhi::IDevice,
        desc: &T,
        stride_align: u32,
        base_level: u32,
        num_levels: u32,
    ) -> Result<Self, rhi::BufferCreateError> {
        let stride_align = stride_align.max(1); // Clamp to >0

        let (data, total_size) =
            LevelOffsets::new_for_desc(desc, stride_align, base_level, num_levels);

        let buffer = UploadBuffer::new_owned(device, total_size as u64)?;

        let out = Self {
            stride_align,
            buffer: buffer.into_smallbox(),
            data,
        };
        Ok(out)
    }

    /// Constructs a new instance from the given allocator and desc by sub-allocating from the
    /// arena with space for the requested mips of the described texture.
    pub fn new_in_bump_arena<T: PhysicalTextureDesc>(
        bump: &UploadBumpAllocator,
        desc: &T,
        stride_align: u32,
        base_level: u32,
        num_levels: u32,
    ) -> Result<Self, rhi::BufferCreateError> {
        assert!(bump.usage().contains(rhi::ResourceUsageFlags::COPY_SOURCE));

        let stride_align = stride_align.max(1); // Clamp to >0

        let (data, total_size) =
            LevelOffsets::new_for_desc(desc, stride_align, base_level, num_levels);

        let buffer = unsafe {
            let block = bump
                .allocate_aligned(total_size, 512)
                .ok_or(rhi::BufferCreateError::OutOfMemory)?;
            let data = NonNull::slice_from_raw_parts(block.result, total_size);
            UploadBuffer::new(bump.buffer().clone(), block.device_offset as u64, data)
        };

        let out = Self {
            stride_align,
            buffer: buffer.into_smallbox(),
            data,
        };
        Ok(out)
    }

    /// The configured 'stride_align' value for this upload desc.
    ///
    /// Specifies the alignment, in bytes, for each row of our upload data. Is used to derive
    /// 'row_pitch' in [`rhi::ImageDataLayout`].
    pub const fn stride_align(&self) -> u32 {
        self.stride_align
    }
}

impl MipUploadDesc {
    /// Constructs a [`BufferToTextureCopyRegion`] that encodes a valid copy command to copy from
    /// the source buffer into the destination texture at the given mip and array layer.
    ///
    /// # Info
    ///
    /// We make some assumptions.
    /// - We only allow uploading entire mip levels and/or array layers so the origin is always
    ///   `(0, 0)` and the extent is assumed to cover the entire subresource.
    pub(crate) fn get_copy_region<T: SimpleTextureDesc>(
        &self,
        desc: &T,
        level: u32,
        aspect: rhi::TextureCopyAspect,
    ) -> rhi::BufferToTextureCopyRegion {
        let mip_offset = self.data.offset_for_level(level).unwrap() as u64;
        let src_offset = self.buffer.device_offset() + mip_offset;
        let extent = desc.as_level(level).storage_extent();
        rhi::BufferToTextureCopyRegion {
            src: rhi::ImageDataLayout {
                offset: src_offset,
                row_pitch: desc
                    .as_level(level)
                    .with_aligned_stride(self.stride_align)
                    .upload_row_texels(),
                extent,
            },
            dst: rhi::TextureCopyInfo {
                mip_level: level,
                array_layer: 0,
                aspect,
                origin: rhi::UOffset3D::new(0, 0, 0),
                extent,
            },
        }
    }
}

/// Encodes a range of mip levels as `base_level..level_offsets.len()`, and provides an offset into
/// some buffer where the data for those mips in some texture can be found.
#[derive(Clone, Hash, Debug, Default)]
pub struct LevelOffsets {
    /// The base mip level that data is provided for in 'level_offsets'.
    pub base_level: u8,

    /// A list of indices into an associated memory buffer for where the beginning of a mip level's
    /// data is.
    ///
    /// All offsets must be aligned along a 512 byte boundary. The actual layout and size of the
    /// data is format specific.
    ///
    /// This field is ordered from the lower mip level upwards, with a starting index specified by
    /// 'base_level'.
    ///
    /// # Example
    ///
    /// Given a 'base_level' of 2, level_offsets[0] would refer to mip 2, level_offsets[1] to mip 3,
    /// and so on.
    ///
    /// # What about disjoint sets?
    ///
    /// We don't see a strong use case for specifying a sparse set of mips (i.e. populate mip 1 and
    /// 3).
    pub level_offsets: ArrayVec<usize, 16>,
}

impl LevelOffsets {
    /// Creates a new instance that contains valid offsets for the provided mip range based on the
    /// given texture description.
    ///
    /// This function will automatically generate offsets based on the physical layout of the
    /// texture, encoding the new instance so it only references the requested mip range. Alignment
    /// will be automatically handled. A user would be able to allocate a buffer and immediately use
    /// the offsets to copy texture data. The offsets are guaranteed to be generated such that no
    /// mip level overlaps.
    pub fn new_for_desc<T: SingleTextureDesc>(
        desc: &T,
        stride_align: u32,
        base_level: u32,
        num_levels: u32,
    ) -> (Self, usize) {
        let min_level = base_level;
        let max_level = min_level + num_levels;

        let mut total_size = 0;
        let mut level_offsets = ArrayVec::new();
        for level in min_level..max_level {
            level_offsets.push(total_size);
            total_size += desc
                .as_level(level)
                .with_aligned_stride(stride_align)
                .upload_bytes();
            total_size = total_size.next_multiple_of(512);
        }

        let offsets = LevelOffsets {
            base_level: base_level as u8, // Cast is checked above
            level_offsets,
        };
        (offsets, total_size)
    }

    /// Gets the offset that is stored for the given mip level the instance is associated with.
    ///
    /// The 'level' param is an absolute mip index, not relative to 'base_level'. If
    /// `level < base_level` this will return [`None`], or if 'level' is any other way out of
    /// bounds.
    pub fn offset_for_level(&self, level: u32) -> Option<usize> {
        // base_level is the index of the first mip. subtracting base level from 'level' gives us
        // the index to lookup in 'level_offsets'. the overflow check doubles as a bounds check
        // for a mips we haven't provided data for.
        let i = level.checked_sub(self.base_level as u32)?;

        // Gets the offset for the mip we want based on the calculated mip level. The bounds check
        // on the index also bounds checks whether we asked for a mip level we don't have data for.
        self.level_offsets.get(i as usize).copied()
    }

    /// Yields a [`Range`] that encodes the mip levels that are available as described by self.
    pub const fn level_range(&self) -> Range<u32> {
        let base_level = self.base_level as u32;
        let end_level = base_level + self.level_offsets.len() as u32;
        base_level..end_level
    }
}
