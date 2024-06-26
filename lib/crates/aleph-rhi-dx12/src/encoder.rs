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

use std::any::TypeId;
use std::marker::PhantomData;
use std::mem::transmute_copy;
use std::ptr::NonNull;

use aleph_any::AnyArc;
use aleph_rhi_api::*;
use aleph_rhi_impl_utils::try_clone_value_into_slot;
use bumpalo::collections::Vec as BumpVec;
use bumpalo::Bump;
use pix::{begin_event_on_list, end_event_on_list, set_marker_on_list};
use windows::Win32::Foundation::RECT;
use windows::Win32::Graphics::Direct3D12::*;
use windows::Win32::Graphics::Dxgi::Common::*;

use crate::command_list::CommandList;
use crate::internal::conv::{
    barrier_access_to_dx12, barrier_sync_to_dx12, image_layout_to_dx12,
    translate_barrier_texture_aspect_to_plane_range, translate_rendering_color_attachment,
    translate_rendering_depth_stencil_attachment,
};
use crate::internal::descriptor_set::DescriptorSet;
use crate::internal::unwrap;
use crate::pipeline::{ComputePipeline, GraphicsPipeline};
use crate::texture::ImageViewObject;

pub struct Encoder<'a> {
    pub(crate) _list: ID3D12GraphicsCommandList7,
    pub(crate) _queue_type: QueueType,
    pub(crate) bound_graphics_pipeline: Option<AnyArc<GraphicsPipeline>>,
    pub(crate) bound_compute_pipeline: Option<AnyArc<ComputePipeline>>,
    pub(crate) input_binding_strides: [u32; 16],
    pub(crate) arena: Bump,
    pub(crate) phantom_data: PhantomData<&'a mut CommandList>,
    pub(crate) bound_graphics_sets: Box<[Option<DescriptorSetHandle>]>,
    pub(crate) bound_compute_sets: Box<[Option<DescriptorSetHandle>]>,
}

impl<'a> Drop for Encoder<'a> {
    fn drop(&mut self) {
        // TODO: Consider an API that forces manually closing so we can avoid the unwrap here
        unsafe { self._list.Close().unwrap() }
    }
}

impl<'a> IGetPlatformInterface for Encoder<'a> {
    unsafe fn __query_platform_interface(&self, target: TypeId, out: *mut ()) -> Option<()> {
        try_clone_value_into_slot::<ID3D12GraphicsCommandList7>(&self._list, out, target)
    }
}

impl<'a> IGeneralEncoder for Encoder<'a> {
    unsafe fn bind_graphics_pipeline(&mut self, pipeline: &dyn IGraphicsPipeline) {
        let concrete = unwrap::graphics_pipeline(pipeline);

        // Binds the pipeline
        self._list.SetPipelineState(&concrete.pipeline);

        // A pipeline is inseparable from its' root signature so we need to bind it here too
        self._list
            .SetGraphicsRootSignature(&concrete.pipeline_layout.root_signature);

        // Vulkan specifies the full primitive topology in the pipeline, unlike D3D12 which
        // defers the full specification to this call below. Vulkan can't implement D3D12's
        // behavior so we have to be like vulkan here so we also set the primitive topology
        self._list
            .IASetPrimitiveTopology(concrete.primitive_topology);

        if let Some((min, max)) = concrete.depth_bounds {
            self._list.OMSetDepthBounds(min, max);
        }

        // Update the state for input binding strides. These get read when binding vertex
        // buffers to fill in the 'stride' field. Vulkan bakes these into the pipeline where
        // d3d12 takes them in 'IASetVertexBuffers'.
        self.input_binding_strides = concrete.input_binding_strides;

        // We need the currently bound pipeline while recording commands to access things like
        // the pipeline layout for handling binding descriptors.
        self.bound_graphics_pipeline = Some(concrete.this.upgrade().unwrap());
        self.bound_graphics_sets.iter_mut().for_each(|v| *v = None);
    }

