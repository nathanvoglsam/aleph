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

use erupt::utils::VulkanResult;
use erupt::{vk, DeviceLoader};
use std::ffi::CString;
use std::ops::Deref;

pub fn set_name<T: HandleNameInfo>(loader: &DeviceLoader, handle: T, name: Option<&str>) {
    // Do nothing if needed extension isn't loaded
    if let Some(func) = loader.set_debug_utils_object_name_ext {
        // Can only set a name if one is provided
        if let Some(name) = name {
            let name = CString::new(name).unwrap();
            let info = vk::DebugUtilsObjectNameInfoEXTBuilder::new()
                .object_type(T::OBJECT_TYPE)
                .object_handle(handle.handle())
                .object_name(&name);
            unsafe {
                let result = (func)(loader.handle, info.deref());
                let result = VulkanResult::new(result, ());
                result.unwrap();
            }
        }
    }
}

pub trait HandleNameInfo {
    const OBJECT_TYPE: vk::ObjectType;
    fn handle(&self) -> u64;
}

macro_rules! handle_name_info_impl {
    ($handle: path, $object_type: path) => {
        impl HandleNameInfo for $handle {
            const OBJECT_TYPE: vk::ObjectType = $object_type;

            fn handle(&self) -> u64 {
                self.object_handle()
            }
        }
    };
}

handle_name_info_impl!(vk::Instance, vk::ObjectType::INSTANCE);
handle_name_info_impl!(vk::PhysicalDevice, vk::ObjectType::PHYSICAL_DEVICE);
handle_name_info_impl!(vk::Device, vk::ObjectType::DEVICE);
handle_name_info_impl!(vk::Queue, vk::ObjectType::QUEUE);
handle_name_info_impl!(vk::Semaphore, vk::ObjectType::SEMAPHORE);
handle_name_info_impl!(vk::CommandBuffer, vk::ObjectType::COMMAND_BUFFER);
handle_name_info_impl!(vk::Fence, vk::ObjectType::FENCE);
handle_name_info_impl!(vk::DeviceMemory, vk::ObjectType::DEVICE_MEMORY);
handle_name_info_impl!(vk::Buffer, vk::ObjectType::BUFFER);
handle_name_info_impl!(vk::Image, vk::ObjectType::IMAGE);
handle_name_info_impl!(vk::Event, vk::ObjectType::EVENT);
handle_name_info_impl!(vk::QueryPool, vk::ObjectType::QUERY_POOL);
handle_name_info_impl!(vk::BufferView, vk::ObjectType::BUFFER_VIEW);
handle_name_info_impl!(vk::ImageView, vk::ObjectType::IMAGE_VIEW);
handle_name_info_impl!(vk::ShaderModule, vk::ObjectType::SHADER_MODULE);
handle_name_info_impl!(vk::PipelineCache, vk::ObjectType::PIPELINE_CACHE);
handle_name_info_impl!(vk::PipelineLayout, vk::ObjectType::PIPELINE_LAYOUT);
handle_name_info_impl!(vk::RenderPass, vk::ObjectType::RENDER_PASS);
handle_name_info_impl!(vk::Pipeline, vk::ObjectType::PIPELINE);
handle_name_info_impl!(
    vk::DescriptorSetLayout,
    vk::ObjectType::DESCRIPTOR_SET_LAYOUT
);
handle_name_info_impl!(vk::Sampler, vk::ObjectType::SAMPLER);
handle_name_info_impl!(vk::DescriptorPool, vk::ObjectType::DESCRIPTOR_POOL);
handle_name_info_impl!(vk::DescriptorSet, vk::ObjectType::DESCRIPTOR_SET);
handle_name_info_impl!(vk::Framebuffer, vk::ObjectType::FRAMEBUFFER);
handle_name_info_impl!(vk::CommandPool, vk::ObjectType::COMMAND_POOL);
