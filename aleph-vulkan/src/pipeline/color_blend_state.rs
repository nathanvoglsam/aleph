//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use crate::vulkan_core::erupt::vk1_0::{
    PipelineColorBlendAttachmentStateBuilder, PipelineColorBlendStateCreateInfoBuilder,
};

///
/// Namespace struct for color blend state
///
pub struct ColorBlendState {}

impl ColorBlendState {
    ///
    /// Creates a state from the given list of attachments with logic ops disabled and default blend
    /// constants
    ///
    pub fn attachments<'a>(
        attachments: &'a [PipelineColorBlendAttachmentStateBuilder],
    ) -> PipelineColorBlendStateCreateInfoBuilder<'a> {
        PipelineColorBlendStateCreateInfoBuilder::new()
            .attachments(attachments)
            .logic_op_enable(false)
    }
}
