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

use aleph_any::AnyArc;
use aleph_device_allocators::LinearDescriptorPool;
use aleph_frame_graph::*;
use aleph_nstr::nstr;
use aleph_pin_board::{BoardParamId, PinBoard};
use aleph_rhi_api::*;

use crate::pass::composite_planes::CompositePlanesLayout;
use crate::pass::GraphArgs;
use crate::{
    shaders, BufferHandle, BufferStreamingRequest, BufferUploadDesc, GenerateMips, IStateCacheKey,
    ShaderDatabaseAccessor, StateCache, TextureHandle, TextureStreamingRequest, TextureUploadDesc,
};

pub struct ResourceProcessorParam {
    pub buffer_requests: Vec<BufferLoadRequest>,
    pub texture_requests: Vec<TextureLoadRequest>,
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

            // TODO: we want to batch all of these, so we need a better interface so we can bundle
            //       the barriers and copy commands for all our loaders into a single batch.
            //
            //       either that or we unify to a single loader type.
            BufferLoader::upload_requests(&input.buffer_requests, encoder);
            TextureLoader::upload_requests(
                device,
                &input.texture_requests,
                encoder,
                args.state_cache,
                arena,
            );
        }
    });
}

struct BufferLoader();

impl BufferLoader {
    pub(crate) unsafe fn upload_requests(
        requests: &[BufferLoadRequest],
        encoder: &mut dyn IGeneralEncoder,
    ) {
        let mut discard_barriers = Vec::new();
        let mut buffers = Vec::new();
        let mut release_barriers = Vec::new();

        for request in requests.iter() {
            Self::process_request(
                &mut discard_barriers,
                &mut buffers,
                &mut release_barriers,
                request,
            );
        }

        // If there are no buffers to upload then we early exit (we don't want to issue empty
        // resource barriers)
        if buffers.is_empty() {
            return;
        }

        // Prepare all our resources in a single barrier command rather than 'n' barriers
        encoder.resource_barrier(&[], &discard_barriers, &[]);

        for upload in buffers.drain(..) {
            let region = upload.load.data.get_copy_region(0);
            encoder.copy_buffer_regions(upload.load.data.buffer.buffer(), upload.buffer, &[region]);

            if let Some(req) = upload.load.request.as_ref() {
                req.mark_complete(upload.load.target).unwrap();
            }
        }

        // Sync our uploaded resources with the access scopes we consider them safe to use
        encoder.resource_barrier(&[], &release_barriers, &[]);
    }

