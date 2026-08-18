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

pub mod local_handle_cache;
pub mod top_level_handle_cache;

use std::io;
use std::io::Read;
use std::panic::catch_unwind;
use std::path::Path;
use std::process::abort;
use std::ptr::NonNull;
use std::sync::Arc;
use std::thread::JoinHandle;

use aleph_alloc::BVec;
use aleph_alloc::instrumentation::{IAllocationCategory, system};
use crossbeam::channel::{Receiver, SendError, Sender, unbounded};

use crate::async_io::local_handle_cache::LocalHandleCache;
use crate::async_io::top_level_handle_cache::TopLevelHandleCache;

pub struct IoQueue {
    top_level_handle_cache: Arc<TopLevelHandleCache>,
    threads: BVec<JoinHandle<()>, AsyncIoSystem>,
    sender: Option<Sender<AsyncRequest>>,
}

impl IoQueue {
    pub fn new(top_level_handle_cache: Arc<TopLevelHandleCache>) -> Arc<IoQueue> {
        let (sender, receiver) = AsyncIo::with(|| unbounded());

        let mut threads = BVec::with_capacity_in(top_level_handle_cache.num_threads(), system());
        for thread_id in 0..top_level_handle_cache.num_threads() {
            AsyncIo::with(|| {
                let recv = receiver.clone();
                let top_level_handle_cache = top_level_handle_cache.clone();
                let thread = std::thread::Builder::new()
                    .name(format!("async-file-io-queue-{}", thread_id))
                    .spawn(move || {
                        let mut worker = IoQueueWorker {
                            thread_id,
                            recv,
                            handle_cache: LocalHandleCache::new(top_level_handle_cache),
                        };
                        let result = catch_unwind(move || {
                            worker.run();
                        });
                        match result {
                            Ok(_) => {}
                            Err(_) => {
                                // We must promote any unhandled error or panic in a worker thread
                                // to a full app abort. Clients of this queue are unable to end the
                                // lifetime of the dst buffers until the worker sends a completion
                                // message.
                                //
                                // If the worker panics then that message will never be sent.
                                // Leaking requests must _not_ happen, so we abort instead.
                                // Otherwise, clients would wait for a message that will never come.
                                //
                                // W/e caused the panic will likely happen again so we will leak
                                // to death, or the whole ioqueue is so borked we can't do io
                                // anymore. W/e the cause it's likely unrecoverable.
                                log::error!("IoQueue worker has panicked. Aborting.");
                                abort()
                            }
                        }
                    })
                    .expect("Failed to spawn IoQueue thread");
                threads.push(thread);
            });
        }

        let out = Self {
            top_level_handle_cache,
            threads,
            sender: Some(sender),
        };

        AsyncIo::with(move || Arc::new(out))
    }

    /// Opens the given 'file' for use within the async io system.
    ///
    /// This will prime the top level handle cache for the given file, and return if the file failed
    /// to open.
    pub fn open(&self, file: &Path) -> Result<(), std::io::Error> {
        self.top_level_handle_cache.get_or_open(file)?;
        Ok(())
    }

    /// Enqueue an async read operation from the given file.
    ///
    /// Will attempt to read at most `buf.len()` bytes from the given file, starting from the given
    /// `offset`. Results for the request will be sent pushed onto the channel using the given
    /// `sender` and `opaque` data.
    ///
    /// # Safety
    ///
    /// The async queue takes temporary ownership of `buf` while the request in flight. It is the
    /// caller's responsibility to respect the transferred ownership until a message is returned
    /// on the response queue that ends the lifetime of the dynamic borrow.
    pub unsafe fn async_read(
        &self,
        file: Arc<Path>,
        dst: NonNull<[u8]>,
        offset: u64,
        sender: Arc<dyn ISender>,
        opaque: [u64; 4],
    ) -> Result<(), SendError<()>> {
        AsyncIo::with(|| {
            let channel = self.sender.as_ref().unwrap();
            let result = channel.send(AsyncRequest::ReadData {
                file,
                buf: dst,
                offset,
                sender,
                opaque,
            });

            match result {
                Ok(_) => Ok(()),
                Err(_) => Err(SendError(())),
            }
        })
    }

