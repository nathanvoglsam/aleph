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

use aleph_math::{UVec2, Vec2, Vec3, Vec4};
use image::{DynamicImage, Luma, LumaA, Pixel, Rgb, Rgba};

use crate::commands::img2ktx::{f32_to_unorm_u16, f32_to_unorm_u8, ISampledImage};

pub trait IFaceSelector {
    fn map_uv_to_direction(u: f32, v: f32) -> Vec3;

    fn get_mapped(u: f32, v: f32) -> Vec3 {
        let u = u * 2.0 - 1.0;
        let v = v * 2.0 - 1.0;
        let dir = Self::map_uv_to_direction(u, v);
        dir.normalized()
    }
}

pub struct PosX;

impl IFaceSelector for PosX {
    fn map_uv_to_direction(u: f32, v: f32) -> Vec3 {
        Vec3::new(1.0, -v, -u)
    }
}

pub struct NegX;

impl IFaceSelector for NegX {
    fn map_uv_to_direction(u: f32, v: f32) -> Vec3 {
        Vec3::new(-1.0, -v, u)
    }
}

pub struct PosY;

impl IFaceSelector for PosY {
    fn map_uv_to_direction(u: f32, v: f32) -> Vec3 {
        Vec3::new(u, 1.0, v)
    }
}

pub struct NegY;

impl IFaceSelector for NegY {
    fn map_uv_to_direction(u: f32, v: f32) -> Vec3 {
        Vec3::new(u, -1.0, -v)
    }
}

pub struct PosZ;

impl IFaceSelector for PosZ {
    fn map_uv_to_direction(u: f32, v: f32) -> Vec3 {
        Vec3::new(u, -v, 1.0)
    }
}

pub struct NegZ;

impl IFaceSelector for NegZ {
    fn map_uv_to_direction(u: f32, v: f32) -> Vec3 {
        Vec3::new(-u, -v, -1.0)
    }
}

pub trait IOutputMapper {
    type OutputType: Pixel;

    fn map_fragment_to_output(f: Vec4) -> Self::OutputType;

    fn construct_target(
        dim: UVec2,
    ) -> image::ImageBuffer<Self::OutputType, Vec<<Self::OutputType as Pixel>::Subpixel>>;
}

impl IOutputMapper for image::ImageBuffer<Luma<u8>, Vec<u8>> {
    type OutputType = Luma<u8>;

    fn map_fragment_to_output(f: Vec4) -> Self::OutputType {
        Luma([f32_to_unorm_u8(f.x)])
    }

    fn construct_target(dim: UVec2) -> Self {
        Self::new(dim.x, dim.y)
    }
}

impl IOutputMapper for image::ImageBuffer<LumaA<u8>, Vec<u8>> {
    type OutputType = LumaA<u8>;

    fn map_fragment_to_output(f: Vec4) -> Self::OutputType {
        LumaA([f32_to_unorm_u8(f.x), f32_to_unorm_u8(f.y)])
    }

    fn construct_target(dim: UVec2) -> Self {
        Self::new(dim.x, dim.y)
    }
}

impl IOutputMapper for image::ImageBuffer<Rgb<u8>, Vec<u8>> {
    type OutputType = Rgb<u8>;

    fn map_fragment_to_output(f: Vec4) -> Self::OutputType {
        Rgb([
            f32_to_unorm_u8(f.x),
            f32_to_unorm_u8(f.y),
            f32_to_unorm_u8(f.z),
        ])
    }

    fn construct_target(dim: UVec2) -> Self {
        Self::new(dim.x, dim.y)
    }
}

impl IOutputMapper for image::ImageBuffer<Rgba<u8>, Vec<u8>> {
    type OutputType = Rgba<u8>;

    fn map_fragment_to_output(f: Vec4) -> Self::OutputType {
        Rgba([
            f32_to_unorm_u8(f.x),
            f32_to_unorm_u8(f.y),
            f32_to_unorm_u8(f.z),
            f32_to_unorm_u8(f.w),
        ])
    }

    fn construct_target(dim: UVec2) -> Self {
        Self::new(dim.x, dim.y)
    }
}

impl IOutputMapper for image::ImageBuffer<Luma<u16>, Vec<u16>> {
    type OutputType = Luma<u16>;

    fn map_fragment_to_output(f: Vec4) -> Self::OutputType {
        Luma([f32_to_unorm_u16(f.x)])
    }

    fn construct_target(dim: UVec2) -> Self {
        Self::new(dim.x, dim.y)
    }
}

impl IOutputMapper for image::ImageBuffer<LumaA<u16>, Vec<u16>> {
    type OutputType = LumaA<u16>;

    fn map_fragment_to_output(f: Vec4) -> Self::OutputType {
        LumaA([f32_to_unorm_u16(f.x), f32_to_unorm_u16(f.y)])
    }

    fn construct_target(dim: UVec2) -> Self {
        Self::new(dim.x, dim.y)
    }
}

impl IOutputMapper for image::ImageBuffer<Rgb<u16>, Vec<u16>> {
    type OutputType = Rgb<u16>;

    fn map_fragment_to_output(f: Vec4) -> Self::OutputType {
        Rgb([
            f32_to_unorm_u16(f.x),
            f32_to_unorm_u16(f.y),
            f32_to_unorm_u16(f.z),
        ])
    }

    fn construct_target(dim: UVec2) -> Self {
        Self::new(dim.x, dim.y)
    }
}

