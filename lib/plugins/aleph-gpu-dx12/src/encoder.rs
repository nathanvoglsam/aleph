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

use crate::buffer::Buffer;
use crate::general_command_list::GeneralCommandList;
use crate::internal::calc_subresource_index;
use crate::internal::conv::{decode_u32_color_to_float, resource_state_to_dx12};
use crate::pipeline::GraphicsPipeline;
use crate::swap_texture::SwapTexture;
use crate::texture::Texture;
use dx12::dxgi;
use interfaces::any::{AnyArc, QueryInterface};
use interfaces::gpu::{
    BufferBarrier, ColorClearValue, CpuAccessMode, DepthStencilClearValue, IComputeEncoder,
    IGeneralEncoder, IGraphicsPipeline, ITexture, ITransferEncoder, IndexType,
    InputAssemblyBufferBinding, QueueTransitionMode, Rect, ResourceStates, SplitBarrierMode,
    TextureBarrier, TextureDesc, TextureSubResourceSet,
};

pub struct Encoder<'a> {
    pub(crate) list: dx12::GraphicsCommandList,
    pub(crate) parent: &'a mut GeneralCommandList,
    pub(crate) input_binding_strides: [u32; 16],
}

impl<'a> Drop for Encoder<'a> {
    fn drop(&mut self) {
        // TODO: Consider an API that forces manually closing so we can avoid the unwrap here
        unsafe { self.list.close().unwrap() }
    }
}

impl<'a> Encoder<'a> {
    #[inline]
    unsafe fn clear_swap_texture(&mut self, concrete: &SwapTexture, value: &ColorClearValue) {
        let buffer = match value {
            ColorClearValue::Float { r, g, b, a } => [*r, *g, *b, *a],
            ColorClearValue::Int(v) => decode_u32_color_to_float(*v),
        };

        self.list
            .clear_render_target_view(concrete.view, &buffer, &[]);
        self.parent
            .tracker
            .images
            .push(AnyArc::map::<dyn ITexture, _>(
                concrete.this.upgrade().unwrap(),
                |v| v,
            ));
    }

    #[inline]
    unsafe fn clear_plain_texture(
        &mut self,
        concrete: &Texture,
        sub_resources: &TextureSubResourceSet,
        value: &ColorClearValue,
    ) {
        if concrete.desc.format.is_depth_stencil() {
            aleph_log::debug!("Tried to perform clear_color on a depth/stencil texture");
            return;
        }

        let buffer = match value {
            ColorClearValue::Float { r, g, b, a } => [*r, *g, *b, *a],
            ColorClearValue::Int(v) => decode_u32_color_to_float(*v),
        };

        let sub_resources = self.clamp_sub_resource_set_to_texture(&concrete.desc, sub_resources);

        // DX12 handles clearing textures differently between render targets and non render target
        // textures.
        if concrete.desc.is_render_target {
            // DX12 can only clear a single mip level per call to ClearRenderTargetView, to clear
            // all the requested layers we need to emit multiple calls to ClearRenderTargetView.
            let begin = sub_resources.base_mip_level;
            let end = begin + sub_resources.num_mip_levels;
            for level in begin..end {
                let level_sub_resources = TextureSubResourceSet {
                    base_mip_level: level,
                    num_mip_levels: 1,
                    base_array_slice: sub_resources.base_array_slice,
                    num_array_slices: sub_resources.base_mip_level,
                };

                let view = concrete.get_or_create_rtv_for_usage(None, &level_sub_resources);

                if let Some(view) = view {
                    self.list.clear_render_target_view(view, &buffer, &[]);
                } else {
                    aleph_log::debug!(
                        "Called IEncoder::clear_texture with TextureSubResourceSet::num_mip_levels = 0."
                    );
                    return;
                }
            }
        } else {
            todo!()
        }

        self.parent
            .tracker
            .images
            .push(AnyArc::map::<dyn ITexture, _>(
                concrete.this.upgrade().unwrap(),
                |v| v,
            ));
    }

