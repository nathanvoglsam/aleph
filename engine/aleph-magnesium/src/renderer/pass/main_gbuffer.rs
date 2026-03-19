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
use aleph_ecs::world::query::Read;
use aleph_frame_graph::*;
use aleph_gen_arena::HandleType;
use aleph_math::ToSingle;
use aleph_nstr::nstr;
use aleph_pin_board::PinBoard;

use crate::internal::renderer::gpu_data_layouts::{CameraLayout, ModelLayout};
use crate::material::{Material, MaterialId};
use crate::material_instance::MaterialInstanceReader;
use crate::renderer::frame_graph::{GraphArgs, GraphSwapImageInfo};
use crate::renderer::shader_accessor::{IShaderAccessor, IShaderAccessorExt};
use crate::renderer::state_cache::{IStateCacheKey, StateCache};
use crate::resource::buffer::BufferHandle;
use crate::scene::camera;
use crate::scene::components::{PerspectiveCamera, RenderTransform, StaticMesh};

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
    device: &dyn rhi::IDevice,
    pin_board: &PinBoard,
    state_cache: &mut StateCache,
) {
    frame_graph.add_pass(nstr!("MainGBufferPass"), |resources| {
        let back_buffer_info: &GraphSwapImageInfo = pin_board.get().unwrap();
        let b_desc = &back_buffer_info.desc;

        // BaseColor+AO
        let gbuffer0_desc = rhi::TextureDesc::texture_2d(b_desc.width, b_desc.height)
            .with_format(rhi::Format::Rgba8UnormSrgb)
            .with_clear_value(rhi::OptimalClearValue::ColorInt(0x00000000))
            .with_name(rhi::obj_name!("Gbuffer0"));
        let gbuffer0 =
            resources.create_texture(&gbuffer0_desc, rhi::ResourceUsageFlags::RENDER_TARGET);

        // WorldNormal
        let gbuffer1_desc = rhi::TextureDesc::texture_2d(b_desc.width, b_desc.height)
            .with_format(rhi::Format::Rg16Unorm)
            .with_clear_value(rhi::OptimalClearValue::ColorInt(0x00000000))
            .with_name(rhi::obj_name!("Gbuffer1"));
        let gbuffer1 =
            resources.create_texture(&gbuffer1_desc, rhi::ResourceUsageFlags::RENDER_TARGET);

        // Metal+Roughnes
        let gbuffer2_desc = rhi::TextureDesc::texture_2d(b_desc.width, b_desc.height)
            .with_format(rhi::Format::Rg8Unorm)
            .with_clear_value(rhi::OptimalClearValue::ColorInt(0x00000000))
            .with_name(rhi::obj_name!("Gbuffer2"));
        let gbuffer2 =
            resources.create_texture(&gbuffer2_desc, rhi::ResourceUsageFlags::RENDER_TARGET);

        let depth_buffer_desc = rhi::TextureDesc::texture_2d(b_desc.width, b_desc.height)
            .with_format(rhi::Format::Depth32Float)
            .with_clear_value(rhi::OptimalClearValue::DepthStencil(0.0, 0))
            .with_name(rhi::obj_name!("DepthBuffer"));
        let depth_buffer =
            resources.create_texture(&depth_buffer_desc, rhi::ResourceUsageFlags::RENDER_TARGET);

        let uniform_buffer = resources.create_buffer(
            &rhi::BufferDesc::new(4 * 1024 * 1024)
                .cpu_write()
                .with_name(rhi::obj_name!("TestUniformBuffer")),
            rhi::ResourceUsageFlags::CONSTANT_BUFFER,
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

        let key = MainOpaqueCommonLayout::key();
        let common_state =
            state_cache.get_or_insert_with(&key, |_, _| MainOpaqueCommonLayout::new(device));

        move |encoder, _graph, resources, args| unsafe {
            let device = resources.device();
            let descriptor_arena = resources.descriptor_arena();

            let swap_info = args
                .scene
                .get_singleton_ref::<GraphSwapImageInfo>()
                .unwrap();
            let (camera_tform, camera_info) = args
                .scene
                .query_one::<(Read<RenderTransform>, Read<PerspectiveCamera>)>(args.camera)
                .unwrap();

            let gbuffer0 = resources.get_texture(data.gbuffer0).unwrap();
            let gbuffer1 = resources.get_texture(data.gbuffer1).unwrap();
            let gbuffer2 = resources.get_texture(data.gbuffer2).unwrap();
            let depth_buffer = resources.get_texture(data.depth_buffer).unwrap();
            let uniform_buffer = resources.get_buffer(data.uniform_buffer).unwrap();

            let u_ptr = device.map_buffer(uniform_buffer).unwrap();
            let u_alloc = UploadBumpAllocator::new_from_block(
                uniform_buffer.clone(),
                rhi::ResourceUsageFlags::CONSTANT_BUFFER,
                u_ptr,
                0,
                4 * 1024 * 1024,
            )
            .unwrap();

            let extent = device.get_texture_desc(gbuffer0).get_extent_2d();
            // let aspect_ratio = extent.width as f32 / extent.height as f32;

            let camera_layout = CameraLayout {
                view_matrix: camera::get_view_matrix(camera_tform).as_array().clone(),
                proj_matrix: camera_info.get_matrix(swap_info.aspect).as_array().clone(),
                position: camera_tform
                    .position
                    .to_single()
                    .into_homogeneous_point()
                    .as_array()
                    .clone(),
                _padding: [0; 112],
            };
            let camera = u_alloc.allocate_object(camera_layout).unwrap();

            let global_block_layout = common_state.global_block_layout.as_ref();
            let global_block = descriptor_arena
                .allocate_block(global_block_layout)
                .unwrap();
            let sampler = &common_state.sampler;
            let params = [
                rhi::BufferWrite::cbv_offset(uniform_buffer, camera.device_offset as u64, 256)
                    .into(),
                rhi::SamplerWrite::new(sampler).into(),
            ];
            device.update_parameter_block(global_block_layout, global_block, 0, &params);

            let gbuffer0_rtv = rhi::ImageView::get_rtv_for(device, gbuffer0).unwrap();
            let gbuffer1_rtv = rhi::ImageView::get_rtv_for(device, gbuffer1).unwrap();
            let gbuffer2_rtv = rhi::ImageView::get_rtv_for(device, gbuffer2).unwrap();
            let depth_buffer_dsv = rhi::ImageView::get_dsv_for(device, depth_buffer).unwrap();

            // Begin a render pass targeting our back buffer
            let mut render = encoder.begin_rendering(
                &rhi::BeginRenderingInfo {
                    layer_count: 1,
                    extent: extent.clone(),
                    color_attachments: &[
                        rhi::RenderingColorAttachmentInfo::new(gbuffer0_rtv)
                            .clear(rhi::ColorClearValue::Int(0x00000000))
                            .store(),
                        rhi::RenderingColorAttachmentInfo::new(gbuffer1_rtv)
                            .clear(rhi::ColorClearValue::Int(0x00000000))
                            .store(),
                        rhi::RenderingColorAttachmentInfo::new(gbuffer2_rtv)
                            .clear(rhi::ColorClearValue::Int(0x00000000))
                            .store(),
                    ],
                    depth_stencil_attachment: Some(
                        &rhi::RenderingDepthStencilAttachmentInfo::new(depth_buffer_dsv)
                            .depth_clear(0.0)
                            .depth_store(),
                    ),
                    allow_uav_writes: false,
                },
                nstr!("MainGBufferPass::render_pass"),
            );

            let objects = args
                .scene
                .query::<(Read<RenderTransform>, Read<StaticMesh>)>();

            let mut commands = Vec::from_iter(objects.map(|(_, (t, o))| {
                let mi = o.material_instance.unwrap();
                let m = args.material_instance_pool.get_ref(mi).unwrap();

                let key = (m.material().id().get() as u64) << 32;
                let sort_key = key | mi.to_bare_handle().to_fields().slot_index as u64;

                DrawCommand {
                    sort_key,
                    tform: t,
                    vtx: o.vtx.unwrap(),
                    idx: o.idx.unwrap(),
                    mat: m,
                }
            }));
            commands.sort_unstable_by_key(|v| v.sort_key);

            if commands.is_empty() {
                return;
            }

            let mut current_material_instance = commands[0].mat;
            let mut current_material_key = {
                let cull_mode = if current_material_instance.double_sided() {
                    rhi::CullMode::None
                } else {
                    rhi::CullMode::Back
                };
                MainOpaqueMaterialLayout::key_for(current_material_instance.material(), cull_mode)
            };
            let mut current_material_state = {
                let mut state_cache = args.state_cache.lock();
                state_cache.get_or_insert_with(&current_material_key, |cache, key| {
                    MainOpaqueMaterialLayout::new(
                        key,
                        cache,
                        device,
                        common_state.as_ref(),
                        &current_material_instance.material(),
                    )
                })
            };

            render.bind_graphics_pipeline(&current_material_state.pipeline);

            render.bind_parameter_blocks(
                current_material_state.binding_signature.as_ref(),
                0,
                &[global_block],
            );

            let material_block_layout = current_material_state.material_block_layout.as_ref();
            let material_block = descriptor_arena
                .allocate_block(material_block_layout)
                .unwrap();
            current_material_instance
                .material()
                .material
                .update_parameter_block(
                    material_block_layout,
                    args.buffer_pool,
                    args.texture_pool,
                    device,
                    current_material_instance,
                    material_block,
                );
            render.bind_parameter_blocks(
                current_material_state.binding_signature.as_ref(),
                1,
                &[material_block],
            );

            render.set_viewports(&[rhi::Viewport {
                x: 0.0,
                y: 0.0,
                width: extent.width as _,
                height: extent.height as _,
                min_depth: 0.0,
                max_depth: 1.0,
            }]);

            render.set_scissor_rects(&[rhi::Rect {
                x: 0,
                y: 0,
                w: extent.width,
                h: extent.height,
            }]);

            for command in commands {
                if !current_material_instance.same_instance(command.mat) {
                    current_material_instance = command.mat;

                    let new_key = {
                        let cull_mode = if current_material_instance.double_sided() {
                            rhi::CullMode::None
                        } else {
                            rhi::CullMode::Back
                        };
                        MainOpaqueMaterialLayout::key_for(
                            current_material_instance.material(),
                            cull_mode,
                        )
                    };

                    if current_material_key != new_key {
                        current_material_key = new_key;
                        current_material_state = {
                            let mut state_cache = args.state_cache.lock();
                            state_cache.get_or_insert_with(&current_material_key, |cache, key| {
                                MainOpaqueMaterialLayout::new(
                                    key,
                                    cache,
                                    device,
                                    common_state.as_ref(),
                                    current_material_instance.material(),
                                )
                            })
                        };

                        render.bind_graphics_pipeline(&current_material_state.pipeline);

                        render.set_viewports(&[rhi::Viewport {
                            x: 0.0,
                            y: 0.0,
                            width: extent.width as _,
                            height: extent.height as _,
                            min_depth: 0.0,
                            max_depth: 1.0,
                        }]);

                        render.set_scissor_rects(&[rhi::Rect {
                            x: 0,
                            y: 0,
                            w: extent.width,
                            h: extent.height,
                        }]);
                    }

                    let material_block_layout =
                        current_material_state.material_block_layout.as_ref();
                    let material_block = descriptor_arena
                        .allocate_block(material_block_layout)
                        .unwrap();
                    current_material_instance
                        .material()
                        .material
                        .update_parameter_block(
                            material_block_layout,
                            args.buffer_pool,
                            args.texture_pool,
                            device,
                            current_material_instance,
                            material_block,
                        );
                    render.bind_parameter_blocks(
                        current_material_state.binding_signature.as_ref(),
                        1,
                        &[material_block],
                    );
                }

                // Bind the model's vertex buffers
                let vtx = args.buffer_pool.get_ref(command.vtx).unwrap();
                let vtx = vtx.handle().unwrap();
                let idx = args.buffer_pool.get_ref(command.idx).unwrap();
                let idx = idx.handle().unwrap();
                render.bind_vertex_buffers(0, &[rhi::InputAssemblyBufferBinding::new(vtx)]);
                render.bind_index_buffer(
                    rhi::IndexType::U32,
                    &rhi::InputAssemblyBufferBinding::new(idx),
                );

                // Upload and rebind the per-model parameters
                let t = command.tform;
                let m = u_alloc
                    .allocate_object(ModelLayout::from_transform(t))
                    .unwrap();

                let params =
                    [
                        rhi::BufferWrite::cbv_offset(uniform_buffer, m.device_offset as u64, 256)
                            .into(),
                    ];
                render.push_parameters(
                    current_material_state.binding_signature.as_ref(),
                    2,
                    0,
                    &params,
                );

                let idx_count = device.get_buffer_desc(idx).size / 4;
                render.draw_indexed(idx_count as u32, 1, 0, 0, 0);
            }

            // End the render pass explicitly
            drop(render);

            device.unmap_buffer(uniform_buffer).unwrap();
        }
    });
}

struct DrawCommand<'a> {
    sort_key: u64,
    tform: &'a RenderTransform,
    vtx: BufferHandle,
    idx: BufferHandle,
    mat: MaterialInstanceReader<'a>,
}

