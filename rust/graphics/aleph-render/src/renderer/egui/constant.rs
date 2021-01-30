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

use aleph_vulkan::pipeline_layout::{PipelineLayout, PipelineLayoutBuilder};
use aleph_vulkan::shader::ShaderModule;
use aleph_vulkan_core::erupt::vk1_0::{
    DescriptorPool, DescriptorPoolCreateFlags, DescriptorPoolCreateInfoBuilder,
    DescriptorPoolSizeBuilder, DescriptorType,
};
use aleph_vulkan_core::DebugName;
use std::ffi::CString;

///
/// A struct to wrap resources that are created and destroyed once during the Imgui renderer's
/// lifecycle
///
pub struct ConstantObjects {
    pub vertex_module: ShaderModule,
    pub fragment_module: ShaderModule,
    pub descriptor_pool: DescriptorPool,
    pub pipeline_layout: PipelineLayout,
}

impl ConstantObjects {
    pub fn init(device: &aleph_vulkan_core::Device) -> Self {
        let (vertex_module, fragment_module) = Self::create_shader_modules(device);
        let descriptor_pool = Self::create_descriptor_pool(device);
        let pipeline_layout =
            Self::create_pipeline_layout(device, &fragment_module, &vertex_module);

        Self {
            vertex_module,
            fragment_module,
            descriptor_pool,
            pipeline_layout,
        }
    }

    pub fn create_descriptor_pool(device: &aleph_vulkan_core::Device) -> DescriptorPool {
        let pool_sizes = [
            DescriptorPoolSizeBuilder::new()
                ._type(DescriptorType::SAMPLER)
                .descriptor_count(16),
            DescriptorPoolSizeBuilder::new()
                ._type(DescriptorType::SAMPLED_IMAGE)
                .descriptor_count(16),
            DescriptorPoolSizeBuilder::new()
                ._type(DescriptorType::COMBINED_IMAGE_SAMPLER)
                .descriptor_count(16),
        ];
        let create_info = DescriptorPoolCreateInfoBuilder::new()
            .flags(DescriptorPoolCreateFlags::FREE_DESCRIPTOR_SET)
            .max_sets(16)
            .pool_sizes(&pool_sizes);
        unsafe {
            let descriptor_pool = device
                .loader()
                .create_descriptor_pool(&create_info, None, None)
                .expect("Failed to create descriptor pool");

            let name = format!("{}::DescriptorPool", module_path!());
            let name = CString::new(name).unwrap();
            descriptor_pool.add_debug_name(device, &name);

            descriptor_pool
        }
    }

    pub fn create_pipeline_layout(
        device: &aleph_vulkan_core::Device,
        fragment_module: &ShaderModule,
        vertex_module: &ShaderModule,
    ) -> PipelineLayout {
        let pipeline_layout = PipelineLayoutBuilder::new()
            .modules(&[(fragment_module, None), (vertex_module, None)])
            .debug_name(aleph_macros::cstr!(concat!(
                module_path!(),
                "::PipelineLayout"
            )))
            .build(device)
            .expect("Failed to create pipeline layout");
        assert_eq!(
            pipeline_layout.set_layouts().len(),
            1,
            "There should only be a single set layout"
        );
        pipeline_layout
    }

    pub fn create_shader_modules(
        device: &aleph_vulkan_core::Device,
    ) -> (ShaderModule, ShaderModule) {
        let (_, words) = aleph_vulkan::embedded::data::shaders::egui_vert_shader();
        let vertex_module = ShaderModule::builder()
            .reflect(true)
            .compile(true)
            .words(words)
            .vertex()
            .debug_name(aleph_macros::cstr!(concat!(
                module_path!(),
                "::VertexShaderModule"
            )))
            .build(Some(device))
            .expect("Failed to create egui vertex module");

        let (_, words) = aleph_vulkan::embedded::data::shaders::egui_frag_shader();
        let fragment_module = ShaderModule::builder()
            .reflect(true)
            .compile(true)
            .words(words)
            .fragment()
            .debug_name(aleph_macros::cstr!(concat!(
                module_path!(),
                "::FragShaderModule"
            )))
            .build(Some(device))
            .expect("Failed to create egui fragment module");

        (vertex_module, fragment_module)
    }

    pub unsafe fn destroy(&self, device: &aleph_vulkan_core::Device) {
        self.pipeline_layout.destroy(device);
        device
            .loader()
            .destroy_descriptor_pool(Some(self.descriptor_pool), None);
        self.vertex_module.destroy(device);
        self.fragment_module.destroy(device);
    }
}
