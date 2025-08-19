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

use std::sync::Arc;

use aleph_device_allocators::LinearDescriptorPool;
use aleph_frame_graph::*;
use aleph_nstr::nstr;
use aleph_pin_board::{BoardParamId, PinBoard};
use aleph_rhi_api::*;
use parking_lot::Mutex;

use crate::pass::GraphArgs;
use crate::pass::composite_planes::CompositePlanesLayout;
use crate::{
    BufferObject, BufferUploadDesc, IShaderAccessor, IShaderAccessorExt, IStateCacheKey,
    ResourceCommand, ResourceCommandBuffer, StateCache, TextureObject, TextureUploadDesc, shaders,
};

pub struct ResourceProcessorParam {
    pub resource_commands: ResourceCommandBuffer,
}
impl BoardParamId for ResourceProcessorParam {
    type Output<'a> = ResourceProcessorParam;
}

pub struct ResourceProcessorOutput {
    /// Execution token for the 'ResourceProcessor' pass.
    pub exec: ResourceRef,
}

pub fn pass(
    frame_graph: &mut FrameGraphBuilder<GraphArgs>,
    device: &dyn IDevice,
    pin_board: &PinBoard,
    state_cache: &mut StateCache,
) {
    frame_graph.add_pass(nstr!("ResourceProcessor"), |resources| {
        MipGenerator::pre_warm(device, state_cache);

        let exec = resources.create_exec_token("ResourceProcessorToken");
        pin_board.publish(ResourceProcessorOutput { exec });

        move |encoder, _graph, resources, args| unsafe {
            let device = resources.device();
            let arena = resources.descriptor_arena();
            let input = args.board.get::<ResourceProcessorParam>().unwrap();

            let mut buffer_requests = Vec::new();
            let mut texture_requests = Vec::new();
            input.resource_commands.walk(|_, c| match c {
                ResourceCommand::BufferUpload(h, u) => {
                    let object = args.buffer_pool.get_ref(*h).unwrap();
                    let buffer = object.get().unwrap();

                    buffer_requests.push(BufferLoadRequest {
                        data: u,
                        object,
                        buffer,
                    });
                }
                ResourceCommand::TextureUpload(h, m, u) => {
                    let object = args.texture_pool.get_ref(*h).unwrap();
                    let texture = object.get().unwrap();

                    texture_requests.push(TextureLoadRequest {
                        data: u,
                        object,
                        texture,
                        mips: *m,
                    });
                }
            });

            // TODO: we want to batch all of these, so we need a better interface so we can bundle
            //       the barriers and copy commands for all our loaders into a single batch.
            //
            //       either that or we unify to a single loader type.
            BufferLoader::upload_requests(&buffer_requests, encoder);
            TextureLoader::upload_requests(
                device,
                &texture_requests,
                encoder,
                args.state_cache,
                arena,
            );
        }
    });
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum GenerateMips {
    Yes,
    No,
}

impl Default for GenerateMips {
    fn default() -> Self {
        Self::Yes
    }
}

struct BufferLoader();

impl BufferLoader {
    pub(crate) unsafe fn upload_requests(
        requests: &[BufferLoadRequest],
        encoder: &mut dyn IGeneralEncoder,
    ) {
        let mut discard_barriers = Vec::new();
        let mut release_barriers = Vec::new();

        for request in requests.iter() {
            Self::process_request(&mut discard_barriers, &mut release_barriers, request);
        }

        // If there are no buffers to upload then we early exit (we don't want to issue empty
        // resource barriers)
        if requests.is_empty() {
            return;
        }

        unsafe {
            // Prepare all our resources in a single barrier command rather than 'n' barriers
            encoder.resource_barrier(&[], &discard_barriers, &[]);

            for r in requests.iter() {
                let buffer = r.object.get().unwrap();

                let region = r.data.get_copy_region(r.object.desc(), 0);
                encoder.copy_buffer_regions(r.data.buffer.buffer(), buffer, &[region]);
            }

            // Sync our uploaded resources with the access scopes we consider them safe to use
            encoder.resource_barrier(&[], &release_barriers, &[]);
        }
    }

    fn process_request<'r>(
        discard_barriers: &mut Vec<BufferBarrier<'r>>,
        release_barriers: &mut Vec<BufferBarrier<'r>>,
        request: &'r BufferLoadRequest,
    ) {
        let desc = request.object.desc();
        let size = desc.size.get();
        let usage = desc.usage;
        discard_barriers.push(BufferBarrier {
            buffer: Some(request.buffer),
            offset: 0,
            size,
            before_sync: BarrierSync::NONE,
            after_sync: BarrierSync::COPY,
            before_access: BarrierAccess::NONE,
            after_access: BarrierAccess::COPY_WRITE,
            queue_transition: None,
        });

        release_barriers.push(BufferBarrier {
            buffer: Some(request.buffer),
            offset: 0,
            size,
            before_sync: BarrierSync::COPY,
            after_sync: usage.default_barrier_sync(true, Format::R8Unorm),
            before_access: BarrierAccess::COPY_WRITE,
            after_access: usage.barrier_access_for_read(Format::R8Unorm),
            queue_transition: None,
        });
    }
}

