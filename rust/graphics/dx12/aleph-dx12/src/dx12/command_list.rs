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

use crate::dx12::resource_barrier::D3D12_RESOURCE_BARRIER;
use crate::dx12::texture_copy_location::D3D12_TEXTURE_COPY_LOCATION;
use crate::raw::windows::win32::direct3d12::ID3D12GraphicsCommandList;
use crate::utils::optional_slice_to_num_ptr_pair;
use crate::{
    dxgi, CPUDescriptorHandle, ClearFlags, CommandListType, CommandSignature, DescriptorHeap,
    DescriptorHeapType, DiscardRegion, GPUDescriptorHandle, IndexBufferView, PipelineState,
    PredicationOp, PrimitiveTopology, QueryHeap, QueryType, Rect, Resource, ResourceBarrier,
    RootSignature, StreamOutputBufferView, TextureCopyLocation, TileCopyFlags, TileRegionSize,
    TiledResourceCoordinate, VertexBufferView, Viewport,
};
use raw::windows::win32::direct3d12::{
    ID3D12DescriptorHeap, D3D12_INDEX_BUFFER_VIEW, D3D12_STREAM_OUTPUT_BUFFER_VIEW,
    D3D12_TILE_REGION_SIZE, D3D12_VERTEX_BUFFER_VIEW,
};
use std::mem::{align_of, forget, size_of, transmute};

#[repr(transparent)]
pub struct OpenGraphicsCommandList(pub(crate) ID3D12GraphicsCommandList);

#[cfg(feature = "pix")]
impl OpenGraphicsCommandList {
    pub fn scoped_event(
        &mut self,
        colour: crate::pix::Colour,
        text: &str,
    ) -> crate::pix::ScopedEvent {
        unsafe { crate::pix::ScopedEvent::for_list(self, colour, text) }
    }

    pub fn scoped_event_cstr(
        &mut self,
        colour: crate::pix::Colour,
        text: &std::ffi::CStr,
    ) -> crate::pix::ScopedEvent {
        unsafe { crate::pix::ScopedEvent::for_list_cstr(self, colour, text) }
    }
}

impl OpenGraphicsCommandList {
    /// `ID3D12GraphicsCommandList::ClearState`
    pub fn clear_state<T: Into<PipelineState> + Clone>(&self, pipeline_state: &T) {
        unsafe { self.0.ClearState(&pipeline_state.clone().into().0) }
    }

    /// `ID3D12GraphicsCommandList::DrawInstanced`
    pub fn draw_instanced(
        &self,
        vertex_count_per_instance: u32,
        instance_count: u32,
        start_vertex_location: u32,
        start_instance_location: u32,
    ) {
        unsafe {
            self.0.DrawInstanced(
                vertex_count_per_instance,
                instance_count,
                start_vertex_location,
                start_instance_location,
            )
        }
    }

    /// `ID3D12GraphicsCommandList::DrawIndexedInstanced`
    pub fn draw_indexed_instanced(
        &self,
        index_count_per_instance: u32,
        instance_count: u32,
        start_index_location: u32,
        base_vertex_location: i32,
        start_instance_location: u32,
    ) {
        unsafe {
            self.0.DrawIndexedInstanced(
                index_count_per_instance,
                instance_count,
                start_index_location,
                base_vertex_location,
                start_instance_location,
            )
        }
    }

    /// `ID3D12GraphicsCommandList::Dispatch`
    pub fn dispatch(
        &self,
        thread_group_count_x: u32,
        thread_group_count_y: u32,
        thread_group_count_z: u32,
    ) {
        unsafe {
            self.0.Dispatch(
                thread_group_count_x,
                thread_group_count_y,
                thread_group_count_z,
            )
        }
    }

    /// `ID3D12GraphicsCommandList::CopyBufferRegion`
    pub fn copy_buffer_region(
        &self,
        dst_buffer: &Resource,
        dst_offset: u64,
        src_buffer: &Resource,
        src_offset: u64,
        num_bytes: u64,
    ) {
        unsafe {
            self.0.CopyBufferRegion(
                &dst_buffer.0,
                dst_offset,
                &src_buffer.0,
                src_offset,
                num_bytes,
            )
        }
    }

