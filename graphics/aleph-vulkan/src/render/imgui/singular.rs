//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use super::ImguiGlobal;
use crate::pipeline::{
    ColorBlendAttachmentState, ColorBlendState, DepthState, DynamicPipelineState,
    InputAssemblyState, MultiSampleState, RasterizationState, VertexInputState, ViewportState,
};
use crate::pipeline_cache::PipelineCache;
use crate::shader::ShaderModule;
use vulkan_core::erupt::vk1_0::{
    AttachmentDescriptionBuilder, AttachmentLoadOp, AttachmentReferenceBuilder, AttachmentStoreOp,
    DynamicState, Format, FrontFace, GraphicsPipelineCreateInfoBuilder, ImageLayout, Pipeline,
    PipelineBindPoint, PipelineLayout, PolygonMode, PrimitiveTopology, RenderPass,
    RenderPassCreateInfoBuilder, SampleCountFlagBits, SubpassDescriptionBuilder,
    VertexInputAttributeDescriptionBuilder, VertexInputBindingDescriptionBuilder, VertexInputRate,
    Vk10DeviceLoaderExt,
};

///
/// This represents resources where only one is needed, but they need to be recreated when the
/// swapchain is rebuilt
///
pub struct ImguiSingular {
    pub render_pass: RenderPass,
    pub pipeline: Pipeline,
}

impl ImguiSingular {
    pub fn init(device: &vulkan_core::Device, global: &ImguiGlobal, format: Format) -> Self {
        let render_pass = Self::create_render_pass(device, format);
        let pipeline = Self::create_pipeline(
            device,
            global.pipeline_layout.pipeline_layout(),
            render_pass,
            &global.vertex_module,
            &global.fragment_module,
        );

        ImguiSingular {
            render_pass,
            pipeline,
        }
    }

    pub fn create_render_pass(device: &vulkan_core::Device, format: Format) -> RenderPass {
        let attachment = AttachmentDescriptionBuilder::new()
            .format(format)
            .samples(SampleCountFlagBits::_1)
            .load_op(AttachmentLoadOp::CLEAR)
            .store_op(AttachmentStoreOp::STORE)
            .stencil_load_op(AttachmentLoadOp::DONT_CARE)
            .stencil_store_op(AttachmentStoreOp::DONT_CARE)
            .initial_layout(ImageLayout::UNDEFINED)
            .final_layout(ImageLayout::PRESENT_SRC_KHR);

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
        device: &vulkan_core::Device,
        pipeline_layout: PipelineLayout,
        render_pass: RenderPass,
        vertex_module: &ShaderModule,
        fragment_module: &ShaderModule,
    ) -> Pipeline {
        assert!(vertex_module.is_vertex_shader());
        assert!(fragment_module.is_fragment_shader());

        let stages = [
            vertex_module.pipeline_shader_stage().unwrap(),
            fragment_module.pipeline_shader_stage().unwrap(),
        ];

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
        let vertex_input = VertexInputState::new(&bindings, &attributes);

        // Check the vertex shader is getting the right input
        vertex_module
            .vertex_layout()
            .unwrap()
            .is_layout_compatible(&vertex_input)
            .expect("Specified vertex format not compatible with vertex shader");

        let input_assembly =
            InputAssemblyState::no_primitive_restart(PrimitiveTopology::TRIANGLE_LIST);

        let viewport = ViewportState::dynamic(1, 1);

        let rasterization =
            RasterizationState::unculled(PolygonMode::FILL, FrontFace::COUNTER_CLOCKWISE);

        let multisample = MultiSampleState::disabled();

        let depth_stencil = DepthState::disabled();

        let attachments = [ColorBlendAttachmentState::alpha_blending()];
        let color_blend = ColorBlendState::attachments(&attachments);

        let dynamic_states = [DynamicState::VIEWPORT, DynamicState::SCISSOR];
        let dynamic_state = DynamicPipelineState::states(&dynamic_states);

        let create_info = GraphicsPipelineCreateInfoBuilder::new()
            .layout(pipeline_layout)
            .render_pass(render_pass)
            .subpass(0)
            .stages(&stages)
            .vertex_input_state(&vertex_input)
            .input_assembly_state(&input_assembly)
            .viewport_state(&viewport)
            .rasterization_state(&rasterization)
            .multisample_state(&multisample)
            .depth_stencil_state(&depth_stencil)
            .color_blend_state(&color_blend)
            .dynamic_state(&dynamic_state);
        unsafe {
            device
                .loader()
                .create_graphics_pipelines(PipelineCache::get(), &[create_info], None)
        }
        .expect("Failed to create pipeline")[0]
    }

    pub unsafe fn destroy(&self, device: &vulkan_core::Device) {
        device.loader().destroy_render_pass(self.render_pass, None);
        device.loader().destroy_pipeline(self.pipeline, None);
    }
}
