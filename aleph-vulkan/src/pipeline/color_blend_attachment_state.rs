//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use crate::vulkan_core::erupt::vk1_0::{
    BlendFactor, BlendOp, ColorComponentFlags, PipelineColorBlendAttachmentStateBuilder,
};

///
/// A namespace struct for colour blend attachment state
///
pub struct ColorBlendAttachmentState {}

impl ColorBlendAttachmentState {
    ///
    /// Color blending disabled
    ///
    pub fn disabled() -> PipelineColorBlendAttachmentStateBuilder<'static> {
        PipelineColorBlendAttachmentStateBuilder::new().blend_enable(false)
    }

    ///
    /// Standard alpha blending
    ///
    /// OUT_COLOR = (SRC_C * SRC_A) + (DST_C * (1 - SRC_A))
    /// OUT_ALPHA = (SRC_A * 1) + (DST_A * 0)
    ///
    pub fn alpha_blending() -> PipelineColorBlendAttachmentStateBuilder<'static> {
        PipelineColorBlendAttachmentStateBuilder::new()
            .blend_enable(true)
            .src_color_blend_factor(BlendFactor::SRC_ALPHA)
            .dst_color_blend_factor(BlendFactor::ONE_MINUS_SRC_ALPHA)
            .color_blend_op(BlendOp::ADD)
            .src_alpha_blend_factor(BlendFactor::ONE)
            .dst_alpha_blend_factor(BlendFactor::ZERO)
            .alpha_blend_op(BlendOp::ADD)
            .color_write_mask(Self::write_all())
    }

    fn write_all() -> ColorComponentFlags {
        ColorComponentFlags::R
            | ColorComponentFlags::G
            | ColorComponentFlags::B
            | ColorComponentFlags::A
    }
}
