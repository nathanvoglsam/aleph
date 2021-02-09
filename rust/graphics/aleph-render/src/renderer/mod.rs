//
//
// This file is a part of Aleph
//
// https://github.com/nathanvoglsam/aleph
//
// MIT License
//
// Copyright (c) 2020 Aleph Engine
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

mod egui;
mod pipelines;

use self::pipelines::{GeometryPipeline, TonePipeline};
use ::egui::PaintJobs;
use aleph_vulkan::embedded::buffers::{CubeMeshBuffers, FullscreenQuadBuffers, SphereMeshBuffers};
use aleph_vulkan::embedded::data::SphereMesh;
use aleph_vulkan::image::{ColourImage, DepthImage};
use aleph_vulkan::pipeline_layout::PipelineLayout;
use aleph_vulkan::reflect::{BindingType, Struct};
use aleph_vulkan::render_pass::AttachmentReference;
use aleph_vulkan::shader::ShaderModule;
use aleph_vulkan::uniform_buffer::Member;
use aleph_vulkan::uniform_buffer::UniformBufferWriter;
use aleph_vulkan_alloc::{Allocation, AllocationCreateInfoBuilder, Allocator, MemoryUsage};
use aleph_vulkan_core::erupt::vk1_0::{
    AccessFlags, AttachmentLoadOp, AttachmentStoreOp, Buffer, BufferCreateInfoBuilder,
    BufferUsageFlags, ClearColorValue, ClearDepthStencilValue, ClearValue, CommandBuffer,
    CommandBufferAllocateInfoBuilder, CommandBufferBeginInfoBuilder, CommandBufferLevel,
    CommandBufferUsageFlags, CommandPool, CommandPoolCreateInfoBuilder,
    DescriptorBufferInfoBuilder, DescriptorImageInfoBuilder, DescriptorPool,
    DescriptorPoolCreateInfoBuilder, DescriptorPoolSizeBuilder, DescriptorSet,
    DescriptorSetAllocateInfoBuilder, DescriptorType, Extent2DBuilder, Fence, Format, Framebuffer,
    FramebufferCreateInfoBuilder, ImageLayout, IndexType, Offset2DBuilder, PipelineBindPoint,
    PipelineStageFlags, Rect2DBuilder, RenderPass, RenderPassBeginInfoBuilder,
    RenderPassCreateInfoBuilder, Semaphore, SemaphoreCreateInfoBuilder, SharingMode,
    SubmitInfoBuilder, SubpassContents, SubpassDependencyBuilder, SubpassDescriptionBuilder,
    WriteDescriptorSetBuilder, SUBPASS_EXTERNAL, WHOLE_SIZE,
};
use aleph_vulkan_core::{DebugName, Device, SwapImage, Swapchain};
use std::sync::Arc;
use std::time::Duration;
use ultraviolet::{Bivec3, Isometry3, Rotor3, Vec3};

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
            .debug_name(concat!(module_path!(), "::GBufferColour"))
            .build(device, allocator);
        let depth_buffer = DepthImage::builder()
            .width(width)
            .height(height)
            .format(Format::D32_SFLOAT)
            .usage_input_attachment()
            .debug_name(concat!(module_path!(), "::GBufferDepth"))
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
        device
            .loader()
            .destroy_framebuffer(Some(self.framebuffer), None);
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
            AttachmentStoreOp::STORE,
        );
        let depth_desc = depth_image.attachment_description(
            ImageLayout::UNDEFINED,
            ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
            AttachmentLoadOp::CLEAR,
            AttachmentStoreOp::DONT_CARE,
        );
        let swap_desc = swap_image.attachment_description(
            ImageLayout::UNDEFINED,
            ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
            AttachmentLoadOp::CLEAR,
            AttachmentStoreOp::STORE,
        );

        //
        // Specify the attachment references for the geometry pass
        //
        let colour_ref = AttachmentReference::color(0);
        let depth_ref = AttachmentReference::depth_stencil(1);

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
        let colour_tone_ref = [AttachmentReference::shader_read_only(0)];
        let colour_swap_ref = [AttachmentReference::color(2)];

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
        let out_dependency = SubpassDependencyBuilder::new()
            .src_subpass(1)
            .dst_subpass(SUBPASS_EXTERNAL)
            .src_stage_mask(PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
            .src_access_mask(AccessFlags::COLOR_ATTACHMENT_WRITE)
            .dst_stage_mask(PipelineStageFlags::TOP_OF_PIPE)
            .dst_access_mask(AccessFlags::MEMORY_READ);

        let attachments = [colour_desc, depth_desc, swap_desc];
        let subpasses = [geom_pass, tone_pass];
        let dependencies = [tone_dependency, out_dependency];
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
        device
            .loader()
            .destroy_render_pass(Some(self.render_pass), None);
    }
}

