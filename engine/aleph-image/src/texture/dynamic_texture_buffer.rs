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
use half::f16;

use crate::texture::TextureType;
use crate::{
    ColorType, PixR, PixRG, PixRGB, PixRGBA, ResizeFilter, SphericalMapping, TextureBuffer,
    TextureOpError, TextureOpResult,
};

#[derive(Clone)]
pub enum DynamicTextureBuffer {
    R8Unorm(TextureBuffer<PixR<u8>>),
    RG8Unorm(TextureBuffer<PixRG<u8>>),
    RGB8Unorm(TextureBuffer<PixRGB<u8>>),
    RGBA8Unorm(TextureBuffer<PixRGBA<u8>>),
    R16Unorm(TextureBuffer<PixR<u16>>),
    RG16Unorm(TextureBuffer<PixRG<u16>>),
    RGB16Unorm(TextureBuffer<PixRGB<u16>>),
    RGBA16Unorm(TextureBuffer<PixRGBA<u16>>),
    R32Unorm(TextureBuffer<PixR<u32>>),
    RG32Unorm(TextureBuffer<PixRG<u32>>),
    RGB32Unorm(TextureBuffer<PixRGB<u32>>),
    RGBA32Unorm(TextureBuffer<PixRGBA<u32>>),
    R16Float(TextureBuffer<PixR<f16>>),
    RG16Float(TextureBuffer<PixRG<f16>>),
    RGB16Float(TextureBuffer<PixRGB<f16>>),
    RGBA16Float(TextureBuffer<PixRGBA<f16>>),
    R32Float(TextureBuffer<PixR<f32>>),
    RG32Float(TextureBuffer<PixRG<f32>>),
    RGB32Float(TextureBuffer<PixRGB<f32>>),
    RGBA32Float(TextureBuffer<PixRGBA<f32>>),
}

macro_rules! impl_for_all_variants {
    ($self: ident, $n: ident, $f: expr) => {
        match $self {
            Self::R8Unorm($n) => $f,
            Self::RG8Unorm($n) => $f,
            Self::RGB8Unorm($n) => $f,
            Self::RGBA8Unorm($n) => $f,
            Self::R16Unorm($n) => $f,
            Self::RG16Unorm($n) => $f,
            Self::RGB16Unorm($n) => $f,
            Self::RGBA16Unorm($n) => $f,
            Self::R32Unorm($n) => $f,
            Self::RG32Unorm($n) => $f,
            Self::RGB32Unorm($n) => $f,
            Self::RGBA32Unorm($n) => $f,
            Self::R16Float($n) => $f,
            Self::RG16Float($n) => $f,
            Self::RGB16Float($n) => $f,
            Self::RGBA16Float($n) => $f,
            Self::R32Float($n) => $f,
            Self::RG32Float($n) => $f,
            Self::RGB32Float($n) => $f,
            Self::RGBA32Float($n) => $f,
        }
    };
}

impl DynamicTextureBuffer {
    /// Returns the dimensions of the underlying texture. This encodes the size of mip 0. For cubes
    /// this encodes the size of mip 0 of each face.
    pub fn dimensions(&self) -> UVec2 {
        impl_for_all_variants!(self, n, n.dimensions())
    }

    pub fn layer_num(&self) -> u32 {
        impl_for_all_variants!(self, n, n.layer_num())
    }

    pub fn level_num(&self) -> u32 {
        impl_for_all_variants!(self, n, n.level_num())
    }

    pub fn get_color_type(&self) -> ColorType {
        match self {
            Self::R8Unorm(_) => ColorType::R8Unorm,
            Self::RG8Unorm(_) => ColorType::RG8Unorm,
            Self::RGB8Unorm(_) => ColorType::RGB8Unorm,
            Self::RGBA8Unorm(_) => ColorType::RGBA8Unorm,
            Self::R16Unorm(_) => ColorType::R16Unorm,
            Self::RG16Unorm(_) => ColorType::RG16Unorm,
            Self::RGB16Unorm(_) => ColorType::RGB16Unorm,
            Self::RGBA16Unorm(_) => ColorType::RGBA16Unorm,
            Self::R32Unorm(_) => ColorType::R32Unorm,
            Self::RG32Unorm(_) => ColorType::RG32Unorm,
            Self::RGB32Unorm(_) => ColorType::RGB32Unorm,
            Self::RGBA32Unorm(_) => ColorType::RGBA32Unorm,
            Self::R16Float(_) => ColorType::R16Float,
            Self::RG16Float(_) => ColorType::RG16Float,
            Self::RGB16Float(_) => ColorType::RGB16Float,
            Self::RGBA16Float(_) => ColorType::RGBA16Float,
            Self::R32Float(_) => ColorType::R32Float,
            Self::RG32Float(_) => ColorType::RG32Float,
            Self::RGB32Float(_) => ColorType::RGB32Float,
            Self::RGBA32Float(_) => ColorType::RGBA32Float,
        }
    }

