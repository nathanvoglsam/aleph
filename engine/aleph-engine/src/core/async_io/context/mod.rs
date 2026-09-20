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
use std::sync::Arc;

use aleph_gen_arena::{HandleType, RawHandle};
use aleph_io_queue::channel::{LoadChannel, OpenChannel, ReadChannel};
use aleph_vfs::async_io::AsyncIoMessage;
use aleph_vfs::file::{IAsyncVFile, IAsyncVFileExt};
use aleph_vfs::path::VPath;
use aleph_vfs::{IRouter, IRouterExt};

use crate::core::async_io::futures::{FileLoad, FileOpen, FileRead};

/// Context struct given to all async tasks that provides access to the executor and vfs.
///
/// Provides utilities so file IO can be performed asynchronously in a way the executor is able to
/// wake and poll the correct future with our completion based async io system.
pub struct IoContext<'a, T> {
    pub(crate) handle: RawHandle,
    pub(crate) sender: T,
    pub(crate) response_slot: &'a Cell<Option<AsyncIoMessage>>,
}

impl<'a, T> UnwindSafe for IoContext<'a, T> where
    T: ReadChannel<Arc<VPath>>
        + LoadChannel<Arc<VPath>>
        + OpenChannel<Arc<dyn IAsyncVFile>>
        + Clone
{
}
impl<'a, T> RefUnwindSafe for IoContext<'a, T> where
    T: ReadChannel<Arc<VPath>>
        + LoadChannel<Arc<VPath>>
        + OpenChannel<Arc<dyn IAsyncVFile>>
        + Clone
{
}

impl<'a, T> IoContext<'a, T>
where
    T: ReadChannel<Arc<VPath>>
        + LoadChannel<Arc<VPath>>
        + OpenChannel<Arc<dyn IAsyncVFile>>
        + Clone,
{
    /// Wrapper over [`IAsyncVFile::read_at`] that will correctly route the completion responses
    /// to the executor the future is running in.
    pub async unsafe fn read_file_at(
        &self,
        file: &dyn IAsyncVFile,
        buf: NonNull<[u8]>,
        offset: u64,
    ) -> io::Result<usize> {
        let result = unsafe {
            file.read_at(
                buf,
                offset,
                self.sender.clone(),
                self.handle.to_bare_handle().into_int().get(),
            )
        };

        let future = match result {
            Ok(_) => FileRead {
                response_slot: self.response_slot,
            },
            Err(_) => {
                log::error!("The async file worker has disconnected.");
                return Err(io::Error::from(io::ErrorKind::ConnectionAborted));
            }
        };

        future.await
    }

    /// Wrapper over [`IAsyncVFile::read_exact_at`] that will correctly route the completion
    /// responses to the executor the future is running in.
    pub async unsafe fn read_file_exact_at(
        &self,
        file: &dyn IAsyncVFile,
        buf: NonNull<[u8]>,
        offset: u64,
    ) -> io::Result<usize> {
        let result = unsafe {
            file.read_exact_at(
                buf,
                offset,
                self.sender.clone(),
                self.handle.to_bare_handle().into_int().get(),
            )
        };

        let future = match result {
            Ok(_) => FileRead {
                response_slot: self.response_slot,
            },
            Err(_) => {
                log::error!("The async file worker has disconnected.");
                return Err(io::Error::from(io::ErrorKind::ConnectionAborted));
            }
        };

        future.await
    }

    /// Wrapper over [`IAsyncVFile::load_file`] that will correctly route the completion responses
    /// to the executor the future is running in.
    pub async fn load_file(&self, file: &dyn IAsyncVFile) -> io::Result<Vec<u8>> {
        let result = file.load(
            self.sender.clone(),
            self.handle.to_bare_handle().into_int().get(),
        );

        let future = match result {
            Ok(_) => FileLoad {
                response_slot: self.response_slot,
            },
            Err(_) => {
                log::error!("The async file worker has disconnected.");
                return Err(io::Error::from(io::ErrorKind::ConnectionAborted));
            }
        };

        future.await
    }

    /// Wrapper over [`IRouter::open_file`] that will correctly route the completion responses
    /// to the executor the future is running in.
    pub async fn open_file(
        &self,
        vfs: &dyn IRouter,
        path: impl AsRef<VPath>,
    ) -> io::Result<Arc<dyn IAsyncVFile>> {
        let result = vfs.open_async(
            self.sender.clone(),
            path,
            self.handle.to_bare_handle().into_int().get(),
        );

        let future = match result {
            Ok(_) => FileOpen {
                response_slot: self.response_slot,
            },
            Err(_) => {
                log::error!("The async file worker has disconnected.");
                return Err(io::Error::from(io::ErrorKind::ConnectionAborted));
            }
        };

        future.await
    }
}