    /// Enqueue an async read operation from the given file.
    ///
    /// Will attempt to read `buf.len()` bytes from the given file, starting from the given
    /// `offset`. Results for the request will be sent pushed onto the channel using the given
    /// `sender` and `opaque` data.
    ///
    /// This is related to the `read_exact` family of calls on [`std::fs::File`]. It will try to
    /// read data to fill the entire buffer. It may issue multiple read syscalls to do so. If any
    /// of the individual read calls fail, or we hit EOF before filling the buffer, then the entire
    /// request is considered as failed.
    ///
    /// # Safety
    ///
    /// The async queue takes temporary ownership of `buf` while the request in flight. It is the
    /// caller's responsibility to respect the transferred ownership until a message is returned
    /// on the response queue that ends the lifetime of the dynamic borrow.
    pub unsafe fn async_read_exact(
        &self,
        file: Arc<Path>,
        dst: NonNull<[u8]>,
        offset: u64,
        sender: Arc<dyn ISender>,
        opaque: [u64; 4],
    ) -> Result<(), SendError<()>> {
        AsyncIo::with(|| {
            let channel = self.sender.as_ref().unwrap();
            let result = channel.send(AsyncRequest::ReadDataExact {
                file,
                buf: dst,
                offset,
                sender,
                opaque,
            });

            match result {
                Ok(_) => Ok(()),
                Err(_) => Err(SendError(())),
            }
        })
    }

    /// Enqueue an async _load_ operation. This will attempt to load the entire file into a buffer
    /// and then pass that buffer back via the given `sender`.
    pub fn async_load(
        &self,
        file: Arc<Path>,
        sender: Arc<dyn ISender>,
        opaque: [u64; 4],
    ) -> Result<(), SendError<()>> {
        AsyncIo::with(|| {
            let channel = self.sender.as_ref().unwrap();
            let result = channel.send(AsyncRequest::LoadFile {
                file,
                sender,
                opaque,
            });

            match result {
                Ok(_) => Ok(()),
                Err(_) => Err(SendError(())),
            }
        })
    }
}

impl Drop for IoQueue {
    fn drop(&mut self) {
        // Drop the sender, which will notify all the threads the channel has been terminated.
        self.sender = None;

        // Wait for all the worker threads to close.
        self.threads.drain(..).for_each(|t| t.join().unwrap());
    }
}

pub struct AsyncIo;
aleph_alloc::new_alloc_category!(AsyncIo, "019feb3d-4b0e-7d50-915f-3d01fb2baf6d");

pub type AsyncIoSystem = aleph_alloc::instrumentation::Instrumented<AsyncIo>;

struct IoQueueWorker {
    /// Assigned ID of the worker. Used to index the file handles to reduce contention to shared
    /// OS resources.
    thread_id: usize,

    /// The listener side of the channel where requests will be received on.
    recv: Receiver<AsyncRequest>,

    /// A local handle cache instance. !Send worker level cache that is a client of a higher level
    /// file handle cache.
    handle_cache: LocalHandleCache,
}

