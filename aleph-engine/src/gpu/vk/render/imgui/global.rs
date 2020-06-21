//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use crate::gpu::vk;
use crate::gpu::vk::reflect::{DescriptorSetReflection, PushConstantReflection};
use crate::gpu::vk::shader::ShaderModule;
use crate::include_bytes_aligned_as;
use erupt::vk1_0::{
    DescriptorPool, DescriptorPoolCreateFlags, DescriptorPoolCreateInfoBuilder,
    DescriptorPoolSizeBuilder, DescriptorSet, DescriptorSetAllocateInfoBuilder,
    DescriptorSetLayout, DescriptorSetLayoutBindingBuilder, DescriptorSetLayoutCreateInfoBuilder,
    DescriptorType, PipelineLayout, PipelineLayoutCreateInfoBuilder, PushConstantRangeBuilder,
    ShaderStageFlags, Vk10DeviceLoaderExt,
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
    pub pipeline_layout: PipelineLayout,
    pub descriptor_set: DescriptorSet,
}

impl ImguiGlobal {
    pub fn init(device: &vk::core::Device) -> Self {
        let (vertex_module, fragment_module) = Self::create_shader_modules(device);
        let descriptor_pool = Self::create_descriptor_pool(device);
        let descriptor_set_layout =
            Self::create_descriptor_set_layout(device, &fragment_module.descriptor_sets()[0]);
        let pipeline_layout = Self::create_pipeline_layout(
            device,
            descriptor_set_layout,
            vertex_module.push_constants().unwrap(),
        );
        let descriptor_set =
            Self::allocate_descriptor_set(device, descriptor_set_layout, descriptor_pool);

        Self {
            vertex_module,
            fragment_module,
            descriptor_pool,
            descriptor_set_layout,
            pipeline_layout,
            descriptor_set,
        }
    }

    pub fn create_descriptor_pool(device: &vk::core::Device) -> DescriptorPool {
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
        device: &vk::core::Device,
        reflected_set_layout: &DescriptorSetReflection,
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

    pub fn create_pipeline_layout(
        device: &vk::core::Device,
        layout: DescriptorSetLayout,
        push_constant_layout: &PushConstantReflection,
    ) -> PipelineLayout {
        let set_layouts = [layout];
        let ranges = [PushConstantRangeBuilder::new()
            .stage_flags(ShaderStageFlags::VERTEX)
            .offset(0)
            .size(push_constant_layout.size_padded())];
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

    pub fn allocate_descriptor_set(
        device: &vk::core::Device,
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

    pub fn create_shader_modules(device: &vk::core::Device) -> (ShaderModule, ShaderModule) {
        let bytes =
            include_bytes_aligned_as!(u32, "../../../../../shaders/compiled/imgui/imgui.vert.spv");
        let slice =
            unsafe { core::slice::from_raw_parts(bytes.as_ptr() as *const u32, bytes.len() / 4) };
        let vertex_module = ShaderModule::builder()
            .reflect(true)
            .compile(true)
            .words(slice)
            .vertex()
            .build(Some(device));

        let bytes =
            include_bytes_aligned_as!(u32, "../../../../../shaders/compiled/imgui/imgui.frag.spv");
        let slice =
            unsafe { core::slice::from_raw_parts(bytes.as_ptr() as *const u32, bytes.len() / 4) };
        let fragment_module = ShaderModule::builder()
            .reflect(true)
            .compile(true)
            .words(slice)
            .fragment()
            .build(Some(device));

        (vertex_module, fragment_module)
    }

    pub unsafe fn destroy(&self, device: &vk::core::Device) {
        device
            .loader()
            .free_descriptor_sets(self.descriptor_pool, &[self.descriptor_set])
            .expect("Failed to free descriptor set");
        device
            .loader()
            .destroy_pipeline_layout(self.pipeline_layout, None);
        device
            .loader()
            .destroy_descriptor_set_layout(self.descriptor_set_layout, None);
        device
            .loader()
            .destroy_descriptor_pool(self.descriptor_pool, None);
        self.vertex_module.destroy(device);
        self.fragment_module.destroy(device);
    }
}
