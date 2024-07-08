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

use crate::render::{TextureHandle, TexturePool, TextureStreamingRequest, TextureUploadSource};
use aleph_rhi_api::*;
use crossbeam::queue::{ArrayQueue, SegQueue};
use interfaces::any::AnyArc;
use std::mem;
use thiserror::Error;

pub struct TextureLoader {
    load_queue: ArrayQueue<LoadRequest>,
    immediate_queue: SegQueue<LoadRequest>,
}

impl TextureLoader {
    /// The default queue size, used by [TextureLoader::new].
    pub const DEFAULT_QUEUE_SIZE: usize = 512;

    /// Creates a new [TextureLoader] with a default queue size. The current default is
    /// [TextureLoader::DEFAULT_QUEUE_SIZE].
    pub fn new() -> Self {
        Self::new_with_queue_size(Self::DEFAULT_QUEUE_SIZE)
    }

    /// Creates a new [TextureLoader] with the given queue size.
    pub fn new_with_queue_size(n: usize) -> Self {
        Self {
            load_queue: ArrayQueue::new(n),
            immediate_queue: SegQueue::new(),
        }
    }

    pub fn immediate_upload(&self, handle: TextureHandle, data: TextureUploadSource) {
        let load = LoadRequest {
            target: Some(handle),
            request: None,
            data,
        };

        self.immediate_queue.push(load);
    }

    pub fn enqueue_new_upload(
        &self,
        request: &TextureStreamingRequest,
        data: TextureUploadSource,
    ) -> Result<(), EnqueueError<TextureUploadSource>> {
        request
            .try_take_ownership()
            .ok_or(EnqueueError::RequestAlreadyQueued)?;

        let load = LoadRequest {
            target: None,
            request: Some(request.clone()),
            data,
        };

        self.enqueue_deferred_request(load)
    }

    pub fn enqueue_update_upload(
        &self,
        request: &TextureStreamingRequest,
        target: TextureHandle,
        data: TextureUploadSource,
    ) -> Result<(), EnqueueError<TextureUploadSource>> {
        request
            .try_take_ownership()
            .ok_or(EnqueueError::RequestAlreadyQueued)?;

        let load = LoadRequest {
            target: Some(target),
            request: Some(request.clone()),
            data,
        };

        self.enqueue_deferred_request(load)
    }

    fn enqueue_deferred_request(
        &self,
        load: LoadRequest,
    ) -> Result<(), EnqueueError<TextureUploadSource>> {
        match self.load_queue.push(load) {
            Ok(_) => Ok(()),
            Err(v) => Err(EnqueueError::QueueFull(v.data)),
        }
    }

    pub(crate) unsafe fn upload_requests(
        &self,
        pool: &mut TexturePool,
        device: &dyn IDevice,
        encoder: &mut dyn IGeneralEncoder,
        count: usize,
    ) {
        let mut discard_barriers = Vec::new();
        let mut textures = Vec::new();
        let mut release_barriers = Vec::new();

        // First we process the uploads that _must_ be processed this frame from the immediate
        // queue.
        while let Some(request) = self.immediate_queue.pop() {
            Self::process_request(
                pool,
                device,
                &mut discard_barriers,
                &mut textures,
                &mut release_barriers,
                request,
            );
        }

        // We want to process no more than 'n' requests. Just pass big number (usize::MAX) if you
        // want to flush the queue completely.
        for _ in 0..count {
            // If we've run out of requests we can early exit from the loop
            let request = match self.load_queue.pop() {
                Some(v) => v,
                None => break,
            };

            Self::process_request(
                pool,
                device,
                &mut discard_barriers,
                &mut textures,
                &mut release_barriers,
                request,
            );
        }

        // Prepare all our resources in a single barrier command rather than 'n' barriers
        encoder.resource_barrier(&[], &[], &discard_barriers);

        for upload in textures.drain(..) {
            let texture = AnyArc::from_raw(upload.texture);
            let region = upload
                .load
                .data
                .get_copy_region(0, 0, TextureCopyAspect::Color);
            encoder.copy_buffer_to_texture(
                upload.load.data.buffer.as_ref(),
                texture.as_ref(),
                &[region],
            );

            if let Some(request) = upload.load.request.as_ref() {
                request.mark_complete(upload.load.target.unwrap());
            }

            // TODO: we leak the upload memory here because it needs to have destruction deferred
            //       somehow. work that out later.
            let data = upload.load.data;
            mem::forget(data);
        }

        // Sync our uploaded resources with the access scopes we consider them safe to use
        encoder.resource_barrier(&[], &[], &release_barriers);
    }

