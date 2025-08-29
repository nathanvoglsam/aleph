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

use aleph_any::AnyArc;
use aleph_device_allocators::{IUploadAllocator, UploadBumpAllocator};
use aleph_frame_graph::*;
use aleph_nstr::nstr;
use aleph_pin_board::PinBoard;
use aleph_rhi_api::*;

use crate::pass::main_gbuffer::{CameraLayout, MainGBufferPassOutput};
use crate::pass::{GraphArgs, GraphSwapImageInfo};
use crate::{CameraInfo, IShaderAccessorExt, IStateCacheKey, StateCache, shaders};

struct LightingResolvePassPayload {
    depth: ResourceRef,
    gbuffer0: ResourceRef,
    gbuffer1: ResourceRef,
    gbuffer2: ResourceRef,
    lighting: ResourceMut,
    uniform_buffer: ResourceMut,
}
pub struct LightingResolvePassOutput {
    pub lighting: ResourceMut,
}

pub fn pass(
    frame_graph: &mut FrameGraphBuilder<GraphArgs>,
    device: &dyn IDevice,
    pin_board: &PinBoard,
    state_cache: &mut StateCache,
) {
    let key = LightResolveState::key();
    let state =
        state_cache.get_or_insert_with(&key, |cache, _| LightResolveState::new(cache, device));

    frame_graph.add_pass(nstr!("DeferredLightingPass"), |resources| {
        let main_gbuffer_pass_output: &MainGBufferPassOutput = pin_board.get().unwrap();
        let back_buffer_info: &GraphSwapImageInfo = pin_board.get().unwrap();
        let b_desc = &back_buffer_info.desc;

        let depth = resources.read_texture(
            main_gbuffer_pass_output.depth_buffer,
            ResourceUsageFlags::SHADER_RESOURCE,
        );
        let gbuffer0 = resources.read_texture(
            main_gbuffer_pass_output.gbuffer0,
            ResourceUsageFlags::SHADER_RESOURCE,
        );
        let gbuffer1 = resources.read_texture(
            main_gbuffer_pass_output.gbuffer1,
            ResourceUsageFlags::SHADER_RESOURCE,
        );
        let gbuffer2 = resources.read_texture(
            main_gbuffer_pass_output.gbuffer2,
            ResourceUsageFlags::SHADER_RESOURCE,
        );
        let lighting = resources.create_texture(
            &TextureDesc::texture_2d(b_desc.width, b_desc.height)
                .with_format(Format::Rgba16Float)
                .with_clear_value(OptimalClearValue::ColorInt(0x000000FF))
                .with_name(obj_name!("OutputLighting")),
            ResourceUsageFlags::UNORDERED_ACCESS,
        );
        let uniform_buffer = resources.create_buffer(
            &BufferDesc::new(1024u64)
                .cpu_write()
                .with_name(obj_name!("TestUniformBuffer")),
            ResourceUsageFlags::CONSTANT_BUFFER,
        );

        let data = LightingResolvePassPayload {
            depth,
            gbuffer0,
            gbuffer1,
            gbuffer2,
            lighting,
            uniform_buffer,
        };
        pin_board.publish(LightingResolvePassOutput { lighting });

        move |encoder, _graph, resources, args| unsafe {
            let device = resources.device();
            let arena = resources.descriptor_arena();
            let camera_info = args.board.get::<CameraInfo>().unwrap();

            let depth = resources.get_texture(data.depth).unwrap();
            let gbuffer0 = resources.get_texture(data.gbuffer0).unwrap();
            let gbuffer1 = resources.get_texture(data.gbuffer1).unwrap();
            let gbuffer2 = resources.get_texture(data.gbuffer2).unwrap();
            let lighting = resources.get_texture(data.lighting).unwrap();
            let uniform_buffer = resources.get_buffer(data.uniform_buffer).unwrap();
            let depth_srv = ImageView::get_srv_for(device, depth).unwrap();
            let gbuffer0_srv = ImageView::get_srv_for(device, gbuffer0).unwrap();
            let gbuffer1_srv = ImageView::get_srv_for(device, gbuffer1).unwrap();
            let gbuffer2_srv = ImageView::get_srv_for(device, gbuffer2).unwrap();
            let lighting_uav = ImageView::get_uav_for(device, lighting).unwrap();

            let u_ptr = device.map_buffer(uniform_buffer).unwrap();
            let u_alloc = UploadBumpAllocator::new_from_block(
                uniform_buffer.clone(),
                ResourceUsageFlags::CONSTANT_BUFFER,
                u_ptr,
                0,
                1024,
            )
            .unwrap();

            // let gbuffer0_desc = gbuffer0.desc();
            // let aspect_ratio = gbuffer0_desc.width as f32 / gbuffer0_desc.height as f32;
            let camera_layout = CameraLayout {
                view_matrix: camera_info.get_view_matrix().as_array().clone(),
                proj_matrix: camera_info.get_proj_matrix().as_array().clone(),
                position: camera_info
                    .position
                    .into_homogeneous_point()
                    .as_array()
                    .clone(),
                _padding: [0; 112],
            };
            u_alloc.allocate_object(camera_layout).unwrap();
            device.unmap_buffer(uniform_buffer).unwrap();

            let block_layout = state.block_layout.as_ref();
            let block = arena.allocate_block(block_layout).unwrap();
            let params = [
                TextureWrite::srv(depth_srv).into(),
                TextureWrite::srv(gbuffer0_srv).into(),
                TextureWrite::srv(gbuffer1_srv).into(),
                TextureWrite::srv(gbuffer2_srv).into(),
                TextureWrite::uav(lighting_uav).into(),
                BufferWrite::cbv(uniform_buffer, 256).into(),
            ];
            device.update_parameter_block(block_layout, block, 0, &params);

            encoder.bind_compute_pipeline(&state.pipeline);
            encoder.bind_parameter_blocks(
                state.binding_signature.as_ref(),
                PipelineBindPoint::Compute,
                0,
                &[block],
            );

            let gbuffer0_desc = device.get_texture_desc(gbuffer0);
            let group_count_x = gbuffer0_desc.width.div_ceil(8);
            let group_count_y = gbuffer0_desc.height.div_ceil(8);
            encoder.dispatch(group_count_x, group_count_y, 1);
        }
    });
}

