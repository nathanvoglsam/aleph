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

use gltf::mesh::Mode;
use gltf::{Glb, Gltf, Semantic};
use once_cell::sync::Lazy;
use std::ops::Deref;

///
/// Internal global data for built in mesh gltf GLB
///
static SPHERE_MESH_GLB: Lazy<Glb<'static>> = Lazy::new(|| {
    let glb = Glb::from_slice(include_bytes!("../models/sphere.glb"))
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
    let acc =
        super::utils::get_accessor_for(&SPHERE_MESH_GLTF, &Semantic::Positions, Mode::Triangles);
    super::utils::get_vec3_bytes(&SPHERE_MESH_GLB, &acc)
});

///
/// Internal global data for built in mesh normals data
///
static SPHERE_MESH_NORMALS: Lazy<&'static [[f32; 3]]> = Lazy::new(|| {
    let acc =
        super::utils::get_accessor_for(&SPHERE_MESH_GLTF, &Semantic::Normals, Mode::Triangles);
    super::utils::get_vec3_bytes(&SPHERE_MESH_GLB, &acc)
});

///
/// Internal global data for built in mesh tangents data
///
static SPHERE_MESH_TANGENTS: Lazy<&'static [[f32; 4]]> = Lazy::new(|| {
    let acc =
        super::utils::get_accessor_for(&SPHERE_MESH_GLTF, &Semantic::Tangents, Mode::Triangles);
    super::utils::get_vec4_bytes(&SPHERE_MESH_GLB, &acc)
});

///
/// Internal global data for built in mesh uv data
///
static SPHERE_MESH_UV: Lazy<&'static [[f32; 2]]> = Lazy::new(|| {
    let acc =
        super::utils::get_accessor_for(&SPHERE_MESH_GLTF, &Semantic::TexCoords(0), Mode::Triangles);
    super::utils::get_vec2_bytes(&SPHERE_MESH_GLB, &acc)
});

///
/// Internal global data for built in mesh index data
///
static SPHERE_MESH_IND: Lazy<&'static [u16]> = Lazy::new(|| {
    let acc = super::utils::get_accessor_for_indices(&SPHERE_MESH_GLTF, Mode::Triangles);
    super::utils::get_u16_bytes(&SPHERE_MESH_GLB, &acc)
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
    pub fn positions() -> &'static [[f32; 3]] {
        SPHERE_MESH_POSITIONS.deref()
    }

    ///
    /// Get the raw vertex normal data
    ///
    pub fn normals() -> &'static [[f32; 3]] {
        SPHERE_MESH_NORMALS.deref()
    }

    ///
    /// Get the raw vertex tangent data
    ///
    pub fn tangents() -> &'static [[f32; 4]] {
        SPHERE_MESH_TANGENTS.deref()
    }

    ///
    /// Get the raw vertex texcoord data
    ///
    pub fn uv() -> &'static [[f32; 2]] {
        SPHERE_MESH_UV.deref()
    }

    ///
    /// Get the raw index data
    ///
    pub fn indices() -> &'static [u16] {
        SPHERE_MESH_IND.deref()
    }
}
