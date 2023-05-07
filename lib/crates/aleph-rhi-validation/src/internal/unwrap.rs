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
    ValidationBuffer, ValidationCommandList, ValidationComputePipeline,
    ValidationDescriptorSetLayout, ValidationDevice, ValidationFence, ValidationGraphicsPipeline,
    ValidationPipelineLayout, ValidationSampler, ValidationSemaphore, ValidationShader,
    ValidationSurface, ValidationSwapChain, ValidationTexture,
};
use aleph_rhi_impl_utils::conversion_function;
use interfaces::gpu::*;

conversion_function!(IBuffer, ValidationBuffer, buffer, buffer_d, buffer_iter);
conversion_function!(
    ICommandList,
    ValidationCommandList,
    command_list,
    command_list_d,
    command_list_iter
);
conversion_function!(
    IDescriptorSetLayout,
    ValidationDescriptorSetLayout,
    descriptor_set_layout,
    descriptor_set_layout_d,
    descriptor_set_layout_iter
);
conversion_function!(IDevice, ValidationDevice, device, device_d, device_iter);
conversion_function!(IFence, ValidationFence, fence, fence_d, fence_iter);
conversion_function!(
    IGraphicsPipeline,
    ValidationGraphicsPipeline,
    graphics_pipeline,
    graphics_pipeline_d,
    graphics_pipeline_iter
);
conversion_function!(
    IComputePipeline,
    ValidationComputePipeline,
    compute_pipeline,
    compute_pipeline_d,
    compute_pipeline_iter
);
conversion_function!(
    IPipelineLayout,
    ValidationPipelineLayout,
    pipeline_layout,
    pipeline_layout_d,
    pipeline_layout_iter
);
conversion_function!(
    ISampler,
    ValidationSampler,
    sampler,
    sampler_d,
    sampler_iter
);
conversion_function!(
    ISemaphore,
    ValidationSemaphore,
    semaphore,
    semaphore_d,
    semaphore_iter
);
conversion_function!(IShader, ValidationShader, shader, shader_d, shader_iter);
conversion_function!(
    ISurface,
    ValidationSurface,
    surface,
    surface_d,
    surface_iter
);
conversion_function!(
    ISwapChain,
    ValidationSwapChain,
    swap_chain,
    swap_chain_d,
    swap_chain_iter
);
conversion_function!(
    ITexture,
    ValidationTexture,
    texture,
    texture_d,
    texture_iter
);
