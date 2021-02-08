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

extern crate aleph_dx12_alloc_raw as alloc_raw;
extern crate aleph_dx12_raw as raw;

mod allocator;
mod command_allocator;
mod command_list;
mod command_queue;
mod cpu_descriptor_handle;
mod debug;
mod descriptor_heap;
mod device;
mod dxgi_adapter;
mod dxgi_factory;
mod event;
mod feature_level;
mod fence;
mod gpu_descriptor_handle;
mod interfaces;
mod mesh_shader_pipeline_desc;
mod pipeline_state_stream;
mod root_parameter;
mod root_signature;
mod root_signature_desc;
mod swapchain;
mod versioned_root_signature_desc;

#[cfg(test)]
mod tests;

pub use allocator::Allocator;
pub use allocator::AllocatorBuilder;
pub use command_allocator::CommandAllocator;
pub use command_list::CommandList;
pub use command_list::CommandListType;
pub use command_list::GraphicsCommandList;
pub use command_queue::CommandQueue;
pub use command_queue::CommandQueueBuilder;
pub use cpu_descriptor_handle::CPUDescriptorHandle;
pub use debug::Debug;
pub use descriptor_heap::DescriptorHeap;
pub use descriptor_heap::DescriptorHeapBuilder;
pub use descriptor_heap::DescriptorHeapType;
pub use device::Device;
pub use dxgi_adapter::DXGIAdapter;
pub use dxgi_factory::DXGIFactory;
pub use event::Event;
pub use event::EventBuilder;
pub use feature_level::FeatureLevel;
pub use fence::Fence;
pub use fence::FenceBuilder;
pub use gpu_descriptor_handle::GPUDescriptorHandle;
pub use interfaces::D3D12DeviceChild;
pub use interfaces::D3D12Object;
pub use mesh_shader_pipeline_desc::MeshShaderPipelineStateDesc;
pub use pipeline_state_stream::ToPipelineStateStream;
pub use raw::windows::initialize_mta;
pub use raw::windows::initialize_sta;
pub use raw::windows::ErrorCode;
pub use raw::windows::Result;
pub use root_parameter::RootParameter;
pub use root_parameter::RootParameter1;
pub use root_parameter::RootParameter1Type;
pub use root_parameter::RootParameterType;
pub use root_signature::RootSignature;
pub use root_signature::RootSignatureBlob;
pub use root_signature_desc::RootSignatureDesc;
pub use root_signature_desc::RootSignatureDesc1;
pub use swapchain::SwapChain;
pub use swapchain::SwapChainBuilder;
pub use versioned_root_signature_desc::VersionedRootSignatureDesc;