    /// `ID3D12GraphicsCommandList::CopyTextureRegion`
    pub fn copy_texture_region(
        &self,
        dst: &TextureCopyLocation,
        dst_x: u32,
        dst_y: u32,
        dst_z: u32,
        src: &TextureCopyLocation,
        src_box: &crate::Box,
    ) {
        let dst: D3D12_TEXTURE_COPY_LOCATION = dst.clone().into();
        let p_dst = &dst as *const D3D12_TEXTURE_COPY_LOCATION;
        let src: D3D12_TEXTURE_COPY_LOCATION = src.clone().into();
        let p_src = &src as *const D3D12_TEXTURE_COPY_LOCATION;
        unsafe {
            self.0.CopyTextureRegion(
                p_dst as *const _,
                dst_x,
                dst_y,
                dst_z,
                p_src as *const _,
                src_box,
            )
        }
    }

    /// `ID3D12GraphicsCommandList::CopyResource`
    pub fn copy_resource(&self, dst_resource: &Resource, src_resource: &Resource) {
        unsafe { self.0.CopyResource(&dst_resource.0, &src_resource.0) }
    }

    /// `ID3D12GraphicsCommandList::CopyTiles`
    pub fn copy_tiles(
        &self,
        tiled_resource: &Resource,
        tile_region_start_coordinate: &TiledResourceCoordinate,
        tile_region_size: &TileRegionSize,
        buffer: &Resource,
        buffer_start_offset_in_bytes: u64,
        flags: TileCopyFlags,
    ) {
        assert_eq!(
            size_of::<TileRegionSize>(),
            size_of::<D3D12_TILE_REGION_SIZE>()
        );
        assert_eq!(
            align_of::<TileRegionSize>(),
            align_of::<D3D12_TILE_REGION_SIZE>()
        );

        unsafe {
            self.0.CopyTiles(
                &tiled_resource.0,
                tile_region_start_coordinate,
                tile_region_size as *const TileRegionSize as *const _,
                &buffer.0,
                buffer_start_offset_in_bytes,
                flags.into(),
            )
        }
    }

    /// `ID3D12GraphicsCommandList::ResolveSubresource`
    pub fn resolve_subresource(
        &self,
        dst_resource: &Resource,
        dst_subresource: u32,
        src_resource: &Resource,
        src_subresource: u32,
        format: dxgi::Format,
    ) {
        unsafe {
            self.0.ResolveSubresource(
                &dst_resource.0,
                dst_subresource,
                &src_resource.0,
                src_subresource,
                format.into(),
            )
        }
    }

    /// `ID3D12GraphicsCommandList::IASetPrimitiveTopology`
    pub fn ia_set_primitive_topology(&self, primitive_topology: PrimitiveTopology) {
        unsafe { self.0.IASetPrimitiveTopology(primitive_topology.into()) }
    }

    /// `ID3D12GraphicsCommandList::RSSetViewports`
    pub fn rs_set_viewports(&self, viewports: &[Viewport]) {
        let num_viewports = viewports.len() as u32;
        let p_viewports = viewports.as_ptr();
        unsafe {
            self.0
                .RSSetViewports(num_viewports, p_viewports as *const _)
        }
    }

    /// `ID3D12GraphicsCommandList::RSSetScissorRects`
    pub fn rs_set_scissor_rects(&self, rects: &[Rect]) {
        let num_rects = rects.len() as u32;
        let p_rects = rects.as_ptr();
        unsafe { self.0.RSSetScissorRects(num_rects, p_rects) }
    }

    /// `ID3D12GraphicsCommandList::OMSetBlendFactor`
    pub fn om_set_blend_factor(&self, blend_factor: Option<&[f32]>) {
        let (num_factors, blend_factor) = optional_slice_to_num_ptr_pair(blend_factor);

        assert_eq!(num_factors, 4);

        unsafe { self.0.OMSetBlendFactor(blend_factor) }
    }

    /// `ID3D12GraphicsCommandList::OMSetStencilRef`
    pub fn om_set_stencil_ref(&self, stencil_ref: u32) {
        unsafe { self.0.OMSetStencilRef(stencil_ref) }
    }

