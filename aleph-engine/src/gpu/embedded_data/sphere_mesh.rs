//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use crate::gpu::vk::alloc::Allocator;
use erupt::vk1_0::{Buffer, CommandBuffer};
use gltf::mesh::Mode;
use gltf::{Glb, Gltf, Semantic};
use once_cell::sync::{Lazy, OnceCell};
use std::ops::Deref;

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

static SPHERE_MESH_BUFFERS: OnceCell<super::utils::StaticMeshBuffers> = OnceCell::new();

///
/// A singleton struct that represents a built in sphere mesh
///
pub struct SphereMesh {}

impl SphereMesh {
    ///
    /// Allocates the buffers and prepares everything for staging the data to the GPU. Records the
    /// staging commands into the given command buffer, including a pipeline barrier, which can then
    /// be queued to the GPU to stage the vertex buffers.
    ///
    pub fn init_buffers(allocator: &Allocator, command_buffer: CommandBuffer) {
        let buffers = super::utils::StaticMeshBuffers::new(
            allocator,
            Self::positions(),
            Self::normals(),
            Self::tangents(),
            Self::uv(),
            Self::indices(),
        );
        buffers.record_buffer_staging(allocator.device(), command_buffer);

        SPHERE_MESH_BUFFERS
            .set(buffers)
            .ok()
            .expect("Mesh buffers already initialized");

        allocator.defer(Self::destroy_staging_buffers);
        allocator.defer(Self::destroy_buffers);
    }

    ///
    /// Destroys the staging buffers used to upload the vertex data
    ///
    /// Unsafe as destruction is not synchronized
    ///
    fn destroy_staging_buffers(allocator: &Allocator) {
        let buffers = SPHERE_MESH_BUFFERS.get().unwrap();
        unsafe {
            buffers.destroy_staging_buffers(allocator);
        }
    }

    ///
    /// Destroys the vertex buffers
    ///
    /// Unsafe as destruction is not synchronized
    ///
    fn destroy_buffers(allocator: &Allocator) {
        let buffers = SPHERE_MESH_BUFFERS.get().unwrap();
        unsafe {
            buffers.destroy_buffers(allocator);
        }
    }

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

    ///
    /// Get the raw vertex position buffer
    ///
    pub fn position_buffer() -> Buffer {
        SPHERE_MESH_BUFFERS.get().unwrap().positions()
    }

    ///
    /// Get the raw vertex normal buffer
    ///
    pub fn normal_buffer() -> Buffer {
        SPHERE_MESH_BUFFERS.get().unwrap().normals()
    }

    ///
    /// Get the raw vertex tangent buffer
    ///
    pub fn tangent_buffer() -> Buffer {
        SPHERE_MESH_BUFFERS.get().unwrap().tangents()
    }

    ///
    /// Get the raw vertex texcoord buffer
    ///
    pub fn uv_buffer() -> Buffer {
        SPHERE_MESH_BUFFERS.get().unwrap().uv()
    }

    ///
    /// Get the raw index data
    ///
    pub fn index_buffer() -> Buffer {
        SPHERE_MESH_BUFFERS.get().unwrap().indices()
    }
}