pub struct UniformBuffers {
    camera_buffer: (Buffer, Allocation),
    model_buffer: (Buffer, Allocation),
}

impl UniformBuffers {
    pub unsafe fn new(allocator: &Allocator, shader_module: &ShaderModule, aspect: f32) -> Self {
        // Find the description of the model and camera uniform buffers
        let mut model = None;
        let mut camera = None;
        shader_module.descriptor_sets().iter().for_each(|v| {
            if model.is_none() {
                model = v.bindings().iter().find(|v| v.name() == "model_buffer");
            }
            if camera.is_none() {
                camera = v.bindings().iter().find(|v| v.name() == "camera_buffer");
            }
        });
        // Ensure we've found a descriptor of the correct type
        let model = model.unwrap();
        let model = if let BindingType::UniformBuffer(v) = model.binding_type() {
            v
        } else {
            panic!()
        };

        // Ensure we've found a descriptor of the correct type
        let camera = camera.unwrap();
        let camera = if let BindingType::UniformBuffer(v) = camera.binding_type() {
            v
        } else {
            panic!()
        };

        // Create the model uniform buffer
        let buffer_create_info = BufferCreateInfoBuilder::new()
            .size(model.size_padded() as _)
            .sharing_mode(SharingMode::EXCLUSIVE)
            .usage(BufferUsageFlags::UNIFORM_BUFFER);
        let alloc_create_info = AllocationCreateInfoBuilder::new().usage(MemoryUsage::CPUToGPU);
        let model_buffer = allocator
            .create_buffer(&buffer_create_info, &alloc_create_info)
            .expect("Failed to allocate model buffer");
        model_buffer.0.add_debug_name(
            allocator.device(),
            aleph_macros::cstr!(concat!(module_path!(), "::UniformBufferModel")),
        );

        // Create the camera uniform buffer
        let buffer_create_info = BufferCreateInfoBuilder::new()
            .size(camera.size_padded() as _)
            .sharing_mode(SharingMode::EXCLUSIVE)
            .usage(BufferUsageFlags::UNIFORM_BUFFER);
        let alloc_create_info = AllocationCreateInfoBuilder::new().usage(MemoryUsage::CPUToGPU);
        let camera_buffer = allocator
            .create_buffer(&buffer_create_info, &alloc_create_info)
            .expect("Failed to allocate model buffer");
        model_buffer.0.add_debug_name(
            allocator.device(),
            aleph_macros::cstr!(concat!(module_path!(), "::UniformBufferCamera")),
        );

        // Write the memory for the uniform buffers
        Self::write_model_buffer(allocator, model, &model_buffer);
        Self::write_camera_buffer(allocator, camera, &camera_buffer, aspect);

        Self {
            camera_buffer,
            model_buffer,
        }
    }

