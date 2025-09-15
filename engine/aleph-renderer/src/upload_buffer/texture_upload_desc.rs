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
use std::ops::Range;
use std::ptr::NonNull;

use aleph_device_allocators::{IUploadAllocator, UploadBumpAllocator};
use aleph_rhi_api::*;
use arrayvec::ArrayVec;
use smallbox::space::S8;
use smallbox::{SmallBox, smallbox};

use crate::{IUploadBuffer, SharedUploadBuffer};

/// This struct describes a texture object for storage in a texture pool. This is a simplifcation
/// over [`TextureDesc`] that omits various options that are only applicable to render targets.
///
/// # Info
///
/// The type of texture is derived from the declared dimensions of the texture. A 1D texture will
/// have 'width' > 0, and 'height' + 'depth' = 0. A 2D texture will have 'width' and 'height' > 0,
/// and 'depth' = 0. And so on for 3D textures. It is illegal for 'width' to ever be zero. Leaving
/// it as 0 will lead to panics.
///
/// - `width`
///     - must always be > 0.
/// - `height`
///     - when > 0 the texture may be 2D or 3D.
/// - `depth`
///     - when > 0 the texture will be 3D if `height` is > 0 too.
///     - if > 0, then `height` must be > 0 too.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct TextureObjectDesc {
    /// The width of the texture in texels. This can _not_ be zero, see [`TextureObjectDesc`] for
    /// info.
    pub width: u32,

    /// The height of the texture in texels. This can be zero, see [`TextureObjectDesc`] for info.
    pub height: u32,

    /// The depth of the texture in texels. This can be zero, see [`TextureObjectDesc`] for info.
    pub depth: u32,

    /// The number of mip levels the texture will have.
    pub num_levels: NonZero<u32>,

    // /// The number of array layers the texture will have.
    // pub num_layers: NonZero<u32>,
    /// The image format of the texture
    pub format: Format,

    /// The set of supported usage flags the resource should be created with
    pub usage: ResourceUsageFlags,
}

