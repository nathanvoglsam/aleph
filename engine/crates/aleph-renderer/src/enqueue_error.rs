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

use std::fmt::Formatter;

use thiserror::Error;

pub struct EnqueueError<T> {
    /// A slot for returning owned data that was passed into an enqueue function that should not
    /// be dropped inside the enqueue function. This is used to return ownership back to the caller
    /// in the event of the owner.
    ///
    /// If you don't care just grab the 'err' field and the caller will drop 'data' itself.
    pub data: T,

    /// The actual error this struct encapsulates
    pub err: EnqueueErrorKind,
}

impl<T> EnqueueError<T> {
    pub const fn new(err: EnqueueErrorKind, data: T) -> Self {
        Self { data, err }
    }
}

impl<T> std::error::Error for EnqueueError<T> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.err.source()
    }

    fn description(&self) -> &str {
        #[allow(deprecated)]
        self.err.description()
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        #[allow(deprecated)]
        self.err.cause()
    }
}

impl<T> std::fmt::Debug for EnqueueError<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.err, f)
    }
}

impl<T> std::fmt::Display for EnqueueError<T> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.err, f)
    }
}

#[derive(Error, Debug)]
pub enum EnqueueErrorKind {
    #[error("The queue is full and the request could not be placed into the queue.")]
    QueueFull,

    #[error("The request object is already enqueued.")]
    RequestAlreadyQueued,
}

impl EnqueueErrorKind {
    pub const fn with_data<T>(self, data: T) -> EnqueueError<T> {
        EnqueueError::new(self, data)
    }
}
