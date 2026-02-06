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
use aleph_nstr::nstr;
use aleph_pin_board::PinBoard;

use crate::internal::shaders;
use crate::renderer::frame_graph::{GraphArgs, GraphSwapImageInfo};
use crate::renderer::pass::lighting_resolve::LightingResolvePassOutput;
use crate::renderer::render_plane::RenderPlaneOutput;
use crate::renderer::shader_accessor::IShaderAccessorExt;
use crate::renderer::state_cache::StateCache;

struct TonemapPassPayload {
    input: ResourceRef,
    output: ResourceMut,
}

pub fn pass(
    frame_graph: &mut FrameGraphBuilder<GraphArgs>,
    device: &dyn rhi::IDevice,
    pin_board: &PinBoard,
    state_cache: &mut StateCache,
) -> RenderPlaneOutput {
    let shader_module = state_cache
        .shader_db()
        .load_stage(shaders::postprocess::tonemapping_cs())
        .unwrap();

    let mut params = Vec::new();
    params.resize_with(
        shader_module.get_parameter_count_for_block(0),
        Default::default,
    );
    shader_module.get_parameters_for_block(0, &mut params);
    let desc = rhi::ParameterBlockDesc {
        params: &params,
        visibility: shader_module.shader_type().into(),
        flags: Default::default(),
        name: rhi::obj_name_opt!("TonemapParameterBlockLayout"),
    };
    let block_layout = device.create_parameter_block_layout(&desc).unwrap();

    let parameter_block_layouts = [block_layout.as_ref()];
    let desc = rhi::BindingSignatureDesc::new()
        .with_parameter_block_layouts(&parameter_block_layouts)
        .with_name(rhi::obj_name!("TonemapLightingBindingSignature"));
    let binding_signature = device.create_binding_signature(&desc).unwrap();

    let pipeline = device
        .create_compute_pipeline(&rhi::ComputePipelineDesc {
            shader_module,
            binding_signature: binding_signature.as_ref(),
            name: rhi::obj_name_opt!("TonemapPipeline"),
        })
        .unwrap();

    let mut result = None;

    frame_graph.add_pass(nstr!("TonemapPass"), |resources| {
        let back_buffer_info: &GraphSwapImageInfo = pin_board.get().unwrap();
        let b_desc = &back_buffer_info.desc;

        let lighting_resolve_pass: &LightingResolvePassOutput = pin_board.get().unwrap();
        let input = resources.read_texture(
            lighting_resolve_pass.lighting,
            rhi::ResourceUsageFlags::SHADER_RESOURCE,
        );

        let output_desc = rhi::TextureDesc::texture_2d(b_desc.width, b_desc.height)
            .with_format(rhi::Format::Bgra8Unorm)
            .with_name(rhi::obj_name!("TonemapOutput"));
        let output = resources.create_texture(
            &output_desc,
            // BarrierSync::COMPUTE_SHADING,
            rhi::ResourceUsageFlags::UNORDERED_ACCESS,
        );
        result = Some(RenderPlaneOutput {
            id: output.into(),
            desc: output_desc.strip_name(),
        });

        let data = TonemapPassPayload { input, output };

        move |encoder, _graph, resources, _args| unsafe {
            let device = resources.device();
            let arena = resources.descriptor_arena();

            let input = resources.get_texture(data.input).unwrap();
            let output = resources.get_texture(data.output).unwrap();
            let input_srv = rhi::ImageView::get_srv_for(device, input).unwrap();
            let desc = rhi::ImageViewDesc::uav_for_texture(device, output)
                .with_format(rhi::Format::Bgra8Unorm); // Can't take UAV of SRGB formats
            let output_uav = device.get_texture_view(output, &desc).unwrap();

            let block = arena.allocate_block(block_layout.as_ref()).unwrap();
            let params = [
                rhi::TextureWrite::srv(input_srv).into(),
                rhi::TextureWrite::uav(output_uav).into(),
            ];
            device.update_parameter_block(block_layout.as_ref(), block, 0, &params);

            let mut compute = encoder.begin_compute(nstr!("TonemapPass::compute_pass"));
            compute.bind_compute_pipeline(&pipeline);
            compute.bind_parameter_blocks(binding_signature.as_ref(), 0, &[block]);

            let input_desc = device.get_texture_desc(input);
            let group_count_x = input_desc.width.div_ceil(8);
            let group_count_y = input_desc.height.div_ceil(8);
            compute.dispatch(group_count_x, group_count_y, 1);

            // End the compute pass explicitly
            drop(compute);
        }
    });

    result.unwrap()
}
