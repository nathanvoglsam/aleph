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
use std::mem::transmute_copy;
use std::ops::Deref;
use std::sync::Arc;

use aleph_object_system::Object;
use aleph_rhi_api::*;
use aleph_rhi_impl_utils::parameter_block_layout_visitor::ParameterBlockLayoutVisitor;
use aleph_rhi_impl_utils::{RhiSystem, try_clone_value_into_slot};
use allocator_api2::vec::Vec as BVec;
use blink_alloc::{Blink, BlinkAlloc};
use pix::{begin_event_cstr_on_list, end_event_on_list, set_marker_cstr_on_list};
use windows::Win32::Foundation::RECT;
use windows::Win32::Graphics::Direct3D12::*;
use windows::Win32::Graphics::Dxgi::Common::*;

use crate::buffer::Buffer;
use crate::command_list::{CommandList, ListState};
use crate::internal::conv::{
    barrier_access_to_dx12, barrier_sync_to_dx12, image_layout_to_dx12,
    translate_barrier_texture_aspect_to_plane_range, translate_rendering_color_attachment,
    translate_rendering_depth_stencil_attachment,
};
use crate::internal::parameter_block::ParameterBlock;
use crate::internal::unwrap;
use crate::pipeline::{ComputePipeline, GraphicsPipeline};
use crate::sampler::Sampler;
use crate::texture::{ImageViewObject, Texture};

pub struct Encoder<'a> {
    pub(crate) _parent: &'a mut CommandList,
    pub(crate) _list: ID3D12GraphicsCommandList7,
    pub(crate) _queue_type: QueueType,
    pub(crate) bound_graphics_pipeline: Option<Arc<Object<GraphicsPipeline>>>,
    pub(crate) bound_compute_pipeline: Option<Arc<Object<ComputePipeline>>>,
    pub(crate) input_binding_strides: [u32; 16],
    pub(crate) arena: Blink<BlinkAlloc<RhiSystem>>,
    pub(crate) bound_graphics_sets: Box<[Option<ParameterBlockHandle>]>,
    pub(crate) bound_compute_sets: Box<[Option<ParameterBlockHandle>]>,
}

impl<'a> IGetPlatformInterface for Encoder<'a> {
    unsafe fn __query_platform_interface(&self, target: TypeId, out: *mut ()) -> Option<()> {
        unsafe { try_clone_value_into_slot::<ID3D12GraphicsCommandList7>(&self._list, out, target) }
    }
}

impl<'a> ICommandEncoderAbi for Encoder<'a> {
    unsafe fn __bind_graphics_pipeline(&mut self, pipeline: &GraphicsPipelineHandle) {
        unsafe {
            let concrete = GraphicsPipeline::get_owned(pipeline);

            // Binds the pipeline
            self._list.SetPipelineState(&concrete.pipeline);

            // A pipeline is inseparable from its' root signature so we need to bind it here too
            self._list
                .SetGraphicsRootSignature(&concrete.binding_signature.root_signature);

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
            self.bound_graphics_pipeline = Some(concrete);
            self.bound_graphics_sets.iter_mut().for_each(|v| *v = None);
        }
    }

    unsafe fn __bind_vertex_buffers(
        &mut self,
        first_binding: u32,
        bindings: &[InputAssemblyBufferBinding],
    ) {
        let mut views: BVec<D3D12_VERTEX_BUFFER_VIEW, _> =
            BVec::with_capacity_in(bindings.len(), self.arena.allocator());
        for (i, v) in bindings.iter().enumerate() {
            let buffer = Buffer::get(v.buffer);

            let buffer_location = buffer.base_address;
            let buffer_location = buffer_location.add(v.offset);

            let size_in_bytes = buffer.desc().size as u32;

            let view = D3D12_VERTEX_BUFFER_VIEW {
                BufferLocation: buffer_location.get_inner().get(),
                SizeInBytes: size_in_bytes,
                StrideInBytes: self.input_binding_strides[i + first_binding as usize],
            };
            views.push(view);
        }

        unsafe {
            self._list.IASetVertexBuffers(first_binding, Some(&views));
        }
    }

