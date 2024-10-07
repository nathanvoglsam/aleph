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

use crate::{
    BufferHandle, BufferPool, BufferStreamingRequest, BufferUploadSource, DeletionPool,
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
        data: BufferUploadSource,
    ) -> Result<(), EnqueueError<BufferUploadSource>> {
        if let Some(req) = request.as_ref() {
            match req.try_take_ownership() {
                Ok(_) => (),
                Err(_) => return Err(EnqueueErrorKind::RequestAlreadyQueued.with_data(data)),
            }
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
        request: BufferStreamingRequest,
        data: BufferUploadSource,
    ) -> Result<(), EnqueueError<BufferUploadSource>> {
        match request.try_take_ownership() {
            Ok(_) => (),
            Err(_) => return Err(EnqueueErrorKind::RequestAlreadyQueued.with_data(data)),
        }

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
        data: BufferUploadSource,
    ) -> Result<(), EnqueueError<BufferUploadSource>> {
        match request.try_take_ownership() {
            Ok(_) => (),
            Err(_) => return Err(EnqueueErrorKind::RequestAlreadyQueued.with_data(data)),
        }

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
    ) -> Result<(), EnqueueError<BufferUploadSource>> {
        match self.load_queue.push(load) {
            Ok(_) => Ok(()),
            Err(v) => Err(EnqueueErrorKind::QueueFull.with_data(v.data)),
        }
    }

    pub(crate) unsafe fn upload_requests(
        &self,
        pool: &mut BufferPool,
        deletion_pool: &mut DeletionPool,
        device: &dyn IDevice,
        encoder: &mut dyn IGeneralEncoder,
        count: usize,
    ) {
        let mut discard_barriers = Vec::new();
        let mut buffers = Vec::new();
        let mut release_barriers = Vec::new();

        // First we process the uploads that _must_ be processed this frame from the immediate
        // queue.
        while let Some(request) = self.immediate_queue.pop() {
            Self::process_request(
                pool,
                deletion_pool,
                device,
                &mut discard_barriers,
                &mut buffers,
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
                &mut buffers,
                &mut release_barriers,
                request,
            );
        }

        // If there are no buffers to upload then we early exit (we don't want to issue empty
        // resource barriers)
        if buffers.is_empty() {
            return;
        }

        // Prepare all our resources in a single barrier command rather than 'n' barriers
        encoder.resource_barrier(&[], &discard_barriers, &[]);

        for upload in buffers.drain(..) {
            let buffer = AnyArc::from_raw(upload.buffer);
            let region = upload.load.data.get_copy_region(0);
            encoder.copy_buffer_regions(upload.load.data.buffer(), buffer.as_ref(), &[region]);

            if let Some(req) = upload.load.request.as_ref() {
                req.mark_complete(upload.load.target.unwrap()).unwrap();
            }

            deletion_pool.push_upload(upload.load.data);
        }

        // Sync our uploaded resources with the access scopes we consider them safe to use
        encoder.resource_barrier(&[], &release_barriers, &[]);
    }

    unsafe fn process_request(
        pool: &mut BufferPool,
        deletion_pool: &mut DeletionPool,
        device: &dyn IDevice,
        discard_barriers: &mut Vec<BufferBarrier>,
        buffers: &mut Vec<Upload>,
        release_barriers: &mut Vec<BufferBarrier>,
        request: LoadRequest,
    ) {
        let result = Self::create_buffer(pool, deletion_pool, device, &request);
        let (handle, buffer) = match result {
            Ok(v) => v,
            Err(err) => {
                // TODO: do we want to push errors into the buffer object so you can query the
                //       error with the buffer handle too?
                // If we fail to create the buffer then we mark the request as a failure and
                // go to the next one in the queue
                log::error!("Failed to create buffer for upload request. Reason: {err:?}");
                if let Some(req) = request.request.as_ref() {
                    req.mark_failed().unwrap();
                }
                return;
            }
        };

        // Need to drop to raw pointers because the borrow checker won't be able to prove what
        // we're doing is safe.
        //
        // Arc's address is stable, but we need to 'move' it into the buffers array. The borrow
        // checker will complain the move is impossible due to outstanding borrows.
        //
        // This is safe because:
        // - All access are through shared references
        // - The address is stable
        let buffer = AnyArc::into_raw(buffer);
        let size = request.data.data_ptr().len();
        let usage = request.data.usage;

        let mut load = request;
        load.target = Some(handle);
        buffers.push(Upload { load, buffer });

        discard_barriers.push(BufferBarrier {
            buffer: buffer.as_ref(),
            offset: 0,
            size: size as u64,
            before_sync: BarrierSync::NONE,
            after_sync: BarrierSync::COPY,
            before_access: BarrierAccess::NONE,
            after_access: BarrierAccess::COPY_WRITE,
            queue_transition: None,
        });

        release_barriers.push(BufferBarrier {
            buffer: buffer.as_ref(),
            offset: 0,
            size: size as u64,
            before_sync: BarrierSync::COPY,
            after_sync: usage.default_barrier_sync(true, Format::R8Unorm),
            before_access: BarrierAccess::COPY_WRITE,
            after_access: usage.barrier_access_for_read(Format::R8Unorm),
            queue_transition: None,
        });
    }

    fn create_buffer<'a>(
        pool: &'a mut BufferPool,
        deletion_pool: &mut DeletionPool,
        device: &dyn IDevice,
        load: &LoadRequest,
    ) -> Result<(BufferHandle, AnyArc<dyn IBuffer>), BufferCreateError> {
        let desc = BufferDesc {
            size: load.data.data_ptr().len() as u64,
            cpu_access: CpuAccessMode::None, // TODO: do we allow something else?
            usage: ResourceUsageFlags::COPY_DEST | load.data.usage,
            name: None,
        };
        let buffer = device.create_buffer(&desc)?;
        match load.target {
            Some(handle) => {
                if let Some(old) = pool.update_buffer(handle, buffer.clone()) {
                    deletion_pool.push_buffer(old);
                }
                Ok((handle, buffer))
            }
            None => {
                let handle = load
                    .target
                    .unwrap_or_else(|| pool.create_buffer(Some(buffer.clone())));
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
    data: BufferUploadSource,
}

struct Upload {
    load: LoadRequest,
    buffer: *const dyn IBuffer,
}
