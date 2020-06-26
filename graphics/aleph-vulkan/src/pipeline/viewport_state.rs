//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use vulkan_core::erupt::vk1_0::{PipelineViewportStateCreateInfoBuilder, Rect2DBuilder, ViewportBuilder};

pub struct ViewportState {}

impl ViewportState {
    pub fn dynamic(
        viewport_count: u32,
        scissor_count: u32,
    ) -> PipelineViewportStateCreateInfoBuilder<'static> {
        PipelineViewportStateCreateInfoBuilder::new()
            .viewport_count(viewport_count)
            .scissor_count(scissor_count)
    }

    pub fn baked<'a>(
        viewports: &'a [ViewportBuilder<'static>],
        scissors: &'a [Rect2DBuilder],
    ) -> PipelineViewportStateCreateInfoBuilder<'a> {
        PipelineViewportStateCreateInfoBuilder::new()
            .viewports(viewports)
            .scissors(scissors)
    }
}
