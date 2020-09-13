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

use aleph_vulkan_alloc::{
    AllocationCreateFlag, AllocationCreateInfoBuilder, Allocator, MemoryUsage,
};

use aleph_imgui::DrawCmd;
use std::sync::Arc;

mod font;
mod frame;
mod global;
mod singular;

use aleph_vulkan_core::erupt::vk1_0::{
    Buffer, BufferCreateInfoBuilder, BufferUsageFlags, CommandBuffer, DependencyFlags, Extent2D,
    Extent2DBuilder, ImageAspectFlags, ImageLayout, ImageMemoryBarrierBuilder,
    ImageSubresourceRangeBuilder, IndexType, Offset2D, PipelineBindPoint, PipelineStageFlags,
    Rect2DBuilder, RenderPassBeginInfoBuilder, ShaderStageFlags, SharingMode, SubpassContents,
    Vk10DeviceLoaderExt, WHOLE_SIZE,
};
use aleph_vulkan_core::DebugName;
pub use font::ImguiFont;
pub use frame::ImguiFrame;
pub use global::ImguiGlobal;
pub use singular::ImguiSingular;

pub struct ImguiRenderer {
    device: Arc<aleph_vulkan_core::Device>,
    allocator: Arc<Allocator>,
    frames: Vec<ImguiFrame>,
    single: ImguiSingular,
    font: ImguiFont,
    global: ImguiGlobal,
}

impl ImguiRenderer {
    pub fn new(
        fonts: aleph_imgui::FontAtlasRefMut,
        device: Arc<aleph_vulkan_core::Device>,
        allocator: Arc<Allocator>,
        swapchain: &aleph_vulkan_core::Swapchain,
    ) -> Self {
        aleph_log::trace!("Initializing ImGui Renderer");
        let global = ImguiGlobal::init(&device);

        let font = ImguiFont::init(fonts, &global, &device, &allocator);

        let single = ImguiSingular::init(&device, &global, &swapchain.images()[0]);

        let frames = (0..swapchain.images().len())
            .into_iter()
            .map(|index| {
                ImguiFrame::init(&device, &allocator, swapchain, index, single.render_pass)
            })
            .collect();

        ImguiRenderer {
            device,
            allocator,
            frames,
            single,
            font,
            global,
        }
    }

    pub unsafe fn recreate_resources(&mut self, swapchain: &aleph_vulkan_core::Swapchain) {
        for frame in self.frames.iter() {
            frame.destroy(&self.device, &self.allocator);
        }
        self.single.destroy(&self.device);

        self.single = ImguiSingular::init(&self.device, &self.global, &swapchain.images()[0]);

        self.frames = (0..swapchain.images().len())
            .into_iter()
            .map(|index| {
                ImguiFrame::init(
                    &self.device,
                    &self.allocator,
                    &swapchain,
                    index,
                    self.single.render_pass,
                )
            })
            .collect();
    }

