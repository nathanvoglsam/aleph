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

use aleph_object_system::ArcObject;

use crate::*;

#[derive(Clone)]
pub struct TextureHandle {
    inner: ArcObject,
}

impl TextureHandle {
    /// # Safety
    ///
    /// It is the caller's responsibility to ensure that the given object refers to an object that
    /// the inner RHI implementation considers a texture objec.
    pub const unsafe fn new(inner: ArcObject) -> Self {
        Self { inner }
    }

    ///
    /// Gets the number of strong ([`TextureHandle`]) pointers to this allocation.
    ///
    /// # Safety
    ///
    /// This method by itself is safe, but using it correctly requires extra care.
    /// Another thread can change the strong count at any time,
    /// including potentially between calling this method and acting on the result.
    ///
    /// # Info
    ///
    /// This is just a wrapper around [`std::sync::Arc::strong_count`]
    ///
    #[inline]
    #[must_use]
    pub fn strong_count(&self) -> usize {
        self.inner.strong_count()
    }

    /// Unwrap the [`TextureHandle`] and get the inner [`ArcObject`]
    #[inline]
    pub fn into_inner(self) -> ArcObject {
        self.inner
    }

    /// Get the inner [`ArcObject`]
    pub const fn get(&self) -> &ArcObject {
        &self.inner
    }
}

/// Description object used for creating a new texture.
#[derive(Clone, PartialEq, Debug, Default)]
pub struct TextureDesc<'a> {
    /// The width of the texture
    pub width: u32,

    /// The height of the texture
    pub height: u32,

    /// The depth of the texture
    pub depth: u32,

    /// The pixel format of the texture
    pub format: Format,

    /// The dimensionality of the texture.
    ///
    /// Declares whether the texture should be a 1D, 2D, 3D or cube texture.
    pub dimension: TextureDimension,

    /// An optional clear value that will be 'optimal' for the underlying implementation.
    pub clear_value: Option<OptimalClearValue>,

    /// Number of image array elements.
    ///
    /// A value of '1' means to create a regular, non-array texture. Setting this to a value >1
    /// declares the texture as a texture array.
    pub array_size: u32,

    /// Number of mip levels.
    pub mip_levels: u32,

    /// Sample count, for MSAA texture.
    ///
    /// A value of '1' means a regular, non MSAA texture. This value must always be a power of two.
    /// Setting this to a value >1 declares the texture as an MSAA texture.
    pub sample_count: u32,

    /// Sample quality, for MSAA texture
    pub sample_quality: u32,

    /// Specifies in what ways the texture can be used
    pub usage: ResourceUsageFlags,

    /// The name of the object
    pub name: Option<&'a str>,
}

impl<'a> TextureDesc<'a> {
    pub const fn get_extent_2d(&self) -> Extent2D {
        Extent2D {
            width: self.width,
            height: self.height,
        }
    }

    pub const fn get_extent_3d(&self) -> Extent3D {
        Extent3D {
            width: self.width,
            height: self.height,
            depth: self.depth,
        }
    }

