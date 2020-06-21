//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use crate::gpu::vk;
use crate::gpu::vk::reflect::{PushConstantLayout, Set};
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
    pub vertex_module: ShaderModule,
    pub fragment_module: ShaderModule,
    pub descriptor_pool: DescriptorPool,
    pub descriptor_set_layout: DescriptorSetLayout,
    pub descriptor_set: DescriptorSet,
    pub push_constant_layout: PushConstantLayout,
    pub reflected_set_layout: Set,
}

impl ImguiGlobal {
    pub fn init(device: &vk::Device) -> Self {
        let (vertex_module, fragment_module) = Self::create_shader_modules(device);
        let reflected_set_layout = Self::reflect_frag_module();
        let push_constant_layout = Self::reflect_vert_module();
        let descriptor_pool = Self::create_descriptor_pool(device);
        let descriptor_set_layout =
            Self::create_descriptor_set_layout(device, &reflected_set_layout);
        let descriptor_set =
            Self::allocate_descriptor_set(device, descriptor_set_layout, descriptor_pool);

        Self {
            descriptor_pool,
            descriptor_set_layout,
            descriptor_set,
            vertex_module,
            fragment_module,
            push_constant_layout,
            reflected_set_layout,
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

    pub fn create_descriptor_set_layout(
        device: &vk::Device,
        reflected_set_layout: &Set,
    ) -> DescriptorSetLayout {
        let bindings: Vec<DescriptorSetLayoutBindingBuilder> = reflected_set_layout
            .bindings()
            .iter()
            .map(|binding| {
                DescriptorSetLayoutBindingBuilder::new()
                    .binding(binding.binding())
                    .descriptor_type(binding.binding_type().descriptor_type())
                    .descriptor_count(1)
                    .stage_flags(ShaderStageFlags::FRAGMENT)
            })
            .collect();
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
        let bytes =
            include_bytes_aligned_as!(u32, "../../../../../shaders/compiled/imgui/imgui.vert.spv");
        let slice =
            unsafe { core::slice::from_raw_parts(bytes.as_ptr() as *const u32, bytes.len() / 4) };
        let create_info = ShaderModuleCreateInfoBuilder::new().code(slice);
        let vertex_module = unsafe {
            device
                .loader()
                .create_shader_module(&create_info, None, None)
        }
        .expect("Failed to create vertex shader module");

        let bytes =
            include_bytes_aligned_as!(u32, "../../../../../shaders/compiled/imgui/imgui.frag.spv");
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

    pub fn reflect_frag_module() -> Set {
        let bytes =
            include_bytes_aligned_as!(u32, "../../../../../shaders/compiled/imgui/imgui.frag.spv");

        let module = spirv_reflect::ShaderModule::load_u8_data(bytes)
            .expect("Failed to reflect spirv module");

        let mut entry_point = module
            .enumerate_entry_points()
            .expect("No entry points found")
            .drain(..)
            .find(|v| v.name == "main")
            .expect("Failed to find \"main\" entry point");

        // Assume we only have a single descriptor set
        Set::reflect(&mut entry_point).drain(..).nth(0).unwrap()
    }

    pub fn reflect_vert_module() -> PushConstantLayout {
        let bytes =
            include_bytes_aligned_as!(u32, "../../../../../shaders/compiled/imgui/imgui.vert.spv");

        let module = spirv_reflect::ShaderModule::load_u8_data(bytes)
            .expect("Failed to reflect spirv module");

        let entry_point = module
            .enumerate_entry_points()
            .expect("No entry points found")
            .drain(..)
            .find(|v| v.name == "main")
            .expect("Failed to find \"main\" entry point");

        // Assume we only have a single push constant block
        let push_constants = module
            .enumerate_push_constant_blocks(Some(&entry_point.name))
            .expect("No push constants found")
            .drain(..)
            .nth(0)
            .unwrap();

        PushConstantLayout::reflect(push_constants)
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
