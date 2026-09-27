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

use std::pin::Pin;
use std::task::{Context, Poll, Waker};
use std::thread::JoinHandle;
use std::time::Duration;
use std::{io, thread};

use aleph_alloc::instrumentation::IAllocationCategory;
use aleph_gen_arena::{GenArena, RawHandle};
use aleph_object_system::unsafe_impl_iobject;
use crossbeam::channel::{Receiver, RecvError, SendError, Sender, unbounded};
use crossbeam::select;
use mg::async_resource_loader::{AsyncResourceLoader, FlushError};

use crate::core::alloc::{Engine, EngineSystem};
use crate::core::async_io::TaskFuture;
use crate::core::async_io::context::IoContext;
use crate::core::async_io::internal::{FutureSpawner, TaskWaker};

/// Sender half of a channel used to communicate and send requests to a [`AsyncLoaderWorker`].
///
/// Tasks can be invoked on the worker thread by enquing them via [`AsyncLoaderQueue::spawn`].
#[derive(Clone)]
#[repr(transparent)]
pub struct AsyncLoaderQueue {
    sender: Sender<WorkerMessage>,
}

unsafe_impl_iobject!(AsyncLoaderQueue, "01a09968-96a7-7521-84fa-6824a6e21498");

impl AsyncLoaderQueue {
    /// Sends an `async fn` to the worker thread and invokes it to spawn a future into the executor
    /// on the worker thread.
    pub fn spawn<T>(&self, spawner: T) -> Result<(), SendError<()>>
    where
        for<'a> T: (AsyncFnOnce(IoContext<'a>) -> io::Result<()>) + Send + 'static,
    {
        let spawner = FutureSpawner::new(spawner);
        match self.sender.send(WorkerMessage::Spawn(spawner)) {
            Ok(()) => Ok(()),
            Err(_) => Err(SendError(())),
        }
    }

    /// Request the [`AsyncLoaderWorker`] to exit.
    ///
    /// The worker will stop accepting new requests and will attempt to complete all currently
    /// active tasks before closing. It is expected that once the worker has completed its thread
    /// will close.
    pub fn quit(self) {
        let _ = self.sender.send(WorkerMessage::Quit);
    }
}

