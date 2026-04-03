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

use aleph_alloc::{BVec, Blink, BlinkAlloc};
use aleph_object_system::Object;
use aleph_rhi_api::*;
use aleph_rhi_impl_utils::{RhiSystem, try_clone_value_into_slot};
use ash::vk;

use crate::buffer::Buffer;
use crate::command_list::{CommandList, ListState};
use crate::context::Context;
use crate::device::Device;
use crate::internal::conv::*;
use crate::internal::unwrap;
use crate::internal::write_descriptors::translate_descriptor_writes;
use crate::pipeline::{ComputePipeline, GraphicsPipeline};
use crate::texture::Texture;

pub struct Encoder<'a> {
    pub(crate) _parent: &'a mut CommandList,
    pub(crate) _buffer: vk::CommandBuffer,
    pub(crate) _context: Arc<Context>,
    pub(crate) _device: Arc<Device>,
    pub(crate) bound_graphics_pipeline: Option<Arc<Object<GraphicsPipeline>>>,
    pub(crate) bound_compute_pipeline: Option<Arc<Object<ComputePipeline>>>,
    pub(crate) arena: Blink<BlinkAlloc<RhiSystem>>,
}

impl<'a> IGetPlatformInterface for Encoder<'a> {
    unsafe fn __query_platform_interface(&self, target: TypeId, out: *mut ()) -> Option<()> {
        unsafe { try_clone_value_into_slot::<vk::CommandBuffer>(&self._buffer, out, target) }
    }
}

impl<'a> ICommandEncoderAbi for Encoder<'a> {
    unsafe fn __bind_graphics_pipeline(&mut self, pipeline: &GraphicsPipelineHandle) {
        unsafe {
            let concrete = GraphicsPipeline::get_owned(pipeline);

            // Binds the pipeline
            self._device.device.cmd_bind_pipeline(
                self._buffer,
                vk::PipelineBindPoint::GRAPHICS,
                concrete.pipeline,
            );

            // We need the currently bound pipeline while recording commands to access things like
            // the pipeline layout for handling binding descriptors.
            self.bound_graphics_pipeline = Some(concrete);
        }
    }

