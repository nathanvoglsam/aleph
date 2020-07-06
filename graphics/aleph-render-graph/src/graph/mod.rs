//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

mod closure_pass;

pub use closure_pass::ClosurePass;

use crate::builder::RenderGraphBuilder;
use aleph_vulkan_core::erupt::vk1_0::CommandBuffer;

///
/// Struct passed into `register_access` for describing the resources accessed by the pass
///
pub struct ResourceAccess {
    pub(crate) reads: Vec<String>,
    pub(crate) writes: Vec<String>,
}

// TODO: This is very temporary and is just an initial implementation for building a graph.
//       The real implementation will not use string resource names. We also need to be able to
//       handle creating resources, probably with a more explicit interface to simplify describing
//       a frame
impl ResourceAccess {
    ///
    /// Register that the given resource is read in this pass
    ///
    pub fn read(&mut self, resource: &str) {
        self.reads.push(resource.to_string());
    }

    ///
    /// Register that the given resource is written in this pass
    ///
    pub fn write(&mut self, resource: &str) {
        self.writes.push(resource.to_string());
    }
}

///
/// The trait that specifies the required interface for a render graph node
///
pub trait RenderGraphPass {
    ///
    /// Function called while building the graph for registering resource accesses by this pass.
    ///
    fn register_access(&mut self, accesses: &mut ResourceAccess);

    ///
    /// Record the commands for this pass onto a command buffer. Appropriate barriers will be
    /// already be recorded into the command buffer.
    ///
    fn record(&mut self, command_buffer: CommandBuffer);
}

///
/// A built render graph
///
pub struct RenderGraph {}

impl RenderGraph {
    ///
    /// Returns a new builder instance
    ///
    pub fn builder<'a>() -> RenderGraphBuilder<'a> {
        RenderGraphBuilder::new()
    }
}
