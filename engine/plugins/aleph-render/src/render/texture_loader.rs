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

use std::fmt::Formatter;

use aleph_rhi_api::*;
use crossbeam::queue::{ArrayQueue, SegQueue};
use interfaces::any::AnyArc;
use thiserror::Error;

use crate::render::{
    LoaderDeletionPool, TextureHandle, TexturePool, TextureStreamingRequest, TextureUploadSource,
};

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

    /// Enqueues a request that is guaranteed to have upload attempted within the frame it was
    /// enqueued in. Any call to [`TextureLoader::immediate_upload`] _must_ be processed and
    /// uploaded when the renderer processes the next batch of upload requests.
    ///
    /// This function is distinct from the [`TextureLoader::enqueue_new_upload`], etc functions in
    /// that it side-steps all upload pacing and budgetting systems. This is expected to be used
    /// very sparingly, and only for textures which absolutely must be available for some component
    /// to work.
    ///
    /// # Info
    ///
    /// You will note that this still takes a [`TextureStreamingRequest`] handle. The happy path
    /// for this function doesn't actually require one as typically this is a fire-and-forget
    /// function. However there is still a catch that means we need one.
    ///
    /// Uploads are deferred. This means the renderer won't actually _do_ anything until
    /// (potentially much) later in the frame. There's no guarantee that the [`TextureUploadSource`]
    /// data is well-formed and no guarantee that the upload won't _fail_. Without a request handle
    /// there would be no way for the caller to know if their upload failed.
    ///
    /// You're free to ignore the request handle, but you're going to ignore errors.
    pub fn immediate_upload(
        &self,
        request: TextureStreamingRequest,
        handle: TextureHandle,
        data: TextureUploadSource,
    ) -> Result<(), EnqueueError<TextureUploadSource>> {
        match request.try_take_ownership() {
            Some(_) => (),
            None => return Err(EnqueueErrorKind::RequestAlreadyQueued.with_data(data)),
        }

        let load = LoadRequest {
            target: Some(handle),
            request,
            data,
        };

        self.immediate_queue.push(load);

        Ok(())
    }

    pub fn enqueue_new_upload(
        &self,
        request: TextureStreamingRequest,
        data: TextureUploadSource,
    ) -> Result<(), EnqueueError<TextureUploadSource>> {
        match request.try_take_ownership() {
            Some(_) => (),
            None => return Err(EnqueueErrorKind::RequestAlreadyQueued.with_data(data)),
        }

        let load = LoadRequest {
            target: None,
            request,
            data,
        };

        self.enqueue_deferred_request(load)
    }

    pub fn enqueue_update_upload(
        &self,
        request: TextureStreamingRequest,
        target: TextureHandle,
        data: TextureUploadSource,
    ) -> Result<(), EnqueueError<TextureUploadSource>> {
        match request.try_take_ownership() {
            Some(_) => (),
            None => return Err(EnqueueErrorKind::RequestAlreadyQueued.with_data(data)),
        }

        let load = LoadRequest {
            target: Some(target),
            request,
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
            Err(v) => Err(EnqueueErrorKind::QueueFull.with_data(v.data)),
        }
    }

    pub(crate) unsafe fn upload_requests(
        &self,
        pool: &mut TexturePool,
        deletion_pool: &mut LoaderDeletionPool,
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
                deletion_pool,
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
                deletion_pool,
                device,
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

        for upload in textures.drain(..) {
            let texture = AnyArc::from_raw(upload.texture);
            let region = upload
                .load
                .data
                .get_copy_region(0, 0, TextureCopyAspect::Color);
            encoder.copy_buffer_to_texture(upload.load.data.buffer(), texture.as_ref(), &[region]);

            upload
                .load
                .request
                .mark_complete(upload.load.target.unwrap());

            deletion_pool.push_upload(upload.load.data);
        }

        // Sync our uploaded resources with the access scopes we consider them safe to use
        encoder.resource_barrier(&[], &[], &release_barriers);
    }

    unsafe fn process_request(
        pool: &mut TexturePool,
        deletion_pool: &mut LoaderDeletionPool,
        device: &dyn IDevice,
        discard_barriers: &mut Vec<TextureBarrier>,
        textures: &mut Vec<Upload>,
        release_barriers: &mut Vec<TextureBarrier>,
        request: LoadRequest,
    ) {
        let result = Self::create_texture(pool, deletion_pool, device, &request);
        let (handle, texture) = match result {
            Ok(v) => v,
            Err(err) => {
                // TODO: do we want to push errors into the texture object so you can query the
                //       error with the texture handle too?
                // If we fail to create the texture then we mark the request as a failure and
                // go to the next one in the queue
                log::error!("Failed to create texture for upload request. Reason: {err:?}");
                request.request.mark_failed();
                return;
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
        deletion_pool: &mut LoaderDeletionPool,
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
                if let Some(old) = pool.update_texture(handle, texture.clone()) {
                    deletion_pool.push_texture(old);
                }
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

pub struct EnqueueError<T> {
    /// A slot for returning owned data that was passed into an enqueue function that should not
    /// be dropped inside the enqueue function. This is used to return ownership back to the caller
    /// in the event of the owner.
    ///
    /// If you don't care just grab the 'err' field and the caller will drop 'data' itself.
    pub data: T,

    /// The actual error this struct encapsulates
    pub err: EnqueueErrorKind,
}

impl<T> EnqueueError<T> {
    pub const fn new(err: EnqueueErrorKind, data: T) -> Self {
        Self { data, err }
    }
}

impl<T> std::error::Error for EnqueueError<T> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.err.source()
    }

    fn description(&self) -> &str {
        #[allow(deprecated)]
        self.err.description()
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        #[allow(deprecated)]
        self.err.cause()
    }
}

impl<T> std::fmt::Debug for EnqueueError<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.err, f)
    }
}

impl<T> std::fmt::Display for EnqueueError<T> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.err, f)
    }
}

#[derive(Error, Debug)]
pub enum EnqueueErrorKind {
    #[error("The queue is full and the request could not be placed into the queue.")]
    QueueFull,

    #[error("The request object is already enqueued.")]
    RequestAlreadyQueued,
}

impl EnqueueErrorKind {
    pub const fn with_data<T>(self, data: T) -> EnqueueError<T> {
        EnqueueError::new(self, data)
    }
}

struct LoadRequest {
    target: Option<TextureHandle>,
    request: TextureStreamingRequest,
    data: TextureUploadSource,
}

struct Upload {
    load: LoadRequest,
    texture: *const dyn ITexture,
}