    unsafe fn write_model_buffer(
        allocator: &Allocator,
        layout: &Struct,
        buffer: &(Buffer, Allocation),
    ) {
        let ptr = allocator
            .map_memory(&buffer.1)
            .expect("Failed to map memory");
        let mem = std::slice::from_raw_parts_mut(ptr, layout.size_padded() as _);
        let mut writer = UniformBufferWriter::new_for_struct(layout, mem).unwrap();

        // Calculate the matrices to upload
        let pos = Vec3::zero();

        let angle = 45f32.to_radians();
        let plane = Bivec3::from_normalized_axis(Vec3::unit_y());
        let rot = Rotor3::from_angle_plane(angle, plane);

        let translation = ultraviolet::transform::Isometry3::new(pos, rot);
        let model = translation.into_homogeneous_matrix();
        let normal = model.inversed().transposed();

        // Write the members
        writer
            .write_member("model_matrix", Member::Mat4x4(model))
            .unwrap();
        writer
            .write_member("normal_matrix", Member::Mat4x4(normal))
            .unwrap();

        // Finalize the writer and ensure that all members have been written
        writer.finalize().unwrap();

        // Flush and unmap the buffer
        allocator.flush_allocation(&buffer.1, 0, mem.len() as _);
        allocator.unmap_memory(&buffer.1);
    }

    unsafe fn write_camera_buffer(
        allocator: &Allocator,
        layout: &Struct,
        buffer: &(Buffer, Allocation),
        aspect: f32,
    ) {
        let ptr = allocator
            .map_memory(&buffer.1)
            .expect("Failed to map memory");
        let mem = std::slice::from_raw_parts_mut(ptr, layout.size_padded() as _);
        let mut writer = UniformBufferWriter::new_for_struct(layout, mem).unwrap();

        // Calculate the matrices to upload
        let pos = Vec3::new(0.5, 0.0, -2.0);
        let rot = Rotor3::identity();
        let view = Isometry3::new(pos, rot);
        let view = view.into_homogeneous_matrix();

        let proj =
            ultraviolet::projection::perspective_vk(90.0f32.to_radians(), aspect, 0.1, 100.0);

        // Write the members
        writer
            .write_member("view_matrix", Member::Mat4x4(view))
            .unwrap();
        writer
            .write_member("proj_matrix", Member::Mat4x4(proj))
            .unwrap();
        writer.write_member("position", Member::Vec3(pos)).unwrap();

        // Finalize the writer and ensure that all members have been written
        writer.finalize().unwrap();

        // Flush and unmap the buffer
        allocator.flush_allocation(&buffer.1, 0, mem.len() as _);
        allocator.unmap_memory(&buffer.1);
    }

    ///
    /// Destroys the underlying buffers.
    ///
    /// Unsafe as destruction is un synchronized
    ///
    pub unsafe fn destroy(&self, allocator: &Allocator) {
        allocator.destroy_buffer(self.camera_buffer.0, self.camera_buffer.1);
        allocator.destroy_buffer(self.model_buffer.0, self.model_buffer.1);
    }
}

pub struct GeometrySets {
    geom_set_pool: DescriptorPool,
    camera_set: DescriptorSet,
    model_set: DescriptorSet,
}

