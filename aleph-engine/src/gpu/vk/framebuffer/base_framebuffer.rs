//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use crate::gpu::vk::Device;
use erupt::vk1_0::{
    Framebuffer, FramebufferCreateInfoBuilder, ImageView, RenderPass, Vk10DeviceLoaderExt,
};

///
/// A builder for a base frame buffer
///
pub struct BaseFramebufferBuilder<'a> {
    inner: FramebufferCreateInfoBuilder<'a>,
}

impl<'a> BaseFramebufferBuilder<'a> {
    ///
    /// Create a new builder
    ///
    pub fn new() -> Self {
        Self {
            inner: FramebufferCreateInfoBuilder::new(),
        }
    }

    ///
    /// The width of the framebuffer
    ///
    pub fn width(mut self, width: u32) -> Self {
        self.inner = self.inner.width(width);
        self
    }

    ///
    /// The height of the framebuffer
    ///
    pub fn height(mut self, height: u32) -> Self {
        self.inner = self.inner.height(height);
        self
    }

    ///
    /// The layer count of the framebuffer
    ///
    pub fn layers(mut self, layers: u32) -> Self {
        self.inner = self.inner.layers(layers);
        self
    }

    ///
    /// The render pass of the framebuffer
    ///
    pub fn render_pass(mut self, render_pass: RenderPass) -> Self {
        self.inner = self.inner.render_pass(render_pass);
        self
    }

    ///
    /// The render pass of the framebuffer
    ///
    pub fn attachments(mut self, attachments: &'a [ImageView]) -> Self {
        self.inner = self.inner.attachments(attachments);
        self
    }

    ///
    /// Creates the framebuffer
    ///
    pub unsafe fn build(self, device: &Device) -> BaseFramebuffer {
        let framebuffer = device
            .loader()
            .create_framebuffer(&self.inner, None, None)
            .expect("Failed to create framebuffer");

        BaseFramebuffer { framebuffer }
    }
}

///
/// A wrapper around a vulkan frame buffer
///
pub struct BaseFramebuffer {
    framebuffer: Framebuffer,
}

impl BaseFramebuffer {
    ///
    /// Returns a builder
    ///
    pub fn builder<'a>() -> BaseFramebufferBuilder<'a> {
        BaseFramebufferBuilder::new()
    }

    ///
    /// Get the internal framebuffer handle
    ///
    pub fn framebuffer(&self) -> Framebuffer {
        self.framebuffer
    }

    ///
    /// Destroys the framebuffer
    ///
    pub unsafe fn destroy(&self, device: &Device) {
        device.loader().destroy_framebuffer(self.framebuffer, None);
    }
}
