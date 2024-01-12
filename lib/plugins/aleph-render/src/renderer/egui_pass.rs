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

use aleph_frame_graph::*;
use aleph_interfaces::any::AnyArc;
use aleph_pin_board::PinBoard;
use aleph_rhi_api::*;
use egui::RenderData;

use crate::renderer::GlobalObjects;

struct EguiPassPayload {
    pipeline_layout: AnyArc<dyn IPipelineLayout>,
    pipeline: AnyArc<dyn IGraphicsPipeline>,
    swap_extent: Extent2D,
    pixels_per_point: f32,
    back_buffer: ResourceMut,
    vtx_buffer: ResourceMut,
    idx_buffer: ResourceMut,
}

pub struct BackBufferInfo {
    pub desc: TextureDesc<'static>,
    pub pixels_per_point: f32,
}

/// The output of the setup phase
pub struct EguiPassOutput {
    pub id: ResourceMut,
}

/// The input the pass expects in the execute phase, to be pulled from the context pin board.
pub struct EguiPassContext {
    pub descriptor_set: DescriptorSetHandle,
    pub render_data: RenderData,
}

pub fn egui_pass(
    frame_graph: &mut FrameGraphBuilder,
    pin_board: &PinBoard,
    global: &GlobalObjects,
) {
    const VERTEX_BUFFER_SIZE: usize = 1024 * 1024 * 4;
    const INDEX_BUFFER_SIZE: usize = 1024 * 1024 * 4;

    frame_graph.add_pass(
        "EguiPass",
        |data: &mut Payload<EguiPassPayload>, resources| {
            let back_buffer_info: &BackBufferInfo = pin_board.get().unwrap();

            let back_buffer_desc = back_buffer_info.desc.clone().with_name("Swap Chain Image");
            let back_buffer = resources.import_texture(
                &TextureImportDesc {
                    desc: &back_buffer_desc,
                    before_sync: BarrierSync::ALL,
                    before_access: BarrierAccess::NONE,
                    before_layout: ImageLayout::Undefined,
                    after_sync: BarrierSync::NONE,
                    after_access: BarrierAccess::NONE,
                    after_layout: ImageLayout::PresentSrc,
                },
                BarrierSync::NONE,
                ResourceUsageFlags::RENDER_TARGET,
            );

            pin_board.publish(EguiPassOutput { id: back_buffer });

            let vtx_buffer = resources.create_buffer(
                &BufferDesc {
                    size: VERTEX_BUFFER_SIZE as u64,
                    cpu_access: CpuAccessMode::Write,
                    name: Some("Egui Vertex Buffer"),
                    ..Default::default()
                },
                BarrierSync::NONE,
                ResourceUsageFlags::VERTEX_BUFFER | ResourceUsageFlags::COPY_DEST,
            );

            let idx_buffer = resources.create_buffer(
                &BufferDesc {
                    size: INDEX_BUFFER_SIZE as u64,
                    cpu_access: CpuAccessMode::Write,
                    name: Some("Egui Index Buffer"),
                    ..Default::default()
                },
                BarrierSync::NONE,
                ResourceUsageFlags::INDEX_BUFFER | ResourceUsageFlags::COPY_DEST,
            );

            let pixels_per_point = back_buffer_info.pixels_per_point;
            data.write(EguiPassPayload {
                pipeline_layout: global.pipeline_layout.clone(),
                pipeline: global.graphics_pipeline.clone(),
                swap_extent: Extent2D::new(back_buffer_desc.width, back_buffer_desc.height),
                pixels_per_point,
                back_buffer,
                vtx_buffer,
                idx_buffer,
            });
        },
        |data, encoder, resources, context| unsafe {
            // Unwrap all our fg resources from our setup payload
            let data = data.unwrap();
            let back_buffer = resources.get_texture(data.back_buffer).unwrap();
            let vtx_buffer = resources.get_buffer(data.vtx_buffer).unwrap();
            let idx_buffer = resources.get_buffer(data.idx_buffer).unwrap();

            let EguiPassContext {
                descriptor_set,
                render_data,
            } = context.get().unwrap();

            // Map and calculate our begin/end pointers for the mapped vertex and index buffer
            // regions
            let mut v_ptr = vtx_buffer.map().unwrap().as_ptr();
            let v_ptr_end = v_ptr.add(VERTEX_BUFFER_SIZE);
            let mut i_ptr = idx_buffer.map().unwrap().as_ptr();
            let i_ptr_end = i_ptr.add(INDEX_BUFFER_SIZE);

            // Get an RTV from our imported back buffer
            let desc = back_buffer.desc();
            let image_view = back_buffer
                .get_rtv(&ImageViewDesc {
                    format: desc.format,
                    view_type: ImageViewType::Tex2D,
                    sub_resources: TextureSubResourceSet {
                        aspect: TextureAspect::COLOR,
                        base_mip_level: 0,
                        num_mip_levels: 1,
                        base_array_slice: 0,
                        num_array_slices: 1,
                    },
                    writable: true,
                })
                .unwrap();

            // Begin a render pass targeting our back buffer
            encoder.begin_rendering(&BeginRenderingInfo {
                layer_count: 1,
                extent: Extent2D {
                    width: desc.width,
                    height: desc.height,
                },
                color_attachments: &[RenderingColorAttachmentInfo {
                    image_view,
                    image_layout: ImageLayout::ColorAttachment,
                    load_op: AttachmentLoadOp::Clear(ColorClearValue::Int(0)),
                    store_op: AttachmentStoreOp::Store,
                }],
                depth_stencil_attachment: None,
                allow_uav_writes: false,
            });

            encoder.bind_graphics_pipeline(data.pipeline.as_ref());

            encoder.bind_descriptor_sets(
                data.pipeline_layout.as_ref(),
                PipelineBindPoint::Graphics,
                0,
                &[descriptor_set.clone()],
            );

            //
            // Push screen size via root constants
            //
            let width_pixels = data.swap_extent.width as f32;
            let height_pixels = data.swap_extent.height as f32;
            let width_points = width_pixels / data.pixels_per_point;
            let height_points = height_pixels / data.pixels_per_point;
            let values_data = [width_points, height_points];
            encoder.set_push_constant_block(0, bytemuck::cast_slice(&values_data));

            //
            // Bind the vertex and index buffers to render with
            //
            encoder.bind_vertex_buffers(
                0,
                &[InputAssemblyBufferBinding {
                    buffer: vtx_buffer,
                    offset: 0,
                }],
            );
            encoder.bind_index_buffer(
                IndexType::U32,
                &InputAssemblyBufferBinding {
                    buffer: idx_buffer,
                    offset: 0,
                },
            );

            //
            // Set the viewport state, we're going to be rendering to the whole frame
            //
            encoder.set_viewports(&[Viewport {
                x: 0.0,
                y: 0.0,
                width: data.swap_extent.width as _,
                height: data.swap_extent.height as _,
                min_depth: 0.0,
                max_depth: 1.0,
            }]);

            let mut vtx_base = 0;
            let mut idx_base = 0;
            for job in render_data.primitives.iter() {
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

                    record_job_commands(
                        encoder,
                        &job,
                        data.swap_extent.clone(),
                        data.pixels_per_point,
                        vtx_base,
                        idx_base,
                    );

                    vtx_base += triangles.vertices.len();
                    idx_base += triangles.indices.len();
                }
            }

            encoder.end_rendering();

            vtx_buffer.unmap();
            idx_buffer.unmap();
        },
    );
}