    unsafe fn bind_vertex_buffers(
        &mut self,
        first_binding: u32,
        bindings: &[InputAssemblyBufferBinding],
    ) {
        let mut views: BumpVec<D3D12_VERTEX_BUFFER_VIEW> =
            BumpVec::with_capacity_in(bindings.len(), &self.arena);
        for (i, v) in bindings.iter().enumerate() {
            let buffer = unwrap::buffer(v.buffer);

            let buffer_location = buffer.base_address;
            let buffer_location = buffer_location.add(v.offset);

            let size_in_bytes = buffer.desc.size as u32;

            let view = D3D12_VERTEX_BUFFER_VIEW {
                BufferLocation: buffer_location.get_inner().get(),
                SizeInBytes: size_in_bytes,
                StrideInBytes: self.input_binding_strides[i + first_binding as usize],
            };
            views.push(view);
        }

        self._list.IASetVertexBuffers(first_binding, Some(&views));
    }

    unsafe fn bind_index_buffer(
        &mut self,
        index_type: IndexType,
        binding: &InputAssemblyBufferBinding,
    ) {
        let buffer = unwrap::buffer(binding.buffer);

        let buffer_location = buffer.base_address;
        let buffer_location = buffer_location.add(binding.offset);

        let size_in_bytes = buffer.desc.size as u32;

        let view = D3D12_INDEX_BUFFER_VIEW {
            BufferLocation: buffer_location.get_inner().get(),
            SizeInBytes: size_in_bytes,
            Format: match index_type {
                IndexType::U16 => DXGI_FORMAT_R16_UINT,
                IndexType::U32 => DXGI_FORMAT_R32_UINT,
            },
        };
        self._list.IASetIndexBuffer(Some(&view));
    }

    unsafe fn set_viewports(&mut self, viewports: &[Viewport]) {
        let mut new_viewports: BumpVec<D3D12_VIEWPORT> =
            BumpVec::with_capacity_in(viewports.len(), &self.arena);
        for v in viewports {
            new_viewports.push(D3D12_VIEWPORT {
                TopLeftX: v.x,
                TopLeftY: v.y,
                Width: v.width,
                Height: v.height,
                MinDepth: v.min_depth,
                MaxDepth: v.max_depth,
            });
        }

        self._list.RSSetViewports(&new_viewports);
    }

    unsafe fn set_scissor_rects(&mut self, rects: &[Rect]) {
        let mut new_rects: BumpVec<RECT> = BumpVec::with_capacity_in(rects.len(), &self.arena);
        for v in rects {
            new_rects.push(RECT {
                left: v.x as i32,
                top: v.y as i32,
                right: (v.x + v.w) as i32,
                bottom: (v.y + v.h) as i32,
            });
        }

        self._list.RSSetScissorRects(&new_rects);
    }

    unsafe fn set_push_constant_block(&mut self, block_index: usize, data: &[u8]) {
        // This command can't work without a bound pipeline, we need the pipeline layout so we can
        // know where in the root signature to write the data
        let pipeline = self.bound_graphics_pipeline.as_deref().unwrap();

        // Lookup the parameter index on the currently bound pipeline (pipeline layout) based on
        // the constant block index
        let block = &pipeline.pipeline_layout.push_constant_blocks[block_index];

        let num32_bit_values_to_set = (data.len() / 4) as u32;
        let p_src_data = data.as_ptr();
        self._list.SetGraphicsRoot32BitConstants(
            block.root_parameter_index,
            num32_bit_values_to_set,
            p_src_data as *const _,
            0,
        );
    }

