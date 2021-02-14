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
            RasterizationState::backface_culled(PolygonMode::FILL, FrontFace::COUNTER_CLOCKWISE);
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
        device.loader().destroy_pipeline(Some(self.pipeline), None);
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
        device.loader().destroy_pipeline(Some(self.pipeline), None);
    }
}
