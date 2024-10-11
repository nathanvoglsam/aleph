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
use aleph_rhi_api::*;

use crate::pass::composite_planes::CompositePlanesLayout;
use crate::{shaders, IStateCacheKey, ShaderDatabaseAccessor, StateCache};

pub struct MipGenerator {
    device: AnyArc<dyn IDevice>,
}

impl MipGenerator {
    pub fn new(device: AnyArc<dyn IDevice>, state_cache: &mut StateCache) -> Self {
        // Pre warm the cache with pipelines for the following common formats
        let pre_warm_formats = [
            Format::Rgba8Unorm,
            Format::Rgba8UnormSrgb,
            Format::Bgra8Unorm,
            Format::Bgra8UnormSrgb,
        ];
        for format in pre_warm_formats {
            let key = MipGeneratorState::key(format);
            let _ = state_cache.get_or_insert_with(&key, |cache, k| {
                MipGeneratorState::new(cache, device.as_ref(), k.0)
            });
        }
        Self { device }
    }

    pub unsafe fn record(
        &self,
        state_cache: &mut StateCache,
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
        let state = state_cache.get_or_insert_with(&key, |cache, k| {
            MipGeneratorState::new(cache, self.device.as_ref(), k.0)
        });

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
            self.device.update_descriptor_sets(&[DescriptorWriteDesc {
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
