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

use aleph_any::AnyArc;
use aleph_device_allocators::{IUploadAllocator, UploadBumpAllocator};
use aleph_frame_graph::*;
use aleph_math::{Mat4, Vec3};
use aleph_nstr::nstr;
use aleph_pin_board::PinBoard;
use aleph_rhi_api::*;

use crate::pass::resource_processor::ResourceProcessorOutput;
use crate::pass::{GraphArgs, GraphSwapImageInfo};
use crate::{
    shaders, CameraInfo, RenderSceneParam, RenderTransform, ShaderDatabaseAccessor, StateCache,
    StaticMesh,
};

struct MainGBufferPassPayload {
    gbuffer0: ResourceMut,
    gbuffer1: ResourceMut,
    gbuffer2: ResourceMut,
    depth_buffer: ResourceMut,
    uniform_buffer: ResourceMut,
}

pub struct MainGBufferPassOutput {
    pub gbuffer0: ResourceMut,
    pub gbuffer1: ResourceMut,
    pub gbuffer2: ResourceMut,
    pub depth_buffer: ResourceMut,
}

pub fn pass(
    frame_graph: &mut FrameGraphBuilder<GraphArgs>,
    device: &dyn IDevice,
    pin_board: &PinBoard,
    state_cache: &mut StateCache,
) {
    let sampler = create_sampler(device);
    let descriptor_set_layout = create_descriptor_set_layout(device);
    let descriptor_set_layout_tex = create_descriptor_set_layout_tex(device, &[sampler.as_ref()]);
    let pipeline_layout = create_root_signature(
        device,
        descriptor_set_layout.as_ref(),
        descriptor_set_layout_tex.as_ref(),
    );

    let pipeline = create_pipeline_state(device, pipeline_layout.as_ref(), state_cache.shader_db());

    frame_graph.add_pass(nstr!("MainGBufferPass"), |resources| {
        let resource_processor: &ResourceProcessorOutput = pin_board.get().unwrap();
        let back_buffer_info: &GraphSwapImageInfo = pin_board.get().unwrap();
        let b_desc = &back_buffer_info.desc;

        // We have to have the resource processor run first!
        resources.execute_after(resource_processor.exec);

        // BaseColor+AO
        let gbuffer0_desc = TextureDesc::texture_2d(b_desc.width, b_desc.height)
            .with_format(Format::Rgba8UnormSrgb)
            .with_clear_value(OptimalClearValue::ColorInt(0x00000000))
            .with_name(obj_name!("Gbuffer0"));
        let gbuffer0 = resources.create_texture(&gbuffer0_desc, ResourceUsageFlags::RENDER_TARGET);

        // WorldNormal
        let gbuffer1_desc = TextureDesc::texture_2d(b_desc.width, b_desc.height)
            .with_format(Format::Rgba32Float)
            .with_clear_value(OptimalClearValue::ColorInt(0x00000000))
            .with_name(obj_name!("Gbuffer1"));
        let gbuffer1 = resources.create_texture(&gbuffer1_desc, ResourceUsageFlags::RENDER_TARGET);

        // Metal+Roughnes
        let gbuffer2_desc = TextureDesc::texture_2d(b_desc.width, b_desc.height)
            .with_format(Format::Rg8Unorm)
            .with_clear_value(OptimalClearValue::ColorInt(0x00000000))
            .with_name(obj_name!("Gbuffer2"));
        let gbuffer2 = resources.create_texture(&gbuffer2_desc, ResourceUsageFlags::RENDER_TARGET);

        let depth_buffer_desc = TextureDesc::texture_2d(b_desc.width, b_desc.height)
            .with_format(Format::Depth32Float)
            .with_clear_value(OptimalClearValue::DepthStencil(0.0, 0))
            .with_name(obj_name!("DepthBuffer"));
        let depth_buffer =
            resources.create_texture(&depth_buffer_desc, ResourceUsageFlags::RENDER_TARGET);

        let uniform_buffer = resources.create_buffer(
            &BufferDesc::new(4 * 1024 * 1024)
                .cpu_write()
                .with_name(obj_name!("TestUniformBuffer")),
            ResourceUsageFlags::CONSTANT_BUFFER,
        );

        let data = MainGBufferPassPayload {
            gbuffer0,
            gbuffer1,
            gbuffer2,
            depth_buffer,
            uniform_buffer,
        };
        pin_board.publish(MainGBufferPassOutput {
            gbuffer0,
            gbuffer1,
            gbuffer2,
            depth_buffer,
        });

        move |encoder, _graph, resources, args| unsafe {
            let set_layout = descriptor_set_layout.as_ref();
            let set_layout_tex = descriptor_set_layout_tex.as_ref();
            let device = resources.device();
            let descriptor_arena = resources.descriptor_arena();
            let camera_info = args.board.get::<CameraInfo>().unwrap();
            let scene = args.board.get::<RenderSceneParam>().copied().unwrap();

            let gbuffer0 = resources.get_texture(data.gbuffer0).unwrap();
            let gbuffer1 = resources.get_texture(data.gbuffer1).unwrap();
            let gbuffer2 = resources.get_texture(data.gbuffer2).unwrap();
            let depth_buffer = resources.get_texture(data.depth_buffer).unwrap();
            let uniform_buffer = resources.get_buffer(data.uniform_buffer).unwrap();

            let u_ptr = uniform_buffer.map().unwrap();
            let u_alloc =
                UploadBumpAllocator::new_from_block(uniform_buffer, u_ptr, 0, 4 * 1024 * 1024)
                    .unwrap();

            let extent = gbuffer0.desc_ref().get_extent_2d();
            // let aspect_ratio = extent.width as f32 / extent.height as f32;

            let camera_layout = CameraLayout {
                view_matrix: camera_info
                    .get_view_matrix()
                    .transposed()
                    .as_array()
                    .clone(),
                proj_matrix: camera_info
                    .get_proj_matrix()
                    .transposed()
                    .as_array()
                    .clone(),
                position: camera_info
                    .position
                    .into_homogeneous_point()
                    .as_array()
                    .clone(),
                _padding: [0; 112],
            };
            let camera = u_alloc.allocate_object(camera_layout).unwrap();

            let descriptor_set = descriptor_arena.allocate_set(set_layout).unwrap();
            let write = BufferDescriptorWrite::uniform_buffer(uniform_buffer, 256);
            device.update_descriptor_sets(&[
                DescriptorWriteDesc::uniform_buffer(
                    descriptor_set,
                    0,
                    &write.clone().with_offset(camera.device_offset as u64),
                ),
                DescriptorWriteDesc::uniform_buffer_dynamic(
                    descriptor_set,
                    1,
                    &write.clone().with_offset(0),
                ),
            ]);

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

            encoder.bind_graphics_pipeline(pipeline.as_ref());

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

            let objects = scene.get_storage_ref::<StaticMesh>().unwrap();
            for (t, o) in objects.iter() {
                let m = u_alloc
                    .allocate_object(ModelLayout::from_transform(t))
                    .unwrap();
                m.result.colour = o.colour;
                m.result.metal_roughness = [o.metalness, o.roughness, 0.0, 0.0];

                let vtx = args.buffer_pool.get_ref(o.vtx).unwrap();
                let vtx = vtx.get().unwrap();
                let idx = args.buffer_pool.get_ref(o.idx).unwrap();
                let idx = idx.get().unwrap();

                encoder.bind_vertex_buffers(0, &[InputAssemblyBufferBinding::new(vtx)]);
                encoder.bind_index_buffer(IndexType::U32, &InputAssemblyBufferBinding::new(idx));

                let colour_tex = args.texture_pool.get_ref(o.colour_tex).unwrap();
                let image_view_c = colour_tex.get_default_view().unwrap();
                // let image_view_c = colour_tex.get_texture().unwrap();
                // let image_view_c = colour_tex
                //     .get_view(&ImageViewDesc {
                //         format: colour_tex.desc_ref().format.to_srgb(),
                //         ..ImageViewDesc::srv_for_texture(colour_tex)
                //     })
                //     .unwrap();
                let image_view_mr = args.texture_pool.get_ref(o.metal_roughness_tex).unwrap();
                let image_view_mr = image_view_mr.get_default_view().unwrap();
                let image_view_nrm = args.texture_pool.get_ref(o.normal_map_tex).unwrap();
                let image_view_nrm = image_view_nrm.get_default_view().unwrap();
                let tex_set = descriptor_arena.allocate_set(set_layout_tex).unwrap();
                device.update_descriptor_sets(&[
                    DescriptorWriteDesc::texture(
                        tex_set,
                        0,
                        &ImageDescriptorWrite::srv(image_view_c),
                    ),
                    DescriptorWriteDesc::texture(
                        tex_set,
                        1,
                        &ImageDescriptorWrite::srv(image_view_mr),
                    ),
                    DescriptorWriteDesc::texture(
                        tex_set,
                        2,
                        &ImageDescriptorWrite::srv(image_view_nrm),
                    ),
                ]);

                encoder.bind_descriptor_sets(
                    pipeline_layout.as_ref(),
                    PipelineBindPoint::Graphics,
                    0,
                    &[descriptor_set, tex_set],
                    &[m.device_offset as u32],
                );

                encoder.draw_indexed((idx.desc_ref().size / 4) as u32, 1, 0, 0, 0);
            }

            encoder.end_rendering();

            uniform_buffer.unmap().unwrap();
        }
    });
}

