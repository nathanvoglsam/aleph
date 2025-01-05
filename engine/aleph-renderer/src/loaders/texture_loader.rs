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

use aleph_any::AnyArc;
use aleph_rhi_api::*;
use crossbeam::queue::{ArrayQueue, SegQueue};

use crate::pass::resource_processor::{MipGeneratorState, TextureLoadRequest, PREWARM_MIP_FORMATS};
use crate::{
    DeletionPool, EnqueueError, EnqueueErrorKind, StateCache, TextureHandle, TextureObject,
    TexturePool, TextureStreamingRequest, TextureUploadDesc,
};

pub struct TextureLoader {
    device: AnyArc<dyn IDevice>,
    load_queue: ArrayQueue<LoadRequest>,
    immediate_queue: SegQueue<LoadRequest>,
}

impl TextureLoader {
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
        request: Option<TextureStreamingRequest>,
        handle: TextureHandle,
        data: TextureUploadDesc,
        mode: TextureAllocMode,
        mips: GenerateMips,
    ) -> Result<(), EnqueueError<TextureUploadDesc>> {
        let load = LoadRequest {
            target: Some(handle),
            request,
            data,
            final_tex: None,
            mips,
        };
        let load = self.prepare_request(mode, mips, load)?;

        self.immediate_queue.push(load);

        Ok(())
    }

    pub fn enqueue_new_upload(
        &self,
        request: TextureStreamingRequest,
        data: TextureUploadDesc,
        mode: TextureAllocMode,
        mips: GenerateMips,
    ) -> Result<(), EnqueueError<TextureUploadDesc>> {
        let load = LoadRequest {
            target: None,
            request: Some(request),
            data,
            final_tex: None,
            mips,
        };
        let load = self.prepare_request(mode, mips, load)?;

        self.enqueue_deferred_request(load)
    }

    pub fn enqueue_update_upload(
        &self,
        request: TextureStreamingRequest,
        target: TextureHandle,
        data: TextureUploadDesc,
        mode: TextureAllocMode,
        mips: GenerateMips,
    ) -> Result<(), EnqueueError<TextureUploadDesc>> {
        let load = LoadRequest {
            target: Some(target),
            request: Some(request),
            data,
            final_tex: None,
            mips,
        };
        let load = self.prepare_request(mode, mips, load)?;

        self.enqueue_deferred_request(load)
    }
}

impl TextureLoader {
    /// The default queue size, used by [TextureLoader::new].
    pub(crate) const DEFAULT_QUEUE_SIZE: usize = 512;

    /// Creates a new [TextureLoader] with a default queue size. The current default is
    /// [TextureLoader::DEFAULT_QUEUE_SIZE].
    pub(crate) fn new(device: AnyArc<dyn IDevice>) -> Self {
        Self::new_with_queue_size(device, Self::DEFAULT_QUEUE_SIZE)
    }

    /// Creates a new [TextureLoader] with the given queue size.
    pub(crate) fn new_with_queue_size(device: AnyArc<dyn IDevice>, n: usize) -> Self {
        Self {
            device,
            load_queue: ArrayQueue::new(n),
            immediate_queue: SegQueue::new(),
        }
    }

    fn prepare_request(
        &self,
        mode: TextureAllocMode,
        mips: GenerateMips,
        mut load: LoadRequest,
    ) -> Result<LoadRequest, EnqueueError<TextureUploadDesc>> {
        // Pre-allocate the texture if requested and patch the final_tex field with that texture
        match mode {
            TextureAllocMode::Deferred => {}
            TextureAllocMode::Immediate => {
                let final_tex = match Self::allocate_texture(self.device.as_ref(), &load.data, mips)
                {
                    Ok(texture) => Some(texture),
                    Err(e) => {
                        return Err(EnqueueErrorKind::TextureCreateError(e).with_data(load.data))
                    }
                };
                load.final_tex = final_tex;
            }
        }

        Ok(load)
    }

    fn enqueue_deferred_request(
        &self,
        load: LoadRequest,
    ) -> Result<(), EnqueueError<TextureUploadDesc>> {
        match self.load_queue.push(load) {
            Ok(_) => Ok(()),
            Err(v) => Err(EnqueueErrorKind::QueueFull.with_data(v.data)),
        }
    }

    pub(crate) unsafe fn pop_and_bundle_requests(
        &self,
        state_cache: &mut StateCache,
        pool: &mut TexturePool,
        deletion_pool: &mut DeletionPool,
        count: usize,
    ) -> Vec<TextureLoadRequest> {
        let mut out = Vec::new();

        // First we process the uploads that _must_ be processed this frame from the immediate
        // queue.
        while let Some(request) = self.immediate_queue.pop() {
            let v = self.process_request(pool, deletion_pool, request);
            if let Some(v) = v {
                if v.mips == GenerateMips::Yes {
                    // Ensure that the pipeline for the texture's format has been compiled
                    let format = v.texture.desc_ref().format;

                    // Skip even checking the state map if the format is in the prewarm set. This
                    // should save us from hitting the hash map in most cases.
                    if !PREWARM_MIP_FORMATS.contains(&format) {
                        let key = MipGeneratorState::key(format);
                        let _ = state_cache.get_or_insert_with(&key, |cache, k| {
                            MipGeneratorState::new(cache, self.device.as_ref(), k.0)
                        });
                    }
                }

                out.push(v);
            }
        }

        // We want to process no more than 'n' requests. Just pass big number (usize::MAX) if you
        // want to flush the queue completely.
        for _ in 0..count {
            // If we've run out of requests we can early exit from the loop
            let request = match self.load_queue.pop() {
                Some(v) => v,
                None => break,
            };

            let v = self.process_request(pool, deletion_pool, request);
            if let Some(v) = v {
                out.push(v);
            }
        }

        out
    }