    pub unsafe fn render_frame(
        &mut self,
        frame: aleph_imgui::Ui,
        command_buffer: CommandBuffer,
        index: usize,
    ) {
        optick::event!();

        let draw_data = frame.render();

        //
        // Free the vertex buffer if one already exists
        //
        if self.frames[index].vtx_buffer.0 != Buffer::null() {
            self.allocator.destroy_buffer(
                self.frames[index].vtx_buffer.0,
                self.frames[index].vtx_buffer.1,
            );
        }

        //
        // Free the index buffer if one already exists
        //
        if self.frames[index].idx_buffer.0 != Buffer::null() {
            self.allocator.destroy_buffer(
                self.frames[index].idx_buffer.0,
                self.frames[index].idx_buffer.1,
            );
        }

        //
        // Make sure there is actually some vertex data to upload
        //
        if draw_data.total_vtx_count != 0 {
            //
            // Allocate a vertex buffer that will only be used for one frame
            //
            let buffer_create_info = BufferCreateInfoBuilder::new()
                .usage(BufferUsageFlags::VERTEX_BUFFER)
                .sharing_mode(SharingMode::EXCLUSIVE)
                .size(
                    draw_data.total_vtx_count as u64
                        * core::mem::size_of::<aleph_imgui::DrawVert>() as u64,
                );
            let alloc_create_info = AllocationCreateInfoBuilder::new()
                .usage(MemoryUsage::CPUToGPU)
                .pool(&self.frames[index].memory_pool)
                .build();
            self.frames[index].vtx_buffer = self
                .allocator
                .create_buffer(&buffer_create_info, &alloc_create_info)
                .expect("Failed to allocate per frame vertex buffer");

            self.frames[index].vtx_buffer.0.add_debug_name(
                &self.device,
                aleph_macros::cstr!(concat!(module_path!(), "::VertexBuffer")),
            );

            //
            // Allocate an index buffer that will only be used for one frame
            //
            let buffer_create_info = BufferCreateInfoBuilder::new()
                .usage(BufferUsageFlags::INDEX_BUFFER)
                .sharing_mode(SharingMode::EXCLUSIVE)
                .size(
                    draw_data.total_idx_count as u64
                        * core::mem::size_of::<aleph_imgui::DrawIdx>() as u64,
                );
            let allocation_create_info = AllocationCreateInfoBuilder::new()
                .flags(AllocationCreateFlag::UPPER_ADDRESS_BIT)
                .usage(MemoryUsage::CPUToGPU)
                .pool(&self.frames[index].memory_pool)
                .build();
            self.frames[index].idx_buffer = self
                .allocator
                .create_buffer(&buffer_create_info, &allocation_create_info)
                .expect("Failed to allocate per frame index buffer");

            self.frames[index].idx_buffer.0.add_debug_name(
                &self.device,
                aleph_macros::cstr!(concat!(module_path!(), "::IndexBuffer")),
            );

            let vtx_buffer = self.frames[index].vtx_buffer;
            let idx_buffer = self.frames[index].idx_buffer;

            //
            // Map the vertex and index buffers
            //
            let mut vptr = self
                .allocator
                .map_memory(&vtx_buffer.1)
                .expect("Failed to map vertex buffer");
            let mut iptr = self
                .allocator
                .map_memory(&idx_buffer.1)
                .expect("Failed to map index buffer");

            //
            // Copy vertex and index buffer data into the vulkan buffers
            //
            draw_data.draw_lists().for_each(|list| {
                let vslice = list.vtx_buffer();
                vptr.copy_from(
                    vslice.as_ptr() as *const _,
                    vslice.len() * core::mem::size_of::<aleph_imgui::DrawVert>(),
                );
                vptr = vptr.add(vslice.len() * core::mem::size_of::<aleph_imgui::DrawVert>());

                let islice = list.idx_buffer();
                iptr.copy_from(
                    islice.as_ptr() as *const _,
                    islice.len() * core::mem::size_of::<aleph_imgui::DrawIdx>(),
                );
                iptr = iptr.add(islice.len() * core::mem::size_of::<aleph_imgui::DrawIdx>());
            });

            //
            // Flush and unmap the vertex and index buffers
            //
            self.allocator
                .flush_allocation(&vtx_buffer.1, 0, WHOLE_SIZE);
            self.allocator
                .flush_allocation(&idx_buffer.1, 0, WHOLE_SIZE);
            self.allocator.unmap_memory(&vtx_buffer.1);
            self.allocator.unmap_memory(&idx_buffer.1);
        }

        let vtx_buffer = self.frames[index].vtx_buffer;
        let idx_buffer = self.frames[index].idx_buffer;

        //
        // We need to special case for when imgui wants to render nothing as vertex buffers wont
        // exist so we can't bind them. We still need to transition the image though so instead we
        // just insert a pipeline barrier to do the transition.
        //
        if draw_data.total_vtx_count != 0 {
            //
            // Begin the render pass
            //
            let extent = Extent2DBuilder::new()
                .width(self.frames[index].swap_image.width())
                .height(self.frames[index].swap_image.height());
            let render_area = Rect2DBuilder::new().extent(*extent);
            let render_pass_begin = RenderPassBeginInfoBuilder::new()
                .render_pass(self.single.render_pass)
                .framebuffer(self.frames[index].framebuffer)
                .clear_values(&[])
                .render_area(*render_area);
            self.device.loader().cmd_begin_render_pass(
                command_buffer,
                &render_pass_begin,
                SubpassContents::INLINE,
            );

            self.reset_render_state(index, command_buffer, vtx_buffer.0, idx_buffer.0, draw_data);

            let clip_off = draw_data.display_pos;
            let clip_scale = draw_data.framebuffer_scale;

            let mut vtx_offset = 0;
            let mut idx_offset = 0;
            draw_data.draw_lists().for_each(|list| {
                list.commands().for_each(|command| {
                    self.render_draw_command(
                        draw_data,
                        index,
                        vtx_buffer.0,
                        idx_buffer.0,
                        command_buffer,
                        &clip_off,
                        &clip_scale,
                        vtx_offset,
                        idx_offset,
                        command,
                    )
                });
                vtx_offset += list.vtx_buffer().len() as i32;
                idx_offset += list.idx_buffer().len() as u32;
            });

            self.device.loader().cmd_end_render_pass(command_buffer);
        } else {
            let range = ImageSubresourceRangeBuilder::new()
                .layer_count(1)
                .level_count(1)
                .base_array_layer(0)
                .base_mip_level(0)
                .aspect_mask(ImageAspectFlags::COLOR)
                .discard();
            let image = ImageMemoryBarrierBuilder::new()
                .image(self.frames[index].swap_image.image())
                .old_layout(ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
                .new_layout(ImageLayout::PRESENT_SRC_KHR)
                .subresource_range(range);
            self.device.loader().cmd_pipeline_barrier(
                command_buffer,
                PipelineStageFlags::BOTTOM_OF_PIPE,
                PipelineStageFlags::TOP_OF_PIPE,
                DependencyFlags::default(),
                &[],
                &[],
                &[image],
            );
        }
    }

    unsafe fn render_draw_command(
        &mut self,
        draw_data: &aleph_imgui::DrawData,
        index: usize,
        vertex_buffer: Buffer,
        index_buffer: Buffer,
        command_buffer: CommandBuffer,
        clip_off: &[f32; 2],
        clip_scale: &[f32; 2],
        vtx_offset: i32,
        idx_offset: u32,
        command: DrawCmd,
    ) {
        match command {
            DrawCmd::Elements { cmd_params, count } => {
                let clip_rect = [
                    (cmd_params.clip_rect[0] - clip_off[0]) * clip_scale[0],
                    (cmd_params.clip_rect[1] - clip_off[1]) * clip_scale[1],
                    (cmd_params.clip_rect[2] - clip_off[0]) * clip_scale[0],
                    (cmd_params.clip_rect[3] - clip_off[1]) * clip_scale[1],
                ];

                let swap_image = &self.frames[index].swap_image;
                let swap_extent = Extent2DBuilder::new()
                    .width(swap_image.width())
                    .height(swap_image.height());
                if clip_rect[0] < swap_extent.width as f32
                    && clip_rect[1] < swap_extent.height as f32
                    && clip_rect[2] > 0.0
                    && clip_rect[3] > 0.0
                {
                    let offset = Offset2D {
                        x: (clip_rect[0] as i32).max(0),
                        y: (clip_rect[1] as i32).max(0),
                    };
                    let extent = Extent2D {
                        width: (clip_rect[2] - clip_rect[0]) as u32,
                        height: (clip_rect[3] - clip_rect[1]) as u32,
                    };
                    let scissor = Rect2DBuilder::new().offset(offset).extent(extent);
                    self.device
                        .loader()
                        .cmd_set_scissor(command_buffer, 0, &[scissor]);

                    self.device.loader().cmd_draw_indexed(
                        command_buffer,
                        count as _,
                        1,
                        cmd_params.idx_offset as u32 + idx_offset,
                        cmd_params.vtx_offset as i32 + vtx_offset,
                        0,
                    )
                }
            }
            DrawCmd::ResetRenderState => {
                self.reset_render_state(
                    index,
                    command_buffer,
                    vertex_buffer,
                    index_buffer,
                    draw_data,
                );
            }
            DrawCmd::RawCallback { .. } => {
                unimplemented!("Cant use callback because bindings make it hard");
            }
        }
    }

    unsafe fn reset_render_state(
        &self,
        index: usize,
        command_buffer: CommandBuffer,
        vertex_buffer: Buffer,
        index_buffer: Buffer,
        draw_data: &aleph_imgui::DrawData,
    ) {
        //
        // Bind the pipeline and descriptor set that we'll be rendering with
        //
        self.device.loader().cmd_bind_pipeline(
            command_buffer,
            PipelineBindPoint::GRAPHICS,
            self.single.pipeline,
        );
        self.device.loader().cmd_bind_descriptor_sets(
            command_buffer,
            PipelineBindPoint::GRAPHICS,
            self.global.pipeline_layout.pipeline_layout(),
            0,
            &[self.global.descriptor_set],
            &[],
        );

        //
        // Bind the vertex and index buffers to render with
        //
        self.device
            .loader()
            .cmd_bind_vertex_buffers(command_buffer, 0, &[vertex_buffer], &[0]);
        self.device.loader().cmd_bind_index_buffer(
            command_buffer,
            index_buffer,
            0,
            IndexType::UINT16,
        );

        //
        // Set the viewport state, we're going to be rendering to the whole frame
        //
        self.device.loader().cmd_set_viewport(
            command_buffer,
            0,
            &[self.frames[index].swap_image.get_viewport_full()],
        );

        //
        // Push transforms via push constants
        //
        let scale = [
            2.0 / draw_data.display_size[0],
            2.0 / draw_data.display_size[1],
        ];
        self.device.loader().cmd_push_constants(
            command_buffer,
            self.global.pipeline_layout.pipeline_layout(),
            ShaderStageFlags::VERTEX,
            0,
            8,
            scale.as_ptr() as *const _,
        );
        let translate = [
            -1.0 - draw_data.display_pos[0] * scale[0],
            -1.0 - draw_data.display_pos[1] * scale[1],
        ];
        self.device.loader().cmd_push_constants(
            command_buffer,
            self.global.pipeline_layout.pipeline_layout(),
            ShaderStageFlags::VERTEX,
            8,
            8,
            translate.as_ptr() as *const _,
        );
    }
}

impl Drop for ImguiRenderer {
    fn drop(&mut self) {
        unsafe {
            for frame in self.frames.iter() {
                frame.destroy(&self.device, &self.allocator)
            }
            self.single.destroy(&self.device);
            self.font.destroy(&self.device, &self.allocator);
            self.global.destroy(&self.device);
        }
    }
}
