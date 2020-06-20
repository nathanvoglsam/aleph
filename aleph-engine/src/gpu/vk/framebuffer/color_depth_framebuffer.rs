//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use crate::gpu::vk::framebuffer::BaseFramebuffer;
use crate::gpu::vk::image::{ColourImage, DepthImage};
use crate::gpu::vk::Device;
use erupt::vk1_0::RenderPass;
use std::ops::Deref;

///
/// A wrapper around a vulkan frame buffer
///
pub struct ColourDepthFramebuffer {
    framebuffer: BaseFramebuffer,
}

impl ColourDepthFramebuffer {
    pub unsafe fn new(
        device: &Device,
        colour: &ColourImage,
        depth: &DepthImage,
        render_pass: RenderPass,
    ) -> Self {
        assert_eq!(colour.width(), depth.width());
        assert_eq!(colour.height(), depth.height());

        let attachments = [colour.image_view(), depth.image_view()];
        let framebuffer = BaseFramebuffer::builder()
            .width(colour.width())
            .height(colour.height())
            .layers(1)
            .attachments(&attachments)
            .render_pass(render_pass)
            .build(device);

        ColourDepthFramebuffer { framebuffer }
    }
}

impl Deref for ColourDepthFramebuffer {
    type Target = BaseFramebuffer;

    fn deref(&self) -> &Self::Target {
        &self.framebuffer
    }
}
