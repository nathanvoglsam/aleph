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

use aleph_device_allocators::{IUploadAllocator, UploadBumpAllocator};
use aleph_frame_graph::*;
use aleph_interfaces::any::AnyArc;
use aleph_pin_board::PinBoard;
use aleph_rhi_api::*;
use egui::RenderData;

use crate::renderer::backbuffer_import_pass::BackBufferHandle;
use crate::renderer::params::BackBufferInfo;
use crate::shader_db_accessor::ShaderDatabaseAccessor;
use crate::shaders;

struct EguiPassPayload {
    back_buffer: ResourceMut,
    vtx_buffer: ResourceMut,
    idx_buffer: ResourceMut,
}

/// The output of the setup phase
pub struct EguiPassOutput {
    pub set_layout: AnyArc<dyn IDescriptorSetLayout>,
}

/// The input the pass expects in the execute phase, to be pulled from the context pin board.
pub struct EguiPassContext {
    pub buffer: AnyArc<dyn IBuffer>,
    pub descriptor_set: DescriptorSetHandle,
    pub render_data: RenderData,
}

pub fn pass(
    frame_graph: &mut FrameGraphBuilder,
    device: &dyn IDevice,
    pin_board: &PinBoard,
    shader_db: &ShaderDatabaseAccessor,
) {
    const VERTEX_BUFFER_SIZE: usize = 1024 * 1024 * 4;
    const INDEX_BUFFER_SIZE: usize = 1024 * 1024 * 4;

    let back_buffer_info: &BackBufferInfo = pin_board.get().unwrap();
    let pixels_per_point = back_buffer_info.pixels_per_point;

    let descriptor_set_layout = create_descriptor_set_layout(device);
    let pipeline_layout = create_root_signature(device, descriptor_set_layout.as_ref());

    let pipeline = create_pipeline_state(device, pipeline_layout.as_ref(), shader_db);

    frame_graph.add_pass("EguiPass", |resources| {
        let BackBufferHandle { back_buffer } = pin_board.get().unwrap();
        let back_buffer = resources.write_texture(*back_buffer, ResourceUsageFlags::RENDER_TARGET);

        let vtx_buffer = resources.create_buffer(
            &BufferDesc::new(VERTEX_BUFFER_SIZE as u64)
                .cpu_write()
                .with_name("Egui Vertex Buffer"),
            ResourceUsageFlags::VERTEX_BUFFER,
        );

        let idx_buffer = resources.create_buffer(
            &BufferDesc::new(INDEX_BUFFER_SIZE as u64)
                .cpu_write()
                .with_name("Egui Index Buffer"),
            ResourceUsageFlags::INDEX_BUFFER,
        );

        let data = EguiPassPayload {
            back_buffer,
            vtx_buffer,
            idx_buffer,
        };

        pin_board.publish(EguiPassOutput {
            set_layout: descriptor_set_layout,
        });
        pin_board.publish(BackBufferHandle { back_buffer });

        move |encoder, resources| unsafe {
            let back_buffer = resources.get_texture(data.back_buffer).unwrap();
            let vtx_buffer = resources.get_buffer(data.vtx_buffer).unwrap();
            let idx_buffer = resources.get_buffer(data.idx_buffer).unwrap();

            let extent = back_buffer.desc_ref().get_extent_2d();

            let EguiPassContext {
                buffer,
                descriptor_set,
                render_data,
            } = resources.context().get().unwrap();

            // Map and calculate our begin/end pointers for the mapped vertex and index buffer
            // regions
            let v_ptr = vtx_buffer.map().unwrap();
            let vtx_alloc =
                UploadBumpAllocator::new_from_block(vtx_buffer, v_ptr, 0, VERTEX_BUFFER_SIZE)
                    .unwrap();

            let i_ptr = idx_buffer.map().unwrap();
            let idx_alloc =
                UploadBumpAllocator::new_from_block(idx_buffer, i_ptr, 0, INDEX_BUFFER_SIZE)
                    .unwrap();

            // Get an RTV from our imported back buffer
            let image_view = back_buffer
                .get_rtv(&ImageViewDesc::rtv_for_texture(back_buffer))
                .unwrap();

            // Begin a render pass targeting our back buffer
            encoder.begin_rendering(&BeginRenderingInfo {
                layer_count: 1,
                extent: extent.clone(),
                color_attachments: &[RenderingColorAttachmentInfo {
                    image_view,
                    image_layout: ImageLayout::ColorAttachment,
                    load_op: AttachmentLoadOp::Load,
                    store_op: AttachmentStoreOp::Store,
                }],
                depth_stencil_attachment: None,
                allow_uav_writes: false,
            });

            encoder.bind_graphics_pipeline(pipeline.as_ref());

            //
            // Push screen size via root constants
            //
            let width_pixels = extent.width as f32;
            let height_pixels = extent.height as f32;
            let width_points = width_pixels / pixels_per_point;
            let height_points = height_pixels / pixels_per_point;
            let values_data = [width_points, height_points];

            let ptr = buffer.map().unwrap();
            let mut ptr = ptr.cast::<[f32; 2]>();
            *ptr.as_mut() = values_data;
            buffer.unmap();

            encoder.bind_descriptor_sets(
                pipeline_layout.as_ref(),
                PipelineBindPoint::Graphics,
                0,
                &[*descriptor_set],
                &[],
            );

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
                width: extent.width as _,
                height: extent.height as _,
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

                    let v_slice = triangles.vertices.as_slice();
                    let _ = vtx_alloc.allocate_objects_clone(v_slice);
                    let i_slice = triangles.indices.as_slice();
                    let _ = idx_alloc.allocate_objects_copy(i_slice);

                    record_job_commands(
                        encoder,
                        job,
                        extent.clone(),
                        pixels_per_point,
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
        }
    });
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

fn create_descriptor_set_layout(device: &dyn IDevice) -> AnyArc<dyn IDescriptorSetLayout> {
    let descriptor_set_layout_desc = DescriptorSetLayoutDesc {
        visibility: DescriptorShaderVisibility::All,
        items: &[
            DescriptorType::UniformBuffer.binding(0),
            DescriptorType::Texture.binding(1),
            DescriptorType::Sampler.binding(2),
        ],
        name: Some("egui::DescriptorSetLayout"),
    };
    device
        .create_descriptor_set_layout(&descriptor_set_layout_desc)
        .unwrap()
}

fn create_root_signature(
    device: &dyn IDevice,
    descriptor_set_layout: &dyn IDescriptorSetLayout,
) -> AnyArc<dyn IPipelineLayout> {
    let pipeline_layout_desc = PipelineLayoutDesc {
        set_layouts: &[descriptor_set_layout],
        push_constant_blocks: &[],
        name: Some("egui::RootSignature"),
    };
    device
        .create_pipeline_layout(&pipeline_layout_desc)
        .unwrap()
}

fn create_pipeline_state(
    device: &dyn IDevice,
    pipeline_layout: &dyn IPipelineLayout,
    shader_db: &ShaderDatabaseAccessor,
) -> AnyArc<dyn IGraphicsPipeline> {
    let rasterizer_state_new = RasterizerStateDesc {
        cull_mode: CullMode::None,
        front_face: FrontFaceOrder::CounterClockwise,
        polygon_mode: PolygonMode::Fill,
        depth_bias: 0,
        depth_bias_clamp: 0.0,
        depth_bias_slope_factor: 0.0,
    };

    let depth_stencil_state_new = DepthStencilStateDesc {
        depth_test: false,
        ..Default::default()
    };

    let vertex_layout_new = VertexInputStateDesc {
        input_bindings: &[VertexInputBindingDesc {
            binding: 0,
            stride: 20,
            input_rate: VertexInputRate::PerVertex,
        }],
        input_attributes: &[
            VertexInputAttributeDesc {
                location: 0,
                binding: 0,
                format: Format::Rg32Float,
                offset: 0,
            },
            VertexInputAttributeDesc {
                location: 1,
                binding: 0,
                format: Format::Rg32Float,
                offset: 8,
            },
            VertexInputAttributeDesc {
                location: 2,
                binding: 0,
                format: Format::Rgba8Unorm,
                offset: 16,
            },
        ],
    };

    let input_assembly_state_new = InputAssemblyStateDesc {
        primitive_topology: PrimitiveTopology::TriangleList,
    };

    let blend_state_new = BlendStateDesc {
        attachments: &[AttachmentBlendState {
            blend_enabled: true,
            src_factor: BlendFactor::One,
            dst_factor: BlendFactor::OneMinusSrcAlpha,
            blend_op: BlendOp::Add,
            alpha_src_factor: BlendFactor::OneMinusDstAlpha,
            alpha_dst_factor: BlendFactor::One,
            alpha_blend_op: BlendOp::Add,
            color_write_mask: ColorComponentFlags::all(),
        }],
    };

    let vertex_shader = shader_db
        .load_stage(shaders::aleph_render::egui::egui_vert())
        .unwrap();
    let fragment_shader = shader_db
        .load_stage(shaders::aleph_render::egui::egui_frag())
        .unwrap();

    let graphics_pipeline_desc_new = GraphicsPipelineDesc {
        shader_stages: &[vertex_shader, fragment_shader],
        pipeline_layout,
        vertex_layout: &vertex_layout_new,
        input_assembly_state: &input_assembly_state_new,
        rasterizer_state: &rasterizer_state_new,
        depth_stencil_state: &depth_stencil_state_new,
        blend_state: &blend_state_new,
        render_target_formats: &[Format::Bgra8UnormSrgb],
        depth_stencil_format: None,
        name: Some("egui::GraphicsPipelineState"),
    };

    device
        .create_graphics_pipeline(&graphics_pipeline_desc_new)
        .unwrap()
}
