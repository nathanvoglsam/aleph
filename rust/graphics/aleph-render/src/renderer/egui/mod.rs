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

mod constant;
mod frame;
mod global;

use crate::renderer::egui::constant::ConstantObjects;
use crate::renderer::egui::frame::PerFrameObjects;
use crate::renderer::egui::global::GlobalObjects;
use aleph_vulkan_alloc::Allocator;
use aleph_vulkan_core::erupt::vk1_0::{
    CommandBuffer, Extent2DBuilder, IndexType, Offset2DBuilder, PipelineBindPoint, Rect2DBuilder,
    RenderPassBeginInfoBuilder, ShaderStageFlags, SubpassContents, WHOLE_SIZE,
};
use egui::paint::PaintJob;
use std::sync::Arc;

pub struct Renderer {
    /// Rendering device
    device: Arc<aleph_vulkan_core::Device>,

    /// Memory allocator
    allocator: Arc<Allocator>,

    /// Vulkan objects that need to be allocated for each in flight frame
    frames: Vec<PerFrameObjects>,

    /// Objects that are shared between each frame during execution
    global: GlobalObjects,

    /// Objects that are shared between each frame, and do not depend on the swap chain
    constant: ConstantObjects,

    ///
    pixels_per_point: f32,
}

impl Renderer {
    pub fn new(
        device: Arc<aleph_vulkan_core::Device>,
        allocator: Arc<Allocator>,
        swapchain: &aleph_vulkan_core::Swapchain,
    ) -> Self {
        aleph_log::trace!("Initializing Egui Renderer");

        let constant = ConstantObjects::init(&device);

        let global = GlobalObjects::init(&device, &constant, &swapchain.images()[0]);

        let frames = (0..swapchain.images().len())
            .into_iter()
            .map(|index| {
                PerFrameObjects::init(
                    &device,
                    &allocator,
                    &constant,
                    swapchain,
                    index,
                    global.render_pass,
                )
            })
            .collect();

        Self {
            device,
            allocator,
            frames,
            global,
            constant,
            pixels_per_point: 1.0,
        }
    }

    pub fn update_screen_info(&mut self, pixels_per_point: f32) {
        self.pixels_per_point = pixels_per_point;
    }

    pub unsafe fn render_frame(
        &mut self,
        index: usize,
        command_buffer: CommandBuffer,
        egui_ctx: &::egui::CtxRef,
        jobs: egui::PaintJobs,
    ) {
        optick::event!();

        self.frames[index].font_objects.upload_texture(
            &self.device,
            &self.allocator,
            command_buffer,
            &self.global,
            &egui_ctx.fonts().texture(),
        );

        // Map the buffers for copying into them
        let (mut v_ptr, v_ptr_end, mut i_ptr, i_ptr_end) = self.map_buffers(index);

        // Begin the render pass and bind our resources
        self.begin_render_pass(index, command_buffer);
        self.bind_resources(index, command_buffer);

        let mut vtx_base = 0;
        let mut idx_base = 0;
        for job in jobs {
            let triangles = &job.1;

            // Skip doing anything for the job if there's nothing to render
            if triangles.vertices.is_empty() || triangles.indices.is_empty() {
                continue;
            }

            // Get the slice to copy from and various byte counts
            let v_slice = &triangles.vertices;
            let v_size = core::mem::size_of_val(&v_slice[0]);
            let v_copy_size = v_slice.len() * v_size;

            // Get the slice to copy from and various byte counts
            let i_slice = &triangles.indices;
            let i_size = core::mem::size_of_val(&i_slice[0]);
            let i_copy_size = i_slice.len() * i_size;

            // Calculate where the pointers will be after writing the current set of data
            let v_ptr_next = v_ptr.add(v_copy_size);
            let i_ptr_next = i_ptr.add(i_copy_size);

            // Check if we're going to over-run the buffers, and panic if we will
            if v_ptr_next >= v_ptr_end || i_ptr_next >= i_ptr_end {
                panic!("Out of memory");
            }

            // Perform the actual copies
            v_ptr.copy_from(v_slice.as_ptr() as *const _, v_copy_size);
            i_ptr.copy_from(i_slice.as_ptr() as *const _, i_copy_size);

            // Setup the pointers for the next iteration
            v_ptr = v_ptr_next;
            i_ptr = i_ptr_next;

            self.record_job_commands(index, command_buffer, &job, vtx_base, idx_base);

            vtx_base += triangles.vertices.len();
            idx_base += triangles.indices.len();
        }

        // End the render pass
        self.end_render_pass(command_buffer);

        // Unmap the buffers
        self.unmap_buffers(index);
    }

    unsafe fn record_job_commands(
        &mut self,
        index: usize,
        command_buffer: CommandBuffer,
        job: &PaintJob,
        vtx_base: usize,
        idx_base: usize,
    ) {
        let triangles = &job.1;

        self.device.loader().cmd_set_scissor(
            command_buffer,
            0,
            &[self.calculate_clip_rect(index, job)],
        );
        self.device.loader().cmd_draw_indexed(
            command_buffer,
            triangles.indices.len() as _,
            1,
            idx_base as _,
            vtx_base as _,
            0,
        );
    }