impl IOutputMapper for image::ImageBuffer<Rgba<u16>, Vec<u16>> {
    type OutputType = Rgba<u16>;

    fn map_fragment_to_output(f: Vec4) -> Self::OutputType {
        Rgba([
            f32_to_unorm_u16(f.x),
            f32_to_unorm_u16(f.y),
            f32_to_unorm_u16(f.z),
            f32_to_unorm_u16(f.w),
        ])
    }

    fn construct_target(dim: UVec2) -> Self {
        Self::new(dim.x, dim.y)
    }
}

impl IOutputMapper for image::ImageBuffer<Rgb<f32>, Vec<f32>> {
    type OutputType = Rgb<f32>;

    fn map_fragment_to_output(f: Vec4) -> Self::OutputType {
        Rgb([f.x, f.y, f.z])
    }

    fn construct_target(dim: UVec2) -> Self {
        Self::new(dim.x, dim.y)
    }
}

impl IOutputMapper for image::ImageBuffer<Rgba<f32>, Vec<f32>> {
    type OutputType = Rgba<f32>;

    fn map_fragment_to_output(f: Vec4) -> Self::OutputType {
        Rgba([f.x, f.y, f.z, f.w])
    }

    fn construct_target(dim: UVec2) -> Self {
        Self::new(dim.x, dim.y)
    }
}

pub fn equi_to_cube<F: IFaceSelector, O: IOutputMapper>(
    src: &impl ISampledImage,
    face_dimension: UVec2,
) -> image::ImageBuffer<O::OutputType, Vec<<O::OutputType as Pixel>::Subpixel>> {
    let sampler = src.get_sampler();

    let mut dst = O::construct_target(face_dimension);
    let dim_f32 = Vec2::new(face_dimension.x as f32, face_dimension.y as f32);

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
            let p = sampler.sample_image(equi_uv);
            let p = O::map_fragment_to_output(p);
            dst.put_pixel(x, y, p);
        }
    }

    dst
}

pub fn equi_to_cube_dyn<F: IFaceSelector>(
    src: &DynamicImage,
    face_dimension: UVec2,
) -> DynamicImage {
    match src {
        DynamicImage::ImageLuma8(src) => {
            type Out = image::ImageBuffer<Luma<u8>, Vec<u8>>;
            let new = equi_to_cube::<F, Out>(src, face_dimension);
            DynamicImage::ImageLuma8(new)
        }
        DynamicImage::ImageLumaA8(src) => {
            type Out = image::ImageBuffer<LumaA<u8>, Vec<u8>>;
            let new = equi_to_cube::<F, Out>(src, face_dimension);
            DynamicImage::ImageLumaA8(new)
        }
        DynamicImage::ImageRgb8(src) => {
            type Out = image::ImageBuffer<Rgb<u8>, Vec<u8>>;
            let new = equi_to_cube::<F, Out>(src, face_dimension);
            DynamicImage::ImageRgb8(new)
        }
        DynamicImage::ImageRgba8(src) => {
            type Out = image::ImageBuffer<Rgba<u8>, Vec<u8>>;
            let new = equi_to_cube::<F, Out>(src, face_dimension);
            DynamicImage::ImageRgba8(new)
        }
        DynamicImage::ImageLuma16(src) => {
            type Out = image::ImageBuffer<Luma<u16>, Vec<u16>>;
            let new = equi_to_cube::<F, Out>(src, face_dimension);
            DynamicImage::ImageLuma16(new)
        }
        DynamicImage::ImageLumaA16(src) => {
            type Out = image::ImageBuffer<LumaA<u16>, Vec<u16>>;
            let new = equi_to_cube::<F, Out>(src, face_dimension);
            DynamicImage::ImageLumaA16(new)
        }
        DynamicImage::ImageRgb16(src) => {
            type Out = image::ImageBuffer<Rgb<u16>, Vec<u16>>;
            let new = equi_to_cube::<F, Out>(src, face_dimension);
            DynamicImage::ImageRgb16(new)
        }
        DynamicImage::ImageRgba16(src) => {
            type Out = image::ImageBuffer<Rgba<u16>, Vec<u16>>;
            let new = equi_to_cube::<F, Out>(src, face_dimension);
            DynamicImage::ImageRgba16(new)
        }
        DynamicImage::ImageRgb32F(src) => {
            type Out = image::ImageBuffer<Rgb<f32>, Vec<f32>>;
            let new = equi_to_cube::<F, Out>(src, face_dimension);
            DynamicImage::ImageRgb32F(new)
        }
        DynamicImage::ImageRgba32F(src) => {
            type Out = image::ImageBuffer<Rgba<f32>, Vec<f32>>;
            let new = equi_to_cube::<F, Out>(src, face_dimension);
            DynamicImage::ImageRgba32F(new)
        }
        _ => unimplemented!(),
    }
}

fn sample_spherical_map(s: Vec3) -> Vec2 {
    use std::f32::consts::PI;
    let xf = f32::atan2(s.x, s.z) * (1.0 / PI);     // range [-1.0, 1.0]
    let yf = f32::asin(s.y) * (2.0 / PI);           // range [-1.0, 1.0]
    let xf = (xf + 1.0) * 0.5;                      // range [0, 1.0]
    let yf = (1.0 - yf) * 0.5;                      // range [0, 1.0]
    return Vec2::new(xf, yf);
}
