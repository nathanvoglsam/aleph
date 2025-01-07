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
    BufferHandle, CameraInfo, IStateCacheKey, Material, MaterialId, MaterialInstanceObject,
    RenderSceneParam, RenderTransform, ShaderDatabaseAccessor, StateCache, StaticMesh,
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

        let key = MainOpaqueCommonLayout::key();
        let common_state =
            state_cache.get_or_insert_with(&key, |_, _| MainOpaqueCommonLayout::new(device));

        move |encoder, _graph, resources, args| unsafe {
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

            let global_set = descriptor_arena
                .allocate_set(common_state.global_set_layout.as_ref())
                .unwrap();
            device.update_descriptor_sets(&[DescriptorWriteDesc::uniform_buffer(
                global_set,
                0,
                &BufferDescriptorWrite::uniform_buffer(uniform_buffer, 256)
                    .with_offset(camera.device_offset as u64),
            )]);

            let model_set = descriptor_arena
                .allocate_set(common_state.model_set_layout.as_ref())
                .unwrap();
            device.update_descriptor_sets(&[DescriptorWriteDesc::uniform_buffer_dynamic(
                model_set,
                0,
                &BufferDescriptorWrite::uniform_buffer(uniform_buffer, 256),
            )]);

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

            let objects = scene.get_storage_ref::<StaticMesh>().unwrap();
            let (transforms, _meshes) = objects.as_slice_ref();

            let mut commands = Vec::from_iter(objects.iter().enumerate().map(|(i, (_, o))| {
                let m = o.material_instance;
                let m = args.material_instance_pool.get_ref(m).unwrap();

                let key = (m.material.id().get() as u64) << 32;
                let sort_key = key | o.material_instance.to_handle().to_fields().slot_index as u64;

                DrawCommand {
                    sort_key,
                    object_index: i,
                    vtx: o.vtx,
                    idx: o.idx,
                    mat: m,
                }
            }));
            commands.sort_unstable_by_key(|v| v.sort_key);

            if commands.is_empty() {
                return;
            }

            let mut current_material = commands[0].sort_key >> 32 & 0xFFFFFFFF;
            let mut current_material_instance = commands[0].mat;
            let mut current_material_state = {
                let mut state_cache = args.state_cache.lock();
                let key =
                    MainOpaqueMaterialLayout::key_for(current_material_instance.material.as_ref());
                state_cache.get_or_insert_with(&key, |cache, _| {
                    MainOpaqueMaterialLayout::new(
                        cache,
                        device,
                        common_state.as_ref(),
                        current_material_instance.material.as_ref(),
                    )
                })
            };

            encoder.bind_graphics_pipeline(current_material_state.pipeline.as_ref());

            encoder.bind_descriptor_sets(
                current_material_state.pipeline_layout.as_ref(),
                PipelineBindPoint::Graphics,
                0,
                &[global_set],
                &[],
            );

            let material_set = descriptor_arena
                .allocate_set(current_material_state.material_set_layout.as_ref())
                .unwrap();
            current_material_instance
                .material
                .material
                .update_descriptor_set(
                    args.buffer_pool,
                    args.texture_pool,
                    device,
                    current_material_instance,
                    material_set,
                );
            encoder.bind_descriptor_sets(
                current_material_state.pipeline_layout.as_ref(),
                PipelineBindPoint::Graphics,
                1,
                &[material_set],
                &[],
            );

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

            for command in commands {
                let this_material = command.sort_key >> 32 & 0xFFFFFFFF;

                if current_material != this_material {
                    current_material = this_material;

                    current_material_state = {
                        let mut state_cache = args.state_cache.lock();
                        let key = MainOpaqueMaterialLayout::key_for(
                            current_material_instance.material.as_ref(),
                        );
                        state_cache.get_or_insert_with(&key, |cache, _| {
                            MainOpaqueMaterialLayout::new(
                                cache,
                                device,
                                common_state.as_ref(),
                                current_material_instance.material.as_ref(),
                            )
                        })
                    };

                    encoder.bind_graphics_pipeline(current_material_state.pipeline.as_ref());

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
                }

                if !std::ptr::addr_eq(current_material_instance, command.mat) {
                    current_material_instance = command.mat;

                    let material_set = descriptor_arena
                        .allocate_set(current_material_state.material_set_layout.as_ref())
                        .unwrap();
                    current_material_instance
                        .material
                        .material
                        .update_descriptor_set(
                            args.buffer_pool,
                            args.texture_pool,
                            device,
                            current_material_instance,
                            material_set,
                        );
                    encoder.bind_descriptor_sets(
                        current_material_state.pipeline_layout.as_ref(),
                        PipelineBindPoint::Graphics,
                        1,
                        &[material_set],
                        &[],
                    );
                }

                // Bind the model's vertex buffers
                let vtx = args.buffer_pool.get_ref(command.vtx).unwrap();
                let vtx = vtx.get().unwrap();
                let idx = args.buffer_pool.get_ref(command.idx).unwrap();
                let idx = idx.get().unwrap();
                encoder.bind_vertex_buffers(0, &[InputAssemblyBufferBinding::new(vtx)]);
                encoder.bind_index_buffer(IndexType::U32, &InputAssemblyBufferBinding::new(idx));

                // Upload and rebind the per-model parameters
                let t = &transforms[command.object_index];
                let m = u_alloc
                    .allocate_object(ModelLayout::from_transform(t))
                    .unwrap();
                encoder.bind_descriptor_sets(
                    current_material_state.pipeline_layout.as_ref(),
                    PipelineBindPoint::Graphics,
                    2,
                    &[model_set],
                    &[m.device_offset as u32],
                );

                encoder.draw_indexed((idx.desc_ref().size / 4) as u32, 1, 0, 0, 0);
            }

            encoder.end_rendering();

            uniform_buffer.unmap().unwrap();
        }
    });
}

