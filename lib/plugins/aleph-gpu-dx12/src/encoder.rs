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
use crate::command_list::CommandList;
use crate::internal::conv::{
    barrier_access_to_dx12, barrier_sync_to_dx12, decode_u32_color_to_float, image_layout_to_dx12,
    translate_rendering_color_attachment, translate_rendering_depth_stencil_attachment,
};
use crate::pipeline::GraphicsPipeline;
use crate::swap_texture::SwapTexture;
use crate::texture::Texture;
use aleph_windows::Win32::Graphics::Direct3D12::*;
use dx12::dxgi;
use interfaces::any::{AnyArc, QueryInterface};
use interfaces::gpu::{
    BeginRenderingInfo, BufferBarrier, ColorClearValue, DepthStencilClearValue, GlobalBarrier,
    IComputeEncoder, IGeneralEncoder, IGraphicsPipeline, ITexture, ITransferEncoder, ImageLayout,
    IndexType, InputAssemblyBufferBinding, Rect, TextureBarrier, TextureDesc,
    TextureSubResourceSet, Viewport,
};
use std::ops::Deref;

pub struct Encoder<'a> {
    pub(crate) list: dx12::GraphicsCommandList,
    pub(crate) parent: &'a mut CommandList,
    pub(crate) bound_graphics_pipeline: Option<AnyArc<GraphicsPipeline>>,
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

            // We need the currently bound pipeline while recording commands to access things like
            // the pipeline layout for handling binding descriptors.
            self.bound_graphics_pipeline = Some(concrete.this.upgrade().unwrap());
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

    unsafe fn set_viewports(&mut self, viewports: &[Viewport]) {
        let viewports: Vec<dx12::Viewport> = viewports
            .iter()
            .map(|v| dx12::Viewport {
                top_left_x: v.x,
                top_left_y: v.y,
                width: v.width,
                height: v.height,
                min_depth: v.min_depth,
                max_depth: v.max_depth,
            })
            .collect();
        self.list.rs_set_viewports(&viewports);
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

    unsafe fn set_push_constant_block(&mut self, block_index: usize, data: &[u8]) {
        // This command can't work without a bound pipeline, we need the pipeline layout so we can
        // know where in the root signature to write the data
        let pipeline = self
            .bound_graphics_pipeline
            .as_ref()
            .map(|v| v.deref())
            .unwrap();

        // Lookup the parameter index on the currently bound pipeline (pipeline layout) based on
        // the constant block index
        let block = &pipeline.pipeline_layout.push_constant_blocks[block_index];

        debug_assert!(
            data.len() % 4 == 0,
            "Push Constant data must have len divisible by 4"
        );

        debug_assert!(
            data.len() <= block.size as usize,
            "Push Constant data larger than the specified block"
        );

        self.list
            .set_graphics_root_32bit_constants(block.root_parameter_index, data, 0);
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

    unsafe fn begin_rendering(&mut self, info: &BeginRenderingInfo) {
        let mut color_attachments = Vec::with_capacity(info.color_attachments.len());
        for info in info.color_attachments {
            color_attachments.push(translate_rendering_color_attachment(
                info,
                Default::default(), // TODO: descriptor
                None,               // TODO: format
            ));
        }

        let depth_stencil = info.depth_stencil_attachment.map(|info| {
            translate_rendering_depth_stencil_attachment(
                info,
                Default::default(), // TODO: descriptor
                None,               // TODO: format
            )
        });

        let depth_stencil_ref = depth_stencil
            .as_ref()
            .map(|v| v as *const _)
            .unwrap_or(std::ptr::null());

        self.list.as_raw().BeginRenderPass(
            &color_attachments,
            depth_stencil_ref,
            D3D12_RENDER_PASS_FLAG_ALLOW_UAV_WRITES, // TODO: This *could* be suboptimal
        );
    }

    unsafe fn end_rendering(&mut self) {
        self.list.as_raw().EndRenderPass();
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
        global_barriers: &[GlobalBarrier],
        buffer_barriers: &[BufferBarrier],
        texture_barriers: &[TextureBarrier],
    ) {
        #![allow(non_snake_case)]
        // TODO: Bump allocator on the command buffer
        let mut translated_global_barriers: Vec<D3D12_GLOBAL_BARRIER> =
            Vec::with_capacity(global_barriers.len());
        let mut translated_buffer_barriers: Vec<D3D12_BUFFER_BARRIER> =
            Vec::with_capacity(buffer_barriers.len());
        let mut translated_texture_barriers: Vec<D3D12_TEXTURE_BARRIER> =
            Vec::with_capacity(texture_barriers.len());
        let mut barrier_groups: Vec<D3D12_BARRIER_GROUP> = Vec::with_capacity(3);

        if !buffer_barriers.is_empty() {
            for barrier in global_barriers {
                translated_global_barriers.push(D3D12_GLOBAL_BARRIER {
                    SyncBefore: barrier_sync_to_dx12(barrier.before_sync),
                    SyncAfter: barrier_sync_to_dx12(barrier.after_sync),
                    AccessBefore: barrier_access_to_dx12(barrier.before_access),
                    AccessAfter: barrier_access_to_dx12(barrier.after_access),
                });
            }

            barrier_groups.push(D3D12_BARRIER_GROUP {
                Type: D3D12_BARRIER_TYPE::GLOBAL,
                NumBarriers: translated_global_barriers.len() as _,
                Anonymous: D3D12_BARRIER_GROUP_0 {
                    pGlobalBarriers: translated_global_barriers.as_ptr(),
                },
            });
        }

        if !buffer_barriers.is_empty() {
            for barrier in buffer_barriers {
                // Grab the d3d12 resource handle
                let resource = barrier
                    .buffer
                    .query_interface::<Buffer>()
                    .expect("Unknown IBuffer implementation");

                translated_buffer_barriers.push(D3D12_BUFFER_BARRIER {
                    SyncBefore: barrier_sync_to_dx12(barrier.before_sync),
                    SyncAfter: barrier_sync_to_dx12(barrier.after_sync),
                    AccessBefore: barrier_access_to_dx12(barrier.before_access),
                    AccessAfter: barrier_access_to_dx12(barrier.after_access),
                    pResource: Some(resource.resource.as_raw().clone()),
                    Offset: barrier.offset,
                    Size: barrier.size,
                });
            }

            barrier_groups.push(D3D12_BARRIER_GROUP {
                Type: D3D12_BARRIER_TYPE::BUFFER,
                NumBarriers: translated_buffer_barriers.len() as _,
                Anonymous: D3D12_BARRIER_GROUP_0 {
                    pBufferBarriers: translated_buffer_barriers.as_ptr(),
                },
            });
        }

        if !texture_barriers.is_empty() {
            for barrier in texture_barriers {
                // Grab the d3d12 resource handle from our texture impls
                let resource = barrier
                    .texture
                    .query_interface::<Texture>()
                    .map(|v| v.resource.as_raw())
                    .unwrap_or_else(|| {
                        barrier
                            .texture
                            .query_interface::<SwapTexture>()
                            .expect("Unknown ITexture implementation")
                            .resource
                            .as_raw()
                    });

                // Vulkan initializes layout metadata automatically when transitioning from
                // undefined to a compressed layout. D3D12 requires a flag to force it, otherwise
                // we need to issue another command. To match behaviour we always use the flag.
                //
                // I would be surprised if this affects performance in any meaningful way, this
                // should only initialize the layout metadata and not the actual data unlike a full
                // clear.
                let Flags = if barrier.before_layout == ImageLayout::Undefined {
                    D3D12_TEXTURE_BARRIER_FLAGS::DISCARD
                } else {
                    D3D12_TEXTURE_BARRIER_FLAGS::empty()
                };

                translated_texture_barriers.push(D3D12_TEXTURE_BARRIER {
                    SyncBefore: barrier_sync_to_dx12(barrier.before_sync),
                    SyncAfter: barrier_sync_to_dx12(barrier.after_sync),
                    AccessBefore: barrier_access_to_dx12(barrier.before_access),
                    AccessAfter: barrier_access_to_dx12(barrier.after_access),
                    LayoutBefore: image_layout_to_dx12(barrier.before_layout),
                    LayoutAfter: image_layout_to_dx12(barrier.after_layout),
                    pResource: Some(resource.clone()),
                    Subresources: D3D12_BARRIER_SUBRESOURCE_RANGE {
                        IndexOrFirstMipLevel: barrier.subresource_range.base_mip_level,
                        NumMipLevels: barrier.subresource_range.num_mip_levels,
                        FirstArraySlice: barrier.subresource_range.base_array_slice,
                        NumArraySlices: barrier.subresource_range.num_array_slices,
                        FirstPlane: 0,
                        NumPlanes: 1,
                    },
                    Flags,
                });
            }
            barrier_groups.push(D3D12_BARRIER_GROUP {
                Type: D3D12_BARRIER_TYPE::TEXTURE,
                NumBarriers: translated_texture_barriers.len() as _,
                Anonymous: D3D12_BARRIER_GROUP_0 {
                    pTextureBarriers: translated_texture_barriers.as_ptr(),
                },
            });
        }

        self.list.as_raw().Barrier(
            barrier_groups.len() as _,
            if barrier_groups.is_empty() {
                std::ptr::null()
            } else {
                barrier_groups.as_ptr()
            },
        );
    }

    unsafe fn dispatch(&mut self, group_count_x: u32, group_count_y: u32, group_count_z: u32) {
        self.list
            .dispatch(group_count_x, group_count_y, group_count_z);
    }
}

impl<'a> ITransferEncoder for Encoder<'a> {}
