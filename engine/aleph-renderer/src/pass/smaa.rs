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

use crate::pass::GraphArgs;
use crate::pass::utils::{
    FullscreenTriangleBindInfo, FullscreenTriangleInfo, create_fullscreen_triangle_pipeline,
    draw_fullscreen_triangle,
};
use crate::{
    DefaultResources, IStateCacheKey, RenderPlaneOutput, ShaderDatabaseAccessor, StateCache,
    TextureHandle, shaders,
};

pub fn pass(
    frame_graph: &mut FrameGraphBuilder<GraphArgs>,
    device: &dyn IDevice,
    _pin_board: &PinBoard,
    state_cache: &mut StateCache,
    default_resources: &DefaultResources,
    colour_input: &RenderPlaneOutput,
) -> RenderPlaneOutput {
    let key = SmaaState::key(colour_input.desc.format);
    let state_handle =
        state_cache.get_or_insert_with(&key, |cache, k| SmaaState::new(cache, device, k.0));

    let area_tex = default_resources.smaa_area_tex();
    let search_tex = default_resources.smaa_search_tex();

    //______________________________________________________________________________________________
    // 1. Edge Texture
    let state = state_handle.clone();
    let edge_texture = edge_pass(frame_graph, state, colour_input);

    //______________________________________________________________________________________________
    // 2. Weights Texture
    let state = state_handle.clone();
    let blend_texture = blend_weight_pass(frame_graph, state, area_tex, search_tex, &edge_texture);

    //______________________________________________________________________________________________
    // 3. AA Resolve
    let state = state_handle.clone();
    let out_texture = aa_blend_resolve_pass(frame_graph, state, &colour_input, &blend_texture);

    out_texture
}

fn edge_pass(
    frame_graph: &mut FrameGraphBuilder<GraphArgs>,
    state: Arc<SmaaState>,
    colour_input: &RenderPlaneOutput,
) -> RenderPlaneOutput {
    let extent = colour_input.desc.get_extent_2d();

    let mut edge_texture = None;
    frame_graph.add_pass(nstr!("SmaaEdgeDetection"), |resources| {
        let colour_input =
            resources.read_texture(colour_input.id, ResourceUsageFlags::SHADER_RESOURCE);

        let edge_desc = TextureDesc::texture_2d(extent.width, extent.height)
            .with_format(Format::Bgra8Unorm)
            .with_name(obj_name!("SmaaEdgesTexture"));
        let edge_tex = resources.create_texture(
            &edge_desc,
            // BarrierSync::RENDER_TARGET,
            ResourceUsageFlags::RENDER_TARGET,
        );
        edge_texture = Some(RenderPlaneOutput {
            id: edge_tex.into(),
            desc: edge_desc.strip_name(),
        });

        move |encoder, _graph, resources, _args| unsafe {
            let device = resources.device();

            let edge_tex = resources.get_texture(edge_tex).unwrap();
            let edge_tex_view = ImageView::get_rtv_for(device, edge_tex).unwrap();

            let colour_input = resources.get_texture(colour_input).unwrap();
            let colour_input_view = ImageView::get_srv_for(device, colour_input).unwrap();

            let set = resources
                .descriptor_arena()
                .allocate_set(&state.edge_set_layout)
                .unwrap();
            resources
                .device()
                .update_descriptor_sets(&[DescriptorWriteDesc::texture(
                    set,
                    0,
                    &colour_input_view.srv_write(),
                )]);

            let metrics = metrics(extent);

            let info = FullscreenTriangleInfo {
                dst_view: edge_tex_view,
                pipeline: &state.edge_pipeline,
                extent,
                load_op: AttachmentLoadOp::Clear(ColorClearValue::Int(0)),
                bindings: &FullscreenTriangleBindInfo {
                    layout: &state.edge_layout,
                    sets: &[set],
                    first_set: 0,
                    dynamic_offsets: &[],
                    constant_blocks: &[(0, &metrics)],
                },
            };
            draw_fullscreen_triangle(encoder, &info);
        }
    });
    edge_texture.unwrap()
}