fn create_descriptor_set_layout(device: &dyn IDevice) -> AnyArc<dyn IDescriptorSetLayout> {
    let descriptor_set_layout_desc = DescriptorSetLayoutDesc {
        visibility: DescriptorShaderVisibility::All,
        items: &[
            DescriptorSetLayoutBinding::with_type(DescriptorType::UniformBuffer)
                .with_binding_num(0),
            DescriptorSetLayoutBinding::with_type(DescriptorType::UniformBufferDynamic)
                .with_binding_num(1),
        ],
        name: obj_name_opt!("DescriptorSetLayout"),
    };
    device
        .create_descriptor_set_layout(&descriptor_set_layout_desc)
        .unwrap()
}

fn create_descriptor_set_layout_tex(
    device: &dyn IDevice,
    sampler: &[&dyn ISampler],
) -> AnyArc<dyn IDescriptorSetLayout> {
    let descriptor_set_layout_desc = DescriptorSetLayoutDesc {
        visibility: DescriptorShaderVisibility::All,
        items: &[
            DescriptorType::Texture.binding(0),
            DescriptorType::Texture.binding(1),
            DescriptorType::Texture.binding(2),
            DescriptorType::Sampler
                .binding(3)
                .with_static_samplers(sampler),
        ],
        name: obj_name_opt!("DescriptorSetLayout"),
    };
    device
        .create_descriptor_set_layout(&descriptor_set_layout_desc)
        .unwrap()
}

