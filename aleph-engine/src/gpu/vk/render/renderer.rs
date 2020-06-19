//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use crate::gpu::vk::alloc::Allocator;
use crate::gpu::vk::framebuffer::ColorDepthFramebuffer;
use crate::gpu::vk::image::{ColourImage, DepthImage};
use crate::gpu::vk::Device;
use erupt::vk1_0::{Format, RenderPass};
use std::sync::Arc;

///
/// Represents a single gbuffer
///
pub struct GBuffer {
    base_colour: ColourImage,
    depth_buffer: DepthImage,
    framebuffer: ColorDepthFramebuffer,
}

impl GBuffer {
    pub unsafe fn new(
        device: &Device,
        allocator: &Allocator,
        width: u32,
        height: u32,
        render_pass: RenderPass,
    ) -> GBuffer {
        let base_colour = ColourImage::builder()
            .width(width)
            .height(height)
            .format(Format::R8G8B8A8_UNORM)
            .usage_input_attachment()
            .build(device, allocator);
        let depth_buffer = DepthImage::builder()
            .width(width)
            .height(height)
            .format(Format::D32_SFLOAT)
            .usage_input_attachment()
            .build(device, allocator);
        let framebuffer =
            ColorDepthFramebuffer::new(device, &base_colour, &depth_buffer, render_pass);
        Self {
            base_colour,
            depth_buffer,
            framebuffer,
        }
    }

    ///
    /// Free the gbuffer data
    ///
    pub unsafe fn destroy(&self, device: &Device, allocator: &Allocator) {
        self.framebuffer.destroy(device);
        self.base_colour.destroy(device, allocator);
        self.depth_buffer.destroy(device, allocator);
    }
}

///
///
///
pub struct Renderer {
    gbuffer: GBuffer,
    device: Arc<Device>,
    allocator: Arc<Allocator>,
}

impl Renderer {
    ///
    /// Creates a new renderer
    ///
    pub unsafe fn new(device: Arc<Device>, allocator: Arc<Allocator>) -> Renderer {
        let gbuffer = GBuffer::new(&device, &allocator, 1024, 1024, RenderPass::null());

        Self {
            gbuffer,
            device,
            allocator,
        }
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        unsafe {
            self.gbuffer.destroy(&self.device, &self.allocator);
        }
    }
}