struct BufferLoadRequest<'a> {
    /// The actual data source for the upload.
    data: &'a BufferUploadDesc,

    /// The target buffer object
    object: &'a BufferObject,

    /// The target API buffer object
    buffer: &'a BufferHandle,
}

struct TextureLoader();

impl TextureLoader {
    pub(crate) unsafe fn upload_requests(
        device: &dyn IDevice,
        requests: &[TextureLoadRequest],
        encoder: &mut dyn IGeneralEncoder,
        state_cache: &Mutex<StateCache>,
        arena: &LinearDescriptorPool,
    ) {
        let mut discard_barriers = Vec::new();
        let mut release_barriers = Vec::new();

        for request in requests {
            unsafe {
                Self::process_request(
                    device,
                    &mut discard_barriers,
                    &mut release_barriers,
                    request,
                );
            }
        }

        // If there are no textures to upload then we early exit (we don't want to issue empty
        // resource barriers)
        if requests.is_empty() {
            return;
        }

        unsafe {
            // Prepare all our resources in a single barrier command rather than 'n' barriers
            encoder.resource_barrier(&[], &[], &discard_barriers);

            for r in requests.iter() {
                let data = r.data;
                for level in data.data.level_range() {
                    let desc = r.object.desc();
                    let texture = r.texture;
                    let region = data.get_copy_region(desc, level, TextureCopyAspect::Color);
                    encoder.copy_buffer_to_texture(r.data.buffer.buffer(), texture, &[region]);
                }
            }

            // Sync our uploaded resources with the access scopes we consider them safe to use
            if !release_barriers.is_empty() {
                encoder.resource_barrier(&[], &[], &release_barriers);
            }
        }

        for r in requests.iter() {
            match r.mips {
                GenerateMips::Yes => {
                    let desc = r.object.desc();
                    let texture = r.texture;
                    unsafe {
                        MipGenerator::record(
                            device,
                            state_cache,
                            arena,
                            encoder,
                            texture,
                            desc.usage,
                        )
                    };
                }
                GenerateMips::No => {
                    // Do nothing here, the texture has been correctly synced with the outside
                    // users
                }
            }
        }
    }

    unsafe fn process_request<'r>(
        device: &dyn IDevice,
        discard_barriers: &mut Vec<TextureBarrier<'r>>,
        release_barriers: &mut Vec<TextureBarrier<'r>>,
        request: &'r TextureLoadRequest,
    ) {
        let tex_desc = device.get_texture_desc(&request.texture);
        let subresources = TextureSubResourceSet::all(tex_desc);

        let usage = request.object.desc().usage;
        let mips = request.mips;
        let format = tex_desc.format;
        discard_barriers.push(TextureBarrier {
            texture: Some(request.texture),
            subresource_range: subresources.clone(),
            before_sync: BarrierSync::NONE,
            after_sync: BarrierSync::COPY,
            before_access: BarrierAccess::NONE,
            after_access: BarrierAccess::COPY_WRITE,
            before_layout: ImageLayout::Undefined,
            after_layout: ImageLayout::CopyDst,
            queue_transition: None,
        });

        match mips {
            GenerateMips::Yes => {
                // We don't do anything here because the mip generator expects the texture to be
                // in the copy layout still.
                //
                // We handle this after the copies have been completed
            }
            GenerateMips::No => {
                release_barriers.push(TextureBarrier {
                    texture: Some(request.texture),
                    subresource_range: subresources,
                    before_sync: BarrierSync::COPY,
                    after_sync: usage.default_barrier_sync(true, format),
                    before_access: BarrierAccess::COPY_WRITE,
                    after_access: usage.barrier_access_for_read(format),
                    before_layout: ImageLayout::CopyDst,
                    after_layout: usage.image_layout(true, format),
                    queue_transition: None,
                });
            }
        }
    }
}