    fn calculate_clip_rect(&self, index: usize, job: &PaintJob) -> Rect2DBuilder {
        let width_pixels = self.frames[index].swap_image.width() as f32;
        let height_pixels = self.frames[index].swap_image.height() as f32;

        // Calculate clip offset
        let min = job.0.min;
        let min = egui::Pos2 {
            x: min.x * self.pixels_per_point,
            y: min.y * self.pixels_per_point,
        };
        let min = egui::Pos2 {
            x: egui::math::clamp(min.x, 0.0..=width_pixels),
            y: egui::math::clamp(min.y, 0.0..=height_pixels),
        };
        let offset = Offset2DBuilder::new()
            .x(min.x.round() as _)
            .y(min.y.round() as _)
            .build();

        // Calculate clip extent
        let max = job.0.max;
        let max = egui::Pos2 {
            x: max.x * self.pixels_per_point,
            y: max.y * self.pixels_per_point,
        };
        let max = egui::Pos2 {
            x: egui::math::clamp(max.x, min.x..=width_pixels),
            y: egui::math::clamp(max.y, min.y..=height_pixels),
        };
        let extent = egui::Pos2 {
            x: max.x.round() - min.x,
            y: max.y.round() - min.y,
        };

        let extent = Extent2DBuilder::new()
            .width(extent.x as _)
            .height(extent.y as _)
            .build();
        Rect2DBuilder::new().offset(offset).extent(extent)
    }

    unsafe fn map_buffers(&self, index: usize) -> (*mut u8, *mut u8, *mut u8, *mut u8) {
        //
        // Map the vertex and index buffers
        //
        let v_ptr = self
            .allocator
            .map_memory(&self.frames[index].vtx_buffer.1)
            .expect("Failed to map vertex buffer");
        let v_ptr_end = v_ptr.add(PerFrameObjects::vertex_buffer_size());

        let i_ptr = self
            .allocator
            .map_memory(&self.frames[index].idx_buffer.1)
            .expect("Failed to map index buffer");
        let i_ptr_end = i_ptr.add(PerFrameObjects::index_buffer_size());

        (v_ptr, v_ptr_end, i_ptr, i_ptr_end)
    }

    unsafe fn unmap_buffers(&self, index: usize) {
        //
        // Flush and unmap the vertex and index buffers
        //
        let vtx_buffer = &self.frames[index].vtx_buffer;
        let idx_buffer = &self.frames[index].idx_buffer;

        self.allocator
            .flush_allocation(&vtx_buffer.1, 0, WHOLE_SIZE);
        self.allocator
            .flush_allocation(&idx_buffer.1, 0, WHOLE_SIZE);

        self.allocator.unmap_memory(&vtx_buffer.1);
        self.allocator.unmap_memory(&idx_buffer.1);
    }

    unsafe fn begin_render_pass(&self, index: usize, command_buffer: CommandBuffer) {
        //
        // Begin the render pass
        //
        let extent = Extent2DBuilder::new()
            .width(self.frames[index].swap_image.width())
            .height(self.frames[index].swap_image.height());
        let render_area = Rect2DBuilder::new().extent(*extent);
        let render_pass_begin = RenderPassBeginInfoBuilder::new()
            .render_pass(self.global.render_pass)
            .framebuffer(self.frames[index].framebuffer)
            .clear_values(&[])
            .render_area(*render_area);
        self.device.loader().cmd_begin_render_pass(
            command_buffer,
            &render_pass_begin,
            SubpassContents::INLINE,
        );
    }

    unsafe fn end_render_pass(&self, command_buffer: CommandBuffer) {
        //
        // End the render pass
        //
        self.device.loader().cmd_end_render_pass(command_buffer);
    }

    unsafe fn bind_resources(&self, index: usize, command_buffer: CommandBuffer) {
        //
        // Bind the pipeline and descriptor set that we'll be rendering with
        //
        self.device.loader().cmd_bind_pipeline(
            command_buffer,
            PipelineBindPoint::GRAPHICS,
            self.global.pipeline,
        );
        self.device.loader().cmd_bind_descriptor_sets(
            command_buffer,
            PipelineBindPoint::GRAPHICS,
            self.constant.pipeline_layout.pipeline_layout(),
            0,
            &[self.frames[index].font_objects.descriptor_set],
            &[],
        );

        //
        // Bind the vertex and index buffers to render with
        //
        let buffers = [self.frames[index].vtx_buffer.0];
        self.device
            .loader()
            .cmd_bind_vertex_buffers(command_buffer, 0, &buffers, &[0]);
        let index_buffer = self.frames[index].idx_buffer.0;
        self.device.loader().cmd_bind_index_buffer(
            command_buffer,
            index_buffer,
            0,
            IndexType::UINT32,
        );

        //
        // Set the viewport state, we're going to be rendering to the whole frame
        //
        let viewports = [self.frames[index].swap_image.get_viewport_full()];
        self.device
            .loader()
            .cmd_set_viewport(command_buffer, 0, &viewports);

        //
        // Push screen size via push constants
        //
        let width_pixels = self.frames[index].swap_image.width() as f32;
        let height_pixels = self.frames[index].swap_image.height() as f32;
        let width_points = width_pixels / self.pixels_per_point;
        let height_points = height_pixels / self.pixels_per_point;
        let screen_size = [width_points, height_points];
        let size = std::mem::size_of_val(&screen_size[0]);
        let size = screen_size.len() * size;
        self.device.loader().cmd_push_constants(
            command_buffer,
            self.constant.pipeline_layout.pipeline_layout(),
            ShaderStageFlags::VERTEX,
            0,
            size as u32,
            screen_size.as_ptr() as *const _,
        );
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        unsafe {
            for frame in self.frames.iter_mut() {
                frame.destroy(&self.device, &self.allocator)
            }
            self.global.destroy(&self.device);
            self.constant.destroy(&self.device);
        }
    }
}
