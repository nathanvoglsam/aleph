//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use crate::gpu::vk::alloc::{Allocation, AllocationCreateInfoBuilder, Allocator, MemoryUsage};
use crate::gpu::vk::core::Device;
use erupt::vk1_0::{
    AccessFlags, Buffer, BufferCopyBuilder, BufferCreateInfoBuilder, BufferMemoryBarrierBuilder,
    BufferUsageFlags, CommandBuffer, DependencyFlags, PipelineStageFlags, SharingMode,
    Vk10DeviceLoaderExt, WHOLE_SIZE,
};
use gltf::buffer::Source;
use gltf::json::accessor::{ComponentType, Type};
use std::mem::size_of;
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
        let pos_staging = init_staging_buffer(allocator, pos);
        let pos_buffer = init_buffer(allocator, BufferUsageFlags::VERTEX_BUFFER, pos);
        let pos_size = buffer_size(pos);

        let nrm_staging = init_staging_buffer(allocator, nrm);
        let nrm_buffer = init_buffer(allocator, BufferUsageFlags::VERTEX_BUFFER, nrm);
        let nrm_size = buffer_size(nrm);

        let tan_staging = init_staging_buffer(allocator, tan);
        let tan_buffer = init_buffer(allocator, BufferUsageFlags::VERTEX_BUFFER, tan);
        let tan_size = buffer_size(tan);

        let uv_staging = init_staging_buffer(allocator, uv);
        let uv_buffer = init_buffer(allocator, BufferUsageFlags::VERTEX_BUFFER, uv);
        let uv_size = buffer_size(uv);

        let ind_staging = init_staging_buffer(allocator, ind);
        let ind_buffer = init_buffer(allocator, BufferUsageFlags::INDEX_BUFFER, ind);
        let ind_size = buffer_size(ind);

        copy_to_buffer(allocator, &pos_staging.1, pos);
        copy_to_buffer(allocator, &nrm_staging.1, nrm);
        copy_to_buffer(allocator, &tan_staging.1, tan);
        copy_to_buffer(allocator, &uv_staging.1, uv);
        copy_to_buffer(allocator, &ind_staging.1, uv);

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

    ///
    /// Destroys the staging buffers for the cube mesh as these only need to exist once but have to
    /// live long enough for the `vkCmdCopyBuffer` calls to get run on a queue so their destruction
    /// needs to be delayed a little.
    ///
    /// Unsafe as the destruction is not synchronized
    ///
    pub unsafe fn destroy_staging_buffers(&self, allocator: &Allocator) {
        destroy_buffer(allocator, &self.pos_staging);
        destroy_buffer(allocator, &self.nrm_staging);
        destroy_buffer(allocator, &self.tan_staging);
        destroy_buffer(allocator, &self.uv_staging);
        destroy_buffer(allocator, &self.ind_staging);
    }

    ///
    /// Destroys the actual vertex buffers for when the engine is shutting down.
    ///
    /// Unsafe as the destruction is not synchronized
    ///
    pub unsafe fn destroy_buffers(&self, allocator: &Allocator) {
        destroy_buffer(allocator, &self.pos_buffer);
        destroy_buffer(allocator, &self.nrm_buffer);
        destroy_buffer(allocator, &self.tan_buffer);
        destroy_buffer(allocator, &self.uv_buffer);
        destroy_buffer(allocator, &self.ind_buffer);
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
        let pos_staging = init_staging_buffer(allocator, pos);
        let pos_buffer = init_buffer(allocator, BufferUsageFlags::VERTEX_BUFFER, pos);
        let pos_size = buffer_size(pos);

        let ind_staging = init_staging_buffer(allocator, pos);
        let ind_buffer = init_buffer(allocator, BufferUsageFlags::INDEX_BUFFER, ind);
        let ind_size = buffer_size(ind);

        copy_to_buffer(allocator, &pos_staging.1, pos);
        copy_to_buffer(allocator, &ind_staging.1, ind);

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

    ///
    /// Destroys the staging buffers for the cube mesh as these only need to exist once but have to
    /// live long enough for the `vkCmdCopyBuffer` calls to get run on a queue so their destruction
    /// needs to be delayed a little.
    ///
    /// Unsafe as the destruction is not synchronized
    ///
    pub unsafe fn destroy_staging_buffers(&self, allocator: &Allocator) {
        destroy_buffer(allocator, &self.pos_staging);
        destroy_buffer(allocator, &self.ind_staging);
    }

    ///
    /// Destroys the actual vertex buffers for when the engine is shutting down.
    ///
    /// Unsafe as the destruction is not synchronized
    ///
    pub unsafe fn destroy_buffers(&self, allocator: &Allocator) {
        destroy_buffer(allocator, &self.pos_buffer);
        destroy_buffer(allocator, &self.ind_buffer);
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

///
/// Destroys the given buffer/allocation pair
///
unsafe fn destroy_buffer(allocator: &Allocator, buffer: &(Buffer, Allocation)) {
    allocator.destroy_buffer(buffer.0, buffer.1);
}