struct DrawCommand<'a> {
    sort_key: u64,
    object_index: usize,
    vtx: BufferHandle,
    idx: BufferHandle,
    mat: &'a MaterialInstanceObject,
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
struct ModelLayout {
    model_matrix: [f32; 16],
    normal_matrix: [f32; 16],
    _padding: [u8; 126],
}

impl ModelLayout {
    fn from_transform(v: &RenderTransform) -> Self {
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
            _padding: [0; 126],
        }
    }
}

#[derive(PartialEq, Eq, Hash)]
pub struct MainOpaqueCommonKey();

impl IStateCacheKey for MainOpaqueCommonKey {
    type Storage = MainOpaqueCommonLayout;
}

pub struct MainOpaqueCommonLayout {
    pub global_set_layout: AnyArc<dyn IDescriptorSetLayout>,
    pub model_set_layout: AnyArc<dyn IDescriptorSetLayout>,
}

impl MainOpaqueCommonLayout {
    pub const fn key() -> MainOpaqueCommonKey {
        MainOpaqueCommonKey()
    }

    pub fn new(device: &dyn IDevice) -> Self {
        let global_set_layout = Self::create_global_descriptor_set_layout(device);
        let model_set_layout = Self::create_model_descriptor_set_layout(device);

        Self {
            global_set_layout,
            model_set_layout,
        }
    }

    fn create_global_descriptor_set_layout(
        device: &dyn IDevice,
    ) -> AnyArc<dyn IDescriptorSetLayout> {
        let descriptor_set_layout_desc = DescriptorSetLayoutDesc {
            visibility: DescriptorShaderVisibility::All,
            items: &[DescriptorType::UniformBuffer.binding(0)],
            name: obj_name_opt!("GlobalDescriptorSetLayout"),
        };
        device
            .create_descriptor_set_layout(&descriptor_set_layout_desc)
            .unwrap()
    }

    fn create_model_descriptor_set_layout(
        device: &dyn IDevice,
    ) -> AnyArc<dyn IDescriptorSetLayout> {
        let descriptor_set_layout_desc = DescriptorSetLayoutDesc {
            visibility: DescriptorShaderVisibility::All,
            items: &[DescriptorType::UniformBufferDynamic.binding(0)],
            name: obj_name_opt!("ModelDescriptorSetLayout"),
        };
        device
            .create_descriptor_set_layout(&descriptor_set_layout_desc)
            .unwrap()
    }
}

#[derive(PartialEq, Eq, Hash)]
pub struct MainOpaqueMaterialKey(MaterialId);

impl IStateCacheKey for MainOpaqueMaterialKey {
    type Storage = MainOpaqueMaterialLayout;
}

pub struct MainOpaqueMaterialLayout {
    pub global_set_layout: AnyArc<dyn IDescriptorSetLayout>,
    pub material_set_layout: AnyArc<dyn IDescriptorSetLayout>,
    pub model_set_layout: AnyArc<dyn IDescriptorSetLayout>,
    pub pipeline_layout: AnyArc<dyn IPipelineLayout>,
    pub pipeline: AnyArc<dyn IGraphicsPipeline>,
}

impl MainOpaqueMaterialLayout {
    pub const fn key_for(material: &Material) -> MainOpaqueMaterialKey {
        MainOpaqueMaterialKey(material.id())
    }

    pub fn new(
        cache: &mut StateCache,
        device: &dyn IDevice,
        common: &MainOpaqueCommonLayout,
        material: &Material,
    ) -> Self {
        let global_set_layout = common.global_set_layout.clone();
        let material_set_layout = Self::create_material_descriptor_set_layout(device, material);
        let model_set_layout = common.model_set_layout.clone();
        let pipeline_layout = Self::create_pipeline_layout(
            device,
            global_set_layout.as_ref(),
            material_set_layout.as_ref(),
            model_set_layout.as_ref(),
        );
        let pipeline = Self::create_pipeline_state(
            device,
            cache.shader_db(),
            pipeline_layout.as_ref(),
            material,
        );

        Self {
            global_set_layout,
            material_set_layout,
            model_set_layout,
            pipeline_layout,
            pipeline,
        }
    }

    fn create_material_descriptor_set_layout(
        device: &dyn IDevice,
        material: &Material,
    ) -> AnyArc<dyn IDescriptorSetLayout> {
        material.material.create_descriptor_set_layout(device)
    }

    fn create_pipeline_layout(
        device: &dyn IDevice,
        global: &dyn IDescriptorSetLayout,
        material: &dyn IDescriptorSetLayout,
        model: &dyn IDescriptorSetLayout,
    ) -> AnyArc<dyn IPipelineLayout> {
        let pipeline_layout_desc = PipelineLayoutDesc {
            set_layouts: &[global, material, model],
            push_constant_blocks: &[],
            name: obj_name_opt!("RootSignature"),
        };
        device
            .create_pipeline_layout(&pipeline_layout_desc)
            .unwrap()
    }

    fn create_pipeline_state(
        device: &dyn IDevice,
        shader_db: &ShaderDatabaseAccessor,
        pipeline_layout: &dyn IPipelineLayout,
        material: &Material,
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

        let vertex_shader = material.material.vert_name();
        let vertex_shader = shader_db.load_stage(vertex_shader).unwrap();
        let fragment_shader = material.material.frag_name();
        let fragment_shader = shader_db.load_stage(fragment_shader).unwrap();

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
}