impl GeometrySets {
    pub unsafe fn new(
        device: &Device,
        pipeline_layout: &PipelineLayout,
        buffers: &UniformBuffers,
    ) -> Self {
        let pool_sizes = [DescriptorPoolSizeBuilder::new()
            ._type(DescriptorType::UNIFORM_BUFFER)
            .descriptor_count(4)];
        let create_info = DescriptorPoolCreateInfoBuilder::new()
            .max_sets(4)
            .pool_sizes(&pool_sizes);
        let geom_set_pool = device
            .loader()
            .create_descriptor_pool(&create_info, None, None)
            .expect("Failed to create descriptor pool");

        let allocate_info = DescriptorSetAllocateInfoBuilder::new()
            .descriptor_pool(geom_set_pool)
            .set_layouts(pipeline_layout.set_layouts());
        let sets = device
            .loader()
            .allocate_descriptor_sets(&allocate_info)
            .expect("Failed to create descriptor set");
        let camera_set = sets[0];
        let model_set = sets[1];

        // Camera UBO
        let camera_buffer_info = [DescriptorBufferInfoBuilder::new()
            .offset(0)
            .buffer(buffers.camera_buffer.0)
            .range(WHOLE_SIZE)];
        let camera_write = WriteDescriptorSetBuilder::new()
            .descriptor_type(DescriptorType::UNIFORM_BUFFER)
            .buffer_info(&camera_buffer_info)
            .dst_set(camera_set)
            .dst_binding(0);

        // Model UBO
        let model_buffer_info = [DescriptorBufferInfoBuilder::new()
            .offset(0)
            .buffer(buffers.model_buffer.0)
            .range(WHOLE_SIZE)];
        let model_write = WriteDescriptorSetBuilder::new()
            .descriptor_type(DescriptorType::UNIFORM_BUFFER)
            .buffer_info(&model_buffer_info)
            .dst_set(model_set)
            .dst_binding(0);

        let writes = [camera_write, model_write];
        device.loader().update_descriptor_sets(&writes, &[]);

        Self {
            geom_set_pool,
            camera_set,
            model_set,
        }
    }

    pub unsafe fn destroy(&self, device: &Device) {
        device
            .loader()
            .destroy_descriptor_pool(Some(self.geom_set_pool), None);
    }
}

pub struct ToneSets {
    tone_set_pool: DescriptorPool,
    tone_set: DescriptorSet,
}

impl ToneSets {
    pub unsafe fn new(
        device: &Device,
        pipeline_layout: &PipelineLayout,
        gbuffer: &GBuffer,
    ) -> Self {
        let pool_sizes = [DescriptorPoolSizeBuilder::new()
            ._type(DescriptorType::INPUT_ATTACHMENT)
            .descriptor_count(4)];
        let create_info = DescriptorPoolCreateInfoBuilder::new()
            .max_sets(4)
            .pool_sizes(&pool_sizes);
        let tone_set_pool = device
            .loader()
            .create_descriptor_pool(&create_info, None, None)
            .expect("Failed to create descriptor pool");

        let allocate_info = DescriptorSetAllocateInfoBuilder::new()
            .descriptor_pool(tone_set_pool)
            .set_layouts(pipeline_layout.set_layouts());
        let sets = device
            .loader()
            .allocate_descriptor_sets(&allocate_info)
            .expect("Failed to create descriptor set");
        let tone_set = sets[0];

        // Input Attachment
        let input_attachment_info = [DescriptorImageInfoBuilder::new()
            .image_view(gbuffer.base_colour.image_view())
            .image_layout(ImageLayout::SHADER_READ_ONLY_OPTIMAL)];
        let tone_write = WriteDescriptorSetBuilder::new()
            .descriptor_type(DescriptorType::INPUT_ATTACHMENT)
            .image_info(&input_attachment_info)
            .dst_set(tone_set)
            .dst_binding(0);

        let writes = [tone_write];
        device.loader().update_descriptor_sets(&writes, &[]);

        Self {
            tone_set_pool,
            tone_set,
        }
    }

    pub unsafe fn destroy(&self, device: &Device) {
        device
            .loader()
            .destroy_descriptor_pool(Some(self.tone_set_pool), None);
    }
}

///
///
///
pub struct Renderer {
    gbuffer: GBuffer,
    gbuffer_pass: GBufferPass,
    gbuffer_framebuffers: Vec<GBufferFramebuffer>,
    geom_frag_module: ShaderModule,
    geom_vert_module: ShaderModule,
    geom_pipe_layout: PipelineLayout,
    geom_pipe: GeometryPipeline,
    geom_sets: GeometrySets,
    tone_frag_module: ShaderModule,
    tone_vert_module: ShaderModule,
    tone_pipe_layout: PipelineLayout,
    tone_pipe: TonePipeline,
    tone_sets: ToneSets,
    uniform_buffers: UniformBuffers,
    command_pool: CommandPool,
    command_buffer: CommandBuffer,
    acquire_semaphore: Semaphore,
    signal_semaphore: Semaphore,
    egui_renderer: egui::Renderer,
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
        Self::init_global_meshes(&device, &allocator);

