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

use crate::Device;
use std::ffi::CStr;

///
/// A trait that can be implemented on types that support having debug names attached with their
/// handles
///
pub trait DebugName {
    ///
    /// Adds the given debug name to the handle.
    ///
    /// Unsafe as there's no way to guarantee a valid handle is being passed
    ///
    unsafe fn add_debug_name(&self, device: &Device, name: &CStr);
}

macro_rules! implement_debug_name_for {
    ($handle:ty, $obj_type:expr) => {
        impl $crate::DebugName for $handle {
            unsafe fn add_debug_name(&self, device: &$crate::Device, name: &std::ffi::CStr) {
                use erupt::extensions::ext_debug_utils::DebugUtilsObjectNameInfoEXTBuilder;
                let ext_loaded = device.instance().enabled().ext_debug_utils;
                if ext_loaded && *self != Default::default() {
                    let name_info = DebugUtilsObjectNameInfoEXTBuilder::new()
                        .object_handle(self.0 as u64)
                        .object_type($obj_type)
                        .object_name(name);
                    device
                        .set_debug_utils_object_name_ext(&name_info)
                        .expect("Failed to add debug name to object");
                }
            }
        }
    };
}

implement_debug_name_for!(erupt::vk1_0::Instance, erupt::vk1_0::ObjectType::INSTANCE);
implement_debug_name_for!(
    erupt::vk1_0::PhysicalDevice,
    erupt::vk1_0::ObjectType::PHYSICAL_DEVICE
);
implement_debug_name_for!(erupt::vk1_0::Device, erupt::vk1_0::ObjectType::DEVICE);
implement_debug_name_for!(erupt::vk1_0::Queue, erupt::vk1_0::ObjectType::QUEUE);
implement_debug_name_for!(erupt::vk1_0::Semaphore, erupt::vk1_0::ObjectType::SEMAPHORE);
implement_debug_name_for!(
    erupt::vk1_0::CommandBuffer,
    erupt::vk1_0::ObjectType::COMMAND_BUFFER
);
implement_debug_name_for!(erupt::vk1_0::Fence, erupt::vk1_0::ObjectType::FENCE);
implement_debug_name_for!(
    erupt::vk1_0::DeviceMemory,
    erupt::vk1_0::ObjectType::DEVICE_MEMORY
);
implement_debug_name_for!(erupt::vk1_0::Buffer, erupt::vk1_0::ObjectType::BUFFER);
implement_debug_name_for!(erupt::vk1_0::Image, erupt::vk1_0::ObjectType::IMAGE);
implement_debug_name_for!(erupt::vk1_0::Event, erupt::vk1_0::ObjectType::EVENT);
implement_debug_name_for!(
    erupt::vk1_0::QueryPool,
    erupt::vk1_0::ObjectType::QUERY_POOL
);
implement_debug_name_for!(
    erupt::vk1_0::BufferView,
    erupt::vk1_0::ObjectType::BUFFER_VIEW
);
implement_debug_name_for!(
    erupt::vk1_0::ImageView,
    erupt::vk1_0::ObjectType::IMAGE_VIEW
);
implement_debug_name_for!(
    erupt::vk1_0::ShaderModule,
    erupt::vk1_0::ObjectType::SHADER_MODULE
);
implement_debug_name_for!(
    erupt::vk1_0::PipelineCache,
    erupt::vk1_0::ObjectType::PIPELINE_CACHE
);
implement_debug_name_for!(
    erupt::vk1_0::PipelineLayout,
    erupt::vk1_0::ObjectType::PIPELINE_LAYOUT
);
implement_debug_name_for!(
    erupt::vk1_0::RenderPass,
    erupt::vk1_0::ObjectType::RENDER_PASS
);
implement_debug_name_for!(erupt::vk1_0::Pipeline, erupt::vk1_0::ObjectType::PIPELINE);
implement_debug_name_for!(
    erupt::vk1_0::DescriptorSetLayout,
    erupt::vk1_0::ObjectType::DESCRIPTOR_SET_LAYOUT
);
implement_debug_name_for!(erupt::vk1_0::Sampler, erupt::vk1_0::ObjectType::SAMPLER);
implement_debug_name_for!(
    erupt::vk1_0::DescriptorPool,
    erupt::vk1_0::ObjectType::DESCRIPTOR_POOL
);
implement_debug_name_for!(
    erupt::vk1_0::DescriptorSet,
    erupt::vk1_0::ObjectType::DESCRIPTOR_SET
);
implement_debug_name_for!(
    erupt::vk1_0::Framebuffer,
    erupt::vk1_0::ObjectType::FRAMEBUFFER
);
implement_debug_name_for!(
    erupt::vk1_0::CommandPool,
    erupt::vk1_0::ObjectType::COMMAND_POOL
);
