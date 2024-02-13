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

use crate::renderer::main_gbuffer_pass::MainGBufferPassOutput;
use crate::renderer::params::BackBufferInfo;
use crate::shader_db_accessor::ShaderDatabaseAccessor;
use crate::shaders;

struct LightingResolvePassPayload {
    device: AnyArc<dyn IDevice>,
    gbuffer0: ResourceRef,
    gbuffer1: ResourceRef,
    gbuffer2: ResourceRef,
    lighting: ResourceMut,
    set_layout: AnyArc<dyn IDescriptorSetLayout>,
    pipeline_layout: AnyArc<dyn IPipelineLayout>,
    pipeline: AnyArc<dyn IComputePipeline>,
}
pub struct LightingResolvePassOutput {
    pub lighting: ResourceMut,
}

pub fn pass(
    frame_graph: &mut FrameGraphBuilder,
    device: &dyn IDevice,
    pin_board: &PinBoard,
    shader_db: &ShaderDatabaseAccessor,
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
                ResourceUsageFlags::UNORDERED_ACCESS,
            );

            let set_layout = device
                .create_descriptor_set_layout(&DescriptorSetLayoutDesc {
                    visibility: DescriptorShaderVisibility::Compute,
                    items: &[
                        DescriptorSetLayoutBinding::with_type(DescriptorType::Texture)
                            .with_binding_num(0),
                        DescriptorSetLayoutBinding::with_type(DescriptorType::Texture)
                            .with_binding_num(1),
                        DescriptorSetLayoutBinding::with_type(DescriptorType::Texture)
                            .with_binding_num(2),
                        DescriptorSetLayoutBinding::with_type(DescriptorType::TextureRW)
                            .with_binding_num(3),
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
                .load(shaders::aleph_render::deferred::deferred_lighting_cs())
                .unwrap();
            let shader_module = device.create_shader(&shader_data).unwrap();

            let pipeline = device
                .create_compute_pipeline(&ComputePipelineDesc {
                    shader_module: shader_module.as_ref(),
                    pipeline_layout: pipeline_layout.as_ref(),
                    name: Some("DeferredLightingPipeline"),
                })
                .unwrap();

            data.write(LightingResolvePassPayload {
                device: device.upgrade(),
                gbuffer0,
                gbuffer1,
                gbuffer2,
                lighting,
                set_layout,
                pipeline_layout,
                pipeline,
            });
            pin_board.publish(LightingResolvePassOutput { lighting });
        },
        |data, encoder, resources, context| unsafe {
            // Unwrap all our fg resources from our setup payload
            let data = data.unwrap();

            // let arena: &mut dyn IDescriptorArena = ();

            let gbuffer0 = resources.get_texture(data.gbuffer0).unwrap();
            let gbuffer1 = resources.get_texture(data.gbuffer1).unwrap();
            let gbuffer2 = resources.get_texture(data.gbuffer2).unwrap();
            let lighting = resources.get_texture(data.lighting).unwrap();
            let _gbuffer0_srv = ImageView::get_srv_for(gbuffer0).unwrap();
            let _gbuffer1_srv = ImageView::get_srv_for(gbuffer1).unwrap();
            let _gbuffer2_srv = ImageView::get_srv_for(gbuffer2).unwrap();
            let _lighting_uav = ImageView::get_uav_for(lighting).unwrap();

            // let set = arena.allocate_set(data.set_layout.as_ref()).unwrap();
            // data.device.update_descriptor_sets(&[
            //     DescriptorWriteDesc::texture(set, 0, &gbuffer0_srv.srv_write()),
            //     DescriptorWriteDesc::texture(set, 1, &gbuffer1_srv.srv_write()),
            //     DescriptorWriteDesc::texture(set, 2, &gbuffer2_srv.srv_write()),
            //     DescriptorWriteDesc::texture(set, 3, &lighting_uav.uav_write()),
            // ]);

            // encoder.bind_compute_pipeline(data.pipeline.as_ref());
            // encoder.bind_descriptor_sets(
            //     data.pipeline_layout.as_ref(),
            //     PipelineBindPoint::Graphics,
            //     0,
            //     &[set],
            //     &[],
            // );

            // let gbuffer0_desc = gbuffer0.desc_ref();
            // let group_count_x = gbuffer0_desc.width.div_ceil(8);
            // let group_count_y = gbuffer0_desc.height.div_ceil(8);
            // encoder.dispatch(group_count_x, group_count_y, 1);
        },
    );
}
