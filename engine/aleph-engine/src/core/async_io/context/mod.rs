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

use std::cell::Cell;
use std::io;
use std::panic::{RefUnwindSafe, UnwindSafe};
use std::ptr::NonNull;
use std::rc::Rc;

use aleph_gen_arena::{HandleType, RawHandle};
use aleph_vfs::file::{AsyncReadResponse, IAsyncVFile};
use crossbeam::channel::Sender;

use crate::core::async_io::futures::AsyncRead;

/// Context struct given to all async tasks that provides access to the executor and vfs.
///
/// Provides utilities so file IO can be performed asynchronously in a way the executor is able to
/// wake and poll the correct future with our completion based async io system.
pub struct IoContext {
    pub(crate) handle: RawHandle,
    pub(crate) sender: Sender<AsyncReadResponse>,
    pub(crate) response_slot: Rc<Cell<Option<AsyncReadResponse>>>,
}

impl UnwindSafe for IoContext {}
impl RefUnwindSafe for IoContext {}

impl IoContext {
    /// Wrapper over [`IAsyncVFile::read_at`] that will correctly route the completion responses
    /// to the executor the future is running in.
    pub unsafe fn read_file_at(
        &self,
        file: &dyn IAsyncVFile,
        buf: NonNull<[u8]>,
        offset: u64,
    ) -> io::Result<AsyncRead<'_>> {
        unsafe {
            file.read_at(
                buf,
                offset,
                self.sender.clone(),
                self.handle.to_bare_handle().into_int().get(),
            )
            .map_err(|_| {
                io::Error::new(
                    io::ErrorKind::ConnectionAborted,
                    "The async file worker has disconnected.",
                )
            })?;
        }
        Ok(AsyncRead {
            response_slot: self.response_slot.as_ref(),
        })
    }

    /// Wrapper over [`IAsyncVFile::read_exact_at`] that will correctly route the completion
    /// responses to the executor the future is running in.
    pub unsafe fn read_file_exact_at(
        &self,
        file: &dyn IAsyncVFile,
        buf: NonNull<[u8]>,
        offset: u64,
    ) -> io::Result<AsyncRead<'_>> {
        unsafe {
            file.read_exact_at(
                buf,
                offset,
                self.sender.clone(),
                self.handle.to_bare_handle().into_int().get(),
            )
            .map_err(|_| {
                io::Error::new(
                    io::ErrorKind::ConnectionAborted,
                    "The async file worker has disconnected.",
                )
            })?;
        }
        Ok(AsyncRead {
            response_slot: self.response_slot.as_ref(),
        })
    }

    /// Wrapper over [`IAsyncVFile::load_file`] that will correctly route the completion responses
    /// to the executor the future is running in.
    pub fn load_file(&self, file: &dyn IAsyncVFile) -> io::Result<AsyncRead<'_>> {
        file.load(
            self.sender.clone(),
            self.handle.to_bare_handle().into_int().get(),
        )
        .map_err(|_| {
            io::Error::new(
                io::ErrorKind::ConnectionAborted,
                "The async file worker has disconnected.",
            )
        })?;
        Ok(AsyncRead {
            response_slot: self.response_slot.as_ref(),
        })
    }
}
