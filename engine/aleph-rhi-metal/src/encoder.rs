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
use std::ptr::NonNull;
use std::sync::Arc;

use aleph_any::AnyArc;
use aleph_object_system::ArcedObject;
use aleph_rhi_api::*;
use allocator_api2::vec::Vec as BVec;
use blink_alloc::Blink;
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_foundation::NSRange;
use objc2_metal::*;

use crate::buffer::Buffer;
use crate::command_list::{CommandList, ListState};
use crate::context::Context;
use crate::device::Device;
use crate::internal::conv;
use crate::internal::image_view::ImageViewObject;
use crate::pipeline::{ComputePipeline, GraphicsPipeline};
use crate::texture::Texture;

pub struct Encoder<'a> {
    pub(crate) _parent: &'a mut CommandList,
    pub(crate) _context: AnyArc<Context>,
    pub(crate) _device: AnyArc<Device>,
    pub(crate) objects: EncoderObjects,
    pub(crate) active: ActiveEncoder,
    pub(crate) bound_graphics_pipeline: Option<Arc<ArcedObject<GraphicsPipeline>>>,
    pub(crate) bound_compute_pipeline: Option<Arc<ArcedObject<ComputePipeline>>>,
    pub(crate) bound_index_buffer: Option<BoundIndexBuffer>,
    pub(crate) bound_pipeline_state: BoundPipelineState,
    pub(crate) arena: Blink,
}

impl<'a> IGetPlatformInterface for Encoder<'a> {
    unsafe fn __query_platform_interface(&self, _target: TypeId, _out: *mut ()) -> Option<()> {
        todo!()
    }
}

impl<'a> IGeneralEncoder for Encoder<'a> {
    unsafe fn bind_graphics_pipeline(&mut self, pipeline: &GraphicsPipelineHandle) {
        let encoder = self.active.get_render();

        let concrete = GraphicsPipeline::get_owned(pipeline);

        encoder.setRenderPipelineState(&concrete.objects.pipeline);
        encoder.setDepthStencilState(Some(&concrete.objects.depth_stencil_state));
        self.bound_pipeline_state.primitive_type = concrete.info.primitive_type;

        encoder.setCullMode(concrete.info.cull_mode);
        encoder.setFrontFacingWinding(concrete.info.front_face);
        encoder.setTriangleFillMode(concrete.info.polygon_mode);
        if concrete.info.depth_bias != 0 {
            encoder.setDepthBias_slopeScale_clamp(
                concrete.info.depth_bias as f32,
                concrete.info.depth_bias_slope_factor,
                concrete.info.depth_bias_clamp,
            );
        }

        self.bound_graphics_pipeline = Some(concrete);
    }

    unsafe fn bind_vertex_buffers(
        &mut self,
        first_binding: u32,
        bindings: &[InputAssemblyBufferBinding],
    ) {
        if bindings.is_empty() {
            return; // Bail if no bindings are provided
        }

        let encoder = self.active.get_render();

        unsafe {
            let mut mtl_buffers = BVec::with_capacity_in(bindings.len(), self.arena.allocator());
            mtl_buffers.extend(bindings.iter().map(|v| {
                let v = Buffer::get(v.buffer);
                v.objects.buffer.as_ref() as *const ProtocolObject<dyn MTLBuffer>
            }));

            let mut mtl_offsets = BVec::with_capacity_in(bindings.len(), self.arena.allocator());
            mtl_offsets.extend(bindings.iter().map(|v| v.offset as usize));

            let range = NSRange {
                location: first_binding as usize,
                length: bindings.len(),
            };

            let p_mtl_buffers = NonNull::new_unchecked(mtl_buffers.as_mut_ptr());
            let p_mtl_offsets = NonNull::new_unchecked(mtl_offsets.as_mut_ptr());
            encoder.setVertexBuffers_offsets_withRange(p_mtl_buffers, p_mtl_offsets, range);
        }
        self.arena.reset();
    }

