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

//! A bunch of utility structs and functions that provide a generalized framework for issuing
//! full-screen-triangle draws. Typcially used for fullscreen pass style shaders that are
//! implemented as pixel shaders instead of compute.

use aleph_nstr::nstr;

pub struct FullscreenTriangleInfo<'a> {
    pub dst_view: rhi::ImageView,
    pub pipeline: &'a rhi::GraphicsPipelineHandle,
    pub extent: rhi::Extent2D,
    pub load_op: rhi::AttachmentLoadOp<rhi::ColorClearValue>,
    pub bindings: &'a FullscreenTriangleBindInfo<'a>,
}

pub struct FullscreenTriangleBindInfo<'a> {
    pub binding_signature: &'a dyn rhi::IBindingSignature,
    pub blocks: &'a [rhi::ParameterBlockHandle],
    pub first_blocks: u32,
    pub constant_block: Option<&'a [u8]>,
}

pub unsafe fn draw_fullscreen_triangle(
    encoder: &mut rhi::CommandEncoder,
    info: &FullscreenTriangleInfo,
) {
    unsafe {
        let mut render = encoder.begin_rendering(
            &rhi::BeginRenderingInfo {
                layer_count: 1,
                extent: info.extent,
                color_attachments: &[rhi::RenderingColorAttachmentInfo {
                    image_view: info.dst_view,
                    image_layout: rhi::ImageLayout::ColorAttachment,
                    load_op: info.load_op.clone(), // We write the whole texture
                    store_op: rhi::AttachmentStoreOp::Store,
                }],
                depth_stencil_attachment: None,
                allow_uav_writes: false,
            },
            nstr!("DrawFullscreenTriangle::render_pass"),
        );
        render.bind_graphics_pipeline(info.pipeline);
        render.set_viewports(&[rhi::Viewport {
            x: 0.0,
            y: 0.0,
            width: info.extent.width as _,
            height: info.extent.height as _,
            min_depth: 0.0,
            max_depth: 1.0,
        }]);
        render.set_scissor_rects(&[rhi::Rect {
            x: 0,
            y: 0,
            w: info.extent.width,
            h: info.extent.height,
        }]);

        render.bind_parameter_blocks(
            info.bindings.binding_signature,
            info.bindings.first_blocks,
            info.bindings.blocks,
        );

        if let Some(data) = info.bindings.constant_block {
            render.set_push_constant_block(data);
        }

        render.draw(3, 1, 0, 0);
    }
}

pub fn create_fullscreen_triangle_pipeline(
    device: &dyn rhi::IDevice,
    binding_signature: &dyn rhi::IBindingSignature,
    format: rhi::Format,
    vertex_shader: &dyn rhi::IShaderCodeSource,
    fragment_shader: &dyn rhi::IShaderCodeSource,
    name: Option<&str>,
) -> Result<rhi::GraphicsPipelineHandle, rhi::PipelineCreateError> {
    let vertex_layout = rhi::VertexInputStateDesc::default();

    let input_assembly_state = rhi::InputAssemblyStateDesc {
        primitive_topology: rhi::PrimitiveTopology::TriangleList,
    };

    let rasterizer_state = rhi::RasterizerStateDesc {
        cull_mode: rhi::CullMode::None,
        front_face: rhi::FrontFaceOrder::CounterClockwise,
        polygon_mode: rhi::PolygonMode::Fill,
        depth_bias: 0,
        depth_bias_clamp: 0.0,
        depth_bias_slope_factor: 0.0,
    };

    let depth_stencil_state = rhi::DepthStencilStateDesc {
        depth_test: false,
        ..Default::default()
    };

    let blend_state_new = rhi::BlendStateDesc {
        attachments: &[rhi::AttachmentBlendState {
            blend_enabled: false,
            color_write_mask: rhi::ColorComponentFlags::all(),
            ..Default::default()
        }],
    };

    let graphics_pipeline_desc_new = rhi::GraphicsPipelineDesc {
        shader_stages: &[vertex_shader, fragment_shader],
        binding_signature,
        vertex_layout: &vertex_layout,
        input_assembly_state: &input_assembly_state,
        rasterizer_state: &rasterizer_state,
        depth_stencil_state: &depth_stencil_state,
        blend_state: &blend_state_new,
        render_target_formats: &[format],
        depth_stencil_format: None,
        name,
    };

    device.create_graphics_pipeline(&graphics_pipeline_desc_new)
}
