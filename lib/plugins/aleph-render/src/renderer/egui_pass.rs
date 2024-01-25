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

use crate::renderer::params::BackBufferInfo;
use aleph_frame_graph::*;
use aleph_interfaces::any::AnyArc;
use aleph_pin_board::PinBoard;
use aleph_rhi_api::*;
use aleph_shader_db::IShaderDatabase;
use egui::RenderData;

struct EguiPassPayload {
    pipeline_layout: AnyArc<dyn IPipelineLayout>,
    pipeline: AnyArc<dyn IGraphicsPipeline>,
    swap_extent: Extent2D,
    pixels_per_point: f32,
    back_buffer: ResourceMut,
    vtx_buffer: ResourceMut,
    idx_buffer: ResourceMut,
}

/// The output of the setup phase
pub struct EguiPassOutput {
    pub set_layout: AnyArc<dyn IDescriptorSetLayout>,
    pub id: ResourceMut,
}

/// The input the pass expects in the execute phase, to be pulled from the context pin board.
pub struct EguiPassContext {
    pub descriptor_set: DescriptorSetHandle,
    pub render_data: RenderData,
}

pub fn egui_pass(
    frame_graph: &mut FrameGraphBuilder,
    device: &dyn IDevice,
    pin_board: &PinBoard,
    shader_db: &dyn IShaderDatabase,
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
                ResourceUsageFlags::RENDER_TARGET,
            );

            let vtx_buffer = resources.create_buffer(
                &BufferDesc {
                    size: VERTEX_BUFFER_SIZE as u64,
                    cpu_access: CpuAccessMode::Write,
                    name: Some("Egui Vertex Buffer"),
                    ..Default::default()
                },
                ResourceUsageFlags::VERTEX_BUFFER | ResourceUsageFlags::COPY_DEST,
            );

            let idx_buffer = resources.create_buffer(
                &BufferDesc {
                    size: INDEX_BUFFER_SIZE as u64,
                    cpu_access: CpuAccessMode::Write,
                    name: Some("Egui Index Buffer"),
                    ..Default::default()
                },
                ResourceUsageFlags::INDEX_BUFFER | ResourceUsageFlags::COPY_DEST,
            );

            let descriptor_set_layout = create_descriptor_set_layout(device);
            let pipeline_layout = create_root_signature(device, descriptor_set_layout.as_ref());

            let vertex_shader = shader_db.get("aleph-render/egui/egui.vert").unwrap();
            let fragment_shader = shader_db.get("aleph-render/egui/egui.frag").unwrap();
            let (vertex_data, fragment_data) = match device.get_backend_api() {
                BackendAPI::Vulkan => (
                    ShaderBinary::Spirv(vertex_shader.spirv),
                    ShaderBinary::Spirv(fragment_shader.spirv),
                ),
                BackendAPI::D3D12 => (
                    ShaderBinary::Dxil(vertex_shader.dxil),
                    ShaderBinary::Dxil(fragment_shader.dxil),
                ),
            };
            let vertex_shader = device
                .create_shader(&ShaderOptions {
                    shader_type: ShaderType::Vertex,
                    data: vertex_data,
                    entry_point: "main",
                    name: Some("egui::VertexShader"),
                })
                .unwrap();

            let fragment_shader = device
                .create_shader(&ShaderOptions {
                    shader_type: ShaderType::Fragment,
                    data: fragment_data,
                    entry_point: "main",
                    name: Some("egui::FragmentShader"),
                })
                .unwrap();

            let graphics_pipeline = create_pipeline_state(
                device,
                pipeline_layout.as_ref(),
                vertex_shader.as_ref(),
                fragment_shader.as_ref(),
            );

            let pixels_per_point = back_buffer_info.pixels_per_point;
            data.write(EguiPassPayload {
                pipeline_layout: pipeline_layout,
                pipeline: graphics_pipeline,
                swap_extent: Extent2D::new(back_buffer_desc.width, back_buffer_desc.height),
                pixels_per_point,
                back_buffer,
                vtx_buffer,
                idx_buffer,
            });
            pin_board.publish(EguiPassOutput {
                id: back_buffer,
                set_layout: descriptor_set_layout,
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
                    sub_resources: TextureSubResourceSet::with_color(),
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

fn create_descriptor_set_layout(device: &dyn IDevice) -> AnyArc<dyn IDescriptorSetLayout> {
    let descriptor_set_layout_desc = DescriptorSetLayoutDesc {
        visibility: DescriptorShaderVisibility::All,
        items: &[
            DescriptorSetLayoutBinding::with_type(DescriptorType::Texture).with_binding_num(0),
            DescriptorSetLayoutBinding::with_type(DescriptorType::Sampler).with_binding_num(1),
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
        push_constant_blocks: &[PushConstantBlock {
            binding: 0,
            visibility: DescriptorShaderVisibility::All,
            size: 8,
        }],
        name: Some("egui::RootSignature"),
    };
    device
        .create_pipeline_layout(&pipeline_layout_desc)
        .unwrap()
}

fn create_pipeline_state(
    device: &dyn IDevice,
    pipeline_layout: &dyn IPipelineLayout,
    vertex_shader: &dyn IShader,
    pixel_shader: &dyn IShader,
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

    let graphics_pipeline_desc_new = GraphicsPipelineDesc {
        shader_stages: &[vertex_shader, pixel_shader],
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
