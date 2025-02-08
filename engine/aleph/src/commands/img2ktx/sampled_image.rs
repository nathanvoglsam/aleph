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

use aleph_math::{UVec2, Vec2, Vec4};
use image::{Luma, LumaA, Rgb, Rgba};

use crate::commands::img2ktx::{unorm_u16_to_f32, unorm_u8_to_f32};

/// Trait that closes over some image like object that enables sampling the image into a floating
/// point format.
///
/// This is very similar to the abstraction found in Vulkan/D3D/OpenGL/etc. The pixel format is
/// opaque and presented uniformly as a float. Noramlized formats encode in the 0-1 range, other
/// formats can range wider. The pixel is always swizzled up to 4 components regardless of the
/// underlying format.
pub trait ISampledImage: Sized {
    fn width(&self) -> u32;

    fn height(&self) -> u32;

    fn dimensions(&self) -> UVec2 {
        UVec2::new(self.width(), self.height())
    }

    fn get_pixel(&self, x: u32, y: u32) -> Vec4;

    fn get_sampler(&self) -> ImageSampler<Self> {
        ImageSampler::new(self)
    }
}

impl ISampledImage for image::ImageBuffer<Luma<u8>, Vec<u8>> {
    fn width(&self) -> u32 {
        Self::width(self)
    }

    fn height(&self) -> u32 {
        Self::height(self)
    }

    fn get_pixel(&self, x: u32, y: u32) -> Vec4 {
        let p = Self::get_pixel(self, x, y);
        let r = unorm_u8_to_f32(p.0[0]);
        let g = 0.0;
        let b = 0.0;
        let a = 0.0;
        Vec4::new(r, g, b, a)
    }
}

impl ISampledImage for image::ImageBuffer<LumaA<u8>, Vec<u8>> {
    fn width(&self) -> u32 {
        Self::width(self)
    }

    fn height(&self) -> u32 {
        Self::height(self)
    }

    fn get_pixel(&self, x: u32, y: u32) -> Vec4 {
        let p = Self::get_pixel(self, x, y);
        let r = unorm_u8_to_f32(p.0[0]);
        let g = unorm_u8_to_f32(p.0[1]);
        let b = 0.0;
        let a = 0.0;
        Vec4::new(r, g, b, a)
    }
}

impl ISampledImage for image::ImageBuffer<Rgb<u8>, Vec<u8>> {
    fn width(&self) -> u32 {
        Self::width(self)
    }

    fn height(&self) -> u32 {
        Self::height(self)
    }

    fn get_pixel(&self, x: u32, y: u32) -> Vec4 {
        let p = Self::get_pixel(self, x, y);
        let r = unorm_u8_to_f32(p.0[0]);
        let g = unorm_u8_to_f32(p.0[1]);
        let b = unorm_u8_to_f32(p.0[2]);
        let a = 0.0;
        Vec4::new(r, g, b, a)
    }
}

impl ISampledImage for image::ImageBuffer<Rgba<u8>, Vec<u8>> {
    fn width(&self) -> u32 {
        Self::width(self)
    }

    fn height(&self) -> u32 {
        Self::height(self)
    }

    fn get_pixel(&self, x: u32, y: u32) -> Vec4 {
        let p = Self::get_pixel(self, x, y);
        let r = unorm_u8_to_f32(p.0[0]);
        let g = unorm_u8_to_f32(p.0[1]);
        let b = unorm_u8_to_f32(p.0[2]);
        let a = unorm_u8_to_f32(p.0[3]);
        Vec4::new(r, g, b, a)
    }
}

impl ISampledImage for image::ImageBuffer<Luma<u16>, Vec<u16>> {
    fn width(&self) -> u32 {
        Self::width(self)
    }

    fn height(&self) -> u32 {
        Self::height(self)
    }

    fn get_pixel(&self, x: u32, y: u32) -> Vec4 {
        let p = Self::get_pixel(self, x, y);
        let r = unorm_u16_to_f32(p.0[0]);
        let g = 0.0;
        let b = 0.0;
        let a = 0.0;
        Vec4::new(r, g, b, a)
    }
}

impl ISampledImage for image::ImageBuffer<LumaA<u16>, Vec<u16>> {
    fn width(&self) -> u32 {
        Self::width(self)
    }

    fn height(&self) -> u32 {
        Self::height(self)
    }

    fn get_pixel(&self, x: u32, y: u32) -> Vec4 {
        let p = Self::get_pixel(self, x, y);
        let r = unorm_u16_to_f32(p.0[0]);
        let g = unorm_u16_to_f32(p.0[1]);
        let b = 0.0;
        let a = 0.0;
        Vec4::new(r, g, b, a)
    }
}

