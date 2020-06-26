//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use crate::pipeline::{
    ColorBlendAttachmentState, ColorBlendState, DepthState, DynamicPipelineState,
    InputAssemblyState, MultiSampleState, RasterizationState, VertexInputState, ViewportState,
};
use crate::pipeline_cache::PipelineCache;
use crate::pipeline_layout::PipelineLayout;
use crate::shader::ShaderModule;
use vulkan_core::erupt::vk1_0::{
    CompareOp, DynamicState, FrontFace, GraphicsPipelineCreateInfoBuilder, Pipeline, PolygonMode,
    PrimitiveTopology, RenderPass, Vk10DeviceLoaderExt,
};
use vulkan_core::Device;

///
/// The pipeline state object for the geometry pass
///
pub struct GeometryPipeline {
    pipeline: Pipeline,
}

impl GeometryPipeline {
    pub fn new(
        device: &Device,
        pipeline_layout: &PipelineLayout,
        render_pass: RenderPass,
        vert_module: &ShaderModule,
        frag_module: &ShaderModule,
    ) -> Self {
        assert!(vert_module.is_vertex_shader());
        assert!(frag_module.is_fragment_shader());

        let color_blend_attachments = [ColorBlendAttachmentState::disabled()];
        let color_blend_state = ColorBlendState::attachments(&color_blend_attachments);

        let depth_stencil_state = DepthState::enabled(true, CompareOp::LESS);

        let dynamic_states = [DynamicState::VIEWPORT, DynamicState::SCISSOR];
        let dynamic_state = DynamicPipelineState::states(&dynamic_states);
        let viewport_state = ViewportState::dynamic(1, 1);

        let input_assembly_state =
            InputAssemblyState::no_primitive_restart(PrimitiveTopology::TRIANGLE_LIST);

        let multisample_state = MultiSampleState::disabled();

        let rasterization_stage =
            RasterizationState::backface_culled(PolygonMode::FILL, FrontFace::COUNTER_CLOCKWISE);

        let stages = [
            vert_module.pipeline_shader_stage().unwrap(),
            frag_module.pipeline_shader_stage().unwrap(),
        ];

        let mut bindings = Vec::new();
        let mut attributes = Vec::new();
        VertexInputState::for_static_mesh(&mut bindings, &mut attributes);
        let vertex_input_state = VertexInputState::new(&bindings, &attributes);

        let create_info = GraphicsPipelineCreateInfoBuilder::new()
            .layout(pipeline_layout.pipeline_layout())
            .render_pass(render_pass)
            .subpass(0)
            .color_blend_state(&color_blend_state)
            .depth_stencil_state(&depth_stencil_state)
            .dynamic_state(&dynamic_state)
            .input_assembly_state(&input_assembly_state)
            .multisample_state(&multisample_state)
            .rasterization_state(&rasterization_stage)
            .stages(&stages)
            .viewport_state(&viewport_state)
            .vertex_input_state(&vertex_input_state);

        let pipeline = unsafe {
            device
                .loader()
                .create_graphics_pipelines(PipelineCache::get(), &[create_info], None)
                .expect("Failed to create geometry pipeline")[0]
        };

        Self { pipeline }
    }

    ///
    /// Destroys the pipeline state object.
    ///
    /// Unsafe as the destroy is not synchronized
    ///
    pub unsafe fn destroy(&self, device: &Device) {
        device.loader().destroy_pipeline(self.pipeline, None);
    }
}

///
/// The pipeline state object for the geometry pass
///
pub struct TonePipeline {
    pipeline: Pipeline,
}

impl TonePipeline {
    pub fn new(
        device: &Device,
        pipeline_layout: &PipelineLayout,
        render_pass: RenderPass,
        vert_module: &ShaderModule,
        frag_module: &ShaderModule,
    ) -> Self {
        assert!(vert_module.is_vertex_shader());
        assert!(frag_module.is_fragment_shader());

        let color_blend_attachments = [ColorBlendAttachmentState::disabled()];
        let color_blend_state = ColorBlendState::attachments(&color_blend_attachments);

        let depth_stencil_state = DepthState::enabled(true, CompareOp::LESS);

        let dynamic_states = [DynamicState::VIEWPORT, DynamicState::SCISSOR];
        let dynamic_state = DynamicPipelineState::states(&dynamic_states);
        let viewport_state = ViewportState::dynamic(1, 1);

        let input_assembly_state =
            InputAssemblyState::no_primitive_restart(PrimitiveTopology::TRIANGLE_LIST);

        let multisample_state = MultiSampleState::disabled();

        let rasterization_stage =
            RasterizationState::backface_culled(PolygonMode::FILL, FrontFace::COUNTER_CLOCKWISE);

        let stages = [
            vert_module.pipeline_shader_stage().unwrap(),
            frag_module.pipeline_shader_stage().unwrap(),
        ];

        let mut bindings = Vec::new();
        let mut attributes = Vec::new();
        VertexInputState::for_fullscreen_quad(&mut bindings, &mut attributes);
        let vertex_input_state = VertexInputState::new(&bindings, &attributes);

        let create_info = GraphicsPipelineCreateInfoBuilder::new()
            .layout(pipeline_layout.pipeline_layout())
            .render_pass(render_pass)
            .subpass(1)
            .color_blend_state(&color_blend_state)
            .depth_stencil_state(&depth_stencil_state)
            .dynamic_state(&dynamic_state)
            .input_assembly_state(&input_assembly_state)
            .multisample_state(&multisample_state)
            .rasterization_state(&rasterization_stage)
            .stages(&stages)
            .viewport_state(&viewport_state)
            .vertex_input_state(&vertex_input_state);

        let pipeline = unsafe {
            device
                .loader()
                .create_graphics_pipelines(PipelineCache::get(), &[create_info], None)
                .expect("Failed to create geometry pipeline")[0]
        };

        Self { pipeline }
    }

    ///
    /// Destroys the pipeline state object.
    ///
    /// Unsafe as the destroy is not synchronized
    ///
    pub unsafe fn destroy(&self, device: &Device) {
        device.loader().destroy_pipeline(self.pipeline, None);
    }
}
