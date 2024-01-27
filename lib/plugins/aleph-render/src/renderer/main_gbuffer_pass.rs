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

use crate::renderer::params::BackBufferInfo;
use crate::shader_db_accessor::ShaderDatabaseAccessor;
use crate::shaders;

struct MainGBufferPassPayload {
    gbuffer0: ResourceMut,
    gbuffer0_format: Format,
    gbuffer1: ResourceMut,
    gbuffer1_format: Format,
    gbuffer2: ResourceMut,
    gbuffer2_format: Format,
    depth_buffer: ResourceMut,
    depth_buffer_format: Format,
    gbuffer_extent: Extent2D,
    pipeline: AnyArc<dyn IGraphicsPipeline>,
}

pub struct MainGBufferPassOutput {
    pub gbuffer0: ResourceMut,
    pub gbuffer1: ResourceMut,
    pub gbuffer2: ResourceMut,
    pub depth_buffer: ResourceMut,
}

pub fn pass(
    frame_graph: &mut FrameGraphBuilder,
    device: &dyn IDevice,
    pin_board: &PinBoard,
    shader_db: &ShaderDatabaseAccessor,
) {
    frame_graph.add_pass(
        "EguiPass",
        |data: &mut Payload<MainGBufferPassPayload>, resources| {
            let back_buffer_info: &BackBufferInfo = pin_board.get().unwrap();
            let b_desc = &back_buffer_info.desc;

            // BaseColor+AO
            let gbuffer0_format = Format::Rgba8Unorm;
            let gbuffer0 = resources.create_texture(
                &TextureDesc {
                    width: b_desc.width,
                    height: b_desc.height,
                    depth: 1,
                    format: gbuffer0_format,
                    dimension: TextureDimension::Texture2D,
                    clear_value: Some(OptimalClearValue::ColorInt(0x00000000)),
                    array_size: 1,
                    mip_levels: 1,
                    sample_count: 1,
                    sample_quality: 0,
                    usage: Default::default(),
                    name: Some("Gbuffer0"),
                },
                ResourceUsageFlags::RENDER_TARGET,
            );

            // WorldNormal
            let gbuffer1_format = Format::Rgba32Float;
            let gbuffer1 = resources.create_texture(
                &TextureDesc {
                    width: b_desc.width,
                    height: b_desc.height,
                    depth: 1,
                    format: gbuffer1_format,
                    dimension: TextureDimension::Texture2D,
                    clear_value: Some(OptimalClearValue::ColorInt(0x00000000)),
                    array_size: 1,
                    mip_levels: 1,
                    sample_count: 1,
                    sample_quality: 0,
                    usage: Default::default(),
                    name: Some("Gbuffer1"),
                },
                ResourceUsageFlags::RENDER_TARGET,
            );

            // Metal+Roughnes
            let gbuffer2_format = Format::Rg8Unorm;
            let gbuffer2 = resources.create_texture(
                &TextureDesc {
                    width: b_desc.width,
                    height: b_desc.height,
                    depth: 1,
                    format: gbuffer2_format,
                    dimension: TextureDimension::Texture2D,
                    clear_value: Some(OptimalClearValue::ColorInt(0x00000000)),
                    array_size: 1,
                    mip_levels: 1,
                    sample_count: 1,
                    sample_quality: 0,
                    usage: Default::default(),
                    name: Some("Gbuffer2"),
                },
                ResourceUsageFlags::RENDER_TARGET,
            );

            let depth_buffer_format = Format::Depth32Float;
            let depth_buffer = resources.create_texture(
                &TextureDesc {
                    width: b_desc.width,
                    height: b_desc.height,
                    depth: 1,
                    format: depth_buffer_format,
                    dimension: TextureDimension::Texture2D,
                    clear_value: Some(OptimalClearValue::DepthStencil(1.0, 0)),
                    array_size: 1,
                    mip_levels: 1,
                    sample_count: 1,
                    sample_quality: 0,
                    usage: Default::default(),
                    name: Some("Gbuffer1"),
                },
                ResourceUsageFlags::RENDER_TARGET,
            );

            let descriptor_set_layout = create_descriptor_set_layout(device);
            let pipeline_layout = create_root_signature(device, descriptor_set_layout.as_ref());

            let vertex_shader = shader_db
                .load(shaders::aleph_render::deferred::main_gbuffer_vert())
                .unwrap();
            let fragment_shader = shader_db
                .load(shaders::aleph_render::deferred::main_gbuffer_frag())
                .unwrap();
            let vertex_shader = device.create_shader(&vertex_shader).unwrap();
            let fragment_shader = device.create_shader(&fragment_shader).unwrap();

            let graphics_pipeline = create_pipeline_state(
                device,
                pipeline_layout.as_ref(),
                vertex_shader.as_ref(),
                fragment_shader.as_ref(),
            );

            data.write(MainGBufferPassPayload {
                gbuffer0,
                gbuffer0_format,
                gbuffer1,
                gbuffer1_format,
                gbuffer2,
                gbuffer2_format,
                depth_buffer,
                depth_buffer_format,
                gbuffer_extent: Extent2D::new(b_desc.width, b_desc.height),
                pipeline: graphics_pipeline,
            });
            pin_board.publish(MainGBufferPassOutput {
                gbuffer0,
                gbuffer1,
                gbuffer2,
                depth_buffer,
            });
        },
        |data, encoder, resources, _| unsafe {
            // Unwrap all our fg resources from our setup payload
            let data = data.unwrap();

            let gbuffer0 = resources.get_texture(data.gbuffer0).unwrap();
            let gbuffer1 = resources.get_texture(data.gbuffer1).unwrap();
            let gbuffer2 = resources.get_texture(data.gbuffer2).unwrap();
            let depth_buffer = resources.get_texture(data.depth_buffer).unwrap();

            let gbuffer0_rtv = gbuffer0
                .get_rtv(&ImageViewDesc {
                    format: data.gbuffer0_format,
                    view_type: ImageViewType::Tex2D,
                    sub_resources: TextureSubResourceSet::with_color(),
                    writable: true,
                })
                .unwrap();

            let gbuffer1_rtv = gbuffer1
                .get_rtv(&ImageViewDesc {
                    format: data.gbuffer1_format,
                    view_type: ImageViewType::Tex2D,
                    sub_resources: TextureSubResourceSet::with_color(),
                    writable: true,
                })
                .unwrap();

            let gbuffer2_rtv = gbuffer2
                .get_rtv(&ImageViewDesc {
                    format: data.gbuffer2_format,
                    view_type: ImageViewType::Tex2D,
                    sub_resources: TextureSubResourceSet::with_color(),
                    writable: true,
                })
                .unwrap();

            let depth_buffer_dsv = depth_buffer
                .get_dsv(&ImageViewDesc {
                    format: data.depth_buffer_format,
                    view_type: ImageViewType::Tex2D,
                    sub_resources: TextureSubResourceSet::with_depth(),
                    writable: true,
                })
                .unwrap();

            // Begin a render pass targeting our back buffer
            encoder.begin_rendering(&BeginRenderingInfo {
                layer_count: 1,
                extent: data.gbuffer_extent.clone(),
                color_attachments: &[
                    RenderingColorAttachmentInfo {
                        image_view: gbuffer0_rtv,
                        image_layout: ImageLayout::ColorAttachment,
                        load_op: AttachmentLoadOp::Clear(ColorClearValue::Int(0x00000000)),
                        store_op: AttachmentStoreOp::Store,
                    },
                    RenderingColorAttachmentInfo {
                        image_view: gbuffer1_rtv,
                        image_layout: ImageLayout::ColorAttachment,
                        load_op: AttachmentLoadOp::Clear(ColorClearValue::Int(0x00000000)),
                        store_op: AttachmentStoreOp::Store,
                    },
                    RenderingColorAttachmentInfo {
                        image_view: gbuffer2_rtv,
                        image_layout: ImageLayout::ColorAttachment,
                        load_op: AttachmentLoadOp::Clear(ColorClearValue::Int(0x00000000)),
                        store_op: AttachmentStoreOp::Store,
                    },
                ],
                depth_stencil_attachment: Some(&RenderingDepthStencilAttachmentInfo {
                    image_view: depth_buffer_dsv,
                    image_layout: ImageLayout::DepthStencilAttachment,
                    depth_load_op: AttachmentLoadOp::Clear(DepthStencilClearValue::depth(1.0)),
                    depth_store_op: AttachmentStoreOp::Store,
                    stencil_load_op: AttachmentLoadOp::None,
                    stencil_store_op: AttachmentStoreOp::None,
                }),
                allow_uav_writes: false,
            });

            encoder.bind_graphics_pipeline(data.pipeline.as_ref());

            // encoder.bind_descriptor_sets(
            //     data.pipeline_layout.as_ref(),
            //     PipelineBindPoint::Graphics,
            //     0,
            //     &[descriptor_set],
            // );

            // //
            // // Bind the vertex and index buffers to render with
            // //
            // encoder.bind_vertex_buffers(
            //     0,
            //     &[InputAssemblyBufferBinding {
            //         buffer: vtx_buffer,
            //         offset: 0,
            //     }],
            // );
            // encoder.bind_index_buffer(
            //     IndexType::U32,
            //     &InputAssemblyBufferBinding {
            //         buffer: idx_buffer,
            //         offset: 0,
            //     },
            // );

            encoder.set_viewports(&[Viewport {
                x: 0.0,
                y: 0.0,
                width: data.gbuffer_extent.width as _,
                height: data.gbuffer_extent.height as _,
                min_depth: 0.0,
                max_depth: 1.0,
            }]);

            encoder.end_rendering();
        },
    );
}

