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

use std::any::TypeId;
use std::sync::Arc;

use aleph_any::AnyArc;
use aleph_object_system::ArcedObject;
use aleph_rhi_api::*;
use blink_alloc::Blink;

use crate::command_list::CommandList;
use crate::context::Context;
use crate::device::Device;
use crate::pipeline::{ComputePipeline, GraphicsPipeline};

pub struct Encoder<'a> {
    pub(crate) _parent: &'a mut CommandList,
    pub(crate) _context: AnyArc<Context>,
    pub(crate) _device: AnyArc<Device>,
    pub(crate) bound_graphics_pipeline: Option<Arc<ArcedObject<GraphicsPipeline>>>,
    pub(crate) bound_compute_pipeline: Option<Arc<ArcedObject<ComputePipeline>>>,
    pub(crate) arena: Blink,
}

impl<'a> IGetPlatformInterface for Encoder<'a> {
    unsafe fn __query_platform_interface(&self, target: TypeId, out: *mut ()) -> Option<()> {
        todo!()
    }
}

impl<'a> IGeneralEncoder for Encoder<'a> {
    unsafe fn bind_graphics_pipeline(&mut self, pipeline: &GraphicsPipelineHandle) {
        todo!()
    }

    unsafe fn bind_vertex_buffers(
        &mut self,
        first_binding: u32,
        bindings: &[InputAssemblyBufferBinding],
    ) {
        todo!()
    }

    unsafe fn bind_index_buffer(
        &mut self,
        index_type: IndexType,
        binding: &InputAssemblyBufferBinding,
    ) {
        todo!()
    }

    unsafe fn set_viewports(&mut self, viewports: &[Viewport]) {
        todo!()
    }

    unsafe fn set_scissor_rects(&mut self, rects: &[Rect]) {
        todo!()
    }

    unsafe fn set_push_constant_block(&mut self, block_index: usize, data: &[u8]) {
        todo!()
    }

    unsafe fn begin_rendering(&mut self, info: &BeginRenderingInfo) {
        todo!()
    }

    unsafe fn end_rendering(&mut self) {
        todo!()
    }

    unsafe fn draw(
        &mut self,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    ) {
        todo!()
    }

    unsafe fn draw_indexed(
        &mut self,
        index_count: u32,
        instance_count: u32,
        first_index: u32,
        first_instance: u32,
        vertex_offset: i32,
    ) {
        todo!()
    }
}

impl<'a> IComputeEncoder for Encoder<'a> {
    unsafe fn bind_compute_pipeline(&mut self, pipeline: &ComputePipelineHandle) {
        todo!()
    }

    unsafe fn bind_descriptor_sets(
        &mut self,
        pipeline_layout: &PipelineLayoutHandle,
        bind_point: PipelineBindPoint,
        first_set: u32,
        sets: &[DescriptorSetHandle],
        dynamic_offsets: &[u32],
    ) {
        todo!()
    }

    unsafe fn dispatch(&mut self, group_count_x: u32, group_count_y: u32, group_count_z: u32) {
        todo!()
    }
}

impl<'a> ITransferEncoder for Encoder<'a> {
    unsafe fn resource_barrier(
        &mut self,
        global_barriers: &[GlobalBarrier],
        buffer_barriers: &[BufferBarrier],
        texture_barriers: &[TextureBarrier],
    ) {
        todo!()
    }

    unsafe fn copy_buffer_regions(
        &mut self,
        src: &BufferHandle,
        dst: &BufferHandle,
        regions: &[BufferCopyRegion],
    ) {
        todo!()
    }

    unsafe fn copy_buffer_to_texture(
        &mut self,
        src: &BufferHandle,
        dst: &TextureHandle,
        regions: &[BufferToTextureCopyRegion],
    ) {
        todo!()
    }

    unsafe fn copy_texture_regions(
        &mut self,
        src: &TextureHandle,
        dst: &TextureHandle,
        regions: &[TextureToTextureCopyInfo],
    ) {
        todo!()
    }

    unsafe fn set_marker(&mut self, color: Color, message: &aleph_nstr::NStr) {
        todo!()
    }

    unsafe fn begin_event(&mut self, color: Color, message: &aleph_nstr::NStr) {
        todo!()
    }

    unsafe fn end_event(&mut self) {
        todo!()
    }

    unsafe fn close(&mut self) -> Result<(), CommandListCloseError> {
        todo!()
    }
}
