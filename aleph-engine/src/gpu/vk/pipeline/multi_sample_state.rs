//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use erupt::vk1_0::{
    PipelineMultisampleStateCreateInfo, PipelineMultisampleStateCreateInfoBuilder,
    SampleCountFlagBits,
};

///
/// Namespace struct for multisampling state
///
pub struct MultiSampleState {}

impl MultiSampleState {
    ///
    /// Multi sampling disabled
    ///
    pub fn disabled() -> PipelineMultisampleStateCreateInfo {
        *PipelineMultisampleStateCreateInfoBuilder::new()
            .rasterization_samples(SampleCountFlagBits::_1)
    }
}
