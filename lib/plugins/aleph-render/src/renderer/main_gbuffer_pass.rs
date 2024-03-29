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

use aleph_device_allocators::IUploadAllocator;
use aleph_device_allocators::UploadBumpAllocator;
use aleph_frame_graph::*;
use aleph_interfaces::any::AnyArc;
use aleph_math::projection::perspective_reversed_infinite_z_wgpu_dx_gl;
use aleph_math::Mat4;
use aleph_math::Vec3;
use aleph_pin_board::PinBoard;
use aleph_rhi_api::*;

use crate::renderer::params::BackBufferInfo;
use crate::shader_db_accessor::ShaderDatabaseAccessor;
use crate::shaders;

struct MainGBufferPassPayload {
    gbuffer0: ResourceMut,
    gbuffer1: ResourceMut,
    gbuffer2: ResourceMut,
    depth_buffer: ResourceMut,
    vtx_buffer: AnyArc<dyn IBuffer>,
    idx_buffer: AnyArc<dyn IBuffer>,
    uniform_buffer: ResourceMut,
    descriptor_set_layout: AnyArc<dyn IDescriptorSetLayout>,
    pipeline_layout: AnyArc<dyn IPipelineLayout>,
    pipeline: AnyArc<dyn IGraphicsPipeline>,
}

pub struct MainGBufferPassOutput {
    pub gbuffer0: ResourceMut,
    pub gbuffer1: ResourceMut,
    pub gbuffer2: ResourceMut,
    pub depth_buffer: ResourceMut,
}

