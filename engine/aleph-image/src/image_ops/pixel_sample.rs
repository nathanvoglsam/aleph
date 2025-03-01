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

use aleph_math::{Vec2, Vec4};

use crate::{IPixelAccess, IPixelStorage, ImageBuffer, PixelFormat};

pub trait IPixelSample: IPixelAccess {
    fn sample(&self, uv: Vec2) -> Vec4;
}

impl<T: PixelFormat> IPixelSample for ImageBuffer<T> {
    fn sample(&self, uv: Vec2) -> Vec4 {
        let dims_f32 = self.dimensions_f32();

        let (t_floor_u, t_ceil_u, w_floor_u, w_ceil_u) =
            texel_coord_to_sample_pos_and_weights(dims_f32.x, uv.x);
        let (t_floor_v, t_ceil_v, w_floor_v, w_ceil_v) =
            texel_coord_to_sample_pos_and_weights(dims_f32.y, uv.y);

        let sample_0 = self.load(t_floor_u as u32, t_floor_v as u32).as_vec4();
        let sample_0 = sample_0 * w_floor_u * w_floor_v;

        let sample_1 = self.load(t_floor_u as u32, t_ceil_v as u32).as_vec4();
        let sample_1 = sample_1 * w_floor_u * w_ceil_v;

        let sample_2 = self.load(t_ceil_u as u32, t_floor_v as u32).as_vec4();
        let sample_2 = sample_2 * w_ceil_u * w_floor_v;

        let sample_3 = self.load(t_ceil_u as u32, t_ceil_v as u32).as_vec4();
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