        let swap_image = &swapchain.images()[0];
        let gbuffer = GBuffer::new(&device, &allocator, swap_image.width(), swap_image.height());
        let gbuffer_pass = GBufferPass::new(
            &device,
            gbuffer.colour_image(),
            gbuffer.depth_image(),
            swap_image,
        );

        let gbuffer_framebuffers: Vec<GBufferFramebuffer> = swapchain
            .images()
            .iter()
            .map(|v| {
                GBufferFramebuffer::new(
                    &device,
                    gbuffer.colour_image(),
                    gbuffer.depth_image(),
                    &v,
                    gbuffer_pass.render_pass(),
                )
            })
            .collect();

        let (_, words) = aleph_vulkan::embedded::data::shaders::standard_frag_shader();
        let geom_frag_module = ShaderModule::builder()
            .debug_name(aleph_macros::cstr!(concat!(
                module_path!(),
                "::GeomFragModule"
            )))
            .reflect(true)
            .compile(true)
            .fragment()
            .words(words)
            .build(Some(&device))
            .expect("Failed to create geom frag module");
        let (_, words) = aleph_vulkan::embedded::data::shaders::standard_vert_shader();
        let geom_vert_module = ShaderModule::builder()
            .debug_name(aleph_macros::cstr!(concat!(
                module_path!(),
                "::GeomVertModule"
            )))
            .reflect(true)
            .compile(true)
            .vertex()
            .words(words)
            .build(Some(&device))
            .expect("Failed to create geom vert module");

        let geom_pipe_layout = PipelineLayout::builder()
            .debug_name(aleph_macros::cstr!(concat!(
                module_path!(),
                "::GeomPipelineLayout"
            )))
            .modules(&[(&geom_frag_module, None), (&geom_vert_module, None)])
            .build(&device)
            .expect("Failed to create geom pipe layout");

        let geom_pipe = GeometryPipeline::new(
            &device,
            &geom_pipe_layout,
            gbuffer_pass.render_pass(),
            &geom_vert_module,
            &geom_frag_module,
        );

        let (_, words) = aleph_vulkan::embedded::data::shaders::tonemapping_frag_shader();
        let tone_frag_module = ShaderModule::builder()
            .debug_name(aleph_macros::cstr!(concat!(
                module_path!(),
                "::ToneFragModule"
            )))
            .reflect(true)
            .compile(true)
            .fragment()
            .words(words)
            .build(Some(&device))
            .expect("Failed to create tone frag module");
        let (_, words) = aleph_vulkan::embedded::data::shaders::fullscreen_quad_vert_shader();
        let tone_vert_module = ShaderModule::builder()
            .debug_name(aleph_macros::cstr!(concat!(
                module_path!(),
                "::ToneVertModule"
            )))
            .reflect(true)
            .compile(true)
            .vertex()
            .words(words)
            .build(Some(&device))
            .expect("Failed to create tone vert module");

        let tone_pipe_layout = PipelineLayout::builder()
            .debug_name(aleph_macros::cstr!(concat!(
                module_path!(),
                "::TonePipelineLayout"
            )))
            .modules(&[(&tone_frag_module, None), (&tone_vert_module, None)])
            .build(&device)
            .expect("Failed to create tone pipe layout");

        let tone_pipe = TonePipeline::new(
            &device,
            &tone_pipe_layout,
            gbuffer_pass.render_pass(),
            &tone_vert_module,
            &tone_frag_module,
        );

