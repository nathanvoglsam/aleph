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

use crate::internal::shaders;
use crate::renderer::frame_graph::{GraphArgs, GraphSwapImageInfo};
use crate::renderer::render_plane::RenderPlaneOutput;
use crate::renderer::shader_accessor::{IShaderAccessor, IShaderAccessorExt};
use crate::renderer::state_cache::{IStateCacheKey, StateCache};

struct Payload {
    planes: Vec<ResourceRef>,
    target: ResourceMut,
}

pub fn pass(
    frame_graph: &mut FrameGraphBuilder<GraphArgs>,
    device: &dyn rhi::IDevice,
    pin_board: &PinBoard,
    state_cache: &mut StateCache,
    planes: &[RenderPlaneOutput],
) -> ResourceMut {
    let desc = pin_board
        .get::<GraphSwapImageInfo>()
        .unwrap()
        .desc
        .clone()
        .with_name(rhi::obj_name!("SwapImage"));

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
                    rhi::ResourceUsageFlags::SHADER_RESOURCE,
                )
            })
            .collect();

        let target = resources.import_texture(
            &TextureImportDesc {
                desc: &desc,
                before_sync: rhi::BarrierSync::NONE,
                before_access: rhi::BarrierAccess::NONE,
                before_layout: rhi::ImageLayout::Undefined,
                after_sync: rhi::BarrierSync::NONE,
                after_access: rhi::BarrierAccess::NONE,
                after_layout: rhi::ImageLayout::PresentSrc,
            },
            // BarrierSync::PIXEL_SHADING,
            rhi::ResourceUsageFlags::RENDER_TARGET,
        );
        result = Some(target);

        let data = Payload { planes, target };
        move |encoder, _graph, resources, _args| unsafe {
            let device = resources.device();

            let output = resources.get_texture(data.target).unwrap();
            let dst_desc = device.get_texture_desc(output);

            let dst_view = device
                .get_texture_rtv(
                    output,
                    &rhi::ImageViewDesc {
                        format: dst_desc.format,
                        view_type: rhi::ImageViewType::Tex2D,
                        sub_resources: rhi::TextureSubResourceSet::all(dst_desc),
                        writable: false,
                    },
                )
                .unwrap();

            let mut render = encoder.begin_rendering(
                &rhi::BeginRenderingInfo {
                    layer_count: 1,
                    extent: dst_desc.get_extent_2d(),
                    color_attachments: &[rhi::RenderingColorAttachmentInfo {
                        image_view: dst_view,
                        image_layout: rhi::ImageLayout::ColorAttachment,
                        load_op: rhi::AttachmentLoadOp::Clear(rhi::ColorClearValue::Int(0)),
                        store_op: rhi::AttachmentStoreOp::Store,
                    }],
                    depth_stencil_attachment: None,
                    allow_uav_writes: false,
                },
                nstr!("CompositePlanes::render_pass"),
            );
            render.bind_graphics_pipeline(&state.pipeline);
            render.set_viewports(&[rhi::Viewport {
                x: 0.0,
                y: 0.0,
                width: dst_desc.width as _,
                height: dst_desc.height as _,
                min_depth: 0.0,
                max_depth: 1.0,
            }]);
            render.set_scissor_rects(&[rhi::Rect {
                x: 0,
                y: 0,
                w: dst_desc.width,
                h: dst_desc.height,
            }]);

            for plane in data.planes.iter().copied() {
                let plane = resources.get_texture(plane).unwrap();
                let src_desc = device.get_texture_desc(plane);

                let src_view = device
                    .get_texture_view(
                        plane,
                        &rhi::ImageViewDesc {
                            format: src_desc.format,
                            view_type: rhi::ImageViewType::Tex2D,
                            sub_resources: rhi::TextureSubResourceSet::all(src_desc),
                            writable: false,
                        },
                    )
                    .unwrap();

                let block_layout = state.layout.block_layout.as_ref();
                let block = resources
                    .descriptor_arena()
                    .allocate_block(block_layout)
                    .unwrap();
                let params = [
                    rhi::TextureWrite::srv(src_view).into(),
                    rhi::SamplerWrite::new(&state.layout.sampler).into(),
                ];
                resources
                    .device()
                    .update_parameter_block(block_layout, block, 0, &params);

                let level = 0.0f32;
                render.set_push_constant_block(bytemuck::bytes_of(&level));

                render.bind_parameter_blocks(state.layout.binding_signature.as_ref(), 0, &[block]);

                render.draw(3, 1, 0, 0);
            }

            // End the compute pass explicitly
            drop(render);
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
    pub block_layout: Arc<dyn rhi::IParameterBlockLayout>,
    pub sampler: rhi::SamplerHandle,
    pub binding_signature: Arc<dyn rhi::IBindingSignature>,
}

impl CompositePlanesLayout {
    pub fn key() -> CompositePlanesLayoutKey {
        CompositePlanesLayoutKey
    }

    pub fn new(device: &dyn rhi::IDevice) -> Self {
        let sampler = Self::create_sampler(device);
        let block_layout = Self::create_block_layout(device);
        let binding_signature = Self::create_binding_signature(device, block_layout.as_ref());

        Self {
            block_layout,
            sampler,
            binding_signature,
        }
    }

