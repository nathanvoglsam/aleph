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

use crate::compute_pipeline_state_desc::ComputePipelineStateDesc;
use crate::depth_stencil_view_desc::DepthStencilViewDesc;
use crate::message_id::MessageId;
use crate::{
    dxgi, CPUDescriptorHandle, ClearValue, CommandAllocator, CommandListType, CommandQueue,
    CommandQueueDesc, ComputePipelineState, DescriptorHeap, DescriptorHeapDesc, DescriptorHeapType,
    FeatureLevel, Fence, FenceFlags, GraphicsCommandList, GraphicsPipelineState,
    GraphicsPipelineStateStream, HeapFlags, HeapProperties, MessageCategory, MessageSeverity,
    RenderTargetViewDesc, Resource, ResourceDesc, ResourceStates, RootSignature, RootSignatureBlob,
    SamplerDesc, ShaderResourceViewDesc,
};
use std::ffi::CStr;
use std::mem::{transmute, transmute_copy};
use utf16_lit::utf16_null;
use windows::core::Interface;
use windows::utils::DynamicLoadCell;
use windows::Win32::Graphics::Direct3D12::{
    ID3D12Device10, ID3D12InfoQueue1, ID3D12Resource, D3D12_CACHED_PIPELINE_STATE,
    D3D12_CLEAR_VALUE, D3D12_COMPUTE_PIPELINE_STATE_DESC, D3D12_CPU_DESCRIPTOR_HANDLE,
    D3D12_DEPTH_STENCIL_VIEW_DESC, D3D12_HEAP_PROPERTIES, D3D12_MESSAGE_CALLBACK_FLAG_NONE,
    D3D12_MESSAGE_CATEGORY, D3D12_MESSAGE_ID, D3D12_MESSAGE_SEVERITY,
    D3D12_PIPELINE_STATE_STREAM_DESC, D3D12_RENDER_TARGET_VIEW_DESC, D3D12_RESOURCE_DESC,
    D3D12_SHADER_BYTECODE, D3D12_SHADER_RESOURCE_VIEW_DESC, PFN_D3D12_CREATE_DEVICE,
};
use windows::Win32::Graphics::Dxgi::IDXGIAdapter1;

pub static CREATE_FN: DynamicLoadCell<PFN_D3D12_CREATE_DEVICE> =
    DynamicLoadCell::new(&utf16_null!("d3d12.dll"), "D3D12CreateDevice\0");

#[repr(transparent)]
pub struct Device(pub(crate) ID3D12Device10);