    #[inline]
    unsafe fn clear_depth_image(
        &mut self,
        concrete: &Texture,
        sub_resources: &TextureSubResourceSet,
        value: &DepthStencilClearValue,
    ) {
        if !concrete.desc.format.is_depth_stencil() {
            aleph_log::debug!("Tried to perform clear_depth_stencil_texture on a color texture");
            return;
        }

        let (depth, stencil, clear_flags) = match value {
            DepthStencilClearValue::DepthStencil(d, s) => (*d, *s, dx12::ClearFlags::all()),
            DepthStencilClearValue::Depth(d) => (*d, 0, dx12::ClearFlags::DEPTH),
            DepthStencilClearValue::Stencil(s) => (0.0, *s, dx12::ClearFlags::STENCIL),
        };

        let sub_resources = self.clamp_sub_resource_set_to_texture(&concrete.desc, sub_resources);

        // DX12 handles clearing textures differently between render targets and non render target
        // textures.
        if concrete.desc.is_render_target {
            // DX12 can only clear a single mip level per call to ClearRenderTargetView, to clear
            // all the requested layers we need to emit multiple calls to ClearRenderTargetView.
            let begin = sub_resources.base_mip_level;
            let end = begin + sub_resources.num_mip_levels;
            for level in begin..end {
                let level_sub_resources = TextureSubResourceSet {
                    base_mip_level: level,
                    num_mip_levels: 1,
                    base_array_slice: sub_resources.base_array_slice,
                    num_array_slices: sub_resources.base_mip_level,
                };

                let view = concrete.get_or_create_dsv_for_usage(None, &level_sub_resources);
                if let Some(view) = view {
                    self.list
                        .clear_depth_stencil_view(view, clear_flags, depth, stencil, &[]);
                } else {
                    aleph_log::debug!(
                    "Called IEncoder::clear_depth_stencil_texture with TextureSubResourceSet::num_mip_levels = 0."
                );
                    return;
                }
            }
        }

        self.parent
            .tracker
            .images
            .push(AnyArc::map::<dyn ITexture, _>(
                concrete.this.upgrade().unwrap(),
                |v| v,
            ));
    }

    #[inline]
    fn clamp_sub_resource_set_to_texture(
        &self,
        texture: &TextureDesc,
        sub_resources: &TextureSubResourceSet,
    ) -> TextureSubResourceSet {
        #[inline(always)]
        fn clamp_range(base: u32, len: u32, min: u32, max: u32) -> (u32, u32) {
            let base_level = base.clamp(min, max);
            let end_level = base_level + len;
            let end_level_clamped = end_level.clamp(min, max);
            let num_levels = (end_level_clamped + 1) - base_level;
            (base_level, num_levels)
        }

        let min_mip_level = 0;
        let max_mip_level = texture.mip_levels - 1;
        let (base_mip_level, num_mip_levels) = clamp_range(
            sub_resources.base_mip_level,
            sub_resources.num_mip_levels,
            min_mip_level,
            max_mip_level,
        );

        let min_array_slice = 0;
        let max_array_slice = texture.array_size - 1;
        let (base_array_slice, num_array_slices) = clamp_range(
            sub_resources.base_array_slice,
            sub_resources.num_array_slices,
            min_array_slice,
            max_array_slice,
        );

        // Warn if the base mip level is out of bounds
        if sub_resources.base_mip_level >= max_mip_level {
            aleph_log::debug!("TextureSubResourceSet out of bounds: base_mip_level");
        }

        // Warn if the sub resource set is addressing the mip levels out of bounds unintentionally
        //
        // u32::MAX flags "use all mip levels"
        let use_all_mips = texture.mip_levels == u32::MAX;
        if (base_mip_level + num_mip_levels) > max_mip_level && !use_all_mips {
            aleph_log::debug!("TextureSubResourceSet out of bounds: num_mip_levels")
        }

        // Warn if the base array slice is out of bounds
        if sub_resources.base_array_slice >= max_array_slice {
            aleph_log::debug!("TextureSubResourceSet out of bounds: base_array_slice");
        }

        // Warn if the sub resource set is addressing the array slices out of bounds unintentionally
        //
        // u32::MAX flags "use all array slices"
        let use_all_slices = texture.array_size == u32::MAX;
        if (base_array_slice + num_array_slices) > max_array_slice && !use_all_slices {
            aleph_log::debug!("TextureSubResourceSet out of bounds: num_array_slices")
        }

        TextureSubResourceSet {
            base_mip_level,
            num_mip_levels,
            base_array_slice,
            num_array_slices,
        }
    }
}

