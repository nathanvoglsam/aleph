//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use vulkan::core::erupt::vk1_0::{
    DescriptorPool, DescriptorPoolCreateFlags, DescriptorPoolCreateInfoBuilder,
    DescriptorPoolSizeBuilder, DescriptorSet, DescriptorSetAllocateInfoBuilder,
    DescriptorSetLayout, DescriptorType, Vk10DeviceLoaderExt,
};
use vulkan::pipeline_layout::{PipelineLayout, PipelineLayoutBuilder};
use vulkan::shader::ShaderModule;

///
/// A struct to wrap resources that are created and destroyed once during the Imgui renderer's
/// lifecycle
///
pub struct ImguiGlobal {
    pub vertex_module: ShaderModule,
    pub fragment_module: ShaderModule,
    pub descriptor_pool: DescriptorPool,
    pub pipeline_layout: PipelineLayout,
    pub descriptor_set: DescriptorSet,
}

impl ImguiGlobal {
    pub fn init(device: &vulkan::core::Device) -> Self {
        let (vertex_module, fragment_module) = Self::create_shader_modules(device);
        let descriptor_pool = Self::create_descriptor_pool(device);
        let pipeline_layout =
            Self::create_pipeline_layout(device, &fragment_module, &vertex_module);
        let descriptor_set = Self::allocate_descriptor_set(
            device,
            pipeline_layout.set_layouts()[0],
            descriptor_pool,
        );

        Self {
            vertex_module,
            fragment_module,
            descriptor_pool,
            pipeline_layout,
            descriptor_set,
        }
    }

    pub fn create_descriptor_pool(device: &vulkan::core::Device) -> DescriptorPool {
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

    pub fn create_pipeline_layout(
        device: &vulkan::core::Device,
        fragment_module: &ShaderModule,
        vertex_module: &ShaderModule,
    ) -> PipelineLayout {
        let pipeline_layout = PipelineLayoutBuilder::new()
            .modules(&[fragment_module, vertex_module])
            .build(device)
            .expect("Failed to create pipeline layout");
        assert_eq!(
            pipeline_layout.set_layouts().len(),
            1,
            "There should only be a single set layout"
        );
        pipeline_layout
    }

    pub fn allocate_descriptor_set(
        device: &vulkan::core::Device,
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

    pub fn create_shader_modules(device: &vulkan::core::Device) -> (ShaderModule, ShaderModule) {
        let (_, words) = vulkan::embedded::data::shaders::imgui_vert_shader();
        let vertex_module = ShaderModule::builder()
            .reflect(true)
            .compile(true)
            .words(words)
            .vertex()
            .build(Some(device))
            .expect("Failed to create imgui vertex module");

        let (_, words) = vulkan::embedded::data::shaders::imgui_frag_shader();
        let fragment_module = ShaderModule::builder()
            .reflect(true)
            .compile(true)
            .words(words)
            .fragment()
            .build(Some(device))
            .expect("Failed to create imgui fragment module");

        (vertex_module, fragment_module)
    }

    pub unsafe fn destroy(&self, device: &vulkan::core::Device) {
        device
            .loader()
            .free_descriptor_sets(self.descriptor_pool, &[self.descriptor_set])
            .expect("Failed to free descriptor set");
        self.pipeline_layout.destroy(device);
        device
            .loader()
            .destroy_descriptor_pool(self.descriptor_pool, None);
        self.vertex_module.destroy(device);
        self.fragment_module.destroy(device);
    }
}
