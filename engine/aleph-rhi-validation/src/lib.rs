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
mod binding_signature;
mod buffer;
mod command_list;
mod context;
mod descriptor_arena;
mod descriptor_pool;
mod device;
mod encoder;
mod fence;
mod internal;
mod parameter_block_layout;
mod pipeline;
mod queue;
mod sampler;
mod semaphore;
mod surface;
mod swap_chain;
mod swap_image;
mod texture;

pub use adapter::ValidationAdapter;
pub use binding_signature::ValidationBindingSignature;
pub use buffer::ValidationBuffer;
pub use command_list::ValidationCommandList;
pub use context::ValidationContext;
pub use descriptor_arena::ValidationDescriptorArena;
pub use descriptor_pool::ValidationDescriptorPool;
pub use device::ValidationDevice;
pub use encoder::ValidationEncoder;
pub use fence::ValidationFence;
pub use parameter_block_layout::ValidationParameterBlockLayout;
pub use pipeline::{ValidationComputePipeline, ValidationGraphicsPipeline};
pub use queue::ValidationQueue;
pub use sampler::ValidationSampler;
pub use semaphore::ValidationSemaphore;
pub use surface::ValidationSurface;
pub use swap_chain::ValidationSwapChain;
pub use swap_image::ValidationSwapImage;
pub use texture::ValidationTexture;
