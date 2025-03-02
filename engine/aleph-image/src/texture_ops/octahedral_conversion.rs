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

use aleph_math::sampling::octahedral_encode;
use aleph_math::UVec2;
use half::f16;

use crate::{
    AddressModeWrap, DynamicImageBuffer, IFaceSelector, IPixelAccess, IPixelSample, IPixelStorage,
    ImageBuffer, PixR, PixRG, PixRGB, PixRGBA, PixelFormat,
};

/// Assuming the input image is an octahedral environment map, this function will compute the
/// requested face of a cubemap by sampling the source image as an octahedral environment map.
///
/// By calling this function 6 times, one for each cube face, you can create a complate cubemap
/// from an input octahedral environment map.
pub fn octahedral_to_cube<F: IFaceSelector, O: PixelFormat>(
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
            let equi_uv = octahedral_encode(dir);

            // Perform a bilinear filtered sample of the equirectangular texture at the appropriate
            // coordinate to fill our cube face.
            let p = src.sample::<AddressModeWrap, AddressModeWrap>(equi_uv);
            let p = O::from_vec4(p);
            dst.store(x, y, p);
        }
    }

    dst
}

/// Wrapper over [`octahedral_to_cube`] for a [`DynamicImageBuffer`]. The correct conversion
/// implementation and output mapper is selected based on the dynamic type of the image.
pub fn octahedral_to_cube_dyn<F: IFaceSelector>(
    src: &DynamicImageBuffer,
    face_dimension: UVec2,
) -> DynamicImageBuffer {
    match src {
        DynamicImageBuffer::R8Unorm(src) => {
            type Out = PixR<u8>;
            let new = octahedral_to_cube::<F, Out>(src, face_dimension);
            DynamicImageBuffer::R8Unorm(new)
        }
        DynamicImageBuffer::RG8Unorm(src) => {
            type Out = PixRG<u8>;
            let new = octahedral_to_cube::<F, Out>(src, face_dimension);
            DynamicImageBuffer::RG8Unorm(new)
        }
        DynamicImageBuffer::RGB8Unorm(src) => {
            type Out = PixRGB<u8>;
            let new = octahedral_to_cube::<F, Out>(src, face_dimension);
            DynamicImageBuffer::RGB8Unorm(new)
        }
        DynamicImageBuffer::RGBA8Unorm(src) => {
            type Out = PixRGBA<u8>;
            let new = octahedral_to_cube::<F, Out>(src, face_dimension);
            DynamicImageBuffer::RGBA8Unorm(new)
        }

        DynamicImageBuffer::R16Unorm(src) => {
            type Out = PixR<u16>;
            let new = octahedral_to_cube::<F, Out>(src, face_dimension);
            DynamicImageBuffer::R16Unorm(new)
        }
        DynamicImageBuffer::RG16Unorm(src) => {
            type Out = PixRG<u16>;
            let new = octahedral_to_cube::<F, Out>(src, face_dimension);
            DynamicImageBuffer::RG16Unorm(new)
        }
        DynamicImageBuffer::RGB16Unorm(src) => {
            type Out = PixRGB<u16>;
            let new = octahedral_to_cube::<F, Out>(src, face_dimension);
            DynamicImageBuffer::RGB16Unorm(new)
        }
        DynamicImageBuffer::RGBA16Unorm(src) => {
            type Out = PixRGBA<u16>;
            let new = octahedral_to_cube::<F, Out>(src, face_dimension);
            DynamicImageBuffer::RGBA16Unorm(new)
        }

        DynamicImageBuffer::R32Unorm(src) => {
            type Out = PixR<u32>;
            let new = octahedral_to_cube::<F, Out>(src, face_dimension);
            DynamicImageBuffer::R32Unorm(new)
        }
        DynamicImageBuffer::RG32Unorm(src) => {
            type Out = PixRG<u32>;
            let new = octahedral_to_cube::<F, Out>(src, face_dimension);
            DynamicImageBuffer::RG32Unorm(new)
        }
        DynamicImageBuffer::RGB32Unorm(src) => {
            type Out = PixRGB<u32>;
            let new = octahedral_to_cube::<F, Out>(src, face_dimension);
            DynamicImageBuffer::RGB32Unorm(new)
        }
        DynamicImageBuffer::RGBA32Unorm(src) => {
            type Out = PixRGBA<u32>;
            let new = octahedral_to_cube::<F, Out>(src, face_dimension);
            DynamicImageBuffer::RGBA32Unorm(new)
        }

        DynamicImageBuffer::R16Float(src) => {
            type Out = PixR<f16>;
            let new = octahedral_to_cube::<F, Out>(src, face_dimension);
            DynamicImageBuffer::R16Float(new)
        }
        DynamicImageBuffer::RG16Float(src) => {
            type Out = PixRG<f16>;
            let new = octahedral_to_cube::<F, Out>(src, face_dimension);
            DynamicImageBuffer::RG16Float(new)
        }
        DynamicImageBuffer::RGB16Float(src) => {
            type Out = PixRGB<f16>;
            let new = octahedral_to_cube::<F, Out>(src, face_dimension);
            DynamicImageBuffer::RGB16Float(new)
        }
        DynamicImageBuffer::RGBA16Float(src) => {
            type Out = PixRGBA<f16>;
            let new = octahedral_to_cube::<F, Out>(src, face_dimension);
            DynamicImageBuffer::RGBA16Float(new)
        }

        DynamicImageBuffer::R32Float(src) => {
            type Out = PixR<f32>;
            let new = octahedral_to_cube::<F, Out>(src, face_dimension);
            DynamicImageBuffer::R32Float(new)
        }
        DynamicImageBuffer::RG32Float(src) => {
            type Out = PixRG<f32>;
            let new = octahedral_to_cube::<F, Out>(src, face_dimension);
            DynamicImageBuffer::RG32Float(new)
        }
        DynamicImageBuffer::RGB32Float(src) => {
            type Out = PixRGB<f32>;
            let new = octahedral_to_cube::<F, Out>(src, face_dimension);
            DynamicImageBuffer::RGB32Float(new)
        }
        DynamicImageBuffer::RGBA32Float(src) => {
            type Out = PixRGBA<f32>;
            let new = octahedral_to_cube::<F, Out>(src, face_dimension);
            DynamicImageBuffer::RGBA32Float(new)
        }
    }
}
