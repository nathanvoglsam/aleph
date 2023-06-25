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

use crate::command_list::CommandList;
use crate::context::Context;
use crate::internal::conv::*;
use crate::internal::unwrap;
use crate::pipeline::GraphicsPipeline;
use aleph_any::AnyArc;
use aleph_rhi_api::*;
use aleph_rhi_impl_utils::try_clone_value_into_slot;
use ash::extensions::khr::Synchronization2;
use ash::vk;
use bumpalo::collections::Vec as BumpVec;
use bumpalo::Bump;
use std::any::TypeId;
use std::ffi::CStr;
use std::marker::PhantomData;
use std::ops::Deref;
use crate::device::Device;

pub struct Encoder<'a> {
    pub(crate) _buffer: vk::CommandBuffer,
    pub(crate) _context: AnyArc<Context>,
    pub(crate) _device: AnyArc<Device>,
    pub(crate) bound_graphics_pipeline: Option<AnyArc<GraphicsPipeline>>,
    pub(crate) arena: Bump,
    pub(crate) enabled_shader_features: SyncShaderFeatures,
    pub(crate) phantom_data: PhantomData<&'a mut CommandList>,
}

impl<'a> Drop for Encoder<'a> {
    fn drop(&mut self) {
        // TODO: Consider an API that forces manually closing so we can avoid the unwrap here
        unsafe {
            self
                ._device
                .device
                .end_command_buffer(self._buffer)
                .unwrap()
        }
    }
}

impl<'a> IGetPlatformInterface for Encoder<'a> {
    unsafe fn __query_platform_interface(&self, target: TypeId, out: *mut ()) -> Option<()> {
        try_clone_value_into_slot::<vk::CommandBuffer>(&self._buffer, out, target)
    }
}

impl<'a> IGeneralEncoder for Encoder<'a> {
    unsafe fn bind_graphics_pipeline(&mut self, pipeline: &dyn IGraphicsPipeline) {
        let concrete = unwrap::graphics_pipeline(pipeline);

        // Binds the pipeline
        self._device.device.cmd_bind_pipeline(
            self._buffer,
            vk::PipelineBindPoint::GRAPHICS,
            concrete.pipeline,
        );

        // We need the currently bound pipeline while recording commands to access things like
        // the pipeline layout for handling binding descriptors.
        self.bound_graphics_pipeline = Some(concrete._this.upgrade().unwrap());
    }

    unsafe fn bind_vertex_buffers(
        &mut self,
        first_binding: u32,
        bindings: &[InputAssemblyBufferBinding],
    ) {
        let mut buffers = BumpVec::with_capacity_in(bindings.len(), &self.arena);
        let mut offsets = BumpVec::with_capacity_in(bindings.len(), &self.arena);
        for v in bindings.iter() {
            let buffer = unwrap::buffer(v.buffer);

            buffers.push(buffer.buffer);
            offsets.push(v.offset);
        }

        self._device.device.cmd_bind_vertex_buffers(
            self._buffer,
            first_binding,
            &buffers,
            &offsets,
        )
    }

    unsafe fn bind_index_buffer(
        &mut self,
        index_type: IndexType,
        binding: &InputAssemblyBufferBinding,
    ) {
        let buffer = unwrap::buffer(binding.buffer);

        let index_type = match index_type {
            IndexType::U16 => vk::IndexType::UINT16,
            IndexType::U32 => vk::IndexType::UINT32,
        };

        self._device.device.cmd_bind_index_buffer(
            self._buffer,
            buffer.buffer,
            binding.offset,
            index_type,
        )
    }

    unsafe fn set_viewports(&mut self, viewports: &[Viewport]) {
        let mut new_viewports = BumpVec::with_capacity_in(viewports.len(), &self.arena);
        for v in viewports {
            new_viewports.push(
                vk::Viewport::builder()
                    .x(v.x)
                    .y(v.y + v.height)
                    .width(v.width)
                    .height(-v.height)
                    .min_depth(v.min_depth)
                    .max_depth(v.max_depth)
                    .build(),
            );
        }

        self
            ._device
            .device
            .cmd_set_viewport(self._buffer, 0, &new_viewports)
    }

