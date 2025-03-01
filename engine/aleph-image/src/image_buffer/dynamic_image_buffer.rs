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

use half::f16;

use crate::{IPixelStorage, IResizeImage, ImageBuffer, PixR, PixRG, PixRGB, PixRGBA};

#[derive(Clone)]
pub enum DynamicImageBuffer {
    R8Unorm(ImageBuffer<PixR<u8>>),
    RG8Unorm(ImageBuffer<PixRG<u8>>),
    RGB8Unorm(ImageBuffer<PixRGB<u8>>),
    RGBA8Unorm(ImageBuffer<PixRGBA<u8>>),
    R16Unorm(ImageBuffer<PixR<u16>>),
    RG16Unorm(ImageBuffer<PixRG<u16>>),
    RGB16Unorm(ImageBuffer<PixRGB<u16>>),
    RGBA16Unorm(ImageBuffer<PixRGBA<u16>>),
    R32Unorm(ImageBuffer<PixR<u32>>),
    RG32Unorm(ImageBuffer<PixRG<u32>>),
    RGB32Unorm(ImageBuffer<PixRGB<u32>>),
    RGBA32Unorm(ImageBuffer<PixRGBA<u32>>),
    R16Float(ImageBuffer<PixR<f16>>),
    RG16Float(ImageBuffer<PixRG<f16>>),
    RGB16Float(ImageBuffer<PixRGB<f16>>),
    RGBA16Float(ImageBuffer<PixRGBA<f16>>),
    R32Float(ImageBuffer<PixR<f32>>),
    RG32Float(ImageBuffer<PixRG<f32>>),
    RGB32Float(ImageBuffer<PixRGB<f32>>),
    RGBA32Float(ImageBuffer<PixRGBA<f32>>),
}

impl DynamicImageBuffer {
    pub const fn color_type(&self) -> ColorType {
        match self {
            DynamicImageBuffer::R8Unorm(_) => ColorType::R8Unorm,
            DynamicImageBuffer::RG8Unorm(_) => ColorType::RG8Unorm,
            DynamicImageBuffer::RGB8Unorm(_) => ColorType::RGB8Unorm,
            DynamicImageBuffer::RGBA8Unorm(_) => ColorType::RGBA8Unorm,
            DynamicImageBuffer::R16Unorm(_) => ColorType::R16Unorm,
            DynamicImageBuffer::RG16Unorm(_) => ColorType::RG16Unorm,
            DynamicImageBuffer::RGB16Unorm(_) => ColorType::RGB16Unorm,
            DynamicImageBuffer::RGBA16Unorm(_) => ColorType::RGBA16Unorm,
            DynamicImageBuffer::R32Unorm(_) => ColorType::R32Unorm,
            DynamicImageBuffer::RG32Unorm(_) => ColorType::RG32Unorm,
            DynamicImageBuffer::RGB32Unorm(_) => ColorType::RGB32Unorm,
            DynamicImageBuffer::RGBA32Unorm(_) => ColorType::RGBA32Unorm,
            DynamicImageBuffer::R16Float(_) => ColorType::R16Float,
            DynamicImageBuffer::RG16Float(_) => ColorType::RG16Float,
            DynamicImageBuffer::RGB16Float(_) => ColorType::RGB16Float,
            DynamicImageBuffer::RGBA16Float(_) => ColorType::RGBA16Float,
            DynamicImageBuffer::R32Float(_) => ColorType::R32Float,
            DynamicImageBuffer::RG32Float(_) => ColorType::RG32Float,
            DynamicImageBuffer::RGB32Float(_) => ColorType::RGB32Float,
            DynamicImageBuffer::RGBA32Float(_) => ColorType::RGBA32Float,
        }
    }

    pub fn from_image(image: image::DynamicImage) -> Self {
        match image {
            image::DynamicImage::ImageLuma8(i) => Self::R8Unorm(ImageBuffer::from_image(i)),
            image::DynamicImage::ImageLumaA8(i) => Self::RG8Unorm(ImageBuffer::from_image(i)),
            image::DynamicImage::ImageRgb8(i) => Self::RGB8Unorm(ImageBuffer::from_image(i)),
            image::DynamicImage::ImageRgba8(i) => Self::RGBA8Unorm(ImageBuffer::from_image(i)),
            image::DynamicImage::ImageLuma16(i) => Self::R16Unorm(ImageBuffer::from_image(i)),
            image::DynamicImage::ImageLumaA16(i) => Self::RG16Unorm(ImageBuffer::from_image(i)),
            image::DynamicImage::ImageRgb16(i) => Self::RGB16Unorm(ImageBuffer::from_image(i)),
            image::DynamicImage::ImageRgba16(i) => Self::RGBA16Unorm(ImageBuffer::from_image(i)),
            image::DynamicImage::ImageRgb32F(i) => Self::RGB32Float(ImageBuffer::from_image(i)),
            image::DynamicImage::ImageRgba32F(i) => Self::RGBA32Float(ImageBuffer::from_image(i)),
            _ => unimplemented!(),
        }
    }

