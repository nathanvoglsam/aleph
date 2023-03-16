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

use crate::command_list::CommandList;
use crate::internal::conv::{
    barrier_access_to_dx12, barrier_sync_to_dx12, image_layout_to_dx12,
    translate_barrier_texture_aspect_to_plane_range, translate_rendering_color_attachment,
    translate_rendering_depth_stencil_attachment,
};
use crate::internal::descriptor_set::DescriptorSet;
use crate::internal::{try_clone_value_into_slot, unwrap};
use crate::pipeline::GraphicsPipeline;
use bumpalo::collections::Vec as BumpVec;
use bumpalo::Bump;
use interfaces::any::AnyArc;
use interfaces::gpu::*;
use pix::{begin_event_on_list, end_event_on_list, set_marker_on_list};
use std::any::TypeId;
use std::ptr::NonNull;
use windows::Win32::Foundation::RECT;
use windows::Win32::Graphics::Direct3D12::*;
use windows::Win32::Graphics::Dxgi::Common::*;

pub struct Encoder<'a> {
    pub(crate) list: ID3D12GraphicsCommandList7,
    pub(crate) _parent: &'a mut CommandList,
    pub(crate) bound_graphics_pipeline: Option<AnyArc<GraphicsPipeline>>,
    pub(crate) input_binding_strides: [u32; 16],
    pub(crate) arena: Bump,
}

impl<'a> Drop for Encoder<'a> {
    fn drop(&mut self) {
        // TODO: Consider an API that forces manually closing so we can avoid the unwrap here
        unsafe { self.list.Close().unwrap() }
    }
}

impl<'a> IGetPlatformInterface for Encoder<'a> {
    unsafe fn __query_platform_interface(&self, target: TypeId, out: *mut ()) -> Option<()> {
        try_clone_value_into_slot::<ID3D12GraphicsCommandList7>(&self.list, out, target)
    }
}

impl<'a> IGeneralEncoder for Encoder<'a> {
    unsafe fn bind_graphics_pipeline(&mut self, pipeline: &dyn IGraphicsPipeline) {
        let concrete = unwrap::graphics_pipeline(pipeline);

        // Binds the pipeline
        self.list.SetPipelineState(&concrete.pipeline);

        // A pipeline is inseparable from its' root signature so we need to bind it here too
        self.list
            .SetGraphicsRootSignature(&concrete.pipeline_layout.root_signature);

        // Vulkan specifies the full primitive topology in the pipeline, unlike D3D12 which
        // defers the full specification to this call below. Vulkan can't implement D3D12's
        // behavior so we have to be like vulkan here so we also set the primitive topology
        self.list
            .IASetPrimitiveTopology(concrete.primitive_topology);

        if let Some((min, max)) = concrete.depth_bounds {
            self.list.OMSetDepthBounds(min, max);
        }

        // Update the state for input binding strides. These get read when binding vertex
        // buffers to fill in the 'stride' field. Vulkan bakes these into the pipeline where
        // d3d12 takes them in 'IASetVertexBuffers'.
        self.input_binding_strides = concrete.input_binding_strides;

        // We need the currently bound pipeline while recording commands to access things like
        // the pipeline layout for handling binding descriptors.
        self.bound_graphics_pipeline = Some(concrete.this.upgrade().unwrap());
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

        self.list.IASetVertexBuffers(first_binding, &views);
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
        self.list.IASetIndexBuffer(&view);
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

        self.list.RSSetViewports(&new_viewports);
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

        self.list.RSSetScissorRects(&new_rects);
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
        self.list.SetGraphicsRoot32BitConstants(
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
            let image = unwrap::texture(attachment.image);
            let descriptor = image
                .get_or_create_rtv_for_usage(
                    None,
                    &TextureSubResourceSet {
                        aspect: TextureAspect::empty(), // TODO: D3D12 doesn't handle plane slices for RTVs so this is meaningless
                        base_mip_level: attachment.mip_level,
                        num_mip_levels: 1,
                        base_array_slice: attachment.base_array_slice,
                        num_array_slices: attachment.num_array_slices,
                    },
                )
                .unwrap();
            let format = image.dxgi_format;
            color_attachments.push(translate_rendering_color_attachment(
                attachment,
                descriptor,
                Some(format),
            ));
        }

        let depth_stencil = info.depth_stencil_attachment.map(|attachment| {
            let image = unwrap::texture(attachment.image);
            let descriptor = image
                .get_or_create_dsv_for_usage(
                    None,
                    &TextureSubResourceSet {
                        aspect: TextureAspect::empty(), // TODO: D3D12 can't create a view over only depth or stencil so this is meaningless
                        base_mip_level: attachment.mip_level,
                        num_mip_levels: 1,
                        base_array_slice: attachment.base_array_slice,
                        num_array_slices: attachment.num_array_slices,
                    },
                )
                .unwrap();
            let format = image.dxgi_format;
            translate_rendering_depth_stencil_attachment(attachment, descriptor, Some(format))
        });

        let depth_stencil_ref = depth_stencil
            .as_ref()
            .map(|v| v as *const _)
            .unwrap_or(std::ptr::null());

        self.list.BeginRenderPass(
            &color_attachments,
            depth_stencil_ref,
            D3D12_RENDER_PASS_FLAG_ALLOW_UAV_WRITES, // TODO: This *could* be suboptimal
        );
    }

