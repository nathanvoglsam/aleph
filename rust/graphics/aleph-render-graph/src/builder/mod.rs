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

mod access;
mod external;
mod usages;

#[cfg(test)]
mod tests;

pub use access::BufferReadDescription;
pub use access::BufferWriteDescription;
pub use access::ImageReadDescription;
pub use access::ImageWriteDescription;
pub use access::ResourceAccess;
pub use external::BufferExport;
pub use external::BufferImport;
pub use external::ImageExport;
pub use external::ImageImport;
pub use external::ResourceExport;
pub use external::ResourceImport;

use crate::builder::usages::{BufferRead, BufferUsage, ImageRead, ImageUsage, ResourceUsage};
use crate::graph::{GraphLink, PassIndex, RenderGraph, RenderGraphPass};
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

///
/// Represents the set of errors that could be produced when building a graph
///
#[derive(Clone, Debug, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub enum GraphBuildError {
    /// Error thrown when two or more passes try to write a resource with the same identifier. SSA
    /// form makes this invalid.
    MultipleWritesToSameResource,

    /// A pass tried to read from a resource that does not exist. This could be because a previous
    /// pass did not write it, or an external resource was not registered.
    ReadResourceDoesNotExist,

    /// Error thrown when a pass tries to use a resource as the incorrect type. For example, an
    /// image resource is written and a subsequent pass tries to read it as a buffer.
    IncorrectResourceType,
}

///
/// We use this builder interface to provided the framework for enumerating the passes that will be
/// in the resulting graph.
///
/// SSA form is very optimal for our graph *description* interface but not for the actual
/// representation of the graph when being used. SSA form was created for use in *intermediate*
/// representations, not direct representation after all.
///
/// This builder is where we list off all our passes, and then pay the cost for converting from our
/// graph description into the final graph that handles rendering
///
pub struct RenderGraphBuilder<'a> {
    ///
    /// Need a list of all the passes we want in the output graph
    ///
    passes: Vec<Box<dyn RenderGraphPass + 'a>>,

    ///
    /// Maps identifiers to images to be imported into the frame graph
    ///
    imports: HashMap<String, ResourceImport>,

    ///
    /// List of images to be exported and the state it should be exported in
    ///
    exports: HashMap<String, ResourceExport>,
}

impl<'a> RenderGraphBuilder<'a> {
    ///
    /// Creates a new builder
    ///
    pub fn new() -> Self {
        Self {
            passes: Vec::new(),
            imports: HashMap::new(),
            exports: HashMap::default(),
        }
    }

    ///
    /// Add a new pass to the builder
    ///
    pub fn pass(&mut self, pass: impl RenderGraphPass + 'a) -> &mut Self {
        self.passes.push(Box::new(pass));
        self
    }

    ///
    /// Mark a resource, by identifier, as a resource that should be exported from the graph.
    ///
    /// # Panics
    ///
    /// Panics if the same resource identifier is asked to be exported multiple times
    ///
    pub fn export_image(&mut self, identifier: &str, image: ImageExport) -> &mut Self {
        self.export_resource(identifier, ResourceExport::Image(image))
    }

    ///
    /// Mark a resource, by identifier, as a resource that should be exported from the graph.
    ///
    /// # Panics
    ///
    /// Panics if the same resource identifier is asked to be exported multiple times
    ///
    pub fn export_buffer(&mut self, identifier: &str, buffer: BufferExport) -> &mut Self {
        self.export_resource(identifier, ResourceExport::Buffer(buffer))
    }

    ///
    /// Mark a resource, by identifier, as a resource that should be exported from the graph.
    ///
    /// # Panics
    ///
    /// Panics if the same resource identifier is asked to be exported multiple times
    ///
    pub fn export_resource(&mut self, identifier: &str, resource: ResourceExport) -> &mut Self {
        let already_exists = self
            .exports
            .insert(identifier.to_string(), resource)
            .is_some();

        assert!(
            !already_exists,
            "Tried inserting multiple resources with the same identifier"
        );

        self
    }

    ///
    /// Import the given resource under the provided identifier
    ///
    /// # Panics
    ///
    /// Panics if a resource has already been imported with the given identifier
    ///
    pub fn import_image(&mut self, identifier: &str, image: ImageImport) -> &mut Self {
        self.import_resource(identifier, ResourceImport::Image(image))
    }

    ///
    /// Mark a resource, by identifier, as a resource that should be exported from the graph.
    ///
    /// # Panics
    ///
    /// Panics if the same resource identifier is asked to be exported multiple times
    ///
    pub fn import_buffer(&mut self, identifier: &str, buffer: BufferImport) -> &mut Self {
        self.import_resource(identifier, ResourceImport::Buffer(buffer))
    }

