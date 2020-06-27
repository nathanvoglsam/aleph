//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

mod imgui;
mod pipelines;

pub use self::imgui::ImguiRenderer;

use self::pipelines::{GeometryPipeline, TonePipeline};
use std::sync::Arc;
use vulkan::alloc::Allocator;
use vulkan::core::erupt::vk1_0::{
    AccessFlags, AttachmentLoadOp, AttachmentReferenceBuilder, AttachmentStoreOp,
    CommandBufferAllocateInfoBuilder, CommandBufferBeginInfoBuilder, CommandBufferLevel,
    CommandBufferUsageFlags, CommandPoolCreateInfoBuilder, Fence, Format, Framebuffer,
    FramebufferCreateInfoBuilder, ImageLayout, PipelineBindPoint, PipelineStageFlags, RenderPass,
    RenderPassCreateInfoBuilder, SubmitInfoBuilder, SubpassDependencyBuilder,
    SubpassDescriptionBuilder, Vk10DeviceLoaderExt,
};
use vulkan::core::{Device, SwapImage, Swapchain};
use vulkan::embedded::buffers::{CubeMeshBuffers, FullscreenQuadBuffers, SphereMeshBuffers};
use vulkan::image::{ColourImage, DepthImage};
use vulkan::pipeline_layout::PipelineLayout;
use vulkan::shader::ShaderModule;

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
/// A wrapper around a vulkan frame buffer
///
pub struct GBufferFramebuffer {
    framebuffer: Framebuffer,
}

impl GBufferFramebuffer {
    pub unsafe fn new(
        device: &Device,
        colour: &ColourImage,
        depth: &DepthImage,
        swap: &SwapImage,
        render_pass: RenderPass,
    ) -> Self {
        //
        // Assert that all the images are of the same size
        //

        // colour and depth same size
        assert_eq!(
            colour.width(),
            depth.width(),
            "Colour and Depth not same size"
        );
        assert_eq!(
            colour.height(),
            depth.height(),
            "Colour and Depth not same size"
        );

        // colour and swap same size (depth must be same size too as it's same size as colour)
        assert_eq!(
            colour.width(),
            swap.width(),
            "Colour and Swap not same size"
        );
        assert_eq!(
            colour.height(),
            swap.height(),
            "Colour and Swap not same size"
        );

        let attachments = [colour.image_view(), depth.image_view(), swap.image_view()];
        let create_info = FramebufferCreateInfoBuilder::new()
            .width(colour.width())
            .height(colour.height())
            .layers(1)
            .attachments(&attachments)
            .render_pass(render_pass);
        let framebuffer = device
            .loader()
            .create_framebuffer(&create_info, None, None)
            .expect("Failed to create GBuffer framebuffer");

        GBufferFramebuffer { framebuffer }
    }

    ///
    /// Gets the internal framebuffer handle
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
        swap_image: &SwapImage,
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
        let swap_desc = swap_image.attachment_description(
            ImageLayout::UNDEFINED,
            ImageLayout::PRESENT_SRC_KHR,
            AttachmentLoadOp::CLEAR,
            AttachmentStoreOp::STORE,
        );

        //
        // Specify the attachment references for the geometry pass
        //
        let colour_ref = AttachmentReferenceBuilder::new()
            .attachment(0)
            .layout(ImageLayout::COLOR_ATTACHMENT_OPTIMAL);
        let depth_ref = AttachmentReferenceBuilder::new()
            .attachment(1)
            .layout(ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL);

        //
        // Create the geometry subpass
        //
        let geom_color_attachments = [colour_ref];
        let geom_pass = SubpassDescriptionBuilder::new()
            .pipeline_bind_point(PipelineBindPoint::GRAPHICS)
            .color_attachments(&geom_color_attachments)
            .depth_stencil_attachment(&depth_ref);

        //
        // Specify the attachment references for the tonemapping pass
        //
        let colour_tone_ref = [AttachmentReferenceBuilder::new()
            .attachment(0)
            .layout(ImageLayout::SHADER_READ_ONLY_OPTIMAL)];
        let colour_swap_ref = [AttachmentReferenceBuilder::new()
            .attachment(2)
            .layout(ImageLayout::COLOR_ATTACHMENT_OPTIMAL)];

