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

use aleph_math::{UVec2, Vec2, Vec3};
use half::f16;

use crate::{
    DynamicImageBuffer, IPixelAccess, IPixelSample, IPixelStorage, ImageBuffer, PixR, PixRG,
    PixRGB, PixRGBA, PixelFormat,
};

pub trait IFaceSelector {
    fn map_uv_to_direction(u: f32, v: f32) -> Vec3;

    fn get_mapped(u: f32, v: f32) -> Vec3 {
        let u = u * 2.0 - 1.0;
        let v = v * 2.0 - 1.0;
        let dir = Self::map_uv_to_direction(u, v);
        dir.normalized()
    }
}

pub struct FacePosX;

impl IFaceSelector for FacePosX {
    fn map_uv_to_direction(u: f32, v: f32) -> Vec3 {
        Vec3::new(1.0, -v, -u)
    }
}

pub struct FaceNegX;

impl IFaceSelector for FaceNegX {
    fn map_uv_to_direction(u: f32, v: f32) -> Vec3 {
        Vec3::new(-1.0, -v, u)
    }
}

pub struct FacePosY;

impl IFaceSelector for FacePosY {
    fn map_uv_to_direction(u: f32, v: f32) -> Vec3 {
        Vec3::new(u, 1.0, v)
    }
}

pub struct FaceNegY;

impl IFaceSelector for FaceNegY {
    fn map_uv_to_direction(u: f32, v: f32) -> Vec3 {
        Vec3::new(u, -1.0, -v)
    }
}

pub struct FacePosZ;

impl IFaceSelector for FacePosZ {
    fn map_uv_to_direction(u: f32, v: f32) -> Vec3 {
        Vec3::new(u, -v, 1.0)
    }
}

pub struct FaceNegZ;

impl IFaceSelector for FaceNegZ {
    fn map_uv_to_direction(u: f32, v: f32) -> Vec3 {
        Vec3::new(-u, -v, -1.0)
    }
}

pub fn equi_to_cube<F: IFaceSelector, O: PixelFormat>(
    src: &impl IPixelSample,
    face_dimension: UVec2,
) -> ImageBuffer<O> {
    let mut dst = ImageBuffer::<O>::new(face_dimension.x, face_dimension.y);
    let dim_f32 = dst.dimensions_f32();

    for y in 0..dst.height() {
        let v = (y as f32 + 0.5) / dim_f32.y;
        for x in 0..dst.width() {
            let u = (x as f32 + 0.5) / dim_f32.x;

            // Use our face selector interface to map the uv space onto the requested cube direction
            // that we want to sample.
            let dir = F::get_mapped(u, v);
            let equi_uv = sample_spherical_map(dir);

            // Perform a bilinear filtered sample of the equirectangular texture at the appropriate
            // coordinate to fill our cube face.
            let p = src.sample(equi_uv);
            let p = O::from_vec4(p);
            dst.store(x, y, p);
        }
    }

    dst
}

