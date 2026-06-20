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
use std::hint::unreachable_unchecked;
use std::num::NonZero;
use std::ptr::NonNull;
use std::sync::Arc;

use aleph_alloc::instrumentation::system;
use aleph_object_system::Object;
use aleph_rhi_api::*;
use aleph_rhi_impl_utils::RhiSystem;
use aleph_rhi_impl_utils::parameter_block_layout_visitor::ParameterBlockLayoutVisitor;
use allocator_api2::vec::Vec as BVec;
use blink_alloc::Blink;
use objc2::rc::{Retained, autoreleasepool};
use objc2::runtime::ProtocolObject;
use objc2_foundation::NSString;
use objc2_metal::*;

use crate::binding_signature::BindingSignature;
use crate::buffer::Buffer;
use crate::command_list::{CommandList, ListState};
use crate::context::Context;
use crate::device::Device;
use crate::internal::image_view::ImageViewObject;
use crate::internal::parameter_block::ParameterBlock;
use crate::internal::upload_bump_allocator::UploadBumpAllocator;
use crate::internal::{conv, unwrap};
use crate::pipeline::{ComputePipeline, GraphicsPipeline};
use crate::sampler::Sampler;
use crate::texture::Texture;

const COMPUTE_STAGES: MTLStages = MTLStages::Dispatch
    .union(MTLStages::Blit)
    .union(MTLStages::AccelerationStructure);

pub struct Encoder<'a> {
    pub(crate) _parent: &'a mut CommandList,
    pub(crate) _context: Arc<Context>,
    pub(crate) _device: Arc<Device>,
    pub(crate) objects: EncoderObjects,
    pub(crate) active: ActiveEncoder,
    pub(crate) bound_graphics_pipeline: Option<Arc<Object<GraphicsPipeline>>>,
    pub(crate) bound_compute_pipeline: Option<Arc<Object<ComputePipeline>>>,
    pub(crate) bound_index_buffer: Option<BoundIndexBuffer>,
    pub(crate) bound_graphics_pipeline_state: BoundPipelineState,
    pub(crate) bound_compute_pipeline_state: BoundPipelineState,
    pub(crate) arena: Blink,
}

impl<'a> IGetPlatformInterface for Encoder<'a> {
    unsafe fn __query_platform_interface(&self, _target: TypeId, _out: *mut ()) -> Option<()> {
        None
    }
}