    /// `ID3D12GraphicsCommandList::SetPipelineState`
    pub fn set_pipeline_state<T: Into<PipelineState> + Clone>(&self, pipeline_state: &T) {
        unsafe { self.0.SetPipelineState(&pipeline_state.clone().into().0) }
    }

    /// `ID3D12GraphicsCommandList::ResourceBarrier`
    pub fn resource_barrier(&self, barriers: &[ResourceBarrier]) {
        // Need to heap alloc to translate the type to something FFI compatible. Can't make the
        // wrapper FFI compatible without forcing very non-idiomatic and unsafe code on the user.
        let barriers: Vec<D3D12_RESOURCE_BARRIER> =
            barriers.iter().map(|v| v.clone().into()).collect();
        let num_barriers = barriers.len() as u32;
        let p_barriers = barriers.as_ptr();
        unsafe { self.0.ResourceBarrier(num_barriers, p_barriers as *const _) }
    }

    /// `ID3D12GraphicsCommandList::ResourceBarrier`
    ///
    /// Alternate version of `resource_barrier` that provides only a single resource barrier, but
    /// skips a heap allocation
    pub fn resource_barrier_single(&self, barrier: &ResourceBarrier) {
        let barrier: D3D12_RESOURCE_BARRIER = barrier.clone().into();
        let p_barriers = &barrier as *const D3D12_RESOURCE_BARRIER;
        unsafe { self.0.ResourceBarrier(1, p_barriers as *const _) }
    }

    /// `ID3D12GraphicsCommandList::ExecuteBundle`
    pub fn execute_bundle(&self, command_list: &ClosedGraphicsCommandList) {
        unsafe { self.0.ExecuteBundle(&command_list.0) }
    }

    /// `ID3D12GraphicsCommandList::SetDescriptorHeaps`
    pub fn set_descriptor_heaps(&self, descriptor_heaps: &[DescriptorHeap]) {
        // Do some validation on the input, should be pretty low overhead
        assert!(descriptor_heaps.len() <= 2);
        if descriptor_heaps.len() == 2 {
            // Get the descriptions to do some validation
            let heap1_desc = descriptor_heaps[0].get_desc();
            let heap2_desc = descriptor_heaps[1].get_desc();

            // We can only pass one of each type of descriptor heap
            assert_ne!(heap1_desc.heap_type, heap2_desc.heap_type);

            // We can't pass these types of descriptor heaps
            assert_ne!(heap1_desc.heap_type, DescriptorHeapType::DepthStencilView);
            assert_ne!(heap1_desc.heap_type, DescriptorHeapType::RenderTargetView);

            // We can't pass these types of descriptor heaps
            assert_ne!(heap2_desc.heap_type, DescriptorHeapType::DepthStencilView);
            assert_ne!(heap2_desc.heap_type, DescriptorHeapType::RenderTargetView);
        }

        // Perform the actual API call
        unsafe {
            let num_descriptor_heaps = descriptor_heaps.len() as u32;

            // This is a load of hacky crap to let the function call actually compile
            //
            // The bindings are generated incorrectly and the function believes it takes a regular
            // ID3D12DescriptorHeap pointer. It actually takes a pointer to an array of
            // ID3D12DescriptorHeap pointers.
            //
            // I have to cast the pointer types around and do some mem::forget stuff to prevent it
            // from calling drop as that would explode very violently
            //
            // Fingers crossed this actually works
            //
            // TODO: Remove this when the bindings are generated correctly by windows-rs
            let pp_descriptor_heaps = descriptor_heaps.as_ptr();
            let pp_descriptor_heaps: ID3D12DescriptorHeap = transmute(pp_descriptor_heaps);

            self.0
                .SetDescriptorHeaps(num_descriptor_heaps, &pp_descriptor_heaps);

            forget(pp_descriptor_heaps);
        }
    }

    /// `ID3D12GraphicsCommandList::SetComputeRootSignature`
    pub fn set_compute_root_signature(&self, root_signature: &RootSignature) {
        unsafe { self.0.SetComputeRootSignature(&root_signature.0) }
    }

    /// `ID3D12GraphicsCommandList::SetGraphicsRootSignature`
    pub fn set_graphics_root_signature(&self, root_signature: &RootSignature) {
        unsafe { self.0.SetGraphicsRootSignature(&root_signature.0) }
    }