    unsafe fn set_scissor_rects(&mut self, rects: &[Rect]) {
        let mut new_rects = BumpVec::with_capacity_in(rects.len(), &self.arena);
        for v in rects {
            let mut rect = vk::Rect2D::builder();
            rect.offset.x = v.x as i32;
            rect.offset.y = v.y as i32;
            rect.extent.width = v.w;
            rect.extent.height = v.h;
            new_rects.push(rect.build());
        }

        self
            ._device
            .device
            .cmd_set_scissor(self._buffer, 0, &new_rects)
    }

    unsafe fn set_push_constant_block(&mut self, block_index: usize, data: &[u8]) {
        let pipeline_layout = self
            .bound_graphics_pipeline
            .as_ref()
            .expect("Can't set push constants without a pipeline bound")
            ._pipeline_layout
            .as_ref();

        let info = &pipeline_layout.push_constant_blocks[block_index];

        self._device.device.cmd_push_constants(
            self._buffer,
            pipeline_layout.pipeline_layout,
            info.stage_flags,
            info.offset,
            data,
        )
    }

    unsafe fn begin_rendering(&mut self, info: &BeginRenderingInfo) {
        let mut color_attachments =
            BumpVec::with_capacity_in(info.color_attachments.len(), &self.arena);
        for v in info.color_attachments {
            let image_view = std::mem::transmute::<_, vk::ImageView>(v.image_view);

            let mut info = vk::RenderingAttachmentInfo::builder()
                .image_view(image_view)
                .image_layout(image_layout_to_vk(v.image_layout))
                .load_op(load_op_to_vk(&v.load_op))
                .store_op(store_op_to_vk(&v.store_op));

            if let AttachmentLoadOp::Clear(v) = &v.load_op {
                info = info.clear_value(vk::ClearValue {
                    color: color_clear_to_vk(v),
                });
            };
            color_attachments.push(info.build());
        }

        let (depth_attachment, stencil_attachment) = if let Some(v) = info.depth_stencil_attachment
        {
            let image_view = std::mem::transmute::<_, vk::ImageView>(v.image_view);

            let depth_info = if !matches!(&v.depth_load_op, &AttachmentLoadOp::None) {
                let mut info = vk::RenderingAttachmentInfo::builder()
                    .image_view(image_view)
                    .image_layout(image_layout_to_vk(v.image_layout))
                    .load_op(load_op_to_vk(&v.depth_load_op))
                    .store_op(store_op_to_vk(&v.depth_store_op));

                if let AttachmentLoadOp::Clear(v) = &v.depth_load_op {
                    info = info.clear_value(vk::ClearValue {
                        depth_stencil: depth_stencil_clear_to_vk(*v),
                    });
                };

                Some(info)
            } else {
                None
            };

            let stencil_info = if !matches!(&v.stencil_load_op, &AttachmentLoadOp::None) {
                let mut info = vk::RenderingAttachmentInfo::builder()
                    .image_view(image_view)
                    .image_layout(image_layout_to_vk(v.image_layout))
                    .load_op(load_op_to_vk(&v.stencil_load_op))
                    .store_op(store_op_to_vk(&v.stencil_store_op));

                if let AttachmentLoadOp::Clear(v) = &v.stencil_load_op {
                    let value = vk::ClearValue {
                        depth_stencil: depth_stencil_clear_to_vk(*v),
                    };
                    info = info.clear_value(value);
                };

                Some(info)
            } else {
                None
            };

            (depth_info, stencil_info)
        } else {
            (None, None)
        };

        // Select the width/height of the first attachment we find. We require that all attachments
        // are the same size in the API so we only need to grab the size for one of them and assume
        // the rest are the same size.
        //
        // The validation layer should catch this.
        let render_extent = {
            vk::Extent2D {
                width: info.extent.width,
                height: info.extent.height,
            }
        };

        let mut info = vk::RenderingInfo::builder()
            .render_area(vk::Rect2D {
                offset: vk::Offset2D::default(),
                extent: render_extent,
            })
            .layer_count(info.layer_count)
            .color_attachments(&color_attachments);
        if let Some(v) = &depth_attachment {
            info = info.depth_attachment(&v);
        }
        if let Some(v) = &stencil_attachment {
            info = info.stencil_attachment(&v);
        }

        self
            ._device
            .dynamic_rendering
            .cmd_begin_rendering(self._buffer, &info);
    }

    unsafe fn end_rendering(&mut self) {
        self
            ._device
            .dynamic_rendering
            .cmd_end_rendering(self._buffer);
    }

