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

use std::any::Any;
use std::cell::Cell;
use std::num::NonZero;
use std::pin::Pin;
use std::process::abort;
use std::rc::Rc;
use std::sync::Arc;
use std::task::{Context, Poll, Waker};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

use aleph_gen_arena::{GenArena, RawHandle};
use aleph_vfs::file::AsyncReadResponse;
use crossbeam::channel::{Receiver, RecvError, Sender, unbounded};
use crossbeam::select;
use mg::async_resource_loader::loader_notify::LoaderNotify;
use mg::async_resource_loader::{AsyncResourceLoader, FlushError};

use crate::core::alloc::EngineSystem;
use crate::core::async_io::context::IoContext;
use crate::render::async_loader::internal::task::{
    ITaskFactory, TaskError, TaskFuture, TaskPayload, TaskResult,
};
use crate::render::async_loader::resources::async_loader_requests::ResourceLoadHandle;

pub struct WorkerTask {
    factory: Arc<dyn ITaskFactory>,
    message: TaskPayload,
}

impl WorkerTask {
    pub fn new<T: Any + Send + 'static>(factory: Arc<dyn ITaskFactory>, message: T) -> Self {
        Self {
            factory,
            message: smallbox::smallbox!(message),
        }
    }
}

pub struct AsyncLoaderWorker {
    request_recv: Receiver<WorkerTask>,
    loader: AsyncResourceLoader<ResourceLoadHandle>,
}

impl AsyncLoaderWorker {
    pub fn spawn_with(
        renderer: &mut mg::renderer::Renderer,
    ) -> Option<(
        JoinHandle<()>,
        Sender<WorkerTask>,
        LoaderNotify<ResourceLoadHandle>,
    )> {
        let (loader, loader_notify) = renderer.create_async_resource_loader(Default::default())?;
        let (loader_thread, loader_sender) = Self::spawn(loader);
        Some((loader_thread, loader_sender, loader_notify))
    }

    pub fn spawn(
        loader: AsyncResourceLoader<ResourceLoadHandle>,
    ) -> (JoinHandle<()>, Sender<WorkerTask>) {
        let (request_send, request_recv) = unbounded();

        let mut this = Self {
            request_recv,
            loader,
        };

        let handle = thread::Builder::new()
            .name("async-worker".into())
            .spawn(move || {
                this.run();
            })
            .expect("Failed to spawn AsyncLoaderWorker thread");

        (handle, request_send)
    }

    pub fn run(&mut self) {
        let request_recv = &self.request_recv;
        let (response_send, response_recv) = unbounded();
        let response_slot = Rc::new(Cell::new(None));

        {
            let tasks = GenArena::new_in();
            Self::run_inner(
                tasks,
                &request_recv,
                &response_recv,
                &response_send,
                response_slot.clone(),
                &self.loader,
            )
        }
    }

