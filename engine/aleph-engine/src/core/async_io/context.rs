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
use std::rc::Rc;
use std::sync::Arc;

use aleph_vfs::file::IAsyncVFile;
use aleph_vfs::path::VPath;
use aleph_vfs::{IRouter, IRouterExt};
use crossbeam::channel::SendError;
use mg::async_resource_loader::AsyncResourceLoader;
use tokio::sync::mpsc::WeakUnboundedSender;

use crate::core::async_io::internal::{FutureSpawner, IoTask};
use crate::core::async_io::worker::{AsyncLoaderQueue, WorkerMessage};

/// Context struct given to all async tasks that provides access to the executor and vfs.
///
/// Provides utilities so file IO can be performed asynchronously in a way the executor is able to
/// wake and poll the correct future with our completion based async io system.
///
/// # No Clone?
///
/// This struct is not cloneable to ensure that all GPU IO tasks are dispatched by our internal
/// dispatcher task. GPU IO tasks must be registered with a local set to prevent GPU IO tasks from
/// being dropped (potentially with outstanding completion based IO requests).
///
/// It would be very elegant to allow using `spawn_local` directly, cloning a context as
/// needed, but this is not currently possible due to the requirement that all GPU IO tasks must
/// be dispatched through the internal dispatcher to ensure proper handling of completion based IO.
pub struct IoContext {
    pub(crate) this: WeakUnboundedSender<WorkerMessage>,
    pub(crate) loader: Rc<AsyncResourceLoader<u64>>,
}

impl IoContext {
    /// Spawns a new async task onto the executor associated with this context.
    ///
    /// The provided closure will be executed with an [`IoContext`] that is connected to the same
    /// executor. Returns an error if the task could not be spawned.
    pub fn spawn<T>(&self, spawner: T) -> Result<(), SendError<()>>
    where
        T: (AsyncFnOnce(&IoContext) -> io::Result<()>) + Send + 'static,
    {
        let this = self.this.upgrade().ok_or(SendError(()))?;
        let spawner = FutureSpawner::new(spawner);
        match this.send(WorkerMessage::Spawn(spawner)) {
            Ok(()) => Ok(()),
            Err(_) => Err(SendError(())),
        }
    }

    /// Returns a new [`AsyncLoaderQueue`] that is connected to the executor that the current task
    /// is executing within. Can be used to issue more async tasks onto the same executor.
    pub fn async_queue(&self) -> Option<AsyncLoaderQueue> {
        AsyncLoaderQueue::from_weak(&self.this)
    }

    /// Wrapper over [`IAsyncVFile::read_at`] that will correctly route the completion responses
    /// to the executor the future is running in.
    pub async unsafe fn read_file_at(
        &self,
        file: &dyn IAsyncVFile,
        buf: NonNull<[u8]>,
        offset: u64,
    ) -> io::Result<usize> {
        let task =
            IoTask::new(|waker| unsafe { file.read_at(buf, offset, waker).map_err(map_send_err) });
        task.await
    }

    /// Wrapper over [`IAsyncVFile::read_exact_at`] that will correctly route the completion
    /// responses to the executor the future is running in.
    pub async unsafe fn read_file_exact_at(
        &self,
        file: &dyn IAsyncVFile,
        buf: NonNull<[u8]>,
        offset: u64,
    ) -> io::Result<usize> {
        let task = IoTask::new(|waker| unsafe {
            file.read_exact_at(buf, offset, waker).map_err(map_send_err)
        });
        task.await
    }

    /// Wrapper over [`IAsyncVFile::load_file`] that will correctly route the completion responses
    /// to the executor the future is running in.
    pub async fn load_file(&self, file: &dyn IAsyncVFile) -> io::Result<Vec<u8>> {
        let task = IoTask::new(|waker| file.load(waker).map_err(map_send_err));
        task.await
    }

    /// Wrapper over [`IRouter::open_file`] that will correctly route the completion responses
    /// to the executor the future is running in.
    pub async fn open_file(
        &self,
        vfs: &dyn IRouter,
        path: impl AsRef<VPath>,
    ) -> io::Result<Arc<dyn IAsyncVFile>> {
        let path = path.as_ref();
        let task = IoTask::new(|waker| vfs.open_async(waker, path));
        task.await?;
        vfs.open_for_async_non_blocking(path)
    }

    /// Returns a reference to the [`AsyncResourceLoader`] that the executor owns.
    pub fn loader(&self) -> &AsyncResourceLoader<u64> {
        &self.loader
    }
}

fn map_send_err<T>(_: SendError<T>) -> io::Error {
    log::error!("The async file worker has disconnected.");
    io::Error::from(io::ErrorKind::ConnectionAborted)
}