    unsafe fn draw(
        &mut self,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    ) {
        self._device.device.cmd_draw(
            self._buffer,
            vertex_count,
            instance_count,
            first_vertex,
            first_instance,
        )
    }

    unsafe fn draw_indexed(
        &mut self,
        index_count: u32,
        instance_count: u32,
        first_index: u32,
        first_instance: u32,
        vertex_offset: i32,
    ) {
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

impl<'a> IComputeEncoder for Encoder<'a> {
    unsafe fn bind_descriptor_sets(
        &mut self,
        pipeline_layout: &dyn IPipelineLayout,
        bind_point: PipelineBindPoint,
        first_set: u32,
        sets: &[DescriptorSetHandle],
    ) {
        let pipeline_layout = unwrap::pipeline_layout(pipeline_layout);
        let bind_point = pipeline_bind_point_to_vk(bind_point);

        let mut new_sets = BumpVec::with_capacity_in(sets.len(), &self.arena);
        for v in sets {
            new_sets.push(std::mem::transmute_copy::<_, vk::DescriptorSet>(v));
        }

        self._device.device.cmd_bind_descriptor_sets(
            self._buffer,
            bind_point,
            pipeline_layout.pipeline_layout,
            first_set,
            &new_sets,
            &[],
        );
    }

    unsafe fn dispatch(&mut self, group_count_x: u32, group_count_y: u32, group_count_z: u32) {
        self._device.device.cmd_dispatch(
            self._buffer,
            group_count_x,
            group_count_y,
            group_count_z,
        );
    }
}

impl<'a> ITransferEncoder for Encoder<'a> {
    unsafe fn resource_barrier(
        &mut self,
        global_barriers: &[GlobalBarrier],
        buffer_barriers: &[BufferBarrier],
        texture_barriers: &[TextureBarrier],
    ) {
        if let Some(loader) = self._device.synchronization_2.clone() {
            self.resource_barrier_sync2(&loader, global_barriers, buffer_barriers, texture_barriers)
        } else {
            self.resource_barrier_fallback(global_barriers, buffer_barriers, texture_barriers)
        }
    }

    unsafe fn copy_buffer_regions(
        &mut self,
        src: &dyn IBuffer,
        dst: &dyn IBuffer,
        regions: &[BufferCopyRegion],
    ) {
        let src = unwrap::buffer(src);
        let dst = unwrap::buffer(dst);

        let mut new_regions = BumpVec::with_capacity_in(regions.len(), &self.arena);
        for v in regions {
            new_regions.push(
                vk::BufferCopy::builder()
                    .src_offset(v.src_offset)
                    .dst_offset(v.dst_offset)
                    .size(v.size)
                    .build(),
            );
        }

        self._device.device.cmd_copy_buffer(
            self._buffer,
            src.buffer,
            dst.buffer,
            &new_regions,
        )
    }

    unsafe fn copy_buffer_to_texture(
        &mut self,
        src: &dyn IBuffer,
        dst: &dyn ITexture,
        dst_layout: ImageLayout,
        regions: &[BufferToTextureCopyRegion],
    ) {
        let src = unwrap::buffer(src);
        let dst = unwrap::texture(dst);

        let mut new_regions = BumpVec::with_capacity_in(regions.len(), &self.arena);
        for v in regions {
            new_regions.push(
                vk::BufferImageCopy::builder()
                    .buffer_offset(v.src.offset)
                    .buffer_row_length(v.src.extent.width)
                    .buffer_image_height(v.src.extent.height)
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
                    })
                    .build(),
            );
        }

        let layout = image_layout_to_vk(dst_layout);

        self._device.device.cmd_copy_buffer_to_image(
            self._buffer,
            src.buffer,
            dst.image,
            layout,
            &new_regions,
        );
    }

    unsafe fn set_marker(&mut self, color: Color, message: &str) {
        if let Some(loader) = self._context.debug_loader.as_ref() {
            // Create our null terminated string in the encoder's arena
            let mut name = BumpVec::with_capacity_in(message.len() + 1, &self.arena);
            name.extend_from_slice(message.as_bytes());
            name.push(0);
            let name_cstr = CStr::from_bytes_with_nul_unchecked(name.as_slice());

            let color: [f32; 4] = color.into();
            let info = vk::DebugUtilsLabelEXT::builder()
                .label_name(name_cstr)
                .color(color);
            loader.cmd_insert_debug_utils_label(self._buffer, info.deref())
        }
    }

