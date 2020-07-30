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

use gltf::buffer::Source;
use gltf::json::accessor::{ComponentType, Type};
use std::ops::Deref;

#[inline]
pub(crate) fn get_accessor_for(
    gltf: &'static gltf::Gltf,
    semantic: &gltf::Semantic,
    primitive_mode: gltf::mesh::Mode,
) -> gltf::Accessor<'static> {
    // Get the first mesh, the cube mesh should only hold one mesh
    let mesh = gltf.meshes().next().unwrap();

    // Get the primitives for the mesh, should only be one
    let prim = mesh.primitives().next().unwrap();

    // Mesh should be of the expected mode
    if prim.mode() != primitive_mode {
        panic!()
    }

    // Get the needed buffer accessor. Must exist
    prim.get(semantic).unwrap()
}

#[inline]
pub(crate) fn get_accessor_for_indices(
    gltf: &'static gltf::Gltf,
    primitive_mode: gltf::mesh::Mode,
) -> gltf::Accessor<'static> {
    // Get the first mesh, the cube mesh should only hold one mesh
    let mesh = gltf.meshes().next().unwrap();

    // Get the primitives for the mesh, should only be one
    let prim = mesh.primitives().next().unwrap();

    // Mesh should be of the expected mode
    if prim.mode() != primitive_mode {
        panic!()
    }

    // Get the needed buffer accessor. Must exist
    prim.indices().unwrap()
}

#[inline]
pub(crate) fn get_vec4_bytes(
    glb: &'static gltf::Glb,
    acc: &gltf::Accessor<'static>,
) -> &'static [[f32; 4]] {
    // Get the buffer view for the accessor. Must exist
    let view = acc.view().unwrap();

    // Must be a f32 data
    if acc.data_type() != ComponentType::F32 {
        panic!()
    }

    // Must be a 3 component vector
    if acc.dimensions() != Type::Vec4 {
        panic!()
    }

    // Data must be contained in the BIN section of a binary GLTF file
    match view.buffer().source() {
        Source::Bin => {}
        _ => panic!(),
    }

    // Get the BIN data slice from the gltf. Must exist
    let bin = glb.bin.as_ref().unwrap().deref();

    unsafe {
        let data = bin.as_ptr().add(view.offset());
        let data = data as *const [f32; 4];
        let len = view.length();
        let len = len / (std::mem::size_of::<f32>() * 4);
        std::slice::from_raw_parts(data, len)
    }
}

#[inline]
pub(crate) fn get_vec3_bytes(
    glb: &'static gltf::Glb,
    acc: &gltf::Accessor<'static>,
) -> &'static [[f32; 3]] {
    // Get the buffer view for the accessor. Must exist
    let view = acc.view().unwrap();

    // Must be a f32 data
    if acc.data_type() != ComponentType::F32 {
        panic!()
    }

    // Must be a 3 component vector
    if acc.dimensions() != Type::Vec3 {
        panic!()
    }

    // Data must be contained in the BIN section of a binary GLTF file
    match view.buffer().source() {
        Source::Bin => {}
        _ => panic!(),
    }

    // Get the BIN data slice from the gltf. Must exist
    let bin = glb.bin.as_ref().unwrap().deref();

    unsafe {
        let data = bin.as_ptr().add(view.offset());
        let data = data as *const [f32; 3];
        let len = view.length();
        let len = len / (std::mem::size_of::<f32>() * 3);
        std::slice::from_raw_parts(data, len)
    }
}

#[inline]
pub(crate) fn get_vec2_bytes(
    glb: &'static gltf::Glb,
    acc: &gltf::Accessor<'static>,
) -> &'static [[f32; 2]] {
    // Get the buffer view for the accessor. Must exist
    let view = acc.view().unwrap();

    // Must be a f32 data
    if acc.data_type() != ComponentType::F32 {
        panic!()
    }

    // Must be a 3 component vector
    if acc.dimensions() != Type::Vec2 {
        panic!()
    }

    // Data must be contained in the BIN section of a binary GLTF file
    match view.buffer().source() {
        Source::Bin => {}
        _ => panic!(),
    }

    // Get the BIN data slice from the gltf. Must exist
    let bin = glb.bin.as_ref().unwrap().deref();

    unsafe {
        let data = bin.as_ptr().add(view.offset());
        let data = data as *const [f32; 2];
        let len = view.length();
        let len = len / (std::mem::size_of::<f32>() * 2);
        std::slice::from_raw_parts(data, len)
    }
}

#[inline]
pub(crate) fn get_u16_bytes(
    glb: &'static gltf::Glb,
    acc: &gltf::Accessor<'static>,
) -> &'static [u16] {
    // Get the buffer view for the accessor. Must exist
    let view = acc.view().unwrap();

    // Must be a U32 data
    if acc.data_type() != ComponentType::U16 {
        panic!()
    }

    // Must be a scalar
    if acc.dimensions() != Type::Scalar {
        panic!()
    }

    // Data must be contained in the BIN section of a binary GLTF file
    match view.buffer().source() {
        Source::Bin => {}
        _ => panic!(),
    }

    // Get the BIN data slice from the gltf. Must exist
    let bin = glb.bin.as_ref().unwrap().deref();

    unsafe {
        let data = bin.as_ptr().add(view.offset());
        let data = data as *const u16;
        let len = view.length();
        let len = len / (std::mem::size_of::<u16>());
        std::slice::from_raw_parts(data, len)
    }
}