fn create_root_signature(
    device: &dyn IDevice,
    descriptor_set_layout: &dyn IDescriptorSetLayout,
    descriptor_set_layout_tex: &dyn IDescriptorSetLayout,
) -> AnyArc<dyn IPipelineLayout> {
    let pipeline_layout_desc = PipelineLayoutDesc {
        set_layouts: &[descriptor_set_layout, descriptor_set_layout_tex],
        push_constant_blocks: &[],
        name: obj_name_opt!("RootSignature"),
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
            stride: 60,
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
                format: Format::Rgba32Float,
                offset: 32,
            },
            VertexInputAttributeDesc {
                location: 4,
                binding: 0,
                format: Format::Rgb32Float,
                offset: 48,
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
        .load_stage(shaders::deferred::main_gbuffer_vert())
        .unwrap();
    let fragment_shader = shader_db
        .load_stage(shaders::deferred::main_gbuffer_frag())
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
        name: obj_name_opt!("GraphicsPipeline"),
    };

    device
        .create_graphics_pipeline(&graphics_pipeline_desc_new)
        .unwrap()
}

#[repr(C)]
#[derive(Debug)]
pub struct CameraLayout {
    pub view_matrix: [f32; 16],
    pub proj_matrix: [f32; 16],
    pub position: [f32; 4],
    pub _padding: [u8; 112],
}

#[repr(C)]
#[derive(Debug)]
pub struct ModelLayout {
    pub model_matrix: [f32; 16],
    pub normal_matrix: [f32; 16],
    pub colour: [f32; 4],
    pub metal_roughness: [f32; 4],
    pub _padding: [u8; 96],
}

impl ModelLayout {
    pub fn from_transform(v: &RenderTransform) -> Self {
        let pos = Vec3::new(
            v.position.x as f32,
            v.position.y as f32,
            v.position.z as f32,
        );

        let t = Mat4::from_translation(pos);
        let r = v.rotation.into_matrix().into_homogeneous();
        let s = Mat4::from_nonuniform_scale(v.scale);

        let model_matrix = t * r * s;
        let normal_matrix = model_matrix.truncate().inversed().transposed();
        Self {
            model_matrix: *model_matrix.transposed().as_array(),
            normal_matrix: *normal_matrix.transposed().into_homogeneous().as_array(),
            colour: [1.0; 4],
            metal_roughness: [0.0, 1.0, 0.0, 0.0],
            _padding: [0; 96],
        }
    }
}

fn create_sampler(device: &dyn IDevice) -> AnyArc<dyn ISampler> {
    let desc = SamplerDesc {
        min_filter: SamplerFilter::Linear,
        mag_filter: SamplerFilter::Linear,
        mip_filter: SamplerMipFilter::Linear,
        address_mode_u: SamplerAddressMode::Wrap,
        address_mode_v: SamplerAddressMode::Wrap,
        address_mode_w: SamplerAddressMode::Wrap,
        enable_anisotropy: true,
        max_anisotropy: 16,
        ..Default::default()
    };
    device.create_sampler(&desc).unwrap()
}
