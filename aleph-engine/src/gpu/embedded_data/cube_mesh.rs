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
use gltf::json::mesh::Mode;
use gltf::{Glb, Gltf, Semantic};
use once_cell::sync::{Lazy, OnceCell};
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
    let acc =
        super::utils::get_accessor_for(&CUBE_MESH_GLTF, &Semantic::Positions, Mode::Triangles);
    super::utils::get_vec3_bytes(&CUBE_MESH_GLB, &acc)
});

///
/// Internal global data for built in mesh normals data
///
static CUBE_MESH_NORMALS: Lazy<&'static [[f32; 3]]> = Lazy::new(|| {
    let acc = super::utils::get_accessor_for(&CUBE_MESH_GLTF, &Semantic::Normals, Mode::Triangles);
    super::utils::get_vec3_bytes(&CUBE_MESH_GLB, &acc)
});

///
/// Internal global data for built in mesh tangents data
///
static CUBE_MESH_TANGENTS: Lazy<&'static [[f32; 4]]> = Lazy::new(|| {
    let acc = super::utils::get_accessor_for(&CUBE_MESH_GLTF, &Semantic::Tangents, Mode::Triangles);
    super::utils::get_vec4_bytes(&CUBE_MESH_GLB, &acc)
});

///
/// Internal global data for built in mesh uv data
///
static CUBE_MESH_UV: Lazy<&'static [[f32; 2]]> = Lazy::new(|| {
    let acc =
        super::utils::get_accessor_for(&CUBE_MESH_GLTF, &Semantic::TexCoords(0), Mode::Triangles);
    super::utils::get_vec2_bytes(&CUBE_MESH_GLB, &acc)
});

///
/// Internal global data for built in mesh index data
///
static CUBE_MESH_IND: Lazy<&'static [u16]> = Lazy::new(|| {
    let acc = super::utils::get_accessor_for_indices(&CUBE_MESH_GLTF, Mode::Triangles);
    super::utils::get_u16_bytes(&CUBE_MESH_GLB, &acc)
});

static CUBE_MESH_BUFFERS: OnceCell<super::utils::StaticMeshBuffers> = OnceCell::new();

///
/// A singleton struct that represents a built in cube mesh
///
pub struct CubeMesh {}

impl CubeMesh {
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

        CUBE_MESH_BUFFERS
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
        let buffers = CUBE_MESH_BUFFERS.get().unwrap();
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
        let buffers = CUBE_MESH_BUFFERS.get().unwrap();
        unsafe {
            buffers.destroy_buffers(allocator);
        }
    }

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

    ///
    /// Get the raw index data
    ///
    pub fn indices() -> &'static [u16] {
        CUBE_MESH_IND.deref()
    }

    ///
    /// Get the raw vertex position data
    ///
    pub fn position_buffer() -> Buffer {
        CUBE_MESH_BUFFERS.get().unwrap().positions()
    }

    ///
    /// Get the raw vertex normal data
    ///
    pub fn normal_buffer() -> Buffer {
        CUBE_MESH_BUFFERS.get().unwrap().normals()
    }

    ///
    /// Get the raw vertex tangent data
    ///
    pub fn tangent_buffer() -> Buffer {
        CUBE_MESH_BUFFERS.get().unwrap().tangents()
    }

    ///
    /// Get the raw vertex texcoord data
    ///
    pub fn uv_buffer() -> Buffer {
        CUBE_MESH_BUFFERS.get().unwrap().uv()
    }

    ///
    /// Get the raw index data
    ///
    pub fn index_buffer() -> Buffer {
        CUBE_MESH_BUFFERS.get().unwrap().indices()
    }
}