    /// `ID3D12GraphicsCommandList::SetComputeRootDescriptorTable`
    pub fn set_compute_root_descriptor_table(
        &self,
        root_parameter_index: u32,
        base_descriptor: GPUDescriptorHandle,
    ) {
        unsafe {
            self.0
                .SetComputeRootDescriptorTable(root_parameter_index, base_descriptor.into())
        }
    }

    /// `ID3D12GraphicsCommandList::SetGraphicsRootDescriptorTable`
    pub fn set_graphics_root_descriptor_table(
        &self,
        root_parameter_index: u32,
        base_descriptor: GPUDescriptorHandle,
    ) {
        unsafe {
            self.0
                .SetGraphicsRootDescriptorTable(root_parameter_index, base_descriptor.into())
        }
    }

    /// `ID3D12GraphicsCommandList::SetComputeRoot32BitConstant`
    pub fn set_compute_root32bit_constant(
        &self,
        root_parameter_index: u32,
        value: u32,
        dest_offset_in_32bit_values: u32,
    ) {
        unsafe {
            self.0.SetComputeRoot32BitConstant(
                root_parameter_index,
                value,
                dest_offset_in_32bit_values,
            )
        }
    }

    /// `ID3D12GraphicsCommandList::SetGraphicsRoot32BitConstant`
    pub fn set_graphics_root32bit_constant(
        &self,
        root_parameter_index: u32,
        value: u32,
        dest_offset_in_32bit_values: u32,
    ) {
        unsafe {
            self.0.SetGraphicsRoot32BitConstant(
                root_parameter_index,
                value,
                dest_offset_in_32bit_values,
            )
        }
    }

    /// `ID3D12GraphicsCommandList::SetComputeRoot32BitConstants`
    pub fn set_compute_root_32bit_constants(
        &self,
        root_parameter_index: u32,
        values: &[u32],
        dest_offset_in_32bit_values: u32,
    ) {
        unsafe {
            let num32_bit_values_to_set = values.len() as u32;
            let p_src_data = values.as_ptr();
            self.0.SetComputeRoot32BitConstants(
                root_parameter_index,
                num32_bit_values_to_set,
                p_src_data as *const _,
                dest_offset_in_32bit_values,
            )
        }
    }

    /// `ID3D12GraphicsCommandList::SetGraphicsRoot32BitConstants`
    pub fn set_graphics_root_32bit_constants(
        &self,
        root_parameter_index: u32,
        values: &[u32],
        dest_offset_in_32bit_values: u32,
    ) {
        unsafe {
            let num32_bit_values_to_set = values.len() as u32;
            let p_src_data = values.as_ptr();
            self.0.SetGraphicsRoot32BitConstants(
                root_parameter_index,
                num32_bit_values_to_set,
                p_src_data as *const _,
                dest_offset_in_32bit_values,
            )
        }
    }

    /// `ID3D12GraphicsCommandList::SetComputeRootConstantBufferView`
    pub fn set_compute_root_constant_buffer_view(
        &self,
        root_parameter_index: u32,
        buffer_location: GPUDescriptorHandle,
    ) {
        unsafe {
            self.0.SetComputeRootConstantBufferView(
                root_parameter_index,
                buffer_location.get_inner().get(),
            )
        }
    }

    /// `ID3D12GraphicsCommandList::SetGraphicsRootConstantBufferView`
    pub fn set_graphics_root_constant_buffer_view(
        &self,
        root_parameter_index: u32,
        buffer_location: GPUDescriptorHandle,
    ) {
        unsafe {
            self.0.SetGraphicsRootConstantBufferView(
                root_parameter_index,
                buffer_location.get_inner().get(),
            )
        }
    }

    /// `ID3D12GraphicsCommandList::SetComputeRootShaderResourceView`
    pub fn set_compute_root_shader_resource_view(
        &self,
        root_parameter_index: u32,
        buffer_location: GPUDescriptorHandle,
    ) {
        unsafe {
            self.0.SetComputeRootShaderResourceView(
                root_parameter_index,
                buffer_location.get_inner().get(),
            )
        }
    }

