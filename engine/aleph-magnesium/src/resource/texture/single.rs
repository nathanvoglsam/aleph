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

use crate::internal::utils::const_max;
use crate::resource::texture::physical::PhysicalTextureLayout;

/// Interface exposed by some struct that provides a description of a single texture.
///
/// This does not encode any reference to mip level, array levels, or any other concept. This
/// interface simply describes the shape of some arbitrary matrix of texels.
pub trait SingleTextureDesc {
    /// The width of the texture, in pixels. May be zero.
    fn width(&self) -> u32;

    /// The height of the texture, in pixels. May be zero.
    fn height(&self) -> u32;

    /// The depth of the texture, in pixels. May be zero.
    fn depth(&self) -> u32;

    /// The pixel format of the texture.
    fn format(&self) -> rhi::Format;

    /// Make a [`SingleTextureLayout`] from self.
    fn as_single_layout(&self) -> SingleTextureLayout {
        SingleTextureLayout {
            width: self.width(),
            height: self.height(),
            depth: self.depth(),
            format: self.format(),
        }
    }

    /// Produces a new [`PhysicalTextureLayout`] from self, adopting the given 'row_stride' value.
    ///
    /// The final value of 'row_stride' in the output will be taken as `max(self.width, row_stride)`
    fn with_stride(&self, row_stride: u32) -> PhysicalTextureLayout {
        PhysicalTextureLayout {
            width: SingleTextureDesc::width(self),
            height: SingleTextureDesc::height(self),
            depth: SingleTextureDesc::depth(self),
            row_stride: const_max(row_stride, SingleTextureDesc::width(self)),
            format: SingleTextureDesc::format(self),
        }
    }

    /// Produces a new [`PhysicalTextureLayout`] from self, deriving a 'row_stride' that is aligned
    /// to 'stride_align' bytes.
    ///
    /// The value for 'stride_align' will be taken as `max(1, stride_align)`.
    fn with_aligned_stride(&self, stride_align: u32) -> PhysicalTextureLayout {
        let stride_align = const_max(1, stride_align); // Clamp to > 0
        let stride_align_texels = const_max(1, stride_align / self.format().bytes_per_element());
        let row_stride = self.width().next_multiple_of(stride_align_texels);
        self.with_stride(row_stride)
    }

    /// Calculates the number of texels along the 'x' dimension.
    #[inline(always)]
    fn storage_width(&self) -> u32 {
        const_max(self.width(), 1)
    }

    /// Calculates the number of texels along the 'y' dimension.
    ///
    /// # Info
    ///
    /// Even for 1D images this will return 1, as there is logically a single row as far as in
    /// memory representation is concerned.
    #[inline(always)]
    fn storage_height(&self) -> u32 {
        const_max(self.height(), 1)
    }

    /// Calculates the number of texels along the 'z' dimension.
    ///
    /// # Info
    ///
    /// Even for 1D/2D images this will return 1, as there is logically a single plane as far as in
    /// memory representation is concerned.
    #[inline(always)]
    fn storage_depth(&self) -> u32 {
        const_max(self.depth(), 1)
    }

    /// Shorthand for calling [`PhysicalTextureDesc::storage_width`], etc.
    ///
    /// Returns (width, height, depth)
    #[inline]
    fn storage_dimensions(&self) -> (u32, u32, u32) {
        (
            self.storage_width(),
            self.storage_height(),
            self.storage_depth(),
        )
    }

    /// Assuming 'self' describes mip 0 in a mip chain, returns the a [`SingleTextureLayout`] that
    /// describes the given mip level.
    ///
    /// This will return the _storage_ size. Any dimension that would've been rounded to 0 will
    /// instead by clamped to 1. For example: given a 2x2 texture as mip 0, mip 1 will be 1x1, mip 2
    /// will be 1x1, and so on.
    #[inline]
    fn as_level(&self, level: u32) -> SingleTextureLayout {
        let width = const_max(self.storage_width() >> level, 1);
        let height = const_max(self.storage_height() >> level, 1);
        let depth = const_max(self.storage_depth() >> level, 1);
        SingleTextureLayout {
            width,
            height,
            depth,
            format: self.format(),
        }
    }

    /// Returns the 'storage extent' of the image (i.e. [`PhysicalTextureDesc::storage_width`],
    /// etc.) as an [`rhi::Extent3D`].
    #[inline]
    fn storage_extent(&self) -> rhi::Extent3D {
        rhi::Extent3D::new(
            self.storage_width(),
            self.storage_height(),
            self.storage_depth(),
        )
    }

    /// Returns the number of rows of texels that make up the image.
    ///
    /// This is defined as (height.max(1) * depth.max(1))
    #[inline]
    fn num_rows(&self) -> usize {
        self.storage_height() as usize * self.storage_depth() as usize
    }

    /// Returns the number of bytes the image consumes when stored in a densely packed matrix
    #[inline]
    fn bytes(&self) -> usize {
        let (width, height, depth) = self.storage_dimensions();
        let bytes_per_row = width as usize * self.format().bytes_per_element() as usize;
        bytes_per_row * height as usize * depth as usize
    }

    /// Returns the number of bytes consumed by a single row of texels.
    ///
    /// This does _not_ include any padding bytes in the size needed to meet minimum row pitch
    /// requirements.
    #[inline]
    fn row_bytes(&self) -> usize {
        let (width, _, _) = self.storage_dimensions();
        width as usize * self.format().bytes_per_element() as usize
    }
}

impl SingleTextureDesc for SingleTextureLayout {
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

impl<'a> SingleTextureDesc for rhi::TextureDesc<'a> {
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

/// A distillation of a texture description. Contains only the fields that impact the physical
/// layout of the texture in memory.
///
/// All dimensions are assumed to encode the size of mip zero.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct SingleTextureLayout {
    /// The width of the texture, in pixels. May be zero.
    pub width: u32,

    /// The height of the texture, in pixels. May be zero.
    pub height: u32,

    /// The depth of the texture, in pixels. May be zero.
    pub depth: u32,

    /// The pixel format of the texture.
    pub format: rhi::Format,
}
