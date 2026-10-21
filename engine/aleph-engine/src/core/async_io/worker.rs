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

use std::rc::Rc;
use std::thread::JoinHandle;
use std::time::Duration;
use std::{io, thread};

use aleph_alloc::instrumentation::IAllocationCategory;
use aleph_object_system::unsafe_impl_iobject;
use crossbeam::channel::SendError;
use mg::async_resource_loader::{AsyncResourceLoader, FlushError, IFenceWaiter, RetireError};
use tokio::runtime::{Builder, LocalOptions};
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender, WeakUnboundedSender};
use tokio::task::LocalSet;

use crate::core::alloc::Engine;
use crate::core::async_io::context::IoContext;
use crate::core::async_io::internal::FutureSpawner;
use crate::core::async_io::internal::futures::{AbortUnwindFuture, ProfiledFuture};
use crate::render::async_fence::{AsyncFenceWaiter, AsyncFenceWaiterQueue};

pub(crate) enum WorkerMessage {
    Spawn(FutureSpawner),
}

/// Sender half of a channel used to communicate and send requests to a [`AsyncLoaderWorker`].
///
/// Tasks can be invoked on the worker thread by enqueuing them via [`AsyncLoaderQueue::spawn`].
#[derive(Clone)]
#[repr(transparent)]
pub struct AsyncLoaderQueue {
    sender: UnboundedSender<WorkerMessage>,
}

unsafe_impl_iobject!(AsyncLoaderQueue, "01a09968-96a7-7521-84fa-6824a6e21498");

impl AsyncLoaderQueue {
    pub(crate) fn from_weak(sender: &WeakUnboundedSender<WorkerMessage>) -> Option<Self> {
        sender.upgrade().map(|sender| Self { sender })
    }

    /// Sends an `async fn` to the worker thread and invokes it to spawn a future into the executor
    /// on the worker thread.
    pub fn spawn<T>(&self, spawner: T) -> Result<(), SendError<()>>
    where
        T: (AsyncFnOnce(&IoContext) -> io::Result<()>) + Send + 'static,
    {
        let spawner = FutureSpawner::new(spawner);
        match self.sender.send(WorkerMessage::Spawn(spawner)) {
            Ok(()) => Ok(()),
            Err(_) => Err(SendError(())),
        }
    }
}

/// Handle to the thread running an [`AsyncLoaderWorker`].
///
/// This handle can be used to join the worker thread and wait for its completion.
///
/// To stop the worker thread, drop all references to the associated [`AsyncLoaderQueue`].
pub struct AsyncLoaderWorkerHandle(JoinHandle<()>);

impl AsyncLoaderWorkerHandle {
    /// Wrapper over [`JoinHandle::join`] that waits for the worker thread to complete.
    pub fn join(self) -> thread::Result<()> {
        self.0.join()
    }
}

/// Worker responsible for handling asynchronous resource loading tasks.
pub struct AsyncLoaderWorker {}

impl AsyncLoaderWorker {
    /// Constructs a new [`AsyncLoaderWorker`] and spawns a thread that will run the worker event
    /// loop until it is requested to close.
    ///
    /// Returns a handle to the thread, and a [`AsyncLoaderQueue`] that can be shared or invoked to
    /// enqueue work onto the worker thread.
    ///
    /// This is a wrapper over [`AsyncLoaderWorker::spawn`] that constructs a new
    /// [`AsyncResourceLoader`] from the given renderer.
    pub fn spawn_with(
        renderer: &mut mg::renderer::Renderer,
    ) -> Option<(AsyncLoaderWorkerHandle, AsyncLoaderQueue)> {
        let loader = renderer.create_async_resource_loader(Default::default())?;
        let (loader_thread, loader_sender) = Self::spawn(loader);
        Some((loader_thread, loader_sender))
    }

    /// Constructs a new [`AsyncLoaderWorker`] and spawns a thread that will run the worker event
    /// loop until it is requested to close.
    ///
    /// Returns a handle to the thread, and a [`AsyncLoaderQueue`] that can be shared or invoked to
    /// enqueue work onto the worker thread.
    pub fn spawn(loader: AsyncResourceLoader<u64>) -> (AsyncLoaderWorkerHandle, AsyncLoaderQueue) {
        Self::spawn_with_sleep_range(loader, 2, 1024)
    }