impl IoQueueWorker {
    fn run(&mut self) {
        while let Ok(msg) = AsyncIo::with(|| self.recv.recv()) {
            match msg {
                AsyncRequest::ReadData {
                    file,
                    mut buf,
                    offset,
                    sender,
                    opaque,
                } => {
                    let handle_set = match self.handle_cache.get_or_open(&file) {
                        Ok(v) => v,
                        Err(err) => {
                            // We don't care if the receiver hung up or not as there's nothing we
                            // can do about it
                            let _ = sender.send_fail(opaque, file, buf, offset, err);
                            continue;
                        }
                    };

                    let handle = &handle_set[self.thread_id % handle_set.len()];
                    let result: io::Result<usize> = cfg_select! {
                        unix => unsafe {
                            use std::os::unix::fs::FileExt;
                            handle.read_at(buf.as_mut(), offset)
                        },
                        windows => unsafe {
                            use std::os::windows::fs::FileExt;
                            handle.seek_read(buf.as_mut(), offset)
                        },
                        _ => {
                            unimplemented!()
                        }
                    };

                    let bytes_transferred = match result {
                        Ok(v) => v,
                        Err(err) => {
                            // We don't care if the receiver hung up or not as there's nothing we
                            // can do about it
                            let _ = sender.send_fail(opaque, file, buf, offset, err);
                            continue;
                        }
                    };

                    // We don't care if the receiver hung up or not as there's nothing we can do
                    // about it
                    let _ = sender.send_success(opaque, file, buf, offset, bytes_transferred);
                }
                AsyncRequest::ReadDataExact {
                    file,
                    mut buf,
                    offset,
                    sender,
                    opaque,
                } => {
                    let handle_set = match self.handle_cache.get_or_open(&file) {
                        Ok(v) => v,
                        Err(err) => {
                            // We don't care if the receiver hung up or not as there's nothing we
                            // can do about it
                            let _ = sender.send_fail(opaque, file, buf, offset, err);
                            continue;
                        }
                    };

                    let handle = &handle_set[self.thread_id % handle_set.len()];
                    let result: io::Result<()> = cfg_select! {
                        unix => unsafe {
                            use std::os::unix::fs::FileExt;
                            handle.read_exact_at(buf.as_mut(), offset)
                        },
                        windows => unsafe {
                            let mut buf = buf.as_mut();
                            let mut offset = offset;
                            while !buf.is_empty() {
                                match handle.seek_read(buf, offset) {
                                    Ok(0) => break,
                                    Ok(n) => {
                                        let tmp = buf;
                                        buf = &mut tmp[n..];
                                        offset += n as u64;
                                    }
                                    Err(ref e) if e.is_interrupted() => {}
                                    Err(e) => break Err(e),
                                }
                            }
                            if !buf.is_empty() {
                                Err(io::Error::new(
                                    io::ErrorKind::UnexpectedEof,
                                    "failed to fill whole buffer",
                                ))
                            } else {
                                Ok(())
                            }
                        },
                        _ => {
                            unimplemented!()
                        }
                    };

                    let bytes_transferred = match result {
                        Ok(_) => buf.len(),
                        Err(err) => {
                            // We don't care if the receiver hung up or not as there's nothing we
                            // can do about it
                            let _ = sender.send_fail(opaque, file, buf, offset, err);
                            continue;
                        }
                    };

                    // We don't care if the receiver hung up or not as there's nothing we can do
                    // about it
                    let _ = sender.send_success(opaque, file, buf, offset, bytes_transferred);
                }
                AsyncRequest::LoadFile {
                    file,
                    sender,
                    opaque,
                } => {
                    let handle_set = match self.handle_cache.get_or_open(&file) {
                        Ok(v) => v,
                        Err(err) => {
                            // We don't care if the receiver hung up or not as there's nothing we
                            // can do about it
                            let _ = sender.send_load_fail(opaque, file, err);
                            continue;
                        }
                    };

                    let mut handle = &handle_set[self.thread_id % handle_set.len()];

                    let mut buf = Vec::new();
                    let result = handle.read_to_end(&mut buf);

                    match result {
                        Ok(_) => {}
                        Err(err) => {
                            // We don't care if the receiver hung up or not as there's nothing we
                            // can do about it
                            let _ = sender.send_load_fail(opaque, file, err);
                            continue;
                        }
                    };

                    // We don't care if the receiver hung up or not as there's nothing we can do
                    // about it
                    let _ = sender.send_load_success(opaque, file, buf);
                }
            }
        }
    }
}

pub trait ISender: Send + Sync {
    fn send_success(
        &self,
        opaque: [u64; 4],
        file: Arc<Path>,
        buf: NonNull<[u8]>,
        offset: u64,
        bytes_transferred: usize,
    ) -> Result<(), SendError<()>>;
    fn send_fail(
        &self,
        opaque: [u64; 4],
        file: Arc<Path>,
        buf: NonNull<[u8]>,
        offset: u64,
        err: io::Error,
    ) -> Result<(), SendError<()>>;
    fn send_load_success(
        &self,
        opaque: [u64; 4],
        file: Arc<Path>,
        data: Vec<u8>,
    ) -> Result<(), SendError<()>>;
    fn send_load_fail(
        &self,
        opaque: [u64; 4],
        file: Arc<Path>,
        err: io::Error,
    ) -> Result<(), SendError<()>>;
}

enum AsyncRequest {
    ReadData {
        /// The path to the file to read
        file: Arc<Path>,

        /// Destination address to read at most `len` bytes into
        buf: NonNull<[u8]>,

        /// Offset into the file that data should be read from
        offset: u64,

        /// The channel on which to send result messages to
        sender: Arc<dyn ISender>,

        /// Opaque
        opaque: [u64; 4],
    },
    ReadDataExact {
        /// The path to the file to read
        file: Arc<Path>,

        /// Destination address to read at most `len` bytes into
        buf: NonNull<[u8]>,

        /// Offset into the file that data should be read from
        offset: u64,

        /// The channel on which to send result messages to
        sender: Arc<dyn ISender>,

        /// Opaque
        opaque: [u64; 4],
    },
    LoadFile {
        /// The path to the file to read
        file: Arc<Path>,

        /// The channel on which to send result messages to
        sender: Arc<dyn ISender>,

        /// Opaque
        opaque: [u64; 4],
    },
}

// Safety: It's the caller's responsibility to ensure the 'dst' pointer is valid to be sent across
//         threads, and to respect the temporary ownership transfer.
unsafe impl Send for AsyncRequest {}
