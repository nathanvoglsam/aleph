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

use smallbox::SmallBox;
use smallbox::space::S8;

use crate::internal::buffer::{BufferObject, BufferObjectStore, make_standard_buffer_desc};
use crate::internal::renderer::immediate_upload_queue::{
    ImmediateBufferUpload, ImmediateTextureUpload, ImmediateUploadQueue,
};
use crate::internal::texture::{
    TextureObject, TextureObjectStore, validate_mip_data_for_immediate_upload,
};
use crate::renderer::{BufferOptions, SimpleTextureOptions};
use crate::resource::buffer::BufferHandle;
use crate::resource::texture::TextureHandle;
use crate::resource::texture::simple::SimpleTextureDesc;
use crate::resource_loader::mip_upload::MipUploadDesc;
use crate::resource_loader::upload_buffer::IUploadBuffer;

/// Accessor over the core renderer that implements APIs for immediate resource construction
///
/// # Immediate Resource Construction
///
/// Using the functions on this accessor will enqueue resource to be created immediately in the
/// next call to 'draw_frame'. The upload is still deferred, but the rhi resource is created
/// immediately and a valid handle is returned to the caller immediately. The renderer makes a
/// promise that the upload data provided at create time will be uploaded and visible for any
/// consumers of the resource, even within the frame it was constructed in.
///
/// These APIs are intended to be used where fully async loads are either impossible, unnecessary or
/// too complex. Use cases like small LUTs and other static resources that are create-once-use-many
/// are ideal.
///
/// The downside to this API is that no attempt at pacing resource uploads is made. A caller could
/// theoretically ask for 10GB of data for immediate upload. This will, obviously, stall the frame
/// for a very long time. All the uploads must complete before any other work in the frame can
/// start.
pub struct ImmediateResourceBuilder<'a> {
    pub device: &'a dyn rhi::IDevice,
    pub(crate) texture_object_store: &'a mut TextureObjectStore,
    pub(crate) buffer_object_store: &'a mut BufferObjectStore,
    pub(crate) immediate_upload_queue: &'a mut ImmediateUploadQueue,
}

impl<'a> ImmediateResourceBuilder<'a> {
    /// Creates a new simple texture immediately, taking the given data.
    ///
    /// This will immediately return a texture handle. The new texture will be added to the
    /// immediate upload queue and the data will be available for use in the current frame.
    ///
    /// By default, 'data' must provide data for the entire texture, including the entire mip chain,
    /// in the given buffer. If the caller has enabled 'generate_mips' in [`SimpleTextureOptions`]
    /// then only mip level 0 needs to be provided. The remaining levels will be generated on the
    /// GPU by downsampling from level 0.
    pub fn create_simple_texture_immediate<T: SimpleTextureDesc>(
        &mut self,
        desc: &T,
        data: MipUploadDesc,
        _options: &SimpleTextureOptions,
    ) -> Result<TextureHandle, ()> {
        validate_mip_data_for_immediate_upload(desc, &data).ok_or(())?;

        // Standard usage valid for access as an SRV + uploads. We need RT for mip generation.
        let usage: rhi::ResourceUsageFlags = rhi::ResourceUsageFlags::COPY_DEST
            | rhi::ResourceUsageFlags::SHADER_RESOURCE
            | rhi::ResourceUsageFlags::RENDER_TARGET;
        let rhi_desc = rhi::TextureDesc {
            width: desc.storage_width(),
            height: desc.storage_height(),
            depth: desc.storage_depth(),
            format: desc.format(),
            dimension: desc.texture_dimension().ok_or(())?,
            clear_value: None,
            array_size: 1,
            mip_levels: desc.num_levels().get(),
            sample_count: 1,
            sample_quality: 0,
            usage,
            name: None,
        };
        let object = self.device.create_texture(&rhi_desc).map_err(|_| ())?;

        let mut object = TextureObject {
            object: Some(object),
            default_view: None,
            subresource_all: rhi::TextureSubResourceSet::all(&rhi_desc),
            format: rhi_desc.format,
        };
        object.recreate_default_view(self.device);

        let handle = self.texture_object_store.pool.alloc(object);

        self.immediate_upload_queue
            .textures
            .push(ImmediateTextureUpload {
                desc: desc.as_simple_layout(),
                target: handle,
                mips: data,
            });

        Ok(handle)
    }

    /// Creates a new buffer immediately, taking the given data.
    ///
    /// This will immediately return a handle. The new object will be added to the immediate upload
    /// queue and the data will be available for use in the current frame.
    pub fn create_buffer_immediate(
        &mut self,
        size: u64,
        data: Option<SmallBox<dyn IUploadBuffer, S8>>,
        _options: &BufferOptions,
    ) -> Result<BufferHandle, ()> {
        // If data is provided it must at least be enough to fully initialize the buffer.
        if let Some(data) = &data {
            if (data.bytes().len() as u64) < size {
                return Err(());
            }
        }

        let rhi_desc = make_standard_buffer_desc(size);
        let object = self.device.create_buffer(&rhi_desc).map_err(|_| ())?;

        let handle = self.buffer_object_store.pool.alloc(BufferObject {
            object: Some(object),
        });

        if let Some(data) = data {
            self.immediate_upload_queue
                .buffers
                .push(ImmediateBufferUpload {
                    target: handle,
                    data,
                });
        }

        Ok(handle)
    }
}
