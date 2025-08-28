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
use aleph_rhi_api::*;

use crate::pass::lighting_resolve::LightingResolvePassOutput;
use crate::pass::{GraphArgs, GraphSwapImageInfo};
use crate::{IShaderAccessorExt, RenderPlaneOutput, StateCache, shaders};

struct TonemapPassPayload {
    input: ResourceRef,
    output: ResourceMut,
}

pub fn pass(
    frame_graph: &mut FrameGraphBuilder<GraphArgs>,
    device: &dyn IDevice,
    pin_board: &PinBoard,
    state_cache: &mut StateCache,
) -> RenderPlaneOutput {
    let block_layout = device
        .create_parameter_block_layout(&ParameterBlockDesc {
            params: &[
                ParameterType::Texture2D.param(),
                ParameterType::RWTexture2D.param(),
            ],
            visibility: DescriptorShaderVisibility::Compute,
            flags: Default::default(),
            name: obj_name_opt!("TonemapParameterBlockLayout"),
        })
        .unwrap();
    let binding_signature = device
        .create_binding_signature(
            &BindingSignatureDesc::new()
                .with_parameter_block_layouts(&[block_layout.as_ref()])
                .with_name(obj_name!("TonemapLightingBindingSignature")),
        )
        .unwrap();

    let shader_module = state_cache
        .shader_db()
        .load_data(shaders::postprocess::tonemapping_cs())
        .unwrap();

    let pipeline = device
        .create_compute_pipeline(&ComputePipelineDesc {
            shader_module,
            binding_signature: binding_signature.as_ref(),
            name: obj_name_opt!("TonemapPipeline"),
        })
        .unwrap();

    let mut result = None;

    frame_graph.add_pass(nstr!("TonemapPass"), |resources| {
        let back_buffer_info: &GraphSwapImageInfo = pin_board.get().unwrap();
        let b_desc = &back_buffer_info.desc;

        let lighting_resolve_pass: &LightingResolvePassOutput = pin_board.get().unwrap();
        let input = resources.read_texture(
            lighting_resolve_pass.lighting,
            ResourceUsageFlags::SHADER_RESOURCE,
        );

        let output_desc = TextureDesc::texture_2d(b_desc.width, b_desc.height)
            .with_format(Format::Bgra8Unorm)
            .with_name(obj_name!("TonemapOutput"));
        let output = resources.create_texture(
            &output_desc,
            // BarrierSync::COMPUTE_SHADING,
            ResourceUsageFlags::UNORDERED_ACCESS,
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
            let input_srv = ImageView::get_srv_for(device, input).unwrap();
            let desc =
                ImageViewDesc::uav_for_texture(device, output).with_format(Format::Bgra8Unorm); // Can't take UAV of SRGB formats
            let output_uav = device.get_texture_view(output, &desc).unwrap();

            let block = arena.allocate_block(block_layout.as_ref()).unwrap();
            let params = [
                TextureWrite::srv(input_srv).into(),
                TextureWrite::uav(output_uav).into(),
            ];
            device.update_parameter_block(block_layout.as_ref(), block, 0, &params);

            encoder.bind_compute_pipeline(&pipeline);
            encoder.bind_parameter_blocks(
                binding_signature.as_ref(),
                PipelineBindPoint::Compute,
                0,
                &[block],
            );

            let input_desc = device.get_texture_desc(input);
            let group_count_x = input_desc.width.div_ceil(8);
            let group_count_y = input_desc.height.div_ceil(8);
            encoder.dispatch(group_count_x, group_count_y, 1);
        }
    });

    result.unwrap()
}