fn blend_weight_pass(
    frame_graph: &mut FrameGraphBuilder<GraphArgs>,
    state: Arc<SmaaState>,
    area_tex: TextureHandle,
    search_tex: TextureHandle,
    edge_texture: &RenderPlaneOutput,
) -> RenderPlaneOutput {
    let extent = edge_texture.desc.get_extent_2d();

    let mut blend_texture = None;
    frame_graph.add_pass(nstr!("SmaaBlendingWeightCalculation"), |resources| {
        let edge_tex = resources.read_texture(edge_texture.id, ResourceUsageFlags::SHADER_RESOURCE);

        let blend_tex_desc = TextureDesc::texture_2d(extent.width, extent.height)
            .with_format(Format::Bgra8Unorm)
            .with_name(obj_name!("SmaaBlendTexture"));
        let blend_tex = resources.create_texture(
            &blend_tex_desc,
            // BarrierSync::RENDER_TARGET,
            ResourceUsageFlags::RENDER_TARGET,
        );
        blend_texture = Some(RenderPlaneOutput {
            id: blend_tex.into(),
            desc: blend_tex_desc.strip_name(),
        });

        move |encoder, _graph, resources, args| unsafe {
            let device = resources.device();

            let blend_tex = resources.get_texture(blend_tex).unwrap();
            let blend_tex_view = ImageView::get_rtv_for(device, blend_tex).unwrap();

            let edge_tex = resources.get_texture(edge_tex).unwrap();
            let edge_tex_view = ImageView::get_srv_for(device, edge_tex).unwrap();

            let area_view = args.texture_pool.get_ref(area_tex).unwrap();
            let area_view = area_view.get_default_view().unwrap();
            let search_view = args.texture_pool.get_ref(search_tex).unwrap();
            let search_view = search_view.get_default_view().unwrap();

            let set = resources
                .descriptor_arena()
                .allocate_set(&state.weight_set_layout)
                .unwrap();
            resources.device().update_descriptor_sets(&[
                DescriptorWriteDesc::texture(set, 0, &edge_tex_view.srv_write()),
                DescriptorWriteDesc::texture(set, 1, &area_view.srv_write()),
                DescriptorWriteDesc::texture(set, 2, &search_view.srv_write()),
            ]);

            let metrics = metrics(extent);

            let info = FullscreenTriangleInfo {
                dst_view: blend_tex_view,
                pipeline: &state.weight_pipeline,
                extent,
                load_op: AttachmentLoadOp::DontCare,
                bindings: &FullscreenTriangleBindInfo {
                    layout: &state.weight_layout,
                    sets: &[set],
                    first_set: 0,
                    dynamic_offsets: &[],
                    constant_blocks: &[(0, &metrics)],
                },
            };
            draw_fullscreen_triangle(encoder, &info);
        }
    });
    blend_texture.unwrap()
}