struct TextureLoadRequest<'a> {
    /// The actual data source for the upload.
    data: &'a TextureUploadDesc,

    /// The target texture object
    object: &'a TextureObject,

    /// The target texture API object
    texture: &'a TextureHandle,

    /// Request the renderer to generate the mipmaps for the texture once loaded
    mips: GenerateMips,
}

pub const PREWARM_MIP_FORMATS: [Format; 4] = [
    Format::Rgba8Unorm,
    Format::Rgba8UnormSrgb,
    Format::Bgra8Unorm,
    Format::Bgra8UnormSrgb,
];

struct MipGenerator();

impl MipGenerator {
    fn pre_warm(device: &dyn IDevice, state_cache: &mut StateCache) {
        for format in PREWARM_MIP_FORMATS {
            let key = MipGeneratorState::key(format);
            let _ = state_cache
                .get_or_insert_with(&key, |cache, k| MipGeneratorState::new(cache, device, k.0));
        }
    }

    unsafe fn record(
        device: &dyn IDevice,
        state_cache: &Mutex<StateCache>,
        arena: &LinearDescriptorPool,
        encoder: &mut dyn IGeneralEncoder,
        texture: &TextureHandle,
        usage: ResourceUsageFlags,
    ) {
        let desc = device.get_texture_desc(texture);
        let format = desc.format;

        // If the texture only has 1 mip level then there's nothing to do, so we early exit
        if desc.mip_levels <= 1 {
            return;
        }

        let state = {
            let mut state_cache = state_cache.lock();
            let key = MipGeneratorState::key(desc.format);
            state_cache
                .get_or_insert_with(&key, |cache, k| MipGeneratorState::new(cache, device, k.0))
        };

        let mut barrier_queue = Vec::new();
        barrier_queue.push(TextureBarrier {
            texture: Some(texture),
            subresource_range: TextureSubResourceSet::with_color().with_mips(0, 1),
            before_sync: BarrierSync::COPY,
            after_sync: BarrierSync::PIXEL_SHADING,
            before_access: BarrierAccess::COPY_WRITE,
            after_access: BarrierAccess::SHADER_READ,
            before_layout: ImageLayout::CopyDst,
            after_layout: ImageLayout::ShaderReadOnly,
            queue_transition: None,
        });

        let mip_levels = desc.mip_levels;
        for level in 1..mip_levels {
            let dst_subresource_range = TextureSubResourceSet::with_color().with_mips(level, 1);
            barrier_queue.push(TextureBarrier {
                texture: Some(texture),
                subresource_range: dst_subresource_range.clone(),
                before_sync: BarrierSync::COPY,
                after_sync: BarrierSync::RENDER_TARGET,
                before_access: BarrierAccess::COPY_WRITE,
                after_access: BarrierAccess::RENDER_TARGET_WRITE,
                before_layout: ImageLayout::CopyDst,
                after_layout: ImageLayout::ColorAttachment,
                queue_transition: None,
            });

            unsafe {
                encoder.resource_barrier(&[], &[], &barrier_queue);
                barrier_queue.clear();

                let extent = Extent2D {
                    width: desc.width >> level,
                    height: desc.height >> level,
                };
                let desc = ImageViewDesc {
                    format,
                    view_type: ImageViewType::Tex2D,
                    sub_resources: dst_subresource_range.clone(),
                    writable: true,
                };
                let image_view = device.get_texture_rtv(texture, &desc).unwrap();
                encoder.begin_rendering(&BeginRenderingInfo {
                    layer_count: 1,
                    extent: extent.clone(),
                    color_attachments: &[RenderingColorAttachmentInfo {
                        image_view,
                        image_layout: ImageLayout::ColorAttachment,
                        load_op: AttachmentLoadOp::DontCare,
                        store_op: AttachmentStoreOp::Store,
                    }],
                    depth_stencil_attachment: None,
                    allow_uav_writes: false,
                });

                encoder.bind_graphics_pipeline(&state.pipeline);
                encoder.set_viewports(&[Viewport {
                    x: 0.0,
                    y: 0.0,
                    width: extent.width as f32,
                    height: extent.height as f32,
                    min_depth: 0.0,
                    max_depth: 1.0,
                }]);
                encoder.set_scissor_rects(&[Rect {
                    x: 0,
                    y: 0,
                    w: extent.width,
                    h: extent.height,
                }]);

                let desc = ImageViewDesc {
                    format,
                    view_type: ImageViewType::Tex2D,
                    sub_resources: TextureSubResourceSet::with_color().with_mips(level - 1, 1),
                    writable: false,
                };
                let src_view = device.get_texture_view(texture, &desc).unwrap();
                let set = arena.allocate_set(&state.layout.set_layout).unwrap();
                device.update_descriptor_sets(&[DescriptorWriteDesc {
                    set,
                    binding: 0,
                    array_element: 0,
                    writes: DescriptorWrites::Texture(&[ImageDescriptorWrite::srv(src_view)]),
                }]);

                encoder.bind_descriptor_sets(
                    &state.layout.pipeline_layout,
                    PipelineBindPoint::Graphics,
                    0,
                    &[set],
                    &[],
                );

                let src_level = (level - 1) as f32;
                encoder.set_push_constant_block(0, bytemuck::bytes_of(&src_level));

                encoder.draw(3, 1, 0, 0);

                encoder.end_rendering();

                barrier_queue.push(TextureBarrier {
                    texture: Some(texture),
                    subresource_range: dst_subresource_range,
                    before_sync: BarrierSync::RENDER_TARGET,
                    after_sync: BarrierSync::PIXEL_SHADING,
                    before_access: BarrierAccess::RENDER_TARGET_WRITE,
                    after_access: BarrierAccess::SHADER_READ,
                    before_layout: ImageLayout::ColorAttachment,
                    after_layout: ImageLayout::ShaderReadOnly,
                    queue_transition: None,
                });
            }
        }

        // Adjust the last barrier, which instead of syncing with usage as an srv is instead syncing
        // with the final resource usage
        barrier_queue[0].after_sync = usage.default_barrier_sync(true, format);
        barrier_queue[0].after_access = usage.barrier_access_for_read(format);
        barrier_queue[0].after_layout = usage.image_layout(true, format);

        // And do the same for all the resources that were just used as srv.
        barrier_queue.push(TextureBarrier {
            texture: Some(texture),
            subresource_range: TextureSubResourceSet {
                aspect: TextureAspect::COLOR,
                base_mip_level: 0,
                num_mip_levels: desc.mip_levels - 2,
                base_array_slice: 0,
                num_array_slices: 1,
            },
            before_sync: BarrierSync::PIXEL_SHADING,
            after_sync: usage.default_barrier_sync(true, format),
            before_access: BarrierAccess::SHADER_READ,
            after_access: usage.barrier_access_for_read(format),
            before_layout: ImageLayout::ShaderReadOnly,
            after_layout: usage.image_layout(true, format),
            queue_transition: None,
        });

        unsafe {
            encoder.resource_barrier(&[], &[], &barrier_queue);
        }

        barrier_queue.clear();
    }
}