    unsafe fn end_rendering(&mut self) {
        self.list.EndRenderPass();
    }

    unsafe fn draw(
        &mut self,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    ) {
        self.list
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
        self.list.DrawIndexedInstanced(
            index_count,
            instance_count,
            first_index,
            vertex_offset,
            first_instance,
        )
    }
}

impl<'a> IComputeEncoder for Encoder<'a> {
    unsafe fn bind_descriptor_sets(
        &mut self,
        _pipeline_layout: &dyn IPipelineLayout,
        bind_point: PipelineBindPoint,
        first_set: u32,
        sets: &[DescriptorSetHandle],
    ) {
        pub unsafe fn set_compute(
            encoder: &Encoder,
            rootparameterindex: u32,
            basedescriptor: D3D12_GPU_DESCRIPTOR_HANDLE,
        ) {
            encoder
                .list
                .SetComputeRootDescriptorTable(rootparameterindex, basedescriptor)
        }
        pub unsafe fn set_graphics(
            encoder: &Encoder,
            rootparameterindex: u32,
            basedescriptor: D3D12_GPU_DESCRIPTOR_HANDLE,
        ) {
            encoder
                .list
                .SetGraphicsRootDescriptorTable(rootparameterindex, basedescriptor)
        }

        // let pipeline_layout = _pipeline_layout.query_interface::<PipelineLayout>()
        //     .expect("Unknown IPipelineLayout implementation");

        let bind_fn = match bind_point {
            PipelineBindPoint::Compute => set_compute,
            PipelineBindPoint::Graphics => set_graphics,
        };

        sets.iter()
            .enumerate()
            .map(|(i, v)| {
                let v: NonNull<()> = v.clone().into();
                let v: NonNull<DescriptorSet> = v.cast();

                // Safety: No checks, all up to the caller to ensure this is safe
                (i as u32, v.as_ref())
            })
            .for_each(|(i, v)| {
                if let Some(handle) = v.resource_handle_gpu {
                    // TODO: I'm not sure if mapping directly to table index is correct
                    bind_fn(self, first_set + i, handle.into());
                }
            });
    }

    unsafe fn dispatch(&mut self, group_count_x: u32, group_count_y: u32, group_count_z: u32) {
        self.list
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
                let resource = unwrap::buffer(barrier.buffer);

                translated_buffer_barriers.push(D3D12_BUFFER_BARRIER {
                    SyncBefore: barrier_sync_to_dx12(barrier.before_sync),
                    SyncAfter: barrier_sync_to_dx12(barrier.after_sync),
                    AccessBefore: barrier_access_to_dx12(barrier.before_access),
                    AccessAfter: barrier_access_to_dx12(barrier.after_access),
                    pResource: Some(resource.resource.clone()),
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
                let texture = unwrap::texture(barrier.texture);

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

                let (first_plane, num_planes) = translate_barrier_texture_aspect_to_plane_range(
                    barrier.subresource_range.aspect,
                    texture.desc.format,
                );

                translated_texture_barriers.push(D3D12_TEXTURE_BARRIER {
                    SyncBefore: barrier_sync_to_dx12(barrier.before_sync),
                    SyncAfter: barrier_sync_to_dx12(barrier.after_sync),
                    AccessBefore: barrier_access_to_dx12(barrier.before_access),
                    AccessAfter: barrier_access_to_dx12(barrier.after_access),
                    LayoutBefore: image_layout_to_dx12(barrier.before_layout),
                    LayoutAfter: image_layout_to_dx12(barrier.after_layout),
                    pResource: Some(texture.resource.clone()),
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
                Type: D3D12_BARRIER_TYPE::TEXTURE,
                NumBarriers: translated_texture_barriers.len() as _,
                Anonymous: D3D12_BARRIER_GROUP_0 {
                    pTextureBarriers: translated_texture_barriers.as_ptr(),
                },
            });
        }

        self.list.Barrier(
            barrier_groups.len() as _,
            if barrier_groups.is_empty() {
                std::ptr::null()
            } else {
                barrier_groups.as_ptr()
            },
        );
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
            self.list.CopyBufferRegion(
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
            pResource: Some(src.resource.clone()),
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
            pResource: Some(dst.resource.clone()),
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

            self.list.CopyTextureRegion(
                &dst_location,
                region.dst.origin.x,
                region.dst.origin.y,
                region.dst.origin.z,
                &src_location,
                &src_box,
            );
        }
    }

    unsafe fn set_marker(&mut self, color: Color, message: &str) {
        set_marker_on_list(&self.list, color.0.into(), message);
    }

    unsafe fn begin_event(&mut self, color: Color, message: &str) {
        begin_event_on_list(&self.list, color.0.into(), message);
    }

    unsafe fn end_event(&mut self) {
        end_event_on_list(&self.list);
    }
}
