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

use std::sync::{Arc, OnceLock};

use aleph_gen_arena::RawHandle;
use crossbeam::channel::{SendError, Sender};

pub struct IoWaker<T> {
    handle: RawHandle,
    notify: Sender<RawHandle>,
    result: OnceLock<T>,
}

impl<T> IoWaker<T> {
    /// Constructs a new [`IoWaker`] in the default, unsignalled and unresolved state.
    pub fn new(handle: RawHandle, notify: Sender<RawHandle>) -> Arc<Self> {
        Arc::new(Self {
            handle,
            notify,
            result: OnceLock::new(),
        })
    }

    /// Attempt to resolve the task by providing the given result value. Will fail if the task has
    /// already been resolved.
    pub fn resolve(&self, result: T) -> Result<(), T> {
        self.result.set(result)
    }

    /// Attempt to take the message that was resolved onto this waker. This should be called to
    /// poll for completion of the task.
    ///
    /// # Mutability
    ///
    /// The expected use case is for two handles to exist to a waker wrapped in an `Arc`. One for
    /// the future object running in the async executor, and the other for whoever is performing the
    /// asynchronous work.
    ///
    /// The async worker is expected to complete the task, call [`IoWaker::resolve`] to provide the
    /// result and then call [`IoWaker::wake`] to signal to the async executor that the async fn
    /// should be scheduled to poll.
    ///
    /// # Timing
    ///
    /// There is a timing window in between [`IoWaker::wake`] being called and the async worker
    /// dropping the `IoWaker` handle where the async executor could try and poll the future. Future
    /// implementations backed by `IoWaker` expect to have exclusive ownership of the waker by the
    /// time `poll` is called. It's theoretically possible for `Arc::get_mut` to fail.
    ///
    /// The solution is to re-queue the future/async fn (perhaps with a timer) and return `Pending`.
    /// The failure mode would be incredibly rare as the timing window is miniscule.
    pub fn take(&mut self) -> Option<T> {
        self.result.take()
    }

    /// Send a notification to the async executor to enqueue the associated future/async fn to be
    /// polled again.
    pub fn wake(&self) -> Result<(), SendError<()>> {
        match self.notify.send(self.handle) {
            Ok(_) => Ok(()),
            Err(_) => Err(SendError(())),
        }
    }

    /// Get the handle the waker was created with.
    pub const fn handle(&self) -> RawHandle {
        self.handle
    }
}