fn aa_blend_resolve_pass(
    frame_graph: &mut FrameGraphBuilder<GraphArgs>,
    state: Arc<SmaaState>,
    colour_input: &RenderPlaneOutput,
    blend_texture: &RenderPlaneOutput,
) -> RenderPlaneOutput {
    let format = colour_input.desc.format;
    let extent = blend_texture.desc.get_extent_2d();

    let mut out_texture = None;
    frame_graph.add_pass(nstr!("SmaaBlend"), |resources| {
        let colour_input =
            resources.read_texture(colour_input.id, ResourceUsageFlags::SHADER_RESOURCE);
        let blend_texture =
            resources.read_texture(blend_texture.id, ResourceUsageFlags::SHADER_RESOURCE);

        let output_desc = TextureDesc::texture_2d(extent.width, extent.height)
            .with_format(format.to_srgb())
            .with_name(obj_name!("SmaaOutput"));
        let output = resources.create_texture(
            &output_desc,
            // BarrierSync::RENDER_TARGET,
            ResourceUsageFlags::RENDER_TARGET,
        );
        out_texture = Some(RenderPlaneOutput {
            id: output.into(),
            desc: output_desc.strip_name(),
        });

        move |encoder, _graph, resources, _args| unsafe {
            let device = resources.device();

            // We want raw access to the encoded SRGB data. We output SRGB encoded data from the
            // shader so we need a UNORM view to ensure we don't double encode it.
            let output = resources.get_texture(output).unwrap();
            let desc =
                ImageViewDesc::rtv_for_texture(device, output).with_format(format.to_non_srgb());
            let output_view = device.get_texture_rtv(output, &desc).unwrap();

            // We want raw access to the encoded SRGB data
            let colour_input_tex = resources.get_texture(colour_input).unwrap();
            let desc = ImageViewDesc::srv_for_texture(device, colour_input_tex)
                .with_format(format.to_non_srgb());
            let colour = device.get_texture_view(colour_input_tex, &desc).unwrap();

            // Blend texture is accessed directly as the native UNORM format.
            let blend_texture = resources.get_texture(blend_texture).unwrap();
            let blend = ImageView::get_srv_for(device, blend_texture).unwrap();

            let set = resources
                .descriptor_arena()
                .allocate_set(&state.blend_set_layout)
                .unwrap();
            resources.device().update_descriptor_sets(&[
                DescriptorWriteDesc::texture(set, 0, &blend.srv_write()),
                DescriptorWriteDesc::texture(set, 1, &colour.srv_write()),
            ]);

            let metrics = metrics(extent);

            let info = FullscreenTriangleInfo {
                dst_view: output_view,
                pipeline: &state.blend_pipeline,
                extent,
                load_op: AttachmentLoadOp::DontCare,
                bindings: &FullscreenTriangleBindInfo {
                    layout: &state.blend_layout,
                    sets: &[set],
                    first_set: 0,
                    dynamic_offsets: &[],
                    constant_blocks: &[(0, &metrics)],
                },
            };
            draw_fullscreen_triangle(encoder, &info);
        }
    });
    out_texture.unwrap()
}

#[derive(PartialEq, Eq, Hash)]
pub struct SmaaStateKey(pub Format);

impl IStateCacheKey for SmaaStateKey {
    type Storage = SmaaState;
}

pub struct SmaaState {
    pub linear_sampler: SamplerHandle,
    pub point_sampler: SamplerHandle,
    pub edge_set_layout: DescriptorSetLayoutHandle,
    pub weight_set_layout: DescriptorSetLayoutHandle,
    pub blend_set_layout: DescriptorSetLayoutHandle,
    pub edge_layout: PipelineLayoutHandle,
    pub weight_layout: PipelineLayoutHandle,
    pub blend_layout: PipelineLayoutHandle,
    pub edge_pipeline: GraphicsPipelineHandle,
    pub weight_pipeline: GraphicsPipelineHandle,
    pub blend_pipeline: GraphicsPipelineHandle,
}

impl SmaaState {
    pub fn key(format: Format) -> SmaaStateKey {
        SmaaStateKey(format)
    }

    pub fn new(cache: &mut StateCache, device: &dyn IDevice, format: Format) -> Self {
        let linear_sampler = Self::create_linear_sampler(device);
        let point_sampler = Self::create_point_sampler(device);

        let edge_set_layout = Self::create_edge_layout(device, &linear_sampler, &point_sampler);

        let weight_set_layout = Self::create_weight_layout(device, &linear_sampler, &point_sampler);

        let blend_set_layout = Self::create_blend_layout(device, &linear_sampler, &point_sampler);

        let edge_layout = Self::create_pipeline_layout(
            device,
            &edge_set_layout,
            obj_name_opt!("EdgePipelineLayout"),
        );

        let weight_layout = Self::create_pipeline_layout(
            device,
            &weight_set_layout,
            obj_name_opt!("WeightPipelineLayout"),
        );

        let blend_layout = Self::create_pipeline_layout(
            device,
            &blend_set_layout,
            obj_name_opt!("BlendPipelineLayout"),
        );

        let edge_pipeline = Self::create_edge_detect_pipeline_state(
            device,
            &edge_layout,
            cache.shader_db(),
            Format::Bgra8Unorm,
        );

        let weight_pipeline = Self::create_weight_calculate_pipeline_state(
            device,
            &weight_layout,
            cache.shader_db(),
            Format::Bgra8Unorm,
        );

        let blend_pipeline = Self::create_blending_pipeline_state(
            device,
            &blend_layout,
            cache.shader_db(),
            format.to_non_srgb(), // Intentional for how this pass is implemented
        );

        Self {
            linear_sampler,
            point_sampler,
            edge_set_layout,
            weight_set_layout,
            blend_set_layout,
            edge_layout,
            weight_layout,
            blend_layout,
            edge_pipeline,
            weight_pipeline,
            blend_pipeline,
        }
    }

