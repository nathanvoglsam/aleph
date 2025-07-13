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

use crate::{IPixelStorage, ImageBuffer, ImageView, ImageViewMut, PixelFormat};

/// An extended interface built atop [`IPixelStorage`] that types that allow accessing the pixels
/// in their stored image should implement.
///
/// This is an extended capability over [`IPixelAccess`] as some image storage types can not provide
/// access to individual pixels within the image (i.e. [`crate::DynamicImageBuffer`]). This
/// interface has an associated type [`IPixelAccess::Result`] which is derived from the pixel format
/// of the image. If the format is not known at compile time the interface can't be implemented.
///
/// # The Interface
///
/// This trait is simple. It exposes various functions for load/store to the pixel grid using the
/// associated [`PixelFormat`] type. All functions take direct pixel coordinates. That is: values
/// in the range 0..width, 0..height.
pub trait IPixelAccess: IPixelStorage {
    /// The type of pixel the image stores, and the type of the value [`IPixelAccess::load`] will
    /// return. This will be derived from the type of data the image stores, defined by the number
    /// of channels and the data type of those channels (i.e. RGBA8Unorm, R16Float, etc).
    type Result: PixelFormat;

    /// Wrapper over [`IPixelAccess::load_checked`] that panics if the requested pixel is out of
    /// bounds of the underlying image.
    fn load(&self, x: u32, y: u32) -> Self::Result {
        match self.load_checked(x, y) {
            Some(v) => v,
            None => {
                let i =
                    calculate_index(x, y, self.width(), self.height(), Self::Result::COMPONENTS);
                let i_end = i + Self::Result::COMPONENTS;
                panic!(
                    "({x}, {y}) is out of bounds! Texture dimensions = ({}, {}). Tried loading range({}..{})",
                    self.width(),
                    self.height(),
                    i,
                    i_end
                );
            }
        }
    }

    /// Loads the pixel stored at the given x/y coordinate.
    ///
    /// Returns [`None`] if the coordinate is out-of-bounds.
    fn load_checked(&self, x: u32, y: u32) -> Option<Self::Result>;

    /// Unsafe wrapper over [`IPixelAccess::load_checked`] that assumes the pixel coordinates are in
    /// bounds. No bounds checks are performed and out-of-bounds values will trigger undefined
    /// behavior.
    ///
    /// # Safety
    ///
    /// It is the caller's responsibility to:
    /// - Ensure that 'x' is < width.
    /// - Ensure that 'y' is < height.
    unsafe fn load_unchecked(&self, x: u32, y: u32) -> Self::Result {
        unsafe { self.load_checked(x, y).unwrap_unchecked() }
    }

    /// Wrapper over [`IPixelAccess::store_checked`] that panics if the requested pixel is out of
    /// bounds of the underlying image.
    fn store(&mut self, x: u32, y: u32, v: Self::Result) {
        match self.store_checked(x, y, v) {
            Some(v) => v,
            None => panic!(
                "({x}, {y}) is out of bounds! Texture dimensions = ({}, {})",
                self.width(),
                self.height()
            ),
        }
    }

    /// Replaces the pixel value at the requested x/y coordinate with the given value.
    ///
    /// Returns [`None`] if the coordinate is out-of-bounds.
    fn store_checked(&mut self, x: u32, y: u32, v: Self::Result) -> Option<()>;

    /// Unsafe wrapper over [`IPixelAccess::store_unchecked`] that assumes the pixel coordinates are
    /// in bounds. No bounds checks are performed and out-of-bounds values will trigger undefined
    /// behavior.
    ///
    /// # Safety
    ///
    /// It is the caller's responsibility to:
    /// - Ensure that 'x' is < width.
    /// - Ensure that 'y' is < height.
    unsafe fn store_unchecked(&mut self, x: u32, y: u32, v: Self::Result) {
        unsafe { self.store_checked(x, y, v).unwrap_unchecked() }
    }
}

impl<T: PixelFormat> IPixelAccess for ImageBuffer<T> {
    type Result = T;

    #[inline]
    fn load_checked(&self, x: u32, y: u32) -> Option<Self::Result> {
        let i = calculate_index(x, y, self.width, self.height, T::COMPONENTS);
        let i_end = i + T::COMPONENTS;
        if let Some(v) = self.data.get(i..i_end) {
            Some(T::from_storage(v))
        } else {
            None
        }
    }

    #[inline]
    fn store_checked(&mut self, x: u32, y: u32, v: T) -> Option<()> {
        let i = calculate_index(x, y, self.width, self.height, T::COMPONENTS);
        let i_end = i + T::COMPONENTS;
        if let Some(dst) = self.data.get_mut(i..i_end) {
            Some(v.write_at(dst))
        } else {
            None
        }
    }
}

impl<'a, T: PixelFormat> IPixelAccess for ImageView<'a, T> {
    type Result = T;

    #[inline]
    fn load_checked(&self, x: u32, y: u32) -> Option<Self::Result> {
        let i = calculate_index(x, y, self.width, self.height, T::COMPONENTS);
        let i_end = i + T::COMPONENTS;
        if let Some(v) = self.data.get(i..i_end) {
            Some(T::from_storage(v))
        } else {
            None
        }
    }

    #[inline]
    fn store_checked(&mut self, _x: u32, _y: u32, _v: T) -> Option<()> {
        unimplemented!("Can't store to a read-only view")
    }
}

impl<'a, T: PixelFormat> IPixelAccess for ImageViewMut<'a, T> {
    type Result = T;

    #[inline]
    fn load_checked(&self, x: u32, y: u32) -> Option<Self::Result> {
        let i = calculate_index(x, y, self.width, self.height, T::COMPONENTS);
        let i_end = i + T::COMPONENTS;
        if let Some(v) = self.data.get(i..i_end) {
            Some(T::from_storage(v))
        } else {
            None
        }
    }

    #[inline]
    fn store_checked(&mut self, x: u32, y: u32, v: T) -> Option<()> {
        let i = calculate_index(x, y, self.width, self.height, T::COMPONENTS);
        let i_end = i + T::COMPONENTS;
        if let Some(dst) = self.data.get_mut(i..i_end) {
            Some(v.write_at(dst))
        } else {
            None
        }
    }
}

const fn calculate_index(x: u32, y: u32, width: u32, _height: u32, components: usize) -> usize {
    (y as usize * width as usize + x as usize) * components
}
