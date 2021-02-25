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

use crate::renderer::egui::constant::ConstantObjects;
use dx12::{dxgi, D3D12Object};

///
/// This represents resources where only one is needed, but they need to be recreated when the
/// swapchain is rebuilt
///
pub struct GlobalObjects {
    pub pipeline_state: dx12::GraphicsPipelineState,
}

impl GlobalObjects {
    pub fn init(device: &dx12::Device, constant: &ConstantObjects) -> Self {
        let pipeline_state = Self::create_pipeline_state(
            device,
            &constant.root_signature,
            embedded_data::shaders::egui_vert_shader(),
            embedded_data::shaders::egui_frag_shader(),
        );

        Self { pipeline_state }
    }

    pub fn create_pipeline_state(
        device: &dx12::Device,
        root_signature: &dx12::RootSignature,
        vertex_shader: &[u8],
        pixel_shader: &[u8],
    ) -> dx12::GraphicsPipelineState {
        let rasterizer_state = dx12::RasterizerDesc::builder()
            .fill_mode(dx12::FillMode::Solid)
            .cull_mode(dx12::CullMode::None)
            .front_counter_clockwise(true)
            .build();

        let depth_stencil_state = dx12::DepthStencilDesc::builder()
            .depth_enable(false)
            .build();

        let input_layout = [
            dx12::InputElementDesc {
                semantic_name: macros::cstr!("POSITION"),
                semantic_index: 0,
                format: dxgi::Format::R32G32Float,
                input_slot: 0,
                aligned_byte_offset: 0,
                input_slot_class: dx12::InputClassification::PerVertex,
                instance_data_step_rate: 0,
            },
            dx12::InputElementDesc {
                semantic_name: macros::cstr!("TEXCOORD"),
                semantic_index: 0,
                format: dxgi::Format::R32G32Float,
                input_slot: 0,
                aligned_byte_offset: 8,
                input_slot_class: dx12::InputClassification::PerVertex,
                instance_data_step_rate: 0,
            },
            dx12::InputElementDesc {
                semantic_name: macros::cstr!("COLOR"),
                semantic_index: 0,
                format: dxgi::Format::R32G32B32A32Float,
                input_slot: 0,
                aligned_byte_offset: 16,
                input_slot_class: dx12::InputClassification::PerVertex,
                instance_data_step_rate: 0,
            },
        ];

        let blend = dx12::RenderTargetBlendDesc::builder()
            .blend_enable(true)
            .logic_op_enable(false)
            .src_blend(dx12::Blend::One)
            .dest_blend(dx12::Blend::SrcAlphaInv)
            .blend_op(dx12::BlendOp::Add)
            .src_blend_alpha(dx12::Blend::DestAlphaInv)
            .dest_blend_alpha(dx12::Blend::One)
            .blend_op_alpha(dx12::BlendOp::Add)
            .build();
        let blend_desc = dx12::BlendDesc::builder()
            .alpha_to_coverage_enable(false)
            .independent_blend_enable(false)
            .render_targets(&[blend])
            .build();

        let state_stream = dx12::GraphicsPipelineStateStream::builder()
            .root_signature(root_signature.clone())
            .vertex_shader(vertex_shader)
            .pixel_shader(pixel_shader)
            .sample_mask(u32::MAX)
            .blend_state(blend_desc)
            .rasterizer_state(rasterizer_state)
            .depth_stencil_state(depth_stencil_state)
            .input_layout(&input_layout)
            .primitive_topology_type(dx12::PrimitiveTopologyType::Triangle)
            .rtv_formats(&[dxgi::Format::R8G8B8A8UnormSRGB])
            .dsv_format(dxgi::Format::D32Float)
            .build();

        let pipeline_state = device
            .create_graphics_pipeline_state(&state_stream)
            .unwrap();

        pipeline_state.set_name("egui::GraphicsPipelineState");

        pipeline_state
    }
}
