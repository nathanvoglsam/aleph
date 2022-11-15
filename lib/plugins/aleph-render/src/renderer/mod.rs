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

use aleph_gpu_dx12::windows::Win32::Graphics::Direct3D12::*;
use aleph_gpu_dx12::{ICommandListExt, IDeviceExt};
use egui::RenderData;
pub(crate) use frame::PerFrameObjects;
pub(crate) use global::GlobalObjects;
use interfaces::any::{AnyArc, QueryInterface, QueryInterfaceBox};
use interfaces::gpu::{
    AttachmentLoadOp, AttachmentStoreOp, BarrierAccess, BarrierSync, BeginRenderingInfo,
    ColorClearValue, ICommandList, IGeneralEncoder, ITexture, ImageLayout, IndexType,
    InputAssemblyBufferBinding, Rect, RenderingColorAttachmentInfo, TextureAspect, TextureBarrier,
    TextureSubResourceSet, Viewport,
};
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;

pub struct EguiRenderer {
    frames: Vec<PerFrameObjects>,
    global: GlobalObjects,

    /// Rendering device
    device: AnyArc<dyn IDeviceExt>,

    ///
    pixels_per_point: f32,
}

impl EguiRenderer {
    pub fn new(device: AnyArc<dyn IDeviceExt>, dimensions: (u32, u32)) -> Self {
        aleph_log::trace!("Initializing Egui Renderer");

        let global = GlobalObjects::new(device.deref(), dimensions);

        let frames = (0..3)
            .into_iter()
            .map(|index| PerFrameObjects::new(device.deref(), &global, index))
            .collect();

        Self {
            device,
            frames,
            global,
            pixels_per_point: 1.0,
        }
    }

    #[allow(unused)]
    pub fn update_screen_info(&mut self, pixels_per_point: f32) {
        self.pixels_per_point = pixels_per_point;
    }

    pub fn recreate_swap_resources(&mut self, new_dimensions: (u32, u32)) {
        self.global.swap_width = new_dimensions.0;
        self.global.swap_height = new_dimensions.1;
    }

    pub unsafe fn record_frame(
        &mut self,
        index: usize,
        texture: &dyn ITexture,
        render_data: RenderData,
    ) -> Box<dyn ICommandList> {
        // Begin recording commands into the command list
        let mut list = self.frames[index]
            .command_allocator
            .create_command_list()
            .unwrap();

        let command_list: ID3D12GraphicsCommandList7 = list
            .deref()
            .query_interface::<dyn ICommandListExt>()
            .unwrap()
            .get_raw_list();

        {
            let mut encoder = list.begin_general().unwrap();

            // If the font texture has changed then we need to update our copy and increment the
            // version to invalidate the per-frame font textures
            for (_, delta) in render_data.textures_delta.set {
                if let egui::epaint::ImageData::Font(_) = &delta.image {
                    self.global.update_font_texture(&delta);
                }
            }

            // If the versions do not match then we should re-upload the texture to the GPU
            if self.frames[index].font_version != self.global.font_texture.version {
                self.frames[index]
                    .update_texture_data(self.device.deref(), &self.global.font_texture);
                self.frames[index].record_texture_upload(encoder.deref_mut());
            }

            // Map the buffers for copying into them
            let (v_ptr, v_ptr_end, i_ptr, i_ptr_end) = self.map_buffers(index);
            let (mut v_ptr, v_ptr_end, mut i_ptr, i_ptr_end) = (
                v_ptr.as_ptr(),
                v_ptr_end.as_ptr(),
                i_ptr.as_ptr(),
                i_ptr_end.as_ptr(),
            );

            encoder.resource_barrier(
                &[],
                &[],
                &[TextureBarrier {
                    texture,
                    subresource_range: TextureSubResourceSet {
                        aspect: TextureAspect::COLOR,
                        base_mip_level: 0,
                        num_mip_levels: 1,
                        base_array_slice: 0,
                        num_array_slices: 1,
                    },
                    before_sync: BarrierSync::ALL,
                    after_sync: BarrierSync::RENDER_TARGET,
                    before_access: BarrierAccess::NONE,
                    after_access: BarrierAccess::RENDER_TARGET_WRITE,
                    before_layout: ImageLayout::Undefined,
                    after_layout: ImageLayout::ColorAttachmentOptimal,
                    queue_transition_mode: Default::default(),
                }],
            );

            encoder.begin_rendering(&BeginRenderingInfo {
                render_area: Rect {
                    x: 0,
                    y: 0,
                    w: 0, // TODO: fill out
                    h: 0, // TODO: fill out
                },
                layer_count: 1,
                view_mask: 0,
                color_attachments: &[RenderingColorAttachmentInfo {
                    image: texture,
                    image_layout: ImageLayout::ColorAttachmentOptimal,
                    load_op: AttachmentLoadOp::Clear(ColorClearValue::Int(0)),
                    store_op: AttachmentStoreOp::Store,
                }],
                depth_stencil_attachment: None,
            });

            self.bind_resources(index, &command_list, encoder.deref_mut());

            let mut vtx_base = 0;
            let mut idx_base = 0;
            for job in render_data.primitives {
                if let aleph_egui::epaint::Primitive::Mesh(triangles) = &job.primitive {
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

                    self.record_job_commands(encoder.as_mut(), &job, vtx_base, idx_base);

                    vtx_base += triangles.vertices.len();
                    idx_base += triangles.indices.len();
                }
            }

            encoder.end_rendering();

            encoder.resource_barrier(
                &[],
                &[],
                &[TextureBarrier {
                    texture,
                    subresource_range: TextureSubResourceSet {
                        aspect: TextureAspect::COLOR,
                        base_mip_level: 0,
                        num_mip_levels: 1,
                        base_array_slice: 0,
                        num_array_slices: 1,
                    },
                    before_sync: BarrierSync::RENDER_TARGET,
                    after_sync: BarrierSync::ALL,
                    before_access: BarrierAccess::RENDER_TARGET_WRITE,
                    after_access: BarrierAccess::NONE,
                    before_layout: ImageLayout::ColorAttachmentOptimal,
                    after_layout: ImageLayout::PresentSrc,
                    queue_transition_mode: Default::default(),
                }],
            );

            // Unmap the buffers
            self.unmap_buffers(index);
        }

        list.query_interface().ok().unwrap()
    }

