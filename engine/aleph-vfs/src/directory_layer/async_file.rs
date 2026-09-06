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
use std::path::Path;
use std::ptr::NonNull;
use std::sync::Arc;

use aleph_io_queue::IoQueue;
use aleph_io_queue::channel::{ChannelError, LoadChannel, OpenChannel, ReadChannel};
use crossbeam::channel::SendError;
use smallbox::SmallBox;

use crate::file::IAsyncVFile;
use crate::path::VPath;

pub struct AsyncVFile {
    pub queue: Arc<IoQueue>,
    pub virtual_path: Arc<VPath>,
    pub path: Arc<Path>,
}

impl IAsyncVFile for AsyncVFile {
    unsafe fn __read_at(
        &self,
        buf: NonNull<[u8]>,
        offset: u64,
        sender: SmallBox<dyn ReadChannel<Arc<VPath>>, [u128; 1]>,
        opaque: u64,
    ) -> Result<(), SendError<()>> {
        unsafe {
            let remap_sender = RemapReadSender {
                path: self.virtual_path.clone(),
                sender,
            };
            self.queue
                .async_read(self.path.clone(), buf, offset, remap_sender, opaque)
        }
    }

    unsafe fn __read_exact_at(
        &self,
        buf: NonNull<[u8]>,
        offset: u64,
        sender: SmallBox<dyn ReadChannel<Arc<VPath>>, [u128; 1]>,
        opaque: u64,
    ) -> Result<(), SendError<()>> {
        unsafe {
            let remap_sender = RemapReadSender {
                path: self.virtual_path.clone(),
                sender,
            };
            self.queue
                .async_read_exact(self.path.clone(), buf, offset, remap_sender, opaque)
        }
    }

    fn __load(
        &self,
        sender: SmallBox<dyn LoadChannel<Arc<VPath>>, [u128; 1]>,
        opaque: u64,
    ) -> Result<(), SendError<()>> {
        let remap_sender = RemapLoadSender {
            path: self.virtual_path.clone(),
            sender,
        };
        self.queue
            .async_load(self.path.clone(), remap_sender, opaque)
    }
}

/// This is an internal [`ISender`] implementation that's intended to be used for async io on a
/// directory layer backed vfile. This handles remapping the raw file io results into virtual file
/// io results.
pub struct RemapOpenSender {
    pub file: Arc<dyn IAsyncVFile>,
    pub sender: SmallBox<dyn OpenChannel<Arc<dyn IAsyncVFile>>, [u128; 1]>,
}

impl<P> OpenChannel<P> for RemapOpenSender {
    fn send_success(&self, opaque: u64, _file: P) -> Result<(), ChannelError> {
        self.sender.send_success(opaque, self.file.clone())
    }

    fn send_fail(&self, opaque: u64, _file: P, err: io::Error) -> Result<(), ChannelError> {
        self.sender.send_fail(opaque, self.file.clone(), err)
    }
}

/// This is an internal [`ISender`] implementation that's intended to be used for async io on a
/// directory layer backed vfile. This handles remapping the raw file io results into virtual file
/// io results.
pub struct RemapReadSender {
    pub path: Arc<VPath>,
    pub sender: SmallBox<dyn ReadChannel<Arc<VPath>>, [u128; 1]>,
}

impl<P> ReadChannel<P> for RemapReadSender {
    fn send_success(
        &self,
        opaque: u64,
        _file: P,
        buf: NonNull<[u8]>,
        offset: u64,
        bytes_transferred: usize,
    ) -> Result<(), ChannelError> {
        self.sender
            .send_success(opaque, self.path.clone(), buf, offset, bytes_transferred)
    }

    fn send_fail(
        &self,
        opaque: u64,
        _file: P,
        buf: NonNull<[u8]>,
        offset: u64,
        err: io::Error,
    ) -> Result<(), ChannelError> {
        self.sender
            .send_fail(opaque, self.path.clone(), buf, offset, err)
    }
}

/// This is an internal [`ISender`] implementation that's intended to be used for async io on a
/// directory layer backed vfile. This handles remapping the raw file io results into virtual file
/// io results.
pub struct RemapLoadSender {
    pub path: Arc<VPath>,
    pub sender: SmallBox<dyn LoadChannel<Arc<VPath>>, [u128; 1]>,
}

impl<P> LoadChannel<P> for RemapLoadSender {
    fn send_success(&self, opaque: u64, _file: P, data: Vec<u8>) -> Result<(), ChannelError> {
        self.sender.send_success(opaque, self.path.clone(), data)
    }

    fn send_fail(&self, opaque: u64, _file: P, err: io::Error) -> Result<(), ChannelError> {
        self.sender.send_fail(opaque, self.path.clone(), err)
    }
}