    unsafe fn bind_index_buffer(
        &mut self,
        index_type: IndexType,
        binding: &InputAssemblyBufferBinding,
    ) {
        let buffer = Buffer::get(binding.buffer);
        let binding = BoundIndexBuffer {
            buffer: buffer.objects.buffer.clone(),
            offset: binding.offset,
            index_type: conv::index_type_to_mtl(index_type),
            index_size: conv::index_type_to_size(index_type),
        };
        self.bound_index_buffer = Some(binding);
    }

    unsafe fn set_viewports(&mut self, viewports: &[Viewport]) {
        if viewports.is_empty() {
            return; // If we don't provide any viewports we just bail
        }

        let encoder = self.active.get_render();

        if viewports.len() == 1 {
            let viewport = conv::viewport_to_mtl(&viewports[0]);
            encoder.setViewport(viewport);
        } else {
            unsafe {
                let mut mtl_viewports =
                    BVec::with_capacity_in(viewports.len(), self.arena.allocator());
                mtl_viewports.extend(viewports.iter().map(conv::viewport_to_mtl));

                encoder.setViewports(&mtl_viewports);
            }
            self.arena.reset();
        }
    }

    unsafe fn set_scissor_rects(&mut self, rects: &[Rect]) {
        if rects.is_empty() {
            return; // If we don't provide any scissor rects we just bail
        }

        let encoder = self.active.get_render();

        if rects.len() == 1 {
            let rect = conv::rect_to_mtl_scissor_rect(&rects[0]);
            encoder.setScissorRect(rect);
        } else {
            unsafe {
                let mut mtl_rects = BVec::with_capacity_in(rects.len(), self.arena.allocator());
                mtl_rects.extend(rects.iter().map(conv::rect_to_mtl_scissor_rect));

                let ptr = NonNull::new_unchecked(mtl_rects.as_mut_ptr());
                encoder.setScissorRects_count(ptr, mtl_rects.len());
            }
            self.arena.reset();
        }
    }

    unsafe fn set_push_constant_block(&mut self, block_index: usize, data: &[u8]) {
        let encoder = self.active.get_render();
    }