    unsafe fn begin_event(&mut self, color: Color, message: &str) {
        if let Some(loader) = self._context.debug_loader.as_ref() {
            let mut name = BumpVec::with_capacity_in(message.len() + 1, &self.arena);
            name.extend_from_slice(message.as_bytes());
            name.push(0);
            let name_cstr = CStr::from_bytes_with_nul_unchecked(name.as_slice());

            let color: [f32; 4] = color.into();
            let info = vk::DebugUtilsLabelEXT::builder()
                .label_name(name_cstr)
                .color(color);
            loader.cmd_begin_debug_utils_label(self._buffer, info.deref())
        }
    }

    unsafe fn end_event(&mut self) {
        if let Some(loader) = self._context.debug_loader.as_ref() {
            loader.cmd_end_debug_utils_label(self._buffer)
        }
    }
}

impl<'a> Encoder<'a> {
    unsafe fn resource_barrier_fallback(
        &mut self,
        global_barriers: &[GlobalBarrier],
        buffer_barriers: &[BufferBarrier],
        texture_barriers: &[TextureBarrier],
    ) {
        #![allow(non_snake_case)]

        let mut translated_global_barriers =
            BumpVec::with_capacity_in(global_barriers.len(), &self.arena);
        let mut translated_buffer_barriers =
            BumpVec::with_capacity_in(buffer_barriers.len(), &self.arena);
        let mut translated_texture_barriers =
            BumpVec::with_capacity_in(texture_barriers.len(), &self.arena);
        let mut src_stage_mask = vk::PipelineStageFlags::default();
        let mut dst_stage_mask = vk::PipelineStageFlags::default();

        for barrier in global_barriers {
            src_stage_mask |=
                barrier_sync_to_vk(barrier.before_sync, &self.enabled_shader_features);
            dst_stage_mask |= barrier_sync_to_vk(barrier.after_sync, &self.enabled_shader_features);
            translated_global_barriers.push(
                vk::MemoryBarrier::builder()
                    .src_access_mask(barrier_access_to_vk(barrier.before_access))
                    .dst_access_mask(barrier_access_to_vk(barrier.after_access))
                    .build(),
            );
        }

        for barrier in buffer_barriers {
            let buffer = unwrap::buffer(barrier.buffer);

            let (src_family, dst_family) = if let Some(transition) = barrier.queue_transition {
                let src_family = self
                    ._device
                    .get_queue_family_index(transition.before_queue);
                let dst_family = self
                    ._device
                    .get_queue_family_index(transition.after_queue);
                (src_family, dst_family)
            } else {
                (0, 0)
            };

            src_stage_mask |=
                barrier_sync_to_vk(barrier.before_sync, &self.enabled_shader_features);
            dst_stage_mask |= barrier_sync_to_vk(barrier.after_sync, &self.enabled_shader_features);
            translated_buffer_barriers.push(
                vk::BufferMemoryBarrier::builder()
                    .src_access_mask(barrier_access_to_vk(barrier.before_access))
                    .dst_access_mask(barrier_access_to_vk(barrier.after_access))
                    .buffer(buffer.buffer)
                    .offset(barrier.offset)
                    .size(barrier.size)
                    .src_queue_family_index(src_family)
                    .dst_queue_family_index(dst_family)
                    .build(),
            );
        }

        for barrier in texture_barriers {
            // Grab the d3d12 resource handle from our texture impls
            let texture = unwrap::texture(barrier.texture);

            let (src_family, dst_family) = if let Some(transition) = barrier.queue_transition {
                let src_family = self
                    ._device
                    .get_queue_family_index(transition.before_queue);
                let dst_family = self
                    ._device
                    .get_queue_family_index(transition.after_queue);
                (src_family, dst_family)
            } else {
                (0, 0)
            };

            src_stage_mask |=
                barrier_sync_to_vk(barrier.before_sync, &self.enabled_shader_features);
            dst_stage_mask |= barrier_sync_to_vk(barrier.after_sync, &self.enabled_shader_features);
            translated_texture_barriers.push(
                vk::ImageMemoryBarrier::builder()
                    .src_access_mask(barrier_access_to_vk(barrier.before_access))
                    .dst_access_mask(barrier_access_to_vk(barrier.after_access))
                    .old_layout(image_layout_to_vk(barrier.before_layout))
                    .new_layout(image_layout_to_vk(barrier.after_layout))
                    .image(texture.image)
                    .subresource_range(subresource_range_to_vk(&barrier.subresource_range))
                    .src_queue_family_index(src_family)
                    .dst_queue_family_index(dst_family)
                    .build(),
            );
        }

        self._device.device.cmd_pipeline_barrier(
            self._buffer,
            src_stage_mask,
            dst_stage_mask,
            vk::DependencyFlags::default(),
            &translated_global_barriers,
            &translated_buffer_barriers,
            &translated_texture_barriers,
        )
    }

