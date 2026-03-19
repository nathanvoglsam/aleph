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

use aleph_math::{Mat4, Vec3};

use crate::scene::components::RenderTransform;

#[repr(C)]
#[derive(Debug)]
pub struct CameraLayout {
    pub view_matrix: [f32; 16],
    pub proj_matrix: [f32; 16],
    pub position: [f32; 4],
    pub _padding: [u8; 112],
}

#[repr(C)]
#[derive(Debug)]
pub struct ModelLayout {
    pub model_matrix: [f32; 16],
    pub normal_matrix: [f32; 16],
    pub _padding: [u8; 126],
}

impl ModelLayout {
    pub fn from_transform(v: &RenderTransform) -> Self {
        let pos = Vec3::new(
            v.position.x as f32,
            v.position.y as f32,
            v.position.z as f32,
        );

        let t = Mat4::from_translation(pos);
        let r = v.rotation.into_matrix().into_homogeneous();
        let s = Mat4::from_nonuniform_scale(v.scale);

        let model_matrix = t * r * s;
        let normal_matrix = model_matrix.truncate().inversed().transposed();
        Self {
            model_matrix: *model_matrix.as_array(),
            normal_matrix: *normal_matrix.into_homogeneous().as_array(),
            _padding: [0; 126],
        }
    }
}