    unsafe fn begin_rendering(&mut self, info: &BeginRenderingInfo) {
        let mtl_desc = unsafe { MTLRenderPassDescriptor::new() };

        let mtl_color_attachments = mtl_desc.colorAttachments();
        for (i, color_attachment) in info.color_attachments.iter().enumerate() {
            let mtl_attachment = unsafe { mtl_color_attachments.objectAtIndexedSubscript(i) };

            let view = color_attachment.image_view;
            let view = unsafe { ImageViewObject::from_view(&view) };
            let texture = view.texture.as_ref();
            mtl_attachment.setTexture(Some(texture));
            mtl_attachment.setLevel(0);
            mtl_attachment.setSlice(0);

            match &color_attachment.load_op {
                AttachmentLoadOp::Load => {
                    mtl_attachment.setLoadAction(MTLLoadAction::Load);
                }
                AttachmentLoadOp::Clear(clear_color) => {
                    mtl_attachment.setLoadAction(MTLLoadAction::Clear);

                    let [r, g, b, a] = clear_color.to_float();
                    let clear_color = MTLClearColor {
                        red: r as f64,
                        green: g as f64,
                        blue: b as f64,
                        alpha: a as f64,
                    };
                    mtl_attachment.setClearColor(clear_color);
                }
                AttachmentLoadOp::DontCare => {
                    mtl_attachment.setLoadAction(MTLLoadAction::DontCare);
                }
                AttachmentLoadOp::None => {
                    // TODO: this doesn't seem right
                    mtl_attachment.setLoadAction(MTLLoadAction::DontCare);
                }
            }

            let store_op = conv::attachment_store_op_to_mtl(color_attachment.store_op);
            mtl_attachment.setStoreAction(store_op);
        }

        if let Some(depth_attachment) = info.depth_stencil_attachment {
            let mtl_attachment = unsafe { MTLRenderPassDepthAttachmentDescriptor::new() };

            let view = depth_attachment.image_view;
            let view = unsafe { ImageViewObject::from_view(&view) };
            let texture = view.texture.as_ref();
            mtl_attachment.setTexture(Some(texture));
            mtl_attachment.setLevel(0);
            mtl_attachment.setSlice(0);

            match &depth_attachment.depth_load_op {
                AttachmentLoadOp::Load => {
                    mtl_attachment.setLoadAction(MTLLoadAction::Load);
                }
                AttachmentLoadOp::Clear(v) => {
                    mtl_attachment.setLoadAction(MTLLoadAction::Clear);
                    mtl_attachment.setClearDepth(v.depth as f64);
                }
                AttachmentLoadOp::DontCare => {
                    mtl_attachment.setLoadAction(MTLLoadAction::DontCare);
                }
                AttachmentLoadOp::None => {
                    // TODO: this doesn't seem right
                    mtl_attachment.setLoadAction(MTLLoadAction::DontCare);
                }
            }

            let store_op = conv::attachment_store_op_to_mtl(depth_attachment.depth_store_op);
            mtl_attachment.setStoreAction(store_op);
            mtl_desc.setDepthAttachment(Some(&mtl_attachment));

            // TODO: if we need a stencil buffer
            if true {
                let mtl_attachment = unsafe { MTLRenderPassStencilAttachmentDescriptor::new() };

                // We use the same attachment here intentionally
                let view = depth_attachment.image_view;
                let view = unsafe { ImageViewObject::from_view(&view) };
                let texture = view.texture.as_ref();
                mtl_attachment.setTexture(Some(texture));
                mtl_attachment.setLevel(0);
                mtl_attachment.setSlice(0);

                match &depth_attachment.stencil_load_op {
                    AttachmentLoadOp::Load => {
                        mtl_attachment.setLoadAction(MTLLoadAction::Load);
                    }
                    AttachmentLoadOp::Clear(v) => {
                        mtl_attachment.setLoadAction(MTLLoadAction::Clear);
                        mtl_attachment.setClearStencil(v.stencil as u32);
                    }
                    AttachmentLoadOp::DontCare => {
                        mtl_attachment.setLoadAction(MTLLoadAction::DontCare);
                    }
                    AttachmentLoadOp::None => {
                        // TODO: this doesn't seem right
                        mtl_attachment.setLoadAction(MTLLoadAction::DontCare);
                    }
                }

                let store_op = conv::attachment_store_op_to_mtl(depth_attachment.stencil_store_op);
                mtl_attachment.setStoreAction(store_op);
                mtl_desc.setStencilAttachment(Some(&mtl_attachment));
            }
        }

        mtl_desc.setRenderTargetWidth(info.extent.width as usize);
        mtl_desc.setRenderTargetHeight(info.extent.height as usize);

        let encoder = self
            .objects
            .list
            .renderCommandEncoderWithDescriptor(&mtl_desc);
        match encoder {
            Some(v) => {
                self.active.set_render(v);
            }
            None => {
                log::error!("Failed to create 'MTLCommandRenderEncoder'!");
                panic!("Failed to create 'MTLCommandRenderEncoder'!");
            }
        }
    }

    unsafe fn end_rendering(&mut self) {
        self.active.end_render();
    }

    unsafe fn draw(
        &mut self,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    ) {
        let encoder = self.active.get_render();

        let primitive_type = self.bound_pipeline_state.primitive_type;
        unsafe {
            encoder.drawPrimitives_vertexStart_vertexCount_instanceCount_baseInstance(
                primitive_type,
                first_vertex as usize,
                vertex_count as usize,
                instance_count as usize,
                first_instance as usize,
            );
        }
    }