unsafe fn record_job_commands(
    encoder: &mut dyn IGeneralEncoder,
    job: &aleph_egui::ClippedPrimitive,
    swap_extent: Extent2D,
    pixels_per_point: f32,
    vtx_base: usize,
    idx_base: usize,
) {
    if let aleph_egui::epaint::Primitive::Mesh(triangles) = &job.primitive {
        let scissor_rect = calculate_clip_rect(job, swap_extent, pixels_per_point);

        // Reject the command if the scissor rect is 0 as we'll never actually draw anything
        if (scissor_rect.w * scissor_rect.h) == 0 {
            return;
        }

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

fn calculate_clip_rect(
    job: &aleph_egui::ClippedPrimitive,
    swap_extent: Extent2D,
    pixels_per_point: f32,
) -> Rect {
    let width_pixels = swap_extent.width as f32;
    let height_pixels = swap_extent.height as f32;

    // Calculate clip offset
    let min = job.clip_rect.min;
    let min = egui::Pos2 {
        x: min.x * pixels_per_point,
        y: min.y * pixels_per_point,
    };
    let min = egui::Pos2 {
        x: min.x.clamp(0.0, width_pixels),
        y: min.y.clamp(0.0, height_pixels),
    };

    // Calculate clip extent
    let max = job.clip_rect.max;
    let max = egui::Pos2 {
        x: max.x * pixels_per_point,
        y: max.y * pixels_per_point,
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
