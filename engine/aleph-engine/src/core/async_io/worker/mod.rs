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
use std::task::{Context, Poll, Waker};
use std::thread::JoinHandle;
use std::time::Duration;
use std::{io, thread};

use aleph_alloc::instrumentation::IAllocationCategory;
use aleph_gen_arena::{GenArena, RawHandle};
use aleph_object_system::unsafe_impl_iobject;
use aleph_vfs::async_io::{AsyncIoMessage, AsyncIoSender};
use crossbeam::channel::{Receiver, RecvError, SendError, Sender, unbounded};
use crossbeam::select;
use mg::async_resource_loader::loader_notify::LoaderNotify;
use mg::async_resource_loader::{AsyncResourceLoader, FlushError};
use smallbox::{SmallBox, smallbox};

use crate::core::alloc::{Engine, EngineSystem};
use crate::core::async_io::context::IoContext;
use crate::core::async_io::task::TaskFuture;
use crate::render::async_loader::resources::async_loader_requests::ResourceLoadHandle;

#[derive(Clone)]
#[repr(transparent)]
pub struct AsyncLoaderQueue {
    sender: Sender<FutureSpawner>,
}

unsafe_impl_iobject!(AsyncLoaderQueue, "01a09968-96a7-7521-84fa-6824a6e21498");

impl AsyncLoaderQueue {
    pub fn spawn<T>(&self, spawner: T) -> Result<(), SendError<()>>
    where
        for<'a> T: (AsyncFnOnce(
                IoContext<'a, AsyncIoSender>,
                &'a AsyncResourceLoader<ResourceLoadHandle>,
            ) -> io::Result<()>)
            + Any
            + Send
            + 'static,
    {
        let spawner = FutureSpawner {
            spawner: smallbox!(spawner),
            unwrapper: unwrapper::<_, T>,
        };
        match self.sender.send(spawner) {
            Ok(()) => Ok(()),
            Err(_) => Err(SendError(())),
        }
    }
}

pub struct AsyncLoaderWorker {
    request_recv: Receiver<FutureSpawner>,
    loader: AsyncResourceLoader<ResourceLoadHandle>,
}

impl AsyncLoaderWorker {
    pub fn spawn_with(
        renderer: &mut mg::renderer::Renderer,
    ) -> Option<(
        JoinHandle<()>,
        AsyncLoaderQueue,
        LoaderNotify<ResourceLoadHandle>,
    )> {
        let (loader, loader_notify) = renderer.create_async_resource_loader(Default::default())?;
        let (loader_thread, loader_sender) = Self::spawn(loader);
        Some((loader_thread, loader_sender, loader_notify))
    }

    pub fn spawn(
        loader: AsyncResourceLoader<ResourceLoadHandle>,
    ) -> (JoinHandle<()>, AsyncLoaderQueue) {
        let (request_send, request_recv) = unbounded();

        let mut this = Self {
            request_recv,
            loader,
        };

        let handle = thread::Builder::new()
            .name("async-worker".into())
            .spawn(move || Engine::with(|| this.run()))
            .expect("Failed to spawn AsyncLoaderWorker thread");

        let request_send = AsyncLoaderQueue {
            sender: request_send,
        };

        (handle, request_send)
    }

    pub fn run(&mut self) {
        let request_recv = &self.request_recv;
        let (response_send, response_recv) = unbounded();
        let response_slot = Cell::new(None);

        {
            let tasks = GenArena::new_in();
            Self::run_inner(
                tasks,
                &request_recv,
                &response_recv,
                &response_send,
                &response_slot,
                &self.loader,
            )
        }
    }

