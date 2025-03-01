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

use image::{Luma, LumaA, Pixel, Primitive, Rgb, Rgba};

use crate::{ImageBuffer, PixR, PixRG, PixRGB, PixRGBA, PixelChannelType};

#[derive(Clone, Copy, Debug, PartialEq, Hash)]
pub enum ResizeFilter {
    /// Nearest Neighbor
    Nearest,

    /// Linear Filter
    Linear,

    /// Cubic Filter
    Cubic,

    /// Gaussian Filter
    Gaussian,

    /// Lanczos with window 3
    Lanczos3,
}

impl ResizeFilter {
    const fn to_image_crate(&self) -> image::imageops::FilterType {
        match self {
            ResizeFilter::Nearest => image::imageops::FilterType::Nearest,
            ResizeFilter::Linear => image::imageops::FilterType::Triangle,
            ResizeFilter::Cubic => image::imageops::FilterType::CatmullRom,
            ResizeFilter::Gaussian => image::imageops::FilterType::Gaussian,
            ResizeFilter::Lanczos3 => image::imageops::FilterType::Lanczos3,
        }
    }
}

pub trait IResizeImage {
    fn resize(&self, new_x: u32, new_y: u32, filter: ResizeFilter) -> Self;
}

impl<P: PixelChannelType + Primitive + 'static> IResizeImage for ImageBuffer<PixR<P>> {
    fn resize(&self, new_x: u32, new_y: u32, filter: ResizeFilter) -> Self {
        let buf = self.data.as_slice();
        let img =
            image::ImageBuffer::<Luma<P>, &[P]>::from_raw(self.width, self.height, buf).unwrap();
        let new = image::imageops::resize(&img, new_x, new_y, filter.to_image_crate());
        let new = new.into_vec();
        Self::from_data(new_x, new_y, new)
    }
}

impl<P: PixelChannelType + Primitive + 'static> IResizeImage for ImageBuffer<PixRG<P>> {
    fn resize(&self, new_x: u32, new_y: u32, filter: ResizeFilter) -> Self {
        let buf = self.data.as_slice();
        let img =
            image::ImageBuffer::<LumaA<P>, &[P]>::from_raw(self.width, self.height, buf).unwrap();
        let new = image::imageops::resize(&img, new_x, new_y, filter.to_image_crate());
        let new = new.into_vec();
        Self::from_data(new_x, new_y, new)
    }
}

impl<P> IResizeImage for ImageBuffer<PixRGB<P>>
where
    Rgb<P>: Pixel<Subpixel = P>,
    P: PixelChannelType + 'static,
{
    fn resize(&self, new_x: u32, new_y: u32, filter: ResizeFilter) -> Self {
        let buf = self.data.as_slice();
        let img =
            image::ImageBuffer::<Rgb<P>, &[P]>::from_raw(self.width, self.height, buf).unwrap();
        let new = image::imageops::resize(&img, new_x, new_y, filter.to_image_crate());
        let new = new.into_vec();
        Self::from_data(new_x, new_y, new)
    }
}

impl<P> IResizeImage for ImageBuffer<PixRGBA<P>>
where
    Rgba<P>: Pixel<Subpixel = P>,
    P: PixelChannelType + 'static,
{
    fn resize(&self, new_x: u32, new_y: u32, filter: ResizeFilter) -> Self {
        let buf = self.data.as_slice();
        let img =
            image::ImageBuffer::<Rgba<P>, &[P]>::from_raw(self.width, self.height, buf).unwrap();
        let new = image::imageops::resize(&img, new_x, new_y, filter.to_image_crate());
        let new = new.into_vec();
        Self::from_data(new_x, new_y, new)
    }
}
