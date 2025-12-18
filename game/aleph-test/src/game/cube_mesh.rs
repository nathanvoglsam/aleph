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

use aleph_engine::interfaces::mg::renderer::{BufferOptions, Renderer};
use aleph_engine::interfaces::mg::resource::buffer::BufferHandle;
use aleph_engine::interfaces::mg::resource_loader::upload_buffer::{IUploadBuffer, UploadBuffer};
use bytemuck::{Pod, Zeroable};

#[aleph_profile::function]
pub fn upload_cube_buffers(renderer: &mut Renderer) -> (BufferHandle, BufferHandle) {
    let v_size = size_of_val(&VERTS) as u64;
    let mut v_buffer = UploadBuffer::new_owned(renderer.device(), v_size).unwrap();

    let src = bytemuck::cast_slice::<Vertex, u8>(&VERTS);
    let vtx_data = &mut v_buffer.bytes_mut()[0..size_of_val(&VERTS)];
    vtx_data.copy_from_slice(src);

    let v_handle = renderer
        .create_buffer_immediate(
            v_size,
            Some(v_buffer.into_smallbox()),
            &BufferOptions::default(),
        )
        .unwrap();

    let i_size = size_of_val(&INDICES) as u64;
    let mut i_buffer = UploadBuffer::new_owned(renderer.device(), i_size).unwrap();

    let src = bytemuck::cast_slice::<_, u8>(&INDICES);
    let idx_data = &mut i_buffer.bytes_mut()[0..size_of_val(&INDICES)];
    idx_data.copy_from_slice(src);

    let i_handle = renderer
        .create_buffer_immediate(
            i_size,
            Some(i_buffer.into_smallbox()),
            &BufferOptions::default(),
        )
        .unwrap();

    (i_handle, v_handle)
}

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub uv: [f32; 2],
    pub normal: [f32; 3],
    pub tangent: [f32; 4],
    pub colour: [f32; 3],
}

impl Vertex {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            position: [x, y, z],
            uv: [0.; 2],
            normal: [0.; 3],
            tangent: [0.; 4],
            colour: [0.5; 3],
        }
    }

    pub const fn normal(mut self, x: f32, y: f32, z: f32) -> Self {
        self.normal = [x, y, z];
        self
    }

    pub const fn tangent(mut self, x: f32, y: f32, z: f32, w: f32) -> Self {
        self.tangent = [x, y, z, w];
        self
    }

    pub const fn uv(mut self, u: f32, v: f32) -> Self {
        self.uv = [u, v];
        self
    }
}

#[rustfmt::skip]
const VERTS: [Vertex; 24] = [
    Vertex::new(-1.0, -1.0,  1.0).normal( 0.0,  0.0,  1.0).tangent( 0.0, 1.0, 0.0, 1.0).uv(0.375, 0.0),
    Vertex::new(-1.0, -1.0,  1.0).normal( 0.0, -1.0,  0.0).tangent( 1.0, 0.0, 0.0, 1.0).uv(0.125, 0.25),
    Vertex::new(-1.0, -1.0,  1.0).normal(-1.0,  0.0,  0.0).tangent( 0.0, 1.0, 0.0, 1.0).uv(0.375, 1.0),
    Vertex::new(-1.0,  1.0,  1.0).normal( 0.0,  0.0,  1.0).tangent( 0.0, 1.0, 0.0, 1.0).uv(0.625, 0.0),
    Vertex::new(-1.0,  1.0,  1.0).normal( 0.0,  1.0,  0.0).tangent(-1.0, 0.0, 0.0, 1.0).uv(0.875, 0.25),
    Vertex::new(-1.0,  1.0,  1.0).normal(-1.0,  0.0,  0.0).tangent( 0.0, 1.0, 0.0, 1.0).uv(0.625, 1.0),
    Vertex::new(-1.0, -1.0, -1.0).normal( 0.0,  0.0, -1.0).tangent( 0.0, 1.0, 0.0, 1.0).uv(0.375, 0.75),
    Vertex::new(-1.0, -1.0, -1.0).normal( 0.0, -1.0,  0.0).tangent( 1.0, 0.0, 0.0, 1.0).uv(0.125, 0.5),
    Vertex::new(-1.0, -1.0, -1.0).normal(-1.0,  0.0,  0.0).tangent( 0.0, 1.0, 0.0, 1.0).uv(0.375, 0.75),
    Vertex::new(-1.0,  1.0, -1.0).normal( 0.0,  0.0, -1.0).tangent( 0.0, 1.0, 0.0, 1.0).uv(0.625, 0.75),
    Vertex::new(-1.0,  1.0, -1.0).normal( 0.0,  1.0,  0.0).tangent(-1.0, 0.0, 0.0, 1.0).uv(0.875, 0.5),
    Vertex::new(-1.0,  1.0, -1.0).normal(-1.0,  0.0,  0.0).tangent( 0.0, 1.0, 0.0, 1.0).uv(0.625, 0.75),
    Vertex::new( 1.0, -1.0,  1.0).normal( 0.0,  0.0,  1.0).tangent( 0.0, 1.0, 0.0, 1.0).uv(0.375, 0.25),
    Vertex::new( 1.0, -1.0,  1.0).normal( 0.0, -1.0,  0.0).tangent( 1.0, 0.0, 0.0, 1.0).uv(0.375, 0.25),
    Vertex::new( 1.0, -1.0,  1.0).normal( 1.0,  0.0,  0.0).tangent( 0.0, 1.0, 0.0, 1.0).uv(0.375, 0.25),
    Vertex::new( 1.0,  1.0,  1.0).normal( 0.0,  0.0,  1.0).tangent( 0.0, 1.0, 0.0, 1.0).uv(0.625, 0.25),
    Vertex::new( 1.0,  1.0,  1.0).normal( 0.0,  1.0,  0.0).tangent(-1.0, 0.0, 0.0, 1.0).uv(0.625, 0.25),
    Vertex::new( 1.0,  1.0,  1.0).normal( 1.0,  0.0,  0.0).tangent( 0.0, 1.0, 0.0, 1.0).uv(0.625, 0.25),
    Vertex::new( 1.0, -1.0, -1.0).normal( 0.0,  0.0, -1.0).tangent( 0.0, 1.0, 0.0, 1.0).uv(0.375, 0.5),
    Vertex::new( 1.0, -1.0, -1.0).normal( 0.0, -1.0,  0.0).tangent( 1.0, 0.0, 0.0, 1.0).uv(0.375, 0.5),
    Vertex::new( 1.0, -1.0, -1.0).normal( 1.0,  0.0,  0.0).tangent( 0.0, 1.0, 0.0, 1.0).uv(0.375, 0.5),
    Vertex::new( 1.0,  1.0, -1.0).normal( 0.0,  0.0, -1.0).tangent( 0.0, 1.0, 0.0, 1.0).uv(0.625, 0.5),
    Vertex::new( 1.0,  1.0, -1.0).normal( 0.0,  1.0,  0.0).tangent(-1.0, 0.0, 0.0, 1.0).uv(0.625, 0.5),
    Vertex::new( 1.0,  1.0, -1.0).normal( 1.0,  0.0,  0.0).tangent( 0.0, 1.0, 0.0, 1.0).uv(0.625, 0.5),
];

#[rustfmt::skip]
const INDICES: [u32; 36] = [
    2, 5, 11,
    2, 11, 8,
    6, 9, 21,
    6, 21, 18,
    20, 23, 17,
    20, 17, 14,
    12, 15, 3,
    12, 3, 0,
    7, 19, 13,
    7, 13, 1,
    22, 10, 4,
    22, 4, 16,
];
