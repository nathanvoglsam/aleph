//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
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
                use erupt::extensions::ext_debug_utils::{
                    DebugUtilsObjectNameInfoEXTBuilder, ExtDebugUtilsDeviceLoaderExt,
                };
                if device.loader().ext_debug_utils.is_some() && *self != Default::default() {
                    let name_info = DebugUtilsObjectNameInfoEXTBuilder::new()
                        .object_handle(self.0 as u64)
                        .object_type($obj_type)
                        .object_name(name);
                    device
                        .loader()
                        .set_debug_utils_object_name_ext(&name_info)
                        .expect("Failed to add debug name to object");
                }
            }
        }
    };
}

implement_debug_name_for!(erupt::vk1_0::Device, erupt::vk1_0::ObjectType::DEVICE);
implement_debug_name_for!(
    erupt::vk1_0::CommandBuffer,
    erupt::vk1_0::ObjectType::COMMAND_BUFFER
);
implement_debug_name_for!(
    erupt::vk1_0::CommandPool,
    erupt::vk1_0::ObjectType::COMMAND_POOL
);
implement_debug_name_for!(erupt::vk1_0::Image, erupt::vk1_0::ObjectType::IMAGE);
implement_debug_name_for!(
    erupt::vk1_0::ImageView,
    erupt::vk1_0::ObjectType::IMAGE_VIEW
);
implement_debug_name_for!(erupt::vk1_0::Sampler, erupt::vk1_0::ObjectType::SAMPLER);
implement_debug_name_for!(
    erupt::vk1_0::Framebuffer,
    erupt::vk1_0::ObjectType::FRAMEBUFFER
);
implement_debug_name_for!(erupt::vk1_0::Pipeline, erupt::vk1_0::ObjectType::PIPELINE);
implement_debug_name_for!(
    erupt::vk1_0::PipelineLayout,
    erupt::vk1_0::ObjectType::PIPELINE_LAYOUT
);
implement_debug_name_for!(
    erupt::vk1_0::PipelineCache,
    erupt::vk1_0::ObjectType::PIPELINE_CACHE
);
implement_debug_name_for!(
    erupt::vk1_0::ShaderModule,
    erupt::vk1_0::ObjectType::SHADER_MODULE
);
implement_debug_name_for!(erupt::vk1_0::Buffer, erupt::vk1_0::ObjectType::BUFFER);
implement_debug_name_for!(
    erupt::vk1_0::BufferView,
    erupt::vk1_0::ObjectType::BUFFER_VIEW
);
implement_debug_name_for!(
    erupt::vk1_0::DescriptorPool,
    erupt::vk1_0::ObjectType::DESCRIPTOR_POOL
);
implement_debug_name_for!(
    erupt::vk1_0::DescriptorSet,
    erupt::vk1_0::ObjectType::DESCRIPTOR_SET
);
implement_debug_name_for!(
    erupt::vk1_0::DescriptorSetLayout,
    erupt::vk1_0::ObjectType::DESCRIPTOR_SET_LAYOUT
);
implement_debug_name_for!(
    erupt::vk1_0::RenderPass,
    erupt::vk1_0::ObjectType::RENDER_PASS
);
