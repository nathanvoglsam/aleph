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
use std::sync::Arc;
use std::task::{Context, Poll};

use aleph_vfs::async_io::AsyncIoMessage;
use aleph_vfs::file::IAsyncVFile;
use crossbeam::queue::ArrayQueue;

/// Basic future that simply polls the executor's internal slot to receive an [`AsyncIoMessage`].
///
/// This will not work outside the executor it was designed to run in.
pub struct FileRead<'a> {
    pub(crate) response_slot: &'a ArrayQueue<AsyncIoMessage>,
}

impl<'a> Future for FileRead<'a> {
    type Output = io::Result<usize>;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.response_slot.pop() {
            Some(msg) => match msg {
                AsyncIoMessage::ReadSuccess {
                    bytes_transferred, ..
                } => Poll::Ready(Ok(bytes_transferred)),
                AsyncIoMessage::ReadFail { err, .. } => Poll::Ready(Err(err)),
                AsyncIoMessage::LoadSuccess { .. }
                | AsyncIoMessage::LoadFail { .. }
                | AsyncIoMessage::OpenSuccess { .. }
                | AsyncIoMessage::OpenFail { .. } => {
                    log::error!("Unexpected message type encountered in Future::poll");
                    Poll::Ready(Err(io::Error::from(io::ErrorKind::Other)))
                }
            },
            None => Poll::Pending,
        }
    }
}

/// Basic future that simply polls the executor's internal slot to receive an [`AsyncIoMessage`].
///
/// This will not work outside the executor it was designed to run in.
pub struct FileLoad<'a> {
    pub(crate) response_slot: &'a ArrayQueue<AsyncIoMessage>,
}

impl<'a> Future for FileLoad<'a> {
    type Output = io::Result<Vec<u8>>;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.response_slot.pop() {
            Some(msg) => match msg {
                AsyncIoMessage::LoadSuccess { data, .. } => Poll::Ready(Ok(data)),
                AsyncIoMessage::LoadFail { err, .. } => Poll::Ready(Err(err)),
                AsyncIoMessage::ReadSuccess { .. }
                | AsyncIoMessage::ReadFail { .. }
                | AsyncIoMessage::OpenSuccess { .. }
                | AsyncIoMessage::OpenFail { .. } => {
                    log::error!("Unexpected message type encountered in Future::poll");
                    Poll::Ready(Err(io::Error::from(io::ErrorKind::Other)))
                }
            },
            None => Poll::Pending,
        }
    }
}

/// Basic future that simply polls the executor's internal slot to receive an [`AsyncIoMessage`].
///
/// This will not work outside the executor it was designed to run in.
pub struct FileOpen<'a> {
    pub(crate) response_slot: &'a ArrayQueue<AsyncIoMessage>,
}

impl<'a> Future for FileOpen<'a> {
    type Output = io::Result<Arc<dyn IAsyncVFile>>;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.response_slot.pop() {
            Some(msg) => match msg {
                AsyncIoMessage::OpenSuccess { file, .. } => Poll::Ready(Ok(file)),
                AsyncIoMessage::OpenFail { err, .. } => Poll::Ready(Err(err)),
                AsyncIoMessage::ReadSuccess { .. }
                | AsyncIoMessage::ReadFail { .. }
                | AsyncIoMessage::LoadSuccess { .. }
                | AsyncIoMessage::LoadFail { .. } => {
                    log::error!("Unexpected message type encountered in Future::poll");
                    Poll::Ready(Err(io::Error::from(io::ErrorKind::Other)))
                }
            },
            None => Poll::Pending,
        }
    }
}
