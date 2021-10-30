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
    IRenderPass, RenderGraphBuilder, RenderPassAccesses, ResourceCreateDesc, ResourceImportDesc,
    ResourceWriteDesc,
};
use std::collections::{HashMap, HashSet};

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

///
/// Internal struct for storing a render pass with its execution dependencies
///
pub(crate) struct RenderPass {
    /// The actual render pass object.
    pub pass: Box<dyn IRenderPass + 'static>,

    /// All direct predecessor nodes that must execute before this render pass can be executed.
    pub predecessors: HashSet<usize>,

    /// All successor nodes that have this pass as a direct dependency.
    pub successors: HashSet<usize>,

    /// The set of resource access this render pass has declared
    pub accesses: RenderPassAccesses,
}

///
/// Internal structure for storing the information for an imported resource
///
pub(crate) struct ImportedResource {
    /// Who uses the resource and how it is used
    pub usage: ResourceUsage,

    /// Import description
    pub desc: ResourceImportDesc,
}

///
/// Internal structure for storing the information for a transient resource
///
pub(crate) struct TransientResource {
    /// The index of the pass that created the transient resource
    pub creator: usize,

    /// Who uses the resource and how it is used
    pub usage: ResourceUsage,

    /// Stores information that depends on the type of the transient resource
    pub r#type: TransientResourceType,
}

///
/// Enum that holds the resource type dependent information
///
pub(crate) enum TransientResourceType {
    /// A root transient resource is a resource that is the direct result of a create operation
    Root { desc: ResourceCreateDesc },

    /// A derived transient resource is a resource that is the result of a write operation to
    /// another resource
    Derived {
        desc: ResourceWriteDesc,
        derived_from: String,
    },
}

///
/// Internal structure for declaring what render passes use a resource and how
///
#[derive(Clone, Default)]
pub(crate) struct ResourceUsage {
    /// The set of passes that read the resource
    pub reads: HashSet<usize>,

    /// The set of passes that write the resource
    pub writes: HashSet<usize>,
}
