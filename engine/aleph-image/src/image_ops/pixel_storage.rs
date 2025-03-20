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

use aleph_math::{UVec2, Vec2};

use crate::{ImageBuffer, ImageView, ImageViewMut, PixelFormat};

/// The interface exposed by a type that stores a 2D grid of pixels. Implementing this trait does
/// not imply the ability to access those pixels.
///
/// This trait only implies that the type stores the pixels, and allows querying some
/// characteristics of the image that is stored such as the dimensions.
pub trait IPixelStorage {
    /// The width/height of the image in pixels.
    fn dimensions(&self) -> UVec2;

    /// The width/height of the image in pixels, but returned as a floating point value.
    ///
    /// # Why?
    ///
    /// This extra interface allows images to pre-calculate the width/height fp32 values. Some code
    /// ([`crate::IPixelSample`]) needs these as a float, and will be called in hot loops. The cost
    /// of the cast will add up if performed every iteration and relying on loop-invariant code
    /// motion to hoist the conversions across function boundaries is brittle.
    fn dimensions_f32(&self) -> Vec2;

    /// Returns the width of the image in pixels
    fn width(&self) -> u32 {
        self.dimensions().x
    }

    /// Returns the height of the image in pixels
    fn height(&self) -> u32 {
        self.dimensions().y
    }
}

impl<T: PixelFormat> IPixelStorage for ImageBuffer<T> {
    #[inline(always)]
    fn dimensions(&self) -> UVec2 {
        UVec2::new(self.width, self.height)
    }

    #[inline(always)]
    fn dimensions_f32(&self) -> Vec2 {
        Vec2::new(self.width_f32, self.height_f32)
    }
}

impl<'a, T: PixelFormat> IPixelStorage for ImageView<'a, T> {
    #[inline(always)]
    fn dimensions(&self) -> UVec2 {
        UVec2::new(self.width, self.height)
    }

    #[inline(always)]
    fn dimensions_f32(&self) -> Vec2 {
        Vec2::new(self.width_f32, self.height_f32)
    }
}

impl<'a, T: PixelFormat> IPixelStorage for ImageViewMut<'a, T> {
    #[inline(always)]
    fn dimensions(&self) -> UVec2 {
        UVec2::new(self.width, self.height)
    }

    #[inline(always)]
    fn dimensions_f32(&self) -> Vec2 {
        Vec2::new(self.width_f32, self.height_f32)
    }
}
