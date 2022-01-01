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

use crate::{
    dxgi, CPUDescriptorHandle, CommandAllocator, CommandListType, CommandQueue, CommandQueueDesc,
    DescriptorHeap, DescriptorHeapDesc, DescriptorHeapType, FeatureLevel, Fence, FenceFlags,
    GraphicsCommandList, GraphicsPipelineState, GraphicsPipelineStateStream, RenderTargetViewDesc,
    Resource, RootSignature, RootSignatureBlob, SamplerDesc, ShaderResourceViewDesc,
};
use std::mem::{transmute, transmute_copy};
use utf16_lit::utf16_null;
use windows::core::Interface;
use windows::utils::DynamicLoadCell;
use windows::Win32::Graphics::Direct3D12::{
    ID3D12Device4, D3D12_CPU_DESCRIPTOR_HANDLE, D3D12_PIPELINE_STATE_STREAM_DESC,
    D3D12_RENDER_TARGET_VIEW_DESC, D3D12_SHADER_RESOURCE_VIEW_DESC, PFN_D3D12_CREATE_DEVICE,
};
use windows::Win32::Graphics::Dxgi::IDXGIAdapter1;

pub static CREATE_FN: DynamicLoadCell<PFN_D3D12_CREATE_DEVICE> =
    DynamicLoadCell::new(&utf16_null!("d3d12.dll"), "D3D12CreateDevice\0");

#[repr(transparent)]
pub struct Device(pub(crate) ID3D12Device4);

impl Device {
    #[inline]
    pub fn new<'a>(
        adapter: impl Into<Option<&'a dxgi::Adapter>>,
        minimum_feature_level: FeatureLevel,
    ) -> windows::core::Result<Device> {
        unsafe {
            let create_fn = CREATE_FN.get().expect("Failed to load d3d12.dll").unwrap();
            let mut device: Option<ID3D12Device4> = None;
            let adapter: Option<IDXGIAdapter1> = adapter.into().map(|v| v.0.clone());
            let ptr = &mut device;
            let ptr = ptr as *mut Option<ID3D12Device4>;
            let ptr = ptr as *mut *mut ::std::ffi::c_void;
            create_fn(
                transmute_copy(&adapter),
                minimum_feature_level.into(),
                &ID3D12Device4::IID,
                ptr,
            )
            .and_some(device)
            .map(|v| Self(v))
        }
    }

    #[inline]
    pub fn create_fence(
        &self,
        initial_value: u64,
        flags: FenceFlags,
    ) -> windows::core::Result<Fence> {
        unsafe {
            self.0
                .CreateFence(initial_value, flags.into())
                .map(|v| Fence(v))
        }
    }

    #[inline]
    pub fn create_command_allocator(
        &self,
        list_type: CommandListType,
    ) -> windows::core::Result<CommandAllocator> {
        unsafe {
            self.0
                .CreateCommandAllocator(list_type.into())
                .map(|v| CommandAllocator(v))
        }
    }

    #[inline]
    pub fn create_graphics_command_list(
        &self,
        list_type: CommandListType,
    ) -> windows::core::Result<GraphicsCommandList> {
        unsafe {
            self.0
                .CreateCommandList1(0, list_type.into(), Default::default())
                .map(|v| GraphicsCommandList(v))
        }
    }

    #[inline]
    pub fn create_graphics_pipeline_state(
        &self,
        state_stream: &GraphicsPipelineStateStream,
    ) -> windows::core::Result<GraphicsPipelineState> {
        unsafe {
            let desc = D3D12_PIPELINE_STATE_STREAM_DESC {
                SizeInBytes: state_stream.len(),
                pPipelineStateSubobjectStream: state_stream.as_ptr() as *mut u8 as *mut _,
            };
            self.0
                .CreatePipelineState(&desc)
                .map(|v| GraphicsPipelineState(v))
        }
    }

    #[inline]
    pub fn create_root_signature(
        &self,
        root_signature_blob: &RootSignatureBlob,
    ) -> windows::core::Result<RootSignature> {
        unsafe {
            self.0
                .CreateRootSignature(
                    0,
                    root_signature_blob.0.GetBufferPointer(),
                    root_signature_blob.0.GetBufferSize(),
                )
                .map(|v| RootSignature(v))
        }
    }

    #[inline]
    pub fn create_descriptor_heap(
        &self,
        descriptor_heap_desc: &DescriptorHeapDesc,
    ) -> windows::core::Result<DescriptorHeap> {
        unsafe {
            let desc = transmute(descriptor_heap_desc.clone());
            self.0
                .CreateDescriptorHeap(&desc)
                .map(|v| DescriptorHeap(v))
        }
    }

    #[inline]
    pub fn create_command_queue(
        &self,
        command_queue_desc: &CommandQueueDesc,
    ) -> windows::core::Result<CommandQueue> {
        unsafe {
            let desc = transmute(command_queue_desc.clone());
            self.0.CreateCommandQueue(&desc).map(|v| CommandQueue(v))
        }
    }

    #[inline]
    pub fn get_descriptor_handle_increment_size(
        &self,
        descriptor_heap_type: DescriptorHeapType,
    ) -> u32 {
        unsafe {
            self.0
                .GetDescriptorHandleIncrementSize(descriptor_heap_type.into())
        }
    }

    #[inline]
    pub unsafe fn create_sampler(&self, sampler_desc: &SamplerDesc, dest: CPUDescriptorHandle) {
        // UNSAFE as can't bounds check or synchronize CPUDescriptorHandle
        let desc = transmute(sampler_desc.clone());
        let dest: D3D12_CPU_DESCRIPTOR_HANDLE = dest.into();
        self.0.CreateSampler(&desc, dest)
    }

    #[inline]
    pub unsafe fn create_shader_resource_view(
        &self,
        resource: &Resource,
        srv_desc: &ShaderResourceViewDesc,
        dest: CPUDescriptorHandle,
    ) {
        // UNSAFE as can't bounds check or synchronize CPUDescriptorHandle
        let desc: D3D12_SHADER_RESOURCE_VIEW_DESC = srv_desc.clone().into();
        let p_desc = &desc as *const D3D12_SHADER_RESOURCE_VIEW_DESC;
        let dest: D3D12_CPU_DESCRIPTOR_HANDLE = dest.into();
        self.0
            .CreateShaderResourceView(&resource.0, p_desc as *const _, dest)
    }

    #[inline]
    pub unsafe fn create_render_target_view(
        &self,
        resource: &Resource,
        rtv_desc: &RenderTargetViewDesc,
        dest: CPUDescriptorHandle,
    ) {
        // UNSAFE as can't bounds check or synchronize CPUDescriptorHandle
        let desc: D3D12_RENDER_TARGET_VIEW_DESC = rtv_desc.clone().into();
        let p_desc = &desc as *const D3D12_RENDER_TARGET_VIEW_DESC;
        let dest: D3D12_CPU_DESCRIPTOR_HANDLE = dest.into();
        self.0
            .CreateRenderTargetView(&resource.0, p_desc as *const _, dest)
    }
}

crate::object_impl!(Device);
crate::shared_object!(Device);
windows::deref_impl!(Device, ID3D12Device4);