    unsafe fn record_job_commands(
        &mut self,
        encoder: &mut dyn IGeneralEncoder,
        job: &aleph_egui::ClippedPrimitive,
        vtx_base: usize,
        idx_base: usize,
    ) {
        if let aleph_egui::epaint::Primitive::Mesh(triangles) = &job.primitive {
            let scissor_rect = self.calculate_clip_rect(job);
            encoder.set_scissor_rects(&[scissor_rect]);
            encoder.draw_indexed(
                triangles.indices.len() as _,
                1,
                idx_base as _,
                0,
                vtx_base as _,
            );
        }
    }

    unsafe fn bind_resources(
        &self,
        index: usize,
        command_list: &ID3D12GraphicsCommandList7,
        encoder: &mut dyn IGeneralEncoder,
    ) {
        encoder.bind_graphics_pipeline(self.global.graphics_pipeline.deref());

        //
        // Bind the descriptor heap
        //
        let heaps = [Some(self.global.srv_heap.as_raw().clone())];
        command_list.SetDescriptorHeaps(&heaps);

        //
        // Bind the texture
        //
        command_list.SetGraphicsRootDescriptorTable(0, self.frames[index].font_gpu_srv.into());

        //
        // Push screen size via root constants
        //
        let width_pixels = self.global.swap_width as f32;
        let height_pixels = self.global.swap_height as f32;
        let width_points = width_pixels / self.pixels_per_point;
        let height_points = height_pixels / self.pixels_per_point;
        let values_data = [width_points, height_points];
        encoder.set_push_constant_block(0, bytemuck::cast_slice(&values_data));

        //
        // Bind the vertex and index buffers to render with
        //
        encoder.bind_vertex_buffers(
            0,
            &[InputAssemblyBufferBinding {
                buffer: self.frames[index].vtx_buffer.deref(),
                offset: 0,
            }],
        );
        encoder.bind_index_buffer(
            IndexType::U32,
            &InputAssemblyBufferBinding {
                buffer: self.frames[index].idx_buffer.deref(),
                offset: 0,
            },
        );

        //
        // Set the viewport state, we're going to be rendering to the whole frame
        //
        encoder.set_viewports(&[Viewport {
            x: 0.0,
            y: 0.0,
            width: self.global.swap_width as _,
            height: self.global.swap_height as _,
            min_depth: 0.0,
            max_depth: 1.0,
        }]);
    }

    unsafe fn map_buffers(
        &self,
        index: usize,
    ) -> (NonNull<u8>, NonNull<u8>, NonNull<u8>, NonNull<u8>) {
        let v_ptr = self.frames[index]
            .vtx_buffer
            .map()
            .expect("Failed to map vertex buffer");
        let v_ptr_end = v_ptr.as_ptr().add(PerFrameObjects::vertex_buffer_size());
        let v_ptr_end = NonNull::new(v_ptr_end).unwrap();

        let i_ptr = self.frames[index]
            .idx_buffer
            .map()
            .expect("Failed to map index buffer");
        let i_ptr_end = i_ptr.as_ptr().add(PerFrameObjects::index_buffer_size());
        let i_ptr_end = NonNull::new(i_ptr_end).unwrap();

        (v_ptr, v_ptr_end, i_ptr, i_ptr_end)
    }

    unsafe fn unmap_buffers(&self, index: usize) {
        self.frames[index].vtx_buffer.unmap();
        self.frames[index].idx_buffer.unmap();
    }

    fn calculate_clip_rect(&self, job: &aleph_egui::ClippedPrimitive) -> Rect {
        let width_pixels = self.global.swap_width as f32;
        let height_pixels = self.global.swap_height as f32;

        // Calculate clip offset
        let min = job.clip_rect.min;
        let min = egui::Pos2 {
            x: min.x * self.pixels_per_point,
            y: min.y * self.pixels_per_point,
        };
        let min = egui::Pos2 {
            x: min.x.clamp(0.0, width_pixels),
            y: min.y.clamp(0.0, height_pixels),
        };

        // Calculate clip extent
        let max = job.clip_rect.max;
        let max = egui::Pos2 {
            x: max.x * self.pixels_per_point,
            y: max.y * self.pixels_per_point,
        };
        let max = egui::Pos2 {
            x: max.x.clamp(min.x, width_pixels),
            y: max.y.clamp(min.y, height_pixels),
        };

        Rect {
            x: min.x.round() as _,
            y: min.y.round() as _,
            w: (max.x - min.x).round() as _,
            h: (max.y - min.y).round() as _,
        }
    }
}
