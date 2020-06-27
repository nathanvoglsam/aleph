//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use aleph_vulkan_core::erupt::vk1_0::{DynamicState, PipelineDynamicStateCreateInfoBuilder};

///
/// Namespace struct for dynamic pipeline state
///
pub struct DynamicPipelineState {}

impl DynamicPipelineState {
    ///
    /// No dynamic pipeline state
    ///
    pub fn none() -> PipelineDynamicStateCreateInfoBuilder<'static> {
        PipelineDynamicStateCreateInfoBuilder::new()
    }

    ///
    /// Creates a state description with the given list of dynamic states
    ///
    pub fn states(
        states: &[aleph_vulkan_core::erupt::vk1_0::DynamicState],
    ) -> PipelineDynamicStateCreateInfoBuilder {
        PipelineDynamicStateCreateInfoBuilder::new().dynamic_states(states)
    }

    ///
    /// Creates a dynamic state description that specifies dynamic viewport and scissor state
    ///
    pub fn viewport_scissor() -> PipelineDynamicStateCreateInfoBuilder<'static> {
        static STATES: [DynamicState; 2] = [DynamicState::VIEWPORT, DynamicState::SCISSOR];
        PipelineDynamicStateCreateInfoBuilder::new().dynamic_states(&STATES)
    }
}
