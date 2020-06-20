//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use crate::gpu::vk::alloc::Allocator;
use crate::gpu::vk::framebuffer::ColourDepthFramebuffer;
use crate::gpu::vk::image::{ColourImage, DepthImage};
use crate::gpu::vk::Device;
use erupt::vk1_0::{
    AttachmentLoadOp, AttachmentReferenceBuilder, AttachmentStoreOp, Format, ImageLayout,
    PipelineBindPoint, RenderPass, RenderPassCreateInfoBuilder, SubpassDescriptionBuilder,
    Vk10DeviceLoaderExt,
};
use std::sync::Arc;

///
/// Represents a single gbuffer
///
pub struct GBuffer {
    base_colour: ColourImage,
    depth_buffer: DepthImage,
}

impl GBuffer {
    pub unsafe fn new(device: &Device, allocator: &Allocator, width: u32, height: u32) -> GBuffer {
        let base_colour = ColourImage::builder()
            .width(width)
            .height(height)
            .format(Format::R16G16B16A16_SFLOAT)
            .usage_input_attachment()
            .build(device, allocator);
        let depth_buffer = DepthImage::builder()
            .width(width)
            .height(height)
            .format(Format::D32_SFLOAT)
            .usage_input_attachment()
            .build(device, allocator);
        Self {
            base_colour,
            depth_buffer,
        }
    }

    ///
    /// Gets the colour image
    ///
    pub fn colour_image(&self) -> &ColourImage {
        &self.base_colour
    }

    ///
    /// Gets the depth image
    ///
    pub fn depth_image(&self) -> &DepthImage {
        &self.depth_buffer
    }

    ///
    /// Free the gbuffer data
    ///
    pub unsafe fn destroy(&self, device: &Device, allocator: &Allocator) {
        self.base_colour.destroy(device, allocator);
        self.depth_buffer.destroy(device, allocator);
    }
}

///
/// Represents the primary rendering gbuffer pass
///
pub struct GBufferPass {
    render_pass: RenderPass,
}

impl GBufferPass {
    ///
    /// Creates a new GBufferPass object
    ///
    pub unsafe fn new(
        device: &Device,
        colour_image: &ColourImage,
        depth_image: &DepthImage,
    ) -> Self {
        //
        // Specify the attachment descriptions for the whole render pass
        //
        let colour_desc = colour_image.attachment_description(
            ImageLayout::UNDEFINED,
            ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
            AttachmentLoadOp::CLEAR,
            AttachmentStoreOp::DONT_CARE,
        );
        let depth_desc = depth_image.attachment_description(
            ImageLayout::UNDEFINED,
            ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
            AttachmentLoadOp::CLEAR,
            AttachmentStoreOp::DONT_CARE,
        );

        //
        // Specify the attachment references for the geometry pass
        //
        let colour_ref = [AttachmentReferenceBuilder::new()
            .attachment(0)
            .layout(ImageLayout::COLOR_ATTACHMENT_OPTIMAL)];
        let depth_ref = AttachmentReferenceBuilder::new()
            .attachment(1)
            .layout(ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL);

        //
        // Create the geometry subpass
        //
        let geom_pass = SubpassDescriptionBuilder::new()
            .pipeline_bind_point(PipelineBindPoint::GRAPHICS)
            .color_attachments(&colour_ref)
            .depth_stencil_attachment(&depth_ref);

        let attachments = [colour_desc, depth_desc];
        let subpasses = [geom_pass];
        let create_info = RenderPassCreateInfoBuilder::new()
            .attachments(&attachments)
            .subpasses(&subpasses);

        let render_pass = device
            .loader()
            .create_render_pass(&create_info, None, None)
            .expect("Failed to create RenderPass");

        Self { render_pass }
    }

    ///
    /// Gets the underlying render pass handle
    ///
    pub fn render_pass(&self) -> RenderPass {
        self.render_pass
    }

    ///
    /// Free the renderpass data
    ///
    pub unsafe fn destroy(&self, device: &Device) {
        device.loader().destroy_render_pass(self.render_pass, None);
    }
}

///
///
///
pub struct Renderer {
    gbuffer: GBuffer,
    gbuffer_pass: GBufferPass,
    gbuffer_framebuffer: ColourDepthFramebuffer,
    device: Arc<Device>,
    allocator: Arc<Allocator>,
}

impl Renderer {
    ///
    /// Creates a new renderer
    ///
    pub unsafe fn new(device: Arc<Device>, allocator: Arc<Allocator>) -> Renderer {
        let gbuffer = GBuffer::new(&device, &allocator, 1024, 1024);
        let gbuffer_pass = GBufferPass::new(&device, gbuffer.colour_image(), gbuffer.depth_image());
        let gbuffer_framebuffer = ColourDepthFramebuffer::new(
            &device,
            gbuffer.colour_image(),
            gbuffer.depth_image(),
            gbuffer_pass.render_pass(),
        );

        Self {
            gbuffer,
            gbuffer_pass,
            gbuffer_framebuffer,
            device,
            allocator,
        }
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        unsafe {
            self.gbuffer_framebuffer.destroy(&self.device);
            self.gbuffer_pass.destroy(&self.device);
            self.gbuffer.destroy(&self.device, &self.allocator);
        }
    }
}
