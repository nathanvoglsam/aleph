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

mod internals;

mod callback_render_pass;
mod render_graph_builder;
mod render_pass;
mod resource_accesses;
mod resource_slot;

pub use callback_render_pass::CallbackPass;
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
pub use resource_slot::ResourceSlot;

use internals::{ImportedResource, RenderPass, TransientResource};
use std::collections::HashMap;

///
/// A `RenderGraph` represents a grouped container of `IRenderPass` objects that are defined against
/// that together represent a graph of GPU work.
///
/// All dependencies specified by the render passes are specified in terms of this
///
pub struct RenderGraph {
    /// Maps the name of a render pass to the index in the pass_storage array that contains it
    pub(crate) pass_names: HashMap<String, usize>,

    /// Storage array for all render passes
    pub(crate) pass_storage: Vec<RenderPass>,

    /// The set of all transient resources used by this render graph
    pub(crate) transients: HashMap<String, TransientResource>,

    /// The set of all resources imported into this render graph
    pub(crate) imports: HashMap<String, ImportedResource>,

    /// The set of all resources exported from this render graph
    pub(crate) exports: HashMap<String, ()>,
}

impl RenderGraph {
    pub fn builder() -> RenderGraphBuilder {
        RenderGraphBuilder::new()
    }
}
