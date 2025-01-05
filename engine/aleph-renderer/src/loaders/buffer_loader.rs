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

use crate::pass::resource_processor::BufferLoadRequest;
use crate::{
    BufferHandle, BufferObject, BufferPool, BufferStreamingRequest, BufferUploadDesc, DeletionPool,
    EnqueueError, EnqueueErrorKind,
};

pub struct BufferLoader {
    load_queue: ArrayQueue<LoadRequest>,
    immediate_queue: SegQueue<LoadRequest>,
}

impl BufferLoader {
    /// Enqueues a request that is guaranteed to have upload attempted within the frame it was
    /// enqueued in. Any call to [`BufferLoader::immediate_upload`] _must_ be processed and
    /// uploaded when the renderer processes the next batch of upload requests.
    ///
    /// This function is distinct from the [`BufferLoader::enqueue_new_upload`], etc functions in
    /// that it side-steps all upload pacing and budgetting systems. This is expected to be used
    /// very sparingly, and only for buffers which absolutely must be available for some component
    /// to work.
    ///
    /// # Info
    ///
    /// You will note that this still takes a [`BufferStreamingRequest`] handle. The happy path
    /// for this function doesn't actually require one as typically this is a fire-and-forget
    /// function. However there is still a catch that means we need one.
    ///
    /// Uploads are deferred. This means the renderer won't actually _do_ anything until
    /// (potentially much) later in the frame. There's no guarantee that the [`BufferUploadSource`]
    /// data is well-formed and no guarantee that the upload won't _fail_. Without a request handle
    /// there would be no way for the caller to know if their upload failed.
    ///
    /// You're free to ignore the request handle, but you're going to ignore errors.
    pub fn immediate_upload(
        &self,
        request: Option<BufferStreamingRequest>,
        handle: BufferHandle,
        data: BufferUploadDesc,
    ) -> Result<(), EnqueueError<BufferUploadDesc>> {
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
        request: BufferStreamingRequest,
        data: BufferUploadDesc,
    ) -> Result<(), EnqueueError<BufferUploadDesc>> {
        let load = LoadRequest {
            target: None,
            request: Some(request),
            data,
        };

        self.enqueue_deferred_request(load)
    }

    pub fn enqueue_update_upload(
        &self,
        request: BufferStreamingRequest,
        target: BufferHandle,
        data: BufferUploadDesc,
    ) -> Result<(), EnqueueError<BufferUploadDesc>> {
        let load = LoadRequest {
            target: Some(target),
            request: Some(request),
            data,
        };

        self.enqueue_deferred_request(load)
    }
}

impl BufferLoader {
    /// The default queue size, used by [BufferLoader::new].
    pub(crate) const DEFAULT_QUEUE_SIZE: usize = 512;

    /// Creates a new [BufferLoader] with a default queue size. The current default is
    /// [BufferLoader::DEFAULT_QUEUE_SIZE].
    pub(crate) fn new() -> Self {
        Self::new_with_queue_size(Self::DEFAULT_QUEUE_SIZE)
    }

    /// Creates a new [BufferLoader] with the given queue size.
    pub(crate) fn new_with_queue_size(n: usize) -> Self {
        Self {
            load_queue: ArrayQueue::new(n),
            immediate_queue: SegQueue::new(),
        }
    }

    fn enqueue_deferred_request(
        &self,
        load: LoadRequest,
    ) -> Result<(), EnqueueError<BufferUploadDesc>> {
        match self.load_queue.push(load) {
            Ok(_) => Ok(()),
            Err(v) => Err(EnqueueErrorKind::QueueFull.with_data(v.data)),
        }
    }

    pub(crate) unsafe fn pop_and_bundle_requests(
        &self,
        pool: &mut BufferPool,
        deletion_pool: &mut DeletionPool,
        device: &dyn IDevice,
        count: usize,
    ) -> Vec<BufferLoadRequest> {
        // First we process the uploads that _must_ be processed this frame from the immediate
        // queue.
        let mut out = Vec::new();

        while let Some(request) = self.immediate_queue.pop() {
            let v = Self::process_request(pool, deletion_pool, device, request);
            if let Some(v) = v {
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

            let v = Self::process_request(pool, deletion_pool, device, request);
            if let Some(v) = v {
                out.push(v);
            }
        }

        out
    }

    unsafe fn process_request(
        pool: &mut BufferPool,
        deletion_pool: &mut DeletionPool,
        device: &dyn IDevice,
        request: LoadRequest,
    ) -> Option<BufferLoadRequest> {
        let result = Self::create_buffer(pool, deletion_pool, device, &request);
        let (target, buffer) = match result {
            Ok(v) => v,
            Err(err) => {
                // TODO: do we want to push errors into the buffer object so you can query the
                //       error with the buffer handle too?
                // If we fail to create the buffer then we mark the request as a failure and
                // go to the next one in the queue
                log::error!("Failed to create buffer for upload request. Reason: {err:?}");
                if let Some(req) = request.request.as_ref() {
                    req.mark_failed(()).unwrap();
                }
                return None;
            }
        };

        deletion_pool.push_buffer(request.data.buffer.buffer().upgrade());

        Some(BufferLoadRequest {
            target,
            request: request.request,
            data: request.data,
            buffer,
        })
    }

    fn create_buffer<'a>(
        pool: &'a mut BufferPool,
        deletion_pool: &mut DeletionPool,
        device: &dyn IDevice,
        load: &LoadRequest,
    ) -> Result<(BufferHandle, AnyArc<dyn IBuffer>), BufferCreateError> {
        let size = load.data.desc.size.get();
        let usage = load.data.desc.usage;

        let desc = BufferDesc {
            size,
            cpu_access: CpuAccessMode::None, // TODO: do we allow something else?
            usage: ResourceUsageFlags::COPY_DEST | usage,
            name: None,
        };
        let buffer = device.create_buffer(&desc)?;
        match load.target {
            Some(handle) => {
                let old = pool
                    .get_mut(handle)
                    .map(|v| v.update(buffer.clone()))
                    .flatten();
                if let Some(old) = old {
                    deletion_pool.push_buffer(old);
                }
                Ok((handle, buffer))
            }
            None => {
                let handle = load.target.unwrap_or_else(|| {
                    let object = BufferObject::new_with(buffer.clone());
                    pool.alloc(object)
                });
                Ok((handle, buffer))
            }
        }
    }
}

struct LoadRequest {
    /// Target resource for the upload operation. If this is `None` then we should make a new one.
    target: Option<BufferHandle>,

    /// Target request object to send request notifications through. If `None` then all
    /// notifications will be dropped.
    request: Option<BufferStreamingRequest>,

    /// The actual data source for the upload.
    data: BufferUploadDesc,
}
