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

use std::io;
use std::pin::Pin;

use thiserror::Error;

use crate::core::async_io::context::IoContext;

#[derive(Error, Debug)]
pub enum TaskError {
    /// Error code for when any IO request fails. The executor is expected to retire the task
    /// and continue working on other requests.
    #[error("An IO operation failed: {0}.")]
    Io(#[from] io::Error),

    /// The task failed for some other reason. The executor can retire the task and continue working
    /// when encountering this error.
    #[error("An unknown error occurred.")]
    Other,
}

/// Alias of `Result<T, TaskError>`
pub type TaskResult<T> = Result<T, TaskError>;

/// Interface of our tasks futures (once boxed, hence the `dyn`).
pub type TaskFuture<'a> = dyn Future<Output = TaskResult<()>> + 'a;

/// Factory object shared with the loader that will spawn the task future in the executor.
///
/// This type is not the future itself, it spawns the future.
pub trait ITask: Send + 'static {
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
    fn spawn_new<'a>(self, ctx: IoContext) -> Pin<Box<TaskFuture<'a>>>;
}