#[derive(PartialEq, Eq, Hash)]
pub struct MipGeneratorStateKey(pub Format);

impl IStateCacheKey for MipGeneratorStateKey {
    type Storage = MipGeneratorState;
}

pub struct MipGeneratorState {
    pub layout: Arc<CompositePlanesLayout>,
    pub pipeline: GraphicsPipelineHandle,
}

impl MipGeneratorState {
    pub fn key(format: Format) -> MipGeneratorStateKey {
        MipGeneratorStateKey(format)
    }

    pub fn new(cache: &mut StateCache, device: &dyn IDevice, format: Format) -> Self {
        let key = CompositePlanesLayout::key();
        let layout = cache.get_or_insert_with(&key, |_, _| CompositePlanesLayout::new(device));

        let pipeline =
            Self::create_pipeline_state(device, &layout.pipeline_layout, cache.shader_db(), format);

        Self { layout, pipeline }
    }

    pub fn create_pipeline_state(
        device: &dyn IDevice,
        pipeline_layout: &PipelineLayoutHandle,
        shader_db: &dyn IShaderAccessor,
        format: Format,
    ) -> GraphicsPipelineHandle {
        let vertex_shader = shader_db
            .load_stage(shaders::composite_planes::vert())
            .unwrap();
        let fragment_shader = shader_db
            .load_stage(shaders::composite_planes::frag())
            .unwrap();

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
            name: obj_name_opt!("GraphicsPipelineState"),
        };

        device
            .create_graphics_pipeline(&graphics_pipeline_desc_new)
            .unwrap()
    }
}
