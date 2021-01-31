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

use aleph_vulkan_alloc::{Allocation, AllocationCreateInfoBuilder, Allocator, MemoryUsage};
use aleph_vulkan_core::erupt::vk1_0::{
    AccessFlags, Buffer, BufferCopyBuilder, BufferCreateInfoBuilder, BufferMemoryBarrierBuilder,
    BufferUsageFlags, CommandBuffer, PipelineStageFlags, SharingMode, WHOLE_SIZE,
};
use aleph_vulkan_core::{DebugName, Device};
use std::ffi::CString;
use std::mem::size_of;

///
/// Internal struct for holding the buffers for the cube mesh
///
pub struct StaticMeshBuffers {
    pos_buffer: (Buffer, Allocation),
    pos_size: u64,
    nrm_buffer: (Buffer, Allocation),
    nrm_size: u64,
    tan_buffer: (Buffer, Allocation),
    tan_size: u64,
    uv_buffer: (Buffer, Allocation),
    uv_size: u64,
    ind_buffer: (Buffer, Allocation),
    ind_size: u64,
    staging_buffer: (Buffer, Allocation),
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
        debug_name: &str,
        pos: &[Pos],
        nrm: &[Nrm],
        tan: &[Tan],
        uv: &[Uv],
        ind: &[Ind],
    ) -> Self {
        // Allocate the vertex buffer for the vertex positions
        let pos_buffer = init_buffer(allocator, BufferUsageFlags::VERTEX_BUFFER, pos);
        let pos_size = buffer_size(pos);

        // Allocate the vertex buffer for the vertex normals
        let nrm_buffer = init_buffer(allocator, BufferUsageFlags::VERTEX_BUFFER, nrm);
        let nrm_size = buffer_size(nrm);

        // Allocate the vertex buffer for the vertex tangents
        let tan_buffer = init_buffer(allocator, BufferUsageFlags::VERTEX_BUFFER, tan);
        let tan_size = buffer_size(tan);

        // Allocate the vertex buffer for the vertex texture coordinates
        let uv_buffer = init_buffer(allocator, BufferUsageFlags::VERTEX_BUFFER, uv);
        let uv_size = buffer_size(uv);

        // Allocate the index buffer
        let ind_buffer = init_buffer(allocator, BufferUsageFlags::INDEX_BUFFER, ind);
        let ind_size = buffer_size(ind);

        let staging_size = pos_size + nrm_size + tan_size + uv_size + ind_size;
        let staging_buffer = init_staging_buffer(allocator, staging_size);

        // Name the buffers
        unsafe {
            let name = format!("{}::Positions", debug_name);
            let name = CString::new(name).unwrap();
            pos_buffer.0.add_debug_name(allocator.device(), &name);

            let name = format!("{}::Normals", debug_name);
            let name = CString::new(name).unwrap();
            nrm_buffer.0.add_debug_name(allocator.device(), &name);

            let name = format!("{}::Tangents", debug_name);
            let name = CString::new(name).unwrap();
            tan_buffer.0.add_debug_name(allocator.device(), &name);

            let name = format!("{}::TexCoords", debug_name);
            let name = CString::new(name).unwrap();
            uv_buffer.0.add_debug_name(allocator.device(), &name);

            let name = format!("{}::Indices", debug_name);
            let name = CString::new(name).unwrap();
            ind_buffer.0.add_debug_name(allocator.device(), &name);

            let name = format!("{}::Staging", debug_name);
            let name = CString::new(name).unwrap();
            staging_buffer.0.add_debug_name(allocator.device(), &name);
        }

        // Map the staging memory
        let dest = map_staging_mem(allocator, &staging_buffer.1);
        let mut dest_offset = 0;

        copy_to_buffer(dest, dest_offset, pos);
        dest_offset += pos_size;

        copy_to_buffer(dest, dest_offset, nrm);
        dest_offset += nrm_size;

        copy_to_buffer(dest, dest_offset, tan);
        dest_offset += tan_size;

        copy_to_buffer(dest, dest_offset, uv);
        dest_offset += uv_size;

        copy_to_buffer(dest, dest_offset, ind);

        // Unmap the staging memory
        unmap_staging_mem(allocator, &staging_buffer.1);

        // Defer the destruction of these buffers until the allocator is being destroyed as we want
        // these to be valid for the entire runtime of the graphics system
        allocator.defer_destruction(pos_buffer);
        allocator.defer_destruction(nrm_buffer);
        allocator.defer_destruction(tan_buffer);
        allocator.defer_destruction(uv_buffer);
        allocator.defer_destruction(ind_buffer);
        allocator.defer_destruction(staging_buffer);

        Self {
            pos_buffer,
            pos_size,
            nrm_buffer,
            nrm_size,
            tan_buffer,
            tan_size,
            uv_buffer,
            uv_size,
            ind_buffer,
            ind_size,
            staging_buffer,
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
            let mut src_offset = 0;

            let region = BufferCopyBuilder::new()
                .size(self.pos_size)
                .dst_offset(0)
                .src_offset(src_offset);
            device.loader().cmd_copy_buffer(
                command_buffer,
                self.staging_buffer.0,
                self.pos_buffer.0,
                &[region],
            );
            src_offset += self.pos_size;

            let region = BufferCopyBuilder::new()
                .size(self.nrm_size)
                .dst_offset(0)
                .src_offset(src_offset);
            device.loader().cmd_copy_buffer(
                command_buffer,
                self.staging_buffer.0,
                self.nrm_buffer.0,
                &[region],
            );
            src_offset += self.nrm_size;

            let region = BufferCopyBuilder::new()
                .size(self.tan_size)
                .dst_offset(0)
                .src_offset(src_offset);
            device.loader().cmd_copy_buffer(
                command_buffer,
                self.staging_buffer.0,
                self.tan_buffer.0,
                &[region],
            );
            src_offset += self.tan_size;

            let region = BufferCopyBuilder::new()
                .size(self.uv_size)
                .dst_offset(0)
                .src_offset(src_offset);
            device.loader().cmd_copy_buffer(
                command_buffer,
                self.staging_buffer.0,
                self.uv_buffer.0,
                &[region],
            );
            src_offset += self.uv_size;

            let region = BufferCopyBuilder::new()
                .size(self.ind_size)
                .dst_offset(0)
                .src_offset(src_offset);
            device.loader().cmd_copy_buffer(
                command_buffer,
                self.staging_buffer.0,
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
                PipelineStageFlags::ALL_COMMANDS,
                PipelineStageFlags::ALL_COMMANDS,
                None,
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
    pos_size: u64,
    ind_buffer: (Buffer, Allocation),
    ind_size: u64,
    staging_buffer: (Buffer, Allocation),
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
    pub fn new<Pos, Ind>(
        allocator: &Allocator,
        debug_name: &str,
        pos: &[Pos],
        ind: &[Ind],
    ) -> Self {
        // Allocate the staging and vertex buffer for the vertex positions
        let pos_buffer = init_buffer(allocator, BufferUsageFlags::VERTEX_BUFFER, pos);
        let pos_size = buffer_size(pos);

        // Allocate the staging and index buffer
        let ind_buffer = init_buffer(allocator, BufferUsageFlags::INDEX_BUFFER, ind);
        let ind_size = buffer_size(ind);

        let staging_size = pos_size + ind_size;
        let staging_buffer = init_staging_buffer(allocator, staging_size);

        // Name the buffers
        unsafe {
            let name = format!("{}::Positions", debug_name);
            let name = CString::new(name).unwrap();
            pos_buffer.0.add_debug_name(allocator.device(), &name);

            let name = format!("{}::Indices", debug_name);
            let name = CString::new(name).unwrap();
            ind_buffer.0.add_debug_name(allocator.device(), &name);

            let name = format!("{}::Staging", debug_name);
            let name = CString::new(name).unwrap();
            staging_buffer.0.add_debug_name(allocator.device(), &name);
        }

        // Map the staging memory
        let dest = map_staging_mem(allocator, &staging_buffer.1);

        copy_to_buffer(dest, 0, pos);
        copy_to_buffer(dest, pos_size, ind);

        // Unmap the staging memory
        unmap_staging_mem(allocator, &staging_buffer.1);

        // Defer destruction of these buffers until the allocator is being destroyed as we want
        // these buffers to live for the entire lifetime of the graphics subsystem
        allocator.defer_destruction(pos_buffer);
        allocator.defer_destruction(ind_buffer);
        allocator.defer_destruction(staging_buffer);

        Self {
            pos_buffer,
            pos_size,
            ind_buffer,
            ind_size,
            staging_buffer,
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
                self.staging_buffer.0,
                self.pos_buffer.0,
                &[region],
            );

            let region = BufferCopyBuilder::new()
                .size(self.ind_size)
                .dst_offset(0)
                .src_offset(self.pos_size);
            device.loader().cmd_copy_buffer(
                command_buffer,
                self.staging_buffer.0,
                self.ind_buffer.0,
                &[region],
            );

            let pos_barrier = buffer_barrier(device, self.pos_buffer.0);
            let ind_barrier = buffer_barrier(device, self.ind_buffer.0);
            device.loader().cmd_pipeline_barrier(
                command_buffer,
                PipelineStageFlags::ALL_COMMANDS,
                PipelineStageFlags::ALL_COMMANDS,
                None,
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
/// Internal function that maps the staging buffer memory
///
fn map_staging_mem(allocator: &Allocator, allocation: &Allocation) -> *mut u8 {
    unsafe {
        allocator
            .map_memory(allocation)
            .expect("Failed to map memory")
    }
}

///
/// Internal function that unmaps the staging buffer memory
///
fn unmap_staging_mem(allocator: &Allocator, allocation: &Allocation) {
    unsafe {
        allocator.unmap_memory(allocation);
    }
}

///
/// Performs the copy from the source buffer memory into the staging buffer
///
fn copy_to_buffer<T>(dest: *mut u8, dest_offset: u64, src: &[T]) {
    unsafe {
        // Get the buffer as bytes, and the number of bytes to write
        let src_ptr = src.as_ptr() as *const u8;
        let count = buffer_size(src) as usize;

        // Copy the buffer data
        dest.add(dest_offset as usize).copy_from(src_ptr, count);
    }
}

///
/// Creates a staging buffer
///
fn init_staging_buffer(allocator: &Allocator, size: u64) -> (Buffer, Allocation) {
    let alloc_create_info = AllocationCreateInfoBuilder::new().usage(MemoryUsage::CPUOnly);

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
