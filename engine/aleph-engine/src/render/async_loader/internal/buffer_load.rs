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

use aleph_vfs::IRouter;
use aleph_vfs::path::VPathBuf;
use mg::async_resource_loader::{BufferLoadResult, FlushError};
use mg::resource::buffer::BufferHandle;

use crate::core::async_io::context::IoContext;
use crate::render::async_loader::internal::utils::try_allocate_buffer_range_for;

pub async fn load_buffer_from_file(
    vfs: &dyn IRouter,
    io: &IoContext<'_>,
    path: VPathBuf,
    offset: u64,
    size: u64,
) -> io::Result<BufferHandle> {
    let (sender, receiver) = kanal::bounded_async(1);
    let loader = io.loader();
    let handle = match loader.begin_buffer_load(sender.to_sync(), size, 0) {
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

    let path = path.as_path();
    let file = match io.open_file(vfs, path).await {
        Ok(v) => v,
        Err(e) => {
            log::error!("Failed to open file '{path}' with error '{e:?}'.");
            loader.fail_buffer_load(handle);
            return Err(e);
        }
    };

    let mut file_offset = offset;
    loop {
        let range = match try_allocate_buffer_range_for(loader, handle) {
            Ok(None) => break,
            Ok(Some(r)) => r,
            Err(e) => {
                log::error!("Error: {e:?}");
                loader.fail_buffer_load(handle);
                return Err(e);
            }
        };

        let mut buffer = range.as_ptr();

        while !buffer.is_empty() {
            // Safety: the safety issues are related to our use of range.as_ptr(). we never
            //         touch the upload memory here and never issue overlapping requests so we
            //         should be golden.
            //
            // we also structure our executor and error conditions in a way where any failure or
            // panic that could lead to the buffer being freed from underneath the in-flight
            // request is promoted to an abort before it can cause UB.
            let result = unsafe { io.read_file_at(file.as_ref(), buffer, file_offset).await };
            match result {
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

    match receiver.recv().await {
        Ok(v) => match v.0 {
            Ok(v) => Ok(v),
            Err(_) => {
                log::error!("'load_buffer_from_file' failed to create GPU on renderer thread.");
                Err(io::Error::from(io::ErrorKind::Other))
            }
        },
        Err(e) => {
            log::error!("The load request was lost '{e:?}'.");
            Err(io::Error::from(io::ErrorKind::Other))
        }
    }
}

pub fn issue_load_buffer_from_data(
    io: &IoContext<'_>,
    sender: kanal::Sender<BufferLoadResult<u64>>,
    cookie: u64,
    data: &[u8],
) -> io::Result<()> {
    let loader = io.loader();
    let handle = match loader.begin_buffer_load(sender, data.len() as u64, cookie) {
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

    let mut src = data;
    let result = loop {
        let mut range = match try_allocate_buffer_range_for(loader, handle) {
            Ok(None) => break Ok(()),
            Ok(Some(r)) => r,
            Err(e) => {
                log::error!("Failed to allocate buffer range: {e:?}");
                break Err(e);
            }
        };

        let buffer = range.as_slice_mut();
        let (read, rest) = src.split_at(buffer.len());
        buffer.copy_from_slice(read);
        src = rest;

        match range.submit() {
            Ok(_) => {}
            Err(e) => match e {
                FlushError::CommandRecordingFailure => {
                    log::error!("Range submission error: {e:?}");
                    break Err(io::Error::from(io::ErrorKind::Other));
                }
                FlushError::DeviceLost => {
                    log::error!("Range submission error: {e:?}");
                    break Err(io::Error::from(io::ErrorKind::Other));
                }
                FlushError::WaitFailure => {
                    log::error!("Range submission error: {e:?}");
                    abort_unwind(|| panic!("Error: {e:?}"))
                }
                FlushError::RendererDisconnected => {
                    log::error!("Range submission error: {e:?}");
                    break Err(io::Error::from(io::ErrorKind::ConnectionAborted));
                }
            },
        }
    };

    result.inspect_err(|_| loader.fail_buffer_load(handle))
}

extern "C" fn abort_unwind<F: FnOnce() -> R, R>(f: F) -> R {
    f()
}