pub fn pass(
    frame_graph: &mut FrameGraphBuilder,
    device: &dyn IDevice,
    pin_board: &PinBoard,
    shader_db: &ShaderDatabaseAccessor,
) {
    let desc = BufferDesc::new(4 * 1024 as u64)
        .cpu_write()
        .with_usage(ResourceUsageFlags::VERTEX_BUFFER)
        .with_name("Test Vertex Buffer");
    let vtx_buffer = device.create_buffer(&desc).unwrap();

    let desc = BufferDesc::new(4 * 1024 as u64)
        .cpu_write()
        .with_usage(ResourceUsageFlags::INDEX_BUFFER)
        .with_name("Test Index Buffer");
    let idx_buffer = device.create_buffer(&desc).unwrap();

    unsafe {
        let v_ptr = vtx_buffer.map().unwrap();
        let v_alloc =
            UploadBumpAllocator::new_from_block(vtx_buffer.as_ref(), v_ptr, 0, 4 * 1024).unwrap();
        v_alloc.allocate_objects_clone(&VERTS);
        vtx_buffer.unmap();

        let i_ptr = idx_buffer.map().unwrap();
        let i_alloc =
            UploadBumpAllocator::new_from_block(idx_buffer.as_ref(), i_ptr, 0, 4 * 1024).unwrap();
        i_alloc.allocate_objects_copy(&INDICES);
        idx_buffer.unmap();
    }

    let descriptor_set_layout = create_descriptor_set_layout(device);
    let pipeline_layout = create_root_signature(device, descriptor_set_layout.as_ref());

    let graphics_pipeline = create_pipeline_state(device, pipeline_layout.as_ref(), shader_db);

    frame_graph.add_pass(
        "MainGBufferPass",
        |data: &mut Payload<MainGBufferPassPayload>, resources| {
            let back_buffer_info: &BackBufferInfo = pin_board.get().unwrap();
            let b_desc = &back_buffer_info.desc;

            // BaseColor+AO
            let gbuffer0_desc = TextureDesc::texture_2d(b_desc.width, b_desc.height)
                .with_format(Format::Rgba8UnormSrgb)
                .with_clear_value(OptimalClearValue::ColorInt(0x00000000))
                .with_name("Gbuffer0");
            let gbuffer0 =
                resources.create_texture(&gbuffer0_desc, ResourceUsageFlags::RENDER_TARGET);

            // WorldNormal
            let gbuffer1_desc = TextureDesc::texture_2d(b_desc.width, b_desc.height)
                .with_format(Format::Rgba32Float)
                .with_clear_value(OptimalClearValue::ColorInt(0x00000000))
                .with_name("Gbuffer1");
            let gbuffer1 =
                resources.create_texture(&gbuffer1_desc, ResourceUsageFlags::RENDER_TARGET);

            // Metal+Roughnes
            let gbuffer2_desc = TextureDesc::texture_2d(b_desc.width, b_desc.height)
                .with_format(Format::Rg8Unorm)
                .with_clear_value(OptimalClearValue::ColorInt(0x00000000))
                .with_name("Gbuffer2");
            let gbuffer2 =
                resources.create_texture(&gbuffer2_desc, ResourceUsageFlags::RENDER_TARGET);

            let depth_buffer_desc = TextureDesc::texture_2d(b_desc.width, b_desc.height)
                .with_format(Format::Depth32Float)
                .with_clear_value(OptimalClearValue::DepthStencil(0.0, 0))
                .with_name("DepthBuffer");
            let depth_buffer =
                resources.create_texture(&depth_buffer_desc, ResourceUsageFlags::RENDER_TARGET);

            let uniform_buffer = resources.create_buffer(
                &BufferDesc::new(4 * 1024 as u64)
                    .cpu_write()
                    .with_name("Test Uniform Buffer"),
                ResourceUsageFlags::CONSTANT_BUFFER,
            );

            data.write(MainGBufferPassPayload {
                gbuffer0,
                gbuffer1,
                gbuffer2,
                depth_buffer,
                vtx_buffer,
                idx_buffer,
                uniform_buffer,
                descriptor_set_layout,
                pipeline_layout,
                pipeline: graphics_pipeline,
            });
            pin_board.publish(MainGBufferPassOutput {
                gbuffer0,
                gbuffer1,
                gbuffer2,
                depth_buffer,
            });
        },
        |data, encoder, resources| unsafe {
            // Unwrap all our fg resources from our setup payload
            let data = data.unwrap();

            let vtx_buffer = data.vtx_buffer.as_ref();
            let idx_buffer = data.idx_buffer.as_ref();
            let set_layout = data.descriptor_set_layout.as_ref();
            let device = resources.device();
            let descriptor_arena = resources.descriptor_arena();

            let gbuffer0 = resources.get_texture(data.gbuffer0).unwrap();
            let gbuffer1 = resources.get_texture(data.gbuffer1).unwrap();
            let gbuffer2 = resources.get_texture(data.gbuffer2).unwrap();
            let depth_buffer = resources.get_texture(data.depth_buffer).unwrap();
            let uniform_buffer = resources.get_buffer(data.uniform_buffer).unwrap();

            let u_ptr = uniform_buffer.map().unwrap();
            let u_alloc =
                UploadBumpAllocator::new_from_block(uniform_buffer, u_ptr, 0, 4 * 1024).unwrap();

            let camera_offset = u_alloc.allocate_object(CameraLayout::init()).device_offset;
            let model_offset = u_alloc.allocate_object(ModelLayout::init()).device_offset;

            uniform_buffer.unmap();

            let gbuffer0_rtv = gbuffer0
                .get_rtv(&ImageViewDesc::rtv_for_texture(gbuffer0))
                .unwrap();

            let gbuffer1_rtv = gbuffer1
                .get_rtv(&ImageViewDesc::rtv_for_texture(gbuffer1))
                .unwrap();

            let gbuffer2_rtv = gbuffer2
                .get_rtv(&ImageViewDesc::rtv_for_texture(gbuffer2))
                .unwrap();

            let depth_buffer_dsv = depth_buffer
                .get_dsv(&ImageViewDesc::dsv_for_texture(depth_buffer))
                .unwrap();

            // Begin a render pass targeting our back buffer
            let extent = gbuffer0.desc_ref().get_extent_2d();
            encoder.begin_rendering(&BeginRenderingInfo {
                layer_count: 1,
                extent: extent.clone(),
                color_attachments: &[
                    RenderingColorAttachmentInfo::new(gbuffer0_rtv)
                        .clear(ColorClearValue::Int(0x00000000))
                        .store(),
                    RenderingColorAttachmentInfo::new(gbuffer1_rtv)
                        .clear(ColorClearValue::Int(0x00000000))
                        .store(),
                    RenderingColorAttachmentInfo::new(gbuffer2_rtv)
                        .clear(ColorClearValue::Int(0x00000000))
                        .store(),
                ],
                depth_stencil_attachment: Some(
                    &RenderingDepthStencilAttachmentInfo::new(depth_buffer_dsv)
                        .depth_clear(DepthStencilClearValue::depth(0.0))
                        .depth_store(),
                ),
                allow_uav_writes: false,
            });

            encoder.bind_graphics_pipeline(data.pipeline.as_ref());

            let descriptor_set = descriptor_arena.allocate_set(set_layout).unwrap();
            let write = BufferDescriptorWrite::uniform_buffer(uniform_buffer, 256);
            device.update_descriptor_sets(&[
                DescriptorWriteDesc::uniform_buffer(
                    descriptor_set,
                    0,
                    &write.clone().with_offset(camera_offset as u64),
                ),
                DescriptorWriteDesc::uniform_buffer(
                    descriptor_set,
                    1,
                    &write.clone().with_offset(model_offset as u64),
                ),
            ]);

            encoder.bind_descriptor_sets(
                data.pipeline_layout.as_ref(),
                PipelineBindPoint::Graphics,
                0,
                &[descriptor_set],
                &[],
            );

            encoder.bind_vertex_buffers(0, &[InputAssemblyBufferBinding::new(vtx_buffer)]);
            encoder.bind_index_buffer(IndexType::U32, &InputAssemblyBufferBinding::new(idx_buffer));

            encoder.set_viewports(&[Viewport {
                x: 0.0,
                y: 0.0,
                width: extent.width as _,
                height: extent.height as _,
                min_depth: 0.0,
                max_depth: 1.0,
            }]);

            encoder.set_scissor_rects(&[Rect {
                x: 0,
                y: 0,
                w: extent.width,
                h: extent.height,
            }]);

            encoder.draw_indexed(INDICES.len() as _, 1, 0, 0, 0);

            encoder.end_rendering();
        },
    );
}

