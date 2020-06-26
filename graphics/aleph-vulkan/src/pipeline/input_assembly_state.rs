//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use vulkan_core::erupt::vk1_0::{PipelineInputAssemblyStateCreateInfoBuilder, PrimitiveTopology};

///
/// Namespace struct for input assembly state
///
pub struct InputAssemblyState {}

impl InputAssemblyState {
    ///
    /// Create an input assembly state with the given topology and primitive restart disabled
    ///
    pub fn no_primitive_restart(
        topology: PrimitiveTopology,
    ) -> PipelineInputAssemblyStateCreateInfoBuilder<'static> {
        PipelineInputAssemblyStateCreateInfoBuilder::new()
            .topology(topology)
            .primitive_restart_enable(false)
    }
}
