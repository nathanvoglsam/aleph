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
use aleph_gpu_utils::conversion_function;
use interfaces::gpu::*;

conversion_function!(IBuffer, ValidationBuffer, buffer, buffer_d);
conversion_function!(
    ICommandList,
    ValidationCommandList,
    command_list,
    command_list_d
);
conversion_function!(
    IDescriptorSetLayout,
    ValidationDescriptorSetLayout,
    descriptor_set_layout,
    descriptor_set_layout_d
);
conversion_function!(IDevice, ValidationDevice, device, device_d);
conversion_function!(IFence, ValidationFence, fence, fence_d);
conversion_function!(
    IGraphicsPipeline,
    ValidationGraphicsPipeline,
    graphics_pipeline,
    graphics_pipeline_d
);
conversion_function!(
    IComputePipeline,
    ValidationComputePipeline,
    compute_pipeline,
    compute_pipeline_d
);
conversion_function!(
    IPipelineLayout,
    ValidationPipelineLayout,
    pipeline_layout,
    pipeline_layout_d
);
conversion_function!(ISampler, ValidationSampler, sampler, sampler_d);
conversion_function!(ISemaphore, ValidationSemaphore, semaphore, semaphore_d);
conversion_function!(IShader, ValidationShader, shader, shader_d);
conversion_function!(ISurface, ValidationSurface, surface, surface_d);
conversion_function!(ISwapChain, ValidationSwapChain, swap_chain, swap_chain_d);
conversion_function!(ITexture, ValidationTexture, texture, texture_d);