    /// A utility function that strips the debug name from the description so we can get a static
    /// lifetime on the desc
    pub const fn strip_name(self) -> TextureDesc<'static> {
        TextureDesc::<'static> {
            width: self.width,
            height: self.height,
            depth: self.depth,
            format: self.format,
            dimension: self.dimension,
            clear_value: self.clear_value,
            array_size: self.array_size,
            mip_levels: self.mip_levels,
            sample_count: self.sample_count,
            sample_quality: self.sample_quality,
            usage: self.usage,
            name: None,
        }
    }

    /// A utility function that replaces any existing name with the given name, yielding a new desc
    /// identical to the source desc differeing only in name.
    pub const fn with_name<'b>(self, name: &'b str) -> TextureDesc<'b> {
        TextureDesc::<'b> {
            width: self.width,
            height: self.height,
            depth: self.depth,
            format: self.format,
            dimension: self.dimension,
            clear_value: self.clear_value,
            array_size: self.array_size,
            mip_levels: self.mip_levels,
            sample_count: self.sample_count,
            sample_quality: self.sample_quality,
            usage: self.usage,
            name: Some(name),
        }
    }

    pub const fn with_width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }

    pub const fn with_height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }

    pub const fn with_depth(mut self, depth: u32) -> Self {
        self.depth = depth;
        self
    }

    pub const fn with_format(mut self, format: Format) -> Self {
        self.format = format;
        self
    }

    pub const fn with_dimension(mut self, dimension: TextureDimension) -> Self {
        self.dimension = dimension;
        self
    }

    pub const fn with_clear_value(mut self, clear_value: OptimalClearValue) -> Self {
        self.clear_value = Some(clear_value);
        self
    }

    pub const fn with_array_size(mut self, array_size: u32) -> Self {
        self.array_size = array_size;
        self
    }

    pub const fn with_mip_levels(mut self, mip_levels: u32) -> Self {
        self.mip_levels = mip_levels;
        self
    }

    pub const fn with_sample_count(mut self, sample_count: u32) -> Self {
        self.sample_count = sample_count;
        self
    }

    pub const fn with_sample_quality(mut self, sample_quality: u32) -> Self {
        self.sample_quality = sample_quality;
        self
    }

    pub const fn with_usage(mut self, usage: ResourceUsageFlags) -> Self {
        self.usage = usage;
        self
    }

    pub const fn texture_1d(width: u32) -> TextureDesc<'static> {
        TextureDesc {
            width,
            height: 1,
            depth: 1,
            format: Format::R8Unorm,
            dimension: TextureDimension::Texture1D,
            clear_value: None,
            array_size: 1,
            mip_levels: 1,
            sample_count: 1,
            sample_quality: 0,
            usage: ResourceUsageFlags::NONE,
            name: None,
        }
    }

    pub const fn texture_1d_array(width: u32, array_size: u32) -> TextureDesc<'static> {
        Self::texture_1d(width).with_array_size(array_size)
    }

    pub const fn texture_2d(width: u32, height: u32) -> TextureDesc<'static> {
        TextureDesc {
            width,
            height,
            depth: 1,
            format: Format::R8Unorm,
            dimension: TextureDimension::Texture2D,
            clear_value: None,
            array_size: 1,
            mip_levels: 1,
            sample_count: 1,
            sample_quality: 0,
            usage: ResourceUsageFlags::NONE,
            name: None,
        }
    }

    pub const fn texture_2d_array(
        width: u32,
        height: u32,
        array_size: u32,
    ) -> TextureDesc<'static> {
        Self::texture_2d(width, height).with_array_size(array_size)
    }

    pub const fn texture_3d(width: u32, height: u32, depth: u32) -> TextureDesc<'static> {
        TextureDesc {
            width,
            height,
            depth,
            format: Format::R8Unorm,
            dimension: TextureDimension::Texture3D,
            clear_value: None,
            array_size: 1,
            mip_levels: 1,
            sample_count: 1,
            sample_quality: 0,
            usage: ResourceUsageFlags::NONE,
            name: None,
        }
    }

    pub const fn texture_3d_array(
        width: u32,
        height: u32,
        depth: u32,
        array_size: u32,
    ) -> TextureDesc<'static> {
        Self::texture_3d(width, height, depth).with_array_size(array_size)
    }
}

/// Enumeration about all major texture types.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum TextureDimension {
    /// One dimensional texture. Logically similar to a 2D image with a height of 1
    Texture1D,

    /// A standard 2D texture.
    Texture2D,

    /// A 3D volume texture.
    Texture3D,
}

impl std::fmt::Display for TextureDimension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextureDimension::Texture1D => f.write_str("Texture1D"),
            TextureDimension::Texture2D => f.write_str("Texture2D"),
            TextureDimension::Texture3D => f.write_str("Texture3D"),
        }
    }
}

impl Default for TextureDimension {
    #[inline(always)]
    fn default() -> Self {
        Self::Texture1D
    }
}

/// An enumeration of all possible input types for initializing a texture's optimal clear color
/// value
#[derive(Clone, Debug, PartialEq)]
pub enum OptimalClearValue {
    /// A full 4-channel f32 colour
    ColorF32 { r: f32, g: f32, b: f32, a: f32 },

    /// A 4-channel color packed into a single u32
    ColorInt(u32),

    /// A floating point + u8 pair for clearing a depth stencil texture
    DepthStencil(f32, u8),
}

impl From<u32> for OptimalClearValue {
    #[inline(always)]
    fn from(v: u32) -> Self {
        Self::ColorInt(v)
    }
}