    unsafe fn __bind_index_buffer(
        &mut self,
        index_type: IndexType,
        binding: &InputAssemblyBufferBinding,
    ) {
        let buffer = Buffer::get(binding.buffer);

        let buffer_location = buffer.base_address;
        let buffer_location = buffer_location.add(binding.offset);

        let size_in_bytes = buffer.desc().size as u32;

        let view = D3D12_INDEX_BUFFER_VIEW {
            BufferLocation: buffer_location.get_inner().get(),
            SizeInBytes: size_in_bytes,
            Format: match index_type {
                IndexType::U16 => DXGI_FORMAT_R16_UINT,
                IndexType::U32 => DXGI_FORMAT_R32_UINT,
            },
        };

        unsafe {
            self._list.IASetIndexBuffer(Some(&view));
        }
    }

    unsafe fn __set_viewports(&mut self, viewports: &[Viewport]) {
        let mut new_viewports: BVec<D3D12_VIEWPORT, _> =
            BVec::with_capacity_in(viewports.len(), self.arena.allocator());
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

        unsafe {
            self._list.RSSetViewports(&new_viewports);
        }
    }

    unsafe fn __set_scissor_rects(&mut self, rects: &[Rect]) {
        let mut new_rects: BVec<RECT, _> =
            BVec::with_capacity_in(rects.len(), self.arena.allocator());
        for v in rects {
            new_rects.push(RECT {
                left: v.x as i32,
                top: v.y as i32,
                right: (v.x + v.w) as i32,
                bottom: (v.y + v.h) as i32,
            });
        }

        unsafe {
            self._list.RSSetScissorRects(&new_rects);
        }
    }

    unsafe fn __set_push_constant_block(&mut self, data: &[u8]) {
        // This command can't work without a bound pipeline, we need the pipeline layout so we can
        // know where in the root signature to write the data
        let pipeline = self.bound_graphics_pipeline.as_deref().unwrap();

        // Lookup the parameter index on the currently bound pipeline (pipeline layout) based on
        // the constant block index
        let block = pipeline
            .binding_signature
            .compiled
            .push_constant_block
            .as_ref()
            .unwrap();

        let num32_bit_values_to_set = (data.len() / 4) as u32;
        let p_src_data = data.as_ptr();

        unsafe {
            self._list.SetGraphicsRoot32BitConstants(
                block.root_parameter_index,
                num32_bit_values_to_set,
                p_src_data as *const _,
                0,
            );
        }
    }