    fn run_inner<'a>(
        mut tasks: Tasks<'a>,
        request_recv: &'a Receiver<WorkerTask>,
        response_recv: &'a Receiver<AsyncIoResponse>,
        response_send: &'a Sender<AsyncIoResponse>,
        response_slot: Rc<Cell<Option<AsyncIoResponse>>>,
        loader: &'a AsyncResourceLoader<ResourceLoadHandle>,
    ) {
        let mut should_close = false;
        'main: loop {
            if should_close && tasks.is_empty() {
                break 'main;
            }

            let result: Poll<TaskResult<()>> = select! {
                recv(request_recv) -> msg => Self::worker_message(
                    &mut tasks,
                    response_send,
                    &response_slot,
                    loader,
                    msg,
                ),
                recv(response_recv) -> msg => Self::async_message(
                    &mut tasks,
                    &response_slot,
                    msg,
                ),
                default(Duration::from_millis(8)) => 'timeout: {
                    // If we've gone to sleep for an extended time it's likely that we've retired
                    // all our work. It's possible that there are submitted uploads that aren't
                    // flushed, and unless we do this here manually nothing will flush them until
                    // we get more requests.
                    //
                    // So we wake up after an extended (for a game) wait and flush manually before
                    // going into a deeper sleep.
                    match loader.flush_submitted_uploads() {
                        Ok(_) => {},
                        Err(FlushError::DeviceLost) => {
                            log::error!("GPU device lost.");
                            break 'timeout Poll::Ready(Err(TaskError::DeviceLost))
                        }
                        Err(FlushError::RendererDisconnected) => {
                            log::error!("Target renderer has been destroyed.");
                            break 'timeout Poll::Ready(Err(TaskError::RendererDisconnected))
                        }
                        Err(e @ FlushError::CommandRecordingFailure) => {
                            log::error!("Fatal: {e:?}");
                            break 'timeout Poll::Ready(Err(TaskError::CommandRecordingFailure))
                        }
                        Err(e @ FlushError::WaitFailure) => {
                            log::error!("Fatal: {e:?}");
                            break 'timeout Poll::Ready(Err(TaskError::FatalAbort))
                        }
                    };

                    select! {
                        recv(request_recv) -> msg => Self::worker_message(
                            &mut tasks,
                            response_send,
                            &response_slot,
                            loader,
                            msg,
                        ),
                        recv(response_recv) -> msg => Self::async_message(
                            &mut tasks,
                            &response_slot,
                            msg,
                        ),
                    }
                },
            };

            // Task still waiting, go back to sleep waiting for responses
            let result = match result {
                Poll::Ready(v) => v,
                Poll::Pending => continue 'main,
            };

            // We completed, but was there an error? We may need to handle it. No errors? Back to
            // sleep!
            let error = match result {
                Err(v) => v,
                Ok(_) => continue 'main,
            };

            match error {
                // This class of error is (slightly) less catastrophic than 'FatalAbort'. In this
                // case we might be able to still unwind the stack and shutdown cleanly.
                TaskError::RendererDisconnected
                | TaskError::CommandRecordingFailure
                | TaskError::DeviceLost => {
                    'inner: for (_, mut task) in tasks.drain() {
                        response_slot.set(None);
                        let result = abort_unwind(|| {
                            task.as_mut().poll(&mut Context::from_waker(Waker::noop()))
                        });
                        let result = match result {
                            Poll::Ready(v) => v,
                            Poll::Pending => {
                                // If any futures are still outstanding we must abort.
                                // Because we use completion based futures that lend memory
                                // from the future onto the async io system it is not
                                // possible to drop futures if they are still pending.
                                log::error!("Fatal exit promoted to abort");
                                abort()
                            }
                        };
                        let error = match result {
                            Err(e) => e,
                            Ok(_) => continue 'inner,
                        };
                        match error {
                            TaskError::FatalAbort => {
                                log::error!("Fatal exit promoted to abort");
                                abort()
                            }
                            TaskError::Io(_)
                            | TaskError::NotEnoughMemory
                            | TaskError::ResourceCreationFailed
                            | TaskError::DeviceLost
                            | TaskError::RendererDisconnected
                            | TaskError::SenderDisconnected
                            | TaskError::CommandRecordingFailure
                            | TaskError::Other => {}
                        }
                    }
                    break 'main;
                }
                // There are some failure conditions that can only be safely handled by
                // aborting. Basically anything where the async lifetimes get hairy or
                // impossible to resolve cleanly.
                //
                // Some classes of GPU errors will cause this. Depending on where we
                // fail it might also be caused by IO.
                TaskError::FatalAbort => {
                    log::error!("A catastrophic fatal error occurred. Aborting...");
                    abort();
                }
                // These error classes are simply failures of an individual task and do not
                // constitute failure of the entire loader. We can safely continue from
                // these.
                TaskError::Io(_)
                | TaskError::NotEnoughMemory
                | TaskError::ResourceCreationFailed
                | TaskError::Other => continue 'main,
                TaskError::SenderDisconnected => {
                    should_close = true;
                    continue 'main;
                }
            }
        }
    }

    fn worker_message<'a>(
        tasks: &mut Tasks<'a>,
        response_send: &Sender<AsyncIoResponse>,
        response_slot: &Rc<Cell<Option<AsyncIoResponse>>>,
        loader: &'a AsyncResourceLoader<ResourceLoadHandle>,
        msg: Result<WorkerTask, RecvError>,
    ) -> Poll<TaskResult<()>> {
        let msg = match msg {
            Ok(v) => v,
            Err(_) => {
                log::error!("AsyncLoaderWorker message sender disconnected.");
                return Poll::Ready(Err(TaskError::SenderDisconnected));
            }
        };

        let task = tasks.alloc_cyclic(move |handle| {
            let ctx = IoContext {
                handle,
                sender: response_send.clone(),
                response_slot: response_slot.clone(),
            };
            msg.factory.spawn_new(ctx, loader, msg.message)
        });

        Self::poll_task(tasks, task)
    }

    fn async_message<'a>(
        tasks: &mut Tasks<'a>,
        response_slot: &Cell<Option<AsyncIoResponse>>,
        msg: Result<AsyncIoResponse, RecvError>,
    ) -> Poll<TaskResult<()>> {
        let msg = match msg {
            Ok(v) => v,
            Err(_) => {
                log::error!("Async IO queue disconnected.");
                return Poll::Ready(Err(TaskError::Other));
            }
        };

        let task = msg.opaque();
        let task = match NonZero::new(task) {
            None => return Poll::Ready(Err(TaskError::Other)),
            Some(v) => v,
        };
        let task = RawHandle::from_int(task);

        response_slot.set(Some(msg));

        Self::poll_task(tasks, task)
    }

    fn poll_task(tasks: &mut Tasks, handle: RawHandle) -> Poll<TaskResult<()>> {
        let task = match tasks.get_mut(handle) {
            None => {
                log::error!("Tried to poll a task with an invalid or out of date handle.");
                return Poll::Ready(Err(TaskError::Other));
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
        let result = abort_unwind(|| task.as_mut().poll(&mut Context::from_waker(Waker::noop())));

        match &result {
            Poll::Ready(_) => {
                tasks.free(handle);
            }
            Poll::Pending => {}
        }
        result
    }
}

type RootBoxedFuture<'a> = Pin<Box<TaskFuture<'a>>>;
type Tasks<'a> = GenArena<RootBoxedFuture<'a>, RawHandle, EngineSystem>;

extern "C" fn abort_unwind<F: FnOnce() -> R, R>(f: F) -> R {
    f()
}
