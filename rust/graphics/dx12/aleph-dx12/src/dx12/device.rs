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

use crate::dx12::shader_resource_view_desc::D3D12_SHADER_RESOURCE_VIEW_DESC;
use crate::raw::windows::win32::direct3d12::{
    ID3D12CommandAllocator, ID3D12CommandQueue, ID3D12DescriptorHeap, ID3D12Device4, ID3D12Fence,
    ID3D12GraphicsCommandList, ID3D12PipelineState, ID3D12RootSignature,
    D3D12_PIPELINE_STATE_STREAM_DESC, PFN_D3D12_CREATE_DEVICE,
};
use crate::raw::windows::{Abi, Interface};
use crate::render_target_view_desc::D3D12_RENDER_TARGET_VIEW_DESC;
use crate::utils::DynamicLoadCell;
use crate::{
    dxgi, CPUDescriptorHandle, CommandAllocator, CommandListType, CommandQueue, CommandQueueDesc,
    DescriptorHeap, DescriptorHeapDesc, DescriptorHeapType, FeatureLevel, Fence, FenceFlags,
    GraphicsCommandList, GraphicsPipelineState, GraphicsPipelineStateStream, RenderTargetViewDesc,
    Resource, RootSignature, RootSignatureBlob, SamplerDesc, ShaderResourceViewDesc,
};
use std::mem::{transmute, transmute_copy};
use std::sync::{Arc, RwLock};
use utf16_lit::utf16_null;

pub static CREATE_FN: DynamicLoadCell<PFN_D3D12_CREATE_DEVICE> =
    DynamicLoadCell::new(&utf16_null!("d3d12.dll"), "D3D12CreateDevice\0");

#[derive(Clone)]
#[repr(transparent)]
pub struct Device(pub(crate) ID3D12Device4);

impl Device {
    pub fn new(
        adapter: Option<&dxgi::Adapter>,
        minimum_feature_level: FeatureLevel,
    ) -> raw::windows::Result<Device> {
        unsafe {
            let create_fn = *CREATE_FN.get().expect("Failed to load d3d12.dll");
            let mut device: Option<ID3D12Device4> = None;
            create_fn(
                adapter.map(|v| transmute_copy(&v.0)),
                minimum_feature_level.into(),
                &ID3D12Device4::IID,
                device.set_abi(),
            )
            .and_some(device)
            .map(|v| Self(v))
        }
    }

    pub fn create_fence(
        &self,
        initial_value: u64,
        flags: FenceFlags,
    ) -> raw::windows::Result<Fence> {
        unsafe {
            let mut fence: Option<ID3D12Fence> = None;
            self.0
                .CreateFence(
                    initial_value,
                    flags.into(),
                    &ID3D12Fence::IID,
                    fence.set_abi(),
                )
                .and_some(fence)
                .map(|v| Fence(v))
        }
    }

    pub fn create_command_allocator(
        &self,
        list_type: CommandListType,
    ) -> raw::windows::Result<CommandAllocator> {
        unsafe {
            let mut out: Option<ID3D12CommandAllocator> = None;
            self.0
                .CreateCommandAllocator(
                    list_type.into(),
                    &ID3D12CommandAllocator::IID,
                    out.set_abi(),
                )
                .and_some(out)
                .map(|v| CommandAllocator(v))
        }
    }

    pub fn create_graphics_command_list(
        &self,
        list_type: CommandListType,
    ) -> raw::windows::Result<GraphicsCommandList> {
        unsafe {
            let mut out: Option<ID3D12GraphicsCommandList> = None;
            self.0
                .CreateCommandList1(
                    0,
                    list_type.into(),
                    Default::default(),
                    &ID3D12GraphicsCommandList::IID,
                    out.set_abi(),
                )
                .and_some(out)
                .map(|v| GraphicsCommandList(Arc::new(RwLock::new(v))))
        }
    }

