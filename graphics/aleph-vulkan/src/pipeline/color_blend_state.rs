//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use aleph_vulkan_core::erupt::vk1_0::{
    BlendFactor, BlendOp, ColorComponentFlagBits, PipelineColorBlendAttachmentState,
    PipelineColorBlendAttachmentStateBuilder, PipelineColorBlendStateCreateInfoBuilder, FALSE,
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

    ///
    /// Creates a state description for disabling blending on all attachments, with num specifying
    /// the number of attachments.
    ///
    /// This supports a `num` of up to 16, any more will trigger a panic as we have a static
    /// array we slice from for handing the `PipelineColorBlendAttachmentStateBuilder` instance
    ///
    pub fn disabled(num: usize) -> PipelineColorBlendStateCreateInfoBuilder<'static> {
        static DESCRIPTIONS: [PipelineColorBlendAttachmentState; 16] = [blend_disabled(); 16];

        let descriptions: &[PipelineColorBlendAttachmentStateBuilder<'static>; 16] =
            unsafe { std::mem::transmute(&DESCRIPTIONS) };

        PipelineColorBlendStateCreateInfoBuilder::new().attachments(&descriptions[0..num])
    }
}

///
/// Internal const function for producing a `PipelineColorBlendAttachmentState` object at compile
/// time
///
const fn blend_disabled() -> PipelineColorBlendAttachmentState {
    PipelineColorBlendAttachmentState {
        blend_enable: FALSE,
        src_color_blend_factor: BlendFactor::ONE,
        dst_color_blend_factor: BlendFactor::ZERO,
        color_blend_op: BlendOp::ADD,
        src_alpha_blend_factor: BlendFactor::ONE,
        dst_alpha_blend_factor: BlendFactor::ZERO,
        alpha_blend_op: BlendOp::ADD,
        color_write_mask: ColorComponentFlagBits(0b1111).bitmask(),
    }
}