    fn run_inner<'a>(
        mut tasks: Tasks<'a>,
        request_recv: &'a Receiver<FutureSpawner>,
        response_recv: &'a Receiver<AsyncIoMessage>,
        response_send: &'a Sender<AsyncIoMessage>,
        response_slot: &'a Cell<Option<AsyncIoMessage>>,
        loader: &'a AsyncResourceLoader<ResourceLoadHandle>,
    ) {
        let mut should_close = false;
        'main: loop {
            if should_close && tasks.is_empty() {
                break 'main;
            }

            let result: io::Result<()> = select! {
                recv(request_recv) -> msg => {
                    let result = Self::worker_message(
                        &mut should_close,
                        &mut tasks,
                        response_send,
                        &response_slot,
                        loader,
                        msg,
                    );
                    match result {
                        Poll::Ready(v) => v,
                        Poll::Pending => continue 'main,
                    }
                },
                recv(response_recv) -> msg => {
                    let result = Self::async_message(
                        &mut tasks,
                        &response_slot,
                        msg,
                    );
                    match result {
                        Poll::Ready(v) => v,
                        Poll::Pending => continue 'main,
                    }
                },
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
                        Err(e @ FlushError::DeviceLost) => {
                            log::error!("Error: {e:?}");
                            break 'timeout Err(io::Error::from(io::ErrorKind::Other));
                        }
                        Err(e @ FlushError::RendererDisconnected) => {
                            log::error!("Error: {e:?}");
                            break 'timeout Err(io::Error::from(io::ErrorKind::ConnectionAborted));
                        }
                        Err(e @ FlushError::CommandRecordingFailure) => {
                            log::error!("Error: {e:?}");
                            break 'timeout Err(io::Error::from(io::ErrorKind::Other));
                        }
                        Err(e @ FlushError::WaitFailure) => {
                            log::error!("Error: {e:?}");
                            abort_unwind(|| panic!("Error: {e:?}"))
                        }
                    };

                    let result = select! {
                        recv(request_recv) -> msg => Self::worker_message(
                            &mut should_close,
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
                    };
                    match result {
                        Poll::Ready(v) => v,
                        Poll::Pending => continue 'main,
                    }
                },
            };

            // We completed, but was there an error? We may need to handle it. No errors? Back to
            // sleep!
            let error = match result {
                Err(v) => v,
                Ok(_) => continue 'main,
            };

            log::error!("Async IO task failed with error: '{error:?}'");
        }
    }

    fn worker_message<'a>(
        should_close: &mut bool,
        tasks: &mut Tasks<'a>,
        response_send: &Sender<AsyncIoMessage>,
        response_slot: &'a Cell<Option<AsyncIoMessage>>,
        loader: &'a AsyncResourceLoader<ResourceLoadHandle>,
        msg: Result<FutureSpawner, RecvError>,
    ) -> Poll<io::Result<()>> {
        let msg = match msg {
            Ok(v) => v,
            Err(_) => {
                log::error!("AsyncLoaderWorker message sender disconnected.");
                *should_close = true;
                return Poll::Ready(Ok(()));
            }
        };

        let task = tasks.alloc_cyclic(move |handle| {
            let io = IoContext {
                handle,
                sender: AsyncIoSender(response_send.clone()),
                response_slot,
            };
            (msg.unwrapper)(io, loader, msg.spawner)
        });

        Self::poll_task(tasks, task)
    }

    fn async_message<'a>(
        tasks: &mut Tasks<'a>,
        response_slot: &'a Cell<Option<AsyncIoMessage>>,
        msg: Result<AsyncIoMessage, RecvError>,
    ) -> Poll<io::Result<()>> {
        let msg = match msg {
            Ok(v) => v,
            Err(_) => {
                log::error!("Async IO queue disconnected.");
                return Poll::Ready(Err(io::Error::from(io::ErrorKind::Other)));
            }
        };

        let task = msg.opaque();
        let task = match NonZero::new(task) {
            None => return Poll::Ready(Err(io::Error::from(io::ErrorKind::Other))),
            Some(v) => v,
        };
        let task = RawHandle::from_int(task);

        response_slot.set(Some(msg));
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

/// Contains the object we send across to the async executor thread that invokes the async fn to
/// be executed on the async thread.
struct FutureSpawner {
    spawner: SmallBox<dyn Any + Send + 'static, [u128; 8]>,
    unwrapper: UnwrapperFn<[u128; 8]>,
}

type UnwrapperFn<Space> = for<'a> fn(
    IoContext<'a, AsyncIoSender>,
    &'a AsyncResourceLoader<ResourceLoadHandle>,
    SmallBox<dyn Any + Send + 'static, Space>,
) -> Pin<Box<TaskFuture<'a>>>;

fn unwrapper<'aa, Space, T>(
    ctx: IoContext<'aa, AsyncIoSender>,
    loader: &'aa AsyncResourceLoader<ResourceLoadHandle>,
    f: SmallBox<dyn Any + Send + 'static, Space>,
) -> Pin<Box<TaskFuture<'aa>>>
where
    for<'a> T: (AsyncFnOnce(
            IoContext<'a, AsyncIoSender>,
            &'a AsyncResourceLoader<ResourceLoadHandle>,
        ) -> io::Result<()>)
        + Any
        + Send
        + 'static,
{
    let f: SmallBox<T, Space> = SmallBox::<dyn Any + Send + 'static, Space>::downcast(f).unwrap();
    Box::pin(f.into_inner()(ctx, loader))
}

type RootBoxedFuture<'a> = Pin<Box<TaskFuture<'a>>>;
type Tasks<'a> = GenArena<RootBoxedFuture<'a>, RawHandle, EngineSystem>;

extern "C" fn abort_unwind<F: FnOnce() -> R, R>(f: F) -> R {
    f()
}