pub fn equi_to_cube_dyn<F: IFaceSelector>(
    src: &DynamicImageBuffer,
    face_dimension: UVec2,
) -> DynamicImageBuffer {
    match src {
        DynamicImageBuffer::R8Unorm(src) => {
            type Out = PixR<u8>;
            let new = equi_to_cube::<F, Out>(src, face_dimension);
            DynamicImageBuffer::R8Unorm(new)
        }
        DynamicImageBuffer::RG8Unorm(src) => {
            type Out = PixRG<u8>;
            let new = equi_to_cube::<F, Out>(src, face_dimension);
            DynamicImageBuffer::RG8Unorm(new)
        }
        DynamicImageBuffer::RGB8Unorm(src) => {
            type Out = PixRGB<u8>;
            let new = equi_to_cube::<F, Out>(src, face_dimension);
            DynamicImageBuffer::RGB8Unorm(new)
        }
        DynamicImageBuffer::RGBA8Unorm(src) => {
            type Out = PixRGBA<u8>;
            let new = equi_to_cube::<F, Out>(src, face_dimension);
            DynamicImageBuffer::RGBA8Unorm(new)
        }

        DynamicImageBuffer::R16Unorm(src) => {
            type Out = PixR<u16>;
            let new = equi_to_cube::<F, Out>(src, face_dimension);
            DynamicImageBuffer::R16Unorm(new)
        }
        DynamicImageBuffer::RG16Unorm(src) => {
            type Out = PixRG<u16>;
            let new = equi_to_cube::<F, Out>(src, face_dimension);
            DynamicImageBuffer::RG16Unorm(new)
        }
        DynamicImageBuffer::RGB16Unorm(src) => {
            type Out = PixRGB<u16>;
            let new = equi_to_cube::<F, Out>(src, face_dimension);
            DynamicImageBuffer::RGB16Unorm(new)
        }
        DynamicImageBuffer::RGBA16Unorm(src) => {
            type Out = PixRGBA<u16>;
            let new = equi_to_cube::<F, Out>(src, face_dimension);
            DynamicImageBuffer::RGBA16Unorm(new)
        }

        DynamicImageBuffer::R32Unorm(src) => {
            type Out = PixR<u32>;
            let new = equi_to_cube::<F, Out>(src, face_dimension);
            DynamicImageBuffer::R32Unorm(new)
        }
        DynamicImageBuffer::RG32Unorm(src) => {
            type Out = PixRG<u32>;
            let new = equi_to_cube::<F, Out>(src, face_dimension);
            DynamicImageBuffer::RG32Unorm(new)
        }
        DynamicImageBuffer::RGB32Unorm(src) => {
            type Out = PixRGB<u32>;
            let new = equi_to_cube::<F, Out>(src, face_dimension);
            DynamicImageBuffer::RGB32Unorm(new)
        }
        DynamicImageBuffer::RGBA32Unorm(src) => {
            type Out = PixRGBA<u32>;
            let new = equi_to_cube::<F, Out>(src, face_dimension);
            DynamicImageBuffer::RGBA32Unorm(new)
        }

        DynamicImageBuffer::R16Float(src) => {
            type Out = PixR<f16>;
            let new = equi_to_cube::<F, Out>(src, face_dimension);
            DynamicImageBuffer::R16Float(new)
        }
        DynamicImageBuffer::RG16Float(src) => {
            type Out = PixRG<f16>;
            let new = equi_to_cube::<F, Out>(src, face_dimension);
            DynamicImageBuffer::RG16Float(new)
        }
        DynamicImageBuffer::RGB16Float(src) => {
            type Out = PixRGB<f16>;
            let new = equi_to_cube::<F, Out>(src, face_dimension);
            DynamicImageBuffer::RGB16Float(new)
        }
        DynamicImageBuffer::RGBA16Float(src) => {
            type Out = PixRGBA<f16>;
            let new = equi_to_cube::<F, Out>(src, face_dimension);
            DynamicImageBuffer::RGBA16Float(new)
        }

        DynamicImageBuffer::R32Float(src) => {
            type Out = PixR<f32>;
            let new = equi_to_cube::<F, Out>(src, face_dimension);
            DynamicImageBuffer::R32Float(new)
        }
        DynamicImageBuffer::RG32Float(src) => {
            type Out = PixRG<f32>;
            let new = equi_to_cube::<F, Out>(src, face_dimension);
            DynamicImageBuffer::RG32Float(new)
        }
        DynamicImageBuffer::RGB32Float(src) => {
            type Out = PixRGB<f32>;
            let new = equi_to_cube::<F, Out>(src, face_dimension);
            DynamicImageBuffer::RGB32Float(new)
        }
        DynamicImageBuffer::RGBA32Float(src) => {
            type Out = PixRGBA<f32>;
            let new = equi_to_cube::<F, Out>(src, face_dimension);
            DynamicImageBuffer::RGBA32Float(new)
        }
    }
}

fn sample_spherical_map(s: Vec3) -> Vec2 {
    use std::f32::consts::PI;
    let xf = f32::atan2(s.x, s.z) * (1.0 / PI); // range [-1.0, 1.0]
    let yf = f32::asin(s.y) * (2.0 / PI); // range [-1.0, 1.0]
    let xf = (xf + 1.0) * 0.5; // range [0, 1.0]
    let yf = (1.0 - yf) * 0.5; // range [0, 1.0]
    return Vec2::new(xf, yf);
}
