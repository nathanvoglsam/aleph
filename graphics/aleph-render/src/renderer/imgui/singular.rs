//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use super::ImguiGlobal;
use aleph_vulkan::pipeline::{
    ColorBlendAttachmentState, ColorBlendState, DepthState, DynamicPipelineState,
    GraphicsPipelineBuilder, InputAssemblyState, MultiSampleState, RasterizationState,
    VertexInputState, ViewportState,
};
use aleph_vulkan::shader::ShaderModule;
use aleph_vulkan_core::erupt::vk1_0::{
    AttachmentLoadOp, AttachmentReferenceBuilder, AttachmentStoreOp, Format, FrontFace,
    ImageLayout, Pipeline, PipelineBindPoint, PipelineLayout, PolygonMode, PrimitiveTopology,
    RenderPass, RenderPassCreateInfoBuilder, SubpassDescriptionBuilder,
    VertexInputAttributeDescriptionBuilder, VertexInputBindingDescriptionBuilder, VertexInputRate,
    Vk10DeviceLoaderExt,
};
use aleph_vulkan_core::SwapImage;

///
/// This represents resources where only one is needed, but they need to be recreated when the
/// swapchain is rebuilt
///
pub struct ImguiSingular {
    pub render_pass: RenderPass,
    pub pipeline: Pipeline,
}

impl ImguiSingular {
    pub fn init(
        device: &aleph_vulkan_core::Device,
        global: &ImguiGlobal,
        swap_image: &SwapImage,
    ) -> Self {
        let render_pass = Self::create_render_pass(device, swap_image);
        let pipeline_layout = global.pipeline_layout.pipeline_layout();
        let pipeline = Self::create_pipeline(
            device,
            pipeline_layout,
            render_pass,
            &global.vertex_module,
            &global.fragment_module,
        );

        ImguiSingular {
            render_pass,
            pipeline,
        }
    }

    pub fn create_render_pass(
        device: &aleph_vulkan_core::Device,
        swap_image: &SwapImage,
    ) -> RenderPass {
        let attachment = swap_image.attachment_description(
            ImageLayout::UNDEFINED,
            ImageLayout::PRESENT_SRC_KHR,
            AttachmentLoadOp::CLEAR,
            AttachmentStoreOp::STORE,
        );

        let attachment_reference = AttachmentReferenceBuilder::new()
            .attachment(0)
            .layout(ImageLayout::COLOR_ATTACHMENT_OPTIMAL);
        let color_attachments = [attachment_reference];
        let subpass = SubpassDescriptionBuilder::new()
            .pipeline_bind_point(PipelineBindPoint::GRAPHICS)
            .color_attachments(&color_attachments);

        let attachments = [attachment];
        let subpasses = [subpass];
        let create_info = RenderPassCreateInfoBuilder::new()
            .attachments(&attachments)
            .subpasses(&subpasses);
        unsafe { device.loader().create_render_pass(&create_info, None, None) }
            .expect("Failed to create render pass")
    }

    pub fn create_pipeline(
        device: &aleph_vulkan_core::Device,
        pipeline_layout: PipelineLayout,
        render_pass: RenderPass,
        vertex_module: &ShaderModule,
        fragment_module: &ShaderModule,
    ) -> Pipeline {
        assert!(vertex_module.is_vertex_shader());
        assert!(fragment_module.is_fragment_shader());

        let binding = VertexInputBindingDescriptionBuilder::new()
            .binding(0)
            .input_rate(VertexInputRate::VERTEX)
            .stride(core::mem::size_of::<imgui::DrawVert>() as u32);
        let pos_attr = VertexInputAttributeDescriptionBuilder::new()
            .binding(0)
            .offset(0)
            .location(0)
            .format(Format::R32G32_SFLOAT);
        let uv_attr = VertexInputAttributeDescriptionBuilder::new()
            .binding(0)
            .offset(8)
            .location(1)
            .format(Format::R32G32_SFLOAT);
        let col_attr = VertexInputAttributeDescriptionBuilder::new()
            .binding(0)
            .offset(16)
            .location(2)
            .format(Format::R8G8B8A8_UNORM);
        let bindings = [binding];
        let attributes = [pos_attr, uv_attr, col_attr];

        // Check the vertex shader is getting the right input
        vertex_module
            .vertex_layout()
            .unwrap()
            .is_layout_compatible(&VertexInputState::new(&bindings, &attributes))
            .expect("Specified vertex format not compatible with vertex shader");

        let input_assembly = InputAssemblyState::no_restart(PrimitiveTopology::TRIANGLE_LIST);
        let rasterization =
            RasterizationState::unculled(PolygonMode::FILL, FrontFace::COUNTER_CLOCKWISE);
        let attachments = [ColorBlendAttachmentState::alpha_blending()];
        let vstage = vertex_module.pipeline_shader_stage().unwrap();
        let fstage = fragment_module.pipeline_shader_stage().unwrap();
        let pipeline = GraphicsPipelineBuilder::new()
            .layout(pipeline_layout)
            .render_pass(render_pass)
            .subpass(0)
            .stages(&[vstage, fstage])
            .vertex_input_state(&VertexInputState::new(&bindings, &attributes))
            .input_assembly_state(&input_assembly)
            .viewport_state(&ViewportState::dynamic(1, 1))
            .rasterization_state(&rasterization)
            .multisample_state(&MultiSampleState::disabled())
            .depth_stencil_state(&DepthState::disabled())
            .color_blend_state(&ColorBlendState::attachments(&attachments))
            .dynamic_state(&DynamicPipelineState::viewport_scissor())
            .build(device)
            .expect("Failed to create pipeline");

        pipeline
    }

    pub unsafe fn destroy(&self, device: &aleph_vulkan_core::Device) {
        device.loader().destroy_render_pass(self.render_pass, None);
        device.loader().destroy_pipeline(self.pipeline, None);
    }
}