    /// Same as [`AsyncLoaderWorker::spawn`] but with a configurable idle flush interval, in
    /// milliseconds.
    ///
    /// The dispatcher sleeps for `min_sleep_ms` initially and after receiving a message. Each time
    /// the sleep times out the interval doubles, clamped to `max_sleep_ms`.
    ///
    /// # Panics
    ///
    /// Panics if `min_sleep_ms` is greater than `max_sleep_ms`.
    pub fn spawn_with_sleep_range(
        loader: AsyncResourceLoader<u64>,
        min_sleep_ms: u64,
        max_sleep_ms: u64,
    ) -> (AsyncLoaderWorkerHandle, AsyncLoaderQueue) {
        assert!(
            min_sleep_ms <= max_sleep_ms,
            "minimum sleep interval must not exceed maximum sleep interval"
        );

        let (ext_send, ext_recv) = mpsc::unbounded_channel();
        let queue = AsyncLoaderQueue { sender: ext_send };
        let queue_sender = queue.sender.downgrade();

        let handle = thread::Builder::new()
            .name("async-worker".into())
            .spawn(move || {
                aleph_profile::register_thread!("async-worker");
                let _result = Engine::with(|| {
                    Self::worker_main(loader, ext_recv, queue_sender, min_sleep_ms, max_sleep_ms)
                });
            })
            .expect("Failed to spawn AsyncLoaderWorker thread");

        (AsyncLoaderWorkerHandle(handle), queue)
    }

    /// The main worker loop that runs on the spawned thread.
    ///
    /// Returns `Some(())` if the worker completed successfully, or `None` if an error occurred.
    fn worker_main(
        loader: AsyncResourceLoader<u64>,
        ext_recv: UnboundedReceiver<WorkerMessage>,
        queue_sender: WeakUnboundedSender<WorkerMessage>,
        min_sleep_ms: u64,
        max_sleep_ms: u64,
    ) -> Option<()> {
        let (waiter_handle, async_fence_waiter) = AsyncFenceWaiter::spawn(loader.get_device());

        let loader = Rc::new(loader);

        Self::run_tokio_runtime(
            &loader,
            ext_recv,
            queue_sender,
            async_fence_waiter,
            min_sleep_ms,
            max_sleep_ms,
        )?;

        // We must ensure the runtime was dropped before the loader. All outstanding
        // handles to the loader should have been dropped now. We enforce this to
        // check for tasks that are still running.
        let _loader = Rc::try_unwrap(loader)
            .ok()
            .expect("Failed to unwrap loader");

        waiter_handle
            .join()
            .expect("Failed to join AsyncFenceWaiter thread");

        Some(())
    }

    /// Creates the Tokio runtime and runs the async loader dispatcher within it.
    ///
    /// Returns `Some(())` if the runtime was successfully created and executed, or `None` if an
    /// error occurred during runtime creation.
    fn run_tokio_runtime(
        loader: &Rc<AsyncResourceLoader<u64>>,
        ext_recv: UnboundedReceiver<WorkerMessage>,
        queue_sender: WeakUnboundedSender<WorkerMessage>,
        fence_waiter_queue: AsyncFenceWaiterQueue,
        min_sleep_ms: u64,
        max_sleep_ms: u64,
    ) -> Option<()> {
        let runtime = Builder::new_current_thread()
            .enable_all()
            .build_local(LocalOptions::default())
            .inspect_err(|e| log::error!("Failed to create tokio runtime: {e:?}"))
            .ok()?;

        abort_unwind(|| {
            let local = LocalSet::new();

            // Drives the dispatch loop until it finished. GPU IO tasks are always
            // spawned onto the local set...
            let dispatcher = AsyncLoaderDispatcher {
                ext_recv,
                queue_sender,
                loader: loader.clone(),
                min_sleep_ms,
                max_sleep_ms,
            };
            let dispatcher = dispatcher.run(&local, fence_waiter_queue);
            let dispatcher = AbortUnwindFuture::new(dispatcher);
            let dispatcher = ProfiledFuture::new(dispatcher, "async-worker-dispatcher");
            runtime.block_on(async { local.run_until(dispatcher).await });

            // Add a helper task to flush the loader in a loop, so that we can block
            // on the local set to finish all GPU IO tasks. We can't cancel them
            // because of our completion based file IO.
            //
            // _this_ task is safe to drop. It will be cancelled when the runtime
            // is dropped. It _must_ be dropped to release the loader handle...
            let flusher = Self::shutdown_flush_loop(loader.clone());
            let flusher = AbortUnwindFuture::new(flusher);
            let _ = runtime.spawn_local(flusher);

            // ...so when the dispatch loop is finished, we can now block on the
            // local set to finish all GPU IO tasks. We can't cancel them because
            // of our completion based file IO.
            //
            // We do have to dispatch an extra task to periodically flush the
            // loader, because the dispatch loop is no longer running to do that
            // for us.
            runtime.block_on(local);
        });
        Some(())
    }