pub struct AsyncLoaderWorker {
    request_send: AsyncLoaderQueue,
    request_recv: Receiver<WorkerMessage>,
    loader: AsyncResourceLoader<u64>,
}

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
    ) -> Option<(JoinHandle<()>, AsyncLoaderQueue)> {
        let loader = renderer.create_async_resource_loader(Default::default())?;
        let (loader_thread, loader_sender) = Self::spawn(loader);
        Some((loader_thread, loader_sender))
    }

    /// Constructs a new [`AsyncLoaderWorker`] and spawns a thread that will run the worker event
    /// loop until it is requested to close.
    ///
    /// Returns a handle to the thread, and a [`AsyncLoaderQueue`] that can be shared or invoked to
    /// enqueue work onto the worker thread.
    pub fn spawn(loader: AsyncResourceLoader<u64>) -> (JoinHandle<()>, AsyncLoaderQueue) {
        let (request_send, request_recv) = unbounded();

        let queue = AsyncLoaderQueue {
            sender: request_send,
        };

        let this = Self {
            request_send: queue.clone(),
            request_recv,
            loader,
        };

        let handle = thread::Builder::new()
            .name("async-worker".into())
            .spawn(move || Engine::with(|| this.run()))
            .expect("Failed to spawn AsyncLoaderWorker thread");

        (handle, queue)
    }

    fn run(self) {
        let request_send = &self.request_send;
        let request_recv = &self.request_recv;
        let (response_send, response_recv) = unbounded();

        {
            let tasks = GenArena::new_in();
            Self::run_inner(
                tasks,
                &request_send,
                &request_recv,
                &response_recv,
                &response_send,
                &self.loader,
            )
        }
    }

    fn run_inner<'a>(
        mut tasks: Tasks<'a>,
        request_send: &'a AsyncLoaderQueue,
        request_recv: &'a Receiver<WorkerMessage>,
        response_recv: &'a Receiver<RawHandle>,
        response_send: &'a Sender<RawHandle>,
        loader: &'a AsyncResourceLoader<u64>,
    ) {
        'main: loop {
            // Initial blocking wait with a timeout. We use a short timeout for the first wait, and
            // if we don't get any requests within that timeout we flush the resource loader before
            // moving into a deep sleep.
            select! {
                recv(request_recv) -> msg => {
                    let msg = msg.unwrap_or_else(|_| {
                        abort_unwind(|| unreachable!())
                    });

                    match msg {
                        WorkerMessage::Spawn(msg) => {
                            let v = Self::worker_message(
                                &mut tasks,
                                request_send,
                                response_send,
                                loader,
                                msg,
                            );
                            if let Poll::Ready(v) = v {
                                let _ = v.inspect_err(|error| {
                                    log::error!("Async IO task failed with error: '{error:?}'")
                                });

                            }
                            continue 'main;
                        }
                        WorkerMessage::Quit => {
                            break 'main;
                        }
                    }
                },
                recv(response_recv) -> msg => {
                    let v = Self::async_message(
                        &mut tasks,
                        msg,
                    );

                    if let Poll::Ready(v) = v {
                        let _ = v.inspect_err(|error| {
                            log::error!("Async IO task failed with error: '{error:?}'")
                        });
                    }

                    continue 'main;
                },
                default(Duration::from_millis(8)) => {},
            }

            // If we've gone to sleep for an extended time it's likely that we've retired
            // all our work. It's possible that there are submitted uploads that aren't
            // flushed, and unless we do this here manually nothing will flush them until
            // we get more requests.
            //
            // So we wake up after an extended (for a game) wait and flush manually before
            // going into a deeper sleep.
            match loader.flush_submitted_uploads() {
                Ok(_) => {}
                Err(e @ FlushError::DeviceLost) => {
                    log::error!("Error: {e:?}");
                    continue 'main;
                }
                Err(e @ FlushError::RendererDisconnected) => {
                    log::error!("Error: {e:?}");
                    continue 'main;
                }
                Err(e @ FlushError::CommandRecordingFailure) => {
                    log::error!("Error: {e:?}");
                    continue 'main;
                }
                Err(e @ FlushError::WaitFailure) => {
                    log::error!("Error: {e:?}");
                    abort_unwind(|| panic!("Error: {e:?}"))
                }
            };

            select! {
                recv(request_recv) -> msg => {
                    let msg = msg.unwrap_or_else(|_| {
                        abort_unwind(|| unreachable!())
                    });

                    match msg {
                        WorkerMessage::Spawn(msg) => {
                            let v = Self::worker_message(
                                &mut tasks,
                                request_send,
                                response_send,
                                loader,
                                msg,
                            );
                            if let Poll::Ready(v) = v {
                                let _ = v.inspect_err(|error| {
                                    log::error!("Async IO task failed with error: '{error:?}'")
                                });
                            }
                            continue 'main;
                        }
                        WorkerMessage::Quit => {
                            break 'main;
                        }
                    }
                },
                recv(response_recv) -> msg => {
                    let v = Self::async_message(
                        &mut tasks,
                        msg,
                    );

                    if let Poll::Ready(v) = v {
                        let _ = v.inspect_err(|error| {
                            log::error!("Async IO task failed with error: '{error:?}'")
                        });
                    }

                    continue 'main;
                },
            }
        }

        Self::run_inner_quit(&mut tasks, response_recv, loader);
    }

    fn run_inner_quit<'a>(
        tasks: &mut Tasks<'a>,
        response_recv: &'a Receiver<RawHandle>,
        loader: &'a AsyncResourceLoader<u64>,
    ) {
        'main: loop {
            // This is an alternate event loop that only blocks for waker messages. This path is
            // intended as part of a clean shutdown of the io worker. This will attempt to complete
            // all outstanding tasks before exiting. New tasks will not be spawned.
            if tasks.is_empty() {
                break 'main;
            }

            // Initial blocking wait with a timeout. We use a short timeout for the first wait, and
            // if we don't get any requests within that timeout we flush the resource loader before
            // moving into a deep sleep.
            select! {
                recv(response_recv) -> msg => {
                    let v = Self::async_message(
                        tasks,
                        msg,
                    );

                    if let Poll::Ready(v) = v {
                        let _ = v.inspect_err(|error| {
                            log::error!("Async IO task failed with error: '{error:?}'")
                        });
                    }

                    continue 'main;
                },
                default(Duration::from_millis(8)) => {},
            }

            // If we've gone to sleep for an extended time it's likely that we've retired
            // all our work. It's possible that there are submitted uploads that aren't
            // flushed, and unless we do this here manually nothing will flush them until
            // we get more requests.
            //
            // So we wake up after an extended (for a game) wait and flush manually before
            // going into a deeper sleep.
            match loader.flush_submitted_uploads() {
                Ok(_) => {}
                Err(e @ FlushError::DeviceLost) => {
                    log::error!("Error: {e:?}");
                    continue 'main;
                }
                Err(e @ FlushError::RendererDisconnected) => {
                    log::error!("Error: {e:?}");
                    continue 'main;
                }
                Err(e @ FlushError::CommandRecordingFailure) => {
                    log::error!("Error: {e:?}");
                    continue 'main;
                }
                Err(e @ FlushError::WaitFailure) => {
                    log::error!("Error: {e:?}");
                    abort_unwind(|| panic!("Error: {e:?}"))
                }
            };

            select! {
                recv(response_recv) -> msg => {
                    let v = Self::async_message(
                        tasks,
                        msg,
                    );

                    if let Poll::Ready(v) = v {
                        let _ = v.inspect_err(|error| {
                            log::error!("Async IO task failed with error: '{error:?}'")
                        });
                    }

                    continue 'main;
                },
            }
        }
    }

    fn worker_message<'a>(
        tasks: &mut Tasks<'a>,
        request_send: &'a AsyncLoaderQueue,
        response_send: &'a Sender<RawHandle>,
        loader: &'a AsyncResourceLoader<u64>,
        msg: FutureSpawner,
    ) -> Poll<io::Result<()>> {
        let task = tasks.alloc_cyclic(move |handle| {
            let io = IoContext {
                this: request_send,
                loader,
            };
            Task {
                task: msg.spawn(io),
                waker: TaskWaker::new_waker(handle, response_send.clone()),
            }
        });

        Self::poll_task(tasks, task)
    }

    fn async_message(tasks: &mut Tasks, msg: Result<RawHandle, RecvError>) -> Poll<io::Result<()>> {
        let task = match msg {
            Ok(v) => v,
            Err(_) => {
                log::error!("Async IO queue disconnected.");
                return Poll::Ready(Err(io::Error::from(io::ErrorKind::Other)));
            }
        };

        Self::poll_task(tasks, task)
    }

    fn poll_task(tasks: &mut Tasks, handle: RawHandle) -> Poll<io::Result<()>> {
        let task = match tasks.get_mut(handle) {
            None => {
                log::error!("Tried to poll a task with an invalid or out of date handle.");
                return Poll::Ready(Err(io::Error::from(io::ErrorKind::Other)));
            }
            Some(v) => v,
        };

        // Safety: there's really no way we are able to recover from a panic in one of our futures.
        //
        // It's critical we don't unwind because we will end up dropping other futures while they
        // may have outstanding io. We can't easily make everything unwind safe either to use
        // catch_unwind.
        //
        // In practice a shipping build will use panic abort anyway so nothing changes here.
        let result = abort_unwind(|| {
            let waker = &task.waker;
            let mut cx = Context::from_waker(waker);
            task.task.as_mut().poll(&mut cx)
        });

        match &result {
            Poll::Ready(_) => {
                tasks.free(handle);
            }
            Poll::Pending => {}
        }
        result
    }
}

enum WorkerMessage {
    Spawn(FutureSpawner),
    Quit,
}

struct Task<'a> {
    task: RootBoxedFuture<'a>,
    waker: Waker,
}

type RootBoxedFuture<'a> = Pin<Box<TaskFuture<'a>>>;
type Tasks<'a> = GenArena<Task<'a>, RawHandle, EngineSystem>;

extern "C" fn abort_unwind<F: FnOnce() -> R, R>(f: F) -> R {
    f()
}
