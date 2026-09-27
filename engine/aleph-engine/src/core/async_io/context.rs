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

use aleph_vfs::file::IAsyncVFile;
use aleph_vfs::path::VPath;
use aleph_vfs::{IRouter, IRouterExt};
use crossbeam::channel::SendError;
use mg::async_resource_loader::AsyncResourceLoader;

use crate::core::async_io::internal::IoTask;
use crate::core::async_io::worker::AsyncLoaderQueue;

/// Context struct given to all async tasks that provides access to the executor and vfs.
///
/// Provides utilities so file IO can be performed asynchronously in a way the executor is able to
/// wake and poll the correct future with our completion based async io system.
pub struct IoContext<'a> {
    pub(crate) this: &'a AsyncLoaderQueue,
    pub(crate) loader: &'a AsyncResourceLoader<u64>,
}

impl<'a> IoContext<'a> {
    pub fn spawn<T>(&self, spawner: T) -> Result<(), SendError<()>>
    where
        for<'aa> T: (AsyncFnOnce(IoContext<'aa>) -> io::Result<()>) + Send + 'static,
    {
        match self.this.spawn(spawner) {
            Ok(()) => Ok(()),
            Err(_) => Err(SendError(())),
        }
    }

    /// Returns a new [`AsyncLoaderQueue`] that is connected to the executor that the current task
    /// is executing within. Can be used to issue more async tasks onto the same executor.
    pub fn async_queue(&self) -> AsyncLoaderQueue {
        self.this.clone()
    }

    /// Wrapper over [`IAsyncVFile::read_at`] that will correctly route the completion responses
    /// to the executor the future is running in.
    pub async unsafe fn read_file_at(
        &self,
        file: &dyn IAsyncVFile,
        buf: NonNull<[u8]>,
        offset: u64,
    ) -> io::Result<usize> {
        IoTask::new(|waker| {
            let result = unsafe { file.read_at(buf, offset, waker) };
            match result {
                Ok(_) => Ok(()),
                Err(_) => {
                    log::error!("The async file worker has disconnected.");
                    Err(io::Error::from(io::ErrorKind::ConnectionAborted))
                }
            }
        })
        .await
    }

    /// Wrapper over [`IAsyncVFile::read_exact_at`] that will correctly route the completion
    /// responses to the executor the future is running in.
    pub async unsafe fn read_file_exact_at(
        &self,
        file: &dyn IAsyncVFile,
        buf: NonNull<[u8]>,
        offset: u64,
    ) -> io::Result<usize> {
        IoTask::new(|waker| {
            let result = unsafe { file.read_exact_at(buf, offset, waker) };
            match result {
                Ok(_) => Ok(()),
                Err(_) => {
                    log::error!("The async file worker has disconnected.");
                    Err(io::Error::from(io::ErrorKind::ConnectionAborted))
                }
            }
        })
        .await
    }

    /// Wrapper over [`IAsyncVFile::load_file`] that will correctly route the completion responses
    /// to the executor the future is running in.
    pub async fn load_file(&self, file: &dyn IAsyncVFile) -> io::Result<Vec<u8>> {
        IoTask::new(|waker| {
            let result = file.load(waker);
            match result {
                Ok(_) => Ok(()),
                Err(_) => {
                    log::error!("The async file worker has disconnected.");
                    Err(io::Error::from(io::ErrorKind::ConnectionAborted))
                }
            }
        })
        .await
    }

    /// Wrapper over [`IRouter::open_file`] that will correctly route the completion responses
    /// to the executor the future is running in.
    pub async fn open_file(
        &self,
        vfs: &dyn IRouter,
        path: impl AsRef<VPath>,
    ) -> io::Result<Arc<dyn IAsyncVFile>> {
        let path = path.as_ref();
        IoTask::new(|waker| {
            let result = vfs.open_async(waker, path);
            match result {
                Ok(_) => Ok(()),
                Err(_) => {
                    log::error!("The async file worker has disconnected.");
                    Err(io::Error::from(io::ErrorKind::ConnectionAborted))
                }
            }
        })
        .await?;
        vfs.open_for_async_non_blocking(path)
    }

    /// Returns a reference to the [`AsyncResourceLoader`] that the executor owns.
    pub fn loader(&self) -> &AsyncResourceLoader<u64> {
        self.loader
    }
}
