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

use aleph_io_queue::channel::IoWaker;
use aleph_vfs::file::IAsyncVFile;
use aleph_vfs::path::VPath;
use aleph_vfs::{IRouter, IRouterExt};

/// Basic future that simply polls the executor's internal slot to receive an [`IoMessage`].
///
/// This will not work outside the executor it was designed to run in.
pub struct FileRead {
    pub(crate) waker: Arc<IoWaker<io::Result<usize>>>,
}

impl Future for FileRead {
    type Output = io::Result<usize>;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(waker) = Arc::get_mut(&mut self.waker) {
            let result = match waker.take() {
                Some(v) => v,
                None => Err(io::Error::from(io::ErrorKind::Other)),
            };
            Poll::Ready(result)
        } else {
            // We should re-queue the task if we fail to get the waker with get_mut. There's a small
            // timing overlap where it's possible for the task to be polled but the worker thread
            // hasn't dropped the waker handle yet. We just need to wait for it to be dropped so
            // we just enqueue the task to be polled again at the end of the queue.
            let _ = self.waker.wake();
            Poll::Pending
        }
    }
}

/// Basic future that simply polls the executor's internal slot to receive an [`IoMessage`].
///
/// This will not work outside the executor it was designed to run in.
pub struct FileLoad {
    pub(crate) waker: Arc<IoWaker<io::Result<Vec<u8>>>>,
}

impl Future for FileLoad {
    type Output = io::Result<Vec<u8>>;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(waker) = Arc::get_mut(&mut self.waker) {
            let result = match waker.take() {
                Some(v) => v,
                None => Err(io::Error::from(io::ErrorKind::Other)),
            };
            Poll::Ready(result)
        } else {
            // We should re-queue the task if we fail to get the waker with get_mut. There's a small
            // timing overlap where it's possible for the task to be polled but the worker thread
            // hasn't dropped the waker handle yet. We just need to wait for it to be dropped so
            // we just enqueue the task to be polled again at the end of the queue.
            let _ = self.waker.wake();
            Poll::Pending
        }
    }
}

/// Basic future that simply polls the executor's internal slot to receive an [`AsyncIoMessage`].
///
/// This will not work outside the executor it was designed to run in.
pub struct FileOpen<'a> {
    pub(crate) path: &'a VPath,
    pub(crate) vfs: &'a dyn IRouter,
    pub(crate) waker: Arc<IoWaker<io::Result<()>>>,
}

impl<'a> Future for FileOpen<'a> {
    type Output = io::Result<Arc<dyn IAsyncVFile>>;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(waker) = Arc::get_mut(&mut self.waker) {
            let result = match waker.take() {
                Some(v) => v,
                None => return Poll::Ready(Err(io::Error::from(io::ErrorKind::Other))),
            };
            match result {
                Ok(_) => {}
                Err(e) => return Poll::Ready(Err(e)),
            }
            let file = self.vfs.open_for_async_non_blocking(self.path);
            Poll::Ready(file)
        } else {
            // We should re-queue the task if we fail to get the waker with get_mut. There's a small
            // timing overlap where it's possible for the task to be polled but the worker thread
            // hasn't dropped the waker handle yet. We just need to wait for it to be dropped so
            // we just enqueue the task to be polled again at the end of the queue.
            let _ = self.waker.wake();
            Poll::Pending
        }
    }
}
