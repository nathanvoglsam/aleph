//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use std::mem::size_of;
use vulkan_alloc::{Allocation, AllocationCreateInfoBuilder, Allocator, MemoryUsage};
use vulkan_core::erupt::vk1_0::Vk10DeviceLoaderExt;
use vulkan_core::erupt::vk1_0::{
    AccessFlags, Buffer, BufferCopyBuilder, BufferCreateInfoBuilder, BufferMemoryBarrierBuilder,
    BufferUsageFlags, CommandBuffer, DependencyFlags, PipelineStageFlags, SharingMode, WHOLE_SIZE,
};
use vulkan_core::Device;

///
/// Internal struct for holding the buffers for the cube mesh
///
pub struct StaticMeshBuffers {
    pos_buffer: (Buffer, Allocation),
    pos_staging: (Buffer, Allocation),
    pos_size: u64,
    nrm_buffer: (Buffer, Allocation),
    nrm_staging: (Buffer, Allocation),
    nrm_size: u64,
    tan_buffer: (Buffer, Allocation),
    tan_staging: (Buffer, Allocation),
    tan_size: u64,
    uv_buffer: (Buffer, Allocation),
    uv_staging: (Buffer, Allocation),
    uv_size: u64,
    ind_buffer: (Buffer, Allocation),
    ind_staging: (Buffer, Allocation),
    ind_size: u64,
}

impl StaticMeshBuffers {
    ///
    /// Creates a new `StaticMeshBuffers` object. It allocates the vertex and staging buffers for
    /// each attribute, then maps and copies the buffer data into the staging buffers.
    ///
    /// Recording and upload to the GPU is deferred to a future call to `record_buffer_staging`
    /// which will record into a given command buffer the commands to upload from the staging buffer
    /// to the GPU resident vertex buffer.
    ///
    /// Once that command buffer has been submitted and executed it is safe to then destroy the
    /// staging buffers with a call to `destroy_staging_buffers`.
    ///
    pub fn new<Pos, Nrm, Tan, Uv, Ind>(
        allocator: &Allocator,
        pos: &[Pos],
        nrm: &[Nrm],
        tan: &[Tan],
        uv: &[Uv],
        ind: &[Ind],
    ) -> Self {
        // Allocate the staging and vertex buffer for the vertex positions
        let pos_staging = init_staging_buffer(allocator, pos);
        let pos_buffer = init_buffer(allocator, BufferUsageFlags::VERTEX_BUFFER, pos);
        let pos_size = buffer_size(pos);

        // Allocate the staging and vertex buffer for the vertex normals
        let nrm_staging = init_staging_buffer(allocator, nrm);
        let nrm_buffer = init_buffer(allocator, BufferUsageFlags::VERTEX_BUFFER, nrm);
        let nrm_size = buffer_size(nrm);

        // Allocate the staging and vertex buffer for the vertex tangents
        let tan_staging = init_staging_buffer(allocator, tan);
        let tan_buffer = init_buffer(allocator, BufferUsageFlags::VERTEX_BUFFER, tan);
        let tan_size = buffer_size(tan);

        // Allocate the staging and vertex buffer for the vertex texture coordinates
        let uv_staging = init_staging_buffer(allocator, uv);
        let uv_buffer = init_buffer(allocator, BufferUsageFlags::VERTEX_BUFFER, uv);
        let uv_size = buffer_size(uv);

        // Allocate the staging and index buffer
        let ind_staging = init_staging_buffer(allocator, ind);
        let ind_buffer = init_buffer(allocator, BufferUsageFlags::INDEX_BUFFER, ind);
        let ind_size = buffer_size(ind);

        // Map, copy, and unmap the data for each buffer
        copy_to_buffer(allocator, &pos_staging.1, pos);
        copy_to_buffer(allocator, &nrm_staging.1, nrm);
        copy_to_buffer(allocator, &tan_staging.1, tan);
        copy_to_buffer(allocator, &uv_staging.1, uv);
        copy_to_buffer(allocator, &ind_staging.1, ind);

        // Defer the destruction of these buffers until the allocator is being destroyed as we want
        // these to be valid for the entire runtime of the graphics system
        allocator.defer_destruction(pos_staging);
        allocator.defer_destruction(pos_buffer);
        allocator.defer_destruction(nrm_staging);
        allocator.defer_destruction(nrm_buffer);
        allocator.defer_destruction(tan_staging);
        allocator.defer_destruction(tan_buffer);
        allocator.defer_destruction(uv_staging);
        allocator.defer_destruction(uv_buffer);
        allocator.defer_destruction(ind_staging);
        allocator.defer_destruction(ind_buffer);

        Self {
            pos_buffer,
            pos_staging,
            pos_size,
            nrm_buffer,
            nrm_staging,
            nrm_size,
            tan_buffer,
            tan_staging,
            tan_size,
            uv_buffer,
            uv_staging,
            uv_size,
            ind_buffer,
            ind_staging,
            ind_size,
        }
    }