    unsafe fn begin_rendering(&mut self, info: &BeginRenderingInfo) {
        let mut color_attachments =
            BumpVec::with_capacity_in(info.color_attachments.len(), &self.arena);

        for attachment in info.color_attachments {
            let view = std::mem::transmute::<_, *const ImageViewObject>(attachment.image_view);
            let descriptor = (*view).handle;
            let format = (*view).format;
            color_attachments.push(translate_rendering_color_attachment(
                attachment,
                descriptor,
                Some(format),
            ));
        }

        let depth_stencil = info.depth_stencil_attachment.map(|attachment| {
            let view = std::mem::transmute::<_, *const ImageViewObject>(attachment.image_view);
            let descriptor = (*view).handle;
            let format = (*view).format;
            translate_rendering_depth_stencil_attachment(attachment, descriptor, Some(format))
        });

        let depth_stencil_ref = depth_stencil
            .as_ref()
            .map(|v| v as *const _)
            .unwrap_or(std::ptr::null());

        let flags = if info.allow_uav_writes {
            D3D12_RENDER_PASS_FLAG_ALLOW_UAV_WRITES
        } else {
            D3D12_RENDER_PASS_FLAG_NONE
        };

        self._list
            .BeginRenderPass(Some(&color_attachments), Some(depth_stencil_ref), flags);
    }

    unsafe fn end_rendering(&mut self) {
        self._list.EndRenderPass();
    }

    unsafe fn draw(
        &mut self,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    ) {
        self._list
            .DrawInstanced(vertex_count, instance_count, first_vertex, first_instance)
    }

    unsafe fn draw_indexed(
        &mut self,
        index_count: u32,
        instance_count: u32,
        first_index: u32,
        first_instance: u32,
        vertex_offset: i32,
    ) {
        self._list.DrawIndexedInstanced(
            index_count,
            instance_count,
            first_index,
            vertex_offset,
            first_instance,
        )
    }
}

impl<'a> IComputeEncoder for Encoder<'a> {
    unsafe fn bind_compute_pipeline(&mut self, pipeline: &dyn IComputePipeline) {
        let concrete = unwrap::compute_pipeline(pipeline);

        // Binds the pipeline
        self._list.SetPipelineState(&concrete.pipeline);

        // A pipeline is inseparable from its' root signature so we need to bind it here too
        self._list
            .SetComputeRootSignature(&concrete.pipeline_layout.root_signature);

        // We need the currently bound pipeline while recording commands to access things like
        // the pipeline layout for handling binding descriptors.
        self.bound_compute_pipeline = Some(concrete.this.upgrade().unwrap());
        self.bound_compute_sets.iter_mut().for_each(|v| *v = None);
    }