#[derive(PartialEq, Eq, Hash)]
pub struct MainOpaqueCommonKey();

impl IStateCacheKey for MainOpaqueCommonKey {
    type Storage = MainOpaqueCommonLayout;
}

pub struct MainOpaqueCommonLayout {
    pub global_block_layout: AnyArc<dyn rhi::IParameterBlockLayout>,
    pub model_block_layout: AnyArc<dyn rhi::IParameterBlockLayout>,
    pub sampler: rhi::SamplerHandle,
}

impl MainOpaqueCommonLayout {
    pub const fn key() -> MainOpaqueCommonKey {
        MainOpaqueCommonKey()
    }

    pub fn new(device: &dyn rhi::IDevice) -> Self {
        let global_block_layout = Self::create_global_block_layout(device);
        let model_block_layout = Self::create_model_block_layout(device);

        let desc = rhi::SamplerDesc {
            min_filter: rhi::SamplerFilter::Linear,
            mag_filter: rhi::SamplerFilter::Linear,
            mip_filter: rhi::SamplerMipFilter::Linear,
            address_mode_u: rhi::SamplerAddressMode::Wrap,
            address_mode_v: rhi::SamplerAddressMode::Wrap,
            address_mode_w: rhi::SamplerAddressMode::Wrap,
            enable_anisotropy: true,
            max_anisotropy: 16,
            ..Default::default()
        };
        let sampler = device.create_sampler(&desc).unwrap();

        Self {
            global_block_layout,
            model_block_layout,
            sampler,
        }
    }