        //
        // Create the tonemapping subpass
        //
        let tone_pass = SubpassDescriptionBuilder::new()
            .pipeline_bind_point(PipelineBindPoint::GRAPHICS)
            .color_attachments(&colour_swap_ref)
            .input_attachments(&colour_tone_ref);
        let tone_dependency = SubpassDependencyBuilder::new()
            .src_subpass(0)
            .dst_subpass(1)
            .src_stage_mask(PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
            .src_access_mask(AccessFlags::COLOR_ATTACHMENT_WRITE)
            .dst_stage_mask(PipelineStageFlags::FRAGMENT_SHADER)
            .dst_access_mask(AccessFlags::INPUT_ATTACHMENT_READ);

        let attachments = [colour_desc, depth_desc, swap_desc];
        let subpasses = [geom_pass, tone_pass];
        let dependencies = [tone_dependency];
        let create_info = RenderPassCreateInfoBuilder::new()
            .attachments(&attachments)
            .subpasses(&subpasses)
            .dependencies(&dependencies);

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
    gbuffer_framebuffer: GBufferFramebuffer,
    geom_frag_module: ShaderModule,
    geom_vert_module: ShaderModule,
    geom_pipe_layout: PipelineLayout,
    geom_pipe: GeometryPipeline,
    tone_frag_module: ShaderModule,
    tone_vert_module: ShaderModule,
    tone_pipe_layout: PipelineLayout,
    tone_pipe: TonePipeline,
    device: Arc<Device>,
    allocator: Arc<Allocator>,
}

impl Renderer {
    ///
    /// Creates a new renderer
    ///
    pub unsafe fn new(
        device: Arc<Device>,
        allocator: Arc<Allocator>,
        swapchain: &Swapchain,
    ) -> Renderer {
        // Create a command pool for creating permanent buffer resources
        let create_info =
            CommandPoolCreateInfoBuilder::new().queue_family_index(device.general_family().index);
        let command_pool = device
            .loader()
            .create_command_pool(&create_info, None, None)
            .expect("Failed to create command pool");

        // Create a command buffer for creating permanent buffer resources
        let allocate_info = CommandBufferAllocateInfoBuilder::new()
            .level(CommandBufferLevel::PRIMARY)
            .command_pool(command_pool)
            .command_buffer_count(1);
        let command_buffer = device
            .loader()
            .allocate_command_buffers(&allocate_info)
            .expect("Failed to allocate command buffer")[0];

        // Beginning recording commands
        let begin_info =
            CommandBufferBeginInfoBuilder::new().flags(CommandBufferUsageFlags::ONE_TIME_SUBMIT);
        device
            .loader()
            .begin_command_buffer(command_buffer, &begin_info)
            .expect("Failed to begin command buffer");

        CubeMeshBuffers::init_buffers(&allocator, command_buffer);
        SphereMeshBuffers::init_buffers(&allocator, command_buffer);
        FullscreenQuadBuffers::init_buffers(&allocator, command_buffer);

        // End recording commands
        device
            .loader()
            .end_command_buffer(command_buffer)
            .expect("Failde to end command buffer");

        // Submit command buffer
        let command_buffers = [command_buffer];
        let submit = SubmitInfoBuilder::new().command_buffers(&command_buffers);
        device
            .loader()
            .queue_submit(device.general_queue(), &[submit], Fence::null())
            .expect("Failed to submit command buffer");

        // Defer freeing the command pool until app shutdown. Easy way to make sure the buffer has
        // finished being used.
        device.defer_destruction(command_pool);

        let swap_image = &swapchain.images()[0];
        let gbuffer = GBuffer::new(&device, &allocator, swap_image.width(), swap_image.height());
        let gbuffer_pass = GBufferPass::new(
            &device,
            gbuffer.colour_image(),
            gbuffer.depth_image(),
            &swap_image,
        );
        let gbuffer_framebuffer = GBufferFramebuffer::new(
            &device,
            gbuffer.colour_image(),
            gbuffer.depth_image(),
            &swap_image,
            gbuffer_pass.render_pass(),
        );
        let (_, words) = vulkan::embedded::data::shaders::standard_frag_shader();
        let geom_frag_module = ShaderModule::builder()
            .reflect(true)
            .compile(true)
            .fragment()
            .words(words)
            .build(Some(&device))
            .expect("Failed to create geom frag module");
        let (_, words) = vulkan::embedded::data::shaders::standard_vert_shader();
        let geom_vert_module = ShaderModule::builder()
            .reflect(true)
            .compile(true)
            .vertex()
            .words(words)
            .build(Some(&device))
            .expect("Failed to create geom vert module");

        let geom_pipe_layout = PipelineLayout::builder()
            .modules(&[&geom_frag_module, &geom_vert_module])
            .build(&device)
            .expect("Failed to create geom pipe layout");

        let geom_pipe = GeometryPipeline::new(
            &device,
            &geom_pipe_layout,
            gbuffer_pass.render_pass(),
            &geom_vert_module,
            &geom_frag_module,
        );

        let (_, words) = vulkan::embedded::data::shaders::tonemapping_frag_shader();
        let tone_frag_module = ShaderModule::builder()
            .reflect(true)
            .compile(true)
            .fragment()
            .words(words)
            .build(Some(&device))
            .expect("Failed to create tone frag module");
        let (_, words) = vulkan::embedded::data::shaders::fullscreen_quad_vert_shader();
        let tone_vert_module = ShaderModule::builder()
            .reflect(true)
            .compile(true)
            .vertex()
            .words(words)
            .build(Some(&device))
            .expect("Failed to create tone vert module");

        let tone_pipe_layout = PipelineLayout::builder()
            .modules(&[&tone_frag_module, &tone_vert_module])
            .build(&device)
            .expect("Failed to create tone pipe layout");

        let tone_pipe = TonePipeline::new(
            &device,
            &tone_pipe_layout,
            gbuffer_pass.render_pass(),
            &tone_vert_module,
            &tone_frag_module,
        );

        Self {
            gbuffer,
            gbuffer_pass,
            gbuffer_framebuffer,
            geom_frag_module,
            geom_vert_module,
            geom_pipe_layout,
            geom_pipe,
            tone_frag_module,
            tone_vert_module,
            tone_pipe_layout,
            tone_pipe,
            device,
            allocator,
        }
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        unsafe {
            self.tone_pipe.destroy(&self.device);
            self.tone_pipe_layout.destroy(&self.device);
            self.tone_vert_module.destroy(&self.device);
            self.tone_frag_module.destroy(&self.device);
            self.geom_pipe.destroy(&self.device);
            self.geom_pipe_layout.destroy(&self.device);
            self.geom_vert_module.destroy(&self.device);
            self.geom_frag_module.destroy(&self.device);
            self.gbuffer_framebuffer.destroy(&self.device);
            self.gbuffer_pass.destroy(&self.device);
            self.gbuffer.destroy(&self.device, &self.allocator);
        }
    }
}