    unsafe fn process_request(
        &self,
        pool: &mut TexturePool,
        deletion_pool: &mut DeletionPool,
        mut request: LoadRequest,
    ) -> Option<TextureLoadRequest> {
        let result = Self::create_texture(pool, deletion_pool, self.device.as_ref(), &mut request);
        let (handle, texture) = match result {
            Ok(v) => v,
            Err(err) => {
                // TODO: do we want to push errors into the texture object so you can query the
                //       error with the texture handle too?
                // If we fail to create the texture then we mark the request as a failure and
                // go to the next one in the queue
                log::error!("Failed to create texture for upload request. Reason: {err:?}");
                if let Some(req) = request.request.as_ref() {
                    req.mark_failed(()).unwrap();
                }
                return None;
            }
        };

        deletion_pool.push_buffer(request.data.buffer.buffer().upgrade());

        Some(TextureLoadRequest {
            handle,
            request: request.request,
            data: request.data,
            texture,
            mips: request.mips,
        })
    }

    fn create_texture<'a>(
        pool: &'a mut TexturePool,
        deletion_pool: &mut DeletionPool,
        device: &dyn IDevice,
        load: &mut LoadRequest,
    ) -> Result<(TextureHandle, AnyArc<dyn ITexture>), TextureCreateError> {
        let texture = if let Some(tex) = load.final_tex.take() {
            tex
        } else {
            Self::allocate_texture(device, &load.data, load.mips)?
        };

        match load.target {
            Some(handle) => {
                let old = pool
                    .get_mut(handle)
                    .map(|v| v.update(texture.clone()))
                    .flatten();
                if let Some(old) = old {
                    deletion_pool.push_texture(old);
                }
                Ok((handle, texture))
            }
            None => {
                let handle = load.target.unwrap_or_else(|| {
                    let mut object = TextureObject::new_with(texture.clone());
                    object.recreate_default_view();
                    pool.alloc(object)
                });
                Ok((handle, texture))
            }
        }
    }

    fn allocate_texture(
        device: &dyn IDevice,
        src: &TextureUploadDesc,
        mips: GenerateMips,
    ) -> Result<AnyArc<dyn ITexture>, TextureCreateError> {
        // We require copy dest so we can initialize the resource
        let mut combined_usage = src.desc.usage | ResourceUsageFlags::COPY_DEST;
        match mips {
            GenerateMips::Yes => {
                // We require shader resource and render target usage to be able to generate mip
                // maps into the texture.
                //
                // TODO: in the future we could use a compute based mip generator which would
                //       require unordered access.
                combined_usage |= ResourceUsageFlags::RENDER_TARGET;
                combined_usage |= ResourceUsageFlags::SHADER_RESOURCE;
            }
            GenerateMips::No => {}
        }

        let desc = TextureDesc {
            width: src.desc.width.max(1),
            height: src.desc.height.max(1),
            depth: src.desc.depth.max(1),
            format: src.desc.format,
            dimension: TextureDimension::Texture2D, // TODO: need to propogate this
            clear_value: None,
            array_size: 1,
            mip_levels: src.desc.num_levels.get(),
            sample_count: 1,
            sample_quality: 0,
            usage: combined_usage,
            name: None,
        };
        device.create_texture(&desc)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum TextureAllocMode {
    /// Do not allocate the texture object inside the enqueue call and instead let the loader thread
    /// allocate the texture just-in-time.
    ///
    /// This will push expensive memory allocations onto the render thread, but won't consume memory
    /// until immediately before the texture is uploaded and used in the frame
    Deferred,

    /// Immediately pre-allocates a texture object inside the enqueue call instead of leaving it
    /// to be allocated on the loader thread.
    ///
    /// Allows distributing the expensive texture allocations across the queueing threads at the
    /// cost of potentially more contention on the allocator and a slightly longer memory lifetime
    /// of the allocated textures.
    Immediate,
}

impl Default for TextureAllocMode {
    fn default() -> Self {
        Self::Immediate
    }
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

struct LoadRequest {
    /// Target resource for the upload operation. If this is `None` then we should make a new one.
    target: Option<TextureHandle>,

    /// Target request object to send request notifications through. If `None` then all
    /// notifications will be dropped.
    request: Option<TextureStreamingRequest>,

    /// The actual data source for the upload.
    data: TextureUploadDesc,

    /// Optional pre-allocated destination texture
    final_tex: Option<AnyArc<dyn ITexture>>,

    /// Request the renderer to generate the mipmaps for the texture once loaded
    mips: GenerateMips,
}
