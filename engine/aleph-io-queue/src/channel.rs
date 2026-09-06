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
use std::ptr::NonNull;

/// Abstraction over some channel (i.e. a queue or mpmc channel) for sending the results of an
/// asynchronous operation to a listener. This is how the async io system notifies results of async
/// operations.
pub trait OpenChannel<P>: Send + Sync + 'static {
    fn send_success(&self, opaque: u64, file: P) -> Result<(), ChannelError>;
    fn send_fail(&self, opaque: u64, file: P, err: io::Error) -> Result<(), ChannelError>;
}

/// Abstraction over some channel (i.e. a queue or mpmc channel) for sending the results of an
/// asynchronous operation to a listener. This is how the async io system notifies results of async
/// operations.
pub trait ReadChannel<P>: Send + Sync + 'static {
    fn send_success(
        &self,
        opaque: u64,
        file: P,
        buf: NonNull<[u8]>,
        offset: u64,
        bytes_transferred: usize,
    ) -> Result<(), ChannelError>;
    fn send_fail(
        &self,
        opaque: u64,
        file: P,
        buf: NonNull<[u8]>,
        offset: u64,
        err: io::Error,
    ) -> Result<(), ChannelError>;
}

/// Abstraction over some channel (i.e. a queue or mpmc channel) for sending the results of an
/// asynchronous operation to a listener. This is how the async io system notifies results of async
/// operations.
pub trait LoadChannel<P>: Send + Sync + 'static {
    fn send_success(&self, opaque: u64, file: P, data: Vec<u8>) -> Result<(), ChannelError>;
    fn send_fail(&self, opaque: u64, file: P, err: io::Error) -> Result<(), ChannelError>;
}

pub struct ChannelError;
