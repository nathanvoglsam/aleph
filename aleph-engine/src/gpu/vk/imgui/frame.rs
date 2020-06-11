//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use crate::gpu::vk;
use crate::gpu::vk::alloc::{Allocation, AllocationCreateInfoBuilder, Allocator, MemoryUsage};
use erupt::vk1_0::{
    Buffer, BufferCreateInfoBuilder, BufferUsageFlags, CommandBuffer,
    CommandBufferAllocateInfoBuilder, CommandBufferLevel, CommandPool, CommandPoolCreateFlags,
    CommandPoolCreateInfoBuilder, ComponentMappingBuilder, ComponentSwizzle, Format, Framebuffer,
    FramebufferCreateInfoBuilder, Image, ImageAspectFlags, ImageSubresourceRangeBuilder, ImageView,
    ImageViewCreateInfoBuilder, ImageViewType, RenderPass, SharingMode, Vk10DeviceLoaderExt,
};
use std::sync::Arc;

///
/// This represents the resources needed for rendering a single imgui frame in parallel. This
/// separation allows for multiple frames in flight
///
pub struct ImguiFrame {
    pub command_pool: CommandPool,
    pub command_buffer: CommandBuffer,
    pub image_view: ImageView,
    pub framebuffer: Framebuffer,
    pub vtx_buffer: (Buffer, Allocation),
    pub idx_buffer: (Buffer, Allocation),
    pub memory_pool: Arc<vk::alloc::Pool>,
}

impl ImguiFrame {
    pub fn init(
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

    pub fn create_command_pool(device: &vk::Device) -> CommandPool {
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

    pub fn allocate_command_buffer(
        device: &vk::Device,
        command_pool: CommandPool,
    ) -> CommandBuffer {
        let allocate_info = CommandBufferAllocateInfoBuilder::new()
            .command_pool(command_pool)
            .level(CommandBufferLevel::PRIMARY)
            .command_buffer_count(1);
        unsafe { device.loader().allocate_command_buffers(&allocate_info) }
            .expect("Failed to create command buffer")[0]
    }

    pub fn create_image_view(device: &vk::Device, image: Image, format: Format) -> ImageView {
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

    pub fn create_framebuffer(
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

    pub unsafe fn destroy(&self, device: &vk::Device, allocator: &Allocator) {
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
