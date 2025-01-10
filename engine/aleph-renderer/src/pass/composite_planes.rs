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

use std::sync::Arc;

use aleph_frame_graph::*;
use aleph_nstr::nstr;
use aleph_pin_board::PinBoard;
use aleph_rhi_api::*;

use crate::pass::{GraphArgs, GraphSwapImageInfo};
use crate::{shaders, IStateCacheKey, RenderPlaneOutput, ShaderDatabaseAccessor, StateCache};

struct Payload {
    planes: Vec<ResourceRef>,
    target: ResourceMut,
}

pub fn pass(
    frame_graph: &mut FrameGraphBuilder<GraphArgs>,
    device: &dyn IDevice,
    pin_board: &PinBoard,
    state_cache: &mut StateCache,
    planes: &[RenderPlaneOutput],
) -> ResourceMut {
    let desc = pin_board
        .get::<GraphSwapImageInfo>()
        .unwrap()
        .desc
        .clone()
        .with_name(obj_name!("SwapImage"));

    let key = CompositePlanesState::key(desc.format);
    let state = state_cache.get_or_insert_with(&key, |cache, k| {
        CompositePlanesState::new(cache, device, k.0)
    });

    let mut result = None;

    frame_graph.add_pass(nstr!("CompositePlanes"), |resources| {
        let planes: Vec<_> = planes
            .iter()
            .map(|v| {
                resources.read_texture(
                    v.id,
                    // BarrierSync::PIXEL_SHADING,
                    ResourceUsageFlags::SHADER_RESOURCE,
                )
            })
            .collect();

        let target = resources.import_texture(
            &TextureImportDesc {
                desc: &desc,
                before_sync: BarrierSync::NONE,
                before_access: BarrierAccess::NONE,
                before_layout: ImageLayout::Undefined,
                after_sync: BarrierSync::NONE,
                after_access: BarrierAccess::NONE,
                after_layout: ImageLayout::PresentSrc,
            },
            // BarrierSync::PIXEL_SHADING,
            ResourceUsageFlags::RENDER_TARGET,
        );
        result = Some(target);

        let data = Payload { planes, target };
        move |encoder, _graph, resources, _args| unsafe {
            let device = resources.device();

            let output = resources.get_texture(data.target).unwrap();
            let dst_desc = device.texture_desc_ref(output);

            let dst_view = device
                .get_texture_rtv(
                    output,
                    &ImageViewDesc {
                        format: dst_desc.format,
                        view_type: ImageViewType::Tex2D,
                        sub_resources: TextureSubResourceSet::all(dst_desc),
                        writable: false,
                    },
                )
                .unwrap();

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
            encoder.bind_graphics_pipeline(&state.pipeline);
            encoder.set_viewports(&[Viewport {
                x: 0.0,
                y: 0.0,
                width: dst_desc.width as _,
                height: dst_desc.height as _,
                min_depth: 0.0,
                max_depth: 1.0,
            }]);
            encoder.set_scissor_rects(&[Rect {
                x: 0,
                y: 0,
                w: dst_desc.width,
                h: dst_desc.height,
            }]);

            for plane in data.planes.iter().copied() {
                let plane = resources.get_texture(plane).unwrap();
                let src_desc = device.texture_desc_ref(plane);

                let src_view = device
                    .get_texture_view(
                        plane,
                        &ImageViewDesc {
                            format: src_desc.format,
                            view_type: ImageViewType::Tex2D,
                            sub_resources: TextureSubResourceSet::all(src_desc),
                            writable: false,
                        },
                    )
                    .unwrap();
                let set = resources
                    .descriptor_arena()
                    .allocate_set(&state.layout.set_layout)
                    .unwrap();
                resources
                    .device()
                    .update_descriptor_sets(&[DescriptorWriteDesc {
                        set,
                        binding: 0,
                        array_element: 0,
                        writes: DescriptorWrites::Texture(&[ImageDescriptorWrite::srv(src_view)]),
                    }]);
                let level = 0.0f32;
                encoder.set_push_constant_block(0, bytemuck::bytes_of(&level));

                encoder.bind_descriptor_sets(
                    &state.layout.pipeline_layout,
                    PipelineBindPoint::Graphics,
                    0,
                    &[set],
                    &[],
                );

                encoder.draw(3, 1, 0, 0);
            }

            encoder.end_rendering();
        }
    });

    result.unwrap()
}

#[derive(PartialEq, Eq, Hash)]
pub struct CompositePlanesLayoutKey;

