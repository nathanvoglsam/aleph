//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use crate::pipeline_cache::PipelineCache;
use aleph_vulkan_core::erupt::utils::VulkanResult;
use aleph_vulkan_core::erupt::vk1_0::{
    GraphicsPipelineCreateInfoBuilder, Pipeline, PipelineColorBlendStateCreateInfo,
    PipelineCreateFlags, PipelineDepthStencilStateCreateInfo, PipelineDynamicStateCreateInfo,
    PipelineInputAssemblyStateCreateInfo, PipelineLayout, PipelineMultisampleStateCreateInfo,
    PipelineRasterizationStateCreateInfo, PipelineShaderStageCreateInfoBuilder,
    PipelineTessellationStateCreateInfo, PipelineVertexInputStateCreateInfo,
    PipelineViewportStateCreateInfo, RenderPass,
};
use aleph_vulkan_core::Device;
use std::ops::Deref;

pub struct GraphicsPipelineBuilder<'a> {
    inner: GraphicsPipelineCreateInfoBuilder<'a>,
}

impl<'a> GraphicsPipelineBuilder<'a> {
    ///
    /// Creates a new builder
    ///
    #[inline]
    pub fn new() -> GraphicsPipelineBuilder<'a> {
        Self {
            inner: GraphicsPipelineCreateInfoBuilder::new(),
        }
    }

    #[inline]
    pub fn flags(mut self, flags: PipelineCreateFlags) -> Self {
        self.inner = self.inner.flags(flags);
        self
    }

    #[inline]
    pub fn stages(mut self, stages: &'a [PipelineShaderStageCreateInfoBuilder]) -> Self {
        self.inner = self.inner.stages(stages);
        self
    }

    #[inline]
    pub fn vertex_input_state(
        mut self,
        vertex_input_state: &'a PipelineVertexInputStateCreateInfo,
    ) -> Self {
        self.inner = self.inner.vertex_input_state(vertex_input_state);
        self
    }

    #[inline]
    pub fn input_assembly_state(
        mut self,
        input_assembly_state: &'a PipelineInputAssemblyStateCreateInfo,
    ) -> Self {
        self.inner = self.inner.input_assembly_state(input_assembly_state);
        self
    }

    #[inline]
    pub fn tessellation_state(
        mut self,
        tessellation_state: &'a PipelineTessellationStateCreateInfo,
    ) -> Self {
        self.inner = self.inner.tessellation_state(tessellation_state);
        self
    }

    #[inline]
    pub fn viewport_state(mut self, viewport_state: &'a PipelineViewportStateCreateInfo) -> Self {
        self.inner = self.inner.viewport_state(viewport_state);
        self
    }

    #[inline]
    pub fn rasterization_state(
        mut self,
        rasterization_state: &'a PipelineRasterizationStateCreateInfo,
    ) -> Self {
        self.inner = self.inner.rasterization_state(rasterization_state);
        self
    }

    #[inline]
    pub fn multisample_state(
        mut self,
        multisample_state: &'a PipelineMultisampleStateCreateInfo,
    ) -> Self {
        self.inner = self.inner.multisample_state(multisample_state);
        self
    }

    #[inline]
    pub fn depth_stencil_state(
        mut self,
        depth_stencil_state: &'a PipelineDepthStencilStateCreateInfo,
    ) -> Self {
        self.inner = self.inner.depth_stencil_state(depth_stencil_state);
        self
    }

    #[inline]
    pub fn color_blend_state(
        mut self,
        color_blend_state: &'a PipelineColorBlendStateCreateInfo,
    ) -> Self {
        self.inner = self.inner.color_blend_state(color_blend_state);
        self
    }

    #[inline]
    pub fn dynamic_state(mut self, dynamic_state: &'a PipelineDynamicStateCreateInfo) -> Self {
        self.inner = self.inner.dynamic_state(dynamic_state);
        self
    }

    #[inline]
    pub fn layout(mut self, layout: PipelineLayout) -> Self {
        self.inner = self.inner.layout(layout);
        self
    }

    #[inline]
    pub fn render_pass(mut self, render_pass: RenderPass) -> Self {
        self.inner = self.inner.render_pass(render_pass);
        self
    }

    #[inline]
    pub fn subpass(mut self, subpass: u32) -> Self {
        self.inner = self.inner.subpass(subpass);
        self
    }

    #[inline]
    pub fn base_pipeline_handle(mut self, base_pipeline_handle: Pipeline) -> Self {
        self.inner = self.inner.base_pipeline_handle(base_pipeline_handle);
        self
    }

    #[inline]
    pub fn base_pipeline_index(mut self, base_pipeline_index: i32) -> Self {
        self.inner = self.inner.base_pipeline_index(base_pipeline_index);
        self
    }

    ///
    /// Builds the pipeline, consuming the builder
    ///
    pub fn build(self, device: &Device) -> VulkanResult<Pipeline> {
        unsafe {
            // Get the `create_graphics_pipelines` function
            let function = device
                .loader()
                .vk1_0
                .as_ref()
                .unwrap()
                .create_graphics_pipelines
                .unwrap();

            // Create a spot for the function to write our pipeline handle to
            let mut pipeline = Pipeline::null();

            // Call the function to create a single pipeline
            let raw = function(
                device.loader().handle,
                PipelineCache::get(),
                1,
                self.inner.deref() as *const _,
                std::ptr::null(),
                &mut pipeline as *mut _,
            );

            // Wrap the raw return value
            VulkanResult::new(raw, pipeline)
        }
    }
}