fn create_descriptor_set_layout(device: &dyn IDevice) -> AnyArc<dyn IDescriptorSetLayout> {
    let descriptor_set_layout_desc = DescriptorSetLayoutDesc {
        visibility: DescriptorShaderVisibility::All,
        items: &[
            DescriptorSetLayoutBinding::with_type(DescriptorType::UniformBuffer)
                .with_binding_num(0),
            DescriptorSetLayoutBinding::with_type(DescriptorType::UniformBuffer)
                .with_binding_num(1),
        ],
        name: Some("main_gbuffer_pass::DescriptorSetLayout"),
    };
    device
        .create_descriptor_set_layout(&descriptor_set_layout_desc)
        .unwrap()
}

fn create_root_signature(
    device: &dyn IDevice,
    descriptor_set_layout: &dyn IDescriptorSetLayout,
) -> AnyArc<dyn IPipelineLayout> {
    let pipeline_layout_desc = PipelineLayoutDesc {
        set_layouts: &[descriptor_set_layout],
        push_constant_blocks: &[],
        name: Some("main_gbuffer_pass::RootSignature"),
    };
    device
        .create_pipeline_layout(&pipeline_layout_desc)
        .unwrap()
}

fn create_pipeline_state(
    device: &dyn IDevice,
    pipeline_layout: &dyn IPipelineLayout,
    shader_db: &ShaderDatabaseAccessor,
) -> AnyArc<dyn IGraphicsPipeline> {
    let rasterizer_state = RasterizerStateDesc {
        cull_mode: CullMode::Back,
        front_face: FrontFaceOrder::CounterClockwise,
        polygon_mode: PolygonMode::Fill,
        depth_bias: 0,
        depth_bias_clamp: 0.0,
        depth_bias_slope_factor: 0.0,
    };

    let depth_stencil_state = DepthStencilStateDesc {
        depth_test: true,
        depth_write: true,
        depth_compare_op: CompareOp::Greater,
        stencil_test: false,
        depth_bounds_enable: false,
        ..Default::default()
    };

    let vertex_layout = VertexInputStateDesc {
        input_bindings: &[VertexInputBindingDesc {
            binding: 0,
            stride: 44,
            input_rate: VertexInputRate::PerVertex,
        }],
        input_attributes: &[
            VertexInputAttributeDesc {
                location: 0,
                binding: 0,
                format: Format::Rgb32Float,
                offset: 0,
            },
            VertexInputAttributeDesc {
                location: 1,
                binding: 0,
                format: Format::Rg32Float,
                offset: 12,
            },
            VertexInputAttributeDesc {
                location: 2,
                binding: 0,
                format: Format::Rgb32Float,
                offset: 20,
            },
            VertexInputAttributeDesc {
                location: 3,
                binding: 0,
                format: Format::Rgb32Float,
                offset: 32,
            },
        ],
    };

    let input_assembly_state = InputAssemblyStateDesc {
        primitive_topology: PrimitiveTopology::TriangleList,
    };

    let blend_state = BlendStateDesc {
        attachments: &[
            AttachmentBlendState {
                blend_enabled: false,
                color_write_mask: ColorComponentFlags::all(),
                ..Default::default()
            },
            AttachmentBlendState {
                blend_enabled: false,
                color_write_mask: ColorComponentFlags::all(),
                ..Default::default()
            },
            AttachmentBlendState {
                blend_enabled: false,
                color_write_mask: ColorComponentFlags::all(),
                ..Default::default()
            },
        ],
    };

    let vertex_shader = shader_db
        .load_stage(shaders::aleph_render::deferred::main_gbuffer_vert())
        .unwrap();
    let fragment_shader = shader_db
        .load_stage(shaders::aleph_render::deferred::main_gbuffer_frag())
        .unwrap();

    let graphics_pipeline_desc_new = GraphicsPipelineDesc {
        shader_stages: &[vertex_shader, fragment_shader],
        pipeline_layout,
        vertex_layout: &vertex_layout,
        input_assembly_state: &input_assembly_state,
        rasterizer_state: &rasterizer_state,
        depth_stencil_state: &depth_stencil_state,
        blend_state: &blend_state,
        render_target_formats: &[
            Format::Rgba8UnormSrgb,
            Format::Rgba32Float,
            Format::Rg8Unorm,
        ],
        depth_stencil_format: Some(Format::Depth32Float),
        name: Some("main_gbuffer_pass::GraphicsPipelineState"),
    };

    device
        .create_graphics_pipeline(&graphics_pipeline_desc_new)
        .unwrap()
}

