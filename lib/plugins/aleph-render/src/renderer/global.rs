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

use crate::dx12;
use crate::dx12::dxgi;
use crate::dx12::D3D12Object;
use aleph_gpu_dx12::{IDeviceExt, IShaderExt};
use egui::ImageData;
use interfaces::any::AnyArc;
use interfaces::gpu::{ShaderOptions, ShaderType};
use std::ops::Deref;

/// Wraps d3d12 objects that don't ever need to be recreated
pub struct GlobalObjects {
    pub srv_heap: dx12::DescriptorHeap,
    pub root_signature: dx12::RootSignature,
    pub vertex_shader: AnyArc<dyn IShaderExt>,
    pub fragment_shader: AnyArc<dyn IShaderExt>,
    pub pipeline_state: dx12::GraphicsPipelineState,
    pub font_texture: FontTexture,
    pub swap_width: u32,
    pub swap_height: u32,
}

impl GlobalObjects {
    pub fn new(device: &dyn IDeviceExt, dimensions: (u32, u32)) -> Self {
        let descriptor_heap_desc = dx12::DescriptorHeapDesc::builder()
            .heap_type(dx12::DescriptorHeapType::CbvSrvUav)
            .num_descriptors(3)
            .flags(dx12::DescriptorHeapFlags::SHADER_VISIBLE)
            .build();
        let srv_heap = device
            .get_raw_handle()
            .create_descriptor_heap(&descriptor_heap_desc)
            .unwrap();
        srv_heap.set_name("egui::SRVHeap").unwrap();

        let root_signature = Self::create_root_signature(&device.get_raw_handle());
        root_signature.set_name("egui::RootSignature").unwrap();

        let vertex_shader = device
            .create_shader(&ShaderOptions {
                shader_type: ShaderType::Vertex,
                data: crate::shaders::egui_vert_shader(),
                entry_point: "main",
            })
            .unwrap()
            .query_interface::<dyn IShaderExt>()
            .unwrap();

        let fragment_shader = device
            .create_shader(&ShaderOptions {
                shader_type: ShaderType::Fragment,
                data: crate::shaders::egui_frag_shader(),
                entry_point: "main",
            })
            .unwrap()
            .query_interface::<dyn IShaderExt>()
            .unwrap();

        let pipeline_state = Self::create_pipeline_state(
            &device.get_raw_handle(),
            &root_signature,
            vertex_shader.deref(),
            fragment_shader.deref(),
        );
        pipeline_state
            .set_name("egui::GraphicsPipelineState")
            .unwrap();

        Self {
            srv_heap,
            root_signature,
            vertex_shader,
            fragment_shader,
            pipeline_state,
            font_texture: FontTexture {
                width: 256,
                height: 1,
                bytes: vec![255; 256],
                version: 1,
            },
            swap_width: dimensions.0,
            swap_height: dimensions.1,
        }
    }

    pub fn create_root_signature(device: &dx12::Device) -> dx12::RootSignature {
        let parameters = [
            dx12::RootParameter::DescriptorTable {
                visibility: dx12::ShaderVisibility::All,
                ranges: &[dx12::DescriptorRange {
                    range_type: dx12::DescriptorRangeType::SRV,
                    num_descriptors: 1,
                    base_shader_register: 0,
                    register_space: 0,
                    offset_in_descriptors_from_table_start: 0,
                }],
            },
            dx12::RootParameter::Constants {
                visibility: dx12::ShaderVisibility::All,
                constants: dx12::RootConstants {
                    shader_register: 0,
                    register_space: 0,
                    num32_bit_values: 2,
                },
            },
        ];
        let static_samplers = [dx12::StaticSamplerDesc::builder()
            .address_u(dx12::TextureAddressMode::Clamp)
            .address_v(dx12::TextureAddressMode::Clamp)
            .address_w(dx12::TextureAddressMode::Clamp)
            .shader_visibility(dx12::ShaderVisibility::All)
            .shader_register(0)
            .register_space(0)
            .build()];
        let desc_builder = dx12::RootSignatureDesc::builder()
            .parameters(&parameters)
            .static_samplers(&static_samplers)
            .flags(dx12::RootSignatureFlags::ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT);
        let desc = desc_builder.build();
        let desc = dx12::VersionedRootSignatureDesc::Desc(desc);
        let root_signature_blob = unsafe { dx12::RootSignatureBlob::new(&desc).unwrap() };
        device.create_root_signature(&root_signature_blob).unwrap()
    }