fn create_descriptor_set_layout(device: &dyn IDevice) -> AnyArc<dyn IDescriptorSetLayout> {
    let descriptor_set_layout_desc = DescriptorSetLayoutDesc {
        visibility: DescriptorShaderVisibility::All,
        items: &[
            DescriptorSetLayoutBinding::with_type(DescriptorType::UniformBuffer)
                .with_binding_num(0),
            DescriptorSetLayoutBinding::with_type(DescriptorType::UniformBuffer)
                .with_binding_num(1),
        ],
        name: Some("main_gbuffer_pass::DescriptorSetLayout"),
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
        name: Some("main_gbuffer_pass::RootSignature"),
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
    let rasterizer_state = RasterizerStateDesc {
        cull_mode: CullMode::Back,
        front_face: FrontFaceOrder::CounterClockwise,
        polygon_mode: PolygonMode::Fill,
        depth_bias: 0,
        depth_bias_clamp: 0.0,
        depth_bias_slope_factor: 0.0,
    };

    let depth_stencil_state = DepthStencilStateDesc {
        depth_test: true,
        depth_write: true,
        depth_compare_op: CompareOp::Less,
        stencil_test: false,
        depth_bounds_enable: false,
        ..Default::default()
    };

    let vertex_layout = VertexInputStateDesc {
        input_bindings: &[VertexInputBindingDesc {
            binding: 0,
            stride: 44,
            input_rate: VertexInputRate::PerVertex,
        }],
        input_attributes: &[
            VertexInputAttributeDesc {
                location: 0,
                binding: 0,
                format: Format::Rgb32Float,
                offset: 0,
            },
            VertexInputAttributeDesc {
                location: 1,
                binding: 0,
                format: Format::Rg32Float,
                offset: 12,
            },
            VertexInputAttributeDesc {
                location: 2,
                binding: 0,
                format: Format::Rgb32Float,
                offset: 20,
            },
            VertexInputAttributeDesc {
                location: 3,
                binding: 0,
                format: Format::Rgb32Float,
                offset: 32,
            },
        ],
    };

    let input_assembly_state = InputAssemblyStateDesc {
        primitive_topology: PrimitiveTopology::TriangleList,
    };

    let blend_state = BlendStateDesc {
        attachments: &[
            AttachmentBlendState {
                blend_enabled: false,
                color_write_mask: ColorComponentFlags::all(),
                ..Default::default()
            },
            AttachmentBlendState {
                blend_enabled: false,
                color_write_mask: ColorComponentFlags::all(),
                ..Default::default()
            },
            AttachmentBlendState {
                blend_enabled: false,
                color_write_mask: ColorComponentFlags::all(),
                ..Default::default()
            },
        ],
    };

    let graphics_pipeline_desc_new = GraphicsPipelineDesc {
        shader_stages: &[vertex_shader, pixel_shader],
        pipeline_layout,
        vertex_layout: &vertex_layout,
        input_assembly_state: &input_assembly_state,
        rasterizer_state: &rasterizer_state,
        depth_stencil_state: &depth_stencil_state,
        blend_state: &blend_state,
        render_target_formats: &[
            Format::Bgra8UnormSrgb,
            Format::Rgba32Float,
            Format::Rg8Unorm,
        ],
        depth_stencil_format: Some(Format::Depth32Float),
        name: Some("main_gbuffer_pass::GraphicsPipelineState"),
    };

    device
        .create_graphics_pipeline(&graphics_pipeline_desc_new)
        .unwrap()
}