impl std::fmt::Display for OptimalClearValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OptimalClearValue::ColorF32 { r, g, b, a } => {
                write!(f, "OptimalClearValue::ColorF32({r}, {g}, {b}, {a})")
            }
            OptimalClearValue::ColorInt(v) => {
                write!(f, "OptimalClearValue::ColorInt({:X})", *v)
            }
            OptimalClearValue::DepthStencil(depth, stencil) => {
                write!(f, "OptimalClearValue::DepthStencil({depth}, {stencil})")
            }
        }
    }
}

bitflags::bitflags! {
    #[derive(Default, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
    pub struct TextureAspect: u32 {
        /// Bit that specifies the 'color' aspect of a texture
        const COLOR = 0b00000001;

        /// Bit that specifies the 'depth' aspect of a texture
        const DEPTH = 0b00000010;

        /// Bit that specifies the 'stencil' aspect of a texture
        const STENCIL = 0b00000100;

        /// A combination of the [TextureAspect::DEPTH] and [TextureAspect::STENCIL] flags
        const DEPTH_STENCIL = Self::DEPTH.bits() | Self::STENCIL.bits();
    }
}

impl From<TextureCopyAspect> for TextureAspect {
    #[inline(always)]
    fn from(val: TextureCopyAspect) -> Self {
        val.as_flag()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct TextureSubResourceSet {
    pub aspect: TextureAspect,
    pub base_mip_level: u32,
    pub num_mip_levels: u32,
    pub base_array_slice: u32,
    pub num_array_slices: u32,
}

impl TextureSubResourceSet {
    /// Returns a [TextureSubResourceSet] derived from the given [TextureDesc] that encodes a
    /// subresource set that encompases all subresources of the texture.
    pub const fn all(desc: &TextureDesc) -> Self {
        Self {
            aspect: desc.format.aspect_mask(),
            base_mip_level: 0,
            num_mip_levels: desc.mip_levels,
            base_array_slice: 0,
            num_array_slices: desc.array_size,
        }
    }

    /// A specialization of [TextureSubResourceSet::with_aspect] for [TextureAspect::COLOR].
    ///
    /// Returns a [TextureSubResourceSet] configured for a single mip/array level and access to the
    /// color aspect
    pub const fn with_color() -> Self {
        Self::with_aspect(TextureAspect::COLOR)
    }

    /// A specialization of [TextureSubResourceSet::with_aspect] for [TextureAspect::DEPTH].
    ///
    /// Returns a [TextureSubResourceSet] configured for a single mip/array level and access to the
    /// depth aspect
    pub const fn with_depth() -> Self {
        Self::with_aspect(TextureAspect::DEPTH)
    }

    /// A specialization of [TextureSubResourceSet::with_aspect] for [TextureAspect::STENCIL].
    ///
    /// Returns a [TextureSubResourceSet] configured for a single mip/array level and access to the
    /// stencil aspect
    pub const fn with_stencil() -> Self {
        Self::with_aspect(TextureAspect::STENCIL)
    }

    /// A specialization of [TextureSubResourceSet::with_aspect] for [TextureAspect::DEPTH_STENCIL].
    ///
    /// Returns a [TextureSubResourceSet] configured for a single mip/array level and access to the
    /// depth-stencil aspect
    pub const fn with_depth_stencil() -> Self {
        Self::with_aspect(TextureAspect::DEPTH_STENCIL)
    }

    /// A utility that configures the given subresource set with the given base mip level and number
    /// of mip levels.
    pub const fn with_mips(mut self, base: u32, num: u32) -> Self {
        self.base_mip_level = base;
        self.num_mip_levels = num;
        self
    }

    /// A utility that configures the given subresource set with the given base array slice and
    /// number of array slices.
    pub const fn with_levels(mut self, base: u32, num: u32) -> Self {
        self.base_array_slice = base;
        self.num_array_slices = num;
        self
    }

    /// A [TextureSubResourceSet] initialized for a single mip level and array slice with the given
    /// aspect flags.
    pub const fn with_aspect(aspect: TextureAspect) -> Self {
        Self {
            aspect,
            base_mip_level: 0,
            num_mip_levels: 1,
            base_array_slice: 0,
            num_array_slices: 1,
        }
    }
}