    unsafe fn resource_barrier_sync2(
        &mut self,
        loader: &Synchronization2,
        global_barriers: &[GlobalBarrier],
        buffer_barriers: &[BufferBarrier],
        texture_barriers: &[TextureBarrier],
    ) {
        #![allow(non_snake_case)]

        let mut translated_global_barriers =
            BumpVec::with_capacity_in(global_barriers.len(), &self.arena);
        let mut translated_buffer_barriers =
            BumpVec::with_capacity_in(buffer_barriers.len(), &self.arena);
        let mut translated_texture_barriers =
            BumpVec::with_capacity_in(texture_barriers.len(), &self.arena);

        for barrier in global_barriers {
            translated_global_barriers.push(
                vk::MemoryBarrier2::builder()
                    .src_stage_mask(barrier_sync_to_vk2(barrier.before_sync))
                    .dst_stage_mask(barrier_sync_to_vk2(barrier.after_sync))
                    .src_access_mask(barrier_access_to_vk2(barrier.before_access))
                    .dst_access_mask(barrier_access_to_vk2(barrier.after_access))
                    .build(),
            );
        }

        for barrier in buffer_barriers {
            let buffer = unwrap::buffer(barrier.buffer);

            let (src_family, dst_family) = if let Some(transition) = barrier.queue_transition {
                let src_family = self
                    ._device
                    .get_queue_family_index(transition.before_queue);
                let dst_family = self
                    ._device
                    .get_queue_family_index(transition.after_queue);
                (src_family, dst_family)
            } else {
                (0, 0)
            };

            translated_buffer_barriers.push(
                vk::BufferMemoryBarrier2::builder()
                    .src_stage_mask(barrier_sync_to_vk2(barrier.before_sync))
                    .dst_stage_mask(barrier_sync_to_vk2(barrier.after_sync))
                    .src_access_mask(barrier_access_to_vk2(barrier.before_access))
                    .dst_access_mask(barrier_access_to_vk2(barrier.after_access))
                    .buffer(buffer.buffer)
                    .offset(barrier.offset)
                    .size(barrier.size)
                    .src_queue_family_index(src_family)
                    .dst_queue_family_index(dst_family)
                    .build(),
            );
        }

        for barrier in texture_barriers {
            // Grab the d3d12 resource handle from our texture impls
            let texture = unwrap::texture(barrier.texture);

            let (src_family, dst_family) = if let Some(transition) = barrier.queue_transition {
                let src_family = self
                    ._device
                    .get_queue_family_index(transition.before_queue);
                let dst_family = self
                    ._device
                    .get_queue_family_index(transition.after_queue);
                (src_family, dst_family)
            } else {
                (0, 0)
            };

            translated_texture_barriers.push(
                vk::ImageMemoryBarrier2::builder()
                    .src_stage_mask(barrier_sync_to_vk2(barrier.before_sync))
                    .dst_stage_mask(barrier_sync_to_vk2(barrier.after_sync))
                    .src_access_mask(barrier_access_to_vk2(barrier.before_access))
                    .dst_access_mask(barrier_access_to_vk2(barrier.after_access))
                    .old_layout(image_layout_to_vk(barrier.before_layout))
                    .new_layout(image_layout_to_vk(barrier.after_layout))
                    .image(texture.image)
                    .subresource_range(subresource_range_to_vk(&barrier.subresource_range))
                    .src_queue_family_index(src_family)
                    .dst_queue_family_index(dst_family)
                    .build(),
            );
        }

        let info = vk::DependencyInfo::builder()
            .memory_barriers(&translated_global_barriers)
            .buffer_memory_barriers(&translated_buffer_barriers)
            .image_memory_barriers(&translated_texture_barriers);
        loader.cmd_pipeline_barrier2(self._buffer, &info)
    }
}