impl Device {
    #[inline]
    pub fn new<'a>(
        adapter: impl Into<Option<&'a dxgi::Adapter>>,
        minimum_feature_level: FeatureLevel,
    ) -> windows::core::Result<Device> {
        unsafe {
            let create_fn = CREATE_FN.get().expect("Failed to load d3d12.dll").unwrap();
            let mut device: Option<ID3D12Device10> = None;
            let adapter: Option<IDXGIAdapter1> = adapter.into().map(|v| v.0.clone());
            let ptr = &mut device;
            let ptr = ptr as *mut Option<ID3D12Device10>;
            let ptr = ptr as *mut *mut ::std::ffi::c_void;
            create_fn(
                transmute_copy(&adapter),
                minimum_feature_level.into(),
                &ID3D12Device10::IID,
                ptr,
            )
            .and_some(device)
            .map(Self)
        }
    }

    #[inline]
    pub fn create_fence(
        &self,
        initial_value: u64,
        flags: FenceFlags,
    ) -> windows::core::Result<Fence> {
        unsafe { self.0.CreateFence(initial_value, flags.into()).map(Fence) }
    }

    #[inline]
    pub fn create_command_allocator(
        &self,
        list_type: CommandListType,
    ) -> windows::core::Result<CommandAllocator> {
        unsafe {
            self.0
                .CreateCommandAllocator(list_type.into())
                .map(CommandAllocator)
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
                .map(GraphicsCommandList)
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
            self.0.CreatePipelineState(&desc).map(GraphicsPipelineState)
        }
    }

    #[inline]
    pub fn create_compute_pipeline_state(
        &self,
        desc: &ComputePipelineStateDesc,
    ) -> windows::core::Result<ComputePipelineState> {
        unsafe {
            let cached_pso = match desc.cached_pso {
                None => D3D12_CACHED_PIPELINE_STATE::default(),
                Some(v) => D3D12_CACHED_PIPELINE_STATE {
                    pCachedBlob: v.as_ptr() as *const _,
                    CachedBlobSizeInBytes: v.len(),
                },
            };
            let desc = D3D12_COMPUTE_PIPELINE_STATE_DESC {
                pRootSignature: Some(desc.root_signature.0.clone()),
                CS: D3D12_SHADER_BYTECODE {
                    pShaderBytecode: desc.shader.as_ptr() as *const _,
                    BytecodeLength: desc.shader.len(),
                },
                NodeMask: desc.node_mask,
                CachedPSO: cached_pso,
                Flags: Default::default(),
            };
            self.0
                .CreateComputePipelineState(&desc)
                .map(ComputePipelineState)
        }
    }

    #[inline]
    pub fn create_root_signature(
        &self,
        root_signature_blob: &RootSignatureBlob,
    ) -> windows::core::Result<RootSignature> {
        unsafe {
            let blob = core::slice::from_raw_parts(
                root_signature_blob.0.GetBufferPointer() as *const u8,
                root_signature_blob.0.GetBufferSize(),
            );
            self.0.CreateRootSignature(0, blob).map(RootSignature)
        }
    }

    #[inline]
    pub fn create_descriptor_heap(
        &self,
        descriptor_heap_desc: &DescriptorHeapDesc,
    ) -> windows::core::Result<DescriptorHeap> {
        unsafe {
            let desc = transmute(descriptor_heap_desc.clone());
            self.0.CreateDescriptorHeap(&desc).map(DescriptorHeap)
        }
    }

    #[inline]
    pub fn create_command_queue(
        &self,
        command_queue_desc: &CommandQueueDesc,
    ) -> windows::core::Result<CommandQueue> {
        unsafe {
            let desc = transmute(command_queue_desc.clone());
            self.0.CreateCommandQueue(&desc).map(CommandQueue)
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

    #[inline]
    pub unsafe fn create_depth_stencil_view(
        &self,
        resource: &Resource,
        dsv_desc: &DepthStencilViewDesc,
        dest: CPUDescriptorHandle,
    ) {
        // UNSAFE as can't bounds check or synchronize CPUDescriptorHandle
        let desc: D3D12_DEPTH_STENCIL_VIEW_DESC = dsv_desc.clone().into();
        let p_desc = &desc as *const D3D12_DEPTH_STENCIL_VIEW_DESC;
        let dest: D3D12_CPU_DESCRIPTOR_HANDLE = dest.into();
        self.0
            .CreateDepthStencilView(&resource.0, p_desc as *const _, dest)
    }

    #[inline]
    pub unsafe fn create_committed_resource(
        &self,
        heap_properties: &HeapProperties,
        heap_flags: HeapFlags,
        resource_desc: &ResourceDesc,
        initial_state: ResourceStates,
        optimized_clear_value: Option<ClearValue>,
    ) -> windows::core::Result<Resource> {
        let optimized_clear_value = optimized_clear_value.map(D3D12_CLEAR_VALUE::from);
        let optimized_clear_value_ref = match optimized_clear_value.as_ref() {
            None => core::ptr::null(),
            Some(v) => v as *const D3D12_CLEAR_VALUE,
        };

        let mut out = None;
        let result: windows::core::Result<()> = self.0.CreateCommittedResource::<ID3D12Resource>(
            heap_properties as *const _ as *const D3D12_HEAP_PROPERTIES,
            heap_flags.into(),
            resource_desc as *const _ as *const D3D12_RESOURCE_DESC,
            initial_state.into(),
            optimized_clear_value_ref,
            &mut out,
        );
        result.map(|_| Resource(out.unwrap_unchecked()))
    }

    pub unsafe fn register_message_callback<
        T: Fn(MessageCategory, MessageSeverity, MessageId, &CStr) + 'static,
    >(
        &self,
        callback: T,
    ) -> windows::core::Result<u32> {
        /// Internal callback that wraps the closure in an FFI compatible function
        unsafe extern "system" fn raw_callback<
            X: Fn(MessageCategory, MessageSeverity, MessageId, &CStr) + 'static,
        >(
            category: D3D12_MESSAGE_CATEGORY,
            severity: D3D12_MESSAGE_SEVERITY,
            id: D3D12_MESSAGE_ID,
            pdescription: windows::core::PCSTR,
            pcontext: *mut core::ffi::c_void,
        ) {
            // Translate the enums
            let category = MessageCategory::from(category);
            let severity = MessageSeverity::from(severity);
            let id = MessageId::from(id);

            // Cast to the concrete type and get a reference
            let context = pcontext as *const X;
            let context = context.as_ref().unwrap();

            let description = CStr::from_ptr(pdescription.0 as *const _);

            // Call the actual callback
            (context)(category, severity, id, description);
        }

        let casted: ID3D12InfoQueue1 = self.0.cast::<ID3D12InfoQueue1>()?;
        let mut cookie = 0;

        let boxed = Box::new(callback);
        let leak = Box::leak(boxed);

        casted.RegisterMessageCallback(
            Some(raw_callback::<T>),
            D3D12_MESSAGE_CALLBACK_FLAG_NONE,
            leak as *const _ as *const core::ffi::c_void,
            &mut cookie,
        )?;

        Ok(cookie)
    }

    pub unsafe fn unregister_message_callback(&self, cookie: u32) -> windows::core::Result<()> {
        let casted: ID3D12InfoQueue1 = self.0.cast::<ID3D12InfoQueue1>()?;
        casted.UnregisterMessageCallback(cookie)?;
        Ok(())
    }
}

crate::object_impl!(Device);
crate::shared_object!(Device);
windows::deref_impl!(Device, ID3D12Device10);