impl<'a> IGeneralEncoder for Encoder<'a> {
    unsafe fn bind_graphics_pipeline(&mut self, pipeline: &dyn IGraphicsPipeline) {
        if let Some(concrete) = pipeline.query_interface::<GraphicsPipeline>() {
            // Binds the pipeline
            self.list.set_pipeline_state(&concrete.pipeline);

            // A pipeline is inseparable from its' root signature so we need to bind it here too
            self.list
                .set_graphics_root_signature(&concrete.pipeline_layout.root_signature);

            // Vulkan specifies the full primitive topology in the pipeline, unlike D3D12 which
            // defers the full specification to this call below. Vulkan can't implement D3D12's
            // behavior so we have to be like vulkan here so we also set the primitive topology
            self.list
                .ia_set_primitive_topology(concrete.primitive_topology);

            // Update the state for input binding strides. These get read when binding vertex
            // buffers to fill in the 'stride' field. Vulkan bakes these into the pipeline where
            // d3d12 takes them in 'IASetVertexBuffers'.
            //
            // TODO: Consider whether we just expose the parameter in the call and pipeline
            //       creation.
            self.input_binding_strides = concrete.input_binding_strides;
        } else {
            panic!("Unknown IGraphicsPipeline implementation");
        }
    }

    unsafe fn bind_vertex_buffers(
        &mut self,
        first_binding: u32,
        bindings: &[InputAssemblyBufferBinding],
    ) {
        let views: Vec<dx12::VertexBufferView> = bindings
            .iter()
            .enumerate()
            .map(|(i, v)| {
                let buffer = v
                    .buffer
                    .query_interface::<Buffer>()
                    .expect("Unkonwn IBuffer implementation");

                let buffer_location = buffer.base_address;
                let buffer_location = buffer_location.add(v.offset);

                let size_in_bytes = buffer.desc.size as u32;

                dx12::VertexBufferView {
                    buffer_location,
                    size_in_bytes,
                    stride_in_bytes: self.input_binding_strides[i + first_binding as usize],
                }
            })
            .collect();
        self.list.ia_set_vertex_buffers(first_binding, &views);
    }

    unsafe fn bind_index_buffer(
        &mut self,
        index_type: IndexType,
        binding: &InputAssemblyBufferBinding,
    ) {
        let buffer = binding
            .buffer
            .query_interface::<Buffer>()
            .expect("Unknown IBuffer implementation");

        let buffer_location = buffer.base_address;
        let buffer_location = buffer_location.add(binding.offset);

        let size_in_bytes = buffer.desc.size as u32;

        let view = dx12::IndexBufferView {
            buffer_location,
            size_in_bytes,
            format: match index_type {
                IndexType::U16 => dxgi::Format::R16Uint,
                IndexType::U32 => dxgi::Format::R32Uint,
            },
        };
        self.list.ia_set_index_buffer(&view);
    }

    unsafe fn set_scissor_rects(&mut self, rects: &[Rect]) {
        // TODO: bump allocator on self for temp allocations like this
        let rects: Vec<dx12::Rect> = rects
            .iter()
            .map(|v| dx12::Rect {
                left: v.x as i32,
                top: v.y as i32,
                right: (v.x + v.w) as i32,
                bottom: (v.y + v.h) as i32,
            })
            .collect();
        self.list.rs_set_scissor_rects(&rects);
    }

    unsafe fn clear_texture(
        &mut self,
        texture: &dyn ITexture,
        sub_resources: &TextureSubResourceSet,
        value: &ColorClearValue,
    ) {
        if let Some(concrete) = texture.query_interface::<Texture>() {
            self.clear_plain_texture(concrete, sub_resources, value);
        } else if let Some(concrete) = texture.query_interface::<SwapTexture>() {
            self.clear_swap_texture(concrete, value);
        } else {
            panic!("Unknown ITexture implementation");
        }
    }

    unsafe fn clear_depth_stencil_texture(
        &mut self,
        texture: &dyn ITexture,
        sub_resources: &TextureSubResourceSet,
        value: &DepthStencilClearValue,
    ) {
        if let Some(concrete) = texture.query_interface::<Texture>() {
            self.clear_depth_image(concrete, sub_resources, value);
        } else if texture.query_interface::<SwapTexture>().is_some() {
            aleph_log::debug!("Tried to clear swap chain image as a depth stencil texture");
        } else {
            panic!("Unknown ITexture implementation");
        }
    }

    unsafe fn draw(
        &mut self,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    ) {
        self.list
            .draw_instanced(vertex_count, instance_count, first_vertex, first_instance)
    }

