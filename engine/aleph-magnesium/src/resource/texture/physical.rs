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
use crate::resource::texture::single::SingleTextureDesc;

/// Interface exposed by some struct that provides a physical description of a texture. This
/// interface is used for describing the in-memory layout of some matrix of texels. This includes
/// a 'row_stride'.
///
/// The expected use case is for buffer->texture uploads.
///
/// All dimensions are assumed to encode the size of mip zero.
pub trait PhysicalTextureDesc: SingleTextureDesc {
    /// The stride of a single texture row, in texels. This determines how wide each texture row is
    /// in memory, separate from the logical width defined by `width`.
    ///
    /// Must be >= `width`.
    fn row_stride(&self) -> u32;

    /// Make a [`PhysicalTextureLayout`] from self.
    fn as_physical_layout(&self) -> PhysicalTextureLayout {
        PhysicalTextureLayout {
            width: self.width(),
            height: self.height(),
            depth: self.depth(),
            row_stride: self.row_stride(),
            format: self.format(),
        }
    }

    /// Returns the number of bytes consumed by a single row of texels, including any padding needed
    /// to reach the minimum row pitch.
    ///
    /// # Warning
    ///
    /// Takes the row stride as `max(self.row_stride, self.width)`.
    #[inline]
    fn upload_row_bytes(&self) -> usize {
        self.upload_row_texels() as usize * self.format().bytes_per_element() as usize
    }

    /// Returns the number of _texels_ consumed by a single row of texels, including padding to
    /// row_stride.
    #[inline]
    fn upload_row_texels(&self) -> u32 {
        const_max(self.row_stride(), self.width())
    }

    /// Returns the number of bytes needed to store the given mip level in an upload buffer.
    ///
    /// This will calculate the size as (self.row_stride) * height.max(1) * depth.max(1).
    #[inline]
    fn upload_bytes(&self) -> usize {
        let bytes_per_row = self.upload_row_bytes();
        let (_, height, depth) = self.storage_dimensions();
        bytes_per_row * height as usize * depth as usize
    }
}

impl SingleTextureDesc for PhysicalTextureLayout {
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

impl PhysicalTextureDesc for PhysicalTextureLayout {
    #[inline]
    fn row_stride(&self) -> u32 {
        self.row_stride
    }
}

/// A distillation of a texture description. Contains only the fields that impact the physical
/// layout of the texture in memory. This struct is used primarily for buffer -> texture upload
/// operations.
///
/// All dimensions are assumed to encode the size of mip zero.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct PhysicalTextureLayout {
    /// The width of the texture, in pixels. May be zero.
    pub width: u32,

    /// The height of the texture, in pixels. May be zero.
    pub height: u32,

    /// The depth of the texture, in pixels. May be zero.
    pub depth: u32,

    /// The stride of a single texture row, in texels. This determines how wide each texture row is
    /// in memory, separate from the logical width defined by `width`.
    ///
    /// Must be >= `width`.
    pub row_stride: u32,

    /// The pixel format of the texture.
    pub format: rhi::Format,
}
