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

use crate::renderer::main_gbuffer_pass::MainGBufferPassOutput;
use crate::renderer::params::BackBufferInfo;
use crate::shaders;
use aleph_frame_graph::*;
use aleph_interfaces::any::AnyArc;
use aleph_pin_board::PinBoard;
use aleph_rhi_api::*;
use aleph_shader_db::{IShaderDatabase, IShaderDatabaseExt};

struct LightingResolvePassPayload {
    gbuffer0: ResourceRef,
    gbuffer1: ResourceRef,
    gbuffer2: ResourceRef,
    lighting: ResourceMut,
    back_buffer_extent: Extent2D,
    pipeline: AnyArc<dyn IComputePipeline>,
}
pub struct LightingResolvePassOutput {
    pub lighting: ResourceMut,
}

pub fn lighting_resolve_pass(
    frame_graph: &mut FrameGraphBuilder,
    device: &dyn IDevice,
    pin_board: &PinBoard,
    shader_db: &dyn IShaderDatabase,
) {
    frame_graph.add_pass(
        "DeferredLightingPass",
        |data: &mut Payload<LightingResolvePassPayload>, resources| {
            let main_gbuffer_pass_output: &MainGBufferPassOutput = pin_board.get().unwrap();
            let back_buffer_info: &BackBufferInfo = pin_board.get().unwrap();
            let b_desc = &back_buffer_info.desc;

            let gbuffer0 = resources.read_texture(
                main_gbuffer_pass_output.gbuffer0,
                ResourceUsageFlags::SHADER_RESOURCE,
            );
            let gbuffer1 = resources.read_texture(
                main_gbuffer_pass_output.gbuffer1,
                ResourceUsageFlags::SHADER_RESOURCE,
            );
            let gbuffer2 = resources.read_texture(
                main_gbuffer_pass_output.gbuffer2,
                ResourceUsageFlags::SHADER_RESOURCE,
            );
            let lighting = resources.create_texture(
                &TextureDesc {
                    width: b_desc.width,
                    height: b_desc.height,
                    depth: 1,
                    format: Format::Rgba16Float,
                    dimension: TextureDimension::Texture2D,
                    clear_value: Some(OptimalClearValue::ColorInt(0x000000FF)),
                    array_size: 1,
                    mip_levels: 1,
                    sample_count: 1,
                    sample_quality: 0,
                    usage: Default::default(),
                    name: Some("OutputLighting"),
                },
                ResourceUsageFlags::RENDER_TARGET,
            );

            let set_layout = device
                .create_descriptor_set_layout(&DescriptorSetLayoutDesc {
                    visibility: DescriptorShaderVisibility::Compute,
                    items: &[
                        DescriptorSetLayoutBinding::with_type(DescriptorType::UniformBuffer)
                            .with_binding_num(0),
                        DescriptorSetLayoutBinding::with_type(DescriptorType::Texture)
                            .with_binding_num(1),
                        DescriptorSetLayoutBinding::with_type(DescriptorType::Texture)
                            .with_binding_num(2),
                        DescriptorSetLayoutBinding::with_type(DescriptorType::Texture)
                            .with_binding_num(3),
                        DescriptorSetLayoutBinding::with_type(DescriptorType::TextureRW)
                            .with_binding_num(4),
                    ],
                    name: Some("DeferredLightingDescriptorSetLayout"),
                })
                .unwrap();
            let pipeline_layout = device
                .create_pipeline_layout(
                    &PipelineLayoutDesc::new()
                        .with_set_layouts(&[set_layout.as_ref()])
                        .with_name("DeferredLightingPipelineLayout"),
                )
                .unwrap();

            let shader_data = shader_db
                .get(shaders::aleph_render::deferred_deferred_lighting_cs())
                .unwrap();
            let shader_data = match device.get_backend_api() {
                BackendAPI::Vulkan => ShaderBinary::Spirv(shader_data.spirv),
                BackendAPI::D3D12 => ShaderBinary::Dxil(shader_data.dxil),
            };
            let shader_module = device
                .create_shader(&ShaderOptions {
                    shader_type: ShaderType::Compute,
                    data: shader_data,
                    entry_point: "main",
                    name: Some("DeferredLightingComputeShader"),
                })
                .unwrap();

            let pipeline = device
                .create_compute_pipeline(&ComputePipelineDesc {
                    shader_module: shader_module.as_ref(),
                    pipeline_layout: pipeline_layout.as_ref(),
                    name: Some("DeferredLightingPipeline"),
                })
                .unwrap();

            data.write(LightingResolvePassPayload {
                gbuffer0,
                gbuffer1,
                gbuffer2,
                lighting,
                back_buffer_extent: Extent2D::new(b_desc.width, b_desc.height),
                pipeline,
            });
            pin_board.publish(LightingResolvePassOutput { lighting });
        },
        |data, encoder, resources, _| unsafe {
            // Unwrap all our fg resources from our setup payload
            let data = data.unwrap();

            let gbuffer0 = resources.get_texture(data.gbuffer0).unwrap();
            let gbuffer0_desc = gbuffer0.desc();
            let gbuffer1 = resources.get_texture(data.gbuffer1).unwrap();
            let gbuffer1_desc = gbuffer1.desc();
            let gbuffer2 = resources.get_texture(data.gbuffer2).unwrap();
            let gbuffer2_desc = gbuffer1.desc();
            let lighting = resources.get_texture(data.lighting).unwrap();
            let lighting_desc = lighting.desc();

            let _gbuffer0_srv = gbuffer0
                .get_view(&ImageViewDesc {
                    format: gbuffer0_desc.format,
                    view_type: ImageViewType::Tex2D,
                    sub_resources: TextureSubResourceSet::with_color(),
                    writable: false,
                })
                .unwrap();
            let _gbuffer1_srv = gbuffer1
                .get_view(&ImageViewDesc {
                    format: gbuffer1_desc.format,
                    view_type: ImageViewType::Tex2D,
                    sub_resources: TextureSubResourceSet::with_color(),
                    writable: false,
                })
                .unwrap();
            let _gbuffer2_srv = gbuffer2
                .get_view(&ImageViewDesc {
                    format: gbuffer2_desc.format,
                    view_type: ImageViewType::Tex2D,
                    sub_resources: TextureSubResourceSet::with_color(),
                    writable: false,
                })
                .unwrap();
            let _lighting_uav = lighting.get_view(&ImageViewDesc {
                format: lighting_desc.format,
                view_type: ImageViewType::Tex2D,
                sub_resources: TextureSubResourceSet::with_color(),
                writable: true,
            });

            encoder.bind_compute_pipeline(data.pipeline.as_ref());

            // TODO: Bind Descriptors

            let group_count_x = data.back_buffer_extent.width.div_ceil(8);
            let group_count_y = data.back_buffer_extent.height.div_ceil(8);
            encoder.dispatch(group_count_x, group_count_y, 1);
        },
    );
}
