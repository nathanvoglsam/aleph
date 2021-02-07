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

pub extern crate aleph_dx12_raw as raw;

mod command_list_type;
mod command_queue;
mod descriptor_heap;
mod descriptor_heap_type;
mod device;
mod event;
mod fence;
mod mesh_shader_pipeline_desc;
mod pipeline_state_stream;
mod swapchain;

pub use command_list_type::CommandListType;
pub use command_queue::CommandQueue;
pub use command_queue::CommandQueueBuilder;
pub use descriptor_heap::DescriptorHeap;
pub use descriptor_heap::DescriptorHeapBuilder;
pub use descriptor_heap_type::DescriptorHeapType;
pub use device::Device;
pub use device::DeviceBuilder;
pub use device::DeviceCreateError;
pub use device::DeviceCreateResult;
pub use event::Event;
pub use event::EventBuilder;
pub use fence::Fence;
pub use fence::FenceBuilder;
pub use mesh_shader_pipeline_desc::MeshShaderPipelineStateDesc;
pub use pipeline_state_stream::ToPipelineStateStream;
pub use swapchain::SwapChain;
pub use swapchain::SwapChainBuilder;
pub use swapchain::SwapChainCreateError;
pub use swapchain::SwapChainCreateResult;