    ///
    /// Gets the positions buffer
    ///
    pub fn positions(&self) -> Buffer {
        self.pos_buffer.0
    }

    ///
    /// Gets the normals buffer
    ///
    pub fn normals(&self) -> Buffer {
        self.nrm_buffer.0
    }

    ///
    /// Gets the tangents buffer
    ///
    pub fn tangents(&self) -> Buffer {
        self.tan_buffer.0
    }

    ///
    /// Gets the uv buffer
    ///
    pub fn uv(&self) -> Buffer {
        self.uv_buffer.0
    }

    ///
    /// Gets the index buffer
    ///
    pub fn indices(&self) -> Buffer {
        self.ind_buffer.0
    }

    pub fn record_buffer_staging(&self, device: &Device, command_buffer: CommandBuffer) {
        unsafe {
            let region = BufferCopyBuilder::new()
                .size(self.pos_size)
                .dst_offset(0)
                .src_offset(0);
            device.loader().cmd_copy_buffer(
                command_buffer,
                self.pos_staging.0,
                self.pos_buffer.0,
                &[region],
            );

            let region = BufferCopyBuilder::new()
                .size(self.nrm_size)
                .dst_offset(0)
                .src_offset(0);
            device.loader().cmd_copy_buffer(
                command_buffer,
                self.nrm_staging.0,
                self.nrm_buffer.0,
                &[region],
            );

            let region = BufferCopyBuilder::new()
                .size(self.tan_size)
                .dst_offset(0)
                .src_offset(0);
            device.loader().cmd_copy_buffer(
                command_buffer,
                self.tan_staging.0,
                self.tan_buffer.0,
                &[region],
            );

            let region = BufferCopyBuilder::new()
                .size(self.uv_size)
                .dst_offset(0)
                .src_offset(0);
            device.loader().cmd_copy_buffer(
                command_buffer,
                self.uv_staging.0,
                self.uv_buffer.0,
                &[region],
            );

            let region = BufferCopyBuilder::new()
                .size(self.ind_size)
                .dst_offset(0)
                .src_offset(0);
            device.loader().cmd_copy_buffer(
                command_buffer,
                self.ind_staging.0,
                self.ind_buffer.0,
                &[region],
            );

            let pos_barrier = buffer_barrier(device, self.pos_buffer.0);
            let nrm_barrier = buffer_barrier(device, self.nrm_buffer.0);
            let tan_barrier = buffer_barrier(device, self.tan_buffer.0);
            let uv_barrier = buffer_barrier(device, self.uv_buffer.0);
            let ind_barrier = buffer_barrier(device, self.ind_buffer.0);
            device.loader().cmd_pipeline_barrier(
                command_buffer,
                PipelineStageFlags::TRANSFER,
                PipelineStageFlags::TOP_OF_PIPE,
                DependencyFlags::default(),
                &[],
                &[
                    pos_barrier,
                    nrm_barrier,
                    tan_barrier,
                    uv_barrier,
                    ind_barrier,
                ],
                &[],
            );
        }
    }
}

///
/// Internal struct for holding the buffers for the cube mesh
///
pub struct PosOnlyMeshBuffers {
    pos_buffer: (Buffer, Allocation),
    pos_staging: (Buffer, Allocation),
    pos_size: u64,
    ind_buffer: (Buffer, Allocation),
    ind_staging: (Buffer, Allocation),
    ind_size: u64,
}

impl PosOnlyMeshBuffers {
    ///
    /// Creates a new `StaticMeshBuffers` object. It allocates the vertex and staging buffers for
    /// each attribute, then maps and copies the buffer data into the staging buffers.
    ///
    /// Recording and upload to the GPU is deferred to a future call to `record_buffer_staging`
    /// which will record into a given command buffer the commands to upload from the staging buffer
    /// to the GPU resident vertex buffer.
    ///
    /// Once that command buffer has been submitted and executed it is safe to then destroy the
    /// staging buffers with a call to `destroy_staging_buffers`.
    ///
    pub fn new<Pos, Ind>(allocator: &Allocator, pos: &[Pos], ind: &[Ind]) -> Self {
        // Allocate the staging and vertex buffer for the vertex positions
        let pos_staging = init_staging_buffer(allocator, pos);
        let pos_buffer = init_buffer(allocator, BufferUsageFlags::VERTEX_BUFFER, pos);
        let pos_size = buffer_size(pos);

        // Allocate the staging and index buffer
        let ind_staging = init_staging_buffer(allocator, pos);
        let ind_buffer = init_buffer(allocator, BufferUsageFlags::INDEX_BUFFER, ind);
        let ind_size = buffer_size(ind);

        // Map, copy and unmap the vertex data
        copy_to_buffer(allocator, &pos_staging.1, pos);
        copy_to_buffer(allocator, &ind_staging.1, ind);

        // Defer destruction of these buffers until the allocator is being destroyed as we want
        // these buffers to live for the entire lifetime of the graphics subsystem
        allocator.defer_destruction(pos_staging);
        allocator.defer_destruction(pos_buffer);
        allocator.defer_destruction(ind_staging);
        allocator.defer_destruction(ind_buffer);

        Self {
            pos_buffer,
            pos_staging,
            pos_size,
            ind_buffer,
            ind_staging,
            ind_size,
        }
    }

