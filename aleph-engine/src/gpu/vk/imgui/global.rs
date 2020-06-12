//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use crate::gpu::vk;
use crate::include_bytes_aligned_as;
use erupt::vk1_0::{
    DescriptorPool, DescriptorPoolCreateFlags, DescriptorPoolCreateInfoBuilder,
    DescriptorPoolSizeBuilder, DescriptorSet, DescriptorSetAllocateInfoBuilder,
    DescriptorSetLayout, DescriptorSetLayoutBindingBuilder, DescriptorSetLayoutCreateInfoBuilder,
    DescriptorType, ShaderModule, ShaderModuleCreateInfoBuilder, ShaderStageFlags,
    Vk10DeviceLoaderExt,
};

///
/// A struct to wrap resources that are created and destroyed once during the Imgui renderer's
/// lifecycle
///
pub struct ImguiGlobal {
    pub descriptor_pool: DescriptorPool,
    pub descriptor_set_layout: DescriptorSetLayout,
    pub descriptor_set: DescriptorSet,
    pub vertex_module: ShaderModule,
    pub fragment_module: ShaderModule,
}

impl ImguiGlobal {
    pub fn init(device: &vk::Device) -> Self {
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

    pub fn create_descriptor_pool(device: &vk::Device) -> DescriptorPool {
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

    pub fn create_descriptor_set_layout(device: &vk::Device) -> DescriptorSetLayout {
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

    pub fn allocate_descriptor_set(
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

    pub fn create_shader_modules(device: &vk::Device) -> (ShaderModule, ShaderModule) {
        // Compiled with
        // `dxc /T vs_6_0 -Fo imgui.vert.spv -spirv .\imgui.vert.hlsl`
        let bytes = include_bytes_aligned_as!(u32, "../../../../shaders/compiled/imgui.vert.spv");
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
        let bytes = include_bytes_aligned_as!(u32, "../../../../shaders/compiled/imgui.frag.spv");
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

    pub unsafe fn destroy(&self, device: &vk::Device) {
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
