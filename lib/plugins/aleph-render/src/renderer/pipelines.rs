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

use dx12::dxgi;

///
/// The pipeline state object for the geometry pass
///
pub struct GeometryPipeline {
    pub pipeline_state: dx12::GraphicsPipelineState,
}

impl GeometryPipeline {
    pub fn new(
        device: &dx12::Device,
        root_signature: &dx12::RootSignature,
        vertex_shader: &[u8],
        pixel_shader: &[u8],
    ) -> Self {
        let rasterizer_state = dx12::RasterizerDesc::builder()
            .fill_mode(dx12::FillMode::Solid)
            .cull_mode(dx12::CullMode::Back)
            .front_counter_clockwise(true)
            .build();

        let depth_stencil_state = dx12::DepthStencilDesc::builder()
            .depth_enable(true)
            .depth_write_mask(dx12::DepthWriteMask::All)
            .depth_func(dx12::ComparisonFunc::Less)
            .build();

        let input_layout = [
            dx12::InputElementDesc {
                semantic_name: macros::cstr!("POSITION").into(),
                semantic_index: 0,
                format: dxgi::Format::R32G32B32Float,
                input_slot: 0,
                aligned_byte_offset: 0,
                input_slot_class: dx12::InputClassification::PerVertex,
                instance_data_step_rate: 0,
            },
            dx12::InputElementDesc {
                semantic_name: macros::cstr!("NORMAL").into(),
                semantic_index: 0,
                format: dxgi::Format::R32G32B32Float,
                input_slot: 0,
                aligned_byte_offset: 12,
                input_slot_class: dx12::InputClassification::PerVertex,
                instance_data_step_rate: 0,
            },
            dx12::InputElementDesc {
                semantic_name: macros::cstr!("TANGENT").into(),
                semantic_index: 0,
                format: dxgi::Format::R32G32B32A32Float,
                input_slot: 0,
                aligned_byte_offset: 24,
                input_slot_class: dx12::InputClassification::PerVertex,
                instance_data_step_rate: 0,
            },
            dx12::InputElementDesc {
                semantic_name: macros::cstr!("TEXCOORD").into(),
                semantic_index: 0,
                format: dxgi::Format::R32G32B32Float,
                input_slot: 0,
                aligned_byte_offset: 40,
                input_slot_class: dx12::InputClassification::PerVertex,
                instance_data_step_rate: 0,
            },
        ];

        let state_stream = dx12::GraphicsPipelineStateStream::builder()
            .root_signature(root_signature.clone())
            .vertex_shader(vertex_shader)
            .pixel_shader(pixel_shader)
            .sample_mask(u32::MAX)
            .rasterizer_state(rasterizer_state)
            .depth_stencil_state(depth_stencil_state)
            .input_layout(&input_layout)
            .primitive_topology_type(dx12::PrimitiveTopologyType::Triangle)
            .rtv_formats(&[dxgi::Format::B8G8R8A8UnormSRGB])
            .dsv_format(dxgi::Format::D24UnormS8Uint)
            .build();

        let pipeline_state = device
            .create_graphics_pipeline_state(&state_stream)
            .unwrap();

        Self { pipeline_state }
    }
}

///
/// The pipeline state object for the geometry pass
///
pub struct TonePipeline {
    pub pipeline_state: dx12::GraphicsPipelineState,
}

impl TonePipeline {
    pub fn new(
        device: &dx12::Device,
        root_signature: &dx12::RootSignature,
        vertex_shader: &[u8],
        pixel_shader: &[u8],
    ) -> Self {
        let rasterizer_state = dx12::RasterizerDesc::builder()
            .fill_mode(dx12::FillMode::Solid)
            .cull_mode(dx12::CullMode::None)
            .front_counter_clockwise(true)
            .build();

        let depth_stencil_state = dx12::DepthStencilDesc::builder()
            .depth_enable(false)
            .build();

        let input_layout = [dx12::InputElementDesc {
            semantic_name: macros::cstr!("POSITION").into(),
            semantic_index: 0,
            format: dxgi::Format::R32G32Float,
            input_slot: 0,
            aligned_byte_offset: 0,
            input_slot_class: dx12::InputClassification::PerVertex,
            instance_data_step_rate: 0,
        }];

        let state_stream = dx12::GraphicsPipelineStateStream::builder()
            .root_signature(root_signature.clone())
            .vertex_shader(vertex_shader)
            .pixel_shader(pixel_shader)
            .sample_mask(u32::MAX)
            .rasterizer_state(rasterizer_state)
            .depth_stencil_state(depth_stencil_state)
            .input_layout(&input_layout)
            .primitive_topology_type(dx12::PrimitiveTopologyType::Triangle)
            .rtv_formats(&[dxgi::Format::B8G8R8A8UnormSRGB])
            .build();

        let pipeline_state = device
            .create_graphics_pipeline_state(&state_stream)
            .unwrap();

        Self { pipeline_state }
    }
}