    pub fn to_little_endian(&mut self) {
        match self {
            DynamicImageBuffer::R8Unorm(i) => i.to_little_endian(),
            DynamicImageBuffer::RG8Unorm(i) => i.to_little_endian(),
            DynamicImageBuffer::RGB8Unorm(i) => i.to_little_endian(),
            DynamicImageBuffer::RGBA8Unorm(i) => i.to_little_endian(),
            DynamicImageBuffer::R16Unorm(i) => i.to_little_endian(),
            DynamicImageBuffer::RG16Unorm(i) => i.to_little_endian(),
            DynamicImageBuffer::RGB16Unorm(i) => i.to_little_endian(),
            DynamicImageBuffer::RGBA16Unorm(i) => i.to_little_endian(),
            DynamicImageBuffer::R32Unorm(i) => i.to_little_endian(),
            DynamicImageBuffer::RG32Unorm(i) => i.to_little_endian(),
            DynamicImageBuffer::RGB32Unorm(i) => i.to_little_endian(),
            DynamicImageBuffer::RGBA32Unorm(i) => i.to_little_endian(),
            DynamicImageBuffer::R16Float(i) => i.to_little_endian(),
            DynamicImageBuffer::RG16Float(i) => i.to_little_endian(),
            DynamicImageBuffer::RGB16Float(i) => i.to_little_endian(),
            DynamicImageBuffer::RGBA16Float(i) => i.to_little_endian(),
            DynamicImageBuffer::R32Float(i) => i.to_little_endian(),
            DynamicImageBuffer::RG32Float(i) => i.to_little_endian(),
            DynamicImageBuffer::RGB32Float(i) => i.to_little_endian(),
            DynamicImageBuffer::RGBA32Float(i) => i.to_little_endian(),
        }
    }

    pub fn to_half(&mut self) {
        *self = match self {
            DynamicImageBuffer::R8Unorm(i) => Self::R16Float(i.to_half()),
            DynamicImageBuffer::RG8Unorm(i) => Self::RG16Float(i.to_half()),
            DynamicImageBuffer::RGB8Unorm(i) => Self::RGB16Float(i.to_half()),
            DynamicImageBuffer::RGBA8Unorm(i) => Self::RGBA16Float(i.to_half()),
            DynamicImageBuffer::R16Unorm(i) => Self::R16Float(i.to_half()),
            DynamicImageBuffer::RG16Unorm(i) => Self::RG16Float(i.to_half()),
            DynamicImageBuffer::RGB16Unorm(i) => Self::RGB16Float(i.to_half()),
            DynamicImageBuffer::RGBA16Unorm(i) => Self::RGBA16Float(i.to_half()),
            DynamicImageBuffer::R32Unorm(i) => Self::R16Float(i.to_half()),
            DynamicImageBuffer::RG32Unorm(i) => Self::RG16Float(i.to_half()),
            DynamicImageBuffer::RGB32Unorm(i) => Self::RGB16Float(i.to_half()),
            DynamicImageBuffer::RGBA32Unorm(i) => Self::RGBA16Float(i.to_half()),
            DynamicImageBuffer::R16Float(i) => Self::R16Float(i.to_half()),
            DynamicImageBuffer::RG16Float(i) => Self::RG16Float(i.to_half()),
            DynamicImageBuffer::RGB16Float(i) => Self::RGB16Float(i.to_half()),
            DynamicImageBuffer::RGBA16Float(i) => Self::RGBA16Float(i.to_half()),
            DynamicImageBuffer::R32Float(i) => Self::R16Float(i.to_half()),
            DynamicImageBuffer::RG32Float(i) => Self::RG16Float(i.to_half()),
            DynamicImageBuffer::RGB32Float(i) => Self::RGB16Float(i.to_half()),
            DynamicImageBuffer::RGBA32Float(i) => Self::RGBA16Float(i.to_half()),
        }
    }
}