    pub fn create_block_layout(device: &dyn rhi::IDevice) -> Arc<dyn rhi::IParameterBlockLayout> {
        let desc = rhi::ParameterBlockDesc {
            params: &[
                rhi::ParameterType::Texture2D.param(),
                rhi::ParameterType::SamplerState.param(),
            ],
            visibility: rhi::DescriptorShaderVisibility::Fragment,
            flags: rhi::ParameterBlockFlags::default(),
            name: rhi::obj_name_opt!("ParameterBlockLayout"),
        };
        device.create_parameter_block_layout(&desc).unwrap()
    }

    pub fn create_binding_signature(
        device: &dyn rhi::IDevice,
        block_layout: &dyn rhi::IParameterBlockLayout,
    ) -> Arc<dyn rhi::IBindingSignature> {
        let desc = rhi::BindingSignatureDesc {
            parameter_block_layouts: &[block_layout],
            push_constant_block: rhi::PushConstantBlock::new(4),
            name: rhi::obj_name_opt!("BindingSignature"),
        };
        device.create_binding_signature(&desc).unwrap()
    }

    pub fn create_sampler(device: &dyn rhi::IDevice) -> rhi::SamplerHandle {
        let desc = rhi::SamplerDesc {
            min_filter: rhi::SamplerFilter::Linear,
            mag_filter: rhi::SamplerFilter::Linear,
            mip_filter: rhi::SamplerMipFilter::Nearest,
            address_mode_u: rhi::SamplerAddressMode::Clamp,
            address_mode_v: rhi::SamplerAddressMode::Clamp,
            address_mode_w: rhi::SamplerAddressMode::Clamp,
            ..Default::default()
        };
        device.create_sampler(&desc).unwrap()
    }
}

#[derive(PartialEq, Eq, Hash)]
pub struct CompositePlanesStateKey(pub rhi::Format);

impl IStateCacheKey for CompositePlanesStateKey {
    type Storage = CompositePlanesState;
}

pub struct CompositePlanesState {
    pub layout: Arc<CompositePlanesLayout>,
    pub pipeline: rhi::GraphicsPipelineHandle,
}

impl CompositePlanesState {
    pub fn key(format: rhi::Format) -> CompositePlanesStateKey {
        CompositePlanesStateKey(format)
    }

    pub fn new(cache: &mut StateCache, device: &dyn rhi::IDevice, format: rhi::Format) -> Self {
        let key = CompositePlanesLayout::key();
        let layout = cache.get_or_insert_with(&key, |_, _| CompositePlanesLayout::new(device));

        let pipeline = Self::create_pipeline_state(
            device,
            layout.binding_signature.as_ref(),
            cache.shader_db(),
            format,
        );

        Self { layout, pipeline }
    }

    pub fn create_pipeline_state(
        device: &dyn rhi::IDevice,
        binding_signature: &dyn rhi::IBindingSignature,
        shader_db: &dyn IShaderAccessor,
        format: rhi::Format,
    ) -> rhi::GraphicsPipelineHandle {
        let vertex_shader = shader_db
            .load_stage(shaders::composite_planes::vert())
            .unwrap();
        let fragment_shader = shader_db
            .load_stage(shaders::composite_planes::frag())
            .unwrap();

        let vertex_layout = rhi::VertexInputStateDesc::default();

        let input_assembly_state = rhi::InputAssemblyStateDesc {
            primitive_topology: rhi::PrimitiveTopology::TriangleList,
        };

        let rasterizer_state = rhi::RasterizerStateDesc {
            cull_mode: rhi::CullMode::None,
            front_face: rhi::FrontFaceOrder::CounterClockwise,
            polygon_mode: rhi::PolygonMode::Fill,
            depth_bias: 0,
            depth_bias_clamp: 0.0,
            depth_bias_slope_factor: 0.0,
        };

        let depth_stencil_state = rhi::DepthStencilStateDesc {
            depth_test: false,
            ..Default::default()
        };

        let blend_state_new = rhi::BlendStateDesc {
            attachments: &[rhi::AttachmentBlendState {
                blend_enabled: true,
                src_factor: rhi::BlendFactor::One,
                dst_factor: rhi::BlendFactor::OneMinusSrcAlpha,
                blend_op: rhi::BlendOp::Add,
                alpha_src_factor: rhi::BlendFactor::OneMinusDstAlpha,
                alpha_dst_factor: rhi::BlendFactor::One,
                alpha_blend_op: rhi::BlendOp::Add,
                color_write_mask: rhi::ColorComponentFlags::all(),
            }],
        };

        let graphics_pipeline_desc_new = rhi::GraphicsPipelineDesc {
            shader_stages: &[vertex_shader, fragment_shader],
            binding_signature,
            vertex_layout: &vertex_layout,
            input_assembly_state: &input_assembly_state,
            rasterizer_state: &rasterizer_state,
            depth_stencil_state: &depth_stencil_state,
            blend_state: &blend_state_new,
            render_target_formats: &[format],
            depth_stencil_format: None,
            name: rhi::obj_name_opt!("GraphicsPipelineState"),
        };

        device
            .create_graphics_pipeline(&graphics_pipeline_desc_new)
            .unwrap()
    }
}