impl TextureObjectDesc {
    pub const fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            depth: 0,
            num_levels: NonZero::new(1).unwrap(),
            // num_layers: NonZero::new(1).unwrap(),
            format: Format::R8Unorm, // zero init
            usage: ResourceUsageFlags::empty(),
        }
    }

    // /// Set the number of array layers in the image. Must != 0.
    // ///
    // /// Defaults to 1.
    // pub const fn num_layers(&mut self, layers: u32) -> &mut Self {
    //     self.num_layers = NonZero::new(layers).unwrap();
    //     self
    // }

    /// Set the number of mip levels in the image. Must != 0.
    ///
    /// Defaults to 1.
    pub const fn num_levels(&mut self, levels: u32) -> &mut Self {
        self.num_levels = NonZero::new(levels).unwrap();
        self
    }

    /// Sets the number of mip levels in the image to the number needed to have a full mip chain
    /// for the current image dimensions.
    ///
    /// # Warning
    ///
    /// This will only be correct if the dimensions have been provided. Ensure width, height, and
    /// depth are correct before calling this function.
    pub fn num_levels_full(&mut self) -> &mut Self {
        let max = const_max(self.width, self.height);
        let max = const_max(max, self.depth);
        let max = const_max(max, 1);
        let levels = max as f32;
        let levels = levels.log2().floor() + 1.0;
        let levels = levels as u32;
        self.num_levels(levels)
    }

    /// The image format
    pub const fn format(&mut self, format: Format) -> &mut Self {
        self.format = format;
        self
    }

    /// The image usage
    pub const fn usage(&mut self, usage: ResourceUsageFlags) -> &mut Self {
        self.usage = usage;
        self
    }

    pub const fn image_1d(&mut self, width: u32) -> &mut Self {
        if width == 0 {
            panic!("width == 0");
        }

        self.width = width;
        self.height = 0;
        self.depth = 0;
        self
    }

    pub const fn image_2d(&mut self, width: u32, height: u32) -> &mut Self {
        if width == 0 {
            panic!("width == 0");
        }
        if height == 0 {
            panic!("height == 0");
        }

        self.width = width;
        self.height = height;
        self.depth = 0;
        self
    }

    pub const fn image_3d(&mut self, width: u32, height: u32, depth: u32) -> &mut Self {
        if width == 0 {
            panic!("width == 0");
        }
        if height == 0 {
            panic!("height == 0");
        }
        if depth == 0 {
            panic!("depth == 0");
        }

        self.width = width;
        self.height = height;
        self.depth = depth;
        self
    }

    /// Calculates the number of texels along the 'x' dimension in mip0.
    pub const fn storage_width(&self) -> u32 {
        const_max(self.width, 1)
    }

    /// Calculates the number of texels along the 'y' dimension in mip0.
    ///
    /// # Info
    ///
    /// Even for 1D images this will return 1, as there is logically a single row as far as in
    /// memory representation is concerned.
    pub const fn storage_height(&self) -> u32 {
        const_max(self.height, 1)
    }

    /// Calculates the number of texels along the 'z' dimension in mip0.
    ///
    /// # Info
    ///
    /// Even for 1D/2D images this will return 1, as there is logically a single plane as far as in
    /// memory representation is concerned.
    pub const fn storage_depth(&self) -> u32 {
        const_max(self.depth, 1)
    }

    /// Deduces the [`TextureDimension`] of the texture described by `self`. Will return `None` if
    /// 'self' doesn't describe a valid texture.
    pub const fn texture_dimension(&self) -> Option<TextureDimension> {
        let has_width = self.width > 0;
        let has_height = self.height > 0;
        let has_depth = self.depth > 0;
        let v = match (has_width, has_height, has_depth) {
            (true, false, false) => TextureDimension::Texture1D,
            (true, true, false) => TextureDimension::Texture2D,
            (true, true, true) => TextureDimension::Texture3D,
            _ => return None,
        };
        Some(v)
    }

    /// Checks if the [`TextureObjectDesc`] describes a valid texture.
    pub const fn validate(&self) -> Option<()> {
        // No ? in const
        let _dimension = match self.texture_dimension() {
            Some(v) => v,
            None => return None,
        };

        // if matches!(dimension, TextureDimension::Texture3D) {
        //     // No 3D arrays are allowed
        //     if self.num_layers.get() > 1 {
        //         return None;
        //     }
        // }

        Some(())
    }

    /// Returns the dimensions of the texture at the given mip level.
    pub const fn dimensions_for_level(&self, level: u32) -> (u32, u32, u32) {
        let width = const_max(self.storage_width() >> level, 1);
        let height = const_max(self.storage_height() >> level, 1);
        let depth = const_max(self.storage_depth() >> level, 1);
        (width, height, depth)
    }

    /// Returns the number of rows of texels that make up the image at the given level.
    ///
    /// This is defined as (height.max(1) * depth.max(1))
    pub fn num_rows_for_level(&self, level: u32) -> usize {
        let (_, height, depth) = self.dimensions_for_level(level);
        height as usize * depth as usize
    }

    /// Returns the number of bytes
    pub const fn bytes_for_level(&self, level: u32) -> usize {
        let (width, height, depth) = self.dimensions_for_level(level);
        let bytes_per_row = width as usize * self.format.bytes_per_element() as usize;
        bytes_per_row * height as usize * depth as usize
    }

    /// Returns the number of bytes consumed by a single row of texels.
    ///
    /// This does _not_ include any padding bytes in the size needed to meet minimum row pitch
    /// requirements.
    pub const fn row_bytes_for_level(&self, level: u32) -> usize {
        let (width, _, _) = self.dimensions_for_level(level);
        width as usize * self.format.bytes_per_element() as usize
    }

    /// Returns the number of bytes consumed by a single row of texels, including any padding needed
    /// to reach the minimum row pitch.
    pub const fn upload_row_bytes_for_level(&self, level: u32) -> usize {
        let bytes_per_row = self.row_bytes_for_level(level);
        bytes_per_row.next_multiple_of(256)
    }

    /// Returns the number of _texels_ consumed by a single row of texels, including any padding
    /// needed to reach the minimum row pitch.
    pub const fn upload_row_texels_for_level(&self, level: u32) -> u32 {
        let (width, _, _) = self.dimensions_for_level(level);
        width.next_multiple_of(self.format.buffer_to_texture_copy_row_pitch())
    }

    /// Returns the number of bytes needed to store the given mip level in an upload buffer.
    ///
    /// This will calculate the size as (padded row size) * height.max(1) * depth.max(1). We need
    /// to pad each row to a 256 byte boundary to meet the minimum row pitch requirements for
    /// upload to the GPU.
    pub const fn upload_bytes_for_level(&self, level: u32) -> usize {
        let bytes_per_row = self.upload_row_bytes_for_level(level);
        let (_, height, depth) = self.dimensions_for_level(level);
        bytes_per_row * height as usize * depth as usize
    }
}

/// A description of what data for some texture is present in an associated upload buffer.
#[derive(Clone, Hash, Debug, Default)]
pub struct TextureUploadDataDesc {
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

impl TextureUploadDataDesc {
    pub fn offset_for_level(&self, level: u32) -> Option<usize> {
        // base_level is the index of the first mip. subtracting base level from 'level' gives us
        // the index to lookup in 'level_offsets'. the overflow check doubles as a bounds check
        // for a mips we haven't provided data for.
        let i = level.checked_sub(self.base_level as u32)?;

        // Gets the offset for the mip we want based on the calculated mip level. The bounds check
        // on the index also bounds checks whether we asked for a mip level we don't have data for.
        self.level_offsets.get(i as usize).copied()
    }

    /// Yields a [`Range`] that yields the mip levels that have data available.
    pub fn level_range(&self) -> Range<u32> {
        let base_level = self.base_level as u32;
        let end_level = base_level + self.level_offsets.len() as u32;
        base_level..end_level
    }