    unsafe fn process_request(
        pool: &mut TexturePool,
        device: &dyn IDevice,
        discard_barriers: &mut Vec<TextureBarrier>,
        textures: &mut Vec<Upload>,
        release_barriers: &mut Vec<TextureBarrier>,
        request: LoadRequest,
    ) {
        let (handle, texture) = match Self::create_texture(pool, device, &request) {
            Ok(v) => v,
            Err(err) => {
                match request.request {
                    None => {
                        // TODO: Is there a sane way to _not_ panic here?
                        panic!("Failed to create texture for immediate upload request. Reason: {err:?}");
                    }
                    Some(v) => {
                        // If we fail to create the texture then we mark the request as a failure and
                        // go to the next one in the queue
                        log::error!("Failed to create texture for upload request. Reason: {err:?}");
                        v.mark_failed();
                        return;
                    }
                }
            }
        };

        let subresources = TextureSubResourceSet::all(texture.desc_ref());

        // Need to drop to raw pointers because the borrow checker won't be able to prove what
        // we're doing is safe.
        //
        // Arc's address is stable, but we need to 'move' it into the textures array. The borrow
        // checker will complain the move is impossible due to outstanding borrows.
        //
        // This is safe because:
        // - All access are through shared references
        // - The address is stable
        let texture = AnyArc::into_raw(texture);

        let mut load = request;
        load.target = Some(handle);
        textures.push(Upload { load, texture });

        discard_barriers.push(TextureBarrier {
            texture: texture.as_ref(),
            subresource_range: subresources.clone(),
            before_sync: BarrierSync::NONE,
            after_sync: BarrierSync::COPY,
            before_access: BarrierAccess::NONE,
            after_access: BarrierAccess::COPY_WRITE,
            before_layout: ImageLayout::Undefined,
            after_layout: ImageLayout::CopyDst,
            queue_transition: None,
        });

        release_barriers.push(TextureBarrier {
            texture: texture.as_ref(),
            subresource_range: subresources,
            before_sync: BarrierSync::COPY,
            after_sync: BarrierSync::PIXEL_SHADING
                | BarrierSync::VERTEX_SHADING
                | BarrierSync::COMPUTE_SHADING,
            before_access: BarrierAccess::COPY_WRITE,
            after_access: BarrierAccess::SHADER_READ,
            before_layout: ImageLayout::CopyDst,
            after_layout: ImageLayout::ShaderReadOnly,
            queue_transition: None,
        });
    }

    fn create_texture<'a>(
        pool: &'a mut TexturePool,
        device: &dyn IDevice,
        load: &LoadRequest,
    ) -> Result<(TextureHandle, AnyArc<dyn ITexture>), TextureCreateError> {
        let desc = TextureDesc {
            width: load.data.desc.width,
            height: load.data.desc.height,
            depth: load.data.desc.depth,
            format: load.data.desc.format,
            dimension: TextureDimension::Texture2D, // TODO: need to propogate this
            clear_value: None,
            array_size: 1,
            mip_levels: 1,
            sample_count: 1,
            sample_quality: 0,
            usage: ResourceUsageFlags::SHADER_RESOURCE | ResourceUsageFlags::COPY_DEST,
            name: None,
        };
        let texture = device.create_texture(&desc)?;
        match load.target {
            Some(handle) => {
                // TODO: deferred deletion of old texture
                let _old_texture = pool.update_texture(handle, texture.clone());
                mem::forget(_old_texture);
                Ok((handle, texture))
            }
            None => {
                let handle = load
                    .target
                    .unwrap_or_else(|| pool.create_texture(texture.clone()));
                Ok((handle, texture))
            }
        }
    }
}

#[derive(Error)]
pub enum EnqueueError<T> {
    #[error("The queue is full and the request could not be placed into the queue.")]
    QueueFull(T),

    #[error("The request object is already enqueued.")]
    RequestAlreadyQueued,
}

struct LoadRequest {
    target: Option<TextureHandle>,
    request: Option<TextureStreamingRequest>,
    data: TextureUploadSource,
}

struct Upload {
    load: LoadRequest,
    texture: *const dyn ITexture,
}