    unsafe fn bind_descriptor_sets(
        &mut self,
        pipeline_layout: &dyn IPipelineLayout,
        bind_point: PipelineBindPoint,
        first_set: u32,
        sets: &[DescriptorSetHandle],
        dynamic_offsets: &[u32],
    ) {
        let pipeline_layout = unwrap::pipeline_layout(pipeline_layout);

        let bind_fn = match bind_point {
            PipelineBindPoint::Compute => set_compute_descriptor_table,
            PipelineBindPoint::Graphics => set_graphics_descriptor_table,
        };

        let dynamic_bind_fn = match bind_point {
            PipelineBindPoint::Compute => set_compute_root_constant_buffer_view,
            PipelineBindPoint::Graphics => set_graphics_root_constant_buffer_view,
        };

        let mut dynamic_offsets = dynamic_offsets;
        for (set_index, set) in sets.iter().enumerate() {
            // Safety: No checks, all up to the caller to ensure this is safe
            let v: NonNull<()> = (*set).into();
            let v: NonNull<DescriptorSet> = v.cast();
            let v = v.as_ref();

            // Computes the index of the set within the pipeline layout.
            let set_global_index = first_set as usize + set_index;

            // Fetch the base root parameter index for this set from the pipeline layout
            let param_index = pipeline_layout.set_root_param_indices[set_global_index];

            // We always place dynamic constant buffers before the tables in the root signature so
            // they get treated as 'higher priority'
            let dynamic_constant_buffers = v.dynamic_constant_buffers.as_ref();

            // Create a sub-slice of the dynamic offsets list that contains the offsets that
            // apply for the current descriptor set
            let set_dynamic_offsets = &dynamic_offsets[0..dynamic_constant_buffers.len()];

            // Iterate over the dynamic constant buffers and matching offsets and update the
            // root buffer views with the new dynamic buffer offset
            for (&dynamic_cb, &offset) in dynamic_constant_buffers.iter().zip(set_dynamic_offsets) {
                let offset = offset as u64;
                let dynamic_cb = dynamic_cb + offset;
                dynamic_bind_fn(self, param_index, dynamic_cb);
            }

            // Consume the offsets we just updated ready for the next set
            dynamic_offsets = set_dynamic_offsets;

            let bound_sets = match bind_point {
                PipelineBindPoint::Compute => &mut self.bound_compute_sets,
                PipelineBindPoint::Graphics => &mut self.bound_graphics_sets,
            };
            let already_bound_set = bound_sets[set_global_index];
            if already_bound_set != Some(*set) {
                bound_sets[set_global_index] = Some(*set);

                // First bind the resource descriptors, which will always take a single descriptor table
                // slot
                let mut param_index = param_index + dynamic_constant_buffers.len() as u32;
                if let Some(handle) = v.resource_handle_gpu {
                    bind_fn(self, param_index, handle.into());

                    // Increment the param index, as the samplers will always be placed in the following
                    // root parameter indices.
                    param_index += 1;
                }

                // Next we bind the samplers, which are separated out and bound as one table for each
                // sampler to work around limits making a single table of samplers impractical.
                let samplers = v.samplers.as_ref();
                for (i, sampler) in samplers.iter().cloned().enumerate() {
                    bind_fn(self, param_index + i as u32, sampler.unwrap().into());
                }
            }
        }
    }

    unsafe fn dispatch(&mut self, group_count_x: u32, group_count_y: u32, group_count_z: u32) {
        self._list
            .Dispatch(group_count_x, group_count_y, group_count_z);
    }
}