impl IPixelStorage for DynamicImageBuffer {
    fn dimensions(&self) -> aleph_math::UVec2 {
        match self {
            DynamicImageBuffer::R8Unorm(i) => i.dimensions(),
            DynamicImageBuffer::RG8Unorm(i) => i.dimensions(),
            DynamicImageBuffer::RGB8Unorm(i) => i.dimensions(),
            DynamicImageBuffer::RGBA8Unorm(i) => i.dimensions(),
            DynamicImageBuffer::R16Unorm(i) => i.dimensions(),
            DynamicImageBuffer::RG16Unorm(i) => i.dimensions(),
            DynamicImageBuffer::RGB16Unorm(i) => i.dimensions(),
            DynamicImageBuffer::RGBA16Unorm(i) => i.dimensions(),
            DynamicImageBuffer::R32Unorm(i) => i.dimensions(),
            DynamicImageBuffer::RG32Unorm(i) => i.dimensions(),
            DynamicImageBuffer::RGB32Unorm(i) => i.dimensions(),
            DynamicImageBuffer::RGBA32Unorm(i) => i.dimensions(),
            DynamicImageBuffer::R16Float(i) => i.dimensions(),
            DynamicImageBuffer::RG16Float(i) => i.dimensions(),
            DynamicImageBuffer::RGB16Float(i) => i.dimensions(),
            DynamicImageBuffer::RGBA16Float(i) => i.dimensions(),
            DynamicImageBuffer::R32Float(i) => i.dimensions(),
            DynamicImageBuffer::RG32Float(i) => i.dimensions(),
            DynamicImageBuffer::RGB32Float(i) => i.dimensions(),
            DynamicImageBuffer::RGBA32Float(i) => i.dimensions(),
        }
    }

    fn dimensions_f32(&self) -> aleph_math::Vec2 {
        match self {
            DynamicImageBuffer::R8Unorm(i) => i.dimensions_f32(),
            DynamicImageBuffer::RG8Unorm(i) => i.dimensions_f32(),
            DynamicImageBuffer::RGB8Unorm(i) => i.dimensions_f32(),
            DynamicImageBuffer::RGBA8Unorm(i) => i.dimensions_f32(),
            DynamicImageBuffer::R16Unorm(i) => i.dimensions_f32(),
            DynamicImageBuffer::RG16Unorm(i) => i.dimensions_f32(),
            DynamicImageBuffer::RGB16Unorm(i) => i.dimensions_f32(),
            DynamicImageBuffer::RGBA16Unorm(i) => i.dimensions_f32(),
            DynamicImageBuffer::R32Unorm(i) => i.dimensions_f32(),
            DynamicImageBuffer::RG32Unorm(i) => i.dimensions_f32(),
            DynamicImageBuffer::RGB32Unorm(i) => i.dimensions_f32(),
            DynamicImageBuffer::RGBA32Unorm(i) => i.dimensions_f32(),
            DynamicImageBuffer::R16Float(i) => i.dimensions_f32(),
            DynamicImageBuffer::RG16Float(i) => i.dimensions_f32(),
            DynamicImageBuffer::RGB16Float(i) => i.dimensions_f32(),
            DynamicImageBuffer::RGBA16Float(i) => i.dimensions_f32(),
            DynamicImageBuffer::R32Float(i) => i.dimensions_f32(),
            DynamicImageBuffer::RG32Float(i) => i.dimensions_f32(),
            DynamicImageBuffer::RGB32Float(i) => i.dimensions_f32(),
            DynamicImageBuffer::RGBA32Float(i) => i.dimensions_f32(),
        }
    }
}

