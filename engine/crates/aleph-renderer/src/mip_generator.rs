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

use std::collections::HashMap;

use aleph_any::AnyArc;
use aleph_device_allocators::LinearDescriptorPool;
use aleph_object_system::unsafe_impl_iobject;
use aleph_rhi_api::*;

use crate::pass::composite_planes::CompositePlanesState;
use crate::ShaderDatabaseAccessor;

pub struct MipGenerator {
    device: AnyArc<dyn IDevice>,
    state: MipGeneratorState,
}

impl MipGenerator {
    pub fn new(device: AnyArc<dyn IDevice>, shader_db: &ShaderDatabaseAccessor) -> Self {
        let state = MipGeneratorState::new(
            device.as_ref(),
            shader_db,
            &[
                Format::Rgba8Unorm,
                Format::Rgba8UnormSrgb,
                Format::Bgra8Unorm,
                Format::Bgra8UnormSrgb,
            ],
        );
        Self { device, state }
    }

    pub unsafe fn record(
        &self,
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

        let pipeline = self
            .state
            .pipelines
            .get(&format)
            .map(|v| v.as_ref())
            .unwrap();

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

            encoder.bind_graphics_pipeline(pipeline);
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
                .allocate_set(self.state.descriptor_set_layout.as_ref())
                .unwrap();
            self.device.update_descriptor_sets(&[DescriptorWriteDesc {
                set,
                binding: 0,
                array_element: 0,
                writes: DescriptorWrites::Texture(&[ImageDescriptorWrite::srv(src_view)]),
            }]);

            encoder.bind_descriptor_sets(
                self.state.pipeline_layout.as_ref(),
                PipelineBindPoint::Graphics,
                0,
                &[set],
                &[],
            );

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

pub struct MipGeneratorState {
    pub descriptor_set_layout: AnyArc<dyn IDescriptorSetLayout>,
    pub pipeline_layout: AnyArc<dyn IPipelineLayout>,
    pub pipelines: HashMap<Format, AnyArc<dyn IGraphicsPipeline>>,
}
unsafe_impl_iobject!(MipGeneratorState, "019275bf-fcc0-77a0-89c4-e09002dfcdd4");

impl MipGeneratorState {
    pub fn new(
        device: &dyn IDevice,
        shader_db: &ShaderDatabaseAccessor,
        supported_formats: &[Format],
    ) -> Self {
        let sampler = CompositePlanesState::create_sampler(device);
        let descriptor_set_layout =
            CompositePlanesState::create_descriptor_set_layout(device, sampler.as_ref());
        let pipeline_layout =
            CompositePlanesState::create_pipeline_layout(device, descriptor_set_layout.as_ref());

        let mut pipelines = HashMap::new();
        for format in supported_formats {
            let pipeline = CompositePlanesState::create_pipeline_state(
                device,
                pipeline_layout.as_ref(),
                shader_db,
                *format,
            );
            pipelines.insert(*format, pipeline);
        }

        Self {
            descriptor_set_layout,
            pipeline_layout,
            pipelines,
        }
    }
}
