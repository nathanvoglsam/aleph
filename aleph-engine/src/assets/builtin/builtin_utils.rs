//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
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
