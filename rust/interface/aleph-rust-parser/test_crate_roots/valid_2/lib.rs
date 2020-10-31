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

mod math;

use math::Vector3;
pub(crate) use math::{Ray3D, Ray2D};
pub use crate::math::Vector2 as Vec2;
use std::slice::Iter;
pub use math::funcs::Vec56;

#[aleph::interface]
#[repr(C)]
pub struct TestSingleton {}

impl TestSingleton {
    pub fn hello_world() {
        println!("Hello, World!");
    }

    pub fn have_a_number() -> u32 {
        56
    }

    pub fn have_a_float() -> f32 {
        21.0f32
    }

    pub fn sqaure_this_number(number: u32) -> u32 {
        number * number
    }

    pub fn trace_this_ray(ray: &Ray3D) -> Vector3 {
        ray.origin
    }

    pub fn count_this_slice(slice: &[Vector3]) -> u64 {
        slice.len() as u64
    }
}

#[aleph::interface(opaque)]
pub struct Opaque {
    data: u8,
}

pub struct IgnoreMe {
    also_data: Vec<u8>,
}