    fn process_request<'r>(
        discard_barriers: &mut Vec<BufferBarrier<'r>>,
        buffers: &mut Vec<BufferUpload<'r>>,
        release_barriers: &mut Vec<BufferBarrier<'r>>,
        request: &'r BufferLoadRequest,
    ) {
        let buffer = request.buffer.as_ref();

        // Need to drop to raw pointers because the borrow checker won't be able to prove what
        // we're doing is safe.
        //
        // Arc's address is stable, but we need to 'move' it into the buffers array. The borrow
        // checker will complain the move is impossible due to outstanding borrows.
        //
        // This is safe because:
        // - All access are through shared references
        // - The address is stable
        let size = request.data.desc.size.get();
        let usage = request.data.desc.usage;

        buffers.push(BufferUpload {
            load: request,
            buffer,
        });

        discard_barriers.push(BufferBarrier {
            buffer: Some(buffer),
            offset: 0,
            size,
            before_sync: BarrierSync::NONE,
            after_sync: BarrierSync::COPY,
            before_access: BarrierAccess::NONE,
            after_access: BarrierAccess::COPY_WRITE,
            queue_transition: None,
        });

        release_barriers.push(BufferBarrier {
            buffer: Some(buffer),
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

pub struct BufferLoadRequest {
    /// Target resource for the upload operation
    pub target: BufferHandle,

    /// Target request object to send request notifications through. If `None` then all
    /// notifications will be dropped.
    pub request: Option<BufferStreamingRequest>,

    /// The actual data source for the upload.
    pub data: BufferUploadDesc,

    /// Destination buffer
    pub buffer: AnyArc<dyn IBuffer>,
}

struct BufferUpload<'a> {
    load: &'a BufferLoadRequest,
    buffer: &'a dyn IBuffer,
}

struct TextureLoader();

impl TextureLoader {
    pub(crate) unsafe fn upload_requests(
        device: &dyn IDevice,
        requests: &[TextureLoadRequest],
        encoder: &mut dyn IGeneralEncoder,
        state_cache: &StateCache,
        arena: &LinearDescriptorPool,
    ) {
        let mut discard_barriers = Vec::new();
        let mut textures = Vec::new();
        let mut release_barriers = Vec::new();

        for request in requests {
            Self::process_request(
                &mut discard_barriers,
                &mut textures,
                &mut release_barriers,
                request,
            );
        }

        // If there are no textures to upload then we early exit (we don't want to issue empty
        // resource barriers)
        if textures.is_empty() {
            return;
        }

        // Prepare all our resources in a single barrier command rather than 'n' barriers
        encoder.resource_barrier(&[], &[], &discard_barriers);

        for upload in textures.iter() {
            let data = &upload.load.data;
            for level in data.data.level_range() {
                let region = data.get_copy_region(level, TextureCopyAspect::Color);
                encoder.copy_buffer_to_texture(
                    upload.load.data.buffer.buffer(),
                    upload.texture,
                    &[region],
                );
            }

            if let Some(req) = upload.load.request.as_ref() {
                req.mark_complete(upload.load.handle).unwrap();
            }
        }

        // Sync our uploaded resources with the access scopes we consider them safe to use
        if !release_barriers.is_empty() {
            encoder.resource_barrier(&[], &[], &release_barriers);
        }

        for upload in textures.drain(..) {
            match upload.load.mips {
                GenerateMips::Yes => {
                    MipGenerator::record(
                        device,
                        state_cache,
                        arena,
                        encoder,
                        upload.texture,
                        upload.load.data.desc.usage,
                    );
                }
                GenerateMips::No => {
                    // Do nothing here, the texture has been correctly synced with the outside
                    // users
                }
            }
        }
    }

    unsafe fn process_request<'r>(
        discard_barriers: &mut Vec<TextureBarrier<'r>>,
        textures: &mut Vec<TextureUpload<'r>>,
        release_barriers: &mut Vec<TextureBarrier<'r>>,
        request: &'r TextureLoadRequest,
    ) {
        let texture = request.texture.as_ref();
        let tex_desc = texture.desc_ref();
        let subresources = TextureSubResourceSet::all(tex_desc);

        let usage = request.data.desc.usage;
        let mips = request.mips;
        let format = tex_desc.format;
        textures.push(TextureUpload {
            load: request,
            texture,
        });

        discard_barriers.push(TextureBarrier {
            texture: Some(texture),
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
                    texture: Some(texture),
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

pub struct TextureLoadRequest {
    /// Target resource for the upload operation
    pub handle: TextureHandle,

    /// Target request object to send request notifications through. If `None` then all
    /// notifications will be dropped.
    pub request: Option<TextureStreamingRequest>,

    /// The actual data source for the upload.
    pub data: TextureUploadDesc,

    /// Destination texture
    pub texture: AnyArc<dyn ITexture>,

    /// Request the renderer to generate the mipmaps for the texture once loaded
    pub mips: GenerateMips,
}

struct TextureUpload<'a> {
    load: &'a TextureLoadRequest,
    texture: &'a dyn ITexture,
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
        state_cache: &StateCache,
        arena: &LinearDescriptorPool,
        encoder: &mut dyn IGeneralEncoder,
        texture: &dyn ITexture,
        usage: ResourceUsageFlags,
    ) {
        let desc = texture.desc_ref();
        let format = desc.format;

        // If the texture only has 1 mip level then there's nothing to do, so we early exit
        if desc.mip_levels <= 1 {
            return;
        }

        let key = MipGeneratorState::key(desc.format);
        let state = state_cache.get(&key).unwrap();

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

            encoder.resource_barrier(&[], &[], &barrier_queue);
            barrier_queue.clear();

            let extent = Extent2D {
                width: desc.width >> level,
                height: desc.height >> level,
            };
            let image_view = texture
                .get_rtv(&ImageViewDesc {
                    format,
                    view_type: ImageViewType::Tex2D,
                    sub_resources: dst_subresource_range.clone(),
                    writable: true,
                })
                .unwrap();
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

            encoder.bind_graphics_pipeline(state.pipeline.as_ref());
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

            let src_view = texture
                .get_view(&ImageViewDesc {
                    format,
                    view_type: ImageViewType::Tex2D,
                    sub_resources: TextureSubResourceSet::with_color().with_mips(level - 1, 1),
                    writable: false,
                })
                .unwrap();
            let set = arena
                .allocate_set(state.layout.set_layout.as_ref())
                .unwrap();
            device.update_descriptor_sets(&[DescriptorWriteDesc {
                set,
                binding: 0,
                array_element: 0,
                writes: DescriptorWrites::Texture(&[ImageDescriptorWrite::srv(src_view)]),
            }]);

            encoder.bind_descriptor_sets(
                state.layout.pipeline_layout.as_ref(),
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

        encoder.resource_barrier(&[], &[], &barrier_queue);
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
    pub pipeline: AnyArc<dyn IGraphicsPipeline>,
}

impl MipGeneratorState {
    pub fn key(format: Format) -> MipGeneratorStateKey {
        MipGeneratorStateKey(format)
    }

    pub fn new(cache: &mut StateCache, device: &dyn IDevice, format: Format) -> Self {
        let key = CompositePlanesLayout::key();
        let layout = cache.get_or_insert_with(&key, |_, _| CompositePlanesLayout::new(device));

        let pipeline = Self::create_pipeline_state(
            device,
            layout.pipeline_layout.as_ref(),
            cache.shader_db(),
            format,
        );

        Self { layout, pipeline }
    }

    pub fn create_pipeline_state(
        device: &dyn IDevice,
        pipeline_layout: &dyn IPipelineLayout,
        shader_db: &ShaderDatabaseAccessor,
        format: Format,
    ) -> AnyArc<dyn IGraphicsPipeline> {
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
