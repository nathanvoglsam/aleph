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

use mg::async_resource_loader::AsyncResourceLoader;
use smallbox::SmallBox;
use thiserror::Error;

use crate::core::async_io::context::IoContext;
use crate::render::async_loader::resources::async_loader_requests::ResourceLoadHandle;

/// Alias of `Result<T, TaskError>`
pub type TaskResult<T> = Result<T, TaskError>;

/// Interface of our tasks futures (once boxed, hence the `dyn`).
pub type TaskFuture<'a> = dyn Future<Output = TaskResult<()>> + 'a;

/// Alias of [`SmallBox`] that covers the requirements of a task payload.
pub type TaskPayload = SmallBox<dyn Any + Send + 'static, [u128; 6]>;

/// Factory object shared with the loader that will spawn the task future on the async loading
/// thread.
///
/// This type is not the future itself, it spawns the future. The expectation is that a task
/// factory is constructed once, at engine boot time, and is then shared with the async workers via
/// an `Arc`. The factory is then invoked via [`ITaskFactory::spawn_new`] with a per-task payload.
pub trait ITaskFactory: Send + Sync + 'static {
    /// Constructs a new boxed future that encapsulates the logic of the task that will be executed
    /// on the async loader thread.
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
    fn spawn_new<'a>(
        &self,
        ctx: IoContext,
        loader: &'a AsyncResourceLoader<ResourceLoadHandle>,
        msg: TaskPayload,
    ) -> Pin<Box<TaskFuture<'a>>>;
}

#[derive(Error, Debug)]
pub enum TaskError {
    /// Error code for when any IO request fails. The executor is expected to retire the task
    /// and continue working on other requests.
    #[error("An IO operation failed: {0}.")]
    Io(#[from] io::Error),

    /// There is not enough upload memory available to complete the upload request. The executor is
    /// expected to retire the task and continue working on other requests.
    #[error("Not enough memory available to complete the upload request.")]
    NotEnoughMemory,

    /// The task failed to create the GPU resource. The executor can retire the task and continue
    /// working when encountering this error.
    #[error("The task failed to create the GPU resource.")]
    ResourceCreationFailed,

    /// The task has failed as the GPU device has been lost. This is a GPU crash. This error is
    /// fatal for the loader, but the loader may still be cleanly shut down in some circumstances.
    #[error("The GPU device was lost.")]
    DeviceLost,

    /// The task has failed because the renderer it is uploading data for has disconnected. This
    /// will typically occur when the renderer is shut down.
    ///
    /// This error will cause the loader to attempt to cleanly shut down.
    #[error("The attached renderer object has disconnected.")]
    RendererDisconnected,

    /// This error occurs when the request channel has been closed. This means no new requests can
    /// be received and is thrown when the worker is awoken to find a closed channel.
    ///
    /// The loader should attempt to retire all in-flight work and shut down cleanly.
    #[error("The channel on which new upload requests are received has disconnected.")]
    SenderDisconnected,

    /// The task has failed because it was unable to record commands on the GPU.
    ///
    /// This error will cause the loader to attempt to cleanly shut down.
    #[error("An error occurred while recording and submitting commands to the GPU.")]
    CommandRecordingFailure,

    /// The task failed for some other reason. The executor can retire the task and continue working
    /// when encountering this error.
    #[error("An unknown error occurred.")]
    Other,

    /// A fatal error where the executor is expected to shut down cleanly. This should be returned
    /// when a task discovers an error that will prevent other tasks from completing.
    ///
    /// This fatal error class is returned when the error leaves the executor in a state where it is
    /// unable to correctly shut down cleanly. This is possible when an error occurs while an IO
    /// request is still outstanding, or if a GPU command buffer submission fails without a device
    /// lost.
    ///
    /// When it is not possible to prove resources are safe to destroy the executor will abort the
    /// application instead.
    #[error("A fatal error occurred that it is unsound to unwind. Root cause has been logged.")]
    FatalAbort,
}
