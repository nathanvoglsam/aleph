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

use crate::{
    FaceNegX, FaceNegY, FaceNegZ, FacePosX, FacePosY, FacePosZ, IDirectionalSampler, IFaceSelector,
    IPixelAccess, IPixelStorage, ImageBuffer, PixelFormat,
};

/// This function will compute the requested face of a cubemap by sampling the source environment
/// map.
///
/// By calling this function 6 times, one for each cube face, you can create a complate cubemap
/// from an input environment map.
pub fn image_to_cube<F: IFaceSelector, O: PixelFormat>(
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
            let dir = F::get_mapped(u, v);
            let p = src.sample(dir);
            let p = O::from_vec4(p);
            dst.store(x, y, p);
        }
    }

    dst
}

pub(crate) fn image_to_whole_cube<O: PixelFormat>(
    dst: &mut Vec<ImageBuffer<O>>,
    src: &impl IDirectionalSampler,
    face_dimensions: UVec2,
) {
    dst.push(image_to_cube::<FacePosX, _>(src, face_dimensions));
    dst.push(image_to_cube::<FaceNegX, _>(src, face_dimensions));
    dst.push(image_to_cube::<FacePosY, _>(src, face_dimensions));
    dst.push(image_to_cube::<FaceNegY, _>(src, face_dimensions));
    dst.push(image_to_cube::<FacePosZ, _>(src, face_dimensions));
    dst.push(image_to_cube::<FaceNegZ, _>(src, face_dimensions));
}
