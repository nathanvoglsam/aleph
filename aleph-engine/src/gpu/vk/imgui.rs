//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use crate::cstr;
use crate::gpu::vk;
use crate::gpu::vk::alloc::{
    Allocation, AllocationCreateFlag, AllocationCreateInfoBuilder, Allocator, MemoryUsage,
};
use crate::include_bytes_aligned_as;
use erupt::vk1_0::{
    AccessFlagBits, AccessFlags, AttachmentDescriptionBuilder, AttachmentLoadOp,
    AttachmentReferenceBuilder, AttachmentStoreOp, BlendFactor, BlendOp, Buffer,
    BufferCreateInfoBuilder, BufferImageCopyBuilder, BufferUsageFlags, ClearColorValue, ClearValue,
    ColorComponentFlags, CommandBuffer, CommandBufferAllocateInfoBuilder,
    CommandBufferBeginInfoBuilder, CommandBufferLevel, CommandBufferUsageFlags, CommandPool,
    CommandPoolCreateFlags, CommandPoolCreateInfoBuilder, ComponentMappingBuilder,
    ComponentSwizzle, CullModeFlags, DependencyFlagBits, DependencyFlags,
    DescriptorImageInfoBuilder, DescriptorPool, DescriptorPoolCreateFlags,
    DescriptorPoolCreateInfoBuilder, DescriptorPoolSizeBuilder, DescriptorSet,
    DescriptorSetAllocateInfoBuilder, DescriptorSetLayout, DescriptorSetLayoutBindingBuilder,
    DescriptorSetLayoutCreateInfoBuilder, DescriptorType, DynamicState, Extent2D, Extent3D, Fence,
    Filter, Format, Framebuffer, FramebufferCreateInfoBuilder, FrontFace,
    GraphicsPipelineCreateInfoBuilder, Image, ImageAspectFlags, ImageCreateInfoBuilder,
    ImageLayout, ImageMemoryBarrierBuilder, ImageSubresourceLayersBuilder,
    ImageSubresourceRangeBuilder, ImageTiling, ImageType, ImageUsageFlags, ImageView,
    ImageViewCreateInfoBuilder, ImageViewType, IndexType, Offset2D, Pipeline, PipelineBindPoint,
    PipelineCache, PipelineColorBlendAttachmentStateBuilder,
    PipelineColorBlendStateCreateInfoBuilder, PipelineDepthStencilStateCreateInfoBuilder,
    PipelineDynamicStateCreateInfoBuilder, PipelineInputAssemblyStateCreateInfoBuilder,
    PipelineLayout, PipelineLayoutCreateInfoBuilder, PipelineMultisampleStateCreateInfoBuilder,
    PipelineRasterizationStateCreateInfoBuilder, PipelineShaderStageCreateInfoBuilder,
    PipelineStageFlags, PipelineVertexInputStateCreateInfoBuilder,
    PipelineViewportStateCreateInfoBuilder, PolygonMode, PrimitiveTopology,
    PushConstantRangeBuilder, Rect2DBuilder, RenderPass, RenderPassBeginInfoBuilder,
    RenderPassCreateInfoBuilder, SampleCountFlagBits, Sampler, SamplerAddressMode,
    SamplerCreateInfoBuilder, SamplerMipmapMode, Semaphore, ShaderModule,
    ShaderModuleCreateInfoBuilder, ShaderStageFlagBits, ShaderStageFlags, SharingMode,
    SubmitInfoBuilder, SubpassContents, SubpassDependencyBuilder, SubpassDescriptionBuilder,
    VertexInputAttributeDescriptionBuilder, VertexInputBindingDescriptionBuilder, VertexInputRate,
    ViewportBuilder, Vk10DeviceLoaderExt, WriteDescriptorSetBuilder, SUBPASS_EXTERNAL, WHOLE_SIZE,
};
use imgui::{DrawCmd, FontConfig, FontSource};
use std::default::Default;
use std::sync::Arc;

///
/// This represents the resources needed for rendering a single imgui frame in parallel. This
/// separation allows for multiple frames in flight
///
struct ImguiFrame {
    command_pool: CommandPool,
    command_buffer: CommandBuffer,
    image_view: ImageView,
    framebuffer: Framebuffer,
    vtx_buffer: (Buffer, Allocation),
    idx_buffer: (Buffer, Allocation),
    memory_pool: Arc<vk::alloc::Pool>,
}

impl ImguiFrame {
    fn init(
        device: &vk::Device,
        allocator: &Arc<Allocator>,
        swapchain: &vk::Swapchain,
        index: usize,
        render_pass: RenderPass,
    ) -> Self {
        let command_pool = Self::create_command_pool(device);
        let command_buffer = Self::allocate_command_buffer(device, command_pool);
        let image_view =
            Self::create_image_view(device, swapchain.images()[index], swapchain.format().format);
        let framebuffer = Self::create_framebuffer(device, swapchain, render_pass, image_view);

        let memory_type_index = unsafe {
            let buffer_create_info = BufferCreateInfoBuilder::new()
                .usage(BufferUsageFlags::VERTEX_BUFFER | BufferUsageFlags::INDEX_BUFFER)
                .sharing_mode(SharingMode::EXCLUSIVE)
                .size(1024 * 1024);
            let allocation_create_info = AllocationCreateInfoBuilder::new()
                .usage(MemoryUsage::CPUOnly)
                .build();
            allocator.find_memory_type_index_for_buffer_info(
                &buffer_create_info,
                &allocation_create_info,
            )
        }
        .expect("Failed to find memory index");

        let memory_pool = unsafe {
            vk::alloc::PoolBuilder::new()
                .flags(vk::alloc::PoolCreateFlag::LINEAR_ALGORITHM_BIT)
                .block_size((1024 * 1024) * 1)
                .memory_type_index(memory_type_index)
                .min_block_count(1)
                .max_block_count(1)
                .frame_in_use_count(2)
                .build(allocator)
        }
        .expect("Failed to create memory pool");

        ImguiFrame {
            command_pool,
            command_buffer,
            image_view,
            framebuffer,
            vtx_buffer: (Buffer::null(), Allocation::null()),
            idx_buffer: (Buffer::null(), Allocation::null()),
            memory_pool,
        }
    }

