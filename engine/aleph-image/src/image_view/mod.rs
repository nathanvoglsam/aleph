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

use aleph_math::UVec2;

use crate::{ImageBuffer, PixelChannelType, PixelFormat};

pub struct ImageView<'a, T: PixelFormat> {
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) width_f32: f32,
    pub(crate) height_f32: f32,
    pub(crate) data: &'a [T::Storage],
}

impl<'a, T: PixelFormat> ImageView<'a, T> {
    /// Constructs a new texture with the given dimensions.
    ///
    /// # Panic
    ///
    /// Will panic if 'data.len()' != 'width * height * PixelFormat::CHANNELS'.
    pub fn from_data(width: u32, height: u32, data: &'a [T::Storage]) -> Self {
        assert!(width <= 65535);
        assert!(height <= 65535);

        let expected_elements = ImageBuffer::<T>::calculate_element_count(width, height);
        assert_eq!(expected_elements, data.len());
        Self {
            width,
            height,
            width_f32: width as f32,
            height_f32: height as f32,
            data,
        }
    }

    /// Unwrap the image back to the raw buffer
    pub fn into_data(self) -> &'a [T::Storage] {
        self.data
    }

    /// Access the image's backing buffer as a flat array.
    #[inline]
    pub fn data(&self) -> &[T::Storage] {
        self.data
    }

    /// Run a function over all the pixels in an image
    #[inline]
    pub fn filter_pixels<F>(&self, mut f: F)
    where
        F: FnMut(UVec2, T),
    {
        let iter = self.data.chunks_exact(T::COMPONENTS).enumerate();
        for (i, p) in iter {
            let y = i as u32 / self.width;
            let x = i as u32 % self.width;

            let p = T::from_storage(p);
            f(UVec2::new(x, y), p)
        }
    }
}

impl<'a, T: PixelFormat> Clone for ImageView<'a, T> {
    fn clone(&self) -> Self {
        Self {
            width: self.width,
            height: self.height,
            width_f32: self.width_f32,
            height_f32: self.height_f32,
            data: self.data,
        }
    }
}

pub struct ImageViewMut<'a, T: PixelFormat> {
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) width_f32: f32,
    pub(crate) height_f32: f32,
    pub(crate) data: &'a mut [T::Storage],
}

impl<'a, T: PixelFormat> ImageViewMut<'a, T> {
    /// Constructs a new texture with the given dimensions.
    ///
    /// # Panic
    ///
    /// Will panic if 'data.len()' != 'width * height * PixelFormat::CHANNELS'.
    pub fn from_data(width: u32, height: u32, data: &'a mut [T::Storage]) -> Self {
        assert!(width <= 65535);
        assert!(height <= 65535);

        let expected_elements = ImageBuffer::<T>::calculate_element_count(width, height);
        assert_eq!(expected_elements, data.len());
        Self {
            width,
            height,
            width_f32: width as f32,
            height_f32: height as f32,
            data,
        }
    }

    /// Unwrap the image back to the raw buffer
    pub fn into_data(self) -> &'a mut [T::Storage] {
        self.data
    }

    /// Access the image's backing buffer as a flat array.
    #[inline]
    pub fn data(&self) -> &[T::Storage] {
        self.data
    }

    /// Access the image's backing buffer as a flat array.
    #[inline]
    pub fn data_mut(&mut self) -> &mut [T::Storage] {
        self.data
    }

    /// Run a function over all the pixels in an image
    #[inline]
    pub fn filter_pixels<F>(&self, mut f: F)
    where
        F: FnMut(UVec2, T),
    {
        let iter = self.data.chunks_exact(T::COMPONENTS).enumerate();
        for (i, p) in iter {
            let y = i as u32 / self.width;
            let x = i as u32 % self.width;

            let p = T::from_storage(p);
            f(UVec2::new(x, y), p)
        }
    }

    /// Run a function over all the pixels in an image. This is an alternate version of
    /// [`ImageBuffer::filter_pixels`] that produces new pixels from the input and will update the
    /// image in place with those new pixels.
    ///
    /// This can be thought of as a 1x1 filter.
    #[inline]
    pub fn filter_pixels_mut<F>(&mut self, mut f: F)
    where
        F: FnMut(UVec2, T) -> T,
    {
        let iter = self.data.chunks_exact_mut(T::COMPONENTS).enumerate();
        for (i, pb) in iter {
            let y = i as u32 / self.width;
            let x = i as u32 % self.width;

            let p = T::from_storage(pb);
            let p = f(UVec2::new(x, y), p);
            T::write_at(&p, pb);
        }
    }

    /// Converts the image buffer into little endian byte order in-place.
    ///
    /// This is a no-op on little endian platforms. On big-endian platforms a byte order swap will
    /// occur.
    pub fn to_little_endian(&mut self) {
        if !cfg!(target_endian = "big") {
            return;
        }

        for p in self.data.iter_mut() {
            p.convert_to_le_inplace();
        }
    }
}
