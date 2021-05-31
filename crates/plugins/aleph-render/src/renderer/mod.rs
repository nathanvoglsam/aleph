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

mod frame;
mod global;
mod swap;

use dx12::dxgi;
pub(crate) use frame::PerFrameObjects;
pub(crate) use global::GlobalObjects;
use std::mem::transmute;
pub(crate) use swap::SwapDependentObjects;

pub struct EguiRenderer {
    frames: Vec<PerFrameObjects>,
    swap_dependent: Vec<SwapDependentObjects>,
    global: GlobalObjects,

    /// Rendering device
    device: dx12::Device,

    /// Memory allocator
    allocator: dx12_alloc::Allocator,

    ///
    pixels_per_point: f32,
}

impl EguiRenderer {
    pub fn new(
        device: dx12::Device,
        allocator: dx12_alloc::Allocator,
        buffers: &[dx12::Resource],
        swap_width: u32,
        swap_height: u32,
    ) -> Self {
        aleph_log::trace!("Initializing Egui Renderer");

        let global = GlobalObjects::new(&device, swap_width, swap_height);

        let frames = (0..3)
            .into_iter()
            .map(|index| PerFrameObjects::new(&device, &allocator, &global, index))
            .collect();

        let swap_dependent = (0..3)
            .into_iter()
            .map(|index| SwapDependentObjects::new(&device, &global, buffers, index))
            .collect();

        Self {
            device,
            allocator,
            frames,
            swap_dependent,
            global,
            pixels_per_point: 1.0,
        }
    }

    pub fn update_screen_info(&mut self, pixels_per_point: f32) {
        self.pixels_per_point = pixels_per_point;
    }

    pub unsafe fn recreate_swap_resources(
        &mut self,
        device: &dx12::Device,
        buffers: &[dx12::Resource],
        new_dimensions: (u32, u32),
    ) {
        self.global.swap_width = new_dimensions.0;
        self.global.swap_height = new_dimensions.1;
        let swap_dependent = (0..3)
            .into_iter()
            .map(|index| SwapDependentObjects::new(device, &self.global, buffers, index))
            .collect();
        self.swap_dependent = swap_dependent;
    }

    pub unsafe fn record_frame(
        &mut self,
        index: usize,
        command_list: &mut dx12::GraphicsCommandList,
        buffers: &[dx12::Resource],
        egui_ctx: &::egui::CtxRef,
        jobs: Vec<aleph_egui::ClippedMesh>,
    ) {
        // Clear the command allocator
        &self.frames[index].command_allocator.reset().unwrap();

        // Begin recording commands into the command list
        command_list
            .reset(
                &self.frames[index].command_allocator,
                &self.global.pipeline_state,
            )
            .unwrap();

        // Handles creating the texture data resources and placing it into the staging buffer if
        // doing so is needed. Will return whether or not the texture data needs to be re-staged.
        let needs_reupload = self.frames[index].update_texture_data(
            &self.device,
            &self.allocator,
            &egui_ctx.fonts().texture(),
        );

        // If a reupload is needed we record into the command buffer the commands required to do so
        if needs_reupload {
            self.frames[index].record_texture_upload(command_list);
        }

        // Map the buffers for copying into them
        let (mut v_ptr, v_ptr_end, mut i_ptr, i_ptr_end) = self.map_buffers(index);

        // Begin the render pass and bind our resources
        self.bind_resources(index, command_list);

        // Transition from present to render target state
        let barrier = dx12::ResourceBarrier::Transition {
            flags: Default::default(),
            resource: Some(buffers[index].clone()),
            subresource: 0,
            state_before: dx12::ResourceStates::PRESENT,
            state_after: dx12::ResourceStates::RENDER_TARGET,
        };
        command_list.resource_barrier(&[barrier]);

        // Clear the render target
        command_list.clear_render_target_view(
            self.swap_dependent[index].rtv_cpu,
            &[0.0, 0.0, 0.0, 0.0],
            None,
        );

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

            self.record_job_commands(command_list, &job, vtx_base, idx_base);

            vtx_base += triangles.vertices.len();
            idx_base += triangles.indices.len();
        }

        let barrier = dx12::ResourceBarrier::Transition {
            flags: Default::default(),
            resource: Some(buffers[index].clone()),
            subresource: 0,
            state_before: dx12::ResourceStates::RENDER_TARGET,
            state_after: dx12::ResourceStates::PRESENT,
        };
        command_list.resource_barrier(&[barrier]);

        command_list.close().unwrap();

