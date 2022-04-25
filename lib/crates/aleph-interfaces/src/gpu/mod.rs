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

pub const API_VERSION_MAJOR: &str = env!("CARGO_PKG_VERSION_MAJOR");
pub const API_VERSION_MINOR: &str = env!("CARGO_PKG_VERSION_MINOR");
pub const API_VERSION_PATCH: &str = env!("CARGO_PKG_VERSION_PATCH");

#[macro_use]
mod misc;

pub use misc::*;

mod adapter;
mod buffer;
mod command_encoder;
mod command_list;
mod command_pool;
mod context;
mod context_provider;
mod descriptor_set;
mod descriptor_set_layout;
mod device;
mod pipeline;
mod queue;
mod resource;
mod sampler;
mod shader;
mod surface;
mod swap_chain;
mod texture;

pub use adapter::*;
pub use buffer::*;
pub use command_encoder::*;
pub use command_list::*;
pub use command_pool::*;
pub use context::*;
pub use context_provider::*;
pub use descriptor_set::*;
pub use descriptor_set_layout::*;
pub use device::*;
pub use pipeline::*;
pub use queue::*;
pub use resource::*;
pub use sampler::*;
pub use sampler::*;
pub use shader::*;
pub use surface::*;
pub use swap_chain::*;
pub use texture::*;