    unsafe fn __bind_vertex_buffers(
        &mut self,
        first_binding: u32,
        bindings: &[InputAssemblyBufferBinding],
    ) {
        {
            let mut buffers = BVec::with_capacity_in(bindings.len(), self.arena.allocator());
            let mut offsets = BVec::with_capacity_in(bindings.len(), self.arena.allocator());
            for v in bindings.iter() {
                let buffer = Buffer::get(v.buffer);

                buffers.push(buffer.buffer);
                offsets.push(v.offset);
            }

            unsafe {
                self._device.device.cmd_bind_vertex_buffers(
                    self._buffer,
                    first_binding,
                    &buffers,
                    &offsets,
                );
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

        let index_type = match index_type {
            IndexType::U16 => vk::IndexType::UINT16,
            IndexType::U32 => vk::IndexType::UINT32,
        };

        unsafe {
            self._device.device.cmd_bind_index_buffer(
                self._buffer,
                buffer.buffer,
                binding.offset,
                index_type,
            )
        }
    }

    unsafe fn __set_viewports(&mut self, viewports: &[Viewport]) {
        {
            let mut new_viewports = BVec::with_capacity_in(viewports.len(), self.arena.allocator());
            for v in viewports {
                new_viewports.push(
                    vk::Viewport::default()
                        .x(v.x)
                        .y(v.y + v.height)
                        .width(v.width)
                        .height(-v.height)
                        .min_depth(v.min_depth)
                        .max_depth(v.max_depth),
                );
            }

            unsafe {
                self._device
                    .device
                    .cmd_set_viewport(self._buffer, 0, &new_viewports);
            }
        }

        self.arena.reset();
    }

    unsafe fn __set_scissor_rects(&mut self, rects: &[Rect]) {
        {
            let mut new_rects = BVec::with_capacity_in(rects.len(), self.arena.allocator());
            for v in rects {
                let mut rect = vk::Rect2D::default();
                rect.offset.x = v.x as i32;
                rect.offset.y = v.y as i32;
                rect.extent.width = v.w;
                rect.extent.height = v.h;
                new_rects.push(rect);
            }

            unsafe {
                self._device
                    .device
                    .cmd_set_scissor(self._buffer, 0, &new_rects);
            }
        }

        self.arena.reset();
    }

    unsafe fn __set_push_constant_block(&mut self, data: &[u8]) {
        let pipeline = self
            .bound_graphics_pipeline
            .as_deref()
            .expect("Can't set push constants without a pipeline bound");
        let binding_signature = pipeline._binding_signature.as_ref();
        let info = binding_signature.push_constant_block.as_ref().unwrap();

        unsafe {
            self._device.device.cmd_push_constants(
                self._buffer,
                binding_signature.pipeline_layout,
                info.stage_flags,
                info.offset,
                data,
            )
        }
    }

    unsafe fn __begin_rendering(&mut self, info: &BeginRenderingInfo) {
        {
            let mut color_attachments =
                BVec::with_capacity_in(info.color_attachments.len(), self.arena.allocator());
            for v in info.color_attachments {
                let image_view: vk::ImageView = unsafe { std::mem::transmute(v.image_view) };

                let mut info = vk::RenderingAttachmentInfo::default()
                    .image_view(image_view)
                    .image_layout(image_layout_to_vk(v.image_layout))
                    .load_op(load_op_to_vk(&v.load_op))
                    .store_op(store_op_to_vk(&v.store_op));

                if let AttachmentLoadOp::Clear(v) = &v.load_op {
                    info = info.clear_value(vk::ClearValue {
                        color: color_clear_to_vk(v),
                    });
                };
                color_attachments.push(info);
            }

            let (depth_attachment, stencil_attachment) =
                if let Some(v) = info.depth_stencil_attachment {
                    let image_view: vk::ImageView = unsafe { std::mem::transmute(v.image_view) };

                    let depth = if let Some(ops) = &v.depth {
                        let mut info = vk::RenderingAttachmentInfo::default()
                            .image_view(image_view)
                            .image_layout(image_layout_to_vk(v.image_layout))
                            .load_op(load_op_to_vk(&ops.load_op))
                            .store_op(store_op_to_vk(&ops.store_op));

                        if let AttachmentLoadOp::Clear(v) = &ops.load_op {
                            info = info.clear_value(vk::ClearValue {
                                depth_stencil: vk::ClearDepthStencilValue {
                                    depth: *v,
                                    stencil: 0,
                                },
                            });
                        };

                        Some(info)
                    } else {
                        None
                    };

                    let stencil = if let Some(ops) = &v.stencil {
                        let mut info = vk::RenderingAttachmentInfo::default()
                            .image_view(image_view)
                            .image_layout(image_layout_to_vk(v.image_layout))
                            .load_op(load_op_to_vk(&ops.load_op))
                            .store_op(store_op_to_vk(&ops.store_op));

                        if let AttachmentLoadOp::Clear(v) = &ops.load_op {
                            let value = vk::ClearValue {
                                depth_stencil: vk::ClearDepthStencilValue {
                                    depth: 0.0,
                                    stencil: *v as u32,
                                },
                            };
                            info = info.clear_value(value);
                        };

                        Some(info)
                    } else {
                        None
                    };

                    (depth, stencil)
                } else {
                    (None, None)
                };

            // Select the width/height of the first attachment we find. We require that all
            // attachments are the same size in the API so we only need to grab the size for one of
            // them and assume the rest are the same size.
            //
            // The validation layer should catch this.
            let render_extent = {
                vk::Extent2D {
                    width: info.extent.width,
                    height: info.extent.height,
                }
            };

            let mut info = vk::RenderingInfo::default()
                .render_area(vk::Rect2D {
                    offset: vk::Offset2D::default(),
                    extent: render_extent,
                })
                .layer_count(info.layer_count)
                .color_attachments(&color_attachments);
            if let Some(v) = &depth_attachment {
                info = info.depth_attachment(v);
            }
            if let Some(v) = &stencil_attachment {
                info = info.stencil_attachment(v);
            }

            unsafe {
                self._device.device.cmd_begin_rendering(self._buffer, &info);
            }
        }

        self.arena.reset();
    }

    unsafe fn __end_rendering(&mut self) {
        unsafe {
            self._device.device.cmd_end_rendering(self._buffer);
        }
    }

    unsafe fn __draw(
        &mut self,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    ) {
        unsafe {
            self._device.device.cmd_draw(
                self._buffer,
                vertex_count,
                instance_count,
                first_vertex,
                first_instance,
            )
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
        unsafe {
            self._device.device.cmd_draw_indexed(
                self._buffer,
                index_count,
                instance_count,
                first_index,
                vertex_offset,
                first_instance,
            )
        }
    }

    unsafe fn __bind_compute_pipeline(&mut self, pipeline: &ComputePipelineHandle) {
        let concrete = ComputePipeline::get_owned(pipeline);

        // Binds the pipeline
        unsafe {
            self._device.device.cmd_bind_pipeline(
                self._buffer,
                vk::PipelineBindPoint::COMPUTE,
                concrete.pipeline,
            );
        }

        // We need the currently bound pipeline while recording commands to access things like
        // the pipeline layout for handling binding descriptors.
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
        let bind_point = pipeline_bind_point_to_vk(bind_point);

        unsafe {
            let new_sets: &[vk::DescriptorSet] = std::mem::transmute(blocks);

            self._device.device.cmd_bind_descriptor_sets(
                self._buffer,
                bind_point,
                binding_signature.pipeline_layout,
                first_block,
                new_sets,
                &[],
            );
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
        {
            let binding_signature = unwrap::binding_signature(binding_signature);
            let bind_point = pipeline_bind_point_to_vk(bind_point);
            let block_layout = &binding_signature.parameter_block_layouts[block as usize];

            let layout_desc = block_layout.desc.get();

            let descriptor_writes = translate_descriptor_writes(
                layout_desc,
                base,
                writes,
                vk::DescriptorSet::null(),
                self.arena.allocator(),
            );

            unsafe {
                self._device.push_descriptor.cmd_push_descriptor_set(
                    self._buffer,
                    bind_point,
                    binding_signature.pipeline_layout,
                    block,
                    &descriptor_writes,
                )
            };
        }

        self.arena.reset();
    }

    unsafe fn __dispatch(&mut self, group_count_x: u32, group_count_y: u32, group_count_z: u32) {
        unsafe {
            self._device.device.cmd_dispatch(
                self._buffer,
                group_count_x,
                group_count_y,
                group_count_z,
            );
        }
    }

    unsafe fn __resource_barrier(
        &mut self,
        global_barriers: &[GlobalBarrier],
        buffer_barriers: &[BufferBarrier],
        texture_barriers: &[TextureBarrier],
    ) {
        #![allow(non_snake_case)]

        {
            let mut translated_global_barriers =
                BVec::with_capacity_in(global_barriers.len(), self.arena.allocator());
            let mut translated_buffer_barriers =
                BVec::with_capacity_in(buffer_barriers.len(), self.arena.allocator());
            let mut translated_texture_barriers =
                BVec::with_capacity_in(texture_barriers.len(), self.arena.allocator());

            for barrier in global_barriers {
                translated_global_barriers.push(
                    vk::MemoryBarrier2::default()
                        .src_stage_mask(barrier_sync_to_vk(barrier.before_sync))
                        .dst_stage_mask(barrier_sync_to_vk(barrier.after_sync))
                        .src_access_mask(barrier_access_to_vk(barrier.before_access))
                        .dst_access_mask(barrier_access_to_vk(barrier.after_access)),
                );
            }

            for barrier in buffer_barriers {
                let buffer = barrier.buffer.unwrap();
                let buffer = Buffer::get(buffer);

                let (src_family, dst_family) = if let Some(transition) = barrier.queue_transition {
                    let src_family = self._device.get_queue_family_index(transition.before_queue);
                    let dst_family = self._device.get_queue_family_index(transition.after_queue);
                    (src_family, dst_family)
                } else {
                    (0, 0)
                };

                translated_buffer_barriers.push(
                    vk::BufferMemoryBarrier2::default()
                        .src_stage_mask(barrier_sync_to_vk(barrier.before_sync))
                        .dst_stage_mask(barrier_sync_to_vk(barrier.after_sync))
                        .src_access_mask(barrier_access_to_vk(barrier.before_access))
                        .dst_access_mask(barrier_access_to_vk(barrier.after_access))
                        .buffer(buffer.buffer)
                        .offset(barrier.offset)
                        .size(barrier.size)
                        .src_queue_family_index(src_family)
                        .dst_queue_family_index(dst_family),
                );
            }

            for barrier in texture_barriers {
                // Grab the d3d12 resource handle from our texture impls
                let texture = Texture::get(barrier.texture.unwrap());

                let (src_family, dst_family) = if let Some(transition) = barrier.queue_transition {
                    let src_family = self._device.get_queue_family_index(transition.before_queue);
                    let dst_family = self._device.get_queue_family_index(transition.after_queue);
                    (src_family, dst_family)
                } else {
                    (0, 0)
                };

                translated_texture_barriers.push(
                    vk::ImageMemoryBarrier2::default()
                        .src_stage_mask(barrier_sync_to_vk(barrier.before_sync))
                        .dst_stage_mask(barrier_sync_to_vk(barrier.after_sync))
                        .src_access_mask(barrier_access_to_vk(barrier.before_access))
                        .dst_access_mask(barrier_access_to_vk(barrier.after_access))
                        .old_layout(image_layout_to_vk(barrier.before_layout))
                        .new_layout(image_layout_to_vk(barrier.after_layout))
                        .image(texture.image)
                        .subresource_range(subresource_range_to_vk(&barrier.subresource_range))
                        .src_queue_family_index(src_family)
                        .dst_queue_family_index(dst_family),
                );
            }

            let info = vk::DependencyInfo::default()
                .memory_barriers(&translated_global_barriers)
                .buffer_memory_barriers(&translated_buffer_barriers)
                .image_memory_barriers(&translated_texture_barriers);

            unsafe {
                self._device
                    .device
                    .cmd_pipeline_barrier2(self._buffer, &info);
            }
        }

        self.arena.reset();
    }

    unsafe fn __copy_buffer_regions(
        &mut self,
        src: &BufferHandle,
        dst: &BufferHandle,
        regions: &[BufferCopyRegion],
    ) {
        {
            let src = Buffer::get(src);
            let dst = Buffer::get(dst);

            let mut new_regions = BVec::with_capacity_in(regions.len(), self.arena.allocator());
            for v in regions {
                new_regions.push(
                    vk::BufferCopy::default()
                        .src_offset(v.src_offset)
                        .dst_offset(v.dst_offset)
                        .size(v.size),
                );
            }

            unsafe {
                self._device.device.cmd_copy_buffer(
                    self._buffer,
                    src.buffer,
                    dst.buffer,
                    &new_regions,
                );
            }
        }
        self.arena.reset();
    }

    unsafe fn __copy_buffer_to_texture(
        &mut self,
        src: &BufferHandle,
        dst: &TextureHandle,
        regions: &[BufferToTextureCopyRegion],
    ) {
        {
            let src = Buffer::get(src);
            let dst = Texture::get(dst);

            let mut new_regions = BVec::with_capacity_in(regions.len(), self.arena.allocator());
            for v in regions {
                new_regions.push(
                    vk::BufferImageCopy::default()
                        .buffer_offset(v.src.offset)
                        .buffer_row_length(v.src.row_pitch)
                        .buffer_image_height(0) // implicitly maps to v.dst.extent.height
                        .image_subresource(vk::ImageSubresourceLayers {
                            aspect_mask: texture_copy_aspect_to_vk(v.dst.aspect),
                            mip_level: v.dst.mip_level,
                            base_array_layer: v.dst.array_layer,
                            layer_count: 1,
                        })
                        .image_offset(vk::Offset3D {
                            x: v.dst.origin.x as i32,
                            y: v.dst.origin.y as i32,
                            z: v.dst.origin.z as i32,
                        })
                        .image_extent(vk::Extent3D {
                            width: v.dst.extent.width,
                            height: v.dst.extent.height,
                            depth: v.dst.extent.depth,
                        }),
                );
            }

            unsafe {
                self._device.device.cmd_copy_buffer_to_image(
                    self._buffer,
                    src.buffer,
                    dst.image,
                    vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                    &new_regions,
                );
            }
        }
        self.arena.reset();
    }

    unsafe fn __copy_texture_regions(
        &mut self,
        src: &TextureHandle,
        dst: &TextureHandle,
        regions: &[TextureToTextureCopyInfo],
    ) {
        {
            let src = Texture::get(src);
            let dst = Texture::get(dst);

            // src_subresource
            // src_offset
            // dst_subresource
            // dst_offset
            let mut new_regions = BVec::with_capacity_in(regions.len(), self.arena.allocator());
            for v in regions {
                let src_subresource = vk::ImageSubresourceLayers {
                    aspect_mask: texture_copy_aspect_to_vk(v.src.aspect),
                    mip_level: v.src.mip_level,
                    base_array_layer: v.src.array_layer,
                    layer_count: 1,
                };
                let src_offset = vk::Offset3D { x: 0, y: 0, z: 0 };
                let dst_subresource = vk::ImageSubresourceLayers {
                    aspect_mask: texture_copy_aspect_to_vk(v.dst.aspect),
                    mip_level: v.dst.mip_level,
                    base_array_layer: v.dst.array_layer,
                    layer_count: 1,
                };
                let dst_offset = vk::Offset3D { x: 0, y: 0, z: 0 };
                let extent = vk::Extent3D {
                    width: v.extent.width,
                    height: v.extent.height,
                    depth: v.extent.depth,
                };
                new_regions.push(vk::ImageCopy {
                    src_subresource,
                    src_offset,
                    dst_subresource,
                    dst_offset,
                    extent,
                });
            }

            unsafe {
                self._device.device.cmd_copy_image(
                    self._buffer,
                    src.image,
                    vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
                    dst.image,
                    vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                    &new_regions,
                )
            }
        }
        self.arena.reset();
    }

    unsafe fn __close(&mut self) -> Result<(), CommandListCloseError> {
        match self._parent.state {
            ListState::Empty => Err(CommandListCloseError::AlreadyClosed),
            ListState::Open => unsafe {
                self._device
                    .device
                    .end_command_buffer(self._buffer)
                    .inspect_err(|v| log::error!("Platform Error: {:#?}", v))
                    .map_err(|_| CommandListCloseError::Platform)?;
                self._parent.state = ListState::Closed;
                Ok(())
            },
            ListState::Closed => Err(CommandListCloseError::AlreadyClosed),
        }
    }

    unsafe fn __set_marker(&mut self, color: Color, message: &aleph_nstr::NStr) {
        if let Some(loader) = self._device.debug_loader.as_ref() {
            let color: [f32; 4] = color.into();
            let info = vk::DebugUtilsLabelEXT::default()
                .label_name(message.to_cstr())
                .color(color);
            unsafe {
                loader.cmd_insert_debug_utils_label(self._buffer, &info);
            }
        }
        self.arena.reset();
    }

    unsafe fn __begin_event(&mut self, color: Color, message: &aleph_nstr::NStr) {
        if let Some(loader) = self._device.debug_loader.as_ref() {
            let color: [f32; 4] = color.into();
            let info = vk::DebugUtilsLabelEXT::default()
                .label_name(message.to_cstr())
                .color(color);
            unsafe {
                loader.cmd_begin_debug_utils_label(self._buffer, &info);
            }
        }
        self.arena.reset();
    }

    unsafe fn __end_event(&mut self) {
        if let Some(loader) = self._device.debug_loader.as_ref() {
            unsafe { loader.cmd_end_debug_utils_label(self._buffer) }
        }
    }
}
