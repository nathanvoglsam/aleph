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
use std::sync::Arc;

use aleph_io_queue::channel::{ChannelError, LoadChannel, OpenChannel, ReadChannel};
use crossbeam::channel::Sender;

use crate::file::IAsyncVFile;
use crate::path::VPath;

/// One possible message format for dispatching messages from [`LoadChannel`], [`ReadChannel`] or
/// [`OpenChannel`] objects.
pub enum AsyncIoMessage {
    ReadSuccess {
        /// The _virtual_ path that the read operation was reading from.
        path: Arc<VPath>,

        /// Buffer pointer that the data was read into.
        buf: NonNull<[u8]>,

        /// Offset in the file that the data was read from.
        offset: u64,

        /// The total number of bytes that were successfully transferred. This may not equal the
        /// number of bytes _requested_.
        bytes_transferred: usize,

        /// An opaque tag that can be used to associate the message with a particular request.
        opaque: u64,
    },
    ReadFail {
        /// The _virtual_ path that the read operation was reading from.
        path: Arc<VPath>,

        /// Buffer pointer that the data was read into.
        buf: NonNull<[u8]>,

        /// Offset in the file that the data was read from.
        offset: u64,

        /// The specific IO error that was thrown that caused the request to fail.
        err: io::Error,

        /// An opaque tag that can be used to associate the message with a particular request.
        opaque: u64,
    },
    LoadSuccess {
        /// The _virtual_ path that the read operation was reading from.
        path: Arc<VPath>,

        /// Buffer that contains the complete contents of the file.
        data: Vec<u8>,

        /// An opaque tag that can be used to associate the message with a particular request.
        opaque: u64,
    },
    LoadFail {
        /// The _virtual_ path that the read operation was reading from.
        path: Arc<VPath>,

        /// The specific IO error that was thrown that caused the request to fail.
        err: io::Error,

        /// An opaque tag that can be used to associate the message with a particular request.
        opaque: u64,
    },
    OpenSuccess {
        file: Arc<dyn IAsyncVFile>,
        /// An opaque tag that can be used to associate the message with a particular request.
        opaque: u64,
    },
    OpenFail {
        /// The specific IO error that was thrown that caused the request to fail.
        err: io::Error,

        /// An opaque tag that can be used to associate the message with a particular request.
        opaque: u64,
    },
}

impl AsyncIoMessage {
    /// Get the opaque tag from whichever message variant `self` contains.
    pub const fn opaque(&self) -> u64 {
        match self {
            AsyncIoMessage::ReadSuccess { opaque, .. } => *opaque,
            AsyncIoMessage::ReadFail { opaque, .. } => *opaque,
            AsyncIoMessage::LoadSuccess { opaque, .. } => *opaque,
            AsyncIoMessage::LoadFail { opaque, .. } => *opaque,
            AsyncIoMessage::OpenSuccess { opaque, .. } => *opaque,
            AsyncIoMessage::OpenFail { opaque, .. } => *opaque,
        }
    }
}

unsafe impl Send for AsyncIoMessage {}

/// New-type over a [`Sender`] to implement foreign traits.
#[repr(transparent)]
#[derive(Clone)]
pub struct AsyncIoSender(pub Sender<AsyncIoMessage>);

impl OpenChannel<Arc<dyn IAsyncVFile>> for AsyncIoSender {
    fn send_success(&self, opaque: u64, file: Arc<dyn IAsyncVFile>) -> Result<(), ChannelError> {
        let result = self.0.send(AsyncIoMessage::OpenSuccess { file, opaque });
        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(ChannelError),
        }
    }

    fn send_fail(
        &self,
        opaque: u64,
        _file: Arc<dyn IAsyncVFile>,
        err: io::Error,
    ) -> Result<(), ChannelError> {
        let result = self.0.send(AsyncIoMessage::OpenFail { err, opaque });
        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(ChannelError),
        }
    }
}

impl ReadChannel<Arc<VPath>> for AsyncIoSender {
    fn send_success(
        &self,
        opaque: u64,
        file: Arc<VPath>,
        buf: NonNull<[u8]>,
        offset: u64,
        bytes_transferred: usize,
    ) -> Result<(), ChannelError> {
        let result = self.0.send(AsyncIoMessage::ReadSuccess {
            path: file,
            buf,
            offset,
            bytes_transferred,
            opaque,
        });
        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(ChannelError),
        }
    }

    fn send_fail(
        &self,
        opaque: u64,
        file: Arc<VPath>,
        buf: NonNull<[u8]>,
        offset: u64,
        err: io::Error,
    ) -> Result<(), ChannelError> {
        let result = self.0.send(AsyncIoMessage::ReadFail {
            path: file,
            buf,
            offset,
            err,
            opaque,
        });
        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(ChannelError),
        }
    }
}

impl LoadChannel<Arc<VPath>> for AsyncIoSender {
    fn send_success(
        &self,
        opaque: u64,
        file: Arc<VPath>,
        data: Vec<u8>,
    ) -> Result<(), ChannelError> {
        let result = self.0.send(AsyncIoMessage::LoadSuccess {
            path: file,
            data,
            opaque,
        });
        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(ChannelError),
        }
    }

    fn send_fail(&self, opaque: u64, file: Arc<VPath>, err: io::Error) -> Result<(), ChannelError> {
        let result = self.0.send(AsyncIoMessage::LoadFail {
            path: file,
            err,
            opaque,
        });
        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(ChannelError),
        }
    }
}