    unsafe fn __begin_rendering(&mut self, info: &BeginRenderingInfo) {
        let mut color_attachments =
            BVec::with_capacity_in(info.color_attachments.len(), self.arena.allocator());

        for attachment in info.color_attachments {
            unsafe {
                let view = std::mem::transmute::<_, *const ImageViewObject>(attachment.image_view);
                let descriptor = (*view).handle;
                let format = (*view).format;
                color_attachments.push(translate_rendering_color_attachment(
                    attachment,
                    descriptor,
                    Some(format),
                ));
            }
        }

        let depth_stencil = info.depth_stencil_attachment.map(|attachment| unsafe {
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

        unsafe {
            self._list
                .BeginRenderPass(Some(&color_attachments), Some(depth_stencil_ref), flags);
        }
    }

    unsafe fn __end_rendering(&mut self) {
        unsafe {
            self._list.EndRenderPass();
        }
    }

    unsafe fn __draw(
        &mut self,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    ) {
        unsafe {
            self._list
                .DrawInstanced(vertex_count, instance_count, first_vertex, first_instance)
        }
    }

    unsafe fn __draw_indexed(
        &mut self,
        index_count: u32,
        instance_count: u32,
        first_index: u32,
        first_instance: u32,
        vertex_offset: i32,
    ) {
        unsafe {
            self._list.DrawIndexedInstanced(
                index_count,
                instance_count,
                first_index,
                vertex_offset,
                first_instance,
            )
        }
    }

    unsafe fn __bind_compute_pipeline(&mut self, pipeline: &ComputePipelineHandle) {
        unsafe {
            let concrete = ComputePipeline::get_owned(pipeline);

            // Binds the pipeline
            self._list.SetPipelineState(&concrete.pipeline);

            // A pipeline is inseparable from its root signature so we need to bind it here too
            self._list
                .SetComputeRootSignature(&concrete.binding_signature.root_signature);

            // We need the currently bound pipeline while recording commands to access things like
            // the pipeline layout for handling binding descriptors.
            self.bound_compute_pipeline = Some(concrete);
            self.bound_compute_sets.iter_mut().for_each(|v| *v = None);
        }
    }

    unsafe fn __bind_parameter_blocks(
        &mut self,
        binding_signature: &dyn IBindingSignature,
        bind_point: PipelineBindPoint,
        first_block: u32,
        blocks: &[ParameterBlockHandle],
    ) {
        let binding_signature = unwrap::binding_signature(binding_signature);

        let bind_fn = match bind_point {
            PipelineBindPoint::Compute => set_compute_descriptor_table,
            PipelineBindPoint::Graphics => set_graphics_descriptor_table,
        };

        for (set_index, set) in blocks.iter().enumerate() {
            // Safety: No checks, all up to the caller to ensure this is safe
            let v = set.into_raw::<ParameterBlock>();
            let v = unsafe { v.as_ref() };

            // Computes the index of the set within the pipeline layout.
            let set_global_index = first_block as usize + set_index;

            // Fetch the base root parameter index for this set from the pipeline layout
            let param_index =
                binding_signature.compiled.block_offsets[set_global_index].root_parameter_index;

            let bound_sets = match bind_point {
                PipelineBindPoint::Compute => &mut self.bound_compute_sets,
                PipelineBindPoint::Graphics => &mut self.bound_graphics_sets,
            };
            let already_bound_set = bound_sets[set_global_index];
            if already_bound_set != Some(*set) {
                bound_sets[set_global_index] = Some(*set);

                // First bind the resource descriptors, which will always take a single descriptor table
                // slot
                let mut param_index = param_index;
                if let Some(handle) = v.resource_handle_gpu {
                    unsafe { bind_fn(self, param_index, handle.into()) };

                    // Increment the param index, as the samplers will always be placed in the following
                    // root parameter indices.
                    param_index += 1;
                }

                // Next we bind the samplers, which are separated out and bound as one table for each
                // sampler to work around limits making a single table of samplers impractical.
                let samplers = unsafe { v.samplers.as_ref() };
                for (i, sampler) in samplers.iter().cloned().enumerate() {
                    unsafe { bind_fn(self, param_index + i as u32, sampler.unwrap().into()) };
                }
            }
        }
    }

    unsafe fn __push_parameters(
        &mut self,
        binding_signature: &dyn IBindingSignature,
        bind_point: PipelineBindPoint,
        block: u32,
        base: u32,
        writes: &[ParameterWrite],
    ) {
        let binding_signature = unwrap::binding_signature(binding_signature);

        let layout_desc = binding_signature._parameter_block_layouts[block as usize]
            .desc
            .get();

        let base_param_index =
            binding_signature.compiled.block_offsets[block as usize].root_parameter_index;

        let set_table = match bind_point {
            PipelineBindPoint::Compute => set_compute_descriptor_table,
            PipelineBindPoint::Graphics => set_graphics_descriptor_table,
        };
        let set_cbv = match bind_point {
            PipelineBindPoint::Compute => set_compute_root_cbv,
            PipelineBindPoint::Graphics => set_graphics_root_cbv,
        };
        let set_srv = match bind_point {
            PipelineBindPoint::Compute => set_compute_root_srv,
            PipelineBindPoint::Graphics => set_graphics_root_srv,
        };
        let set_uav = match bind_point {
            PipelineBindPoint::Compute => set_compute_root_uav,
            PipelineBindPoint::Graphics => set_graphics_root_uav,
        };

        let visitor = ParameterBlockLayoutVisitor::new(layout_desc, base as u64, writes).unwrap();
        for v in visitor {
            let param_index = base_param_index + v.index as u32;
            for write in v.writes {
                match write {
                    ParameterWrite::Sampler(write) => unsafe {
                        let src = Sampler::get(write.sampler);
                        set_table(self, param_index, src.gpu_handle.into());
                    },
                    ParameterWrite::Buffer(write) => unsafe {
                        let buffer = Buffer::get(write.buffer);
                        let base_addr = buffer.base_address.add(write.offset).get_inner().get();
                        match v.ty {
                            ParameterType::ConstantBuffer => set_cbv(self, param_index, base_addr),
                            ParameterType::StructuredBuffer | ParameterType::ByteAddressBuffer => {
                                set_srv(self, param_index, base_addr)
                            }
                            ParameterType::RWStructuredBuffer
                            | ParameterType::RWByteAddressBuffer => {
                                set_uav(self, param_index, base_addr)
                            }
                            ParameterType::AccelerationStructure => unimplemented!(),
                            _ => unreachable!(),
                        }
                    },
                    ParameterWrite::Texture(_) => unreachable!(),
                    ParameterWrite::TextureBuffer(_) => unimplemented!(),
                }
            }
        }
    }

    unsafe fn __dispatch(&mut self, group_count_x: u32, group_count_y: u32, group_count_z: u32) {
        unsafe {
            self._list
                .Dispatch(group_count_x, group_count_y, group_count_z);
        }
    }

    unsafe fn __resource_barrier(
        &mut self,
        global_barriers: &[GlobalBarrier],
        buffer_barriers: &[BufferBarrier],
        texture_barriers: &[TextureBarrier],
    ) {
        #![allow(non_snake_case)]

        let mut translated_global_barriers =
            BVec::with_capacity_in(global_barriers.len(), self.arena.allocator());
        let mut translated_buffer_barriers =
            BVec::with_capacity_in(buffer_barriers.len(), self.arena.allocator());
        let mut translated_texture_barriers =
            BVec::with_capacity_in(texture_barriers.len(), self.arena.allocator());
        let mut barrier_groups = BVec::with_capacity_in(3, self.arena.allocator());

        if !global_barriers.is_empty() {
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
                let resource = Buffer::get(barrier.buffer.unwrap());

                translated_buffer_barriers.push(D3D12_BUFFER_BARRIER {
                    SyncBefore: barrier_sync_to_dx12(barrier.before_sync),
                    SyncAfter: barrier_sync_to_dx12(barrier.after_sync),
                    AccessBefore: barrier_access_to_dx12(barrier.before_access),
                    AccessAfter: barrier_access_to_dx12(barrier.after_access),
                    pResource: unsafe { transmute_copy(&resource.resource) },
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
                let texture = Texture::get(barrier.texture.unwrap());

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
                    texture.desc().format,
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
                    pResource: unsafe { transmute_copy(&texture.resource) },
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

        unsafe {
            self._list.Barrier(&barrier_groups);
        }
    }

    unsafe fn __copy_buffer_regions(
        &mut self,
        src: &BufferHandle,
        dst: &BufferHandle,
        regions: &[BufferCopyRegion],
    ) {
        let src = Buffer::get(src);
        let dst = Buffer::get(dst);

        for region in regions {
            unsafe {
                self._list.CopyBufferRegion(
                    dst.resource.deref(),
                    region.dst_offset,
                    src.resource.deref(),
                    region.src_offset,
                    region.size,
                );
            }
        }
    }

    unsafe fn __copy_buffer_to_texture(
        &mut self,
        src: &BufferHandle,
        dst: &TextureHandle,
        regions: &[BufferToTextureCopyRegion],
    ) {
        let src = Buffer::get(src);
        let dst = Texture::get(dst);

        let bytes_per_element = dst.desc().format.bytes_per_element();
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
            let footprint = unsafe { &mut src_location.Anonymous.PlacedFootprint };
            footprint.Offset = region.src.offset;
            footprint.Footprint.Width = region.dst.extent.width;
            footprint.Footprint.Height = region.dst.extent.height;
            footprint.Footprint.Depth = region.dst.extent.depth;
            footprint.Footprint.RowPitch = region.src.row_pitch * bytes_per_element;

            unsafe {
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
    }

    unsafe fn __copy_texture_regions(
        &mut self,
        src: &TextureHandle,
        dst: &TextureHandle,
        regions: &[TextureToTextureCopyInfo],
    ) {
        let src = Texture::get(src);
        let dst = Texture::get(dst);

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

            unsafe {
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
    }

    unsafe fn __close(&mut self) -> Result<(), CommandListCloseError> {
        unsafe {
            match self._parent.state {
                ListState::Empty => Err(CommandListCloseError::AlreadyClosed),
                ListState::Open => {
                    self._list
                        .Close()
                        .inspect_err(|v| log::error!("Platform Error: {:#?}", v))
                        .map_err(|_| CommandListCloseError::Platform)?;
                    self._parent.state = ListState::Closed;
                    Ok(())
                }
                ListState::Closed => Err(CommandListCloseError::AlreadyClosed),
            }
        }
    }

    unsafe fn __set_marker(&mut self, color: Color, message: &aleph_nstr::NStr) {
        unsafe {
            set_marker_cstr_on_list(&self._list, color.0.into(), message.to_cstr());
        }
    }

    unsafe fn __begin_event(&mut self, color: Color, message: &aleph_nstr::NStr) {
        unsafe {
            begin_event_cstr_on_list(&self._list, color.0.into(), message.to_cstr());
        }
    }

    unsafe fn __end_event(&mut self) {
        unsafe {
            end_event_on_list(&self._list);
        }
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

unsafe fn set_compute_root_cbv(encoder: &Encoder, rootparameterindex: u32, buffer_location: u64) {
    unsafe {
        encoder
            ._list
            .SetComputeRootConstantBufferView(rootparameterindex, buffer_location)
    }
}
unsafe fn set_graphics_root_cbv(encoder: &Encoder, rootparameterindex: u32, buffer_location: u64) {
    unsafe {
        encoder
            ._list
            .SetGraphicsRootConstantBufferView(rootparameterindex, buffer_location)
    }
}

unsafe fn set_compute_root_srv(encoder: &Encoder, rootparameterindex: u32, buffer_location: u64) {
    unsafe {
        encoder
            ._list
            .SetComputeRootShaderResourceView(rootparameterindex, buffer_location)
    }
}
unsafe fn set_graphics_root_srv(encoder: &Encoder, rootparameterindex: u32, buffer_location: u64) {
    unsafe {
        encoder
            ._list
            .SetGraphicsRootShaderResourceView(rootparameterindex, buffer_location)
    }
}

unsafe fn set_compute_root_uav(encoder: &Encoder, rootparameterindex: u32, buffer_location: u64) {
    unsafe {
        encoder
            ._list
            .SetComputeRootShaderResourceView(rootparameterindex, buffer_location)
    }
}
unsafe fn set_graphics_root_uav(encoder: &Encoder, rootparameterindex: u32, buffer_location: u64) {
    unsafe {
        encoder
            ._list
            .SetGraphicsRootShaderResourceView(rootparameterindex, buffer_location)
    }
}

unsafe fn set_compute_descriptor_table(
    encoder: &Encoder,
    rootparameterindex: u32,
    basedescriptor: D3D12_GPU_DESCRIPTOR_HANDLE,
) {
    unsafe {
        encoder
            ._list
            .SetComputeRootDescriptorTable(rootparameterindex, basedescriptor)
    }
}
unsafe fn set_graphics_descriptor_table(
    encoder: &Encoder,
    rootparameterindex: u32,
    basedescriptor: D3D12_GPU_DESCRIPTOR_HANDLE,
) {
    unsafe {
        encoder
            ._list
            .SetGraphicsRootDescriptorTable(rootparameterindex, basedescriptor)
    }
}
