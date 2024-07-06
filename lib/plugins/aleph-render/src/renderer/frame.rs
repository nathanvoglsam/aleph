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

use aleph_rhi_api::*;
use interfaces::any::AnyArc;

use crate::render::{TextureUploadDesc, TextureUploadSource};
use crate::renderer::FontTexture;

pub struct PerFrameObjects {
    pub acquire_semaphore: AnyArc<dyn ISemaphore>,
    pub present_semaphore: AnyArc<dyn ISemaphore>,

    pub uniform_buffer: AnyArc<dyn IBuffer>,

    pub font_version: usize,
    pub font: Option<AnyArc<dyn ITexture>>,
    pub font_view: Option<ImageView>,
    pub font_size: (u32, u32),

    pub done_fence: AnyArc<dyn IFence>,

    pub deferred_buffers: Vec<AnyArc<dyn IBuffer>>,
}

impl PerFrameObjects {
    pub fn new(device: &dyn IDevice) -> Self {
        let uniform_buffer = device
            .create_buffer(&BufferDesc {
                size: 1024,
                cpu_access: CpuAccessMode::Write,
                usage: ResourceUsageFlags::CONSTANT_BUFFER,
                name: Some("egui::ConstantBuffer"),
            })
            .unwrap();

        Self {
            acquire_semaphore: device.create_semaphore().unwrap(),
            present_semaphore: device.create_semaphore().unwrap(),
            uniform_buffer,
            font_version: 0,
            font: None,
            font_view: None,
            font_size: (0, 0),
            done_fence: device.create_fence(true).unwrap(),
            deferred_buffers: Vec::with_capacity(4),
        }
    }

    pub unsafe fn record_texture_upload(
        &mut self,
        encoder: &mut dyn IGeneralEncoder,
        device: &dyn IDevice,
        texture: &FontTexture,
    ) {
        encoder.begin_event(Color::GREEN, "Egui Texture Upload");

        // Check the data is correct
        assert_eq!(texture.bytes.len(), texture.width * texture.height);

        // Crunch our dimensions for d3d12
        let dimensions = (texture.width as u32, texture.height as u32);

        let staging_buffer = TextureUploadSource::new_owned(
            device,
            TextureUploadDesc::new(dimensions.0, dimensions.1, 1, Format::R8Unorm),
        )
        .unwrap();

        assert_eq!(
            staging_buffer.desc.aligned_width(),
            staging_buffer.desc.width,
            "Currently we don't handle row pitch here"
        );

        // Create the GPU image with the new dimensions
        let font = Self::create_font_texture(device, dimensions);
        let view = font
            .get_view(&ImageViewDesc {
                format: Format::R8Unorm,
                view_type: ImageViewType::Tex2D,
                sub_resources: TextureSubResourceSet::with_color(),
                writable: false,
            })
            .unwrap();

        encoder.resource_barrier(
            &[],
            &[],
            &[TextureBarrier {
                texture: Some(font.as_ref()),
                subresource_range: TextureSubResourceSet::with_color(),
                before_sync: BarrierSync::NONE,
                after_sync: BarrierSync::COPY,
                before_access: BarrierAccess::NONE,
                after_access: BarrierAccess::COPY_WRITE,
                before_layout: ImageLayout::Undefined,
                after_layout: ImageLayout::CopyDst,
                queue_transition: None,
            }],
        );

        encoder.copy_buffer_to_texture(
            staging_buffer.buffer.as_ref(),
            font.as_ref(),
            &[staging_buffer.get_copy_region(0, 0, TextureCopyAspect::Color)],
        );

        encoder.resource_barrier(
            &[],
            &[],
            &[TextureBarrier {
                texture: Some(font.as_ref()),
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

        // Update the metadata for determining when to re-upload the texture
        self.font = Some(font);
        self.font_view = Some(view);
        self.font_version = texture.version;
        self.font_size = dimensions;

        staging_buffer
            .data
            .cast::<u8>()
            .as_ptr()
            .copy_from_nonoverlapping(texture.bytes.as_ptr(), texture.bytes.len());

        self.deferred_buffers.clear();
        staging_buffer.buffer.unmap();
        self.deferred_buffers.push(staging_buffer.buffer);

        encoder.end_event();
    }

    /// Allocates the font texture on GPU memory
    fn create_font_texture(device: &dyn IDevice, dimensions: (u32, u32)) -> AnyArc<dyn ITexture> {
        device
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
            .unwrap()
    }
}