        let aspect = swap_image.width() as f32 / swap_image.height() as f32;
        let uniform_buffers = UniformBuffers::new(&allocator, &geom_vert_module, aspect);

        let geom_sets = GeometrySets::new(&device, &geom_pipe_layout, &uniform_buffers);
        let tone_sets = ToneSets::new(&device, &tone_pipe_layout, &gbuffer);

        let create_info =
            CommandPoolCreateInfoBuilder::new().queue_family_index(device.general_family().index);
        let command_pool = device
            .loader()
            .create_command_pool(&create_info, None, None)
            .expect("Failed to create command pool");

        let allocate_info = CommandBufferAllocateInfoBuilder::new()
            .command_pool(command_pool)
            .command_buffer_count(1)
            .level(CommandBufferLevel::PRIMARY);
        let command_buffer = device
            .loader()
            .allocate_command_buffers(&allocate_info)
            .expect("Failed to allocate command buffer")[0];

        let create_info = SemaphoreCreateInfoBuilder::new();
        let acquire_semaphore = device
            .loader()
            .create_semaphore(&create_info, None, None)
            .expect("Failed to create acquire semaphore");
        device.defer_destruction(acquire_semaphore);
        let signal_semaphore = device
            .loader()
            .create_semaphore(&create_info, None, None)
            .expect("Failed to create barrier semaphore");
        device.defer_destruction(signal_semaphore);

        let egui_renderer = egui::Renderer::new(device.clone(), allocator.clone(), swapchain);

