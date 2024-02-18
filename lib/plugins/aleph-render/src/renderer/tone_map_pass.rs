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

use crate::renderer::lighting_resolve_pass::LightingResolvePassOutput;
use crate::renderer::params::BackBufferInfo;
use crate::shader_db_accessor::ShaderDatabaseAccessor;
use crate::shaders;

struct TonemapPassPayload {
    input: ResourceRef,
    output: ResourceMut,
    set_layout: AnyArc<dyn IDescriptorSetLayout>,
    pipeline_layout: AnyArc<dyn IPipelineLayout>,
    pipeline: AnyArc<dyn IComputePipeline>,
}

pub struct TonemapPassOutput {
    pub output: ResourceMut,
}

pub fn pass(
    frame_graph: &mut FrameGraphBuilder,
    device: &dyn IDevice,
    pin_board: &PinBoard,
    shader_db: &ShaderDatabaseAccessor,
) {
    let set_layout = device
        .create_descriptor_set_layout(&DescriptorSetLayoutDesc {
            visibility: DescriptorShaderVisibility::Compute,
            items: &[
                DescriptorType::Texture.binding(0),
                DescriptorType::TextureRW.binding(1),
            ],
            name: Some("TonemapDescriptorSetLayout"),
        })
        .unwrap();
    let pipeline_layout = device
        .create_pipeline_layout(
            &PipelineLayoutDesc::new()
                .with_set_layouts(&[set_layout.as_ref()])
                .with_name("TonemapLightingPipelineLayout"),
        )
        .unwrap();

    let shader_module = shader_db
        .load_data(shaders::aleph_render::postprocess::tonemapping_cs())
        .unwrap();

    let pipeline = device
        .create_compute_pipeline(&ComputePipelineDesc {
            shader_module,
            pipeline_layout: pipeline_layout.as_ref(),
            name: Some("TonemapPipeline"),
        })
        .unwrap();

    frame_graph.add_pass(
        "TonemapPass",
        |data: &mut Payload<TonemapPassPayload>, resources| {
            let back_buffer_info: &BackBufferInfo = pin_board.get().unwrap();
            let b_desc = &back_buffer_info.desc;

            let lighting_resolve_pass: &LightingResolvePassOutput = pin_board.get().unwrap();

            let input = resources.read_texture(
                lighting_resolve_pass.lighting,
                ResourceUsageFlags::SHADER_RESOURCE,
            );
            let output_desc = TextureDesc::texture_2d(b_desc.width, b_desc.height)
                .with_format(Format::Bgra8Unorm)
                .with_name("TonemapOutput");
            let output =
                resources.create_texture(&output_desc, ResourceUsageFlags::UNORDERED_ACCESS);

            data.write(TonemapPassPayload {
                input,
                output,
                set_layout,
                pipeline_layout,
                pipeline,
            });
            pin_board.publish(TonemapPassOutput { output });
        },
        |data, encoder, resources| unsafe {
            // Unwrap all our fg resources from our setup payload
            let data = data.unwrap();

            let device = resources.device();
            let arena = resources.descriptor_arena();

            let input = resources.get_texture(data.input).unwrap();
            let output = resources.get_texture(data.output).unwrap();
            let input_srv = ImageView::get_srv_for(input).unwrap();
            let output_uav = ImageView::get_uav_for(output).unwrap();

            let set = arena.allocate_set(data.set_layout.as_ref()).unwrap();
            device.update_descriptor_sets(&[
                DescriptorWriteDesc::texture(set, 0, &input_srv.srv_write()),
                DescriptorWriteDesc::texture_rw(set, 1, &output_uav.uav_write()),
            ]);

            encoder.bind_compute_pipeline(data.pipeline.as_ref());
            encoder.bind_descriptor_sets(
                data.pipeline_layout.as_ref(),
                PipelineBindPoint::Compute,
                0,
                &[set],
                &[],
            );

            let input_desc = input.desc_ref();
            let group_count_x = input_desc.width.div_ceil(8);
            let group_count_y = input_desc.height.div_ceil(8);
            encoder.dispatch(group_count_x, group_count_y, 1);
        },
    );
}
