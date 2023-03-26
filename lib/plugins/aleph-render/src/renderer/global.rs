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

use egui::ImageData;
use interfaces::any::AnyArc;
use interfaces::gpu::*;
use std::ops::Deref;

/// Wraps d3d12 objects that don't ever need to be recreated
pub struct GlobalObjects {
    pub sampler: AnyArc<dyn ISampler>,
    pub descriptor_set_layout: AnyArc<dyn IDescriptorSetLayout>,
    pub pipeline_layout: AnyArc<dyn IPipelineLayout>,
    pub vertex_shader: AnyArc<dyn IShader>,
    pub fragment_shader: AnyArc<dyn IShader>,
    pub graphics_pipeline: AnyArc<dyn IGraphicsPipeline>,
    pub font_texture: FontTexture,
    pub swap_width: u32,
    pub swap_height: u32,
}

impl GlobalObjects {
    pub fn new(device: &dyn IDevice, dimensions: (u32, u32)) -> Self {
        let sampler = Self::create_sampler(device);
        let descriptor_set_layout = Self::create_descriptor_set_layout(device, sampler.deref());
        let pipeline_layout = Self::create_root_signature(device, descriptor_set_layout.deref());

        let (vertex_data, fragment_data) = match device.get_backend_api() {
            BackendAPI::Vulkan => (
                crate::shaders::egui_vert_shader_vk(),
                crate::shaders::egui_frag_shader_vk(),
            ),
            BackendAPI::D3D12 => (
                crate::shaders::egui_vert_shader_dx(),
                crate::shaders::egui_frag_shader_dx(),
            ),
        };
        let vertex_shader = device
            .create_shader(&ShaderOptions {
                shader_type: ShaderType::Vertex,
                data: vertex_data,
                entry_point: "main",
                name: Some("egui::VertexShader"),
            })
            .unwrap();

        let fragment_shader = device
            .create_shader(&ShaderOptions {
                shader_type: ShaderType::Fragment,
                data: fragment_data,
                entry_point: "main",
                name: Some("egui::FragmentShader"),
            })
            .unwrap();

        let graphics_pipeline = Self::create_pipeline_state(
            device,
            pipeline_layout.deref(),
            vertex_shader.deref(),
            fragment_shader.deref(),
        );

        Self {
            sampler,
            descriptor_set_layout,
            pipeline_layout,
            vertex_shader,
            fragment_shader,
            graphics_pipeline,
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

    pub fn create_sampler(device: &dyn IDevice) -> AnyArc<dyn ISampler> {
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

    pub fn create_descriptor_set_layout(
        device: &dyn IDevice,
        sampler: &dyn ISampler,
    ) -> AnyArc<dyn IDescriptorSetLayout> {
        let samplers = [sampler];
        let descriptor_set_layout_desc = DescriptorSetLayoutDesc {
            visibility: DescriptorShaderVisibility::All,
            items: &[
                DescriptorSetLayoutBinding {
                    binding_num: 0,
                    binding_type: DescriptorType::SampledImage,
                    binding_count: None,
                    ..Default::default()
                },
                DescriptorSetLayoutBinding {
                    binding_num: 1,
                    binding_type: DescriptorType::Sampler,
                    binding_count: None,
                    static_samplers: Some(&samplers),
                    ..Default::default()
                },
            ],
            name: Some("egui::DescriptorSetLayout"),
        };
        device
            .create_descriptor_set_layout(&descriptor_set_layout_desc)
            .unwrap()
    }

    pub fn create_root_signature(
        device: &dyn IDevice,
        descriptor_set_layout: &dyn IDescriptorSetLayout,
    ) -> AnyArc<dyn IPipelineLayout> {
        let pipeline_layout_desc = PipelineLayoutDesc {
            set_layouts: &[descriptor_set_layout],
            push_constant_blocks: &[PushConstantBlock {
                binding: 0,
                visibility: DescriptorShaderVisibility::All,
                size: 8,
            }],
            name: Some("egui::RootSignature"),
        };
        device
            .create_pipeline_layout(&pipeline_layout_desc)
            .unwrap()
    }

    pub fn create_pipeline_state(
        device: &dyn IDevice,
        pipeline_layout: &dyn IPipelineLayout,
        vertex_shader: &dyn IShader,
        pixel_shader: &dyn IShader,
    ) -> AnyArc<dyn IGraphicsPipeline> {
        let rasterizer_state_new = RasterizerStateDesc {
            cull_mode: CullMode::None,
            front_face: FrontFaceOrder::CounterClockwise,
            polygon_mode: PolygonMode::Fill,
        };

        let depth_stencil_state_new = DepthStencilStateDesc {
            depth_test: false,
            ..Default::default()
        };

        let vertex_layout_new = VertexInputStateDesc {
            input_bindings: &[VertexInputBindingDesc {
                binding: 0,
                stride: 20,
                input_rate: VertexInputRate::PerVertex,
            }],
            input_attributes: &[
                VertexInputAttributeDesc {
                    location: 0,
                    binding: 0,
                    format: Format::Rg32Float,
                    offset: 0,
                },
                VertexInputAttributeDesc {
                    location: 1,
                    binding: 0,
                    format: Format::Rg32Float,
                    offset: 8,
                },
                VertexInputAttributeDesc {
                    location: 2,
                    binding: 0,
                    format: Format::Rgba8Unorm,
                    offset: 16,
                },
            ],
        };

        let input_assembly_state_new = InputAssemblyStateDesc {
            primitive_topology: PrimitiveTopology::TriangleList,
        };

        let blend_state_new = BlendStateDesc {
            attachments: &[AttachmentBlendState {
                blend_enabled: true,
                src_factor: BlendFactor::One,
                dst_factor: BlendFactor::OneMinusSrcAlpha,
                blend_op: BlendOp::Add,
                alpha_src_factor: BlendFactor::OneMinusDstAlpha,
                alpha_dst_factor: BlendFactor::One,
                alpha_blend_op: BlendOp::Add,
                color_write_mask: ColorComponentFlags::all(),
            }],
        };

        let graphics_pipeline_desc_new = GraphicsPipelineDesc {
            shader_stages: &[vertex_shader, pixel_shader],
            pipeline_layout,
            vertex_layout: &vertex_layout_new,
            input_assembly_state: &input_assembly_state_new,
            rasterizer_state: &rasterizer_state_new,
            depth_stencil_state: &depth_stencil_state_new,
            blend_state: &blend_state_new,
            render_target_formats: &[Format::Bgra8UnormSrgb],
            depth_stencil_format: None,
            name: Some("egui::GraphicsPipelineState"),
        };

        device
            .create_graphics_pipeline(&graphics_pipeline_desc_new)
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