#[derive(PartialEq, Eq, Hash)]
pub struct LightResolveStateKey;

impl IStateCacheKey for LightResolveStateKey {
    type Storage = LightResolveState;
}

pub struct LightResolveState {
    pub block_layout: AnyArc<dyn IParameterBlockLayout>,
    pub binding_signature: AnyArc<dyn IBindingSignature>,
    pub pipeline: ComputePipelineHandle,
}

impl LightResolveState {
    pub fn key() -> LightResolveStateKey {
        LightResolveStateKey
    }

    pub fn new(cache: &mut StateCache, device: &dyn IDevice) -> Self {
        let shader_module = cache
            .shader_db()
            .load_stage(shaders::deferred::deferred_lighting_cs())
            .unwrap();

        let block_layout = Self::create_block_layout(device, shader_module);
        let binding_signature = Self::create_binding_signature(device, block_layout.as_ref());
        let pipeline =
            Self::create_pipeline_state(device, binding_signature.as_ref(), shader_module);

        Self {
            block_layout,
            binding_signature,
            pipeline,
        }
    }

    pub fn create_block_layout(
        device: &dyn IDevice,
        shader: &dyn IShaderCodeSource,
    ) -> AnyArc<dyn IParameterBlockLayout> {
        let mut params = Vec::new();
        params.resize_with(shader.get_parameter_count_for_block(0), Default::default);
        shader.get_parameters_for_block(0, &mut params);

        let desc = ParameterBlockDesc {
            params: &params,
            visibility: shader.shader_type().into(),
            flags: Default::default(),
            name: obj_name_opt!("ParameterBlockLayout"),
        };
        device.create_parameter_block_layout(&desc).unwrap()
    }

    pub fn create_binding_signature(
        device: &dyn IDevice,
        block_layout: &dyn IParameterBlockLayout,
    ) -> AnyArc<dyn IBindingSignature> {
        device
            .create_binding_signature(
                &BindingSignatureDesc::new()
                    .with_parameter_block_layouts(&[block_layout])
                    .with_name(obj_name!("BindingSignature")),
            )
            .unwrap()
    }

    pub fn create_pipeline_state(
        device: &dyn IDevice,
        binding_signature: &dyn IBindingSignature,
        shader_module: &dyn IShaderCodeSource,
    ) -> ComputePipelineHandle {
        device
            .create_compute_pipeline(&ComputePipelineDesc {
                shader_module,
                binding_signature,
                name: obj_name_opt!("ComputePipeline"),
            })
            .unwrap()
    }
}