    pub fn create_edge_layout(
        device: &dyn IDevice,
        linear_sampler: &SamplerHandle,
        point_sampler: &SamplerHandle,
    ) -> DescriptorSetLayoutHandle {
        let linear_sampler = [linear_sampler];
        let point_sampler = [point_sampler];

        let descriptor_set_layout_desc = DescriptorSetLayoutDesc {
            visibility: DescriptorShaderVisibility::Fragment,
            items: &[
                DescriptorType::Texture.binding(0),
                DescriptorType::Sampler
                    .binding(1)
                    .with_static_samplers(&linear_sampler),
                DescriptorType::Sampler
                    .binding(2)
                    .with_static_samplers(&point_sampler),
            ],
            name: obj_name_opt!("EdgeDescriptorSetLayout"),
        };
        device
            .create_descriptor_set_layout(&descriptor_set_layout_desc)
            .unwrap()
    }

    pub fn create_weight_layout(
        device: &dyn IDevice,
        linear_sampler: &SamplerHandle,
        point_sampler: &SamplerHandle,
    ) -> DescriptorSetLayoutHandle {
        let linear_sampler = [linear_sampler];
        let point_sampler = [point_sampler];

        let descriptor_set_layout_desc = DescriptorSetLayoutDesc {
            visibility: DescriptorShaderVisibility::Fragment,
            items: &[
                DescriptorType::Texture.binding(0),
                DescriptorType::Texture.binding(1),
                DescriptorType::Texture.binding(2),
                DescriptorType::Sampler
                    .binding(3)
                    .with_static_samplers(&linear_sampler),
                DescriptorType::Sampler
                    .binding(4)
                    .with_static_samplers(&point_sampler),
            ],
            name: obj_name_opt!("WeightDescriptorSetLayout"),
        };
        device
            .create_descriptor_set_layout(&descriptor_set_layout_desc)
            .unwrap()
    }

    pub fn create_blend_layout(
        device: &dyn IDevice,
        linear_sampler: &SamplerHandle,
        point_sampler: &SamplerHandle,
    ) -> DescriptorSetLayoutHandle {
        let linear_sampler = [linear_sampler];
        let point_sampler = [point_sampler];

        let descriptor_set_layout_desc = DescriptorSetLayoutDesc {
            visibility: DescriptorShaderVisibility::Fragment,
            items: &[
                DescriptorType::Texture.binding(0),
                DescriptorType::Texture.binding(1),
                DescriptorType::Sampler
                    .binding(2)
                    .with_static_samplers(&linear_sampler),
                DescriptorType::Sampler
                    .binding(3)
                    .with_static_samplers(&point_sampler),
            ],
            name: obj_name_opt!("BlendDescriptorSetLayout"),
        };
        device
            .create_descriptor_set_layout(&descriptor_set_layout_desc)
            .unwrap()
    }

    pub fn create_pipeline_layout(
        device: &dyn IDevice,
        set_layout: &DescriptorSetLayoutHandle,
        name: Option<&str>,
    ) -> PipelineLayoutHandle {
        let pipeline_layout_desc = PipelineLayoutDesc {
            set_layouts: &[set_layout],
            push_constant_blocks: &[PushConstantBlock {
                binding: 0,
                visibility: DescriptorShaderVisibility::All,
                size: 16,
            }],
            name,
        };
        device
            .create_pipeline_layout(&pipeline_layout_desc)
            .unwrap()
    }

