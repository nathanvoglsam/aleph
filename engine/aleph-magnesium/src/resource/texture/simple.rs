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

use crate::internal::utils::const_max;
use crate::resource::texture::single::SingleTextureDesc;

/// Describes a complete, simple texture object. This is an extension over [`SingleTextureDesc`]
/// that also encodes a number of mip levels. This _does_ however omit an array layer count for
/// simplicity.
///
/// Restricting to non-array textures avoids having to handle the non-existence of 3D array
/// textures when deducing texture dimension from just width/height/depth.
///
/// # Info
///
/// The type of texture is derived from the declared dimensions of the texture. A 1D texture will
/// have 'width' > 0, and 'height' + 'depth' = 0. A 2D texture will have 'width' and 'height' > 0,
/// and 'depth' = 0. And so on for 3D textures. It is illegal for 'width' to ever be zero. Leaving
/// it as 0 will lead to panics. We don't use [`NonZero`] for ease of use, but it will be checked.
///
/// - `width`
///     - should always be > 0.
/// - `height`
///     - when > 0 the texture may be 2D or 3D.
/// - `depth`
///     - when > 0 the texture will be 3D if `height` is > 0 too.
///     - if > 0, then `height` must be > 0 too.
pub trait SimpleTextureDesc: SingleTextureDesc {
    /// The number of mip levels the texture will have.
    fn num_levels(&self) -> NonZero<u32>;

    /// Produces a new [`SimpleTextureLayout`] from self.
    fn as_simple_layout(&self) -> SimpleTextureLayout {
        SimpleTextureLayout {
            width: SingleTextureDesc::width(self),
            height: SingleTextureDesc::height(self),
            depth: SingleTextureDesc::depth(self),
            format: SingleTextureDesc::format(self),
            num_levels: SimpleTextureDesc::num_levels(self),
        }
    }

    /// Deduces the [`TextureDimension`] of the texture described by `self`. Will return `None` if
    /// 'self' doesn't describe a valid texture.
    #[inline]
    fn texture_dimension(&self) -> Option<rhi::TextureDimension> {
        let has_width = self.width() > 0;
        let has_height = self.height() > 0;
        let has_depth = self.depth() > 0;
        let v = match (has_width, has_height, has_depth) {
            (true, false, false) => rhi::TextureDimension::Texture1D,
            (true, true, false) => rhi::TextureDimension::Texture2D,
            (true, true, true) => rhi::TextureDimension::Texture3D,
            _ => return None,
        };
        Some(v)
    }
}

impl SimpleTextureDesc for SimpleTextureLayout {
    #[inline]
    fn num_levels(&self) -> NonZero<u32> {
        self.num_levels
    }
}

impl SingleTextureDesc for SimpleTextureLayout {
    #[inline]
    fn width(&self) -> u32 {
        self.width
    }

    #[inline]
    fn height(&self) -> u32 {
        self.height
    }

    #[inline]
    fn depth(&self) -> u32 {
        self.depth
    }

    #[inline]
    fn format(&self) -> rhi::Format {
        self.format
    }
}

impl<'a> SimpleTextureDesc for rhi::TextureDesc<'a> {
    #[inline]
    fn num_levels(&self) -> NonZero<u32> {
        NonZero::new(self.mip_levels.max(1)).unwrap()
    }
}

/// Describes a complete, simple texture object. This is an extension over [`SingleTextureDesc`]
/// that also encodes a number of mip levels. This _does_ however omit an array layer count for
/// simplicity.
///
/// See [`SimpleTextureDesc`] for more.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct SimpleTextureLayout {
    /// The width of the texture in texels. This should _not_ be zero, see [`SimpleTextureDesc`] for
    /// info.
    pub width: u32,

    /// The height of the texture in texels. This can be zero, see [`SimpleTextureDesc`] for info.
    pub height: u32,

    /// The depth of the texture in texels. This can be zero, see [`SimpleTextureDesc`] for info.
    pub depth: u32,

    /// The image format of the texture
    pub format: rhi::Format,

    /// The number of mip levels the texture will have.
    pub num_levels: NonZero<u32>,
}

impl SimpleTextureLayout {
    /// Constructs a new, default initialized [`SimpleTextureLayout`]. The texture being described
    /// by this new instance will be invalid (width = 0). It is recommended to use one of the
    /// initializer utilities to complete constructing the texture. i.e.
    ///
    /// - [`SimpleTextureLayout::image_1d`]
    /// - [`SimpleTextureLayout::image_2d`]
    /// - [`SimpleTextureLayout::image_3d`]
    ///
    /// # Why Start Invalid?
    ///
    /// Keeps the interface simple, and catches when default initialized layouts are used when by
    /// mistake. We _could_ require [`NonZero`] on width, but that adds a lot of function noise when
    /// interacting with this API. Rather than front loading validation on the caller we defer it
    /// to first usage. This may make data-driven use cases easier too.
    pub const fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            depth: 0,
            format: rhi::Format::R8Unorm, // zero init
            num_levels: NonZero::new(1).unwrap(),
        }
    }

    /// Constructs a new [`SimpleTextureLayout`] that describes a 1D texture of some format with
    /// the given width.
    ///
    /// Will assert all dimensions are valid for a 1D texture.
    pub const fn image_1d(&mut self, width: u32) -> &mut Self {
        assert!(width > 0, "width == 0");

        self.width = width;
        self.height = 0;
        self.depth = 0;
        self
    }

    /// Constructs a new [`SimpleTextureLayout`] that describes a 2D texture of some format with
    /// the given width.
    ///
    /// Will assert all dimensions are valid for a 12D texture.
    pub const fn image_2d(&mut self, width: u32, height: u32) -> &mut Self {
        assert!(width > 0, "width == 0");
        assert!(height > 0, "height == 0");

        self.width = width;
        self.height = height;
        self.depth = 0;
        self
    }

    /// Constructs a new [`SimpleTextureLayout`] that describes a 3D texture of some format with
    /// the given width.
    ///
    /// Will assert all dimensions are valid for a 3D texture.
    pub const fn image_3d(&mut self, width: u32, height: u32, depth: u32) -> &mut Self {
        assert!(width > 0, "width == 0");
        assert!(height > 0, "height == 0");
        assert!(depth > 0, "depth == 0");

        self.width = width;
        self.height = height;
        self.depth = depth;
        self
    }

    /// The image format
    pub const fn with_format(&mut self, format: rhi::Format) -> &mut Self {
        self.format = format;
        self
    }

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
    #[inline]
    pub fn num_levels_full(&mut self) -> &mut Self {
        let max = const_max(self.width, self.height);
        let max = const_max(max, self.depth);
        let max = const_max(max, 1); // Clamp to 1 in case some silly person forgot to set width.
        let levels = max as f32;
        let levels = levels.log2().floor() + 1.0;
        let levels = levels as u32;
        self.num_levels(levels)
    }
}