    ///
    /// Import the given resource under the provided identifier
    ///
    /// # Panics
    ///
    /// Panics if a resource has already been imported with the given identifier
    ///
    pub fn import_resource(&mut self, identifier: &str, resource: ResourceImport) -> &mut Self {
        let already_exists = self
            .imports
            .insert(identifier.to_string(), resource)
            .is_some();

        assert!(
            !already_exists,
            "Tried inserting multiple resources with the same identifier"
        );

        self
    }

    ///
    /// Consume the builder and build a render graph
    ///
    pub fn build(self) -> Result<RenderGraph<'a>, GraphBuildError> {
        // Move passes out of self for naming consistency
        let mut passes = self.passes;
        let imports = self.imports;
        let exports = self.exports;

        let accesses = Self::resolve_accesses(&mut passes);
        let mut usages = Self::resolve_usages(&imports, &exports, &accesses)?;
        let links = Self::resolve_links(&mut passes, &mut usages);

        // Build the actual render graph struct itself
        let graph = RenderGraph::<'a> {
            passes,
            links,
            accesses,
        };

        Ok(graph)
    }

    ///
    /// Internal function for resolving the passes access declarations into a list of said accesses
    ///
    fn resolve_accesses(passes: &mut Vec<Box<dyn RenderGraphPass + 'a>>) -> Vec<ResourceAccess> {
        // First we need to resolve all the resource access objects from the provided passes into
        // a single list. We have to do this as we will be performing multiple passes over this list
        // to resolve the graph
        let accesses: Vec<ResourceAccess> = passes
            .iter_mut()
            .map(|pass| {
                let mut accesses = ResourceAccess {
                    image_reads: Vec::new(),
                    image_writes: Vec::new(),
                    buffer_reads: Vec::new(),
                    buffer_writes: Vec::new(),
                };

                pass.register_access(&mut accesses);

                accesses
            })
            .collect();
        accesses
    }

    ///
    /// Internal function for resolving all resource usage information that will be consumed by the
    /// pass scheduling phase
    ///
    fn resolve_usages(
        imports: &HashMap<String, ResourceImport>,
        exports: &HashMap<String, ResourceExport>,
        accesses: &Vec<ResourceAccess>,
    ) -> Result<HashMap<String, ResourceUsage>, GraphBuildError> {
        let mut usages: HashMap<String, ResourceUsage> = HashMap::new();

        Self::resolve_imports(imports, &mut usages);
        Self::resolve_writes(accesses, &mut usages)?;
        Self::resolve_reads(accesses, &mut usages)?;
        Self::resolve_exports(exports, &mut usages)?;

        Ok(usages)
    }

    ///
    /// Imports all external resources as writes by `PassIndex::EXTERNAL` with the widest possible
    /// synchronization scope.
    ///
    fn resolve_imports(
        imports: &HashMap<String, ResourceImport>,
        usages: &mut HashMap<String, ResourceUsage>,
    ) {
        // Each import is equal to a write from `PassIndex::EXTERNAL` so we insert a new resource
        // usage into the map
        for (identifier, import) in imports.iter() {
            let resource = match import {
                ResourceImport::Image(import) => {
                    let usage = ImageUsage {
                        writen_by: PassIndex::EXTERNAL,
                        read_types: Vec::new(),
                        read_by: Vec::new(),
                        layout: import.current_layout,
                        stages: import.stages,
                        access_types: import.access_types,
                    };
                    ResourceUsage::Image(usage)
                }
                ResourceImport::Buffer(import) => {
                    let usage = BufferUsage {
                        writen_by: PassIndex::EXTERNAL,
                        read_type: BufferRead {
                            ..Default::default()
                        },
                        read_by: Vec::new(),
                        stages: import.stages,
                        access_types: import.access_types,
                    };
                    ResourceUsage::Buffer(usage)
                }
            };
            // This is valid because the list of imports is already checked for duplicates. If it
            // wasn't then we would have to check if there was already a resource written with the
            // same identifier and emit an error if there was
            usages.insert(identifier.clone(), resource);
        }
    }

    ///
    /// Resolves all resource writes into the usages map. This MUST be run before the corresponding
    /// reads resolution as it sets up the map for the reads resolution to consume.
    ///
    fn resolve_writes(
        accesses: &Vec<ResourceAccess>,
        usages: &mut HashMap<String, ResourceUsage>,
    ) -> Result<(), GraphBuildError> {
        // Iterate over all the access sets emitted by the passes from their `register_access` phase
        // and insert and validate the image writes that each pass requested
        for (pass_index, access) in accesses.iter().enumerate() {
            let pass_index = PassIndex::new(pass_index);
            for image_write in &access.image_writes {
                Self::handle_image_write(usages, pass_index, image_write)?;
            }
            for buffer_write in &access.buffer_writes {
                Self::handle_buffer_write(usages, pass_index, buffer_write)?;
            }
        }
        Ok(())
    }

    ///
    /// Internal function used by `resolve_image_writes` to make the loop less hideous
    ///
    fn handle_image_write(
        usages: &mut HashMap<String, ResourceUsage>,
        pass_index: PassIndex,
        image_write: &ImageWriteDescription,
    ) -> Result<(), GraphBuildError> {
        // We use the entry API here, and check if there already exists a resource usage for the
        // identifier we're providing. This check is required by the constraints of SSA form, which
        // specifies that each value is written to exactly once.
        match usages.entry(image_write.identifier.clone()) {
            Entry::Occupied(_) => {
                return Err(GraphBuildError::MultipleWritesToSameResource);
            }
            Entry::Vacant(vacant) => {
                let usage = ImageUsage {
                    writen_by: pass_index,
                    read_types: Vec::new(),
                    read_by: Vec::new(),
                    layout: image_write.layout,
                    stages: image_write.stages,
                    access_types: image_write.access_types,
                };
                vacant.insert(ResourceUsage::Image(usage));
            }
        }
        Ok(())
    }

    ///
    /// Internal function used by `resolve_image_writes` to make the loop less hideous
    ///
    fn handle_buffer_write(
        usages: &mut HashMap<String, ResourceUsage>,
        pass_index: PassIndex,
        buffer_write: &BufferWriteDescription,
    ) -> Result<(), GraphBuildError> {
        // We use the entry API here, and check if there already exists a resource usage for the
        // identifier we're providing. This check is required by the constraints of SSA form, which
        // specifies that each value is written to exactly once.
        match usages.entry(buffer_write.identifier.clone()) {
            Entry::Occupied(_) => {
                return Err(GraphBuildError::MultipleWritesToSameResource);
            }
            Entry::Vacant(vacant) => {
                let usage = BufferUsage {
                    writen_by: pass_index,
                    read_type: BufferRead {
                        ..Default::default()
                    },
                    read_by: Vec::new(),
                    stages: buffer_write.stages,
                    access_types: buffer_write.access_types,
                };
                vacant.insert(ResourceUsage::Buffer(usage));
            }
        }
        Ok(())
    }

    ///
    /// Internal function for resolving the list of reads into the list of resource usages
    ///
    fn resolve_reads(
        accesses: &Vec<ResourceAccess>,
        usages: &mut HashMap<String, ResourceUsage>,
    ) -> Result<(), GraphBuildError> {
        // Iterate over all the access sets emitted by the passes from their `register_access` phase
        // and insert and validate the image reads that each pass requested
        for (pass_index, access) in accesses.iter().enumerate() {
            let pass_index = PassIndex::new(pass_index);
            for image_read in &access.image_reads {
                Self::handle_image_read(usages, pass_index, image_read)?;
            }
            for buffer_read in &access.buffer_reads {
                Self::handle_buffer_read(usages, pass_index, buffer_read)?;
            }
        }
        Ok(())
    }

    ///
    /// Internal function used by `resolve_image_reads` to make the loop less hideous
    ///
    fn handle_image_read(
        usages: &mut HashMap<String, ResourceUsage>,
        pass_index: PassIndex,
        image_read: &ImageReadDescription,
    ) -> Result<(), GraphBuildError> {
        // Get the usage info for the resource we're trying to read from by identifier. If we can't
        // find an entry then we're tying to read from a non existent resource and should emit an
        // error.
        if let Some(usage) = usages.get_mut(&image_read.identifier) {
            // We're handling image reads so assert that the resource is actually an image resource
            if let ResourceUsage::Image(usage) = usage {
                // Find a read in the same layout
                let read_type = usage
                    .read_types
                    .iter_mut()
                    .enumerate()
                    .find(|(_, v)| v.layout == image_read.layout);

                // If there is a read in the same layout, merge the barrier information
                // otherwise insert a new read type.
                //
                // This is important so we can group reads to the same image in the same layout
                // together and emit a single barrier that can be shared by multiple passes.
                if let Some((read_type_index, read_type)) = read_type {
                    read_type.stages |= image_read.stages;
                    read_type.access_types |= image_read.access_types;
                    usage.read_by.push((pass_index, read_type_index));
                } else {
                    let read_type_index = usage.read_types.len();
                    let read_type = ImageRead {
                        layout: image_read.layout,
                        stages: image_read.stages,
                        access_types: image_read.access_types,
                    };
                    usage.read_types.push(read_type);
                    usage.read_by.push((pass_index, read_type_index));
                }
            } else {
                return Err(GraphBuildError::IncorrectResourceType);
            }
        } else {
            return Err(GraphBuildError::ReadResourceDoesNotExist);
        }
        Ok(())
    }

    ///
    /// Internal function used by `resolve_image_reads` to make the loop less hideous
    ///
    fn handle_buffer_read(
        usages: &mut HashMap<String, ResourceUsage>,
        pass_index: PassIndex,
        buffer_read: &BufferReadDescription,
    ) -> Result<(), GraphBuildError> {
        // Get the usage info for the resource we're trying to read from by identifier. If we can't
        // find an entry then we're tying to read from a non existent resource and should emit an
        // error.
        if let Some(usage) = usages.get_mut(&buffer_read.identifier) {
            // We're handling buffer reads so assert that the resource is actually a buffer resource
            if let ResourceUsage::Buffer(usage) = usage {
                usage.read_type.stages |= buffer_read.stages;
                usage.read_type.access_types |= buffer_read.access_types;
                usage.read_by.push((pass_index, 0));
            } else {
                return Err(GraphBuildError::IncorrectResourceType);
            }
        } else {
            return Err(GraphBuildError::ReadResourceDoesNotExist);
        }
        Ok(())
    }

    ///
    /// Exports all resources that are wanted outside of the graph as reads from PASS_INDEX_EXTERNAL
    ///
    fn resolve_exports(
        exports: &HashMap<String, ResourceExport>,
        usages: &mut HashMap<String, ResourceUsage>,
    ) -> Result<(), GraphBuildError> {
        // Each export is equal to a read from `PassIndex::EXTERNAL` so we try to insert a new read
        // for each export into the usage list
        for (identifier, export) in exports.iter() {
            // Get the usage info for the resource we're trying to read from by identifier. If we can't
            // find an entry then we're tying to read from a non existent resource and should emit an
            // error.
            if let Some(usage) = usages.get_mut(identifier) {
                match usage {
                    ResourceUsage::Image(usage) => {
                        if let ResourceExport::Image(export) = export {
                            let pass_index = PassIndex::EXTERNAL;
                            let read_type = ImageRead {
                                layout: export.layout,
                                stages: export.stages,
                                access_types: export.access_types,
                            };
                            let read_type_index = usage.read_types.len();
                            usage.read_types.push(read_type);
                            usage.read_by.push((pass_index, read_type_index));
                        } else {
                            return Err(GraphBuildError::IncorrectResourceType);
                        }
                    }
                    ResourceUsage::Buffer(usage) => {
                        if let ResourceExport::Buffer(export) = export {
                            let pass_index = PassIndex::EXTERNAL;
                            usage.read_type.stages |= export.stages;
                            usage.read_type.access_types |= export.access_types;
                            usage.read_by.push((pass_index, 0));
                        } else {
                            return Err(GraphBuildError::IncorrectResourceType);
                        }
                    }
                }
            } else {
                return Err(GraphBuildError::ReadResourceDoesNotExist);
            }
        }
        Ok(())
    }

    fn resolve_links(
        passes: &mut Vec<Box<dyn RenderGraphPass + 'a>>,
        usages: &mut HashMap<String, ResourceUsage>,
    ) -> Vec<GraphLink> {
        // Create a list of links, with each link blank in preparation for building the graph from
        // the list of resources
        let mut links: Vec<GraphLink> = passes
            .iter()
            .map(|_| GraphLink {
                depends_on: HashSet::new(),
                waited_on_by: HashSet::new(),
            })
            .collect();

        // Because of our SSA form requirement building the links for the graph is really this easy,
        // just iterate over the resources and make anything that reads from a resource depend on
        // the writer and update the writer's link to include the reader.
        usages.iter().for_each(|(_, usage)| {
            usage.read_by().iter().for_each(|(read, _)| {
                // Make sure we don't try and write dependencies for PASS_INDEX_EXTERNAL
                if let Some(read) = read.get() {
                    links[read].depends_on.insert(usage.writen_by());
                }
                if let Some(writen_by) = usage.writen_by().get() {
                    links[writen_by].waited_on_by.insert(*read);
                }
            });
        });
        links
    }
}
