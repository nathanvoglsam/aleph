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
use crate::descriptor_set_layout::DescriptorSetLayout;
use crate::device::Device;
use crate::fence::Fence;
use crate::pipeline::GraphicsPipeline;
use crate::pipeline_layout::PipelineLayout;
use crate::sampler::Sampler;
use crate::semaphore::Semaphore;
use crate::shader::Shader;
use crate::surface::Surface;
use crate::swap_chain::SwapChain;
use crate::texture::Texture;
use aleph_rhi_impl_utils::conversion_function;
use interfaces::gpu::*;

conversion_function!(IBuffer, Buffer, buffer, buffer_d, buffer_iter);
conversion_function!(
    ICommandList,
    CommandList,
    command_list,
    command_list_d,
    command_list_iter
);
conversion_function!(
    IDescriptorSetLayout,
    DescriptorSetLayout,
    descriptor_set_layout,
    descriptor_set_layout_d,
    descriptor_set_layout_iter
);
conversion_function!(IDevice, Device, device, device_d, device_iter);
conversion_function!(IFence, Fence, fence, fence_d, fence_iter);
conversion_function!(
    IGraphicsPipeline,
    GraphicsPipeline,
    graphics_pipeline,
    graphics_pipeline_d,
    graphics_pipeline_iter
);
conversion_function!(
    IPipelineLayout,
    PipelineLayout,
    pipeline_layout,
    pipeline_layout_d,
    pipeline_layout_iter
);
conversion_function!(ISampler, Sampler, sampler, sampler_d, sampler_iter);
conversion_function!(
    ISemaphore,
    Semaphore,
    semaphore,
    semaphore_d,
    semaphore_iter
);
conversion_function!(IShader, Shader, shader, shader_d, shader_iter);
conversion_function!(ISurface, Surface, surface, surface_d, surface_iter);

conversion_function!(
    ISwapChain,
    SwapChain,
    swap_chain,
    swap_chain_d,
    swap_chain_iter
);
conversion_function!(ITexture, Texture, texture, texture_d, texture_iter);
