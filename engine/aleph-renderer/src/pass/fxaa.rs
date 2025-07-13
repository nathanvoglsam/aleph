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

use aleph_device_allocators::{IUploadAllocator, UploadBumpAllocator};
use aleph_frame_graph::*;
use aleph_nstr::nstr;
use aleph_pin_board::PinBoard;
use aleph_rhi_api::*;

use crate::pass::GraphArgs;
use crate::pass::utils::{
    FullscreenTriangleBindInfo, FullscreenTriangleInfo, create_fullscreen_triangle_pipeline,
    draw_fullscreen_triangle,
};
use crate::{IStateCacheKey, RenderPlaneOutput, ShaderDatabaseAccessor, StateCache, shaders};

struct Payload {
    src: ResourceRef,
    dst: ResourceMut,
    uniform_buffer: ResourceMut,
}

pub fn pass(
    frame_graph: &mut FrameGraphBuilder<GraphArgs>,
    device: &dyn IDevice,
    _pin_board: &PinBoard,
    state_cache: &mut StateCache,
    src: &RenderPlaneOutput,
) -> RenderPlaneOutput {
    let src_extent = src.desc.get_extent_2d();
    let src_format = src.desc.format;

    // We take a non-SRGB view of the dst texture, hence to_non_srgb. The fxaa shader stays in SRGB
    // space from input to output so we need to skip the implicit SRGB conversion from SRGB formats.
    let key = FxaaState::key(src_format.to_non_srgb());
    let state = state_cache.get_or_insert_with(&key, |cache, k| FxaaState::new(cache, device, k.0));

    let mut result = None;

    frame_graph.add_pass(nstr!("Fxaa"), |resources| {
        let src = resources.read_texture(src.id, ResourceUsageFlags::SHADER_RESOURCE);

        let output_desc = TextureDesc::texture_2d(src_extent.width, src_extent.height)
            .with_format(src_format.to_srgb())
            .with_name(obj_name!("FxaaOutput"));
        let dst = resources.create_texture(
            &output_desc,
            // BarrierSync::RENDER_TARGET,
            ResourceUsageFlags::RENDER_TARGET,
        );
        result = Some(RenderPlaneOutput {
            id: dst.into(),
            desc: output_desc.strip_name(),
        });

        let uniform_buffer = resources.create_buffer(
            &BufferDesc::new(1024u64)
                .cpu_write()
                .with_name(obj_name!("FxaaUniformBuffer")),
            ResourceUsageFlags::CONSTANT_BUFFER,
        );

        let data = Payload {
            src,
            dst,
            uniform_buffer,
        };
        move |encoder, _graph, resources, _args| unsafe {
            let device = resources.device();

            let dst = resources.get_texture(data.dst).unwrap();
            let desc =
                ImageViewDesc::rtv_for_texture(device, dst).with_format(src_format.to_non_srgb());
            let dst_view = device.get_texture_rtv(dst, &desc).unwrap();

            let src = resources.get_texture(data.src).unwrap();
            let desc =
                ImageViewDesc::srv_for_texture(device, src).with_format(src_format.to_non_srgb());
            let src_view = device.get_texture_view(src, &desc).unwrap();

            let buffer = resources.get_buffer(data.uniform_buffer).unwrap();
            let u_ptr = device.map_buffer(buffer).unwrap();
            let u_alloc = UploadBumpAllocator::new_from_block(
                buffer.clone(),
                ResourceUsageFlags::CONSTANT_BUFFER,
                u_ptr,
                0,
                1024,
            )
            .unwrap();
            u_alloc
                .allocate_object(FxaaParams::new(src_extent))
                .unwrap();
            device.unmap_buffer(buffer).unwrap();

            let set = resources
                .descriptor_arena()
                .allocate_set(&state.layout.set_layout)
                .unwrap();
            device.update_descriptor_sets(&[
                DescriptorWriteDesc::uniform_buffer(set, 0, &buffer.uniform_buffer_write(256)),
                DescriptorWriteDesc::texture(set, 1, &src_view.srv_write()),
            ]);

            let info = FullscreenTriangleInfo {
                dst_view,
                pipeline: &state.pipeline,
                extent: src_extent,
                load_op: AttachmentLoadOp::DontCare,
                bindings: &FullscreenTriangleBindInfo {
                    layout: &state.layout.pipeline_layout,
                    sets: &[set],
                    first_set: 0,
                    dynamic_offsets: &[],
                    constant_blocks: &[],
                },
            };
            draw_fullscreen_triangle(encoder, &info);
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
                DescriptorType::UniformBuffer.binding(0),
                DescriptorType::Texture.binding(1),
                DescriptorType::Sampler
                    .binding(2)
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
            enable_anisotropy: true,
            max_anisotropy: 4,
            ..Default::default()
        };
        device.create_sampler(&desc).unwrap()
    }
}

#[derive(PartialEq, Eq, Hash)]
pub struct FxaaStateKey(pub Format);

impl IStateCacheKey for FxaaStateKey {
    type Storage = FxaaState;
}

pub struct FxaaState {
    pub layout: Arc<CompositePlanesLayout>,
    pub pipeline: GraphicsPipelineHandle,
}

impl FxaaState {
    pub fn key(format: Format) -> FxaaStateKey {
        FxaaStateKey(format)
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
        let vertex_shader = shader_db.load_stage(shaders::fxaa::vert()).unwrap();
        let fragment_shader = shader_db.load_stage(shaders::fxaa::frag()).unwrap();

        create_fullscreen_triangle_pipeline(
            device,
            pipeline_layout,
            format,
            vertex_shader,
            fragment_shader,
            obj_name_opt!("GraphicsPipelineState"),
        )
        .unwrap()
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::AnyBitPattern)]
struct FxaaParams {
    /// Only used on FXAA Quality.
    ///
    /// This must be from a constant/uniform.
    ///
    /// # Layout
    ///
    /// - {x_} = 1.0/screenWidthInPixels
    /// - {_y} = 1.0/screenHeightInPixels
    fxaa_quality_rcp_frame: [f32; 2],

    /// Padding word
    __pad1: f32,

    /// Padding word
    __pad2: f32,

    /// Only used on FXAA Console.
    ///
    /// This must be from a constant/uniform. This effects sub-pixel AA quality and inversely
    /// sharpness.
    ///
    /// N ranges between:
    /// - N = 0.50 (default)
    /// - N = 0.33 (sharper)
    ///
    /// # Layout
    ///
    /// - {x---} = -N/screenWidthInPixels  
    /// - {-y--} = -N/screenHeightInPixels
    /// - {--z-} =  N/screenWidthInPixels  
    /// - {---w} =  N/screenHeightInPixels
    fxaa_console_rcp_frame_opt: [f32; 4],

    /// Only used on FXAA Console.
    /// Not used on 360, but used on PS3 and PC. This must be from a constant/uniform.
    ///
    /// # Layout
    ///
    /// - {x---} = -2.0/screenWidthInPixels  
    /// - {-y--} = -2.0/screenHeightInPixels
    /// - {--z-} =  2.0/screenWidthInPixels  
    /// - {---w} =  2.0/screenHeightInPixels
    fxaa_console_rcp_frame_opt2: [f32; 4],

    /// Only used on FXAA Console.
    ///
    /// Only used on 360 in place of fxaaConsoleRcpFrameOpt2. This must be from a constant/uniform.
    ///
    /// # Layout
    ///
    /// - {x---} =  8.0/screenWidthInPixels  
    /// - {-y--} =  8.0/screenHeightInPixels
    /// - {--z-} = -4.0/screenWidthInPixels  
    /// - {---w} = -4.0/screenHeightInPixels
    fxaa_console360_rcp_frame_opt2: [f32; 4],

    /// Only used on FXAA Quality.
    ///
    /// This used to be the FXAA_QUALITY__SUBPIX define. It is here now to allow easier tuning.
    /// Choose the amount of sub-pixel aliasing removal. This can effect sharpness.
    ///
    /// - 1.00: upper limit (softer)
    /// - 0.75: default amount of filtering
    /// - 0.50: lower limit (sharper, less sub-pixel aliasing removal)
    /// - 0.25: almost off
    /// - 0.00: completely off
    fxaa_quality_subpix: f32,

    /// Only used on FXAA Quality.
    ///
    /// This used to be the FXAA_QUALITY__EDGE_THRESHOLD define. It is here now to allow easier
    /// tuning. The minimum amount of local contrast required to apply algorithm.
    ///
    /// - 0.333: too little (faster)
    /// - 0.250: low quality
    /// - 0.166: default
    /// - 0.125: high quality
    /// - 0.063: overkill (slower)
    fxaa_quality_edge_threshold: f32,

    /// Only used on FXAA Quality.
    ///
    /// This used to be the FXAA_QUALITY__EDGE_THRESHOLD_MIN define.
    /// It is here now to allow easier tuning.
    /// Trims the algorithm from processing darks.
    ///
    /// - 0.0833: upper limit (default, the start of visible unfiltered edges)
    /// - 0.0625: high quality (faster)
    /// - 0.0312: visible limit (slower)
    ///
    /// # Special Notes
    ///
    /// When using FXAA_GREEN_AS_LUMA, likely want to set this to zero. As colors that are mostly
    /// not-green will appear very dark in the green channel! Tune by looking at mostly non-green
    /// content, then start at zero and increase until aliasing is a problem.
    fxaa_quality_edge_threshold_min: f32,

    /// Only used on FXAA Console.
    ///
    /// This used to be the FXAA_CONSOLE__EDGE_SHARPNESS define. It is here now to allow easier
    /// tuning. This does not effect PS3, as this needs to be compiled in.
    ///
    /// Use FXAA_CONSOLE__PS3_EDGE_SHARPNESS for PS3. Due to the PS3 being ALU bound, there are only
    /// three safe values here: 2 and 4 and 8. These options use the shaders ability to a
    /// free * or / by 2|4|8.
    ///
    /// For all other platforms can be a non-power of two.
    ///
    /// - 8.0 is sharper (default!!!)
    /// - 4.0 is softer
    /// - 2.0 is really soft (good only for vector graphics inputs)
    fxaa_console_edge_sharpness: f32,

    /// Only used on FXAA Console.
    ///
    /// This used to be the FXAA_CONSOLE__EDGE_THRESHOLD define.
    /// It is here now to allow easier tuning.
    /// This does not effect PS3, as this needs to be compiled in.
    ///
    /// Use FXAA_CONSOLE__PS3_EDGE_THRESHOLD for PS3. Due to the PS3 being ALU bound, there are only
    /// two safe values here: 1/4 and 1/8. These options use the shaders ability to a free * or / by
    /// 2|4|8.
    ///
    /// The console setting has a different mapping than the quality setting. Other platforms can
    /// use other values.
    ///
    /// - 0.125 leaves less aliasing, but is softer (default!!!)
    /// - 0.25 leaves more aliasing, and is sharper
    fxaa_console_edge_threshold: f32,

    /// Only used on FXAA Console.
    ///
    /// This used to be the FXAA_CONSOLE__EDGE_THRESHOLD_MIN define. It is here now to allow easier
    /// tuning. Trims the algorithm from processing darks. The console setting has a different
    /// mapping than the quality setting. This only applies when FXAA_EARLY_EXIT is 1. This does not
    /// apply to PS3, PS3 was simplified to avoid more shader instructions.
    ///
    /// - 0.06: faster but more aliasing in darks
    /// - 0.05: default
    /// - 0.04: slower and less aliasing in darks
    ///
    /// # Special Notes
    ///
    /// When using FXAA_GREEN_AS_LUMA, likely want to set this to zero. As colors that are mostly
    /// not-green will appear very dark in the green channel! Tune by looking at mostly non-green
    /// content, then start at zero and increase until aliasing is a problem.
    fxaa_console_edge_threshold_min: f32,

    /// Padding word
    __pad3: f32,

    /// Padding word
    __pad4: f32,

    /// Extra constants for 360 FXAA Console only.
    ///
    /// Use zeros or anything else for other platforms. These must be in physical constant registers
    /// and NOT immedates. Immedates will result in compiler un-optimizing.
    ///
    /// {xyzw} = float4(1.0, -1.0, 0.25, -0.25)
    fxaa_console360_const_dir: [f32; 4],
}

impl FxaaParams {
    pub fn new(extent: Extent2D) -> Self {
        let width = extent.width as f32;
        let height = extent.height as f32;
        let rcp_width = 1.0 / width;
        let rcp_height = 1.0 / height;

        let fxaa_quality_rcp_frame = [rcp_width, rcp_height];

        let console_sharpness = 0.5;
        let fxaa_console_rcp_frame_opt = [
            -console_sharpness / width,
            -console_sharpness / height,
            console_sharpness / width,
            console_sharpness / height,
        ];

        let fxaa_console_rcp_frame_opt2 = [-2.0 / width, -2.0 / height, 2.0 / width, 2.0 / height];

        let fxaa_console360_rcp_frame_opt2 =
            [8.0 / width, 8.0 / height, -4.0 / width, -4.0 / height];

        let fxaa_quality_subpix = 1.00;
        let fxaa_quality_edge_threshold = 0.063;
        let fxaa_quality_edge_threshold_min = 0.0312;
        let fxaa_console_edge_sharpness = 8.0;
        let fxaa_console_edge_threshold = 0.125;
        let fxaa_console_edge_threshold_min = 0.05;

        let fxaa_console360_const_dir = [0.0, 0.0, 0.0, 0.0];

        Self {
            fxaa_quality_rcp_frame,
            __pad1: 0.0,
            __pad2: 0.0,
            fxaa_console_rcp_frame_opt,
            fxaa_console_rcp_frame_opt2,
            fxaa_console360_rcp_frame_opt2,
            fxaa_quality_subpix,
            fxaa_quality_edge_threshold,
            fxaa_quality_edge_threshold_min,
            fxaa_console_edge_sharpness,
            fxaa_console_edge_threshold,
            fxaa_console_edge_threshold_min,
            __pad3: 0.0,
            __pad4: 0.0,
            fxaa_console360_const_dir,
        }
    }
}