    /// Validates that the data description is compatible with the given [`TextureObjectDesc`].
    pub fn validate(&self, object_desc: &TextureObjectDesc) -> Option<()> {
        for offset in self.level_offsets.iter() {
            // Offsets must be 512 byte aligned
            if *offset % 512 != 0 {
                return None;
            }
        }

        let min_level = self.base_level as usize;
        let max_level = min_level + self.level_offsets.len();

        // Mip chain addresses an out of bounds mip level
        if min_level >= object_desc.num_levels.get() as usize {
            return None;
        }

        // Mip chain addresses an out of bounds mip level
        if max_level > object_desc.num_levels.get() as usize {
            return None;
        }

        Some(())
    }
}

pub struct TextureUploadDesc {
    /// The buffer that our image levels have been populated in ready for upload to the GPU.
    pub buffer: SmallBox<dyn IUploadBuffer, S8>,

    /// A description of where each mip level we've provided data for can be found within
    /// [`Self::buffer`].
    pub data: TextureUploadDataDesc,
}

impl TextureUploadDesc {
    /// Constructs a new owned [`TextureUploadDesc`] for the given buffer upload description.
    pub fn new_owned(
        device: &dyn IDevice,
        desc: &TextureObjectDesc,
        base_level: u32,
        num_levels: u32,
    ) -> Result<Self, BufferCreateError> {
        desc.validate().unwrap();

        let min_level = base_level;
        let max_level = min_level + num_levels;

        assert!(min_level < desc.num_levels.get());
        assert!(max_level <= desc.num_levels.get());

        let mut total_size = 0;
        let mut level_offsets = ArrayVec::new();
        for level in min_level..max_level {
            level_offsets.push(total_size);
            total_size += desc.upload_bytes_for_level(level);
            total_size = total_size.next_multiple_of(512);
        }

        let data = TextureUploadDataDesc {
            base_level: base_level as u8, // Cast is checked above
            level_offsets,
        };

        let buffer = unsafe {
            let buffer = device.create_buffer(&BufferDesc {
                size: total_size as u64,
                cpu_access: CpuAccessMode::Write,
                usage: ResourceUsageFlags::COPY_SOURCE,
                name: None,
            })?;

            let ptr = device.map_buffer(&buffer).unwrap();
            let data = NonNull::slice_from_raw_parts(ptr, total_size);
            SharedUploadBuffer::new(buffer, 0, data)
        };

        let out = Self {
            buffer: smallbox!(buffer),
            data,
        };
        Ok(out)
    }

    /// Constructs a new owned [`TextureUploadDesc`] for the given buffer upload description.
    pub fn new_in_bump_arena(
        bump: &UploadBumpAllocator,
        desc: &TextureObjectDesc,
        base_level: u32,
        num_levels: u32,
    ) -> Result<Self, BufferCreateError> {
        assert!(bump.usage().contains(ResourceUsageFlags::COPY_SOURCE));

        desc.validate().unwrap();

        let min_level = base_level;
        let max_level = min_level + num_levels;

        assert!(min_level < desc.num_levels.get());
        assert!(max_level <= desc.num_levels.get());

        let mut total_size = 0;
        let mut level_offsets = ArrayVec::new();
        for level in min_level..max_level {
            level_offsets.push(total_size);
            total_size += desc.upload_bytes_for_level(level);
            total_size = total_size.next_multiple_of(512);
        }

        let data = TextureUploadDataDesc {
            base_level: base_level as u8, // Cast is checked above
            level_offsets,
        };

        let buffer = unsafe {
            let block = bump
                .allocate_aligned(total_size, 512)
                .ok_or(BufferCreateError::OutOfMemory)?;
            let data = NonNull::slice_from_raw_parts(block.result, total_size);
            SharedUploadBuffer::new(bump.buffer().clone(), block.device_offset as u64, data)
        };

        let out = Self {
            buffer: smallbox!(buffer),
            data,
        };
        Ok(out)
    }

    /// Constructs a [BufferToTextureCopyRegion] that encodes a valid copy command to copy from the
    /// source buffer into the destination texture at the given mip and array layer.
    ///
    /// # Info
    ///
    /// We make some assumptions.
    /// - We only allow uploading entire mip levels and/or array layers so the origin is always
    ///   `(0, 0)` and the extent is assumed to cover the entire subresource.
    pub(crate) fn get_copy_region(
        &self,
        desc: &TextureObjectDesc,
        level: u32,
        // array_layer: u32,
        aspect: TextureCopyAspect,
    ) -> BufferToTextureCopyRegion {
        let mip_offset = self.data.offset_for_level(level).unwrap() as u64;
        let src_offset = self.buffer.device_offset() + mip_offset;
        let (w, h, d) = desc.dimensions_for_level(level);
        let extent = Extent3D::new(w, h, d);
        BufferToTextureCopyRegion {
            src: ImageDataLayout {
                offset: src_offset,
                row_pitch: desc.upload_row_texels_for_level(level),
                extent,
            },
            dst: TextureCopyInfo {
                mip_level: level,
                array_layer: 0,
                aspect,
                origin: UOffset3D::new(0, 0, 0),
                extent,
            },
        }
    }
}

const fn const_max(a: u32, b: u32) -> u32 {
    if a < b { b } else { a }
}