    /// `ID3D12GraphicsCommandList::SetGraphicsRootShaderResourceView`
    pub fn set_graphics_root_shader_resource_view(
        &self,
        root_parameter_index: u32,
        buffer_location: GPUDescriptorHandle,
    ) {
        unsafe {
            self.0.SetGraphicsRootShaderResourceView(
                root_parameter_index,
                buffer_location.get_inner().get(),
            )
        }
    }

    /// `ID3D12GraphicsCommandList::SetComputeRootUnorderedAccessView`
    pub fn set_compute_root_unordered_access_view(
        &self,
        root_parameter_index: u32,
        buffer_location: GPUDescriptorHandle,
    ) {
        unsafe {
            self.0.SetComputeRootUnorderedAccessView(
                root_parameter_index,
                buffer_location.get_inner().get(),
            )
        }
    }

    /// `ID3D12GraphicsCommandList::SetGraphicsRootUnorderedAccessView`
    pub fn set_graphics_root_unordered_access_view(
        &self,
        root_parameter_index: u32,
        buffer_location: GPUDescriptorHandle,
    ) {
        unsafe {
            self.0.SetGraphicsRootUnorderedAccessView(
                root_parameter_index,
                buffer_location.get_inner().get(),
            )
        }
    }

    /// `ID3D12GraphicsCommandList::IASetIndexBuffer`
    pub fn ia_set_index_buffer(&self, view: &IndexBufferView) {
        assert_eq!(
            size_of::<IndexBufferView>(),
            size_of::<D3D12_INDEX_BUFFER_VIEW>()
        );
        assert_eq!(
            align_of::<IndexBufferView>(),
            align_of::<D3D12_INDEX_BUFFER_VIEW>()
        );
        unsafe {
            self.0
                .IASetIndexBuffer(view as *const IndexBufferView as *const _)
        }
    }

    /// `ID3D12GraphicsCommandList::IASetVertexBuffers`
    pub fn ia_set_vertex_buffers(&self, start_slot: u32, views: &[VertexBufferView]) {
        assert_eq!(
            size_of::<VertexBufferView>(),
            size_of::<D3D12_VERTEX_BUFFER_VIEW>()
        );
        assert_eq!(
            align_of::<VertexBufferView>(),
            align_of::<D3D12_VERTEX_BUFFER_VIEW>()
        );
        let num_views = views.len() as u32;
        let p_views = views.as_ptr();
        unsafe {
            self.0
                .IASetVertexBuffers(start_slot, num_views, p_views as *const _)
        }
    }

    /// `ID3D12GraphicsCommandList::SOSetTargets`
    pub fn so_set_targets(&self, start_slot: u32, views: &[StreamOutputBufferView]) {
        assert_eq!(
            size_of::<StreamOutputBufferView>(),
            size_of::<D3D12_STREAM_OUTPUT_BUFFER_VIEW>()
        );
        assert_eq!(
            align_of::<StreamOutputBufferView>(),
            align_of::<D3D12_STREAM_OUTPUT_BUFFER_VIEW>()
        );

        // Check that `buffer_filled_size_location` is provided if `size_in_bytes` is non zero
        views.iter().for_each(|v| {
            if v.size_in_bytes > 0 {
                assert!(v.buffer_filled_size_location.is_some())
            }
        });

        let num_views = views.len() as u32;
        let p_views = views.as_ptr();
        unsafe {
            self.0
                .SOSetTargets(start_slot, num_views, p_views as *const _)
        }
    }

    /// `ID3D12GraphicsCommandList::OMSetRenderTargets`
    ///
    /// Wrapper for `ID3D12GraphicsCommandList::OMSetRenderTargets` with
    /// `rts_single_handle_to_descriptor_range` set to false. This special behavior is too difficult
    /// to expose in a sane way to rust as only a single function
    ///
    /// See official Direct3D12 docs to explain this function's behavior
    pub fn om_set_render_targets(
        &self,
        render_target_descriptors: Option<&[CPUDescriptorHandle]>,
        depth_stencil_descriptor: Option<CPUDescriptorHandle>,
    ) {
        // Unpack the optional render target list
        let (num_rt, p_rt_desc) = optional_slice_to_num_ptr_pair(render_target_descriptors);

        // Unpack the optional depth stencil view
        let p_ds_desc = if let Some(ds_desc) = &depth_stencil_descriptor {
            ds_desc as *const CPUDescriptorHandle
        } else {
            std::ptr::null()
        };

        unsafe {
            self.0.OMSetRenderTargets(
                num_rt,
                p_rt_desc as *const _,
                false.into(),
                p_ds_desc as *const _,
            )
        }
    }

