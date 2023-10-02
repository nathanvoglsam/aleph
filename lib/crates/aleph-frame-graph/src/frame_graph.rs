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

use crate::frame_graph_builder::internal::{ResourceHandleInfo, ResourceRoot, ResourceVersion};
use crate::render_pass::IRenderPass;
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
    pub(crate) arena: Bump,

    /// The list of all the render passes in the graph. The index of the pass in this list is the
    /// identity of the pass and is used to key to a number of different names
    pub(crate) render_passes: Vec<NonNull<dyn IRenderPass>>,

    /// Stores the names of each render pass keyed by the matching index in the render_passes list.
    pub(crate) render_pass_names: Vec<(*const u8, usize)>,

    pub(crate) root_resources: Vec<ResourceRoot>,
    pub(crate) resource_versions: Vec<ResourceVersion>,

    /// Stores debug information for each resource handle generated at a resource rename event. This
    /// is used to help validate resources are accessed in a valid way.
    pub(crate) resource_handles: Vec<ResourceHandleInfo>,

    /// The head of the dropper linked-list that contains all the drop functions for the render
    /// passes.
    pub(crate) pass_dropper_head: Option<NonNull<DropLink>>,

    /// The head of the dropper linked-list that contains droppers for the callback pass payloads.
    pub(crate) payload_dropper_head: Option<NonNull<DropLink>>,
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
            DropLink::drop_and_null(&mut self.pass_dropper_head);
            DropLink::drop_and_null(&mut self.payload_dropper_head);
        }
    }
}