    async fn shutdown_flush_loop(loader: Rc<AsyncResourceLoader<u64>>) {
        loop {
            aleph_profile::scope_named!("AsyncLoaderWorker::pre_sleep_flush");
            flush_loader_checked(&loader);
            tokio::time::sleep(Duration::from_millis(500)).await;
        }
    }
}

/// The dispatcher responsible for handling external messages and managing the async loader.
struct AsyncLoaderDispatcher {
    ext_recv: UnboundedReceiver<WorkerMessage>,
    queue_sender: WeakUnboundedSender<WorkerMessage>,
    loader: Rc<AsyncResourceLoader<u64>>,
    min_sleep_ms: u64,
    max_sleep_ms: u64,
}

impl AsyncLoaderDispatcher {
    async fn run(mut self, local_set: &LocalSet, fence_waiter_queue: AsyncFenceWaiterQueue) {
        aleph_profile::scope_named!("AsyncLoaderWorker::run");

        let mut sleep_ms = self.min_sleep_ms;
        loop {
            tokio::select! {
                msg = self.ext_recv.recv() => {
                    if let Some(msg) = msg {
                        sleep_ms = self.min_sleep_ms;
                        self.worker_message(local_set, msg);
                    } else {
                        break;
                    }
                },
                _ = tokio::time::sleep(Duration::from_millis(sleep_ms)) => {
                    sleep_ms = sleep_ms.saturating_mul(2).min(self.max_sleep_ms);
                    flush_loader_checked(&self.loader);
                    wait_all_loader_checked_async(&self.loader, &fence_waiter_queue).await;
                },
            };
        }
    }

    fn worker_message(&self, local_set: &LocalSet, msg: WorkerMessage) {
        aleph_profile::scope_named!("AsyncLoaderWorker::worker_message");
        match msg {
            WorkerMessage::Spawn(msg) => {
                let io = IoContext {
                    this: self.queue_sender.clone(),
                    loader: self.loader.clone(),
                };

                let task = async move {
                    if let Err(e) = msg.spawn(&io).await {
                        log::error!("Async IO task failed with error: '{e:?}'");
                    }
                };
                let task = AbortUnwindFuture::new(task);
                let task = ProfiledFuture::new(task, "gpu-io-task-poll");
                let _ = local_set.spawn_local(task);
            }
        }
    }
}

fn flush_loader_checked(loader: &AsyncResourceLoader<u64>) {
    match loader.flush_submitted_uploads() {
        Ok(_) => {}
        Err(e @ FlushError::DeviceLost)
        | Err(e @ FlushError::RendererDisconnected)
        | Err(e @ FlushError::CommandRecordingFailure) => {
            log::error!("'pre_sleep_flush' flush error: {e:?}");
        }
        Err(e @ FlushError::WaitFailure) => {
            log::error!("'pre_sleep_flush' flush error: {e:?}");
            panic!("'pre_sleep_flush' flush error: {e:?}")
        }
    }
}

async fn wait_all_loader_checked_async(
    loader: &AsyncResourceLoader<u64>,
    fence_waiter_queue: &AsyncFenceWaiterQueue,
) {
    let waiter = NonBlockingFenceWaiter { fence_waiter_queue };
    match loader.wait_all_submissions_async(waiter).await {
        Ok(_) => {}
        Err(e @ RetireError::DeviceLost) | Err(e @ RetireError::RendererDisconnected) => {
            log::error!("'pre_sleep_flush' wait error: {e:?}");
        }
        Err(e @ RetireError::WaitFailure) => {
            log::error!("'pre_sleep_flush' wait error: {e:?}");
            panic!("'pre_sleep_flush' wait error: {e:?}")
        }
    }
}

struct NonBlockingFenceWaiter<'a> {
    fence_waiter_queue: &'a AsyncFenceWaiterQueue,
}

unsafe impl<'a> IFenceWaiter for NonBlockingFenceWaiter<'a> {
    fn wait_fence(
        self,
        fence: rhi::FenceHandle,
        value: u64,
    ) -> impl std::future::Future<Output = Result<rhi::FenceWaitResult, rhi::FenceWaitError>> {
        let (sender, receiver) = tokio::sync::oneshot::channel();
        self.fence_waiter_queue
            .wait_for(fence, value, sender)
            .unwrap();
        async move {
            match receiver.await {
                Ok(result) => result.map(|_| rhi::FenceWaitResult::Complete),
                Err(_) => Err(rhi::FenceWaitError::Platform),
            }
        }
    }
}

extern "C" fn abort_unwind<F: FnOnce() -> R, R>(f: F) -> R {
    f()
}