impl<'a> ITransferEncoder for Encoder<'a> {
    unsafe fn resource_barrier(
        &mut self,
        global_barriers: &[GlobalBarrier],
        buffer_barriers: &[BufferBarrier],
        texture_barriers: &[TextureBarrier],
    ) {
        #![allow(non_snake_case)]

        let mut translated_global_barriers =
            BumpVec::with_capacity_in(global_barriers.len(), &self.arena);
        let mut translated_buffer_barriers =
            BumpVec::with_capacity_in(buffer_barriers.len(), &self.arena);
        let mut translated_texture_barriers =
            BumpVec::with_capacity_in(texture_barriers.len(), &self.arena);
        let mut barrier_groups = BumpVec::with_capacity_in(3, &self.arena);

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
                Type: D3D12_BARRIER_TYPE_GLOBAL,
                NumBarriers: translated_global_barriers.len() as _,
                Anonymous: D3D12_BARRIER_GROUP_0 {
                    pGlobalBarriers: translated_global_barriers.as_ptr(),
                },
            });
        }

        if !buffer_barriers.is_empty() {
            for barrier in buffer_barriers {
                // Grab the d3d12 resource handle
                let resource = unwrap::buffer(barrier.buffer);

                translated_buffer_barriers.push(D3D12_BUFFER_BARRIER {
                    SyncBefore: barrier_sync_to_dx12(barrier.before_sync),
                    SyncAfter: barrier_sync_to_dx12(barrier.after_sync),
                    AccessBefore: barrier_access_to_dx12(barrier.before_access),
                    AccessAfter: barrier_access_to_dx12(barrier.after_access),
                    pResource: transmute_copy(&resource.resource),
                    Offset: barrier.offset,
                    Size: barrier.size,
                });
            }

            barrier_groups.push(D3D12_BARRIER_GROUP {
                Type: D3D12_BARRIER_TYPE_BUFFER,
                NumBarriers: translated_buffer_barriers.len() as _,
                Anonymous: D3D12_BARRIER_GROUP_0 {
                    pBufferBarriers: translated_buffer_barriers.as_ptr(),
                },
            });
        }

        if !texture_barriers.is_empty() {
            for barrier in texture_barriers {
                // Grab the d3d12 resource handle from our texture impls
                let texture = unwrap::texture(barrier.texture);

                // Vulkan initializes layout metadata automatically when transitioning from
                // undefined to a compressed layout. D3D12 requires a flag to force it, otherwise
                // we need to issue another command. To match behaviour we always use the flag.
                //
                // I would be surprised if this affects performance in any meaningful way, this
                // should only initialize the layout metadata and not the actual data unlike a full
                // clear.
                let Flags = if barrier.before_layout == ImageLayout::Undefined {
                    D3D12_TEXTURE_BARRIER_FLAG_DISCARD
                } else {
                    D3D12_TEXTURE_BARRIER_FLAGS::default()
                };

                let (first_plane, num_planes) = translate_barrier_texture_aspect_to_plane_range(
                    barrier.subresource_range.aspect,
                    texture.desc.format,
                );

                let (layout_before, layout_after, access_before, access_after) =
                    if let Some(transition) = barrier.queue_transition {
                        // A queue transition requires a more complex translation, where we
                        // transition into and out of common in the release/acquire and flag
                        // NO_ACCESS in the release/acquire edge.
                        self.barrier_args_with_queue_transition(barrier, transition)
                    } else {
                        // In the no transition state we just translate the layout and access
                        // directly
                        (
                            image_layout_to_dx12(barrier.before_layout, Some(self._queue_type)),
                            image_layout_to_dx12(barrier.after_layout, Some(self._queue_type)),
                            barrier_access_to_dx12(barrier.before_access),
                            barrier_access_to_dx12(barrier.after_access),
                        )
                    };

                translated_texture_barriers.push(D3D12_TEXTURE_BARRIER {
                    SyncBefore: barrier_sync_to_dx12(barrier.before_sync),
                    SyncAfter: barrier_sync_to_dx12(barrier.after_sync),
                    AccessBefore: access_before,
                    AccessAfter: access_after,
                    LayoutBefore: layout_before,
                    LayoutAfter: layout_after,
                    pResource: transmute_copy(&texture.resource),
                    Subresources: D3D12_BARRIER_SUBRESOURCE_RANGE {
                        IndexOrFirstMipLevel: barrier.subresource_range.base_mip_level,
                        NumMipLevels: barrier.subresource_range.num_mip_levels,
                        FirstArraySlice: barrier.subresource_range.base_array_slice,
                        NumArraySlices: barrier.subresource_range.num_array_slices,
                        FirstPlane: first_plane,
                        NumPlanes: num_planes,
                    },
                    Flags,
                });
            }
            barrier_groups.push(D3D12_BARRIER_GROUP {
                Type: D3D12_BARRIER_TYPE_TEXTURE,
                NumBarriers: translated_texture_barriers.len() as _,
                Anonymous: D3D12_BARRIER_GROUP_0 {
                    pTextureBarriers: translated_texture_barriers.as_ptr(),
                },
            });
        }

        self._list.Barrier(&barrier_groups);
    }

    unsafe fn copy_buffer_regions(
        &mut self,
        src: &dyn IBuffer,
        dst: &dyn IBuffer,
        regions: &[BufferCopyRegion],
    ) {
        let src = unwrap::buffer(src);
        let dst = unwrap::buffer(dst);

        for region in regions {
            self._list.CopyBufferRegion(
                &dst.resource,
                region.dst_offset,
                &src.resource,
                region.src_offset,
                region.size,
            );
        }
    }

    unsafe fn copy_buffer_to_texture(
        &mut self,
        src: &dyn IBuffer,
        dst: &dyn ITexture,
        _dst_layout: ImageLayout,
        regions: &[BufferToTextureCopyRegion],
    ) {
        let src = unwrap::buffer(src);
        let dst = unwrap::texture(dst);

        let bytes_per_element = dst.desc.format.bytes_per_element();
        let mut src_location = D3D12_TEXTURE_COPY_LOCATION {
            pResource: unsafe { transmute_copy(&src.resource) },
            Type: D3D12_TEXTURE_COPY_TYPE_PLACED_FOOTPRINT,
            Anonymous: D3D12_TEXTURE_COPY_LOCATION_0 {
                PlacedFootprint: D3D12_PLACED_SUBRESOURCE_FOOTPRINT {
                    Offset: 0,
                    Footprint: D3D12_SUBRESOURCE_FOOTPRINT {
                        Format: dst.dxgi_format,
                        Width: 0,
                        Height: 0,
                        Depth: 0,
                        RowPitch: 0,
                    },
                },
            },
        };

        let mut dst_location = D3D12_TEXTURE_COPY_LOCATION {
            pResource: unsafe { transmute_copy(&dst.resource) },
            Type: D3D12_TEXTURE_COPY_TYPE_SUBRESOURCE_INDEX,
            Anonymous: D3D12_TEXTURE_COPY_LOCATION_0 {
                SubresourceIndex: 0,
            },
        };

        for region in regions {
            // Vulkan can only copy starting at (0, 0, 0). The feature can't be trivially emulated
            // so we don't expose a src offset.
            //
            // Thus 'left', 'top', 'front' will always be 0.
            let src_box = D3D12_BOX {
                left: 0,
                top: 0,
                front: 0,
                right: region.dst.extent.width,
                bottom: region.dst.extent.height,
                back: region.dst.extent.depth,
            };

            let index = dst.subresource_index_for(
                region.dst.mip_level,
                region.dst.array_layer,
                region.dst.aspect,
            );
            dst_location.Anonymous.SubresourceIndex = index.unwrap_or(0);

            // Translate the source layout description to D3D12's 'subresource footprint'
            let footprint = &mut src_location.Anonymous.PlacedFootprint;
            footprint.Offset = region.src.offset;
            footprint.Footprint.Width = region.src.extent.width;
            footprint.Footprint.Height = region.src.extent.height;
            footprint.Footprint.Depth = region.src.extent.depth;
            footprint.Footprint.RowPitch = region.src.extent.width * bytes_per_element;

            self._list.CopyTextureRegion(
                &dst_location,
                region.dst.origin.x,
                region.dst.origin.y,
                region.dst.origin.z,
                &src_location,
                Some(&src_box),
            );
        }
    }

    unsafe fn copy_texture_regions(
        &mut self,
        src: &dyn ITexture,
        dst: &dyn ITexture,
        regions: &[TextureToTextureCopyInfo],
    ) {
        let src = unwrap::texture(src);
        let dst = unwrap::texture(dst);

        for region in regions {
            let subresource = src
                .subresource_index_for(
                    region.src.mip_level,
                    region.src.array_layer,
                    region.src.aspect,
                )
                .unwrap();
            let src = D3D12_TEXTURE_COPY_LOCATION {
                pResource: unsafe { transmute_copy(&src.resource) },
                Type: D3D12_TEXTURE_COPY_TYPE_SUBRESOURCE_INDEX,
                Anonymous: D3D12_TEXTURE_COPY_LOCATION_0 {
                    SubresourceIndex: subresource,
                },
            };

            let subresource = dst
                .subresource_index_for(
                    region.dst.mip_level,
                    region.dst.array_layer,
                    region.dst.aspect,
                )
                .unwrap();
            let dst = D3D12_TEXTURE_COPY_LOCATION {
                pResource: unsafe { transmute_copy(&dst.resource) },
                Type: D3D12_TEXTURE_COPY_TYPE_SUBRESOURCE_INDEX,
                Anonymous: D3D12_TEXTURE_COPY_LOCATION_0 {
                    SubresourceIndex: subresource,
                },
            };

            let src_box = D3D12_BOX {
                left: region.src.offset.x,
                top: region.src.offset.y,
                front: region.src.offset.z,
                right: region.src.offset.x + region.extent.width,
                bottom: region.src.offset.y + region.extent.height,
                back: region.src.offset.z + region.extent.depth,
            };

            self._list.CopyTextureRegion(
                &dst,
                region.dst.offset.x,
                region.dst.offset.y,
                region.dst.offset.z,
                &src,
                Some(&src_box),
            );
        }
    }

    unsafe fn set_marker(&mut self, color: Color, message: &str) {
        set_marker_on_list(&self._list, color.0.into(), message);
    }

    unsafe fn begin_event(&mut self, color: Color, message: &str) {
        begin_event_on_list(&self._list, color.0.into(), message);
    }

    unsafe fn end_event(&mut self) {
        end_event_on_list(&self._list);
    }
}