    unsafe fn draw_indexed(
        &mut self,
        index_count: u32,
        instance_count: u32,
        first_index: u32,
        first_instance: u32,
        vertex_offset: i32,
    ) {
        self.list.draw_indexed_instanced(
            index_count,
            instance_count,
            first_index,
            vertex_offset,
            first_instance,
        )
    }
}

impl<'a> IComputeEncoder for Encoder<'a> {
    unsafe fn resource_barrier(
        &mut self,
        buffer_barriers: &[BufferBarrier],
        texture_barriers: &[TextureBarrier],
    ) {
        let buffer_barriers = buffer_barriers
            .iter()
            .filter_map(|v| {
                // Filter out any buffers from foreign backends
                if let Some(b) = v.buffer.query_interface::<Buffer>() {
                    Some((b, v))
                } else {
                    None
                }
            })
            .filter(|(b, _v)| {
                // Filter out any non GPU visible buffers as resource transitions mean nothing for
                // CPU only resources
                !matches!(b.desc.cpu_access, CpuAccessMode::None)
            })
            .map(|(b, v)| {
                let old_uav = v.before_state == ResourceStates::UNORDERED_ACCESS;
                let new_uav = v.after_state == ResourceStates::UNORDERED_ACCESS;
                if old_uav && new_uav {
                    dx12::ResourceBarrier::UAV {
                        flags: Default::default(),
                        resource: Some(b.resource.clone()),
                    }
                } else {
                    dx12::ResourceBarrier::Transition {
                        flags: Default::default(),
                        resource: Some(b.resource.clone()),
                        subresource: u32::MAX,
                        state_before: resource_state_to_dx12(v.before_state),
                        state_after: resource_state_to_dx12(v.after_state),
                    }
                }
            });

        let texture_barriers = texture_barriers
            .iter()
            .filter_map(|v| {
                // Filter out any textures from foreign backends
                if let Some(t) = v.texture.query_interface::<Texture>() {
                    Some(((&t.resource, &t.desc), v))
                } else if let Some(t) = v.texture.query_interface::<SwapTexture>() {
                    Some(((&t.resource, &t.desc), v))
                } else {
                    None
                }
            })
            .map(|(t, v)| {
                let old_uav = v.before_state == ResourceStates::UNORDERED_ACCESS;
                let new_uav = v.after_state == ResourceStates::UNORDERED_ACCESS;
                if old_uav && new_uav {
                    dx12::ResourceBarrier::UAV {
                        flags: Default::default(),
                        resource: Some(t.0.clone()),
                    }
                } else {
                    // Translate the split barrier mode request
                    let flags = match v.split_barrier_mode {
                        SplitBarrierMode::None => dx12::ResourceBarrierFlags::NONE,
                        SplitBarrierMode::Begin => dx12::ResourceBarrierFlags::BEGIN_ONLY,
                        SplitBarrierMode::End => dx12::ResourceBarrierFlags::END_ONLY,
                    };

                    let subresource = if let Some(o) = v.subresource {
                        calc_subresource_index(
                            o.mip_level as _,
                            o.array_layer as _,
                            0,
                            t.1.mip_levels,
                            t.1.array_size,
                        )
                    } else {
                        u32::MAX
                    };

                    let (state_before, state_after) = match v.queue_transition_mode {
                        QueueTransitionMode::None => (
                            resource_state_to_dx12(v.before_state),
                            resource_state_to_dx12(v.after_state),
                        ),
                        QueueTransitionMode::Acquire(_) => (
                            dx12::ResourceStates::COMMON,
                            resource_state_to_dx12(v.after_state),
                        ),
                        QueueTransitionMode::Release(_) => (
                            resource_state_to_dx12(v.before_state),
                            dx12::ResourceStates::COMMON,
                        ),
                    };

                    dx12::ResourceBarrier::Transition {
                        flags,
                        resource: Some(t.0.clone()),
                        subresource,
                        state_before,
                        state_after,
                    }
                }
            });

        let barriers = buffer_barriers.chain(texture_barriers);

        self.list.resource_barrier_dynamic(barriers);
    }

    unsafe fn dispatch(&mut self, group_count_x: u32, group_count_y: u32, group_count_z: u32) {
        self.list
            .dispatch(group_count_x, group_count_y, group_count_z);
    }
}

impl<'a> ITransferEncoder for Encoder<'a> {}
