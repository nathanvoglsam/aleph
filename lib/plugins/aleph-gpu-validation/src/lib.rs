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

extern crate aleph_interfaces as interfaces;

mod adapter;
mod buffer;
mod command_list;
mod command_pool;
mod context;
mod descriptor_pool;
mod descriptor_set_layout;
mod device;
mod encoder;
mod internal;
mod pipeline;
mod pipeline_layout;
mod queue;
mod sampler;
mod shader;
mod surface;
mod swap_chain;
mod texture;

pub use buffer::ValidationBuffer;
pub use command_list::ValidationCommandList;
pub use context::ValidationContext;
pub use device::ValidationDevice;
pub use pipeline::ValidationComputePipeline;
pub use pipeline::ValidationGraphicsPipeline;
pub use pipeline_layout::ValidationPipelineLayout;
pub use shader::ValidationShader;
pub use swap_chain::ValidationSwapChain;
pub use texture::ValidationTexture;