impl<'a> Encoder<'a> {
    fn barrier_args_with_queue_transition(
        &self,
        barrier: &TextureBarrier,
        transition: QueueTransition,
    ) -> (
        D3D12_BARRIER_LAYOUT,
        D3D12_BARRIER_LAYOUT,
        D3D12_BARRIER_ACCESS,
        D3D12_BARRIER_ACCESS,
    ) {
        if transition.before_queue == transition.after_queue {
            panic!(
                "Trying to transition from queue {:?} to queue {:?}, but they're the same queue",
                transition.before_queue, transition.after_queue
            );
        }

        let (layout_before, access_before) = if transition.before_queue == self._queue_type {
            // If the before queue and current queue are the same then we issue the layout and
            // access like normal to make all the work on the before queue available to the
            // transition operation.
            (
                image_layout_to_dx12(barrier.before_layout, Some(self._queue_type)),
                barrier_access_to_dx12(barrier.before_access),
            )
        } else {
            // If the before and current queue are different then this is the 'acquire' half of the
            // transition and so we should have no accesses before and we're transitioning from the
            // common layout to our target real layout.
            (D3D12_BARRIER_LAYOUT_COMMON, D3D12_BARRIER_ACCESS_NO_ACCESS)
        };

        // Handle the 'acquire' edge of the transition
        let (layout_after, access_after) = if transition.after_queue == self._queue_type {
            // Like in the before case, when 'after_queue' and the current queue are the same then
            // we should translate the layout and access as normal.
            (
                image_layout_to_dx12(barrier.after_layout, Some(self._queue_type)),
                barrier_access_to_dx12(barrier.after_access),
            )
        } else {
            // The mirror to the above before case, this is the 'release' half of the transition.
            // Here we transition to COMMON layout and say the resource is _not_ accessed after the
            // transition.
            (D3D12_BARRIER_LAYOUT_COMMON, D3D12_BARRIER_ACCESS_NO_ACCESS)
        };

        (layout_before, layout_after, access_before, access_after)
    }
}

unsafe fn set_compute_root_constant_buffer_view(
    encoder: &Encoder,
    rootparameterindex: u32,
    buffer_location: u64,
) {
    encoder
        ._list
        .SetComputeRootConstantBufferView(rootparameterindex, buffer_location)
}
unsafe fn set_graphics_root_constant_buffer_view(
    encoder: &Encoder,
    rootparameterindex: u32,
    buffer_location: u64,
) {
    encoder
        ._list
        .SetGraphicsRootConstantBufferView(rootparameterindex, buffer_location)
}

unsafe fn set_compute_descriptor_table(
    encoder: &Encoder,
    rootparameterindex: u32,
    basedescriptor: D3D12_GPU_DESCRIPTOR_HANDLE,
) {
    encoder
        ._list
        .SetComputeRootDescriptorTable(rootparameterindex, basedescriptor)
}
unsafe fn set_graphics_descriptor_table(
    encoder: &Encoder,
    rootparameterindex: u32,
    basedescriptor: D3D12_GPU_DESCRIPTOR_HANDLE,
) {
    encoder
        ._list
        .SetGraphicsRootDescriptorTable(rootparameterindex, basedescriptor)
}
