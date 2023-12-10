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

use crate::import_bundle::ImportBundle;
use crate::internal::{RenderPass, ResourceRoot, ResourceVersion};
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

    /// The set of resources within the graph that were imported, stored as indices into the
    /// root_resources array.
    pub(crate) imported_resources: Vec<u16>,

    /// The head of the dropper linked-list that contains all the drop functions for objects
    /// allocated from the graph arena
    pub(crate) drop_head: Option<NonNull<DropLink>>,
}

impl FrameGraph {
    pub fn builder() -> FrameGraphBuilder {
        FrameGraphBuilder::new()
    }

    pub unsafe fn execute(&mut self, import_bundle: &ImportBundle) {
        self.execute_pre_assertions(import_bundle);

        for v in self.render_pass_order.iter().copied() {
            let pass = &mut self.render_passes[v];
            unsafe { pass.pass.as_mut().execute() }
        }
    }
}

impl FrameGraph {
    /// Internal function that implements the debug assertions that run prior to the main execute
    /// pass in the frame graph.
    unsafe fn execute_pre_assertions(&mut self, import_bundle: &ImportBundle) {
        if cfg!(debug_assertions) {
            for v in self.imported_resources.iter() {
                let root_resource = &self.root_resources[*v as usize];
                let imported_resource = import_bundle.imports.get(v);

                let name = root_resource
                    .resource_type
                    .name()
                    .unwrap_or("Unnamed resource");

                let is_import = root_resource.resource_type.is_import();
                assert!(
                    is_import,
                    "INTERNAL ERROR: Resource '{}' import status internal mismatch",
                    name
                );

                let contains_import = imported_resource.is_some();
                assert!(
                    contains_import,
                    "The ImportBundle does not contain handle for imported graph resource '{}'",
                    name
                );
                let imported_resource = imported_resource.unwrap();

                match imported_resource {
                    crate::import_bundle::ResourceVariant::Buffer(i) => {
                        match &root_resource.resource_type {
                            crate::internal::ResourceType::Buffer(r) => {
                                assert_eq!(
                                    r.desc.size,
                                    i.desc().size,
                                    "Buffer '{}' not expected size",
                                    name
                                );
                            }
                            crate::internal::ResourceType::Texture(_r) => panic!(
                                "Imported buffer '{}' was provided a texture in the ImportBundle",
                                name
                            ),
                        }
                    }
                    crate::import_bundle::ResourceVariant::Texture(i) => {
                        match &root_resource.resource_type {
                            crate::internal::ResourceType::Buffer(_r) => panic!(
                                "Imported texture '{}' was provided a buffer in the ImportBundle",
                                name
                            ),
                            crate::internal::ResourceType::Texture(r) => {
                                let i_desc = i.desc();
                                assert_eq!(
                                    r.desc.width, i_desc.width,
                                    "Texture '{}' not expected width",
                                    name
                                );
                                assert_eq!(
                                    r.desc.height, i_desc.height,
                                    "Texture '{}' not expected height",
                                    name
                                );
                                assert_eq!(
                                    r.desc.depth, i_desc.depth,
                                    "Texture '{}' not expected depth",
                                    name
                                );
                                assert_eq!(
                                    r.desc.format, i_desc.format,
                                    "Texture '{}' not expected format",
                                    name
                                );
                                assert_eq!(
                                    r.desc.dimension, i_desc.dimension,
                                    "Texture '{}' not expected dimension",
                                    name
                                );
                                assert_eq!(
                                    r.desc.clear_value, i_desc.clear_value,
                                    "Texture '{}' not expected clear_value",
                                    name
                                );
                                assert_eq!(
                                    r.desc.array_size, i_desc.array_size,
                                    "Texture '{}' not expected array_size",
                                    name
                                );
                                assert_eq!(
                                    r.desc.mip_levels, i_desc.mip_levels,
                                    "Texture '{}' not expected mip_levels",
                                    name
                                );
                                assert_eq!(
                                    r.desc.sample_count, i_desc.sample_count,
                                    "Texture '{}' not expected sample_count",
                                    name
                                );
                                assert_eq!(
                                    r.desc.sample_quality, i_desc.sample_quality,
                                    "Texture '{}' not expected sample_quality",
                                    name
                                );
                            }
                        }
                    }
                }
            }
        }
    }
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
