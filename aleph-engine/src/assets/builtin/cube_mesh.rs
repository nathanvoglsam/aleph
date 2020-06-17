//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use gltf::json::mesh::Mode;
use gltf::{Glb, Gltf, Semantic};
use once_cell::sync::Lazy;
use std::ops::Deref;

const ERROR_MSG: &'static str = "Builtin Cube Mesh Invalid";

///
/// Internal global data for built in mesh gltf GLB
///
static CUBE_MESH_GLB: Lazy<Glb<'static>> = Lazy::new(|| {
    let glb = Glb::from_slice(include_bytes!("../../../models/cube.glb")).expect(ERROR_MSG);
    glb
});

///
/// Internal global data for built in mesh gltf document
///
static CUBE_MESH_GLTF: Lazy<Gltf> = Lazy::new(|| {
    let gltf = Gltf::from_slice(&CUBE_MESH_GLB.json).expect(ERROR_MSG);
    gltf
});

///
/// Internal global data for built in mesh position data
///
static CUBE_MESH_POSITIONS: Lazy<&'static [[f32; 3]]> = Lazy::new(|| {
    let acc = super::builtin_utils::get_accessor_for(
        &CUBE_MESH_GLTF,
        &Semantic::Positions,
        Mode::Triangles,
    );
    super::builtin_utils::get_vec3_bytes(&CUBE_MESH_GLB, &acc)
});

///
/// Internal global data for built in mesh normals data
///
static CUBE_MESH_NORMALS: Lazy<&'static [[f32; 3]]> = Lazy::new(|| {
    let acc = super::builtin_utils::get_accessor_for(
        &CUBE_MESH_GLTF,
        &Semantic::Normals,
        Mode::Triangles,
    );
    super::builtin_utils::get_vec3_bytes(&CUBE_MESH_GLB, &acc)
});

///
/// Internal global data for built in mesh tangents data
///
static CUBE_MESH_TANGENTS: Lazy<&'static [[f32; 4]]> = Lazy::new(|| {
    let acc = super::builtin_utils::get_accessor_for(
        &CUBE_MESH_GLTF,
        &Semantic::Tangents,
        Mode::Triangles,
    );
    super::builtin_utils::get_vec4_bytes(&CUBE_MESH_GLB, &acc)
});

///
/// Internal global data for built in mesh uv data
///
static CUBE_MESH_UV: Lazy<&'static [[f32; 2]]> = Lazy::new(|| {
    let acc = super::builtin_utils::get_accessor_for(
        &CUBE_MESH_GLTF,
        &Semantic::TexCoords(0),
        Mode::Triangles,
    );
    super::builtin_utils::get_vec2_bytes(&CUBE_MESH_GLB, &acc)
});

///
/// A singleton struct that represents a built in cube mesh
///
pub struct CubeMesh {}

impl CubeMesh {
    ///
    /// Get the cube mesh GLB object
    ///
    pub fn glb() -> &'static Glb<'static> {
        CUBE_MESH_GLB.deref()
    }

    ///
    /// Get the cube mesh GLTF object
    ///
    pub fn gltf() -> &'static Gltf {
        CUBE_MESH_GLTF.deref()
    }

    ///
    /// Get the raw vertex position data
    ///
    pub fn positions() -> &'static [[f32; 3]] {
        CUBE_MESH_POSITIONS.deref()
    }

    ///
    /// Get the raw vertex normal data
    ///
    pub fn normals() -> &'static [[f32; 3]] {
        CUBE_MESH_NORMALS.deref()
    }

    ///
    /// Get the raw vertex tangent data
    ///
    pub fn tangents() -> &'static [[f32; 4]] {
        CUBE_MESH_TANGENTS.deref()
    }

    ///
    /// Get the raw vertex texcoord data
    ///
    pub fn uv() -> &'static [[f32; 2]] {
        CUBE_MESH_UV.deref()
    }
}
