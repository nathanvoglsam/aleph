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

use aleph_any::AnyArc;
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
    DefaultResources, IShaderAccessor, IShaderAccessorExt, IStateCacheKey, RenderPlaneOutput,
    StateCache, TextureHandle, shaders,
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

            let block_layout = state.edge_block_layout.as_ref();
            let block = resources
                .descriptor_arena()
                .allocate_block(block_layout)
                .unwrap();
            let linear_sampler = &state.linear_sampler;
            let point_sampler = &state.point_sampler;
            let params = [
                TextureWrite::srv(colour_input_view).into(),
                SamplerWrite::new(linear_sampler).into(),
                SamplerWrite::new(point_sampler).into(),
            ];
            resources
                .device()
                .update_parameter_block(block_layout, block, 0, &params);

            let metrics = metrics(extent);

            let info = FullscreenTriangleInfo {
                dst_view: edge_tex_view,
                pipeline: &state.edge_pipeline,
                extent,
                load_op: AttachmentLoadOp::Clear(ColorClearValue::Int(0)),
                bindings: &FullscreenTriangleBindInfo {
                    binding_signature: state.edge_signature.as_ref(),
                    blocks: &[block],
                    first_blocks: 0,
                    constant_block: Some(&metrics),
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

            let block_layout = state.weight_block_layout.as_ref();
            let block = resources
                .descriptor_arena()
                .allocate_block(block_layout)
                .unwrap();
            let linear_sampler = &state.linear_sampler;
            let point_sampler = &state.point_sampler;
            let params = [
                TextureWrite::srv(edge_tex_view).into(),
                TextureWrite::srv(area_view).into(),
                TextureWrite::srv(search_view).into(),
                SamplerWrite::new(linear_sampler).into(),
                SamplerWrite::new(point_sampler).into(),
            ];
            resources
                .device()
                .update_parameter_block(block_layout, block, 0, &params);

            let metrics = metrics(extent);

            let info = FullscreenTriangleInfo {
                dst_view: blend_tex_view,
                pipeline: &state.weight_pipeline,
                extent,
                load_op: AttachmentLoadOp::DontCare,
                bindings: &FullscreenTriangleBindInfo {
                    binding_signature: state.weight_signature.as_ref(),
                    blocks: &[block],
                    first_blocks: 0,
                    constant_block: Some(&metrics),
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

            let block_layout = state.blend_block_layout.as_ref();
            let block = resources
                .descriptor_arena()
                .allocate_block(block_layout)
                .unwrap();
            let linear_sampler = &state.linear_sampler;
            let point_sampler = &state.point_sampler;
            let params = [
                TextureWrite::srv(blend).into(),
                TextureWrite::srv(colour).into(),
                SamplerWrite::new(linear_sampler).into(),
                SamplerWrite::new(point_sampler).into(),
            ];
            resources
                .device()
                .update_parameter_block(block_layout, block, 0, &params);

            let metrics = metrics(extent);

            let info = FullscreenTriangleInfo {
                dst_view: output_view,
                pipeline: &state.blend_pipeline,
                extent,
                load_op: AttachmentLoadOp::DontCare,
                bindings: &FullscreenTriangleBindInfo {
                    binding_signature: state.blend_signature.as_ref(),
                    blocks: &[block],
                    first_blocks: 0,
                    constant_block: Some(&metrics),
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
    pub edge_block_layout: AnyArc<dyn IParameterBlockLayout>,
    pub weight_block_layout: AnyArc<dyn IParameterBlockLayout>,
    pub blend_block_layout: AnyArc<dyn IParameterBlockLayout>,
    pub edge_signature: AnyArc<dyn IBindingSignature>,
    pub weight_signature: AnyArc<dyn IBindingSignature>,
    pub blend_signature: AnyArc<dyn IBindingSignature>,
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

        let edge_block_layout = Self::create_edge_layout(device);

        let weight_block_layout = Self::create_weight_layout(device);

        let blend_block_layout = Self::create_blend_layout(device);

        let edge_signature = Self::create_binding_signature(
            device,
            edge_block_layout.as_ref(),
            obj_name_opt!("EdgeBindingSignature"),
        );

        let weight_signature = Self::create_binding_signature(
            device,
            weight_block_layout.as_ref(),
            obj_name_opt!("WeightBindingSignature"),
        );

        let blend_signature = Self::create_binding_signature(
            device,
            blend_block_layout.as_ref(),
            obj_name_opt!("BlendBindingSignature"),
        );

        let edge_pipeline = Self::create_edge_detect_pipeline_state(
            device,
            edge_signature.as_ref(),
            cache.shader_db(),
            Format::Bgra8Unorm,
        );

        let weight_pipeline = Self::create_weight_calculate_pipeline_state(
            device,
            weight_signature.as_ref(),
            cache.shader_db(),
            Format::Bgra8Unorm,
        );

        let blend_pipeline = Self::create_blending_pipeline_state(
            device,
            blend_signature.as_ref(),
            cache.shader_db(),
            format.to_non_srgb(), // Intentional for how this pass is implemented
        );

        Self {
            linear_sampler,
            point_sampler,
            edge_block_layout,
            weight_block_layout,
            blend_block_layout,
            edge_signature,
            weight_signature,
            blend_signature,
            edge_pipeline,
            weight_pipeline,
            blend_pipeline,
        }
    }

    pub fn create_edge_layout(device: &dyn IDevice) -> AnyArc<dyn IParameterBlockLayout> {
        let desc = ParameterBlockDesc {
            params: &[
                ParameterType::Texture2D.param(),
                ParameterType::SamplerState.param(),
                ParameterType::SamplerState.param(),
            ],
            visibility: DescriptorShaderVisibility::Fragment,
            flags: Default::default(),
            name: obj_name_opt!("EdgeParameterBlockLayout"),
        };
        device.create_parameter_block_layout(&desc).unwrap()
    }

    pub fn create_weight_layout(device: &dyn IDevice) -> AnyArc<dyn IParameterBlockLayout> {
        let desc = ParameterBlockDesc {
            params: &[
                ParameterType::Texture2D.param(),
                ParameterType::Texture2D.param(),
                ParameterType::Texture2D.param(),
                ParameterType::SamplerState.param(),
                ParameterType::SamplerState.param(),
            ],
            visibility: DescriptorShaderVisibility::Fragment,
            flags: Default::default(),
            name: obj_name_opt!("WeightParameterBlockLayout"),
        };
        device.create_parameter_block_layout(&desc).unwrap()
    }

    pub fn create_blend_layout(device: &dyn IDevice) -> AnyArc<dyn IParameterBlockLayout> {
        let desc = ParameterBlockDesc {
            params: &[
                ParameterType::Texture2D.param(),
                ParameterType::Texture2D.param(),
                ParameterType::SamplerState.param(),
                ParameterType::SamplerState.param(),
            ],
            visibility: DescriptorShaderVisibility::Fragment,
            flags: Default::default(),
            name: obj_name_opt!("BlendParameterBlockLayout"),
        };
        device.create_parameter_block_layout(&desc).unwrap()
    }

    pub fn create_binding_signature(
        device: &dyn IDevice,
        block_layout: &dyn IParameterBlockLayout,
        name: Option<&str>,
    ) -> AnyArc<dyn IBindingSignature> {
        let desc = BindingSignatureDesc {
            parameter_block_layouts: &[block_layout],
            push_constant_block: PushConstantBlock::new(16),
            name,
        };
        device.create_binding_signature(&desc).unwrap()
    }

    pub fn create_edge_detect_pipeline_state(
        device: &dyn IDevice,
        binding_signature: &dyn IBindingSignature,
        shader_db: &dyn IShaderAccessor,
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
            binding_signature,
            format,
            vertex_shader,
            fragment_shader,
            obj_name_opt!("EdgeDetectGraphicsPipelineState"),
        )
        .unwrap()
    }

    pub fn create_weight_calculate_pipeline_state(
        device: &dyn IDevice,
        binding_signature: &dyn IBindingSignature,
        shader_db: &dyn IShaderAccessor,
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
            binding_signature,
            format,
            vertex_shader,
            fragment_shader,
            obj_name_opt!("WeightCalculateGraphicsPipelineState"),
        )
        .unwrap()
    }

    pub fn create_blending_pipeline_state(
        device: &dyn IDevice,
        binding_signature: &dyn IBindingSignature,
        shader_db: &dyn IShaderAccessor,
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
            binding_signature,
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