        // Unmap the buffers
        self.unmap_buffers(index);
    }

    unsafe fn record_job_commands(
        &mut self,
        command_list: &mut dx12::GraphicsCommandList,
        job: &aleph_egui::ClippedMesh,
        vtx_base: usize,
        idx_base: usize,
    ) {
        let triangles = &job.1;

        let scissor_rect = self.calculate_clip_rect(job);
        command_list.rs_set_scissor_rects(&[scissor_rect]);
        command_list.draw_indexed_instanced(
            triangles.indices.len() as _,
            1,
            idx_base as _,
            vtx_base as _,
            0,
        );
    }

    unsafe fn bind_resources(&self, index: usize, command_list: &mut dx12::GraphicsCommandList) {
        //
        // Bind the Root Signature
        //
        command_list.set_graphics_root_signature(&self.global.root_signature);

        //
        // Bind the descriptor heap
        //
        command_list.set_descriptor_heaps(&[self.global.srv_heap.clone()]);

        //
        // Bind the texture
        //
        command_list.set_graphics_root_descriptor_table(0, self.frames[index].font_gpu_srv);

        //
        // Push screen size via root constants
        //
        let width_pixels = self.global.swap_width as f32;
        let height_pixels = self.global.swap_height as f32;
        let width_points = width_pixels / self.pixels_per_point;
        let height_points = height_pixels / self.pixels_per_point;
        let values = [transmute(width_points), transmute(height_points)];
        command_list.set_graphics_root_32bit_constants(1, &values, 0);

        command_list.ia_set_primitive_topology(dx12::PrimitiveTopology::TriangleList);

        //
        // Bind the vertex and index buffers to render with
        //
        let buffer_location = self.frames[index]
            .vtx_buffer
            .get_resource()
            .unwrap()
            .get_gpu_virtual_address()
            .unwrap();
        command_list.ia_set_vertex_buffers(
            0,
            &[dx12::VertexBufferView {
                buffer_location,
                size_in_bytes: PerFrameObjects::vertex_buffer_size() as _,
                stride_in_bytes: (4 * 4) + 4,
            }],
        );
        let buffer_location = self.frames[index]
            .idx_buffer
            .get_resource()
            .unwrap()
            .get_gpu_virtual_address()
            .unwrap();
        command_list.ia_set_index_buffer(&dx12::IndexBufferView {
            buffer_location,
            size_in_bytes: PerFrameObjects::index_buffer_size() as _,
            format: dxgi::Format::R32Uint,
        });

        //
        // Bind the render target
        //
        command_list.om_set_render_targets(Some(&[self.swap_dependent[index].rtv_cpu]), None);

        //
        // Set the viewport state, we're going to be rendering to the whole frame
        //
        command_list.rs_set_viewports(&[dx12::Viewport {
            top_left_x: 0.0,
            top_left_y: 0.0,
            width: self.global.swap_width as _,
            height: self.global.swap_height as _,
            min_depth: 0.0,
            max_depth: 1.0,
        }]);
    }

    unsafe fn map_buffers(&self, index: usize) -> (*mut u8, *mut u8, *mut u8, *mut u8) {
        //
        // Map the vertex and index buffers
        //
        let v_ptr = self.frames[index]
            .vtx_buffer
            .get_resource()
            .unwrap()
            .map(0, Some(0..0))
            .unwrap()
            .unwrap()
            .as_ptr();
        let v_ptr_end = v_ptr.add(PerFrameObjects::vertex_buffer_size());

        let i_ptr = self.frames[index]
            .idx_buffer
            .get_resource()
            .unwrap()
            .map(0, Some(0..0))
            .unwrap()
            .unwrap()
            .as_ptr();
        let i_ptr_end = i_ptr.add(PerFrameObjects::index_buffer_size());

        (v_ptr, v_ptr_end, i_ptr, i_ptr_end)
    }

    unsafe fn unmap_buffers(&self, index: usize) {
        //
        // Flush and unmap the vertex and index buffers
        //
        let vtx_buffer = &self.frames[index].vtx_buffer.get_resource().unwrap();
        let idx_buffer = &self.frames[index].idx_buffer.get_resource().unwrap();

        vtx_buffer.unmap(0, None);
        idx_buffer.unmap(0, None);
    }

    fn calculate_clip_rect(&self, job: &aleph_egui::ClippedMesh) -> dx12::Rect {
        let width_pixels = self.global.swap_width as f32;
        let height_pixels = self.global.swap_height as f32;

        // Calculate clip offset
        let min = job.0.min;
        let min = egui::Pos2 {
            x: min.x * self.pixels_per_point,
            y: min.y * self.pixels_per_point,
        };
        let min = egui::Pos2 {
            x: min.x.clamp(0.0, width_pixels),
            y: min.y.clamp(0.0, height_pixels),
        };

        // Calculate clip extent
        let max = job.0.max;
        let max = egui::Pos2 {
            x: max.x * self.pixels_per_point,
            y: max.y * self.pixels_per_point,
        };
        let max = egui::Pos2 {
            x: max.x.clamp(min.x, width_pixels),
            y: max.y.clamp(min.y, height_pixels),
        };

        dx12::Rect {
            left: min.x.round() as _,
            top: min.y.round() as _,
            right: max.x.round() as _,
            bottom: max.y.round() as _,
        }
    }
}
