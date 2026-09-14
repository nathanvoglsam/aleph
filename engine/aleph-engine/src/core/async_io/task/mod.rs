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
use std::io;
use std::pin::Pin;

use aleph_vfs::async_io::AsyncIoSender;
use mg::async_resource_loader::AsyncResourceLoader;
use smallbox::SmallBox;

use crate::core::async_io::context::IoContext;
use crate::render::async_loader::resources::async_loader_requests::ResourceLoadHandle;

/// Interface of our tasks futures (once boxed, hence the `dyn`).
pub type TaskFuture<'a> = dyn Future<Output = io::Result<()>> + 'a;

/// Alias of [`SmallBox`] that covers the requirements of a task payload.
pub type TaskPayload = SmallBox<dyn Any + Send + 'static, [u128; 6]>;

/// Factory object shared with the loader that will spawn the task future on the async loading
/// thread.
///
/// This type is not the future itself, it spawns the future. The expectation is that a task
/// factory is constructed once, at engine boot time, and is then shared with the async workers via
/// an `Arc`. The factory is then invoked via [`ITaskFactory::spawn_new`] with a per-task payload.
///
/// This is a dyn-safe layer on top of [`TaskFactory`]. You should not need to implement this
/// interface directly. A blanket impl is provided for all types that impl [`TaskFactory`] that
/// correctly handles the necessary boilerplate plumbing.
pub trait ITaskFactory: Send + Sync + 'static {
    /// Constructs a new boxed future that encapsulates the logic of the task that will be executed
    /// on the async loader thread.
    ///
    /// See [`TaskFactory::task`] for more info.
    fn spawn_new<'a>(
        &self,
        ctx: IoContext<'a, AsyncIoSender>,
        loader: &'a AsyncResourceLoader<ResourceLoadHandle>,
        msg: TaskPayload,
    ) -> Pin<Box<TaskFuture<'a>>>;
}

/// Factory object shared with the loader that will spawn the task future on the async loading
/// thread.
///
/// This type is not the future itself, it spawns the future. The expectation is that a task
/// factory is constructed once, at engine boot time, and is then shared with the async workers via
/// an `Arc`. The factory is then invoked via [`TaskFactory::task`] with a per-task payload.
///
/// This is a non-dyn-safe layer below [`ITaskFactory`]. A blanket impl is provided that implements
/// the dyn-safe interface in terms of [`TaskFactory`]. You shouldn't need to implement
/// [`ITaskFactory`] directly.
pub trait TaskFactory: Send + Sync + 'static {
    /// Context type that each spawned future will receive an instance of. Generally this will clone
    /// handles stored in the factory to be given to the spawned future.
    type Context: Send + Sync + 'static;

    /// Payload type that each spawned future will receive an instance of. Each spawned future will
    /// be given an instance constructed by the caller who queued the task. This is the primary
    /// channel to pass task specific arguments.
    type Payload: Any;

    /// Constructs a new [`TaskFactory::Context`] instance. It is expected this will be passed to
    /// [`TaskFactory::task`].
    fn context(&self) -> Self::Context;

    /// The async future that encapsulates the logic of the task that will be executed on the async
    /// loader thread.
    ///
    /// The async loader executor takes responsibility of polling these to completion.
    ///
    /// # Async
    ///
    /// The async executor we use is very basic, and is only designed to work with our custom async
    /// IO primitives. Our executor is completion based, and will not interact with wakers at all.
    /// Any future other than those spawned via [`IoContext`] will not work as the executor does
    /// not use the 'waker' to know when to poll the future again.
    ///
    /// ## Why?
    ///
    /// On some operating systems the best interface available is completion based. Completion based
    /// async IO does not match very will with the [`Future`] trait without caveats and careful
    /// integration with the executor. Performance is key, and our interface has been designed to
    /// minimize the number of copies from disk -> GPU.
    ///
    /// We choose to trade flexibility for the ability to use certain platforms completion based
    /// async io primitives. This enables issuing async reads _directly_ into memory mapped from the
    /// RHI with no intermediate copies within the engine.
    fn task<'a>(
        ctx: Self::Context,
        io: IoContext<'a, AsyncIoSender>,
        loader: &'a AsyncResourceLoader<ResourceLoadHandle>,
        msg: Self::Payload,
    ) -> impl Future<Output = io::Result<()>> + 'a;
}

impl<T: TaskFactory> ITaskFactory for T {
    fn spawn_new<'a>(
        &self,
        io: IoContext<'a, AsyncIoSender>,
        loader: &'a AsyncResourceLoader<ResourceLoadHandle>,
        msg: TaskPayload,
    ) -> Pin<Box<TaskFuture<'a>>> {
        let ctx = self.context();
        let future = async move {
            let msg = match TaskPayload::downcast::<T::Payload>(msg) {
                Ok(v) => v,
                Err(_) => {
                    log::error!("Tried to spawn task with incorrect payload type!");
                    return Err(io::Error::from(io::ErrorKind::InvalidInput));
                }
            };
            Self::task(ctx, io, loader, msg.into_inner()).await
        };
        Box::pin(future)
    }
}