    fn create_command_pool(device: &vk::Device) -> CommandPool {
        let create_info = CommandPoolCreateInfoBuilder::new()
            .queue_family_index(device.general_family().index)
            .flags(CommandPoolCreateFlags::RESET_COMMAND_BUFFER);
        unsafe {
            device
                .loader()
                .create_command_pool(&create_info, None, None)
        }
        .expect("Failed to create command pool")
    }

    fn allocate_command_buffer(device: &vk::Device, command_pool: CommandPool) -> CommandBuffer {
        let allocate_info = CommandBufferAllocateInfoBuilder::new()
            .command_pool(command_pool)
            .level(CommandBufferLevel::PRIMARY)
            .command_buffer_count(1);
        unsafe { device.loader().allocate_command_buffers(&allocate_info) }
            .expect("Failed to create command buffer")[0]
    }

    fn create_image_view(device: &vk::Device, image: Image, format: Format) -> ImageView {
        let components = ComponentMappingBuilder::new()
            .r(ComponentSwizzle::R)
            .g(ComponentSwizzle::G)
            .b(ComponentSwizzle::B)
            .a(ComponentSwizzle::A);
        let subresource_range = ImageSubresourceRangeBuilder::new()
            .level_count(1)
            .layer_count(1)
            .base_mip_level(0)
            .base_array_layer(0)
            .aspect_mask(ImageAspectFlags::COLOR);
        let create_info = ImageViewCreateInfoBuilder::new()
            .format(format)
            .image(image)
            .subresource_range(*subresource_range)
            .components(*components)
            .view_type(ImageViewType::_2D);
        unsafe { device.loader().create_image_view(&create_info, None, None) }
            .expect("Failed to create image view")
    }

    fn create_framebuffer(
        device: &vk::Device,
        swapchain: &vk::Swapchain,
        render_pass: RenderPass,
        image_view: ImageView,
    ) -> Framebuffer {
        let attachments = [image_view];
        let create_info = FramebufferCreateInfoBuilder::new()
            .render_pass(render_pass)
            .width(swapchain.extents().width)
            .height(swapchain.extents().height)
            .attachments(&attachments)
            .layers(1);
        unsafe { device.loader().create_framebuffer(&create_info, None, None) }
            .expect("Failed to create framebuffer")
    }

    unsafe fn destroy(&self, device: &vk::Device, allocator: &Allocator) {
        if self.vtx_buffer.0 != Buffer::null() {
            allocator.destroy_buffer(self.vtx_buffer.0, self.vtx_buffer.1);
        }
        if self.idx_buffer.0 != Buffer::null() {
            allocator.destroy_buffer(self.idx_buffer.0, self.idx_buffer.1);
        }
        device.loader().destroy_framebuffer(self.framebuffer, None);
        device.loader().destroy_image_view(self.image_view, None);
        device
            .loader()
            .free_command_buffers(self.command_pool, &[self.command_buffer]);
        device
            .loader()
            .destroy_command_pool(self.command_pool, None);
    }
}

///
/// This represents resources where only one is needed, but they need to be recreated when the
/// swapchain is rebuilt
///
struct ImguiSingular {
    render_pass: RenderPass,
    pipeline_layout: PipelineLayout,
    pipeline: Pipeline,
}

impl ImguiSingular {
    fn init(device: &vk::Device, global: &ImguiGlobal, format: Format) -> Self {
        let render_pass = Self::create_render_pass(device, format);
        let pipeline_layout = Self::create_pipeline_layout(device, global.descriptor_set_layout);
        let pipeline = Self::create_pipeline(
            device,
            pipeline_layout,
            render_pass,
            global.vertex_module,
            global.fragment_module,
        );

        ImguiSingular {
            render_pass,
            pipeline_layout,
            pipeline,
        }
    }

    fn create_pipeline_layout(device: &vk::Device, layout: DescriptorSetLayout) -> PipelineLayout {
        let set_layouts = [layout];
        let ranges = [PushConstantRangeBuilder::new()
            .stage_flags(ShaderStageFlags::VERTEX)
            .offset(0)
            .size(4 * 4)];
        let create_info = PipelineLayoutCreateInfoBuilder::new()
            .set_layouts(&set_layouts)
            .push_constant_ranges(&ranges);
        unsafe {
            device
                .loader()
                .create_pipeline_layout(&create_info, None, None)
        }
        .expect("Failed to create pipeline layout")
    }