    pub const fn get_texture_type(&self) -> TextureType {
        impl_for_all_variants!(self, n, n.get_texture_type())
    }

    pub fn validate_image_count(&self) {
        impl_for_all_variants!(self, n, n.validate_image_count());
    }

    pub fn get_buffer_references(&self) -> Vec<&[u8]> {
        impl_for_all_variants!(self, n, n.get_buffer_references())
    }

    pub fn generate_mips(&mut self, filter: ResizeFilter) -> TextureOpResult<()> {
        match self {
            Self::R8Unorm(n) => n.generate_mips(filter),
            Self::RG8Unorm(n) => n.generate_mips(filter),
            Self::RGB8Unorm(n) => n.generate_mips(filter),
            Self::RGBA8Unorm(n) => n.generate_mips(filter),
            Self::R16Unorm(n) => n.generate_mips(filter),
            Self::RG16Unorm(n) => n.generate_mips(filter),
            Self::RGB16Unorm(n) => n.generate_mips(filter),
            Self::RGBA16Unorm(n) => n.generate_mips(filter),
            Self::R32Unorm(n) => n.generate_mips(filter),
            Self::RG32Unorm(n) => n.generate_mips(filter),
            Self::RGB32Unorm(n) => n.generate_mips(filter),
            Self::RGBA32Unorm(n) => n.generate_mips(filter),
            Self::R16Float(_) => return Err(TextureOpError::InvalidSrcFormat),
            Self::RG16Float(_) => return Err(TextureOpError::InvalidSrcFormat),
            Self::RGB16Float(_) => return Err(TextureOpError::InvalidSrcFormat),
            Self::RGBA16Float(_) => return Err(TextureOpError::InvalidSrcFormat),
            Self::R32Float(n) => n.generate_mips(filter),
            Self::RG32Float(n) => n.generate_mips(filter),
            Self::RGB32Float(n) => n.generate_mips(filter),
            Self::RGBA32Float(n) => n.generate_mips(filter),
        }
        Ok(())
    }

    pub fn normalize(&mut self) -> TextureOpResult<()> {
        match self {
            Self::R8Unorm(_) => return Err(TextureOpError::InvalidSrcFormat),
            Self::RG8Unorm(_) => return Err(TextureOpError::InvalidSrcFormat),
            Self::RGB8Unorm(n) => n.normalize(),
            Self::RGBA8Unorm(_) => return Err(TextureOpError::InvalidSrcFormat),
            Self::R16Unorm(_) => return Err(TextureOpError::InvalidSrcFormat),
            Self::RG16Unorm(_) => return Err(TextureOpError::InvalidSrcFormat),
            Self::RGB16Unorm(n) => n.normalize(),
            Self::RGBA16Unorm(_) => return Err(TextureOpError::InvalidSrcFormat),
            Self::R32Unorm(_) => return Err(TextureOpError::InvalidSrcFormat),
            Self::RG32Unorm(_) => return Err(TextureOpError::InvalidSrcFormat),
            Self::RGB32Unorm(n) => n.normalize(),
            Self::RGBA32Unorm(_) => return Err(TextureOpError::InvalidSrcFormat),
            Self::R16Float(_) => return Err(TextureOpError::InvalidSrcFormat),
            Self::RG16Float(_) => return Err(TextureOpError::InvalidSrcFormat),
            Self::RGB16Float(n) => n.normalize(),
            Self::RGBA16Float(_) => return Err(TextureOpError::InvalidSrcFormat),
            Self::R32Float(_) => return Err(TextureOpError::InvalidSrcFormat),
            Self::RG32Float(_) => return Err(TextureOpError::InvalidSrcFormat),
            Self::RGB32Float(n) => n.normalize(),
            Self::RGBA32Float(_) => return Err(TextureOpError::InvalidSrcFormat),
        }
        Ok(())
    }