impl IStateCacheKey for CompositePlanesLayoutKey {
    type Storage = CompositePlanesLayout;
}

pub struct CompositePlanesLayout {
    pub set_layout: DescriptorSetLayoutHandle,
    pub pipeline_layout: PipelineLayoutHandle,
}

impl CompositePlanesLayout {
    pub fn key() -> CompositePlanesLayoutKey {
        CompositePlanesLayoutKey
    }

    pub fn new(device: &dyn IDevice) -> Self {
        let sampler = Self::create_sampler(device);
        let set_layout = Self::create_set_layout(device, &sampler);
        let pipeline_layout = Self::create_pipeline_layout(device, &set_layout);

        Self {
            set_layout: set_layout,
            pipeline_layout,
        }
    }

    pub fn create_set_layout(
        device: &dyn IDevice,
        sampler: &SamplerHandle,
    ) -> DescriptorSetLayoutHandle {
        let sampler = [sampler];
        let descriptor_set_layout_desc = DescriptorSetLayoutDesc {
            visibility: DescriptorShaderVisibility::Fragment,
            items: &[
                DescriptorType::Texture.binding(0),
                DescriptorType::Sampler
                    .binding(1)
                    .with_static_samplers(&sampler),
            ],
            name: obj_name_opt!("DescriptorSetLayout"),
        };
        device
            .create_descriptor_set_layout(&descriptor_set_layout_desc)
            .unwrap()
    }

    pub fn create_pipeline_layout(
        device: &dyn IDevice,
        set_layout: &DescriptorSetLayoutHandle,
    ) -> PipelineLayoutHandle {
        let pipeline_layout_desc = PipelineLayoutDesc {
            set_layouts: &[set_layout],
            push_constant_blocks: &[PushConstantBlock {
                binding: 0,
                visibility: DescriptorShaderVisibility::All,
                size: 4,
            }],
            name: obj_name_opt!("PipelineLayout"),
        };
        device
            .create_pipeline_layout(&pipeline_layout_desc)
            .unwrap()
    }

    pub fn create_sampler(device: &dyn IDevice) -> SamplerHandle {
        let desc = SamplerDesc {
            min_filter: SamplerFilter::Linear,
            mag_filter: SamplerFilter::Linear,
            mip_filter: SamplerMipFilter::Nearest,
            address_mode_u: SamplerAddressMode::Clamp,
            address_mode_v: SamplerAddressMode::Clamp,
            address_mode_w: SamplerAddressMode::Clamp,
            ..Default::default()
        };
        device.create_sampler(&desc).unwrap()
    }
}

#[derive(PartialEq, Eq, Hash)]
pub struct CompositePlanesStateKey(pub Format);

impl IStateCacheKey for CompositePlanesStateKey {
    type Storage = CompositePlanesState;
}

pub struct CompositePlanesState {
    pub layout: Arc<CompositePlanesLayout>,
    pub pipeline: GraphicsPipelineHandle,
}

impl CompositePlanesState {
    pub fn key(format: Format) -> CompositePlanesStateKey {
        CompositePlanesStateKey(format)
    }

    pub fn new(cache: &mut StateCache, device: &dyn IDevice, format: Format) -> Self {
        let key = CompositePlanesLayout::key();
        let layout = cache.get_or_insert_with(&key, |_, _| CompositePlanesLayout::new(device));

        let pipeline =
            Self::create_pipeline_state(device, &layout.pipeline_layout, cache.shader_db(), format);

        Self { layout, pipeline }
    }

    pub fn create_pipeline_state(
        device: &dyn IDevice,
        pipeline_layout: &PipelineLayoutHandle,
        shader_db: &ShaderDatabaseAccessor,
        format: Format,
    ) -> GraphicsPipelineHandle {
        let vertex_shader = shader_db
            .load_stage(shaders::composite_planes::vert())
            .unwrap();
        let fragment_shader = shader_db
            .load_stage(shaders::composite_planes::frag())
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
            shader_stages: &[vertex_shader, fragment_shader],
            pipeline_layout,
            vertex_layout: &vertex_layout,
            input_assembly_state: &input_assembly_state,
            rasterizer_state: &rasterizer_state,
            depth_stencil_state: &depth_stencil_state,
            blend_state: &blend_state_new,
            render_target_formats: &[format],
            depth_stencil_format: None,
            name: obj_name_opt!("GraphicsPipelineState"),
        };

        device
            .create_graphics_pipeline(&graphics_pipeline_desc_new)
            .unwrap()
    }
}