        Self {
            gbuffer,
            gbuffer_pass,
            gbuffer_framebuffers,
            geom_frag_module,
            geom_vert_module,
            geom_pipe_layout,
            geom_pipe,
            geom_sets,
            tone_frag_module,
            tone_vert_module,
            tone_pipe_layout,
            tone_pipe,
            tone_sets,
            uniform_buffers,
            command_pool,
            command_buffer,
            acquire_semaphore,
            signal_semaphore,
            egui_renderer,
            device,
            allocator,
        }
    }

    pub unsafe fn acquire_swap_image(
        &self,
        swapchain: &mut Swapchain,
        drawable_size: (u32, u32),
    ) -> Option<usize> {
        self.device
            .loader()
            .device_wait_idle()
            .expect("Failed to wait on device idle");

        if swapchain.requires_rebuild() {
            let _ = swapchain.rebuild(drawable_size);
            //TODO: renderer.recreate_resources(&swapchain);
        }

        match swapchain.acquire_next(
            Duration::from_millis(10000),
            self.acquire_semaphore,
            Fence::null(),
        ) {
            Ok(v) => Some(v),
            Err(_) => None,
        }
    }

    pub unsafe fn render_frame(
        &mut self,
        index: usize,
        swapchain: &mut Swapchain,
        egui_ctx: &::egui::CtxRef,
        jobs: PaintJobs,
    ) {
        self.egui_renderer
            .update_screen_info(egui_ctx.pixels_per_point());

        self.device
            .loader()
            .reset_command_pool(self.command_pool, None)
            .expect("Failed to reset command pool");

        //
        // Begin command buffer recording
        //
        let begin_info =
            CommandBufferBeginInfoBuilder::new().flags(CommandBufferUsageFlags::ONE_TIME_SUBMIT);
        self.device
            .loader()
            .begin_command_buffer(self.command_buffer, &begin_info)
            .expect("Failed to begin command buffer");

        self.record_frame(index, self.command_buffer);

        self.egui_renderer
            .render_frame(index, self.command_buffer, egui_ctx, jobs);

        self.device
            .loader()
            .end_command_buffer(self.command_buffer)
            .expect("Failed to end command buffer");

        {
            let command_buffers = [self.command_buffer];
            let wait_semaphores = [self.acquire_semaphore];
            let signal_semaphores = [self.signal_semaphore];
            let wait_dst_stage_mask = [PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
            let submit = SubmitInfoBuilder::new()
                .command_buffers(&command_buffers)
                .wait_semaphores(&wait_semaphores)
                .signal_semaphores(&signal_semaphores)
                .wait_dst_stage_mask(&wait_dst_stage_mask);
            self.device
                .loader()
                .queue_submit(self.device.general_queue(), &[submit], None)
                .expect("Failed to submit to queue");

            swapchain.present(self.device.general_queue(), index, &[self.signal_semaphore]);
        }
    }

    pub unsafe fn record_frame(&self, index: usize, command_buffer: CommandBuffer) {
        // Build render area over entire image
        let base_colour = &self.gbuffer.base_colour;
        let offset = Offset2DBuilder::new().x(0).y(0);
        let extent = Extent2DBuilder::new()
            .width(base_colour.width())
            .height(base_colour.height());
        let render_area = Rect2DBuilder::new().extent(*extent).offset(*offset);

        // Create the clear values
        let colour_clear = ClearValue {
            color: ClearColorValue {
                uint32: [0, 0, 0, 0],
            },
        };
        let depth_clear = ClearValue {
            depth_stencil: ClearDepthStencilValue {
                depth: 1.0,
                stencil: 0,
            },
        };
        let swap_clear = ClearValue {
            color: ClearColorValue {
                uint32: [0, 0, 0, 0],
            },
        };

        // RenderPass begin for the gbuffer pass on the given framebuffer index
        let clear_values = [colour_clear, depth_clear, swap_clear];
        let render_pass_begin = RenderPassBeginInfoBuilder::new()
            .render_pass(self.gbuffer_pass.render_pass())
            .framebuffer(self.gbuffer_framebuffers[index].framebuffer())
            .render_area(*render_area)
            .clear_values(&clear_values);

        self.device.loader().cmd_begin_render_pass(
            command_buffer,
            &render_pass_begin,
            SubpassContents::INLINE,
        );

        // =========================================================================================
        // GEOMETRY SUBPASS
        // =========================================================================================

        // Bind the geometry pipeline
        self.device.loader().cmd_bind_pipeline(
            command_buffer,
            PipelineBindPoint::GRAPHICS,
            self.geom_pipe.pipeline(),
        );

        self.device.loader().cmd_set_viewport(
            command_buffer,
            0,
            &[self.gbuffer.base_colour.get_viewport_full()],
        );

        self.device.loader().cmd_set_scissor(
            command_buffer,
            0,
            &[self.gbuffer.base_colour.get_scissor_full()],
        );

        // Bind the descriptors
        let descriptor_sets = [self.geom_sets.camera_set, self.geom_sets.model_set];
        self.device.loader().cmd_bind_descriptor_sets(
            command_buffer,
            PipelineBindPoint::GRAPHICS,
            self.geom_pipe_layout.pipeline_layout(),
            0,
            &descriptor_sets,
            &[],
        );

        // Bind all the vertex attributes for a static mesh
        let buffers = [
            SphereMeshBuffers::positions(),
            SphereMeshBuffers::normals(),
            SphereMeshBuffers::tangents(),
            SphereMeshBuffers::uvs(),
        ];
        let offsets = [0, 0, 0, 0];
        self.device
            .loader()
            .cmd_bind_vertex_buffers(command_buffer, 0, &buffers, &offsets);

        self.device.loader().cmd_bind_index_buffer(
            command_buffer,
            SphereMeshBuffers::indices(),
            0,
            IndexType::UINT16,
        );
        self.device.loader().cmd_draw_indexed(
            command_buffer,
            SphereMesh::indices().len() as _,
            1,
            0,
            0,
            0,
        );

        // =========================================================================================
        // TONEMAPPING SUBPASS
        // =========================================================================================
        self.device
            .loader()
            .cmd_next_subpass(command_buffer, SubpassContents::INLINE);

        // Bind the tonemapping pipeline
        self.device.loader().cmd_bind_pipeline(
            command_buffer,
            PipelineBindPoint::GRAPHICS,
            self.tone_pipe.pipeline(),
        );

        self.device.loader().cmd_set_viewport(
            command_buffer,
            0,
            &[self.gbuffer.base_colour.get_viewport_full()],
        );

        self.device.loader().cmd_set_scissor(
            command_buffer,
            0,
            &[self.gbuffer.base_colour.get_scissor_full()],
        );

        // Bind the descriptors
        let descriptor_sets = [self.tone_sets.tone_set];
        self.device.loader().cmd_bind_descriptor_sets(
            command_buffer,
            PipelineBindPoint::GRAPHICS,
            self.tone_pipe_layout.pipeline_layout(),
            0,
            &descriptor_sets,
            &[],
        );

        // Bind the FS quad vertex buffer
        let buffers = [FullscreenQuadBuffers::positions()];
        let offsets = [0];
        self.device
            .loader()
            .cmd_bind_vertex_buffers(command_buffer, 0, &buffers, &offsets);

        // Bind the FS quad index buffer
        self.device.loader().cmd_bind_index_buffer(
            command_buffer,
            FullscreenQuadBuffers::indices(),
            0,
            IndexType::UINT16,
        );

        // Draw a single full screen quad
        self.device
            .loader()
            .cmd_draw_indexed(command_buffer, 6, 1, 0, 0, 0);

        // End the render pass
        self.device.loader().cmd_end_render_pass(command_buffer);
    }

    ///
    /// Internal function for initializing global mesh data
    ///
    unsafe fn init_global_meshes(device: &Device, allocator: &Allocator) {
        // Create a command pool for creating permanent buffer resources
        let create_info =
            CommandPoolCreateInfoBuilder::new().queue_family_index(device.general_family().index);
        let command_pool = device
            .loader()
            .create_command_pool(&create_info, None, None)
            .expect("Failed to create command pool");
        command_pool.add_debug_name(
            &device,
            aleph_macros::cstr!(concat!(module_path!(), "::InitGlobalMeshes::CommandPool")),
        );

        // Create a command buffer for creating permanent buffer resources
        let allocate_info = CommandBufferAllocateInfoBuilder::new()
            .level(CommandBufferLevel::PRIMARY)
            .command_pool(command_pool)
            .command_buffer_count(1);
        let command_buffer = device
            .loader()
            .allocate_command_buffers(&allocate_info)
            .expect("Failed to allocate command buffer")[0];
        command_buffer.add_debug_name(
            &device,
            aleph_macros::cstr!(concat!(module_path!(), "::InitGlobalMeshes::CommandBuffer")),
        );

        // Beginning recording commands
        let begin_info =
            CommandBufferBeginInfoBuilder::new().flags(CommandBufferUsageFlags::ONE_TIME_SUBMIT);
        device
            .loader()
            .begin_command_buffer(command_buffer, &begin_info)
            .expect("Failed to begin command buffer");

        CubeMeshBuffers::init_buffers(allocator, command_buffer);
        SphereMeshBuffers::init_buffers(allocator, command_buffer);
        FullscreenQuadBuffers::init_buffers(allocator, command_buffer);

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
            .queue_submit(device.general_queue(), &[submit], None)
            .expect("Failed to submit command buffer");

        // Defer freeing the command pool until app shutdown. Easy way to make sure the buffer has
        // finished being used.
        device.defer_destruction(command_pool);
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
            self.gbuffer_framebuffers
                .iter()
                .for_each(|v| v.destroy(&self.device));
            self.gbuffer_pass.destroy(&self.device);
            self.gbuffer.destroy(&self.device, &self.allocator);
            self.geom_sets.destroy(&self.device);
            self.tone_sets.destroy(&self.device);
            self.uniform_buffers.destroy(&self.allocator);
            self.device
                .loader()
                .destroy_command_pool(Some(self.command_pool), None);
        }
    }
}
