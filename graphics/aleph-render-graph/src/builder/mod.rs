//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

mod access;
mod usages;

#[cfg(test)]
mod tests;

pub use access::ImageReadDescription;
pub use access::ImageWriteDescription;
pub use access::ResourceAccess;

use crate::builder::usages::{ImageRead, ImageUsage, ResourceUsage};
use crate::graph::{GraphLink, RenderGraph, RenderGraphPass, PASS_INDEX_EXTERNAL};
use crate::resource::{ImageResource, Resource, SWAP_IMAGE_RESERVED_NAME};
use aleph_vulkan_core::erupt::vk1_0::{AccessFlags, Extent2D, ImageLayout, PipelineStageFlags};
use aleph_vulkan_core::SwapImage;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

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
    IncorrectResourceTypeError,
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
    /// The list of resources imported into the graph from external sources
    ///
    resources: HashMap<String, Resource>,
}

impl<'a> RenderGraphBuilder<'a> {
    ///
    /// Creates a new builder
    ///
    pub fn new() -> Self {
        Self {
            passes: Vec::new(),
            resources: HashMap::new(),
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
    /// Register the given SwapImage with the graph builder. A SwapImage is a simplified type of
    /// image resource that is assumed to be in an undefined layout.
    ///
    pub fn swap_image(&mut self, swap_image: SwapImage) -> &mut Self {
        let image_view = swap_image.image_view();
        let initial_layout = ImageLayout::UNDEFINED;
        let format = swap_image.format();
        let extent = Extent2D {
            width: swap_image.width(),
            height: swap_image.height(),
        };
        let resource = ImageResource {
            image_view,
            initial_layout,
            format,
            extent,
        };
        self.resource(SWAP_IMAGE_RESERVED_NAME, Resource::Image(resource))
    }

    ///
    /// Internal wrapper around the underlying resource map
    ///
    fn resource(&mut self, identifier: &str, resource: Resource) -> &mut Self {
        let already_exists = self
            .resources
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
        let mut resources = self.resources;

        let accesses = Self::resolve_accesses(&mut passes);
        let mut resources = Self::resolve_usages(&mut resources, &accesses)?;
        let links = Self::resolve_links(&mut passes, &mut resources);

        // Build the actual render graph struct itself
        let graph = RenderGraph::<'a> {
            passes,
            links,
            accesses,
        };

        Ok(graph)
    }

    ///
    /// Internal function for resolving the passess access declarations into a list of said accesses
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
        resources: &HashMap<String, Resource>,
        accesses: &Vec<ResourceAccess>,
    ) -> Result<HashMap<String, ResourceUsage>, GraphBuildError> {
        let mut usages: HashMap<String, ResourceUsage> = HashMap::new();

        // First we import all external resources by marking them as writes from an "external" pass
        // and use the widest possible synchronization for the resource.
        Self::resolve_usages_external(resources, &mut usages);

        // Creates new usage entries for each image write. This also prepares the usage entry for
        // the read resolution stage.
        Self::resolve_usages_image_writes(accesses, &mut usages)?;

        // Adds to each image "created" with a write the passes and ways the resource is written in
        // after it's initial write.
        Self::resolve_usages_image_reads(accesses, &mut usages)?;

        Ok(usages)
    }

    ///
    /// Imports all external resources as writes by "PASS_INDEX_EXTERNAL" with the widest possible
    /// synchronization scope.
    ///
    fn resolve_usages_external(
        resources: &HashMap<String, Resource>,
        usages: &mut HashMap<String, ResourceUsage>,
    ) {
        // For each
        for (identifier, resource) in resources.iter() {
            match resource {
                Resource::Image(resource) => {
                    let usage = ImageUsage {
                        writen_by: PASS_INDEX_EXTERNAL,
                        read_types: Vec::new(),
                        read_by: Vec::new(),
                        layout: resource.initial_layout,
                        stages: PipelineStageFlags::ALL_COMMANDS,
                        access_types: AccessFlags::MEMORY_WRITE,
                    };
                    usages.insert(identifier.clone(), ResourceUsage::Image(usage));
                }
            }
        }
    }

    ///
    /// Resolves all resource writes into the usages map. This MUST be run before the corresponding
    /// reads resolution as it sets up the map for the reads resolution to consume.
    ///
    fn resolve_usages_image_writes(
        accesses: &Vec<ResourceAccess>,
        usages: &mut HashMap<String, ResourceUsage>,
    ) -> Result<(), GraphBuildError> {
        for (pass_index, access) in accesses.iter().enumerate() {
            for image_write in &access.image_writes {
                match usages.entry(image_write.identifier.clone()) {
                    // If the entry already exists it means the value was already written by another
                    // pass. This is an error so we should report the error and exit
                    Entry::Occupied(_) => {
                        return Err(GraphBuildError::MultipleWritesToSameResource);
                    }
                    // Otherwise we insert a new resource object
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
            }
        }
        Ok(())
    }

    fn resolve_usages_image_reads(
        accesses: &Vec<ResourceAccess>,
        usages: &mut HashMap<String, ResourceUsage>,
    ) -> Result<(), GraphBuildError> {
        for (pass_index, access) in accesses.iter().enumerate() {
            for image_read in &access.image_reads {
                if let Some(usage) = usages.get_mut(&image_read.identifier) {
                    if let ResourceUsage::Image(usage) = usage {
                        // Find a read in the same layout
                        let read_type = usage
                            .read_types
                            .iter_mut()
                            .enumerate()
                            .find(|(_, v)| v.layout == image_read.layout);

                        // If there is a read in the same layout, merge the barrier information
                        // otherwise insert a new read type
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
                        return Err(GraphBuildError::IncorrectResourceTypeError);
                    }
                } else {
                    return Err(GraphBuildError::ReadResourceDoesNotExist);
                }
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
                depends_on: Vec::new(),
                waited_on_by: Vec::new(),
            })
            .collect();

        // Because of our SSA form requirement building the links for the graph is really this easy,
        // just iterate over the resources and make anything that reads from a resource depend on
        // the writer and update the writer's link to include the reader.
        usages.iter().for_each(|(_, usage)| {
            usage.read_by().for_each(|read: usize| {
                // Make sure we don't try and write dependencies for PASS_INDEX_EXTERNAL
                if read != PASS_INDEX_EXTERNAL {
                    links[read].depends_on.push(usage.writen_by());
                }
                if usage.writen_by() != PASS_INDEX_EXTERNAL {
                    links[usage.writen_by()].waited_on_by.push(read);
                }
            });
        });
        links
    }
}