    ///
    /// Gets the positions buffer
    ///
    pub fn positions(&self) -> Buffer {
        self.pos_buffer.0
    }

    ///
    /// Gets the index buffer
    ///
    pub fn indices(&self) -> Buffer {
        self.ind_buffer.0
    }

    pub fn record_buffer_staging(&self, device: &Device, command_buffer: CommandBuffer) {
        unsafe {
            let region = BufferCopyBuilder::new()
                .size(self.pos_size)
                .dst_offset(0)
                .src_offset(0);
            device.loader().cmd_copy_buffer(
                command_buffer,
                self.pos_staging.0,
                self.pos_buffer.0,
                &[region],
            );

            let region = BufferCopyBuilder::new()
                .size(self.ind_size)
                .dst_offset(0)
                .src_offset(0);
            device.loader().cmd_copy_buffer(
                command_buffer,
                self.ind_staging.0,
                self.ind_buffer.0,
                &[region],
            );

            let pos_barrier = buffer_barrier(device, self.pos_buffer.0);
            let ind_barrier = buffer_barrier(device, self.ind_buffer.0);
            device.loader().cmd_pipeline_barrier(
                command_buffer,
                PipelineStageFlags::BOTTOM_OF_PIPE,
                PipelineStageFlags::TOP_OF_PIPE,
                DependencyFlags::default(),
                &[],
                &[pos_barrier, ind_barrier],
                &[],
            );
        }
    }
}

///
/// Internal function for creating a buffer barrier for synchronizing the vertex buffer's usage
/// with the rest of the program after intial upload
///
fn buffer_barrier(device: &Device, buffer: Buffer) -> BufferMemoryBarrierBuilder<'static> {
    BufferMemoryBarrierBuilder::new()
        .size(WHOLE_SIZE)
        .offset(0)
        .buffer(buffer)
        .src_queue_family_index(device.general_family().index)
        .dst_queue_family_index(device.general_family().index)
        .src_access_mask(AccessFlags::MEMORY_WRITE)
        .dst_access_mask(AccessFlags::MEMORY_READ)
}

///
/// Performs the copy from the source buffer memory into the staging buffer
///
fn copy_to_buffer<T>(allocator: &Allocator, allocation: &Allocation, buffer: &[T]) {
    unsafe {
        // Get the buffer as bytes, and the number of bytes to write
        let src = buffer.as_ptr() as *const u8;
        let count = buffer_size(buffer) as usize;

        // Map the memory
        let ptr = allocator
            .map_memory(allocation)
            .expect("Failed to map memory");

        // Copy the buffer data
        ptr.copy_from(src, count);

        // Unmap the memory
        allocator.unmap_memory(allocation);
    }
}

///
/// Creates a staging buffer
///
fn init_staging_buffer<T>(allocator: &Allocator, buffer: &[T]) -> (Buffer, Allocation) {
    let alloc_create_info = AllocationCreateInfoBuilder::new().usage(MemoryUsage::CPUOnly);

    let size = buffer_size(buffer);
    let buffer_create_info = BufferCreateInfoBuilder::new()
        .usage(BufferUsageFlags::TRANSFER_SRC)
        .sharing_mode(SharingMode::EXCLUSIVE)
        .size(size);
    unsafe {
        allocator
            .create_buffer(&buffer_create_info, &alloc_create_info)
            .expect("Failed to allocate staging buffer")
    }
}

///
/// Internal function for creating a buffer
///
fn init_buffer<T>(
    allocator: &Allocator,
    usage: BufferUsageFlags,
    buffer: &[T],
) -> (Buffer, Allocation) {
    let alloc_create_info = AllocationCreateInfoBuilder::new().usage(MemoryUsage::GPUOnly);

    let size = buffer_size(buffer);
    let buffer_create_info = BufferCreateInfoBuilder::new()
        .usage(BufferUsageFlags::TRANSFER_DST | usage)
        .sharing_mode(SharingMode::EXCLUSIVE)
        .size(size);
    unsafe {
        allocator
            .create_buffer(&buffer_create_info, &alloc_create_info)
            .expect("Failed to allocate staging buffer")
    }
}

///
/// Internal function for getting the size of a buffer
///
fn buffer_size<T>(slice: &[T]) -> u64 {
    slice.len() as u64 * size_of::<T>() as u64
}
