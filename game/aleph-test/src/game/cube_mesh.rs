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

use aleph_engine::interfaces::renderer::{BufferHandle, BufferUploadSource, Renderer};
use aleph_rhi_api::*;
use bytemuck::{Pod, Zeroable};

#[aleph_profile::function]
pub fn upload_cube_buffers(renderer: &mut Renderer) -> (BufferHandle, BufferHandle) {
    let mut vtx_buffer = unsafe {
        BufferUploadSource::new_owned(
            renderer.device(),
            size_of_val(&VERTS),
            ResourceUsageFlags::VERTEX_BUFFER,
        )
        .unwrap()
    };
    let mut idx_buffer = unsafe {
        BufferUploadSource::new_owned(
            renderer.device(),
            size_of_val(&INDICES),
            ResourceUsageFlags::INDEX_BUFFER,
        )
        .unwrap()
    };

    let src = bytemuck::cast_slice::<Vertex, u8>(&VERTS);
    let vtx_data = &mut vtx_buffer.data_mut()[0..size_of_val(&VERTS)];
    vtx_data.copy_from_slice(src);

    let src = bytemuck::cast_slice::<_, u8>(&INDICES);
    let idx_data = &mut idx_buffer.data_mut()[0..size_of_val(&INDICES)];
    idx_data.copy_from_slice(src);

    let vtx = renderer.create_buffer(vtx_buffer).unwrap();
    let idx = renderer.create_buffer(idx_buffer).unwrap();

    (idx, vtx)
}

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
struct Vertex {
    pub position: [f32; 3],
    pub uv: [f32; 2],
    pub normal: [f32; 3],
    pub tangent: [f32; 3],
    pub colour: [f32; 3],
}

impl Vertex {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            position: [x, y, z],
            uv: [0.; 2],
            normal: [0.; 3],
            tangent: [0.; 3],
            colour: [0.5; 3],
        }
    }

    pub const fn normal(mut self, x: f32, y: f32, z: f32) -> Self {
        self.normal = [x, y, z];
        self
    }

    pub const fn uv(mut self, u: f32, v: f32) -> Self {
        self.uv = [u, v];
        self
    }
}

#[rustfmt::skip]
const VERTS: [Vertex; 24] = [
    Vertex::new(-1.,  1., -1.).normal( 0.,  1.,  0.).uv(0.875, 0.5),
    Vertex::new( 1.,  1.,  1.).normal( 0.,  1.,  0.).uv(0.625, 0.75),
    Vertex::new( 1.,  1., -1.).normal( 0.,  1.,  0.).uv(0.625, 0.5),
    Vertex::new( 1.,  1.,  1.).normal( 0.,  0.,  1.).uv(0.625, 0.75),
    Vertex::new(-1., -1.,  1.).normal( 0.,  0.,  1.).uv(0.375, 1.),
    Vertex::new( 1., -1.,  1.).normal( 0.,  0.,  1.).uv(0.375, 0.75),
    Vertex::new(-1.,  1.,  1.).normal(-1.,  0.,  0.).uv(0.625, 0.),
    Vertex::new(-1., -1., -1.).normal(-1.,  0.,  0.).uv(0.375, 0.25),
    Vertex::new(-1., -1.,  1.).normal(-1.,  0.,  0.).uv(0.375, 0.),
    Vertex::new( 1., -1., -1.).normal( 0., -1.,  0.).uv(0.375, 0.5),
    Vertex::new(-1., -1.,  1.).normal( 0., -1.,  0.).uv(0.125, 0.75),
    Vertex::new(-1., -1., -1.).normal( 0., -1.,  0.).uv(0.125, 0.5),
    Vertex::new( 1.,  1., -1.).normal( 1.,  0.,  0.).uv(0.625, 0.5),
    Vertex::new( 1., -1.,  1.).normal( 1.,  0.,  0.).uv(0.375, 0.75),
    Vertex::new( 1., -1., -1.).normal( 1.,  0.,  0.).uv(0.375, 0.5),
    Vertex::new(-1.,  1., -1.).normal( 0.,  0., -1.).uv(0.625, 0.25),
    Vertex::new( 1., -1., -1.).normal( 0.,  0., -1.).uv(0.375, 0.5),
    Vertex::new(-1., -1., -1.).normal( 0.,  0., -1.).uv(0.375, 0.25),
    Vertex::new(-1.,  1.,  1.).normal( 0.,  1.,  0.).uv(0.875, 0.75),
    Vertex::new(-1.,  1.,  1.).normal( 0.,  0.,  1.).uv(0.625, 1.),
    Vertex::new(-1.,  1., -1.).normal(-1.,  0.,  0.).uv(0.625, 0.25),
    Vertex::new( 1., -1.,  1.).normal( 0., -1.,  0.).uv(0.375, 0.75),
    Vertex::new( 1.,  1.,  1.).normal( 1.,  0.,  0.).uv(0.625, 0.75),
    Vertex::new( 1.,  1., -1.).normal( 0.,  0., -1.).uv(0.625, 0.5),
];

#[rustfmt::skip]
const INDICES: [u32; 36] = [
    0, 1, 2,
    3, 4, 5,
    6, 7, 8,
    9, 10, 11,
    12, 13, 14,
    15, 16, 17,
    0, 18, 1,
    3, 19, 4,
    6, 20, 7,
    9, 21, 10,
    12, 22, 13,
    15, 23, 16,
];
