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

mod dynamic_image_buffer;

pub use dynamic_image_buffer::{ColorType, DynamicImageBuffer};

use half::f16;

use crate::{FromImagePixel, PixR, PixRG, PixRGB, PixRGBA, PixelChannelType, PixelFormat};

pub struct ImageBuffer<T: PixelFormat> {
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) width_f32: f32,
    pub(crate) height_f32: f32,
    pub(crate) data: Vec<T::Storage>,
}

impl<T: PixelFormat> ImageBuffer<T> {
    pub fn new(width: u32, height: u32) -> Self {
        assert!(width <= 65535);
        assert!(height <= 65535);

        let elements = Self::calculate_element_count(width, height);
        let mut data = Vec::with_capacity(elements);
        data.resize(elements, Default::default());

        Self {
            width,
            height,
            width_f32: width as f32,
            height_f32: height as f32,
            data,
        }
    }

    pub fn from_data(width: u32, height: u32, data: Vec<T::Storage>) -> Self {
        assert!(width <= 65535);
        assert!(height <= 65535);

        let expected_elements = Self::calculate_element_count(width, height);
        assert_eq!(expected_elements, data.len());
        Self {
            width,
            height,
            width_f32: width as f32,
            height_f32: height as f32,
            data,
        }
    }

    pub fn into_data(self) -> Vec<T::Storage> {
        self.data
    }

    #[inline]
    pub fn data(&self) -> &[T::Storage] {
        self.data.as_slice()
    }

    #[inline]
    pub fn data_mut(&mut self) -> &mut [T::Storage] {
        self.data.as_mut_slice()
    }

    pub const fn calculate_element_count(width: u32, height: u32) -> usize {
        let pixels = width as usize * height as usize;
        let elements = pixels * T::COMPONENTS;
        elements
    }

    pub fn to_little_endian(&mut self) {
        if !cfg!(target_endian = "big") {
            return;
        }

        for p in self.data.iter_mut() {
            p.to_le();
        }
    }
}

impl<P: PixelChannelType> ImageBuffer<PixR<P>> {
    pub fn to_half(&self) -> ImageBuffer<PixR<f16>> {
        let data = Vec::from_iter(self.data.iter().copied().map(|v| {
            const MIN: f32 = f16::MIN.to_f32_const();
            const MAX: f32 = f16::MAX.to_f32_const();

            let v = v.into_float();
            let v = v.clamp(MIN, MAX);
            let v = f16::from_f32(v);
            v
        }));
        ImageBuffer::from_data(self.width, self.height, data)
    }
}

impl<P: PixelChannelType> ImageBuffer<PixRG<P>> {
    pub fn to_half(&self) -> ImageBuffer<PixRG<f16>> {
        let data = Vec::from_iter(self.data.iter().copied().map(|v| {
            const MIN: f32 = f16::MIN.to_f32_const();
            const MAX: f32 = f16::MAX.to_f32_const();

            let v = v.into_float();
            let v = v.clamp(MIN, MAX);
            let v = f16::from_f32(v);
            v
        }));
        ImageBuffer::from_data(self.width, self.height, data)
    }
}

impl<P: PixelChannelType> ImageBuffer<PixRGB<P>> {
    pub fn to_half(&self) -> ImageBuffer<PixRGB<f16>> {
        let data = Vec::from_iter(self.data.iter().copied().map(|v| {
            const MIN: f32 = f16::MIN.to_f32_const();
            const MAX: f32 = f16::MAX.to_f32_const();

            let v = v.into_float();
            let v = v.clamp(MIN, MAX);
            let v = f16::from_f32(v);
            v
        }));
        ImageBuffer::from_data(self.width, self.height, data)
    }
}

impl<P: PixelChannelType> ImageBuffer<PixRGBA<P>> {
    pub fn to_half(&self) -> ImageBuffer<PixRGBA<f16>> {
        let data = Vec::from_iter(self.data.iter().copied().map(|v| {
            const MIN: f32 = f16::MIN.to_f32_const();
            const MAX: f32 = f16::MAX.to_f32_const();

            let v = v.into_float();
            let v = v.clamp(MIN, MAX);
            let v = f16::from_f32(v);
            v
        }));
        ImageBuffer::from_data(self.width, self.height, data)
    }
}

impl<T: FromImagePixel> ImageBuffer<T> {
    pub fn from_image(image: image::ImageBuffer<T::Source, Vec<T::Storage>>) -> Self {
        let width = image.width();
        let height = image.height();
        let data = image.into_vec();
        Self::from_data(width, height, data)
    }
}

impl<T: PixelFormat> Clone for ImageBuffer<T> {
    fn clone(&self) -> Self {
        Self {
            width: self.width.clone(),
            height: self.height.clone(),
            width_f32: self.width_f32.clone(),
            height_f32: self.height_f32.clone(),
            data: self.data.clone(),
        }
    }
}
