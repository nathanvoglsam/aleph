//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use once_cell::sync::OnceCell;
use crate::vulkan_alloc::Allocator;
use crate::vulkan_core::erupt::vk1_0::{CommandBuffer, Buffer};

mod utils;

static CUBE_MESH_BUFFERS: OnceCell<self::utils::StaticMeshBuffers> = OnceCell::new();

///
/// Namespace structs for the global, always usable cube mesh vertex buffers
///
pub struct CubeMeshBuffers {}

impl CubeMeshBuffers {
    ///
    /// Allocates the buffers and prepares everything for staging the data to the GPU. Records the
    /// staging commands into the given command buffer, including a pipeline barrier, which can then
    /// be queued to the GPU to stage the vertex buffers.
    ///
    pub fn init_buffers(allocator: &Allocator, command_buffer: CommandBuffer) {
        let buffers = self::utils::StaticMeshBuffers::new(
            allocator,
            super::data::CubeMesh::positions(),
            super::data::CubeMesh::normals(),
            super::data::CubeMesh::tangents(),
            super::data::CubeMesh::uv(),
            super::data::CubeMesh::indices(),
        );
        buffers.record_buffer_staging(allocator.device(), command_buffer);

        CUBE_MESH_BUFFERS
            .set(buffers)
            .ok()
            .expect("Mesh buffers already initialized");
    }

    ///
    /// Get the raw vertex position data
    ///
    pub fn positions() -> Buffer {
        CUBE_MESH_BUFFERS.get().unwrap().positions()
    }

    ///
    /// Get the raw vertex normal data
    ///
    pub fn normals() -> Buffer {
        CUBE_MESH_BUFFERS.get().unwrap().normals()
    }

    ///
    /// Get the raw vertex tangent data
    ///
    pub fn tangents() -> Buffer {
        CUBE_MESH_BUFFERS.get().unwrap().tangents()
    }

    ///
    /// Get the raw vertex texcoord data
    ///
    pub fn uvs() -> Buffer {
        CUBE_MESH_BUFFERS.get().unwrap().uv()
    }

    ///
    /// Get the raw index data
    ///
    pub fn indices() -> Buffer {
        CUBE_MESH_BUFFERS.get().unwrap().indices()
    }
}

static SPHERE_MESH_BUFFERS: OnceCell<self::utils::StaticMeshBuffers> = OnceCell::new();

///
/// Namespace structs for the global, always usable sphere mesh vertex buffers
///
pub struct SphereMeshBuffers {}

impl SphereMeshBuffers {
    ///
    /// Allocates the buffers and prepares everything for staging the data to the GPU. Records the
    /// staging commands into the given command buffer, including a pipeline barrier, which can then
    /// be queued to the GPU to stage the vertex buffers.
    ///
    pub fn init_buffers(allocator: &Allocator, command_buffer: CommandBuffer) {
        let buffers = self::utils::StaticMeshBuffers::new(
            allocator,
            super::data::SphereMesh::positions(),
            super::data::SphereMesh::normals(),
            super::data::SphereMesh::tangents(),
            super::data::SphereMesh::uv(),
            super::data::SphereMesh::indices(),
        );
        buffers.record_buffer_staging(allocator.device(), command_buffer);

        SPHERE_MESH_BUFFERS
            .set(buffers)
            .ok()
            .expect("Mesh buffers already initialized");
    }

    ///
    /// Get the raw vertex position data
    ///
    pub fn positions() -> Buffer {
        SPHERE_MESH_BUFFERS.get().unwrap().positions()
    }

    ///
    /// Get the raw vertex normal data
    ///
    pub fn normals() -> Buffer {
        SPHERE_MESH_BUFFERS.get().unwrap().normals()
    }

    ///
    /// Get the raw vertex tangent data
    ///
    pub fn tangents() -> Buffer {
        SPHERE_MESH_BUFFERS.get().unwrap().tangents()
    }

    ///
    /// Get the raw vertex texcoord data
    ///
    pub fn uvs() -> Buffer {
        SPHERE_MESH_BUFFERS.get().unwrap().uv()
    }

    ///
    /// Get the raw index data
    ///
    pub fn indices() -> Buffer {
        SPHERE_MESH_BUFFERS.get().unwrap().indices()
    }
}

static FS_QUAD_BUFFER: OnceCell<self::utils::PosOnlyMeshBuffers> = OnceCell::new();

///
/// A namespace struct for a fullscreen quad
///
pub struct FullscreenQuadBuffers {}

impl FullscreenQuadBuffers {
    ///
    /// Allocates the buffers and prepares everything for staging the data to the GPU. Records the
    /// staging commands into the given command buffer, including a pipeline barrier, which can then
    /// be queued to the GPU to stage the vertex buffers.
    ///
    pub fn init_buffers(allocator: &Allocator, command_buffer: CommandBuffer) {
        let buffers =
            self::utils::PosOnlyMeshBuffers::new(allocator, super::data::FullscreenQuad::positions(), super::data::FullscreenQuad::indices());
        buffers.record_buffer_staging(allocator.device(), command_buffer);

        FS_QUAD_BUFFER
            .set(buffers)
            .ok()
            .expect("Mesh buffers already initialized");
    }

    ///
    /// Get the raw vertex position data
    ///
    pub fn position_buffer() -> Buffer {
        FS_QUAD_BUFFER.get().unwrap().positions()
    }

    ///
    /// Get the raw index data
    ///
    pub fn index_buffer() -> Buffer {
        FS_QUAD_BUFFER.get().unwrap().indices()
    }
}