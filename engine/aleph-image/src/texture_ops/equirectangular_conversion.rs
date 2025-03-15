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

use aleph_math::sampling::equirectangular_uv_to_direction;
use aleph_math::{UVec2, Vec2};

use crate::{
    DynamicImageBuffer, IDirectionalSampler, IPixelAccess, IPixelStorage, ImageBuffer,
    OctahderalDirectionalSampler, PixelFormat,
};

/// This function will resample the input image into an equivalent environment map in an
/// equirectangular encoding.
pub fn image_to_equi<O: PixelFormat>(
    src: &impl IDirectionalSampler,
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
            let dir = equirectangular_uv_to_direction(Vec2::new(u, v));
            let p = src.sample(dir);
            let p = O::from_vec4(p);
            dst.store(x, y, p);
        }
    }

    dst
}

/// Wrapper over [`image_to_equi`] for a [`DynamicImageBuffer`]. The correct conversion
/// implementation and output mapper is selected based on the dynamic type of the image.
///
/// This will assume the input is an octahedral environment map and sample it as one to produce
/// an equirectangular environment map.
pub fn octahedral_to_equi_dyn(
    src: &DynamicImageBuffer,
    face_dimension: UVec2,
) -> DynamicImageBuffer {
    match src {
        DynamicImageBuffer::R8Unorm(src) => {
            let new = image_to_equi(&OctahderalDirectionalSampler(src), face_dimension);
            DynamicImageBuffer::R8Unorm(new)
        }
        DynamicImageBuffer::RG8Unorm(src) => {
            let new = image_to_equi(&OctahderalDirectionalSampler(src), face_dimension);
            DynamicImageBuffer::RG8Unorm(new)
        }
        DynamicImageBuffer::RGB8Unorm(src) => {
            let new = image_to_equi(&OctahderalDirectionalSampler(src), face_dimension);
            DynamicImageBuffer::RGB8Unorm(new)
        }
        DynamicImageBuffer::RGBA8Unorm(src) => {
            let new = image_to_equi(&OctahderalDirectionalSampler(src), face_dimension);
            DynamicImageBuffer::RGBA8Unorm(new)
        }

        DynamicImageBuffer::R16Unorm(src) => {
            let new = image_to_equi(&OctahderalDirectionalSampler(src), face_dimension);
            DynamicImageBuffer::R16Unorm(new)
        }
        DynamicImageBuffer::RG16Unorm(src) => {
            let new = image_to_equi(&OctahderalDirectionalSampler(src), face_dimension);
            DynamicImageBuffer::RG16Unorm(new)
        }
        DynamicImageBuffer::RGB16Unorm(src) => {
            let new = image_to_equi(&OctahderalDirectionalSampler(src), face_dimension);
            DynamicImageBuffer::RGB16Unorm(new)
        }
        DynamicImageBuffer::RGBA16Unorm(src) => {
            let new = image_to_equi(&OctahderalDirectionalSampler(src), face_dimension);
            DynamicImageBuffer::RGBA16Unorm(new)
        }

        DynamicImageBuffer::R32Unorm(src) => {
            let new = image_to_equi(&OctahderalDirectionalSampler(src), face_dimension);
            DynamicImageBuffer::R32Unorm(new)
        }
        DynamicImageBuffer::RG32Unorm(src) => {
            let new = image_to_equi(&OctahderalDirectionalSampler(src), face_dimension);
            DynamicImageBuffer::RG32Unorm(new)
        }
        DynamicImageBuffer::RGB32Unorm(src) => {
            let new = image_to_equi(&OctahderalDirectionalSampler(src), face_dimension);
            DynamicImageBuffer::RGB32Unorm(new)
        }
        DynamicImageBuffer::RGBA32Unorm(src) => {
            let new = image_to_equi(&OctahderalDirectionalSampler(src), face_dimension);
            DynamicImageBuffer::RGBA32Unorm(new)
        }

        DynamicImageBuffer::R16Float(src) => {
            let new = image_to_equi(&OctahderalDirectionalSampler(src), face_dimension);
            DynamicImageBuffer::R16Float(new)
        }
        DynamicImageBuffer::RG16Float(src) => {
            let new = image_to_equi(&OctahderalDirectionalSampler(src), face_dimension);
            DynamicImageBuffer::RG16Float(new)
        }
        DynamicImageBuffer::RGB16Float(src) => {
            let new = image_to_equi(&OctahderalDirectionalSampler(src), face_dimension);
            DynamicImageBuffer::RGB16Float(new)
        }
        DynamicImageBuffer::RGBA16Float(src) => {
            let new = image_to_equi(&OctahderalDirectionalSampler(src), face_dimension);
            DynamicImageBuffer::RGBA16Float(new)
        }

        DynamicImageBuffer::R32Float(src) => {
            let new = image_to_equi(&OctahderalDirectionalSampler(src), face_dimension);
            DynamicImageBuffer::R32Float(new)
        }
        DynamicImageBuffer::RG32Float(src) => {
            let new = image_to_equi(&OctahderalDirectionalSampler(src), face_dimension);
            DynamicImageBuffer::RG32Float(new)
        }
        DynamicImageBuffer::RGB32Float(src) => {
            let new = image_to_equi(&OctahderalDirectionalSampler(src), face_dimension);
            DynamicImageBuffer::RGB32Float(new)
        }
        DynamicImageBuffer::RGBA32Float(src) => {
            let new = image_to_equi(&OctahderalDirectionalSampler(src), face_dimension);
            DynamicImageBuffer::RGBA32Float(new)
        }
    }
}