    unsafe fn draw_indexed(
        &mut self,
        index_count: u32,
        instance_count: u32,
        first_index: u32,
        first_instance: u32,
        vertex_offset: i32,
    ) {
        let encoder = self.active.get_render();

        let primitive_type = self.bound_pipeline_state.primitive_type;
        let index_buffer = self.bound_index_buffer.as_ref().unwrap();
        let offset = index_buffer.offset;
        let offset = offset + (first_index as u64 * index_buffer.index_size as u64);
        unsafe {
            encoder.drawIndexedPrimitives_indexCount_indexType_indexBuffer_indexBufferOffset_instanceCount_baseVertex_baseInstance(
                primitive_type,
                index_count as usize,
                index_buffer.index_type,
                &index_buffer.buffer,
                offset as usize,
                instance_count as usize,
                vertex_offset as isize,
                first_instance as usize,
            );
        }
    }
}

impl<'a> IComputeEncoder for Encoder<'a> {
    unsafe fn bind_compute_pipeline(&mut self, pipeline: &ComputePipelineHandle) {
        self.active.test_begin_compute(&self.objects.list);
        let encoder = self.active.get_compute();

        let concrete = ComputePipeline::get_owned(pipeline);

        encoder.setComputePipelineState(&concrete.objects.pipeline);

        self.bound_compute_pipeline = Some(concrete);
    }

    unsafe fn bind_descriptor_sets(
        &mut self,
        pipeline_layout: &PipelineLayoutHandle,
        bind_point: PipelineBindPoint,
        first_set: u32,
        sets: &[DescriptorSetHandle],
        dynamic_offsets: &[u32],
    ) {
        match bind_point {
            PipelineBindPoint::Compute => {
                let _encoder = self.active.get_compute();
                todo!()
            }
            PipelineBindPoint::Graphics => {
                let _encoder = self.active.get_render();
                todo!()
            }
        }
    }

    unsafe fn dispatch(&mut self, group_count_x: u32, group_count_y: u32, group_count_z: u32) {
        self.active.test_begin_compute(&self.objects.list);
        let encoder = self.active.get_compute();

        let threadgroup_count = MTLSize {
            width: group_count_x as usize,
            height: group_count_y as usize,
            depth: group_count_z as usize,
        };
        let threads_per_threadgroup = MTLSize {
            width: 0,
            height: 0,
            depth: 0,
        }; // TODO: this

        encoder
            .dispatchThreadgroups_threadsPerThreadgroup(threadgroup_count, threads_per_threadgroup);
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
        let src = Buffer::get(src);
        let dst = Buffer::get(dst);

        self.active.test_begin_blit(&self.objects.list);
        let encoder = self.active.get_blit();

        for region in regions {
            unsafe {
                encoder.copyFromBuffer_sourceOffset_toBuffer_destinationOffset_size(
                    &src.objects.buffer,
                    region.src_offset as usize,
                    &dst.objects.buffer,
                    region.dst_offset as usize,
                    region.size as usize,
                );
            }
        }
    }

    unsafe fn copy_buffer_to_texture(
        &mut self,
        src: &BufferHandle,
        dst: &TextureHandle,
        regions: &[BufferToTextureCopyRegion],
    ) {
        let src = Buffer::get(src);
        let dst = Texture::get(dst);

        self.active.test_begin_blit(&self.objects.list);
        let encoder = self.active.get_blit();

        for region in regions {
            unsafe {
                let bytes_per_element = dst.desc().format.bytes_per_element() as usize;
                let source_bytes_per_row = region.src.row_pitch as usize * bytes_per_element;
                let source_bytes_per_image = match dst.desc.get().dimension {
                    TextureDimension::Texture1D | TextureDimension::Texture2D => 0,
                    TextureDimension::Texture3D => {
                        // Only 3D textures should have this != 0.
                        source_bytes_per_row * region.dst.extent.depth as usize
                    }
                };
                let destination_origin = conv::u_offset_to_mtl_origin(&region.dst.origin);
                let source_size = conv::extent_to_mtl_size(&region.dst.extent);
                encoder.copyFromBuffer_sourceOffset_sourceBytesPerRow_sourceBytesPerImage_sourceSize_toTexture_destinationSlice_destinationLevel_destinationOrigin_options(
                    &src.objects.buffer,
                    region.src.offset as usize,
                    source_bytes_per_row,
                    source_bytes_per_image,
                    source_size,
                    &dst.objects.texture,
                    region.dst.array_layer as usize,
                    region.dst.mip_level as usize,
                    destination_origin,
                    MTLBlitOption::None
                );
            }
        }
    }