    pub fn create_graphics_pipeline_state(
        &self,
        state_stream: &GraphicsPipelineStateStream,
    ) -> raw::windows::Result<GraphicsPipelineState> {
        unsafe {
            let desc = D3D12_PIPELINE_STATE_STREAM_DESC {
                size_in_bytes: state_stream.len(),
                p_pipeline_state_subobject_stream: state_stream.as_ptr() as *mut u8 as *mut _,
            };
            let mut out: Option<ID3D12PipelineState> = None;
            self.0
                .CreatePipelineState(&desc, &ID3D12PipelineState::IID, out.set_abi())
                .and_some(out)
                .map(|v| GraphicsPipelineState(v))
        }
    }

    pub fn create_root_signature(
        &self,
        root_signature_blob: &RootSignatureBlob,
    ) -> raw::windows::Result<RootSignature> {
        unsafe {
            let mut out: Option<ID3D12RootSignature> = None;
            self.0
                .CreateRootSignature(
                    0,
                    root_signature_blob.0.GetBufferPointer(),
                    root_signature_blob.0.GetBufferSize(),
                    &ID3D12RootSignature::IID,
                    out.set_abi(),
                )
                .and_some(out)
                .map(|v| RootSignature(v))
        }
    }

    pub fn create_descriptor_heap(
        &self,
        descriptor_heap_desc: &DescriptorHeapDesc,
    ) -> raw::windows::Result<DescriptorHeap> {
        unsafe {
            let desc = transmute(descriptor_heap_desc.clone());
            let mut out: Option<ID3D12DescriptorHeap> = None;
            self.0
                .CreateDescriptorHeap(&desc, &ID3D12DescriptorHeap::IID, out.set_abi())
                .and_some(out)
                .map(|v| DescriptorHeap(v))
        }
    }

    pub fn create_command_queue(
        &self,
        command_queue_desc: &CommandQueueDesc,
    ) -> raw::windows::Result<CommandQueue> {
        unsafe {
            let desc = transmute(command_queue_desc.clone());
            let mut out: Option<ID3D12CommandQueue> = None;
            self.0
                .CreateCommandQueue(&desc, &ID3D12CommandQueue::IID, out.set_abi())
                .and_some(out)
                .map(|v| CommandQueue(Arc::new(RwLock::new(v))))
        }
    }

    pub fn get_descriptor_handle_increment_size(
        &self,
        descriptor_heap_type: DescriptorHeapType,
    ) -> u32 {
        unsafe {
            self.0
                .GetDescriptorHandleIncrementSize(descriptor_heap_type.into())
        }
    }

    pub unsafe fn create_sampler(&self, sampler_desc: &SamplerDesc, dest: CPUDescriptorHandle) {
        // UNSAFE as can't bounds check or synchronize CPUDescriptorHandle
        let desc = transmute(sampler_desc.clone());
        self.0.CreateSampler(&desc, dest.into())
    }

    pub unsafe fn create_shader_resource_view(
        &self,
        resource: &Resource,
        srv_desc: &ShaderResourceViewDesc,
        dest: CPUDescriptorHandle,
    ) {
        // UNSAFE as can't bounds check or synchronize CPUDescriptorHandle
        let desc: D3D12_SHADER_RESOURCE_VIEW_DESC = srv_desc.clone().into();
        let p_desc = &desc as *const D3D12_SHADER_RESOURCE_VIEW_DESC;
        self.0
            .CreateShaderResourceView(&resource.0, p_desc as *const _, dest.into())
    }

    pub unsafe fn create_render_target_view(
        &self,
        resource: &Resource,
        rtv_desc: &RenderTargetViewDesc,
        dest: CPUDescriptorHandle,
    ) {
        // UNSAFE as can't bounds check or synchronize CPUDescriptorHandle
        let desc: D3D12_RENDER_TARGET_VIEW_DESC = rtv_desc.clone().into();
        let p_desc = &desc as *const D3D12_RENDER_TARGET_VIEW_DESC;
        self.0
            .CreateRenderTargetView(&resource.0, p_desc as *const _, dest.into())
    }
}

crate::object_impl!(Device);
