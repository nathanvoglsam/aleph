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
use aleph_io_queue::channel::IoWaker;
use crossbeam::channel::SendError;

use crate::file::IAsyncVFile;
use crate::path::VPath;

pub struct AsyncVFile {
    pub queue: Arc<IoQueue>,
    pub virtual_path: Arc<VPath>,
    pub path: Arc<Path>,
}

impl IAsyncVFile for AsyncVFile {
    unsafe fn read_at(
        &self,
        buf: NonNull<[u8]>,
        offset: u64,
        waker: Arc<IoWaker<io::Result<usize>>>,
    ) -> Result<(), SendError<()>> {
        unsafe { self.queue.async_read(self.path.clone(), buf, offset, waker) }
    }

    unsafe fn read_exact_at(
        &self,
        buf: NonNull<[u8]>,
        offset: u64,
        waker: Arc<IoWaker<io::Result<usize>>>,
    ) -> Result<(), SendError<()>> {
        unsafe { self.queue.async_read(self.path.clone(), buf, offset, waker) }
    }

    fn load(&self, waker: Arc<IoWaker<io::Result<Vec<u8>>>>) -> Result<(), SendError<()>> {
        self.queue.async_load(self.path.clone(), waker)
    }

    fn path(&self) -> &VPath {
        self.virtual_path.as_ref()
    }
}
