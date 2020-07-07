//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

#[cfg(test)]
mod tests;

use crate::graph::{RenderGraph, RenderGraphPass, ResourceAccess};
use std::collections::hash_map::Entry;
use std::collections::HashMap;

///
/// Represents the set of errors that could be produced when building a graph
///
#[derive(Clone, Debug, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub enum GraphBuildError {
    MultipleWritesToSameResource,
    ReadResourceDontExist,
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
}

impl<'a> RenderGraphBuilder<'a> {
    ///
    /// Creates a new builder
    ///
    pub fn new() -> Self {
        Self { passes: Vec::new() }
    }

    ///
    /// Add a new pass to the builder
    ///
    pub fn pass(&mut self, pass: impl RenderGraphPass + 'a) -> &mut Self {
        self.passes.push(Box::new(pass));
        self
    }

    ///
    /// Consume the builder and build a render graph
    ///
    pub fn build(mut self) -> Result<RenderGraph, GraphBuildError> {
        // First we need to resolve all the resource access objects from the provided passes into
        // a single list. We have to do this as we will be performing multiple passes over this list
        // to resolve the graph
        let mut accesses: Vec<ResourceAccess> = self
            .passes
            .iter_mut()
            .map(|pass| {
                let mut accesses = ResourceAccess {
                    reads: Vec::new(),
                    writes: Vec::new(),
                };

                pass.register_access(&mut accesses);

                accesses
            })
            .collect();

        // Represents a unique resource
        #[allow(dead_code)]
        struct Resource {
            writen_by: usize,
            read_by: Vec<usize>,
        }

        // In this phase we register all resources that are written throughout the frame. This step
        // will check if any resources are written more than once.
        let mut resources = HashMap::new();
        for (index, access) in accesses.iter_mut().enumerate() {
            for write in access.writes.drain(..) {
                match resources.entry(write) {
                    // If the entry already exists it means the value was already written by another
                    // pass. This is an error so we should report the error and exit
                    Entry::Occupied(_) => {
                        return Err(GraphBuildError::MultipleWritesToSameResource);
                    }
                    // Otherwise we insert a new resource object
                    Entry::Vacant(vacant) => {
                        let resource = Resource {
                            writen_by: index,
                            read_by: Vec::new(),
                        };
                        vacant.insert(resource);
                    }
                }
            }
        }

        // In this phase we register all reads and validate that there aren't any reads to resources
        // that don't exists
        for (index, access) in accesses.iter().enumerate() {
            for read in &access.reads {
                if let Some(resource) = resources.get_mut(read) {
                    resource.read_by.push(index);
                } else {
                    return Err(GraphBuildError::ReadResourceDontExist);
                }
            }
        }

        let graph = RenderGraph {};

        Ok(graph)
    }
}
