//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use aleph_vulkan_alloc::{Allocation, AllocationCreateInfoBuilder, Allocator, MemoryUsage};
use aleph_vulkan_core::erupt::vk1_0::{
    Buffer, BufferCreateInfoBuilder, BufferUsageFlags, CommandBuffer,
    CommandBufferAllocateInfoBuilder, CommandBufferLevel, CommandPool,
    CommandPoolCreateInfoBuilder, Framebuffer, FramebufferCreateInfoBuilder, ImageView, RenderPass,
    SharingMode, Vk10DeviceLoaderExt,
};
use aleph_vulkan_core::SwapImage;
use std::sync::Arc;

///
/// This represents the resources needed for rendering a single imgui frame in parallel. This
/// separation allows for multiple frames in flight
///
pub struct ImguiFrame {
    pub command_pool: CommandPool,
    pub command_buffer: CommandBuffer,
    pub swap_image: SwapImage,
    pub framebuffer: Framebuffer,
    pub vtx_buffer: (Buffer, Allocation),
    pub idx_buffer: (Buffer, Allocation),
    pub memory_pool: Arc<aleph_vulkan_alloc::Pool>,
}

impl ImguiFrame {
    pub fn init(
        device: &aleph_vulkan_core::Device,
        allocator: &Arc<Allocator>,
        swapchain: &aleph_vulkan_core::Swapchain,
        index: usize,
        render_pass: RenderPass,
    ) -> Self {
        let command_pool = Self::create_command_pool(device);
        let command_buffer = Self::allocate_command_buffer(device, command_pool);
        let swap_image = swapchain.images()[index].clone();
        let framebuffer =
            Self::create_framebuffer(device, swapchain, render_pass, swap_image.image_view());

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
            aleph_vulkan_alloc::PoolBuilder::new()
                .flags(aleph_vulkan_alloc::PoolCreateFlag::LINEAR_ALGORITHM_BIT)
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
            swap_image,
            framebuffer,
            vtx_buffer: (Buffer::null(), Allocation::null()),
            idx_buffer: (Buffer::null(), Allocation::null()),
            memory_pool,
        }
    }

    pub fn create_command_pool(device: &aleph_vulkan_core::Device) -> CommandPool {
        let create_info =
            CommandPoolCreateInfoBuilder::new().queue_family_index(device.general_family().index);
        unsafe {
            device
                .loader()
                .create_command_pool(&create_info, None, None)
        }
        .expect("Failed to create command pool")
    }

    pub fn allocate_command_buffer(
        device: &aleph_vulkan_core::Device,
        command_pool: CommandPool,
    ) -> CommandBuffer {
        let allocate_info = CommandBufferAllocateInfoBuilder::new()
            .command_pool(command_pool)
            .level(CommandBufferLevel::PRIMARY)
            .command_buffer_count(1);
        unsafe { device.loader().allocate_command_buffers(&allocate_info) }
            .expect("Failed to create command buffer")[0]
    }

    pub fn create_framebuffer(
        device: &aleph_vulkan_core::Device,
        swapchain: &aleph_vulkan_core::Swapchain,
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

    pub unsafe fn destroy(&self, device: &aleph_vulkan_core::Device, allocator: &Allocator) {
        if self.vtx_buffer.0 != Buffer::null() {
            allocator.destroy_buffer(self.vtx_buffer.0, self.vtx_buffer.1);
        }
        if self.idx_buffer.0 != Buffer::null() {
            allocator.destroy_buffer(self.idx_buffer.0, self.idx_buffer.1);
        }
        device.loader().destroy_framebuffer(self.framebuffer, None);
        device
            .loader()
            .free_command_buffers(self.command_pool, &[self.command_buffer]);
        device
            .loader()
            .destroy_command_pool(self.command_pool, None);
    }
}
