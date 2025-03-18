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

use aleph_math::sampling::octahedral_decode;
use aleph_math::{UVec2, Vec2};

use crate::{IDirectionalSampler, IPixelAccess, IPixelStorage, ImageBuffer, PixelFormat};

/// This function will resample the input image into an equivalent environment map in an octahedral
/// encoding.
pub fn image_to_octahedral<O: PixelFormat>(
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
            let dir = octahedral_decode(Vec2::new(u, v));
            let p = src.sample(dir);
            let p = O::from_vec4(p);
            dst.store(x, y, p);
        }
    }

    dst
}
