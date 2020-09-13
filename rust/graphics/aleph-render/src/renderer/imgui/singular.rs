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

use super::ImguiGlobal;
use aleph_vulkan::pipeline::{
    ColorBlendAttachmentState, ColorBlendState, DepthState, DynamicPipelineState,
    GraphicsPipelineBuilder, InputAssemblyState, MultiSampleState, RasterizationState,
    VertexInputState, ViewportState,
};
use aleph_vulkan::render_pass::AttachmentReference;
use aleph_vulkan::shader::ShaderModule;
use aleph_vulkan_core::erupt::vk1_0::{
    AccessFlags, AttachmentLoadOp, AttachmentStoreOp, Format, FrontFace, ImageLayout, Pipeline,
    PipelineBindPoint, PipelineLayout, PipelineStageFlags, PolygonMode, PrimitiveTopology,
    RenderPass, RenderPassCreateInfoBuilder, SubpassDependencyBuilder, SubpassDescriptionBuilder,
    VertexInputAttributeDescriptionBuilder, VertexInputBindingDescriptionBuilder, VertexInputRate,
    Vk10DeviceLoaderExt, SUBPASS_EXTERNAL,
};
use aleph_vulkan_core::{DebugName, SwapImage};
use std::ffi::CString;

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
            ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
            ImageLayout::PRESENT_SRC_KHR,
            AttachmentLoadOp::LOAD,
            AttachmentStoreOp::STORE,
        );

        let color_attachments = [AttachmentReference::color(0)];
        let subpass = SubpassDescriptionBuilder::new()
            .pipeline_bind_point(PipelineBindPoint::GRAPHICS)
            .color_attachments(&color_attachments);

        let dependency = SubpassDependencyBuilder::new()
            .src_subpass(SUBPASS_EXTERNAL)
            .dst_subpass(0)
            .src_access_mask(AccessFlags::COLOR_ATTACHMENT_WRITE)
            .dst_access_mask(AccessFlags::COLOR_ATTACHMENT_WRITE)
            .src_stage_mask(PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
            .dst_stage_mask(PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT);

        let attachments = [attachment];
        let subpasses = [subpass];
        let dependencies = [dependency];
        let create_info = RenderPassCreateInfoBuilder::new()
            .attachments(&attachments)
            .subpasses(&subpasses)
            .dependencies(&dependencies);
        unsafe {
            let render_pass = device
                .loader()
                .create_render_pass(&create_info, None, None)
                .expect("Failed to create render pass");

            let name = format!("{}::RenderPass", module_path!());
            let name = CString::new(name).unwrap();
            render_pass.add_debug_name(device, &name);

            render_pass
        }
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
            .debug_name(aleph_macros::cstr!(concat!(module_path!(), "::Pipeline")))
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