impl<'a> ICommandEncoderAbi for Encoder<'a> {
    unsafe fn __bind_graphics_pipeline(&mut self, pipeline: &GraphicsPipelineHandle) {
        let encoder = self.active.get_render();

        let concrete = GraphicsPipeline::get_owned(pipeline);

        encoder.setRenderPipelineState(&concrete.objects.pipeline);
        encoder.setDepthStencilState(Some(&concrete.objects.depth_stencil_state));
        self.bound_graphics_pipeline_state.primitive_type = concrete.info.primitive_type;

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

    unsafe fn __bind_vertex_buffers(
        &mut self,
        first_binding: u32,
        bindings: &[InputAssemblyBufferBinding],
    ) {
        if bindings.is_empty() {
            return; // Bail if no bindings are provided
        }

        unsafe {
            for (i, binding) in bindings.iter().enumerate() {
                let buffer = Buffer::get(binding.buffer);
                let addr = buffer.gpu_addr.get() + binding.offset;
                self._parent
                    .objects
                    .argument_table
                    .setAddress_atIndex(addr, 10 + i + first_binding as usize);
            }
        }
        self.arena.reset();
    }

    unsafe fn __bind_index_buffer(
        &mut self,
        index_type: IndexType,
        binding: &InputAssemblyBufferBinding,
    ) {
        let buffer = Buffer::get(binding.buffer);
        let addr = buffer.gpu_addr.saturating_add(binding.offset);
        let binding = BoundIndexBuffer {
            addr,
            index_type: conv::index_type_to_mtl(index_type),
            index_size: conv::index_type_to_size(index_type),
        };
        self.bound_index_buffer = Some(binding);
    }

    unsafe fn __set_viewports(&mut self, viewports: &[Viewport]) {
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

                let ptr = NonNull::new_unchecked(mtl_viewports.as_mut_ptr());
                encoder.setViewports_count(ptr, mtl_viewports.len());
            }
            self.arena.reset();
        }
    }

    unsafe fn __set_scissor_rects(&mut self, rects: &[Rect]) {
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

    unsafe fn __set_push_constant_block(&mut self, data: &[u8]) {
        // TODO: push constants currently not working for compute

        // This command can't work without a bound pipeline, we need the pipeline layout so we can
        // know where in the root signature to write the data
        let pipeline = self.bound_graphics_pipeline.as_deref().unwrap();

        let state = &mut self.bound_graphics_pipeline_state;

        // Lookup the parameter index on the currently bound pipeline (pipeline layout) based on
        // the constant block index
        let block = pipeline
            ._binding_signature
            .push_constant_block
            .as_ref()
            .unwrap();

        state.push_constant_block[..data.len()].copy_from_slice(data);
        let block_bytes = &state.push_constant_block[0..block.size.get() as usize];

        let (cpu_addr, gpu_addr) = self
            ._parent
            .push_constant_allocator
            .allocate(&self._device, block_bytes.len());

        unsafe {
            cpu_addr.copy_from(NonNull::from(block_bytes).cast::<u8>(), block_bytes.len());
        }

        unsafe {
            const PUSH_CONSTANT_INDEX: usize = 9;
            self._parent
                .objects
                .argument_table
                .setAddress_atIndex(gpu_addr.get(), PUSH_CONSTANT_INDEX);
        }
    }

    unsafe fn __begin_rendering(&mut self, info: &BeginRenderingInfo) {
        autoreleasepool(|_| {
            let mtl_desc = MTL4RenderPassDescriptor::new();

            let mtl_color_attachments = mtl_desc.colorAttachments();
            for (i, color_attachment) in info.color_attachments.iter().enumerate() {
                let mtl_attachment = unsafe { mtl_color_attachments.objectAtIndexedSubscript(i) };

                let view = color_attachment.image_view;
                let view = unsafe { view.into_raw::<ImageViewObject>().as_ref() };
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
                }

                let store_op = conv::attachment_store_op_to_mtl(color_attachment.store_op);
                mtl_attachment.setStoreAction(store_op);
            }

            if let Some(attachment) = info.depth_stencil_attachment {
                let view = attachment.image_view;
                let view = unsafe { view.into_raw::<ImageViewObject>().as_ref() };
                let texture = view.texture.as_ref();

                if let Some(ops) = &attachment.depth {
                    let mtl_attachment = MTLRenderPassDepthAttachmentDescriptor::new();
                    mtl_attachment.setTexture(Some(texture));
                    mtl_attachment.setLevel(0);
                    mtl_attachment.setSlice(0);

                    match &ops.load_op {
                        AttachmentLoadOp::Load => {
                            mtl_attachment.setLoadAction(MTLLoadAction::Load);
                        }
                        AttachmentLoadOp::Clear(v) => {
                            mtl_attachment.setLoadAction(MTLLoadAction::Clear);
                            mtl_attachment.setClearDepth(*v as f64);
                        }
                        AttachmentLoadOp::DontCare => {
                            mtl_attachment.setLoadAction(MTLLoadAction::DontCare);
                        }
                    }

                    let store_op = conv::attachment_store_op_to_mtl(ops.store_op);
                    mtl_attachment.setStoreAction(store_op);
                    mtl_desc.setDepthAttachment(Some(&mtl_attachment));
                }

                if let Some(ops) = &attachment.stencil {
                    let mtl_attachment = MTLRenderPassStencilAttachmentDescriptor::new();

                    // We use the same attachment here intentionally
                    mtl_attachment.setTexture(Some(texture));
                    mtl_attachment.setLevel(0);
                    mtl_attachment.setSlice(0);

                    match &ops.load_op {
                        AttachmentLoadOp::Load => {
                            mtl_attachment.setLoadAction(MTLLoadAction::Load);
                        }
                        AttachmentLoadOp::Clear(v) => {
                            mtl_attachment.setLoadAction(MTLLoadAction::Clear);
                            mtl_attachment.setClearStencil(*v as u32);
                        }
                        AttachmentLoadOp::DontCare => {
                            mtl_attachment.setLoadAction(MTLLoadAction::DontCare);
                        }
                    }

                    let store_op = conv::attachment_store_op_to_mtl(ops.store_op);
                    mtl_attachment.setStoreAction(store_op);
                    mtl_desc.setStencilAttachment(Some(&mtl_attachment));
                }
            }

            mtl_desc.setRenderTargetWidth(info.extent.width as usize);
            mtl_desc.setRenderTargetHeight(info.extent.height as usize);

            self.active.set_render(
                &self.objects.list,
                &self._parent.objects.argument_table,
                &mtl_desc,
            );
        })
    }

    unsafe fn __end_rendering(&mut self) {
        // autoreleasepool(|_| {
        //     self.active.end_render();
        // })
    }

    unsafe fn __draw(
        &mut self,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    ) {
        let encoder = self.active.get_render();

        let pipeline = self.bound_graphics_pipeline.as_deref().unwrap();

        unsafe {
            self.bound_graphics_pipeline_state.maybe_flush_params(
                &self._device,
                &mut self._parent.push_constant_allocator,
                &self._parent.objects.argument_table,
                &pipeline._binding_signature,
            );
        }

        let primitive_type = self.bound_graphics_pipeline_state.primitive_type;
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

    unsafe fn __draw_indexed(
        &mut self,
        index_count: u32,
        instance_count: u32,
        first_index: u32,
        first_instance: u32,
        vertex_offset: i32,
    ) {
        let encoder = self.active.get_render();

        let pipeline = self.bound_graphics_pipeline.as_deref().unwrap();

        unsafe {
            self.bound_graphics_pipeline_state.maybe_flush_params(
                &self._device,
                &mut self._parent.push_constant_allocator,
                &self._parent.objects.argument_table,
                &pipeline._binding_signature,
            );
        }

        let primitive_type = self.bound_graphics_pipeline_state.primitive_type;
        let index_buffer = self.bound_index_buffer.as_ref().unwrap();

        let draw_index_offset = first_index as u64 * index_buffer.index_size as u64;
        let addr = index_buffer.addr.get() + draw_index_offset;
        let len = index_count as usize * index_buffer.index_size;

        unsafe {
            encoder.drawIndexedPrimitives_indexCount_indexType_indexBuffer_indexBufferLength_instanceCount_baseVertex_baseInstance(
                primitive_type,
                index_count as usize,
                index_buffer.index_type,
                addr,
                len,
                instance_count as usize,
                vertex_offset as isize,
                first_instance as usize,
            );
        }
    }

    unsafe fn __bind_compute_pipeline(&mut self, pipeline: &ComputePipelineHandle) {
        let encoder = self
            .active
            .begin_compute(&self.objects.list, &self._parent.objects.argument_table);

        let concrete = ComputePipeline::get_owned(pipeline);

        encoder.setComputePipelineState(&concrete.objects.pipeline);

        self.bound_compute_pipeline = Some(concrete);
    }

    unsafe fn __bind_parameter_blocks(
        &mut self,
        binding_signature: &dyn IBindingSignature,
        bind_point: PipelineBindPoint,
        first_block: u32,
        blocks: &[ParameterBlockHandle],
    ) {
        let binding_signature = unwrap::binding_signature(binding_signature);
        match bind_point {
            PipelineBindPoint::Compute => {
                for (i, block) in blocks.iter().enumerate() {
                    let i = first_block as usize + i;

                    let block = unsafe { block.into_raw::<ParameterBlock>().as_mut() };

                    unsafe {
                        self._parent
                            .objects
                            .argument_table
                            .setAddress_atIndex(block.gpu_addr.unwrap_unchecked().get(), i);
                    }
                }
            }
            PipelineBindPoint::Graphics => {
                for (i, block) in blocks.iter().enumerate() {
                    let i = first_block as usize + i;

                    let block_layout = &binding_signature._parameter_block_layouts[i];
                    let block_layout_desc = block_layout.desc.get();

                    let block = unsafe { block.into_raw::<ParameterBlock>().as_mut() };

                    let visibility =
                        conv::descriptor_visibility_to_mtl(block_layout_desc.visibility);
                    if !visibility.is_empty() {
                        unsafe {
                            self._parent
                                .objects
                                .argument_table
                                .setAddress_atIndex(block.gpu_addr.unwrap_unchecked().get(), i);
                        }
                    }
                }
            }
        }
    }

    unsafe fn __push_parameters(
        &mut self,
        binding_signature: &dyn IBindingSignature,
        bind_point: PipelineBindPoint,
        block: u32,
        base: u32,
        writes: &[ParameterWrite],
    ) {
        let binding_signature = unwrap::binding_signature(binding_signature);
        let layout = &binding_signature._parameter_block_layouts[block as usize];

        let push_params = match bind_point {
            PipelineBindPoint::Compute => &mut self.bound_compute_pipeline_state.push_params,
            PipelineBindPoint::Graphics => &mut self.bound_graphics_pipeline_state.push_params,
        };
        let push_params = &mut push_params[block as usize];

        // Ensure the arrays are of the minimum required size
        push_params.resize(layout.compiled.num_arguments, 0);

        let visitor =
            ParameterBlockLayoutVisitor::new(layout.desc.get(), base as u64, writes).unwrap();
        for write_group in visitor {
            for (i, write) in write_group.writes.iter().enumerate() {
                let i = i + write_group.index as usize;
                match write {
                    ParameterWrite::Sampler(v) => {
                        let sampler = Sampler::get(v.sampler);
                        let id = sampler.objects.sampler.gpuResourceID();
                        let id = id.to_raw();
                        push_params[i] = id;
                    }
                    ParameterWrite::Buffer(v) => {
                        let src = Buffer::get(v.buffer);
                        let addr = src.gpu_addr.get() + v.offset;
                        push_params[i] = addr;
                    }
                    ParameterWrite::Texture(_) => unreachable!(),
                    ParameterWrite::TextureBuffer(_) => unreachable!(),
                }
            }
        }

        match bind_point {
            PipelineBindPoint::Compute => {
                self.bound_compute_pipeline_state.push_params_dirty = true
            }
            PipelineBindPoint::Graphics => {
                self.bound_graphics_pipeline_state.push_params_dirty = true
            }
        }
    }

    unsafe fn __dispatch(&mut self, group_count_x: u32, group_count_y: u32, group_count_z: u32) {
        let encoder = self
            .active
            .begin_compute(&self.objects.list, &self._parent.objects.argument_table);

        let pipeline = self.bound_compute_pipeline.as_deref().unwrap();

        unsafe {
            self.bound_graphics_pipeline_state.maybe_flush_params(
                &self._device,
                &mut self._parent.push_constant_allocator,
                &self._parent.objects.argument_table,
                &pipeline._binding_signature,
            );
        }

        encoder.dispatchThreadgroups_threadsPerThreadgroup(
            MTLSize {
                width: group_count_x as usize,
                height: group_count_y as usize,
                depth: group_count_z as usize,
            },
            pipeline.workgroup_size,
        );
    }

    unsafe fn __resource_barrier(
        &mut self,
        global_barriers: &[GlobalBarrier],
        buffer_barriers: &[BufferBarrier],
        texture_barriers: &[TextureBarrier],
    ) {
        // TODO: we may be able to elide cache flushes?

        // Metal cares not for your buffers and textures, not even your access masks it seems. They
        // just want the stages. It's also super helpful because I use before/after for my scopes,
        // and so does Metal. However, Metal's before/after mean the opposite to mine. Awesome.
        let mut src = MTLStages::empty();
        let mut dst = MTLStages::empty();
        for barrier in global_barriers {
            src |= conv::barrier_sync_to_mtl(barrier.before_sync);
            dst |= conv::barrier_sync_to_mtl(barrier.after_sync);
        }
        for barrier in buffer_barriers {
            src |= conv::barrier_sync_to_mtl(barrier.before_sync);
            dst |= conv::barrier_sync_to_mtl(barrier.after_sync);
        }
        for barrier in texture_barriers {
            src |= conv::barrier_sync_to_mtl(barrier.before_sync);
            dst |= conv::barrier_sync_to_mtl(barrier.after_sync);
        }

        // If either half of the barrier is empty then we just skip it because an empty edge has
        // no meaning to Metal.
        //
        // If one half is empty then it doesn't matter what the other half is, because the command
        // doesn't actually sync with anything (on Metal).
        if src.is_empty() || dst.is_empty() {
            return;
        }

        match &self.active.inner {
            ActiveEncoderInner::Graphics(_) => {
                // barriers can't be issued inside render passes within our api. however we would
                // like to issue producer barriers wherever possible. to that end, we don't eagerly
                // end the render __encoder__ when a caller ends the rhi render pass.
                //
                // this means we can actually still be inside a render encoder when a resource
                // barrier command comes in.
                //
                // we just add the relevant stages to the dirty sets and flush them as a producer
                // barrier when ending the render encoder.
                //
                // we don't issue an intra-pass barrier because our rhi doesn't allow barriers
                // within render passes. you can't have intra-pass sync under aleph-rhi.
                self.active.dirty_src |= src;
                self.active.dirty_dst |= dst;
            }
            ActiveEncoderInner::Compute(v) => {
                let src_compute = src & COMPUTE_STAGES;
                let dst_compute = dst & COMPUTE_STAGES;

                // immediately issue an intra-pass barrier if the dst mask intersects with any
                // stages for a compute encoder. this will ensure we correctly synchronize with
                // possible previous work earlier in the encoder.
                if !dst_compute.is_empty() {
                    v.barrierAfterEncoderStages_beforeEncoderStages_visibilityOptions(
                        src_compute,
                        dst_compute,
                        MTL4VisibilityOptions::Device,
                    );
                }

                self.active.dirty_src |= src;
                self.active.dirty_dst |= dst;
            }
            ActiveEncoderInner::None => {
                // there's no active encoder, so we can't eagerly issue a barrier.
                //
                // instead we just mark the stages as dirty. when we begin a new encoder
                // we will issue a consumer barrier that will synchronize these stages before
                // issuing any more commands.
                self.active.dirty_src |= src;
                self.active.dirty_dst |= dst;
            }
        }
    }

    unsafe fn __copy_buffer_regions(
        &mut self,
        src: &BufferHandle,
        dst: &BufferHandle,
        regions: &[BufferCopyRegion],
    ) {
        let src = Buffer::get(src);
        let dst = Buffer::get(dst);

        let encoder = self
            .active
            .begin_compute(&self.objects.list, &self._parent.objects.argument_table);

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

    unsafe fn __copy_buffer_to_texture(
        &mut self,
        src: &BufferHandle,
        dst: &TextureHandle,
        regions: &[BufferToTextureCopyRegion],
    ) {
        let src = Buffer::get(src);
        let dst = Texture::get(dst);

        let encoder = self
            .active
            .begin_compute(&self.objects.list, &self._parent.objects.argument_table);

        for region in regions {
            unsafe {
                let bytes_per_element = dst.desc().format.bytes_per_element() as usize;
                let source_bytes_per_row = region.src.row_pitch as usize * bytes_per_element;
                let source_bytes_per_image = match dst.desc.get().dimension {
                    TextureDimension::Texture1D | TextureDimension::Texture2D => 0,
                    TextureDimension::Texture3D => {
                        // Only 3D textures should have this != 0.
                        source_bytes_per_row * region.dst.extent.height as usize
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

    unsafe fn __copy_texture_regions(
        &mut self,
        src: &TextureHandle,
        dst: &TextureHandle,
        regions: &[TextureToTextureCopyInfo],
    ) {
        let src = Texture::get(src);
        let dst = Texture::get(dst);

        let encoder = self
            .active
            .begin_compute(&self.objects.list, &self._parent.objects.argument_table);

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

    unsafe fn __close(&mut self) -> Result<(), CommandListCloseError> {
        match self._parent.state {
            ListState::Empty => Err(CommandListCloseError::AlreadyClosed),
            ListState::Open => {
                self.active.end_all(&self._parent.objects.list);
                self._parent.objects.list.endCommandBuffer();
                self._parent.state = ListState::Closed;
                Ok(())
            }
            ListState::Closed => Err(CommandListCloseError::AlreadyClosed),
        }
    }

    unsafe fn __set_marker(&mut self, _color: Color, _message: &aleph_nstr::NStr) {
        // TODO: this
    }

    unsafe fn __begin_event(&mut self, _color: Color, message: &aleph_nstr::NStr) {
        autoreleasepool(|_| {
            self.objects
                .list
                .pushDebugGroup(&NSString::from_str(message.to_str()))
        })
    }

    unsafe fn __end_event(&mut self) {
        autoreleasepool(|_| {
            self.objects.list.popDebugGroup();
        })
    }
}

/// Wrapper to limit the scope of our 'unsafe impl Send'
pub struct EncoderObjects {
    pub list: Retained<ProtocolObject<dyn MTL4CommandBuffer>>,
}

unsafe impl Send for EncoderObjects {}

pub struct ActiveEncoder {
    dirty_src: MTLStages,
    dirty_dst: MTLStages,
    inner: ActiveEncoderInner,
}

enum ActiveEncoderInner {
    Graphics(Retained<ProtocolObject<dyn MTL4RenderCommandEncoder>>),
    Compute(Retained<ProtocolObject<dyn MTL4ComputeCommandEncoder>>),
    None,
}

impl ActiveEncoder {
    pub const fn new() -> Self {
        Self {
            dirty_src: MTLStages::empty(),
            dirty_dst: MTLStages::empty(),
            inner: ActiveEncoderInner::None,
        }
    }

    pub fn set_render(
        &mut self,
        list: &ProtocolObject<dyn MTL4CommandBuffer>,
        argument_table: &ProtocolObject<dyn MTL4ArgumentTable>,
        desc: &MTL4RenderPassDescriptor,
    ) {
        match &self.inner {
            ActiveEncoderInner::Graphics(old) => {
                Self::flush_producer_barrier(
                    &mut self.dirty_src,
                    &mut self.dirty_dst,
                    old.as_ref(),
                );
                old.endEncoding();
            }
            ActiveEncoderInner::Compute(old) => {
                Self::flush_producer_barrier(
                    &mut self.dirty_src,
                    &mut self.dirty_dst,
                    old.as_ref(),
                );

                old.endEncoding();
            }
            ActiveEncoderInner::None => {
                // do nothing, dirty stages need to be handled with a consumer barrier instead.
            }
        }

        let encoder = list.renderCommandEncoderWithDescriptor(desc);
        match encoder {
            Some(v) => {
                // TODO: can/should we be smarter about the stage mask?
                Self::flush_consumer_barrier(&mut self.dirty_src, &mut self.dirty_dst, v.as_ref());
                v.setArgumentTable_atStages(argument_table, MTLRenderStages::all());
                self.inner = ActiveEncoderInner::Graphics(v);
            }
            None => {
                log::error!("Failed to create 'MTLCommandRenderEncoder'!");
                panic!("Failed to create 'MTLCommandRenderEncoder'!");
            }
        }
    }

    pub fn get_render(&self) -> &ProtocolObject<dyn MTL4RenderCommandEncoder> {
        match &self.inner {
            ActiveEncoderInner::Graphics(v) => v.as_ref(),
            ActiveEncoderInner::Compute(_) => panic!(),
            ActiveEncoderInner::None => {
                log::error!("Must begin render encoder with 'begin_rendering'!");
                panic!("Must begin render encoder with 'begin_rendering'!")
            }
        }
    }

    pub fn begin_compute<'a>(
        &'a mut self,
        list: &ProtocolObject<dyn MTL4CommandBuffer>,
        argument_table: &ProtocolObject<dyn MTL4ArgumentTable>,
    ) -> &'a ProtocolObject<dyn MTL4ComputeCommandEncoder> {
        // This code is a bit deranged because the borrow checker doesn't like the simpler version.
        // It is what it is I guess /shrug.
        match &self.inner {
            ActiveEncoderInner::Graphics(old) => {
                Self::flush_producer_barrier(
                    &mut self.dirty_src,
                    &mut self.dirty_dst,
                    old.as_ref(),
                );

                old.endEncoding();
            }
            ActiveEncoderInner::Compute(_) => {}
            ActiveEncoderInner::None => {}
        };

        match &self.inner {
            ActiveEncoderInner::Graphics(_) | ActiveEncoderInner::None => {
                autoreleasepool(|_| {
                    let v = list.computeCommandEncoder().unwrap();
                    Self::flush_consumer_barrier(
                        &mut self.dirty_src,
                        &mut self.dirty_dst,
                        v.as_ref(),
                    );
                    v.setArgumentTable(Some(argument_table));
                    self.inner = ActiveEncoderInner::Compute(v);
                });
            }
            ActiveEncoderInner::Compute(_) => {}
        }

        match &self.inner {
            ActiveEncoderInner::Compute(v) => v.as_ref(),
            _ => unsafe { unreachable_unchecked() },
        }
    }

    pub fn end_all(&mut self, list: &ProtocolObject<dyn MTL4CommandBuffer>) {
        autoreleasepool(|_| {
            match &self.inner {
                ActiveEncoderInner::Graphics(old) => {
                    Self::flush_producer_barrier(
                        &mut self.dirty_src,
                        &mut self.dirty_dst,
                        old.as_ref(),
                    );
                    old.endEncoding();
                }
                ActiveEncoderInner::Compute(old) => {
                    Self::flush_producer_barrier(
                        &mut self.dirty_src,
                        &mut self.dirty_dst,
                        old.as_ref(),
                    );
                    old.endEncoding();
                }
                ActiveEncoderInner::None => {
                    if !self.dirty_src.is_empty() || !self.dirty_dst.is_empty() {
                        autoreleasepool(|_| {
                            let v = list.computeCommandEncoder().unwrap();
                            Self::flush_producer_barrier(
                                &mut self.dirty_src,
                                &mut self.dirty_dst,
                                v.as_ref(),
                            );
                            v.endEncoding();
                        });
                    }
                }
            }
            self.inner = ActiveEncoderInner::None;
        })
    }

    fn should_flush(dirty_src: &MTLStages, dirty_dst: &MTLStages) -> bool {
        let both_empty = dirty_src.is_empty() && dirty_dst.is_empty();
        let none_empty = !dirty_src.is_empty() && !dirty_dst.is_empty();

        debug_assert!(
            both_empty || none_empty,
            "It is illegal for only one half of the dirty stages to be empty"
        );

        none_empty
    }

    fn flush_producer_barrier(
        dirty_src: &mut MTLStages,
        dirty_dst: &mut MTLStages,
        encoder: &ProtocolObject<dyn MTL4CommandEncoder>,
    ) {
        // Skip issuing a barrier if there are no dirty stages
        if !Self::should_flush(dirty_src, dirty_dst) {
            return;
        }

        // Flush all the outstanding dirty stages with a producer barrier before ending
        // the old encoder
        encoder.barrierAfterStages_beforeQueueStages_visibilityOptions(
            *dirty_src,
            *dirty_dst,
            MTL4VisibilityOptions::Device,
        );
        *dirty_src = MTLStages::empty();
        *dirty_dst = MTLStages::empty();
    }

    fn flush_consumer_barrier(
        dirty_src: &mut MTLStages,
        dirty_dst: &mut MTLStages,
        encoder: &ProtocolObject<dyn MTL4CommandEncoder>,
    ) {
        // Skip issuing a barrier if there are no dirty stages
        if !Self::should_flush(dirty_src, dirty_dst) {
            return;
        }

        // Flush all the outstanding dirty stages with a producer barrier before ending
        // the old encoder
        encoder.barrierAfterQueueStages_beforeStages_visibilityOptions(
            *dirty_src,
            *dirty_dst,
            MTL4VisibilityOptions::Device,
        );
        *dirty_src = MTLStages::empty();
        *dirty_dst = MTLStages::empty();
    }
}

unsafe impl Send for ActiveEncoder {}

pub struct BoundIndexBuffer {
    addr: NonZero<u64>,
    index_type: MTLIndexType,
    index_size: usize,
}

unsafe impl Send for BoundIndexBuffer {}

pub struct BoundPipelineState {
    primitive_type: MTLPrimitiveType,
    push_params: [BVec<u64, RhiSystem>; 8],
    push_params_dirty: bool,
    push_constant_block: BVec<u8, RhiSystem>,
}

impl BoundPipelineState {
    pub unsafe fn maybe_flush_params(
        &mut self,
        device: &Device,
        allocator: &mut UploadBumpAllocator,
        argument_table: &ProtocolObject<dyn MTL4ArgumentTable>,
        binding_signature: &BindingSignature,
    ) {
        if self.push_params_dirty {
            let iter = binding_signature
                ._parameter_block_layouts
                .iter()
                .enumerate();
            for (i, block) in iter {
                let is_push_descriptor_block = block
                    .desc()
                    .flags
                    .contains(ParameterBlockFlags::PUSH_DESCRIPTOR);
                if is_push_descriptor_block {
                    unsafe {
                        self.flush_params(device, allocator, argument_table, i);
                    }
                }
            }
            self.push_params_dirty = false;
        }
    }

    unsafe fn flush_params(
        &self,
        device: &Device,
        allocator: &mut UploadBumpAllocator,
        argument_table: &ProtocolObject<dyn MTL4ArgumentTable>,
        i: usize,
    ) {
        let params = self.push_params[i].as_slice();

        let (cpu_addr, gpu_addr) = allocator.allocate(device, params.len() * size_of::<u64>());
        let cpu_addr = cpu_addr.cast::<u64>();

        unsafe {
            cpu_addr.copy_from(NonNull::from(params).cast::<u64>(), params.len());
        }

        unsafe {
            argument_table.setAddress_atIndex(gpu_addr.get(), i);
        }
    }
}

impl Default for BoundPipelineState {
    fn default() -> Self {
        Self {
            primitive_type: MTLPrimitiveType::Point,
            push_params: [const { BVec::new_in(system()) }; 8],
            push_params_dirty: false,
            push_constant_block: aleph_alloc::vec![in system(); 0u8; 256],
        }
    }
}

unsafe impl Send for BoundPipelineState {}