fn proj_matrix() -> [f32; 16] {
    perspective_reversed_infinite_z_wgpu_dx_gl(90.0f32.to_radians(), 16. / 9., 0.1)
        .transposed()
        .as_array()
        .clone() // Hopefully gets elided
}

fn view_matrix() -> [f32; 16] {
    Mat4::identity().as_array().clone()
}

fn camera_position() -> [f32; 4] {
    [0., 0., 0., 0.]
}

#[repr(align(256))]
#[derive(Default, Debug)]
pub struct CameraLayout {
    _view_matrix: [f32; 16],
    _proj_matrix: [f32; 16],
    _position: [f32; 4],
}

impl CameraLayout {
    pub fn init() -> Self {
        Self {
            _view_matrix: view_matrix(),
            _proj_matrix: proj_matrix(),
            _position: camera_position(),
        }
    }
}

#[repr(align(256))]
#[derive(Default, Debug)]
struct ModelLayout {
    _model_matrix: [f32; 16],
    _normal_matrix: [f32; 16],
}

impl ModelLayout {
    pub fn init() -> Self {
        Self {
            _model_matrix: Mat4::from_translation(Vec3::new(0., 0., -3.))
                .transposed()
                .as_array()
                .clone(),
            _normal_matrix: Mat4::identity().as_array().clone(),
        }
    }
}

#[repr(C)]
#[derive(Clone)]
struct Vertex {
    pub position: [f32; 3],
    pub uv: [f32; 2],
    pub normal: [f32; 3],
    pub tangent: [f32; 3],
}

impl Vertex {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            position: [x, y, z],
            uv: [0.; 2],
            normal: [0.; 3],
            tangent: [0.; 3],
        }
    }
}

#[rustfmt::skip]
const VERTS: [Vertex; 8] = [
    Vertex::new( 1.,  1., -1.), //0
    Vertex::new( 1., -1., -1.), //1
    Vertex::new( 1.,  1.,  1.), //2
    Vertex::new( 1., -1.,  1.), //3
    Vertex::new(-1.,  1., -1.), //4
    Vertex::new(-1., -1., -1.), //5
    Vertex::new(-1.,  1.,  1.), //6
    Vertex::new(-1., -1.,  1.), //7
];

#[rustfmt::skip]
const INDICES: [u32; 36] = [
    4, 2, 0,
    2, 7, 3,
    6, 5, 7,
    1, 7, 5,
    0, 3, 1,
    4, 1, 5,
    4, 6, 2,
    2, 6, 7,
    6, 4, 5,
    1, 3, 7,
    0, 2, 3,
    4, 0, 1,
];