    /// `ID3D12GraphicsCommandList::OMSetRenderTargets`
    ///
    /// Wrapper for `ID3D12GraphicsCommandList::OMSetRenderTargets` with
    /// `rts_single_handle_to_descriptor_range` set to true. This special behavior is too difficult
    /// to expose in a sane way to rust as only a single function.
    ///
    /// See official Direct3D12 docs to explain this function's behavior
    pub fn om_set_render_target_range(
        &self,
        num_render_target_descriptors: u32,
        render_target_descriptor_base: Option<CPUDescriptorHandle>,
        depth_stencil_descriptor: Option<CPUDescriptorHandle>,
    ) {
        // Check that the correct value of `num_render_target_descriptors` is correct
        if render_target_descriptor_base.is_some() {
            assert_ne!(num_render_target_descriptors, 0);
        } else {
            assert_eq!(num_render_target_descriptors, 0);
        }

        // Unpack the optional render target view
        let p_rt_desc = if let Some(rt_desc) = &render_target_descriptor_base {
            rt_desc as *const CPUDescriptorHandle
        } else {
            std::ptr::null()
        };

        // Unpack the optional depth stencil view
        let p_ds_desc = if let Some(ds_desc) = &depth_stencil_descriptor {
            ds_desc as *const CPUDescriptorHandle
        } else {
            std::ptr::null()
        };

        unsafe {
            self.0.OMSetRenderTargets(
                num_render_target_descriptors,
                p_rt_desc as *const _,
                true.into(),
                p_ds_desc as *const _,
            )
        }
    }

    /// `ID3D12GraphicsCommandList::ClearDepthStencilView`
    pub fn clear_depth_stencil_view(
        &self,
        depth_stencil_view: CPUDescriptorHandle,
        clear_flags: ClearFlags,
        depth: f32,
        stencil: u8,
        rects: Option<&[Rect]>,
    ) {
        let (num_rects, p_rects) = optional_slice_to_num_ptr_pair(rects);

        unsafe {
            self.0.ClearDepthStencilView(
                depth_stencil_view.into(),
                clear_flags.into(),
                depth,
                stencil,
                num_rects,
                p_rects,
            )
        }
    }

    /// `ID3D12GraphicsCommandList::ClearRenderTargetView`
    pub fn clear_render_target_view(
        &self,
        render_target_view: CPUDescriptorHandle,
        color_rgba: &[f32],
        rects: Option<&[Rect]>,
    ) {
        assert_eq!(color_rgba.len(), 4);

        let (num_rects, p_rects) = optional_slice_to_num_ptr_pair(rects);

        unsafe {
            self.0.ClearRenderTargetView(
                render_target_view.into(),
                color_rgba.as_ptr(),
                num_rects,
                p_rects,
            )
        }
    }

    /// `ID3D12GraphicsCommandList::ClearUnorderedAccessViewUint`
    pub fn clear_unordered_acces_view_uint(
        &self,
        view_gpu_handle_in_current_heap: GPUDescriptorHandle,
        view_cpu_handle: CPUDescriptorHandle,
        resource: &Resource,
        values: &[u32],
        rects: Option<&[Rect]>,
    ) {
        assert_eq!(values.len(), 4);

        let (num_rects, p_rects) = optional_slice_to_num_ptr_pair(rects);

        unsafe {
            self.0.ClearUnorderedAccessViewUint(
                view_gpu_handle_in_current_heap.into(),
                view_cpu_handle.into(),
                &resource.0,
                values.as_ptr(),
                num_rects,
                p_rects,
            )
        }
    }