    unsafe fn copy_texture_regions(
        &mut self,
        src: &TextureHandle,
        dst: &TextureHandle,
        regions: &[TextureToTextureCopyInfo],
    ) {
        let src = Texture::get(src);
        let dst = Texture::get(dst);

        self.active.test_begin_blit(&self.objects.list);
        let encoder = self.active.get_blit();

        for region in regions {
            unsafe {
                let source_origin = conv::u_offset_to_mtl_origin(&region.src.offset);
                let destination_origin = conv::u_offset_to_mtl_origin(&region.dst.offset);
                let source_size = conv::extent_to_mtl_size(&region.extent);
                encoder.copyFromTexture_sourceSlice_sourceLevel_sourceOrigin_sourceSize_toTexture_destinationSlice_destinationLevel_destinationOrigin(
                    &src.objects.texture,
                    region.src.array_layer as usize,
                    region.src.mip_level as usize,
                    source_origin,
                    source_size,
                    &dst.objects.texture,
                    region.dst.array_layer as usize,
                    region.dst.mip_level as usize,
                    destination_origin,
                );
            }
        }
    }

    unsafe fn set_marker(&mut self, _color: Color, _message: &aleph_nstr::NStr) {
        // TODO: this
    }

    unsafe fn begin_event(&mut self, _color: Color, _message: &aleph_nstr::NStr) {
        // TODO: this
    }

    unsafe fn end_event(&mut self) {
        // TODO: this
    }

    unsafe fn close(&mut self) -> Result<(), CommandListCloseError> {
        match self._parent.state {
            ListState::Empty => Err(CommandListCloseError::AlreadyClosed),
            ListState::Open => {
                self.active.end_all();
                self._parent.state = ListState::Closed;
                Ok(())
            }
            ListState::Closed => Err(CommandListCloseError::AlreadyClosed),
        }
    }
}

/// Wrapper to limit the scope of our 'unsafe impl Send'
pub struct EncoderObjects {
    pub list: Retained<ProtocolObject<dyn MTLCommandBuffer>>,
}

unsafe impl Send for EncoderObjects {}

pub enum ActiveEncoder {
    Graphics(Retained<ProtocolObject<dyn MTLRenderCommandEncoder>>),
    Compute(Retained<ProtocolObject<dyn MTLComputeCommandEncoder>>),
    Copy(Retained<ProtocolObject<dyn MTLBlitCommandEncoder>>),
    None,
}

impl ActiveEncoder {
    pub fn set_render(&mut self, encoder: Retained<ProtocolObject<dyn MTLRenderCommandEncoder>>) {
        match self {
            ActiveEncoder::Graphics(_) => {
                log::error!("Must end previous render encoder with 'end_rendering'!");
                panic!("Must end previous render encoder with 'end_rendering'!")
            }
            ActiveEncoder::Compute(old) => {
                old.endEncoding();
                *self = ActiveEncoder::Graphics(encoder);
            }
            ActiveEncoder::Copy(old) => {
                old.endEncoding();
                *self = ActiveEncoder::Graphics(encoder);
            }
            ActiveEncoder::None => {
                *self = ActiveEncoder::Graphics(encoder);
            }
        }
    }

    pub fn end_render(&mut self) {
        match self {
            ActiveEncoder::Graphics(old) => {
                old.endEncoding();
            }
            _ => {
                log::error!("No render encoder is active to 'endEncoding'!");
                panic!("No render encoder is active to 'endEncoding'!");
            }
        }
        *self = ActiveEncoder::None;
    }