impl ISampledImage for image::ImageBuffer<Rgb<u16>, Vec<u16>> {
    fn width(&self) -> u32 {
        Self::width(self)
    }

    fn height(&self) -> u32 {
        Self::height(self)
    }

    fn get_pixel(&self, x: u32, y: u32) -> Vec4 {
        let p = Self::get_pixel(self, x, y);
        let r = unorm_u16_to_f32(p.0[0]);
        let g = unorm_u16_to_f32(p.0[1]);
        let b = unorm_u16_to_f32(p.0[2]);
        let a = 0.0;
        Vec4::new(r, g, b, a)
    }
}

impl ISampledImage for image::ImageBuffer<Rgba<u16>, Vec<u16>> {
    fn width(&self) -> u32 {
        Self::width(self)
    }

    fn height(&self) -> u32 {
        Self::height(self)
    }

    fn get_pixel(&self, x: u32, y: u32) -> Vec4 {
        let p = Self::get_pixel(self, x, y);
        let r = unorm_u16_to_f32(p.0[0]);
        let g = unorm_u16_to_f32(p.0[1]);
        let b = unorm_u16_to_f32(p.0[2]);
        let a = unorm_u16_to_f32(p.0[3]);
        Vec4::new(r, g, b, a)
    }
}

impl ISampledImage for image::ImageBuffer<Rgb<f32>, Vec<f32>> {
    fn width(&self) -> u32 {
        Self::width(self)
    }

    fn height(&self) -> u32 {
        Self::height(self)
    }

    fn get_pixel(&self, x: u32, y: u32) -> Vec4 {
        let p = Self::get_pixel(self, x, y);
        let r = p.0[0];
        let g = p.0[1];
        let b = p.0[2];
        let a = 0.0;
        Vec4::new(r, g, b, a)
    }
}

impl ISampledImage for image::ImageBuffer<Rgba<f32>, Vec<f32>> {
    fn width(&self) -> u32 {
        Self::width(self)
    }

    fn height(&self) -> u32 {
        Self::height(self)
    }

    fn get_pixel(&self, x: u32, y: u32) -> Vec4 {
        let p = Self::get_pixel(self, x, y);
        let r = p.0[0];
        let g = p.0[1];
        let b = p.0[2];
        let a = p.0[3];
        Vec4::new(r, g, b, a)
    }
}

pub struct ImageSampler<'a, T> {
    image: &'a T,
    dimensions_f32: Vec2,
}

impl<'a, T: ISampledImage> ImageSampler<'a, T> {
    pub fn new(image: &'a T) -> Self {
        let dimensions_int = image.dimensions();
        let dimensions_f32 = Vec2::new(dimensions_int.x as f32, dimensions_int.y as f32);
        Self {
            image,
            dimensions_f32,
        }
    }

    pub fn sample_image(&self, uv: Vec2) -> Vec4 {
        let (t_floor_u, t_ceil_u, w_floor_u, w_ceil_u) =
            texel_coord_to_sample_pos_and_weights(self.dimensions_f32.x, uv.x);
        let (t_floor_v, t_ceil_v, w_floor_v, w_ceil_v) =
            texel_coord_to_sample_pos_and_weights(self.dimensions_f32.y, uv.y);

        let sample_0 = self.image.get_pixel(t_floor_u as u32, t_floor_v as u32);
        let sample_0 = sample_0 * w_floor_u * w_floor_v;

        let sample_1 = self.image.get_pixel(t_floor_u as u32, t_ceil_v as u32);
        let sample_1 = sample_1 * w_floor_u * w_ceil_v;

        let sample_2 = self.image.get_pixel(t_ceil_u as u32, t_floor_v as u32);
        let sample_2 = sample_2 * w_ceil_u * w_floor_v;

        let sample_3 = self.image.get_pixel(t_ceil_u as u32, t_ceil_v as u32);
        let sample_3 = sample_3 * w_ceil_u * w_ceil_v;

        sample_0 + sample_1 + sample_2 + sample_3
    }
}

fn texel_coord_to_sample_pos_and_weights(dim: f32, x: f32) -> (f32, f32, f32, f32) {
    let x = reduce_range_wrap(x);
    let scaled = x * dim - 0.5;

    let floor = scaled.floor();
    let floor = sample_address_wrap(dim, floor);

    let ceil = floor + 1.0;
    let ceil = sample_address_wrap(dim, ceil);

    let ceil_w = scaled.fract();
    let floor_w = 1.0 - ceil_w;

    (floor, ceil, floor_w, ceil_w)
}

fn reduce_range_wrap(x: f32) -> f32 {
    x.fract()
}

fn sample_address_wrap(dim: f32, x: f32) -> f32 {
    let x = x % dim;
    if x < 0.0 {
        x + dim
    } else {
        x
    }
}
