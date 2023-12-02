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

use crate::internal::{RenderPass, ResourceHandleInfo, ResourceRoot, ResourceVersion};
use crate::FrameGraphBuilder;
use aleph_arena_drop_list::DropLink;
use bumpalo::Bump;
use std::ptr::NonNull;

pub struct FrameGraph {
    /// The bump allocation arena that provides the backing memory for the render passes and any
    /// other memory that's needed for them.
    ///
    /// This typically includes the IRenderPass objects themselves, their name strings and the
    /// payload objects for callback passes.
    ///
    /// This won't be directly used by a constructed graph but must be stored inside the graph in
    /// order to keep the allocations for all the render passes alive.
    pub(crate) _arena: Bump,

    /// List of render pass indices in order from first to run to last to run
    pub(crate) render_pass_order: Vec<usize>,

    /// The list of all the render passes in the graph. The index of the pass in this list is the
    /// identity of the pass and is used to key to a number of different names
    pub(crate) render_passes: Vec<RenderPass>,

    /// The backing storage used for all of the root resourcs objects. A root resource represents
    /// a concrete [ITexture] or [IBuffer] as created by the graph. This includes both the created
    /// transient resources as well as imported resources. Imported resources are identified by
    /// having their index in the 'imported_resources' set.
    pub(crate) root_resources: Vec<ResourceRoot>,

    /// The backing storage used for all the resource version objects. A resource version is an
    /// indexed set that is used to identify a particular version of a root resource.
    ///
    /// A 'ResourceVersion' contains the accumulated usages of the resource as well as a link to
    /// the previous version to form a linked-list of resource versions back to the first version
    /// when the resource was created or imported.
    ///
    /// These entries are critical as resource versions are what form the core of the graph. They
    /// are what allows the graph to construct a stable program order via an SSA form graph
    /// construction.
    pub(crate) resource_versions: Vec<ResourceVersion>,

    /// The head of the dropper linked-list that contains all the drop functions for objects
    /// allocated from the graph arena
    pub(crate) drop_head: Option<NonNull<DropLink>>,
}

impl FrameGraph {
    pub fn builder() -> FrameGraphBuilder {
        FrameGraphBuilder::new()
    }

    pub unsafe fn execute(&mut self) {}
}

impl Drop for FrameGraph {
    fn drop(&mut self) {
        // Safety: implementation and API guarantees that dropper only gets called once per
        //         object, and always on the correct type.
        unsafe {
            DropLink::drop_and_null(&mut self.drop_head);
        }
    }
}
