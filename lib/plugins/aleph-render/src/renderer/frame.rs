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

use std::ops::Deref;

use aleph_rhi_api::*;
use interfaces::any::AnyArc;

use crate::renderer::FontTexture;

pub struct PerFrameObjects {
    pub acquire_semaphore: AnyArc<dyn ISemaphore>,
    pub present_semaphore: AnyArc<dyn ISemaphore>,

    pub uniform_buffer: AnyArc<dyn IBuffer>,

    pub font_version: usize,
    pub font_staging_buffer: AnyArc<dyn IBuffer>,

    pub font_staged: Option<AnyArc<dyn ITexture>>,
    pub font_staged_size: (u32, u32),

    pub descriptor_pool: Box<dyn IDescriptorPool>,
    pub descriptor_set: DescriptorSetHandle,

    pub done_fence: AnyArc<dyn IFence>,
}

impl PerFrameObjects {
    pub fn new(device: &dyn IDevice, set_layout: &dyn IDescriptorSetLayout) -> Self {
        let font_staging_buffer = Self::create_font_staging_allocation(device, (4096, 4096));

        let desc = DescriptorPoolDesc {
            layout: set_layout,
            num_sets: 2,
            name: Some("egui::DescriptorPool"),
        };
        let mut descriptor_pool = device.create_descriptor_pool(&desc).unwrap();
        let set = descriptor_pool.allocate_set().unwrap();

        let uniform_buffer = device
            .create_buffer(&BufferDesc {
                size: 1024,
                cpu_access: CpuAccessMode::Write,
                usage: ResourceUsageFlags::CONSTANT_BUFFER,
                name: Some("egui::ConstantBuffer"),
            })
            .unwrap();
        unsafe {
            device.update_descriptor_sets(&[DescriptorWriteDesc::uniform_buffer(
                set,
                0,
                &BufferDescriptorWrite::uniform_buffer(uniform_buffer.as_ref(), 256),
            )]);
        }

        Self {
            acquire_semaphore: device.create_semaphore().unwrap(),
            present_semaphore: device.create_semaphore().unwrap(),
            uniform_buffer,
            font_version: 0,
            font_staging_buffer,
            font_staged: None,
            font_staged_size: (0, 0),
            descriptor_pool,
            descriptor_set: set,
            done_fence: device.create_fence(true).unwrap(),
        }
    }

    pub unsafe fn update_texture_data(
        &mut self,
        device: &dyn IDevice,
        sampler: &dyn ISampler,
        texture: &FontTexture,
    ) {
        // Check the data is correct
        assert_eq!(texture.bytes.len(), texture.width * texture.height);

        // Crunch our dimensions for d3d12
        let dimensions = (texture.width as u32, texture.height as u32);

        // Explicitly drop staged image so the pool's memory will be free to create the new
        // image
        self.font_staged = None;

        // Create the GPU image with the new dimensions
        self.create_staged_resources(device, dimensions);

        // Update the srv to point at the newly created image
        self.update_srv(device, sampler);

        // Update the metadata for determining when to re-upload the texture
        self.font_version = texture.version;
        self.font_staged_size = dimensions;

        // Map and write the texture data to our staging buffer

        self.font_staging_buffer
            .map()
            .unwrap()
            .as_ptr()
            .copy_from_nonoverlapping(texture.bytes.as_ptr(), texture.bytes.len());
        self.font_staging_buffer.unmap();
    }

    pub unsafe fn record_texture_upload(&mut self, encoder: &mut dyn IGeneralEncoder) {
        encoder.begin_event(Color::GREEN, "Egui Texture Upload");

        let staged_resource = self.font_staged.as_ref().unwrap();

        encoder.resource_barrier(
            &[],
            &[],
            &[TextureBarrier {
                texture: self.font_staged.as_ref().unwrap().deref(),
                subresource_range: TextureSubResourceSet::with_color(),
                before_sync: BarrierSync::ALL,
                after_sync: BarrierSync::COPY,
                before_access: BarrierAccess::NONE,
                after_access: BarrierAccess::COPY_WRITE,
                before_layout: ImageLayout::Undefined,
                after_layout: ImageLayout::CopyDst,
                queue_transition: None,
            }],
        );

        let extent = Extent3D {
            width: self.font_staged_size.0,
            height: self.font_staged_size.1,
            depth: 1,
        };
        encoder.copy_buffer_to_texture(
            self.font_staging_buffer.deref(),
            staged_resource.deref(),
            ImageLayout::CopyDst,
            &[BufferToTextureCopyRegion {
                src: ImageDataLayout {
                    offset: 0,
                    extent: extent.clone(),
                },
                dst: TextureCopyInfo {
                    mip_level: 0,
                    array_layer: 0,
                    aspect: TextureCopyAspect::Color,
                    origin: UOffset3D::default(),
                    extent,
                },
            }],
        );

        encoder.resource_barrier(
            &[],
            &[],
            &[TextureBarrier {
                texture: self.font_staged.as_deref().unwrap(),
                subresource_range: TextureSubResourceSet::with_color(),
                before_sync: BarrierSync::COPY,
                after_sync: BarrierSync::ALL,
                before_access: BarrierAccess::COPY_WRITE,
                after_access: BarrierAccess::SHADER_READ,
                before_layout: ImageLayout::CopyDst,
                after_layout: ImageLayout::ShaderReadOnly,
                queue_transition: None,
            }],
        );

        encoder.end_event();
    }

    /// Allocates the font texture on GPU memory
    fn create_staged_resources(&mut self, device: &dyn IDevice, dimensions: (u32, u32)) {
        let image = device
            .create_texture(&TextureDesc {
                width: dimensions.0,
                height: dimensions.1,
                depth: 1,
                format: Format::R8Unorm,
                dimension: TextureDimension::Texture2D,
                array_size: 1,
                mip_levels: 1,
                sample_count: 1,
                sample_quality: 0,
                usage: ResourceUsageFlags::COPY_DEST | ResourceUsageFlags::SHADER_RESOURCE,
                ..Default::default()
            })
            .unwrap();

        self.font_staged = Some(image);
    }

    unsafe fn update_srv(&mut self, device: &dyn IDevice, sampler: &dyn ISampler) {
        let view = self
            .font_staged
            .as_ref()
            .unwrap()
            .get_view(&ImageViewDesc {
                format: Format::R8Unorm,
                view_type: ImageViewType::Tex2D,
                sub_resources: TextureSubResourceSet::with_color(),
                writable: false,
            })
            .unwrap();

        let set = self.descriptor_set;
        device.update_descriptor_sets(&[
            DescriptorWriteDesc::texture(set, 1, &view.srv_write()),
            DescriptorWriteDesc::sampler(set, 2, &SamplerDescriptorWrite { sampler }),
        ]);
    }

    fn create_font_staging_allocation(
        device: &dyn IDevice,
        dimensions: (u32, u32),
    ) -> AnyArc<dyn IBuffer> {
        device
            .create_buffer(&BufferDesc {
                size: (dimensions.0 * dimensions.1) as u64,
                usage: ResourceUsageFlags::COPY_SOURCE,
                cpu_access: CpuAccessMode::Write,
                ..Default::default()
            })
            .unwrap()
    }
}
