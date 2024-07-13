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

use crate::renderer::backbuffer_import_pass::BackBufferHandle;
use crate::renderer::tone_map_pass::TonemapPassOutput;
use crate::shader_db_accessor::ShaderDatabaseAccessor;
use crate::shaders;
use aleph_frame_graph::*;
use aleph_pin_board::PinBoard;
use aleph_rhi_api::*;
use interfaces::any::AnyArc;

struct CopyTexturePassPayload {
    input: ResourceRef,
    output: ResourceMut,
}

pub fn pass(
    frame_graph: &mut FrameGraphBuilder,
    device: &dyn IDevice,
    pin_board: &PinBoard,
    shader_db: &ShaderDatabaseAccessor,
) {
    let sampler = create_sampler(device);
    let descriptor_set_layout = create_descriptor_set_layout(device, sampler.as_ref());
    let pipeline_layout = create_root_signature(device, descriptor_set_layout.as_ref());
    let pipeline = create_pipeline_state(
        device,
        pipeline_layout.as_ref(),
        shader_db,
        Format::Bgra8UnormSrgb,
    );

    frame_graph.add_pass("CopyTexturePass", |resources| {
        let tonemap_pass: &TonemapPassOutput = pin_board.get().unwrap();
        let BackBufferHandle { back_buffer } = pin_board.get().unwrap();

        let input =
            resources.read_texture(tonemap_pass.output, ResourceUsageFlags::SHADER_RESOURCE);
        let output = resources.write_texture(*back_buffer, ResourceUsageFlags::RENDER_TARGET);

        let data = CopyTexturePassPayload { input, output };
        pin_board.publish(BackBufferHandle {
            back_buffer: output,
        });

        move |encoder, resources| unsafe {
            let input = resources.get_texture(data.input).unwrap();
            let src_desc = input.desc_ref();

            let output = resources.get_texture(data.output).unwrap();
            let dst_desc = output.desc_ref();

            let src_view = input
                .get_view(&ImageViewDesc {
                    format: src_desc.format,
                    view_type: ImageViewType::Tex2D,
                    sub_resources: TextureSubResourceSet::all(src_desc),
                    writable: false,
                })
                .unwrap();
            let dst_view = output
                .get_rtv(&ImageViewDesc {
                    format: dst_desc.format,
                    view_type: ImageViewType::Tex2D,
                    sub_resources: TextureSubResourceSet::all(dst_desc),
                    writable: false,
                })
                .unwrap();

            let set = resources
                .descriptor_arena()
                .allocate_set(descriptor_set_layout.as_ref())
                .unwrap();
            resources
                .device()
                .update_descriptor_sets(&[DescriptorWriteDesc {
                    set,
                    binding: 0,
                    array_element: 0,
                    writes: DescriptorWrites::Texture(&[ImageDescriptorWrite::srv(src_view)]),
                }]);

            encoder.begin_rendering(&BeginRenderingInfo {
                layer_count: 1,
                extent: dst_desc.get_extent_2d(),
                color_attachments: &[RenderingColorAttachmentInfo {
                    image_view: dst_view,
                    image_layout: ImageLayout::ColorAttachment,
                    load_op: AttachmentLoadOp::Clear(ColorClearValue::Int(0)),
                    store_op: AttachmentStoreOp::Store,
                }],
                depth_stencil_attachment: None,
                allow_uav_writes: false,
            });
            encoder.bind_graphics_pipeline(pipeline.as_ref());
            encoder.bind_descriptor_sets(
                pipeline_layout.as_ref(),
                PipelineBindPoint::Graphics,
                0,
                &[set],
                &[],
            );
            encoder.set_viewports(&[Viewport {
                x: 0.0,
                y: 0.0,
                width: dst_desc.width as _,
                height: dst_desc.height as _,
                min_depth: 0.0,
                max_depth: 1.0,
            }]);
            encoder.draw(3, 1, 0, 0);
            encoder.end_rendering();
        }
    });
}

fn create_descriptor_set_layout(
    device: &dyn IDevice,
    sampler: &dyn ISampler,
) -> AnyArc<dyn IDescriptorSetLayout> {
    let sampler = [sampler];
    let descriptor_set_layout_desc = DescriptorSetLayoutDesc {
        visibility: DescriptorShaderVisibility::Fragment,
        items: &[
            DescriptorType::Texture.binding(0),
            DescriptorType::Sampler
                .binding(1)
                .with_static_samplers(&sampler),
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
        name: Some("copy::RootSignature"),
    };
    device
        .create_pipeline_layout(&pipeline_layout_desc)
        .unwrap()
}

fn create_pipeline_state(
    device: &dyn IDevice,
    pipeline_layout: &dyn IPipelineLayout,
    shader_db: &ShaderDatabaseAccessor,
    format: Format,
) -> AnyArc<dyn IGraphicsPipeline> {
    let vertex_shader = shader_db
        .load_stage(shaders::aleph_render::fullscreen_tri_vert())
        .unwrap();
    let fragment_shader = shader_db
        .load_stage(shaders::aleph_render::fullscreen_tri_copy_frag())
        .unwrap();

    let vertex_layout = VertexInputStateDesc::default();

    let input_assembly_state = InputAssemblyStateDesc {
        primitive_topology: PrimitiveTopology::TriangleList,
    };

    let rasterizer_state = RasterizerStateDesc {
        cull_mode: CullMode::None,
        front_face: FrontFaceOrder::CounterClockwise,
        polygon_mode: PolygonMode::Fill,
        depth_bias: 0,
        depth_bias_clamp: 0.0,
        depth_bias_slope_factor: 0.0,
    };

    let depth_stencil_state = DepthStencilStateDesc {
        depth_test: false,
        ..Default::default()
    };

    let blend_state_new = BlendStateDesc {
        attachments: &[AttachmentBlendState {
            blend_enabled: false,
            ..Default::default()
        }],
    };

    let graphics_pipeline_desc_new = GraphicsPipelineDesc {
        shader_stages: &[vertex_shader, fragment_shader],
        pipeline_layout,
        vertex_layout: &vertex_layout,
        input_assembly_state: &input_assembly_state,
        rasterizer_state: &rasterizer_state,
        depth_stencil_state: &depth_stencil_state,
        blend_state: &blend_state_new,
        render_target_formats: &[format],
        depth_stencil_format: None,
        name: Some("copy::GraphicsPipelineState"),
    };

    device
        .create_graphics_pipeline(&graphics_pipeline_desc_new)
        .unwrap()
}

fn create_sampler(device: &dyn IDevice) -> AnyArc<dyn ISampler> {
    let desc = SamplerDesc {
        min_filter: SamplerFilter::Linear,
        mag_filter: SamplerFilter::Linear,
        mip_filter: SamplerMipFilter::Linear,
        address_mode_u: SamplerAddressMode::Clamp,
        address_mode_v: SamplerAddressMode::Clamp,
        address_mode_w: SamplerAddressMode::Clamp,
        ..Default::default()
    };
    device.create_sampler(&desc).unwrap()
}