    pub fn to_half(&self) -> TextureOpResult<DynamicTextureBuffer> {
        let out = match self {
            Self::R8Unorm(n) => Self::R16Float(n.to_half()),
            Self::RG8Unorm(n) => Self::RG16Float(n.to_half()),
            Self::RGB8Unorm(n) => Self::RGB16Float(n.to_half()),
            Self::RGBA8Unorm(n) => Self::RGBA16Float(n.to_half()),
            Self::R16Unorm(n) => Self::R16Float(n.to_half()),
            Self::RG16Unorm(n) => Self::RG16Float(n.to_half()),
            Self::RGB16Unorm(n) => Self::RGB16Float(n.to_half()),
            Self::RGBA16Unorm(n) => Self::RGBA16Float(n.to_half()),
            Self::R32Unorm(n) => Self::R16Float(n.to_half()),
            Self::RG32Unorm(n) => Self::RG16Float(n.to_half()),
            Self::RGB32Unorm(n) => Self::RGB16Float(n.to_half()),
            Self::RGBA32Unorm(n) => Self::RGBA16Float(n.to_half()),
            Self::R16Float(n) => Self::R16Float(n.to_half()),
            Self::RG16Float(n) => Self::RG16Float(n.to_half()),
            Self::RGB16Float(n) => Self::RGB16Float(n.to_half()),
            Self::RGBA16Float(n) => Self::RGBA16Float(n.to_half()),
            Self::R32Float(n) => Self::R16Float(n.to_half()),
            Self::RG32Float(n) => Self::RG16Float(n.to_half()),
            Self::RGB32Float(n) => Self::RGB16Float(n.to_half()),
            Self::RGBA32Float(n) => Self::RGBA16Float(n.to_half()),
        };
        Ok(out)
    }

    pub fn to_little_endian(&mut self) -> TextureOpResult<()> {
        match self {
            Self::R8Unorm(n) => n.to_little_endian(),
            Self::RG8Unorm(n) => n.to_little_endian(),
            Self::RGB8Unorm(n) => n.to_little_endian(),
            Self::RGBA8Unorm(n) => n.to_little_endian(),
            Self::R16Unorm(n) => n.to_little_endian(),
            Self::RG16Unorm(n) => n.to_little_endian(),
            Self::RGB16Unorm(n) => n.to_little_endian(),
            Self::RGBA16Unorm(n) => n.to_little_endian(),
            Self::R32Unorm(n) => n.to_little_endian(),
            Self::RG32Unorm(n) => n.to_little_endian(),
            Self::RGB32Unorm(n) => n.to_little_endian(),
            Self::RGBA32Unorm(n) => n.to_little_endian(),
            Self::R16Float(n) => n.to_little_endian(),
            Self::RG16Float(n) => n.to_little_endian(),
            Self::RGB16Float(n) => n.to_little_endian(),
            Self::RGBA16Float(n) => n.to_little_endian(),
            Self::R32Float(n) => n.to_little_endian(),
            Self::RG32Float(n) => n.to_little_endian(),
            Self::RGB32Float(n) => n.to_little_endian(),
            Self::RGBA32Float(n) => n.to_little_endian(),
        }
        Ok(())
    }

    pub fn spherical_map_to_cube_map(
        &mut self,
        mapping_2d: SphericalMapping,
        face_dimensions: UVec2,
    ) -> TextureOpResult<()> {
        impl_for_all_variants!(
            self,
            n,
            n.spherical_map_to_cube_map(mapping_2d, face_dimensions)
        )
    }

    pub fn spherical_map_to_equirectangular_map(
        &mut self,
        mapping_2d: SphericalMapping,
        face_dimensions: UVec2,
    ) -> TextureOpResult<()> {
        impl_for_all_variants!(
            self,
            n,
            n.spherical_map_to_equirectangular_map(mapping_2d, face_dimensions)
        )
    }

    pub fn spherical_map_to_octahedral_map(
        &mut self,
        mapping_2d: SphericalMapping,
        face_dimensions: UVec2,
    ) -> TextureOpResult<()> {
        impl_for_all_variants!(
            self,
            n,
            n.spherical_map_to_octahedral_map(mapping_2d, face_dimensions)
        )
    }
}
