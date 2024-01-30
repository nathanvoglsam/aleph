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

#[doc(hidden)]
extern crate aleph_rhi_api;

mod adapter;
mod buffer;
mod command_list;
mod context;
mod descriptor_pool;
mod descriptor_set_layout;
mod device;
mod encoder;
mod fence;
mod internal;
mod pipeline;
mod pipeline_layout;
mod queue;
mod sampler;
mod semaphore;
mod shader;
mod surface;
mod swap_chain;
mod texture;

pub use adapter::NullAdapter;
pub use buffer::NullBuffer;
pub use command_list::NullCommandList;
pub use context::NullContext;
pub use descriptor_pool::NullDescriptorPool;
pub use descriptor_set_layout::NullDescriptorSetLayout;
pub use device::NullDevice;
pub use encoder::NullEncoder;
pub use fence::NullFence;
pub use pipeline::{NullComputePipeline, NullGraphicsPipeline};
pub use pipeline_layout::NullPipelineLayout;
pub use queue::NullQueue;
pub use sampler::NullSampler;
pub use semaphore::NullSemaphore;
pub use shader::NullShader;
pub use surface::NullSurface;
pub use swap_chain::NullSwapChain;
pub use texture::NullTexture;
