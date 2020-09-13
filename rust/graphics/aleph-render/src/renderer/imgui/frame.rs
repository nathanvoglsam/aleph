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

use aleph_vulkan_alloc::{Allocation, AllocationCreateInfoBuilder, Allocator, MemoryUsage};
use aleph_vulkan_core::erupt::vk1_0::{
    Buffer, BufferCreateInfoBuilder, BufferUsageFlags, Framebuffer, FramebufferCreateInfoBuilder,
    RenderPass, SharingMode, Vk10DeviceLoaderExt,
};
use aleph_vulkan_core::{DebugName, SwapImage};
use std::ffi::CString;
use std::sync::Arc;

///
/// This represents the resources needed for rendering a single imgui frame in parallel. This
/// separation allows for multiple frames in flight
///
pub struct ImguiFrame {
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
        let swap_image = swapchain.images()[index].clone();
        let framebuffer = Self::create_framebuffer(device, render_pass, &swap_image, index);

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
            swap_image,
            framebuffer,
            vtx_buffer: (Buffer::null(), Allocation::null()),
            idx_buffer: (Buffer::null(), Allocation::null()),
            memory_pool,
        }
    }

    pub fn create_framebuffer(
        device: &aleph_vulkan_core::Device,
        render_pass: RenderPass,
        swap_image: &SwapImage,
        index: usize,
    ) -> Framebuffer {
        let attachments = [swap_image.image_view()];
        let create_info = FramebufferCreateInfoBuilder::new()
            .render_pass(render_pass)
            .width(swap_image.width())
            .height(swap_image.height())
            .attachments(&attachments)
            .layers(1);
        unsafe {
            let framebuffer = device
                .loader()
                .create_framebuffer(&create_info, None, None)
                .expect("Failed to create framebuffer");

            let name = format!("{}::{}::FrameBuffer", module_path!(), index);
            let name = CString::new(name).unwrap();
            framebuffer.add_debug_name(device, &name);

            framebuffer
        }
    }

    pub unsafe fn destroy(&self, device: &aleph_vulkan_core::Device, allocator: &Allocator) {
        if self.vtx_buffer.0 != Buffer::null() {
            allocator.destroy_buffer(self.vtx_buffer.0, self.vtx_buffer.1);
        }
        if self.idx_buffer.0 != Buffer::null() {
            allocator.destroy_buffer(self.idx_buffer.0, self.idx_buffer.1);
        }
        device.loader().destroy_framebuffer(self.framebuffer, None);
    }
}
