//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use gltf::{Glb, Gltf, Semantic};
use once_cell::sync::Lazy;
use std::ops::Deref;
use gltf::mesh::Mode;

///
/// Internal global data for built in mesh gltf GLB
///
static SPHERE_MESH_GLB: Lazy<Glb<'static>> = Lazy::new(|| {
    let glb = Glb::from_slice(include_bytes!("../../../models/sphere.glb"))
        .expect("Builtin Sphere Mesh Invalid");
    glb
});

///
/// Internal global data for built in mesh gltf document
///
static SPHERE_MESH_GLTF: Lazy<Gltf> = Lazy::new(|| {
    let gltf = Gltf::from_slice(&SPHERE_MESH_GLB.json).expect("Builtin Sphere Mesh Invalid");
    gltf
});

///
/// Internal global data for built in mesh position data
///
static SPHERE_MESH_POSITIONS: Lazy<&'static [[f32; 3]]> = Lazy::new(|| {
    let acc = super::builtin_utils::get_accessor_for(&SPHERE_MESH_GLTF, &Semantic::Positions, Mode::Triangles);
    super::builtin_utils::get_vec3_bytes(&SPHERE_MESH_GLB, &acc)
});

///
/// Internal global data for built in mesh normals data
///
static SPHERE_MESH_NORMALS: Lazy<&'static [[f32; 3]]> = Lazy::new(|| {
    let acc = super::builtin_utils::get_accessor_for(&SPHERE_MESH_GLTF, &Semantic::Normals, Mode::Triangles);
    super::builtin_utils::get_vec3_bytes(&SPHERE_MESH_GLB, &acc)
});

///
/// Internal global data for built in mesh tangents data
///
static SPHERE_MESH_TANGENTS: Lazy<&'static [[f32; 4]]> = Lazy::new(|| {
    let acc = super::builtin_utils::get_accessor_for(&SPHERE_MESH_GLTF, &Semantic::Tangents, Mode::Triangles);
    super::builtin_utils::get_vec4_bytes(&SPHERE_MESH_GLB, &acc)
});

///
/// Internal global data for built in mesh uv data
///
static SPHERE_MESH_UV: Lazy<&'static [[f32; 2]]> = Lazy::new(|| {
    let acc = super::builtin_utils::get_accessor_for(&SPHERE_MESH_GLTF, &Semantic::TexCoords(0), Mode::Triangles);
    super::builtin_utils::get_vec2_bytes(&SPHERE_MESH_GLB, &acc)
});

///
/// A singleton struct that represents a built in sphere mesh
///
pub struct SphereMesh {}

impl SphereMesh {
    ///
    /// Get the sphere mesh GLB object
    ///
    pub fn glb() -> &'static Glb<'static> {
        SPHERE_MESH_GLB.deref()
    }

    ///
    /// Get the sphere mesh GLTF object
    ///
    pub fn gltf() -> &'static Gltf {
        SPHERE_MESH_GLTF.deref()
    }

    ///
    /// Get the raw vertex position data
    ///
    pub fn positions() -> &'static [[f32;3]] {
        SPHERE_MESH_POSITIONS.deref()
    }

    ///
    /// Get the raw vertex normal data
    ///
    pub fn normals() -> &'static [[f32;3]] {
        SPHERE_MESH_NORMALS.deref()
    }

    ///
    /// Get the raw vertex tangent data
    ///
    pub fn tangents() -> &'static [[f32;4]] {
        SPHERE_MESH_TANGENTS.deref()
    }

    ///
    /// Get the raw vertex texcoord data
    ///
    pub fn uv() -> &'static [[f32;2]] {
        SPHERE_MESH_UV.deref()
    }
}
