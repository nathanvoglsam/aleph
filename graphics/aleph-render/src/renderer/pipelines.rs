//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use aleph_vulkan::pipeline::{
    ColorBlendState, DepthState, DynamicPipelineState, GraphicsPipelineBuilder, InputAssemblyState,
    MultiSampleState, RasterizationState, VertexInputState, ViewportState,
};
use aleph_vulkan::pipeline_layout::PipelineLayout;
use aleph_vulkan::shader::ShaderModule;
use aleph_vulkan_core::erupt::vk1_0::{
    CompareOp, FrontFace, Pipeline, PolygonMode, PrimitiveTopology, RenderPass, Vk10DeviceLoaderExt,
};
use aleph_vulkan_core::Device;

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

        // Fill out the list of bindings and attributes for compatibility with a static mesh
        let mut bindings = Vec::new();
        let mut attributes = Vec::new();
        VertexInputState::for_static_mesh(&mut bindings, &mut attributes);

        let input_assembly_state = InputAssemblyState::no_restart(PrimitiveTopology::TRIANGLE_LIST);
        let rasterization_state =
            RasterizationState::backface_culled(PolygonMode::FILL, FrontFace::CLOCKWISE);
        let vstage = vert_module.pipeline_shader_stage().unwrap();
        let fstage = frag_module.pipeline_shader_stage().unwrap();
        let pipeline = GraphicsPipelineBuilder::new()
            .debug_name(aleph_macros::cstr!(concat!(
                module_path!(),
                "::GeomPipeline"
            )))
            .layout(pipeline_layout.pipeline_layout())
            .render_pass(render_pass)
            .subpass(0)
            .color_blend_state(&ColorBlendState::disabled(1))
            .depth_stencil_state(&DepthState::enabled(true, CompareOp::LESS))
            .dynamic_state(&DynamicPipelineState::viewport_scissor())
            .input_assembly_state(&input_assembly_state)
            .multisample_state(&MultiSampleState::disabled())
            .rasterization_state(&rasterization_state)
            .stages(&[vstage, fstage])
            .viewport_state(&ViewportState::dynamic(1, 1))
            .vertex_input_state(&VertexInputState::new(&bindings, &attributes))
            .build(device)
            .expect("Failed to create geometry pipeline");

        Self { pipeline }
    }

    ///
    /// Gets the underlying pipeline handle
    ///
    pub fn pipeline(&self) -> Pipeline {
        self.pipeline
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

        // Fill out the list of bindings and attributes for compatibility with a static mesh
        let mut bindings = Vec::new();
        let mut attributes = Vec::new();
        VertexInputState::for_fullscreen_quad(&mut bindings, &mut attributes);

        let input_assembly_state = InputAssemblyState::no_restart(PrimitiveTopology::TRIANGLE_LIST);
        let rasterization_state =
            RasterizationState::unculled(PolygonMode::FILL, FrontFace::CLOCKWISE);
        let vstage = vert_module.pipeline_shader_stage().unwrap();
        let fstage = frag_module.pipeline_shader_stage().unwrap();
        let pipeline = GraphicsPipelineBuilder::new()
            .debug_name(aleph_macros::cstr!(concat!(
                module_path!(),
                "::TonePipeline"
            )))
            .layout(pipeline_layout.pipeline_layout())
            .render_pass(render_pass)
            .subpass(1)
            .color_blend_state(&ColorBlendState::disabled(1))
            .depth_stencil_state(&DepthState::disabled())
            .dynamic_state(&DynamicPipelineState::viewport_scissor())
            .input_assembly_state(&input_assembly_state)
            .multisample_state(&MultiSampleState::disabled())
            .rasterization_state(&rasterization_state)
            .stages(&[vstage, fstage])
            .viewport_state(&ViewportState::dynamic(1, 1))
            .vertex_input_state(&VertexInputState::new(&bindings, &attributes))
            .build(device)
            .expect("Failed to create tonemapping pipeline");

        Self { pipeline }
    }

    ///
    /// Gets the underlying pipeline handle
    ///
    pub fn pipeline(&self) -> Pipeline {
        self.pipeline
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