    pub fn get_render(&self) -> &ProtocolObject<dyn MTLRenderCommandEncoder> {
        match self {
            ActiveEncoder::Graphics(v) => v.as_ref(),
            ActiveEncoder::Compute(_) => panic!(),
            ActiveEncoder::Copy(_) => panic!(),
            ActiveEncoder::None => {
                log::error!("Must begin render encoder with 'begin_rendering'!");
                panic!("Must begin render encoder with 'begin_rendering'!")
            }
        }
    }

    pub fn test_begin_compute<'a>(&'a mut self, list: &ProtocolObject<dyn MTLCommandBuffer>) {
        match self {
            ActiveEncoder::Graphics(_) => {
                log::error!("Must end render encoders with 'end_rendering'!");
                panic!("Must end render encoders with 'end_rendering'!")
            }
            ActiveEncoder::Compute(_) => {}
            ActiveEncoder::Copy(old) => {
                old.endEncoding();
                let encoder = list
                    .computeCommandEncoderWithDispatchType(MTLDispatchType::Concurrent)
                    .unwrap();
                *self = ActiveEncoder::Compute(encoder);
            }
            ActiveEncoder::None => {
                let encoder = list
                    .computeCommandEncoderWithDispatchType(MTLDispatchType::Concurrent)
                    .unwrap();
                *self = ActiveEncoder::Compute(encoder);
            }
        };
    }

    pub fn get_compute<'a>(&'a self) -> &'a ProtocolObject<dyn MTLComputeCommandEncoder> {
        match self {
            ActiveEncoder::Graphics(_) => panic!(),
            ActiveEncoder::Compute(v) => v.as_ref(),
            ActiveEncoder::Copy(_) => panic!(),
            ActiveEncoder::None => panic!(),
        }
    }

    pub fn test_begin_blit<'a>(&'a mut self, list: &ProtocolObject<dyn MTLCommandBuffer>) {
        match self {
            ActiveEncoder::Graphics(_) => {
                log::error!("Must end render encoders with 'end_rendering'!");
                panic!("Must end render encoders with 'end_rendering'!")
            }
            ActiveEncoder::Compute(old) => {
                old.endEncoding();
                let encoder = list.blitCommandEncoder().unwrap();
                *self = ActiveEncoder::Copy(encoder);
            }
            ActiveEncoder::Copy(_) => {}
            ActiveEncoder::None => {
                let encoder = list.blitCommandEncoder().unwrap();
                *self = ActiveEncoder::Copy(encoder);
            }
        };
    }

    pub fn get_blit<'a>(&'a self) -> &'a ProtocolObject<dyn MTLBlitCommandEncoder> {
        match self {
            ActiveEncoder::Graphics(_) => panic!(),
            ActiveEncoder::Compute(_) => panic!(),
            ActiveEncoder::Copy(v) => v.as_ref(),
            ActiveEncoder::None => panic!(),
        }
    }

    pub fn end_all(&mut self) {
        match self {
            ActiveEncoder::Graphics(old) => {
                old.endEncoding();
            }
            ActiveEncoder::Compute(old) => {
                old.endEncoding();
            }
            ActiveEncoder::Copy(old) => {
                old.endEncoding();
            }
            ActiveEncoder::None => {
                log::debug!("Trying to end an already ended encoder!");
            }
        }
        *self = ActiveEncoder::None;
    }
}

unsafe impl Send for ActiveEncoder {}

pub struct BoundIndexBuffer {
    buffer: Retained<ProtocolObject<dyn MTLBuffer>>,
    offset: u64,
    index_type: MTLIndexType,
    index_size: usize,
}

unsafe impl Send for BoundIndexBuffer {}

pub struct BoundPipelineState {
    primitive_type: MTLPrimitiveType,
}

impl Default for BoundPipelineState {
    fn default() -> Self {
        Self {
            primitive_type: MTLPrimitiveType::Point,
        }
    }
}