impl IResizeImage for DynamicImageBuffer {
    fn resize(&self, new_x: u32, new_y: u32, filter: crate::ResizeFilter) -> Self {
        match self {
            DynamicImageBuffer::R8Unorm(i) => {
                DynamicImageBuffer::R8Unorm(i.resize(new_x, new_y, filter))
            }
            DynamicImageBuffer::RG8Unorm(i) => {
                DynamicImageBuffer::RG8Unorm(i.resize(new_x, new_y, filter))
            }
            DynamicImageBuffer::RGB8Unorm(i) => {
                DynamicImageBuffer::RGB8Unorm(i.resize(new_x, new_y, filter))
            }
            DynamicImageBuffer::RGBA8Unorm(i) => {
                DynamicImageBuffer::RGBA8Unorm(i.resize(new_x, new_y, filter))
            }
            DynamicImageBuffer::R16Unorm(i) => {
                DynamicImageBuffer::R16Unorm(i.resize(new_x, new_y, filter))
            }
            DynamicImageBuffer::RG16Unorm(i) => {
                DynamicImageBuffer::RG16Unorm(i.resize(new_x, new_y, filter))
            }
            DynamicImageBuffer::RGB16Unorm(i) => {
                DynamicImageBuffer::RGB16Unorm(i.resize(new_x, new_y, filter))
            }
            DynamicImageBuffer::RGBA16Unorm(i) => {
                DynamicImageBuffer::RGBA16Unorm(i.resize(new_x, new_y, filter))
            }
            DynamicImageBuffer::R32Unorm(i) => {
                DynamicImageBuffer::R32Unorm(i.resize(new_x, new_y, filter))
            }
            DynamicImageBuffer::RG32Unorm(i) => {
                DynamicImageBuffer::RG32Unorm(i.resize(new_x, new_y, filter))
            }
            DynamicImageBuffer::RGB32Unorm(i) => {
                DynamicImageBuffer::RGB32Unorm(i.resize(new_x, new_y, filter))
            }
            DynamicImageBuffer::RGBA32Unorm(i) => {
                DynamicImageBuffer::RGBA32Unorm(i.resize(new_x, new_y, filter))
            }
            DynamicImageBuffer::R16Float(_i) => {
                unimplemented!()
                // DynamicImageBuffer::R16Float(i.resize(new_x, new_y, filter))
            }
            DynamicImageBuffer::RG16Float(_i) => {
                unimplemented!()
                // DynamicImageBuffer::RG16Float(i.resize(new_x, new_y, filter))
            }
            DynamicImageBuffer::RGB16Float(_i) => {
                unimplemented!()
                // DynamicImageBuffer::RGB16Float(i.resize(new_x, new_y, filter))
            }
            DynamicImageBuffer::RGBA16Float(_i) => {
                unimplemented!()
                // DynamicImageBuffer::RGBA16Float(i.resize(new_x, new_y, filter))
            }
            DynamicImageBuffer::R32Float(i) => {
                DynamicImageBuffer::R32Float(i.resize(new_x, new_y, filter))
            }
            DynamicImageBuffer::RG32Float(i) => {
                DynamicImageBuffer::RG32Float(i.resize(new_x, new_y, filter))
            }
            DynamicImageBuffer::RGB32Float(i) => {
                DynamicImageBuffer::RGB32Float(i.resize(new_x, new_y, filter))
            }
            DynamicImageBuffer::RGBA32Float(i) => {
                DynamicImageBuffer::RGBA32Float(i.resize(new_x, new_y, filter))
            }
        }
    }
}

macro_rules! from_image_buffer_impl {
    ($x: path, $v: ident) => {
        impl ::core::convert::From<$x> for DynamicImageBuffer {
            fn from(value: $x) -> Self {
                Self::$v(value)
            }
        }
    };
}

from_image_buffer_impl!(ImageBuffer<PixR<u8>>, R8Unorm);
from_image_buffer_impl!(ImageBuffer<PixRG<u8>>, RG8Unorm);
from_image_buffer_impl!(ImageBuffer<PixRGB<u8>>, RGB8Unorm);
from_image_buffer_impl!(ImageBuffer<PixRGBA<u8>>, RGBA8Unorm);
from_image_buffer_impl!(ImageBuffer<PixR<u16>>, R16Unorm);
from_image_buffer_impl!(ImageBuffer<PixRG<u16>>, RG16Unorm);
from_image_buffer_impl!(ImageBuffer<PixRGB<u16>>, RGB16Unorm);
from_image_buffer_impl!(ImageBuffer<PixRGBA<u16>>, RGBA16Unorm);
from_image_buffer_impl!(ImageBuffer<PixR<u32>>, R32Unorm);
from_image_buffer_impl!(ImageBuffer<PixRG<u32>>, RG32Unorm);
from_image_buffer_impl!(ImageBuffer<PixRGB<u32>>, RGB32Unorm);
from_image_buffer_impl!(ImageBuffer<PixRGBA<u32>>, RGBA32Unorm);
from_image_buffer_impl!(ImageBuffer<PixR<f16>>, R16Float);
from_image_buffer_impl!(ImageBuffer<PixRG<f16>>, RG16Float);
from_image_buffer_impl!(ImageBuffer<PixRGB<f16>>, RGB16Float);
from_image_buffer_impl!(ImageBuffer<PixRGBA<f16>>, RGBA16Float);
from_image_buffer_impl!(ImageBuffer<PixR<f32>>, R32Float);
from_image_buffer_impl!(ImageBuffer<PixRG<f32>>, RG32Float);
from_image_buffer_impl!(ImageBuffer<PixRGB<f32>>, RGB32Float);
from_image_buffer_impl!(ImageBuffer<PixRGBA<f32>>, RGBA32Float);

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum ColorType {
    R8Unorm,
    RG8Unorm,
    RGB8Unorm,
    RGBA8Unorm,
    R16Unorm,
    RG16Unorm,
    RGB16Unorm,
    RGBA16Unorm,
    R32Unorm,
    RG32Unorm,
    RGB32Unorm,
    RGBA32Unorm,
    R16Float,
    RG16Float,
    RGB16Float,
    RGBA16Float,
    R32Float,
    RG32Float,
    RGB32Float,
    RGBA32Float,
}
