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

use crate::render_graph::internals::{
    ImportedResource, RenderPass, ResourceUsage, TransientResource, TransientResourceType,
};
use crate::render_graph::{IRenderPass, RenderGraph, RenderPassAccesses, ResourceImportDesc};
use std::collections::{HashMap, HashSet};

pub struct RenderGraphBuilder {
    pass_names: HashMap<String, usize>,
    pass_storage: Vec<RenderPass>,
    imports: HashMap<String, ImportedResource>,
    exports: HashMap<String, ()>,
}

impl RenderGraphBuilder {
    pub fn new() -> Self {
        Self {
            pass_names: HashMap::new(),
            pass_storage: Vec::new(),
            imports: HashMap::new(),
            exports: HashMap::new(),
        }
    }

    #[inline]
    pub fn add_pass<T: IRenderPass + 'static>(&mut self, name: impl Into<String>, pass: T) {
        // Capture the current pass index
        let index = self.pass_storage.len();

        // Box and push the pass
        self.pass_storage.push(RenderPass {
            pass: Box::new(pass),
            predecessors: HashSet::new(),
            successors: HashSet::new(),
            accesses: RenderPassAccesses::default(),
        });

        // Insert the name mapping
        assert!(self.pass_names.insert(name.into(), index).is_none());
    }

    #[inline]
    pub fn import_resource(
        &mut self,
        name: impl Into<String>,
        desc: impl Into<ResourceImportDesc>,
    ) {
        assert!(self
            .imports
            .insert(
                name.into(),
                ImportedResource {
                    usage: Default::default(),
                    desc: desc.into()
                }
            )
            .is_none())
    }

    #[inline]
    pub fn export_resource(&mut self, name: impl Into<String>) {
        assert!(self.exports.insert(name.into(), ()).is_none())
    }

    pub fn build(mut self) -> RenderGraph {
        // Collect the resource access of each render pass
        self.pass_storage.iter_mut().for_each(|v| {
            v.pass.declare_access(&mut v.accesses);
        });

        // Collect information on root transient resources and derived transient resources
        let mut transients = HashMap::new();
        self.collect_root_transients(&mut transients);
        self.collect_derived_transients(&mut transients);
        self.validate_transients(&transients);

        let mut last_reads = HashMap::new();
        let mut last_write = HashMap::new();
        for pass_index in 0..self.pass_storage.len() {
            self.handle_writes(
                &mut transients,
                &mut last_write,
                &mut last_reads,
                pass_index,
            );
            self.handle_reads(
                &mut transients,
                &mut last_write,
                &mut last_reads,
                pass_index,
            );
        }

        RenderGraph {
            pass_names: self.pass_names,
            pass_storage: self.pass_storage,
            transients,
            imports: self.imports,
            exports: self.exports,
        }
    }

    fn collect_root_transients(&mut self, transients: &mut HashMap<String, TransientResource>) {
        // This stores the set of resources that exist as the result of explicit create operations.
        //
        // This doesn't include the results of write operations, which while technically being "new"
        // resources they are not "root" resources as the resource is basically just a renamed
        // handle to the underlying root transient resource.
        self.pass_storage.iter().enumerate().for_each(|(i, v)| {
            v.accesses.creates.iter().for_each(|v| {
                let name = v.0;
                let is_duplicate = transients
                    .insert(
                        name.clone(),
                        TransientResource {
                            creator: i,
                            usage: ResourceUsage::default(),
                            r#type: TransientResourceType::Root { desc: v.1.clone() },
                        },
                    )
                    .is_some();

                // It is invalid to create a resource with the same name as an imported resource as
                // all names must be unique
                assert!(!self.imports.contains_key(name));

                // It is invalid to create two resources with the same name as all resources names
                // must be unique
                assert!(!is_duplicate);
            });
        });
    }

    fn collect_derived_transients(&mut self, transients: &mut HashMap<String, TransientResource>) {
        // This stores the set of resources that exist as a result of write operations producing
        // a renamed handle to the resource being written to.
        self.pass_storage.iter().enumerate().for_each(|(i, v)| {
            v.accesses.writes.iter().for_each(|v| {
                let name = &v.1.result;
                let transient = TransientResource {
                    creator: i,
                    usage: ResourceUsage::default(),
                    r#type: TransientResourceType::Derived {
                        derived_from: v.0.clone(),
                        desc: v.1.access.clone(),
                    },
                };
                let is_duplicate = transients.insert(name.clone(), transient).is_some();

                // A derived resource's result name can not clash with the names of any imported
                // resources as all names must be unique
                assert!(!self.imports.contains_key(name));

                // A derived resource's result name can not clash with any of the root resource
                // names as all names must be unique
                assert!(!transients.contains_key(name));

                // It is invalid to write to the same resource twice as a write "consumes" the
                // resource and emits a new handle that refers to the results of the write
                assert!(!is_duplicate);
            });
        });
    }

    fn validate_transients(&self, transients: &HashMap<String, TransientResource>) {
        transients
            .iter()
            .filter_map(|(resource, info)| match &info.r#type {
                TransientResourceType::Root { .. } => None,
                TransientResourceType::Derived { derived_from, .. } => {
                    Some((resource, derived_from))
                }
            })
            .for_each(|(resource, derived_from)| {
                let reads_transient = transients.contains_key(derived_from);
                let reads_existing_resource =
                    reads_transient || self.imports.contains_key(derived_from);
                assert!(
                    reads_existing_resource,
                    "Resource \"{}\" is derived from resource \"{}\", but \"{}\" does not exist",
                    resource, derived_from, derived_from
                );
            })
    }

    fn handle_writes(
        &mut self,
        transients: &mut HashMap<String, TransientResource>,
        last_write: &mut HashMap<String, usize>,
        last_reads: &mut HashMap<String, Vec<usize>>,
        pass_index: usize,
    ) {
        let accesses = std::mem::take(&mut self.pass_storage[pass_index].accesses);
        let writes = accesses.writes.keys().chain(accesses.creates.keys());

        for write in writes {
            lookup_resource_usage(&mut self.imports, transients, write)
                .writes
                .insert(pass_index);

            last_write.insert(write.to_owned(), pass_index);

            match last_reads.get_mut(write) {
                None => {}
                Some(reads) => {
                    for read in reads.iter().copied() {
                        if read != pass_index {
                            self.pass_storage[pass_index].predecessors.insert(read);
                            self.pass_storage[read].successors.insert(pass_index);
                        }
                    }
                    reads.clear();
                }
            }
        }

        self.pass_storage[pass_index].accesses = accesses;
    }

    fn handle_reads(
        &mut self,
        transients: &mut HashMap<String, TransientResource>,
        last_write: &mut HashMap<String, usize>,
        last_reads: &mut HashMap<String, Vec<usize>>,
        pass_index: usize,
    ) {
        let accesses = std::mem::take(&mut self.pass_storage[pass_index].accesses);
        let reads = accesses.reads.keys();

        for read in reads {
            lookup_resource_usage(&mut self.imports, transients, read)
                .reads
                .insert(pass_index);

            match last_reads.get_mut(read) {
                None => {
                    let mut vec = Vec::with_capacity(4);
                    vec.push(pass_index);
                    last_reads.insert(read.to_owned(), vec);
                }
                Some(vec) => {
                    vec.push(pass_index);
                }
            }

            match last_write.get(read).copied() {
                None => {}
                Some(write) => {
                    if write != pass_index {
                        self.pass_storage[pass_index].predecessors.insert(write);
                        self.pass_storage[write].successors.insert(pass_index);
                    }
                }
            }
        }

        self.pass_storage[pass_index].accesses = accesses;
    }
}

impl Default for RenderGraphBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Utility for looking up a resource by name across all types
fn lookup_resource_usage<'a, 'b>(
    imported: &'a mut HashMap<String, ImportedResource>,
    transients: &'a mut HashMap<String, TransientResource>,
    transient: &'b str,
) -> &'a mut ResourceUsage {
    transients
        .get_mut(transient)
        .map(|v| &mut v.usage)
        .unwrap_or_else(move || imported.get_mut(transient).map(|v| &mut v.usage).unwrap())
}