    fn create_global_block_layout(
        device: &dyn rhi::IDevice,
    ) -> AnyArc<dyn rhi::IParameterBlockLayout> {
        let desc = rhi::ParameterBlockDesc {
            params: &[
                rhi::ParameterType::ConstantBuffer.param(),
                rhi::ParameterType::SamplerState.param(),
            ],
            visibility: rhi::DescriptorShaderVisibility::All,
            flags: Default::default(),
            name: rhi::obj_name_opt!("GlobalParameterBlockLayout"),
        };
        device.create_parameter_block_layout(&desc).unwrap()
    }

    fn create_model_block_layout(
        device: &dyn rhi::IDevice,
    ) -> AnyArc<dyn rhi::IParameterBlockLayout> {
        let desc = rhi::ParameterBlockDesc {
            params: &[rhi::ParameterType::ConstantBuffer.param()],
            visibility: rhi::DescriptorShaderVisibility::All,
            flags: rhi::ParameterBlockFlags::PUSH_DESCRIPTOR,
            name: rhi::obj_name_opt!("ModelParameterBlockLayout"),
        };
        device.create_parameter_block_layout(&desc).unwrap()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct MainOpaqueMaterialKey(MaterialId, rhi::CullMode);

impl IStateCacheKey for MainOpaqueMaterialKey {
    type Storage = MainOpaqueMaterialLayout;
}

pub struct MainOpaqueMaterialLayout {
    pub global_block_layout: AnyArc<dyn rhi::IParameterBlockLayout>,
    pub material_block_layout: AnyArc<dyn rhi::IParameterBlockLayout>,
    pub model_block_layout: AnyArc<dyn rhi::IParameterBlockLayout>,
    pub binding_signature: AnyArc<dyn rhi::IBindingSignature>,
    pub pipeline: rhi::GraphicsPipelineHandle,
}

impl MainOpaqueMaterialLayout {
    pub const fn key_for(material: &Material, cull_mode: rhi::CullMode) -> MainOpaqueMaterialKey {
        MainOpaqueMaterialKey(material.id(), cull_mode)
    }

    pub fn new(
        key: &MainOpaqueMaterialKey,
        cache: &mut StateCache,
        device: &dyn rhi::IDevice,
        common: &MainOpaqueCommonLayout,
        material: &Material,
    ) -> Self {
        let global_block_layout = common.global_block_layout.clone();
        let material_block_layout = Self::create_material_block_layout(device, material);
        let model_block_layout = common.model_block_layout.clone();
        let binding_signature = Self::create_binding_signature(
            device,
            global_block_layout.as_ref(),
            material_block_layout.as_ref(),
            model_block_layout.as_ref(),
        );
        let pipeline = Self::create_pipeline_state(
            device,
            key,
            cache.shader_db(),
            binding_signature.as_ref(),
            material,
        );

        Self {
            global_block_layout,
            material_block_layout,
            model_block_layout,
            binding_signature,
            pipeline,
        }
    }

    fn create_material_block_layout(
        device: &dyn rhi::IDevice,
        material: &Material,
    ) -> AnyArc<dyn rhi::IParameterBlockLayout> {
        material.material.create_parameter_block_layout(device)
    }

    fn create_binding_signature(
        device: &dyn rhi::IDevice,
        global: &dyn rhi::IParameterBlockLayout,
        material: &dyn rhi::IParameterBlockLayout,
        model: &dyn rhi::IParameterBlockLayout,
    ) -> AnyArc<dyn rhi::IBindingSignature> {
        let desc = rhi::BindingSignatureDesc {
            parameter_block_layouts: &[global, material, model],
            push_constant_block: None,
            name: rhi::obj_name_opt!("RootSignature"),
        };
        device.create_binding_signature(&desc).unwrap()
    }

    fn create_pipeline_state(
        device: &dyn rhi::IDevice,
        key: &MainOpaqueMaterialKey,
        shader_db: &dyn IShaderAccessor,
        binding_signature: &dyn rhi::IBindingSignature,
        material: &Material,
    ) -> rhi::GraphicsPipelineHandle {
        let rasterizer_state = rhi::RasterizerStateDesc {
            cull_mode: key.1,
            front_face: rhi::FrontFaceOrder::CounterClockwise,
            polygon_mode: rhi::PolygonMode::Fill,
            depth_bias: 0,
            depth_bias_clamp: 0.0,
            depth_bias_slope_factor: 0.0,
        };

        let depth_stencil_state = rhi::DepthStencilStateDesc {
            depth_test: true,
            depth_write: true,
            depth_compare_op: rhi::CompareOp::Greater,
            stencil_test: false,
            depth_bounds_enable: false,
            ..Default::default()
        };

        let vertex_layout = rhi::VertexInputStateDesc {
            input_bindings: &[rhi::VertexInputBindingDesc {
                binding: 0,
                stride: 60,
                input_rate: rhi::VertexInputRate::PerVertex,
            }],
            input_attributes: &[
                rhi::VertexInputAttributeDesc {
                    location: 0,
                    binding: 0,
                    format: rhi::VertexFormat::Float3,
                    offset: 0,
                },
                rhi::VertexInputAttributeDesc {
                    location: 1,
                    binding: 0,
                    format: rhi::VertexFormat::Float2,
                    offset: 12,
                },
                rhi::VertexInputAttributeDesc {
                    location: 2,
                    binding: 0,
                    format: rhi::VertexFormat::Float3,
                    offset: 20,
                },
                rhi::VertexInputAttributeDesc {
                    location: 3,
                    binding: 0,
                    format: rhi::VertexFormat::Float4,
                    offset: 32,
                },
                rhi::VertexInputAttributeDesc {
                    location: 4,
                    binding: 0,
                    format: rhi::VertexFormat::Float3,
                    offset: 48,
                },
            ],
        };

        let input_assembly_state = rhi::InputAssemblyStateDesc {
            primitive_topology: rhi::PrimitiveTopology::TriangleList,
        };

        let blend_state = rhi::BlendStateDesc {
            attachments: &[
                rhi::AttachmentBlendState {
                    blend_enabled: false,
                    color_write_mask: rhi::ColorComponentFlags::all(),
                    ..Default::default()
                },
                rhi::AttachmentBlendState {
                    blend_enabled: false,
                    color_write_mask: rhi::ColorComponentFlags::all(),
                    ..Default::default()
                },
                rhi::AttachmentBlendState {
                    blend_enabled: false,
                    color_write_mask: rhi::ColorComponentFlags::all(),
                    ..Default::default()
                },
            ],
        };

        let vertex_shader = material.material.vert_name();
        let vertex_shader = shader_db.load_stage(vertex_shader).unwrap();
        let fragment_shader = material.material.frag_name();
        let fragment_shader = shader_db.load_stage(fragment_shader).unwrap();

        let graphics_pipeline_desc_new = rhi::GraphicsPipelineDesc {
            shader_stages: &[vertex_shader, fragment_shader],
            binding_signature,
            vertex_layout: &vertex_layout,
            input_assembly_state: &input_assembly_state,
            rasterizer_state: &rasterizer_state,
            depth_stencil_state: &depth_stencil_state,
            blend_state: &blend_state,
            render_target_formats: &[
                rhi::Format::Rgba8UnormSrgb,
                rhi::Format::Rg16Unorm,
                rhi::Format::Rg8Unorm,
            ],
            depth_stencil_format: Some(rhi::Format::Depth32Float),
            name: rhi::obj_name_opt!("GraphicsPipeline"),
        };

        device
            .create_graphics_pipeline(&graphics_pipeline_desc_new)
            .unwrap()
    }
}