    pub fn create_edge_detect_pipeline_state(
        device: &dyn IDevice,
        pipeline_layout: &PipelineLayoutHandle,
        shader_db: &ShaderDatabaseAccessor,
        format: Format,
    ) -> GraphicsPipelineHandle {
        let vertex_shader = shader_db
            .load_stage(shaders::smaa::edge_detect::vert())
            .unwrap();
        let fragment_shader = shader_db
            .load_stage(shaders::smaa::edge_detect::frag())
            .unwrap();

        create_fullscreen_triangle_pipeline(
            device,
            pipeline_layout,
            format,
            vertex_shader,
            fragment_shader,
            obj_name_opt!("EdgeDetectGraphicsPipelineState"),
        )
        .unwrap()
    }

    pub fn create_weight_calculate_pipeline_state(
        device: &dyn IDevice,
        pipeline_layout: &PipelineLayoutHandle,
        shader_db: &ShaderDatabaseAccessor,
        format: Format,
    ) -> GraphicsPipelineHandle {
        let vertex_shader = shader_db
            .load_stage(shaders::smaa::weight_calculate::vert())
            .unwrap();
        let fragment_shader = shader_db
            .load_stage(shaders::smaa::weight_calculate::frag())
            .unwrap();

        create_fullscreen_triangle_pipeline(
            device,
            pipeline_layout,
            format,
            vertex_shader,
            fragment_shader,
            obj_name_opt!("WeightCalculateGraphicsPipelineState"),
        )
        .unwrap()
    }

    pub fn create_blending_pipeline_state(
        device: &dyn IDevice,
        pipeline_layout: &PipelineLayoutHandle,
        shader_db: &ShaderDatabaseAccessor,
        format: Format,
    ) -> GraphicsPipelineHandle {
        let vertex_shader = shader_db
            .load_stage(shaders::smaa::blending::vert())
            .unwrap();
        let fragment_shader = shader_db
            .load_stage(shaders::smaa::blending::frag())
            .unwrap();

        create_fullscreen_triangle_pipeline(
            device,
            pipeline_layout,
            format,
            vertex_shader,
            fragment_shader,
            obj_name_opt!("BlendingGraphicsPipelineState"),
        )
        .unwrap()
    }

    pub fn create_linear_sampler(device: &dyn IDevice) -> SamplerHandle {
        let desc = SamplerDesc {
            min_filter: SamplerFilter::Linear,
            mag_filter: SamplerFilter::Linear,
            mip_filter: SamplerMipFilter::Nearest,
            address_mode_u: SamplerAddressMode::Clamp,
            address_mode_v: SamplerAddressMode::Clamp,
            address_mode_w: SamplerAddressMode::Clamp,
            name: obj_name_opt!("LinearSampler"),
            ..Default::default()
        };
        device.create_sampler(&desc).unwrap()
    }

    pub fn create_point_sampler(device: &dyn IDevice) -> SamplerHandle {
        let desc = SamplerDesc {
            min_filter: SamplerFilter::Nearest,
            mag_filter: SamplerFilter::Nearest,
            mip_filter: SamplerMipFilter::Nearest,
            address_mode_u: SamplerAddressMode::Clamp,
            address_mode_v: SamplerAddressMode::Clamp,
            address_mode_w: SamplerAddressMode::Clamp,
            name: obj_name_opt!("PointSampler"),
            ..Default::default()
        };
        device.create_sampler(&desc).unwrap()
    }
}

fn metrics(extent: Extent2D) -> [u8; 16] {
    let metric_w = extent.width as f32;
    let metric_h = extent.height as f32;
    let metrics: [f32; 4] = [1.0 / metric_w, 1.0 / metric_h, metric_w, metric_h];

    bytemuck::cast(metrics)
}
