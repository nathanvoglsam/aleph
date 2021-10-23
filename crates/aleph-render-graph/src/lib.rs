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

#![cfg(target_os = "windows")]

//!
//!
//!

extern crate aleph_dx12 as dx12;
extern crate aleph_dx12_alloc as dx12_alloc;
extern crate aleph_pix as pix;

extern crate aleph_interfaces as interfaces;
extern crate aleph_log as log;

mod callback_render_pass;
mod render_graph;
mod render_graph_builder;
mod render_pass;
mod resource_accesses;

pub use callback_render_pass::CallbackPass;
pub use render_graph::RenderGraph;
pub use render_graph_builder::RenderGraphBuilder;
pub use render_pass::IRenderPass;
pub use render_pass::RenderPassAccesses;
pub use resource_accesses::BufferCreateDesc;
pub use resource_accesses::BufferImportDesc;
pub use resource_accesses::ResourceCreateDesc;
pub use resource_accesses::ResourceImportDesc;
pub use resource_accesses::ResourceReadDesc;
pub use resource_accesses::ResourceWriteDesc;
pub use resource_accesses::TextureCreateDesc;
pub use resource_accesses::TextureImportDesc;