    pub fn create_pipeline_state(
        device: &dx12::Device,
        root_signature: &dx12::RootSignature,
        vertex_shader: &dyn IShaderExt,
        pixel_shader: &dyn IShaderExt,
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
                semantic_name: cstr::cstr!("A").into(),
                semantic_index: 0,
                format: dxgi::Format::R32G32Float,
                input_slot: 0,
                aligned_byte_offset: 0,
                input_slot_class: dx12::InputClassification::PerVertex,
                instance_data_step_rate: 0,
            },
            dx12::InputElementDesc {
                semantic_name: cstr::cstr!("A").into(),
                semantic_index: 1,
                format: dxgi::Format::R32G32Float,
                input_slot: 0,
                aligned_byte_offset: 8,
                input_slot_class: dx12::InputClassification::PerVertex,
                instance_data_step_rate: 0,
            },
            dx12::InputElementDesc {
                semantic_name: cstr::cstr!("A").into(),
                semantic_index: 2,
                format: dxgi::Format::R8G8B8A8Unorm,
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
            .vertex_shader(vertex_shader.get_raw_buffer())
            .pixel_shader(pixel_shader.get_raw_buffer())
            .sample_mask(u32::MAX)
            .blend_state(blend_desc)
            .rasterizer_state(rasterizer_state)
            .depth_stencil_state(depth_stencil_state)
            .input_layout(&input_layout)
            .primitive_topology_type(dx12::PrimitiveTopologyType::Triangle)
            .rtv_formats(&[dxgi::Format::B8G8R8A8UnormSRGB])
            .build();

        device
            .create_graphics_pipeline_state(&state_stream)
            .unwrap()
    }

    pub fn update_font_texture(&mut self, delta: &egui::epaint::ImageDelta) {
        fn coverage_mapper(v: &f32) -> u8 {
            // Function jigged from egui
            fn fast_round(r: f32) -> u8 {
                (r + 0.5).floor() as _ // rust does a saturating cast since 1.45
            }

            fast_round(v.powf(1.0 / 2.2) * 255.0)
        }

        // Increment the version to invalidate the cached textures on the GPU
        self.font_texture.version += 1;

        // We only support font images here so we panic if we get something else
        match &delta.image {
            ImageData::Font(font) => {
                // In the event of a whole update we need to re-allocate the texture as the size may have
                // increased.
                //
                // Partial updates patch the data in place
                if let Some(position) = &delta.pos {
                    // Handle a partial update
                    let x = position[0];
                    let y = position[1];
                    let w = font.size[0];
                    let h = font.size[1];

                    // Assert that we can't access the texture out of bounds based on the input we
                    // got.
                    assert!(x < self.font_texture.width);
                    assert!(y < self.font_texture.height);
                    assert!(x + w <= self.font_texture.width);
                    assert!(y + h <= self.font_texture.height);

                    // Assert that the buffers are big enough.
                    //
                    // We're trying to convince the optimizer that it can elide the bounds checks
                    // on array indexing.
                    assert!(
                        self.font_texture.bytes.len()
                            >= self.font_texture.width * self.font_texture.height
                    );
                    assert!(font.pixels.len() >= w * h);

                    // Iterate over each row
                    for d_row in 0..w {
                        // Transform our row in the delta pixels to our texture's pixel
                        let f_row = d_row + x;

                        // Iterate over all the columns in the current row
                        for d_col in 0..h {
                            // Transform our column in the delta pixels to our texture's pixels
                            let f_col = d_col + y;

                            // Calculate indices
                            let d_idx = d_row + d_col * w; // In delta tex
                            let f_idx = f_row + f_col * self.font_texture.width; // In our tex

                            // Copy and map our coverage sample into our font texture
                            self.font_texture.bytes[f_idx] = coverage_mapper(&font.pixels[d_idx]);
                        }
                    }
                } else {
                    // Handle a full update

                    // Just replace the old texture with the new data, mapped to u8
                    self.font_texture.width = delta.image.width();
                    self.font_texture.height = delta.image.height();
                    self.font_texture.bytes = font.pixels.iter().map(coverage_mapper).collect();
                }
            }
            _ => {
                unimplemented!()
            }
        }
    }
}

pub struct FontTexture {
    /// Width in pixels of the texture
    pub width: usize,

    /// Height in pixels of the texture
    pub height: usize,

    /// Raw data for the texture
    pub bytes: Vec<u8>,

    /// Version index that should be incremented every time the texture is changed so the per-frame
    /// data can detect when it needs to update
    pub version: usize,
}
