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

use aleph_rhi_api::*;

pub struct FullscreenTriangleInfo<'a> {
    pub dst_view: ImageView,
    pub pipeline: &'a GraphicsPipelineHandle,
    pub extent: Extent2D,
    pub load_op: AttachmentLoadOp<ColorClearValue>,
    pub bindings: &'a FullscreenTriangleBindInfo<'a>,
}

pub struct FullscreenTriangleBindInfo<'a> {
    pub layout: &'a PipelineLayoutHandle,
    pub sets: &'a [DescriptorSetHandle],
    pub first_set: u32,
    pub dynamic_offsets: &'a [u32],
    pub constant_blocks: &'a [(usize, &'a [u8])],
}

pub unsafe fn draw_fullscreen_triangle(
    encoder: &mut dyn IGeneralEncoder,
    info: &FullscreenTriangleInfo,
) {
    unsafe {
        encoder.begin_rendering(&BeginRenderingInfo {
            layer_count: 1,
            extent: info.extent,
            color_attachments: &[RenderingColorAttachmentInfo {
                image_view: info.dst_view,
                image_layout: ImageLayout::ColorAttachment,
                load_op: info.load_op.clone(), // We write the whole texture
                store_op: AttachmentStoreOp::Store,
            }],
            depth_stencil_attachment: None,
            allow_uav_writes: false,
        });
        encoder.bind_graphics_pipeline(info.pipeline);
        encoder.set_viewports(&[Viewport {
            x: 0.0,
            y: 0.0,
            width: info.extent.width as _,
            height: info.extent.height as _,
            min_depth: 0.0,
            max_depth: 1.0,
        }]);
        encoder.set_scissor_rects(&[Rect {
            x: 0,
            y: 0,
            w: info.extent.width,
            h: info.extent.height,
        }]);

        encoder.bind_descriptor_sets(
            info.bindings.layout,
            PipelineBindPoint::Graphics,
            info.bindings.first_set,
            info.bindings.sets,
            info.bindings.dynamic_offsets,
        );

        for &(block_index, data) in info.bindings.constant_blocks {
            encoder.set_push_constant_block(block_index, data);
        }

        encoder.draw(3, 1, 0, 0);

        encoder.end_rendering();
    }
}

pub fn create_fullscreen_triangle_pipeline(
    device: &dyn IDevice,
    pipeline_layout: &PipelineLayoutHandle,
    format: Format,
    vertex_shader: ShaderStage,
    fragment_shader: ShaderStage,
    name: Option<&str>,
) -> Result<GraphicsPipelineHandle, PipelineCreateError> {
    let vertex_layout = VertexInputStateDesc::default();

    let input_assembly_state = InputAssemblyStateDesc {
        primitive_topology: PrimitiveTopology::TriangleList,
    };

    let rasterizer_state = RasterizerStateDesc {
        cull_mode: CullMode::None,
        front_face: FrontFaceOrder::CounterClockwise,
        polygon_mode: PolygonMode::Fill,
        depth_bias: 0,
        depth_bias_clamp: 0.0,
        depth_bias_slope_factor: 0.0,
    };

    let depth_stencil_state = DepthStencilStateDesc {
        depth_test: false,
        ..Default::default()
    };

    let blend_state_new = BlendStateDesc {
        attachments: &[AttachmentBlendState {
            blend_enabled: false,
            color_write_mask: ColorComponentFlags::all(),
            ..Default::default()
        }],
    };

    let graphics_pipeline_desc_new = GraphicsPipelineDesc {
        shader_stages: &[vertex_shader, fragment_shader],
        pipeline_layout,
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
