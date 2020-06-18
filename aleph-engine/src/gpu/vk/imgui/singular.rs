//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use crate::gpu::vk;
use crate::gpu::vk::imgui::ImguiGlobal;
use crate::gpu::vk::pipeline::{DepthState, MultiSampleState, RasterizationState, ShaderStage};
use crate::gpu::vk::PipelineCache;
use erupt::vk1_0::{
    AttachmentDescriptionBuilder, AttachmentLoadOp, AttachmentReferenceBuilder, AttachmentStoreOp,
    BlendFactor, BlendOp, ColorComponentFlags, DescriptorSetLayout, DynamicState, Format,
    FrontFace, GraphicsPipelineCreateInfoBuilder, ImageLayout, Pipeline, PipelineBindPoint,
    PipelineColorBlendAttachmentStateBuilder, PipelineColorBlendStateCreateInfoBuilder,
    PipelineDynamicStateCreateInfoBuilder, PipelineInputAssemblyStateCreateInfoBuilder,
    PipelineLayout, PipelineLayoutCreateInfoBuilder, PipelineVertexInputStateCreateInfoBuilder,
    PipelineViewportStateCreateInfoBuilder, PolygonMode, PrimitiveTopology,
    PushConstantRangeBuilder, RenderPass, RenderPassCreateInfoBuilder, SampleCountFlagBits,
    ShaderModule, ShaderStageFlags, SubpassDescriptionBuilder,
    VertexInputAttributeDescriptionBuilder, VertexInputBindingDescriptionBuilder, VertexInputRate,
    Vk10DeviceLoaderExt,
};

///
/// This represents resources where only one is needed, but they need to be recreated when the
/// swapchain is rebuilt
///
pub struct ImguiSingular {
    pub render_pass: RenderPass,
    pub pipeline_layout: PipelineLayout,
    pub pipeline: Pipeline,
}

impl ImguiSingular {
    pub fn init(device: &vk::Device, global: &ImguiGlobal, format: Format) -> Self {
        let render_pass = Self::create_render_pass(device, format);
        let pipeline_layout = Self::create_pipeline_layout(device, global.descriptor_set_layout);
        let pipeline = Self::create_pipeline(
            device,
            pipeline_layout,
            render_pass,
            global.vertex_module,
            global.fragment_module,
        );

        ImguiSingular {
            render_pass,
            pipeline_layout,
            pipeline,
        }
    }

    pub fn create_pipeline_layout(
        device: &vk::Device,
        layout: DescriptorSetLayout,
    ) -> PipelineLayout {
        let set_layouts = [layout];
        let ranges = [PushConstantRangeBuilder::new()
            .stage_flags(ShaderStageFlags::VERTEX)
            .offset(0)
            .size(4 * 4)];
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

    pub fn create_render_pass(device: &vk::Device, format: Format) -> RenderPass {
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
        device: &vk::Device,
        pipeline_layout: PipelineLayout,
        render_pass: RenderPass,
        vertex_module: ShaderModule,
        fragment_module: ShaderModule,
    ) -> Pipeline {
        let vertex_stage = ShaderStage::vertex(vertex_module);
        let fragment_stage = ShaderStage::fragment(fragment_module);
        let stages = [vertex_stage, fragment_stage];

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
        let vertex_input = PipelineVertexInputStateCreateInfoBuilder::new()
            .vertex_binding_descriptions(&bindings)
            .vertex_attribute_descriptions(&attributes);

        let input_assembly = PipelineInputAssemblyStateCreateInfoBuilder::new()
            .topology(PrimitiveTopology::TRIANGLE_LIST);

        let viewport = PipelineViewportStateCreateInfoBuilder::new()
            .viewport_count(1)
            .scissor_count(1);

        let rasterization =
            RasterizationState::unculled(PolygonMode::FILL, FrontFace::COUNTER_CLOCKWISE);

        let multisample = MultiSampleState::disabled();

        let depth_stencil = DepthState::disabled();

        let color_blend = PipelineColorBlendAttachmentStateBuilder::new()
            .blend_enable(true)
            .src_color_blend_factor(BlendFactor::SRC_ALPHA)
            .dst_color_blend_factor(BlendFactor::ONE_MINUS_SRC_ALPHA)
            .color_blend_op(BlendOp::ADD)
            .src_alpha_blend_factor(BlendFactor::ONE_MINUS_SRC_ALPHA)
            .dst_alpha_blend_factor(BlendFactor::ZERO)
            .alpha_blend_op(BlendOp::ADD)
            .color_write_mask(
                ColorComponentFlags::R
                    | ColorComponentFlags::G
                    | ColorComponentFlags::B
                    | ColorComponentFlags::A,
            );
        let attachments = [color_blend];
        let color_blend = PipelineColorBlendStateCreateInfoBuilder::new().attachments(&attachments);

        let dynamic_states = [DynamicState::VIEWPORT, DynamicState::SCISSOR];
        let dynamic = PipelineDynamicStateCreateInfoBuilder::new().dynamic_states(&dynamic_states);

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
            .dynamic_state(&dynamic);
        unsafe {
            device
                .loader()
                .create_graphics_pipelines(PipelineCache::get(), &[create_info], None)
        }
        .expect("Failed to create pipeline")[0]
    }

    pub unsafe fn destroy(&self, device: &vk::Device) {
        device.loader().destroy_render_pass(self.render_pass, None);
        device.loader().destroy_pipeline(self.pipeline, None);
        device
            .loader()
            .destroy_pipeline_layout(self.pipeline_layout, None);
    }
}
