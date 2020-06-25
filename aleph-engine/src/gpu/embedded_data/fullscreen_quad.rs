//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use crate::gpu::embedded_data::utils::PosOnlyMeshBuffers;
use crate::gpu::vk::alloc::Allocator;
use erupt::vk1_0::{Buffer, CommandBuffer};
use once_cell::sync::OnceCell;

static FS_QUAD_BUFFER: OnceCell<PosOnlyMeshBuffers> = OnceCell::new();

///
/// A namespace struct for a fullscreen quad
///
pub struct FullscreenQuad {}

impl FullscreenQuad {
    ///
    /// Allocates the buffers and prepares everything for staging the data to the GPU. Records the
    /// staging commands into the given command buffer, including a pipeline barrier, which can then
    /// be queued to the GPU to stage the vertex buffers.
    ///
    pub fn init_buffers(allocator: &Allocator, command_buffer: CommandBuffer) {
        let buffers =
            super::utils::PosOnlyMeshBuffers::new(allocator, Self::positions(), Self::indices());
        buffers.record_buffer_staging(allocator.device(), command_buffer);

        FS_QUAD_BUFFER
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
        let buffers = FS_QUAD_BUFFER.get().unwrap();
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
        let buffers = FS_QUAD_BUFFER.get().unwrap();
        unsafe {
            buffers.destroy_buffers(allocator);
        }
    }

    ///
    /// Gets the vertex position buffer for a fullscreen quad
    ///
    pub fn positions() -> &'static [[f32; 2]; 4] {
        static POS: [[f32; 2]; 4] = [[-1.0, -1.0], [1.0, -1.0], [1.0, 1.0], [-1.0, 1.0]];
        &POS
    }

    ///
    /// Gets the index buffer for a fullscreen quad
    ///
    pub fn indices() -> &'static [u32; 6] {
        static IND: [u32; 6] = [0, 1, 2, 3, 0, 2];
        &IND
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
