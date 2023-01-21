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

use crate::renderer::global::FontTexture;
use crate::renderer::GlobalObjects;
use interfaces::any::AnyArc;
use interfaces::gpu::{
    BarrierAccess, BarrierSync, BufferDesc, BufferToTextureCopyRegion, Color, CpuAccessMode,
    DescriptorSetHandle, DescriptorType, DescriptorWriteDesc, DescriptorWrites, Extent3D, Format,
    IBuffer, ICommandPool, IDescriptorPool, IDevice, IGeneralEncoder, ITexture, ImageDataLayout,
    ImageDescriptorWrite, ImageLayout, ImageViewType, TextureAspect, TextureBarrier,
    TextureCopyAspect, TextureCopyInfo, TextureDesc, TextureDimension, TextureSubResourceSet,
    UOffset3D,
};
use std::ops::Deref;

pub struct PerFrameObjects {
    pub vtx_buffer: AnyArc<dyn IBuffer>,
    pub idx_buffer: AnyArc<dyn IBuffer>,

    pub command_allocator: AnyArc<dyn ICommandPool>,

    pub font_version: usize,
    pub font_staging_buffer: AnyArc<dyn IBuffer>,

    pub font_staged: Option<AnyArc<dyn ITexture>>,
    pub font_staged_size: (u32, u32),

    pub descriptor_pool: Box<dyn IDescriptorPool>,
    pub descriptor_set: DescriptorSetHandle,
}

impl PerFrameObjects {
    pub fn new(device: &dyn IDevice, global: &GlobalObjects) -> Self {
        let vtx_buffer = {
            let desc = BufferDesc {
                size: Self::vertex_buffer_size() as _,
                cpu_access: CpuAccessMode::Write,
                is_vertex_buffer: true,
                ..Default::default()
            };
            device.create_buffer(&desc).unwrap()
        };

        let idx_buffer = {
            let desc = BufferDesc {
                size: Self::index_buffer_size() as _,
                cpu_access: CpuAccessMode::Write,
                is_index_buffer: true,
                ..Default::default()
            };
            device.create_buffer(&desc).unwrap()
        };

        let font_staging_buffer = Self::create_font_staging_allocation(device, (4096, 4096));

        let command_allocator = device.create_command_pool().unwrap();
        command_allocator.set_name("egui::CommandAllocator");

        let mut descriptor_pool = device
            .create_descriptor_pool(global.descriptor_set_layout.deref(), 2)
            .unwrap();
        let descriptor_set = descriptor_pool.allocate_set().unwrap();

        Self {
            vtx_buffer,
            idx_buffer,
            command_allocator,
            font_version: 0,
            font_staging_buffer,
            font_staged: None,
            font_staged_size: (0, 0),
            descriptor_pool,
            descriptor_set,
        }
    }

    pub unsafe fn update_texture_data(&mut self, device: &dyn IDevice, texture: &FontTexture) {
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
        self.update_srv(device.deref());

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
                subresource_range: TextureSubResourceSet {
                    aspect: TextureAspect::COLOR,
                    base_mip_level: 0,
                    num_mip_levels: 1,
                    base_array_slice: 0,
                    num_array_slices: 1,
                },
                before_sync: BarrierSync::ALL,
                after_sync: BarrierSync::COPY,
                before_access: BarrierAccess::NONE,
                after_access: BarrierAccess::COPY_WRITE,
                before_layout: ImageLayout::Undefined,
                after_layout: ImageLayout::CopyDst,
                queue_transition_mode: Default::default(),
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
                texture: self.font_staged.as_ref().unwrap().deref(),
                subresource_range: TextureSubResourceSet {
                    aspect: TextureAspect::COLOR,
                    base_mip_level: 0,
                    num_mip_levels: 1,
                    base_array_slice: 0,
                    num_array_slices: 1,
                },
                before_sync: BarrierSync::COPY,
                after_sync: BarrierSync::ALL,
                before_access: BarrierAccess::COPY_WRITE,
                after_access: BarrierAccess::SHADER_SAMPLED_READ,
                before_layout: ImageLayout::CopyDst,
                after_layout: ImageLayout::ShaderReadOnlyOptimal,
                queue_transition_mode: Default::default(),
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
                ..Default::default()
            })
            .unwrap();

        self.font_staged = Some(image);
    }

    unsafe fn update_srv(&mut self, device: &dyn IDevice) {
        device.update_descriptor_sets(&[DescriptorWriteDesc {
            set: self.descriptor_set.clone(),
            binding: 0,
            array_element: 0,
            descriptor_type: DescriptorType::SampledImage,
            writes: DescriptorWrites::Image(&[ImageDescriptorWrite {
                image: self.font_staged.as_ref().unwrap().deref(),
                format: Format::R8Unorm,
                view_type: ImageViewType::Tex2D,
                sub_resources: TextureSubResourceSet {
                    aspect: TextureAspect::COLOR,
                    base_mip_level: 0,
                    num_mip_levels: 1,
                    base_array_slice: 0,
                    num_array_slices: 1,
                },
                writable: false,
            }]),
        }]);
    }

    fn create_font_staging_allocation(
        device: &dyn IDevice,
        dimensions: (u32, u32),
    ) -> AnyArc<dyn IBuffer> {
        device
            .create_buffer(&BufferDesc {
                size: (dimensions.0 * dimensions.1) as u64,
                cpu_access: CpuAccessMode::Write,
                ..Default::default()
            })
            .unwrap()
    }

    pub fn vertex_buffer_size() -> usize {
        1024 * 1024 * 4
    }

    pub fn index_buffer_size() -> usize {
        1024 * 1024 * 2
    }
}
