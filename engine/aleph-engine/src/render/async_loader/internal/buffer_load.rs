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

use aleph_vfs::IRouter;
use aleph_vfs::async_io::AsyncIoSender;
use aleph_vfs::path::VPathBuf;
use mg::async_resource_loader::{AsyncResourceLoader, FlushError};

use crate::core::async_io::context::IoContext;
use crate::core::async_io::task::{ITaskFactory, TaskFactory};
use crate::render::async_loader::internal::utils::try_allocate_buffer_range_for;
use crate::render::async_loader::resources::async_loader_requests::ResourceLoadHandle;

pub struct BufferLoadPayload {
    /// Cookie tag to correlate messages with the initial request.
    pub cookie: ResourceLoadHandle,

    /// The vfs path of the file to open and read from.
    pub path: VPathBuf,

    /// Offset into the file to start reading data from.
    pub offset: u64,

    /// The size of the buffer to create, and the number of bytes to read from the file at the
    /// given offset.
    pub size: u64,
}

#[derive(Clone)]
pub struct BufferLoadTask {
    vfs: Arc<dyn IRouter>,
}

impl BufferLoadTask {
    pub fn new(vfs: Arc<dyn IRouter>) -> Arc<dyn ITaskFactory> {
        Arc::new(Self { vfs })
    }
}

impl TaskFactory for BufferLoadTask {
    type Context = BufferLoadTask;
    type Payload = BufferLoadPayload;

    fn context(&self) -> Self::Context {
        self.clone()
    }

    async fn task(
        ctx: Self::Context,
        io: IoContext<'_, AsyncIoSender>,
        loader: &AsyncResourceLoader<ResourceLoadHandle>,
        msg: Self::Payload,
    ) -> io::Result<()> {
        let handle = match loader.begin_buffer_load(msg.size, msg.cookie) {
            Ok(v) => v,
            Err(e) => {
                // If we failed to create the GPU resource then we should remove
                // the upload from the working set and try and process another
                // upload instead.
                //
                // The magnesium loader handles notifying the renderer. We just
                // log a message.
                log::error!("Failed to create GPU resource with error '{e:?}'.");
                return Err(io::Error::from(io::ErrorKind::Other));
            }
        };

        let path = msg.path.as_path();
        let result = match io.open_file(ctx.vfs.as_ref(), path) {
            Ok(v) => v,
            Err(e) => {
                log::error!("Failed to open file '{path}' with error '{e:?}'.");
                loader.fail_buffer_load(handle);
                return Err(e);
            }
        };
        let file = match result.await {
            Ok(v) => v,
            Err(e) => {
                log::error!("Failed to open file '{path}' with error '{e:?}'.");
                loader.fail_buffer_load(handle);
                return Err(e);
            }
        };

        loop {
            let range = match try_allocate_buffer_range_for(loader, handle) {
                Ok(None) => return Ok(()),
                Ok(Some(r)) => r,
                Err(e) => {
                    log::error!("Error: {e:?}");
                    loader.fail_buffer_load(handle);
                    return Err(e);
                }
            };

            let mut buffer = range.as_ptr();
            let mut file_offset = msg.offset;

            while !buffer.is_empty() {
                // Safety: the safety issues are related to our use of range.as_ptr(). we never
                //         touch the upload memory here and never issue overlapping requests so we
                //         should be golden.
                //
                // we also structure our executor and error conditions in a way where any failure or
                // panic that could lead to the buffer being freed from underneath the in-flight
                // request is promoted to an abort before it can cause UB.
                let future =
                    match unsafe { io.read_file_at(file.as_ref(), range.as_ptr(), file_offset) } {
                        Ok(v) => v,
                        Err(e) => {
                            // The only way the read_file_at call can fail is if the async reader
                            // system has shut down.
                            log::error!("Async IO system has disconnected.");
                            loader.fail_buffer_load(handle);
                            return Err(e);
                        }
                    };

                match future.await {
                    Ok(bytes_transferred) => {
                        file_offset = file_offset + bytes_transferred as u64;
                        buffer = unsafe {
                            let remaining = buffer.len() - bytes_transferred;
                            NonNull::slice_from_raw_parts(
                                buffer.byte_add(buffer.len()).cast(),
                                remaining,
                            )
                        };
                    }
                    Err(err) => {
                        // There are no in-flight IO requests on this request so it is safe to fail
                        // it.
                        log::error!("Failed to read file '{path}' with error '{err:?}'.");
                        loader.fail_buffer_load(handle);
                        return Err(err);
                    }
                }
            }

            match range.submit() {
                Ok(_) => {}
                Err(e) => match e {
                    FlushError::CommandRecordingFailure => {
                        log::error!("Error: {e:?}");
                        loader.fail_buffer_load(handle);
                        return Err(io::Error::from(io::ErrorKind::Other));
                    }
                    FlushError::DeviceLost => {
                        log::error!("Error: {e:?}");
                        loader.fail_buffer_load(handle);
                        return Err(io::Error::from(io::ErrorKind::Other));
                    }
                    FlushError::WaitFailure => {
                        log::error!("Error: {e:?}");
                        abort_unwind(|| panic!("Error: {e:?}"))
                    }
                    FlushError::RendererDisconnected => {
                        log::error!("Error: {e:?}");
                        loader.fail_buffer_load(handle);
                        return Err(io::Error::from(io::ErrorKind::ConnectionAborted));
                    }
                },
            }
        }
    }
}

extern "C" fn abort_unwind<F: FnOnce() -> R, R>(f: F) -> R {
    f()
}