    fn create_render_pass(device: &vk::Device, format: Format) -> RenderPass {
        let attachment = AttachmentDescriptionBuilder::new()
            .format(format)
            .samples(SampleCountFlagBits::_1)
            .load_op(AttachmentLoadOp::CLEAR)
            .store_op(AttachmentStoreOp::STORE)
            .stencil_load_op(AttachmentLoadOp::DONT_CARE)
            .stencil_store_op(AttachmentStoreOp::DONT_CARE)
            .initial_layout(ImageLayout::UNDEFINED)
            .final_layout(ImageLayout::PRESENT_SRC_KHR);

        let attachment_reference = AttachmentReferenceBuilder::new()
            .attachment(0)
            .layout(ImageLayout::COLOR_ATTACHMENT_OPTIMAL);
        let color_attachments = [attachment_reference];
        let subpass = SubpassDescriptionBuilder::new()
            .pipeline_bind_point(PipelineBindPoint::GRAPHICS)
            .color_attachments(&color_attachments);

        let dependency = SubpassDependencyBuilder::new()
            .src_subpass(SUBPASS_EXTERNAL)
            .dst_subpass(0)
            .src_stage_mask(PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
            .dst_stage_mask(PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
            .src_access_mask(AccessFlagBits(0).bitmask())
            .dst_access_mask(AccessFlags::COLOR_ATTACHMENT_WRITE);

        let attachments = [attachment];
        let subpasses = [subpass];
        let dependencies = [dependency];
        let create_info = RenderPassCreateInfoBuilder::new()
            .attachments(&attachments)
            .subpasses(&subpasses)
            .dependencies(&dependencies);
        unsafe { device.loader().create_render_pass(&create_info, None, None) }
            .expect("Failed to create render pass")
    }

    fn create_pipeline(
        device: &vk::Device,
        pipeline_layout: PipelineLayout,
        render_pass: RenderPass,
        vertex_module: ShaderModule,
        fragment_module: ShaderModule,
    ) -> Pipeline {
        let vertex_stage = PipelineShaderStageCreateInfoBuilder::new()
            .module(vertex_module)
            .stage(ShaderStageFlagBits::VERTEX)
            .name(cstr!("main"));
        let fragment_stage = PipelineShaderStageCreateInfoBuilder::new()
            .module(fragment_module)
            .stage(ShaderStageFlagBits::FRAGMENT)
            .name(cstr!("main"));
        let stages = [vertex_stage, fragment_stage];

        let binding = VertexInputBindingDescriptionBuilder::new()
            .binding(0)
            .input_rate(VertexInputRate::VERTEX)
            .stride(core::mem::size_of::<imgui::DrawVert>() as u32);
        let pos_attr = VertexInputAttributeDescriptionBuilder::new()
            .binding(0)
            .offset(0)
            .location(0)
            .format(Format::R32G32_SFLOAT);
        let uv_attr = VertexInputAttributeDescriptionBuilder::new()
            .binding(0)
            .offset(8)
            .location(1)
            .format(Format::R32G32_SFLOAT);
        let col_attr = VertexInputAttributeDescriptionBuilder::new()
            .binding(0)
            .offset(16)
            .location(2)
            .format(Format::R8G8B8A8_UNORM);
        let bindings = [binding];
        let attributes = [pos_attr, uv_attr, col_attr];
        let vertex_input = PipelineVertexInputStateCreateInfoBuilder::new()
            .vertex_binding_descriptions(&bindings)
            .vertex_attribute_descriptions(&attributes);

        let input_assembly = PipelineInputAssemblyStateCreateInfoBuilder::new()
            .topology(PrimitiveTopology::TRIANGLE_LIST);

        let viewport = PipelineViewportStateCreateInfoBuilder::new()
            .viewport_count(1)
            .scissor_count(1);

        let rasterization = PipelineRasterizationStateCreateInfoBuilder::new()
            .polygon_mode(PolygonMode::FILL)
            .cull_mode(CullModeFlags::NONE)
            .front_face(FrontFace::COUNTER_CLOCKWISE)
            .line_width(1.0);

        let multisample = PipelineMultisampleStateCreateInfoBuilder::new()
            .rasterization_samples(SampleCountFlagBits::_1);

        let depth_stencil = PipelineDepthStencilStateCreateInfoBuilder::new()
            .depth_test_enable(false)
            .depth_write_enable(false);

        let color_blend = PipelineColorBlendAttachmentStateBuilder::new()
            .blend_enable(true)
            .src_color_blend_factor(BlendFactor::SRC_ALPHA)
            .dst_color_blend_factor(BlendFactor::ONE_MINUS_SRC_ALPHA)
            .color_blend_op(BlendOp::ADD)
            .src_alpha_blend_factor(BlendFactor::ONE_MINUS_SRC_ALPHA)
            .dst_alpha_blend_factor(BlendFactor::ZERO)
            .alpha_blend_op(BlendOp::ADD)
            .color_write_mask(
                ColorComponentFlags::R
                    | ColorComponentFlags::G
                    | ColorComponentFlags::B
                    | ColorComponentFlags::A,
            );
        let attachments = [color_blend];
        let color_blend = PipelineColorBlendStateCreateInfoBuilder::new().attachments(&attachments);

        let dynamic_states = [DynamicState::VIEWPORT, DynamicState::SCISSOR];
        let dynamic = PipelineDynamicStateCreateInfoBuilder::new().dynamic_states(&dynamic_states);

        let create_info = GraphicsPipelineCreateInfoBuilder::new()
            .layout(pipeline_layout)
            .render_pass(render_pass)
            .subpass(0)
            .stages(&stages)
            .vertex_input_state(&vertex_input)
            .input_assembly_state(&input_assembly)
            .viewport_state(&viewport)
            .rasterization_state(&rasterization)
            .multisample_state(&multisample)
            .depth_stencil_state(&depth_stencil)
            .color_blend_state(&color_blend)
            .dynamic_state(&dynamic);
        unsafe {
            device
                .loader()
                .create_graphics_pipelines(PipelineCache::null(), &[create_info], None)
        }
        .expect("Failed to create pipeline")[0]
    }

    unsafe fn destroy(&self, device: &vk::Device) {
        device.loader().destroy_render_pass(self.render_pass, None);
        device.loader().destroy_pipeline(self.pipeline, None);
        device
            .loader()
            .destroy_pipeline_layout(self.pipeline_layout, None);
    }
}

struct ImguiFont {
    sampler: Sampler,
    allocation: Allocation,
    dimensions: (u32, u32),
    image: Image,
    image_view: ImageView,
}

impl ImguiFont {
    fn init(
        mut fonts: imgui::FontAtlasRefMut,
        global: &ImguiGlobal,
        device: &vk::Device,
        allocator: &Allocator,
    ) -> Self {
        fonts.clear_fonts();

        let mut config = FontConfig::default();
        config.name = Some("Cascadia Code 16pt".to_owned());
        let sources = [FontSource::TtfData {
            data: include_bytes!("../../../fonts/CascadiaCode.ttf"),
            size_pixels: 16.0,
            config: Some(config),
        }];
        fonts.add_font(&sources);

        let mut config = FontConfig::default();
        config.name = Some("Cascadia Code 20pt".to_owned());
        let sources = [FontSource::TtfData {
            data: include_bytes!("../../../fonts/CascadiaCode.ttf"),
            size_pixels: 20.0,
            config: Some(config),
        }];
        fonts.add_font(&sources);

        let mut config = FontConfig::default();
        config.name = Some("Cascadia Code 24pt".to_owned());
        let sources = [FontSource::TtfData {
            data: include_bytes!("../../../fonts/CascadiaCode.ttf"),
            size_pixels: 24.0,
            config: Some(config),
        }];
        fonts.add_font(&sources);

        let mut config = FontConfig::default();
        config.name = Some("Cascadia Code 36pt".to_owned());
        let sources = [FontSource::TtfData {
            data: include_bytes!("../../../fonts/CascadiaCode.ttf"),
            size_pixels: 36.0,
            config: Some(config),
        }];
        fonts.add_font(&sources);

        let data = fonts.build_rgba32_texture();
        let dimensions = (data.width, data.height);
        let data = data.data;
        let sampler = Self::create_sampler(device);
        let (image, allocation) = Self::create_image(allocator, dimensions);
        let image_view = Self::create_image_view(device, image);

        unsafe {
            let info = [DescriptorImageInfoBuilder::new()
                .image_view(image_view)
                .sampler(sampler)
                .image_layout(ImageLayout::SHADER_READ_ONLY_OPTIMAL)];
            let write = WriteDescriptorSetBuilder::new()
                .descriptor_type(DescriptorType::COMBINED_IMAGE_SAMPLER)
                .dst_set(global.descriptor_set)
                .image_info(&info);
            device.loader().update_descriptor_sets(&[write], &[])
        }

        let font = ImguiFont {
            sampler,
            allocation,
            dimensions,
            image,
            image_view,
        };

        font.upload_font(device, allocator, data);

        font
    }

    fn create_sampler(device: &vk::Device) -> Sampler {
        let create_info = SamplerCreateInfoBuilder::new()
            .address_mode_u(SamplerAddressMode::REPEAT)
            .address_mode_v(SamplerAddressMode::REPEAT)
            .address_mode_w(SamplerAddressMode::REPEAT)
            .anisotropy_enable(false)
            .min_filter(Filter::LINEAR)
            .mag_filter(Filter::LINEAR)
            .mipmap_mode(SamplerMipmapMode::LINEAR)
            .min_lod(-1000.0)
            .max_lod(1000.0);
        unsafe { device.loader().create_sampler(&create_info, None, None) }
            .expect("Failed to create sampler")
    }

    fn create_image(allocator: &Allocator, dimensions: (u32, u32)) -> (Image, Allocation) {
        let mut extent = Extent3D::default();
        extent.width = dimensions.0;
        extent.height = dimensions.1;
        extent.depth = 1;
        let image_create_info = ImageCreateInfoBuilder::new()
            .format(Format::R8G8B8A8_UNORM)
            .initial_layout(ImageLayout::UNDEFINED)
            .samples(SampleCountFlagBits::_1)
            .tiling(ImageTiling::OPTIMAL)
            .usage(ImageUsageFlags::SAMPLED | ImageUsageFlags::TRANSFER_DST)
            .sharing_mode(SharingMode::EXCLUSIVE)
            .image_type(ImageType::_2D)
            .mip_levels(1)
            .array_layers(1)
            .extent(extent);
        let alloc_create_info = AllocationCreateInfoBuilder::new()
            .usage(MemoryUsage::GPUOnly)
            .build();
        unsafe { allocator.create_image(&image_create_info, &alloc_create_info) }
            .expect("Failed to create image")
    }

    fn create_image_view(device: &vk::Device, image: Image) -> ImageView {
        let subresource_range = ImageSubresourceRangeBuilder::new()
            .aspect_mask(ImageAspectFlags::COLOR)
            .base_array_layer(0)
            .base_mip_level(0)
            .layer_count(1)
            .level_count(1);
        let create_info = ImageViewCreateInfoBuilder::new()
            .image(image)
            .format(Format::R8G8B8A8_UNORM)
            .view_type(ImageViewType::_2D)
            .subresource_range(*subresource_range);
        unsafe { device.loader().create_image_view(&create_info, None, None) }
            .expect("Failed to create image view")
    }

    fn upload_font(&self, device: &vk::Device, allocator: &Allocator, data: &[u8]) {
        //
        // Creating then immediately destroying a command pool isn't very efficient, but I don't
        // care that much. It's just for uploading the ImGui font texture which will happen once
        //
        let create_info = CommandPoolCreateInfoBuilder::new()
            .queue_family_index(device.general_family().index)
            .flags(CommandPoolCreateFlags::TRANSIENT);
        let command_pool = unsafe {
            device
                .loader()
                .create_command_pool(&create_info, None, None)
        }
        .expect("Failed to create command pool");

        let allocate_info = CommandBufferAllocateInfoBuilder::new()
            .command_pool(command_pool)
            .command_buffer_count(1)
            .level(CommandBufferLevel::PRIMARY);
        let command_buffer = unsafe { device.loader().allocate_command_buffers(&allocate_info) }
            .expect("Failed to allocate command buffer")[0];

        let size = self.dimensions.0 * self.dimensions.1 * 4;
        let size = size as u64;
        let buffer_create_info = BufferCreateInfoBuilder::new()
            .usage(BufferUsageFlags::TRANSFER_SRC)
            .sharing_mode(SharingMode::EXCLUSIVE)
            .size(size);
        let alloc_create_info = AllocationCreateInfoBuilder::new()
            .usage(MemoryUsage::CPUOnly)
            .build();
        let (upload_buffer, upload_allocation) =
            unsafe { allocator.create_buffer(&buffer_create_info, &alloc_create_info) }
                .expect("Failed to create font image staging buffer");

        unsafe {
            let mem = allocator
                .map_memory(&upload_allocation)
                .expect("Failed to map upload memory");
            mem.copy_from(data.as_ptr(), data.len());
            allocator.unmap_memory(&upload_allocation);
        }

        unsafe {
            let begin_info = CommandBufferBeginInfoBuilder::new()
                .flags(CommandBufferUsageFlags::ONE_TIME_SUBMIT);
            device
                .loader()
                .begin_command_buffer(command_buffer, &begin_info)
                .expect("Failed to begin command buffer");

            //
            // Transition the image from raw state to the state needed for copying into the image
            //
            let memory_barriers = [];
            let buffer_memory_barriers = [];
            let subresource_range = ImageSubresourceRangeBuilder::new()
                .aspect_mask(ImageAspectFlags::COLOR)
                .level_count(1)
                .layer_count(1)
                .base_mip_level(0)
                .base_array_layer(0);
            let image_memory_barriers = [ImageMemoryBarrierBuilder::new()
                .image(self.image)
                .subresource_range(*subresource_range)
                .src_access_mask(AccessFlags::default())
                .dst_access_mask(AccessFlags::TRANSFER_WRITE)
                .old_layout(ImageLayout::UNDEFINED)
                .new_layout(ImageLayout::TRANSFER_DST_OPTIMAL)];
            device.loader().cmd_pipeline_barrier(
                command_buffer,
                PipelineStageFlags::HOST,
                PipelineStageFlags::TRANSFER,
                DependencyFlagBits(0).bitmask(),
                &memory_barriers,
                &buffer_memory_barriers,
                &image_memory_barriers,
            );

            //
            // Copy from the upload buffer into the image
            //
            let subresource = ImageSubresourceLayersBuilder::new()
                .aspect_mask(ImageAspectFlags::COLOR)
                .base_array_layer(0)
                .layer_count(1)
                .mip_level(0);
            let mut extent = Extent3D::default();
            extent.width = self.dimensions.0;
            extent.height = self.dimensions.1;
            extent.depth = 1;
            let region = BufferImageCopyBuilder::new()
                .image_subresource(*subresource)
                .image_extent(extent);
            let regions = [region];
            device.loader().cmd_copy_buffer_to_image(
                command_buffer,
                upload_buffer,
                self.image,
                ImageLayout::TRANSFER_DST_OPTIMAL,
                &regions,
            );

            //
            // Transition the image into a state that can be used for being sampled from
            //
            let memory_barriers = [];
            let buffer_memory_barriers = [];
            let subresource_range = ImageSubresourceRangeBuilder::new()
                .aspect_mask(ImageAspectFlags::COLOR)
                .level_count(1)
                .layer_count(1)
                .base_mip_level(0)
                .base_array_layer(0);
            let image_barrier = ImageMemoryBarrierBuilder::new()
                .image(self.image)
                .subresource_range(*subresource_range)
                .src_access_mask(AccessFlags::TRANSFER_WRITE)
                .dst_access_mask(AccessFlags::SHADER_READ)
                .old_layout(ImageLayout::TRANSFER_DST_OPTIMAL)
                .new_layout(ImageLayout::SHADER_READ_ONLY_OPTIMAL);
            let image_memory_barriers = [image_barrier];
            device.loader().cmd_pipeline_barrier(
                command_buffer,
                PipelineStageFlags::TRANSFER,
                PipelineStageFlags::FRAGMENT_SHADER,
                DependencyFlagBits(0).bitmask(),
                &memory_barriers,
                &buffer_memory_barriers,
                &image_memory_barriers,
            );

            device
                .loader()
                .end_command_buffer(command_buffer)
                .expect("Failed to end command buffer");
        }

        unsafe {
            let command_buffers = [command_buffer];
            let submit = SubmitInfoBuilder::new().command_buffers(&command_buffers);
            let submits = [submit];
            device
                .loader()
                .queue_submit(device.general_queue(), &submits, Fence::null())
                .expect("Failed to submit command buffer");
            device
                .loader()
                .queue_wait_idle(device.general_queue())
                .expect("Failed to wait on queue idle");
        }

        unsafe {
            allocator.destroy_buffer(upload_buffer, upload_allocation);
            device.loader().destroy_command_pool(command_pool, None);
        }
    }

    unsafe fn destroy(&self, device: &vk::Device, allocator: &Allocator) {
        device.loader().destroy_image_view(self.image_view, None);
        allocator.destroy_image(self.image, self.allocation);
        device.loader().destroy_sampler(self.sampler, None);
    }
}

///
/// A struct to wrap resources that are created and destroyed once during the Imgui renderer's
/// lifecycle
///
struct ImguiGlobal {
    descriptor_pool: DescriptorPool,
    descriptor_set_layout: DescriptorSetLayout,
    descriptor_set: DescriptorSet,
    vertex_module: ShaderModule,
    fragment_module: ShaderModule,
}

impl ImguiGlobal {
    fn init(device: &vk::Device) -> Self {
        let descriptor_pool = Self::create_descriptor_pool(device);
        let descriptor_set_layout = Self::create_descriptor_set_layout(device);
        let descriptor_set =
            Self::allocate_descriptor_set(device, descriptor_set_layout, descriptor_pool);
        let (vertex_module, fragment_module) = Self::create_shader_modules(device);

        Self {
            descriptor_pool,
            descriptor_set_layout,
            descriptor_set,
            vertex_module,
            fragment_module,
        }
    }

    fn create_descriptor_pool(device: &vk::Device) -> DescriptorPool {
        let pool_sizes = [
            DescriptorPoolSizeBuilder::new()
                ._type(DescriptorType::SAMPLER)
                .descriptor_count(32),
            DescriptorPoolSizeBuilder::new()
                ._type(DescriptorType::SAMPLED_IMAGE)
                .descriptor_count(32),
            DescriptorPoolSizeBuilder::new()
                ._type(DescriptorType::COMBINED_IMAGE_SAMPLER)
                .descriptor_count(32),
        ];
        let create_info = DescriptorPoolCreateInfoBuilder::new()
            .flags(DescriptorPoolCreateFlags::FREE_DESCRIPTOR_SET)
            .max_sets(32)
            .pool_sizes(&pool_sizes);
        unsafe {
            device
                .loader()
                .create_descriptor_pool(&create_info, None, None)
        }
        .expect("Failed to create descriptor pool")
    }

    fn create_descriptor_set_layout(device: &vk::Device) -> DescriptorSetLayout {
        let binding = DescriptorSetLayoutBindingBuilder::new()
            .binding(0)
            .descriptor_type(DescriptorType::COMBINED_IMAGE_SAMPLER)
            .descriptor_count(1)
            .stage_flags(ShaderStageFlags::FRAGMENT);
        let bindings = [binding];
        let create_info = DescriptorSetLayoutCreateInfoBuilder::new().bindings(&bindings);
        unsafe {
            device
                .loader()
                .create_descriptor_set_layout(&create_info, None, None)
        }
        .expect("Failed to create descriptor set layout")
    }

    fn allocate_descriptor_set(
        device: &vk::Device,
        layout: DescriptorSetLayout,
        pool: DescriptorPool,
    ) -> DescriptorSet {
        let set_layouts = [layout];
        let allocate_info = DescriptorSetAllocateInfoBuilder::new()
            .descriptor_pool(pool)
            .set_layouts(&set_layouts);
        unsafe { device.loader().allocate_descriptor_sets(&allocate_info) }
            .expect("Failed to allocate descriptor sets")[0]
    }

    fn create_shader_modules(device: &vk::Device) -> (ShaderModule, ShaderModule) {
        // Compiled with
        // `dxc /T vs_6_0 -Fo imgui.vert.spv -spirv .\imgui.vert.hlsl`
        let bytes = include_bytes_aligned_as!(u32, "../../../shaders/imgui.vert.spv");
        let slice =
            unsafe { core::slice::from_raw_parts(bytes.as_ptr() as *const u32, bytes.len() / 4) };
        let create_info = ShaderModuleCreateInfoBuilder::new().code(slice);
        let vertex_module = unsafe {
            device
                .loader()
                .create_shader_module(&create_info, None, None)
        }
        .expect("Failed to create vertex shader module");

        // Compiled with
        // `dxc /T ps_6_0 -Fo imgui.frag.spv -spirv .\imgui.frag.hlsl`
        let bytes = include_bytes_aligned_as!(u32, "../../../shaders/imgui.frag.spv");
        let slice =
            unsafe { core::slice::from_raw_parts(bytes.as_ptr() as *const u32, bytes.len() / 4) };
        let create_info = ShaderModuleCreateInfoBuilder::new().code(slice);
        let fragment_module = unsafe {
            device
                .loader()
                .create_shader_module(&create_info, None, None)
        }
        .expect("Failed to create vertex shader module");

        (vertex_module, fragment_module)
    }

    unsafe fn destroy(&self, device: &vk::Device) {
        device
            .loader()
            .destroy_shader_module(self.fragment_module, None);
        device
            .loader()
            .destroy_shader_module(self.vertex_module, None);
        device
            .loader()
            .free_descriptor_sets(self.descriptor_pool, &[self.descriptor_set])
            .expect("Failed to free descriptor set");
        device
            .loader()
            .destroy_descriptor_set_layout(self.descriptor_set_layout, None);
        device
            .loader()
            .destroy_descriptor_pool(self.descriptor_pool, None);
    }
}

pub struct ImguiRenderer {
    device: Arc<vk::Device>,
    allocator: Arc<Allocator>,
    frames: Vec<ImguiFrame>,
    single: ImguiSingular,
    font: ImguiFont,
    global: ImguiGlobal,
}

impl ImguiRenderer {
    pub fn new(
        fonts: imgui::FontAtlasRefMut,
        device: Arc<vk::Device>,
        allocator: Arc<Allocator>,
        swapchain: &vk::Swapchain,
    ) -> Self {
        log::info!("Initializing ImGui Renderer");
        let global = ImguiGlobal::init(&device);

        let font = ImguiFont::init(fonts, &global, &device, &allocator);

        let single = ImguiSingular::init(&device, &global, swapchain.format().format);

        let frames = (0..swapchain.images().len())
            .into_iter()
            .map(|index| {
                ImguiFrame::init(&device, &allocator, swapchain, index, single.render_pass)
            })
            .collect();

        ImguiRenderer {
            device,
            allocator,
            frames,
            single,
            font,
            global,
        }
    }

    pub unsafe fn recreate_resources(&mut self, swapchain: &vk::Swapchain) {
        for frame in self.frames.iter() {
            frame.destroy(&self.device, &self.allocator);
        }
        self.single.destroy(&self.device);

        self.single = ImguiSingular::init(&self.device, &self.global, swapchain.format().format);

        self.frames = (0..swapchain.images().len())
            .into_iter()
            .map(|index| {
                ImguiFrame::init(
                    &self.device,
                    &self.allocator,
                    &swapchain,
                    index,
                    self.single.render_pass,
                )
            })
            .collect();
    }

    pub unsafe fn render_frame(
        &mut self,
        frame: imgui::Ui,
        swapchain: &vk::Swapchain,
        acquire_semaphore: Semaphore,
        signal_semaphore: Semaphore,
        index: usize,
    ) {
        let draw_data = frame.render();

        //
        // Free the vertex buffer if one already exists
        //
        if self.frames[index].vtx_buffer.0 != Buffer::null() {
            self.allocator.destroy_buffer(
                self.frames[index].vtx_buffer.0,
                self.frames[index].vtx_buffer.1,
            );
        }

        //
        // Free the index buffer if one already exists
        //
        if self.frames[index].idx_buffer.0 != Buffer::null() {
            self.allocator.destroy_buffer(
                self.frames[index].idx_buffer.0,
                self.frames[index].idx_buffer.1,
            );
        }

        //
        // Make sure there is actually some vertex data to upload
        //
        if draw_data.total_vtx_count != 0 {
            //
            // Allocate a vertex buffer that will only be used for one frame
            //
            let buffer_create_info = BufferCreateInfoBuilder::new()
                .usage(BufferUsageFlags::VERTEX_BUFFER)
                .sharing_mode(SharingMode::EXCLUSIVE)
                .size(
                    draw_data.total_vtx_count as u64
                        * core::mem::size_of::<imgui::DrawVert>() as u64,
                );
            let alloc_create_info = AllocationCreateInfoBuilder::new()
                .usage(MemoryUsage::CPUToGPU)
                .pool(&self.frames[index].memory_pool)
                .build();
            self.frames[index].vtx_buffer = self
                .allocator
                .create_buffer(&buffer_create_info, &alloc_create_info)
                .expect("Failed to allocate per frame vertex buffer");

            //
            // Allocate an index buffer that will only be used for one frame
            //
            let buffer_create_info = BufferCreateInfoBuilder::new()
                .usage(BufferUsageFlags::INDEX_BUFFER)
                .sharing_mode(SharingMode::EXCLUSIVE)
                .size(
                    draw_data.total_idx_count as u64
                        * core::mem::size_of::<imgui::DrawIdx>() as u64,
                );
            let allocation_create_info = AllocationCreateInfoBuilder::new()
                .flags(AllocationCreateFlag::UPPER_ADDRESS_BIT)
                .usage(MemoryUsage::CPUToGPU)
                .pool(&self.frames[index].memory_pool)
                .build();
            self.frames[index].idx_buffer = self
                .allocator
                .create_buffer(&buffer_create_info, &allocation_create_info)
                .expect("Failed to allocate per frame index buffer");

            let vtx_buffer = self.frames[index].vtx_buffer;
            let idx_buffer = self.frames[index].idx_buffer;

            //
            // Map the vertex and index buffers
            //
            let mut vptr = self
                .allocator
                .map_memory(&vtx_buffer.1)
                .expect("Failed to map vertex buffer");
            let mut iptr = self
                .allocator
                .map_memory(&idx_buffer.1)
                .expect("Failed to map index buffer");

            //
            // Copy vertex and index buffer data into the vulkan buffers
            //
            draw_data.draw_lists().for_each(|list| {
                let vslice = list.vtx_buffer();
                vptr.copy_from(
                    vslice.as_ptr() as *const _,
                    vslice.len() * core::mem::size_of::<imgui::DrawVert>(),
                );
                vptr = vptr.add(vslice.len() * core::mem::size_of::<imgui::DrawVert>());

                let islice = list.idx_buffer();
                iptr.copy_from(
                    islice.as_ptr() as *const _,
                    islice.len() * core::mem::size_of::<imgui::DrawIdx>(),
                );
                iptr = iptr.add(islice.len() * core::mem::size_of::<imgui::DrawIdx>());
            });

            //
            // Flush and unmap the vertex and index buffers
            //
            self.allocator
                .flush_allocation(&vtx_buffer.1, 0, WHOLE_SIZE);
            self.allocator
                .flush_allocation(&idx_buffer.1, 0, WHOLE_SIZE);
            self.allocator.unmap_memory(&vtx_buffer.1);
            self.allocator.unmap_memory(&idx_buffer.1);
        }

        let vtx_buffer = self.frames[index].vtx_buffer;
        let idx_buffer = self.frames[index].idx_buffer;
        let command_buffer = self.frames[index].command_buffer;

        //
        // Begin command buffer recording
        //
        let begin_info =
            CommandBufferBeginInfoBuilder::new().flags(CommandBufferUsageFlags::ONE_TIME_SUBMIT);
        self.device
            .loader()
            .begin_command_buffer(command_buffer, &begin_info)
            .expect("Failed to begin command buffer");

        //
        // We need to special case for when imgui wants to render nothing as vertex buffers wont
        // exist so we can't bind them. We still need to transition the image though so instead we
        // just insert a pipeline barrier to do the transition.
        //
        if draw_data.total_vtx_count != 0 {
            //
            // Begin the render pass
            //
            let clear_values = [ClearValue {
                color: ClearColorValue {
                    float32: [0.0, 0.0, 0.0, 1.0],
                },
            }];
            let render_area = Rect2DBuilder::new().extent(swapchain.extents());
            let render_pass_begin = RenderPassBeginInfoBuilder::new()
                .render_pass(self.single.render_pass)
                .framebuffer(self.frames[index].framebuffer)
                .clear_values(&clear_values)
                .render_area(*render_area);
            self.device.loader().cmd_begin_render_pass(
                command_buffer,
                &render_pass_begin,
                SubpassContents::INLINE,
            );

            self.reset_render_state(
                swapchain,
                command_buffer,
                vtx_buffer.0,
                idx_buffer.0,
                draw_data,
            );

            let clip_off = draw_data.display_pos;
            let clip_scale = draw_data.framebuffer_scale;

            let mut vtx_offset = 0;
            let mut idx_offset = 0;
            draw_data.draw_lists().for_each(|list| {
                list.commands().for_each(|command| {
                    self.render_draw_command(
                        draw_data,
                        swapchain,
                        vtx_buffer.0,
                        idx_buffer.0,
                        command_buffer,
                        &clip_off,
                        &clip_scale,
                        vtx_offset,
                        idx_offset,
                        command,
                    )
                });
                vtx_offset += list.vtx_buffer().len() as i32;
                idx_offset += list.idx_buffer().len() as u32;
            });

            self.device.loader().cmd_end_render_pass(command_buffer);
        } else {
            let src = PipelineStageFlags::BOTTOM_OF_PIPE;
            let dst = PipelineStageFlags::TOP_OF_PIPE;
            let flags = DependencyFlags::from_bits(0).unwrap();
            let range = ImageSubresourceRangeBuilder::new()
                .layer_count(1)
                .level_count(1)
                .base_array_layer(0)
                .base_mip_level(0)
                .aspect_mask(ImageAspectFlags::COLOR)
                .discard();
            let memory = [];
            let buffer = [];
            let image = [ImageMemoryBarrierBuilder::new()
                .image(swapchain.images()[index])
                .old_layout(ImageLayout::UNDEFINED)
                .new_layout(ImageLayout::PRESENT_SRC_KHR)
                .subresource_range(range)];
            self.device.loader().cmd_pipeline_barrier(
                command_buffer,
                src,
                dst,
                flags,
                &memory,
                &buffer,
                &image,
            );
        }

        self.device
            .loader()
            .end_command_buffer(command_buffer)
            .expect("Failed to end command buffer");

        let command_buffers = [command_buffer];
        let wait_semaphores = [acquire_semaphore];
        let signal_semaphores = [signal_semaphore];
        let wait_dst_stage_mask = [PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
        let submit = SubmitInfoBuilder::new()
            .command_buffers(&command_buffers)
            .wait_semaphores(&wait_semaphores)
            .signal_semaphores(&signal_semaphores)
            .wait_dst_stage_mask(&wait_dst_stage_mask);
        self.device
            .loader()
            .queue_submit(self.device.general_queue(), &[submit], Fence::null())
            .expect("Failed to submit to queue");
    }

    unsafe fn render_draw_command(
        &mut self,
        draw_data: &imgui::DrawData,
        swapchain: &vk::Swapchain,
        vertex_buffer: Buffer,
        index_buffer: Buffer,
        command_buffer: CommandBuffer,
        clip_off: &[f32; 2],
        clip_scale: &[f32; 2],
        vtx_offset: i32,
        idx_offset: u32,
        command: DrawCmd,
    ) {
        match command {
            DrawCmd::Elements { cmd_params, count } => {
                let clip_rect = [
                    (cmd_params.clip_rect[0] - clip_off[0]) * clip_scale[0],
                    (cmd_params.clip_rect[1] - clip_off[1]) * clip_scale[1],
                    (cmd_params.clip_rect[2] - clip_off[0]) * clip_scale[0],
                    (cmd_params.clip_rect[3] - clip_off[1]) * clip_scale[1],
                ];

                let swap_extent = swapchain.extents();
                if clip_rect[0] < swap_extent.width as f32
                    && clip_rect[1] < swap_extent.height as f32
                    && clip_rect[2] > 0.0
                    && clip_rect[3] > 0.0
                {
                    let offset = Offset2D {
                        x: (clip_rect[0] as i32).max(0),
                        y: (clip_rect[1] as i32).max(0),
                    };
                    let extent = Extent2D {
                        width: (clip_rect[2] - clip_rect[0]) as u32,
                        height: (clip_rect[3] - clip_rect[1]) as u32,
                    };
                    let scissor = Rect2DBuilder::new().offset(offset).extent(extent);
                    self.device
                        .loader()
                        .cmd_set_scissor(command_buffer, 0, &[scissor]);

                    self.device.loader().cmd_draw_indexed(
                        command_buffer,
                        count as _,
                        1,
                        cmd_params.idx_offset as u32 + idx_offset,
                        cmd_params.vtx_offset as i32 + vtx_offset,
                        0,
                    )
                }
            }
            DrawCmd::ResetRenderState => {
                self.reset_render_state(
                    swapchain,
                    command_buffer,
                    vertex_buffer,
                    index_buffer,
                    draw_data,
                );
            }
            DrawCmd::RawCallback { .. } => {
                unimplemented!("Cant use callback because bindings make it hard");
            }
        }
    }

    unsafe fn reset_render_state(
        &self,
        swapchain: &vk::Swapchain,
        command_buffer: CommandBuffer,
        vertex_buffer: Buffer,
        index_buffer: Buffer,
        draw_data: &imgui::DrawData,
    ) {
        //
        // Bind the pipeline and descriptor set that we'll be rendering with
        //
        self.device.loader().cmd_bind_pipeline(
            command_buffer,
            PipelineBindPoint::GRAPHICS,
            self.single.pipeline,
        );
        self.device.loader().cmd_bind_descriptor_sets(
            command_buffer,
            PipelineBindPoint::GRAPHICS,
            self.single.pipeline_layout,
            0,
            &[self.global.descriptor_set],
            &[],
        );

        //
        // Bind the vertex and index buffers to render with
        //
        self.device
            .loader()
            .cmd_bind_vertex_buffers(command_buffer, 0, &[vertex_buffer], &[0]);
        self.device.loader().cmd_bind_index_buffer(
            command_buffer,
            index_buffer,
            0,
            IndexType::UINT16,
        );

        //
        // Set the viewport state, we're going to be rendering to the whole frame
        //
        let viewport = ViewportBuilder::new()
            .width(swapchain.extents().width as f32)
            .height(swapchain.extents().height as f32)
            .min_depth(0.0)
            .max_depth(1.0)
            .x(0.0)
            .y(0.0);
        self.device
            .loader()
            .cmd_set_viewport(command_buffer, 0, &[viewport]);

        //
        // Push transforms via push constants
        //
        let scale = [
            2.0 / draw_data.display_size[0],
            2.0 / draw_data.display_size[1],
        ];
        self.device.loader().cmd_push_constants(
            command_buffer,
            self.single.pipeline_layout,
            ShaderStageFlags::VERTEX,
            0,
            8,
            scale.as_ptr() as *const _,
        );
        let translate = [
            -1.0 - draw_data.display_pos[0] * scale[0],
            -1.0 - draw_data.display_pos[1] * scale[1],
        ];
        self.device.loader().cmd_push_constants(
            command_buffer,
            self.single.pipeline_layout,
            ShaderStageFlags::VERTEX,
            8,
            8,
            translate.as_ptr() as *const _,
        );
    }
}

impl Drop for ImguiRenderer {
    fn drop(&mut self) {
        unsafe {
            for frame in self.frames.iter() {
                frame.destroy(&self.device, &self.allocator)
            }
            self.single.destroy(&self.device);
            self.font.destroy(&self.device, &self.allocator);
            self.global.destroy(&self.device);
        }
    }
}