    /// `ID3D12GraphicsCommandList::ClearUnorderedAccessViewFloat`
    pub fn clear_unordered_access_view_float(
        &self,
        view_gpu_handle_in_current_heap: GPUDescriptorHandle,
        view_cpu_handle: CPUDescriptorHandle,
        resource: &Resource,
        values: &[f32],
        rects: Option<&[Rect]>,
    ) {
        assert_eq!(values.len(), 4);

        let (num_rects, p_rects) = optional_slice_to_num_ptr_pair(rects);

        unsafe {
            self.0.ClearUnorderedAccessViewFloat(
                view_gpu_handle_in_current_heap.into(),
                view_cpu_handle.into(),
                &resource.0,
                values.as_ptr(),
                num_rects,
                p_rects,
            )
        }
    }

    /// `ID3D12GraphicsCommandList::DiscardResource`
    pub fn discard_resource(&self, resource: &Resource, region: Option<&DiscardRegion>) {
        if let Some(region) = region {
            let region = region.clone().into();
            unsafe { self.0.DiscardResource(&resource.0, &region) }
        } else {
            unsafe { self.0.DiscardResource(&resource.0, std::ptr::null()) }
        }
    }

    /// `ID3D12GraphicsCommandList::BeginQuery`
    pub fn begin_query(&self, query_heap: &QueryHeap, query_type: QueryType, index: u32) {
        unsafe { self.0.BeginQuery(&query_heap.0, query_type.into(), index) }
    }

    /// `ID3D12GraphicsCommandList::EndQuery`
    pub fn end_query(&self, query_heap: &QueryHeap, query_type: QueryType, index: u32) {
        unsafe { self.0.EndQuery(&query_heap.0, query_type.into(), index) }
    }

    /// `ID3D12GraphicsCommandList::ResolveQueryData`
    pub fn resolve_query_data(
        &self,
        query_heap: &QueryHeap,
        query_type: QueryType,
        start_index: u32,
        num_queries: u32,
        destination_buffer: &Resource,
        aligned_destination_buffer_offset: u64,
    ) {
        assert_eq!(
            aligned_destination_buffer_offset % 8,
            0,
            "Must be a multiple of 8"
        );
        unsafe {
            self.0.ResolveQueryData(
                &query_heap.0,
                query_type.into(),
                start_index,
                num_queries,
                &destination_buffer.0,
                aligned_destination_buffer_offset,
            )
        }
    }

    /// `ID3D12GraphicsCommandList::SetPredication`
    pub fn set_predication(
        &self,
        buffer: &Resource,
        aligned_buffer_offset: u64,
        operation: PredicationOp,
    ) {
        unsafe {
            self.0
                .SetPredication(&buffer.0, aligned_buffer_offset, operation.into())
        }
    }

    /// `ID3D12GraphicsCommandList::ExecuteIndirect`
    pub fn execute_indirect(
        &self,
        command_signature: Option<&CommandSignature>,
        max_command_count: u32,
        argument_buffer: &Resource,
        argument_buffer_offset: u64,
        count_buffer: &Resource,
        count_buffer_offset: u64,
    ) {
        unsafe {
            if let Some(command_signature) = command_signature {
                self.0.ExecuteIndirect(
                    &command_signature.0,
                    max_command_count,
                    &argument_buffer.0,
                    argument_buffer_offset,
                    &count_buffer.0,
                    count_buffer_offset,
                )
            } else {
                self.0.ExecuteIndirect(
                    None,
                    max_command_count,
                    &argument_buffer.0,
                    argument_buffer_offset,
                    &count_buffer.0,
                    count_buffer_offset,
                )
            }
        }
    }

    /// `ID3D12GraphicsCommandList::GetType`
    pub fn get_type(&self) -> CommandListType {
        unsafe { CommandListType::from_raw(self.0.GetType()).unwrap() }
    }
}

crate::object_impl!(OpenGraphicsCommandList);
crate::device_child_impl!(OpenGraphicsCommandList);

#[repr(transparent)]
pub struct ClosedGraphicsCommandList(pub(crate) ID3D12GraphicsCommandList);

impl ClosedGraphicsCommandList {
    /// `ID3D12GraphicsCommandList::GetType`
    pub fn get_type(&self) -> CommandListType {
        unsafe { CommandListType::from_raw(self.0.GetType()).unwrap() }
    }
}

crate::object_impl!(ClosedGraphicsCommandList);
crate::device_child_impl!(ClosedGraphicsCommandList);
