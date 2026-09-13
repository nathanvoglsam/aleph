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

use mg::async_resource_loader::buffer_upload_range::BufferUploadRange;
use mg::async_resource_loader::{
    AllocateRangeError, AsyncResourceLoader, BufferLoadHandle, RetireError,
};

use crate::render::async_loader::resources::async_loader_requests::ResourceLoadHandle;

/// Utility wrapper over [`AsyncResourceLoader::allocate_range_for_buffer_load`] that handles
/// wait and retry logic for allocating upload ranges.
///
/// This function will try to allocate an upload range from the loader. If the first attempt
/// fails with 'NotEnoughMemory' then the function will block and wait using
/// [`AsyncResourceLoader::wait_all_submissions`]. This will retire all in-flight copy commands
/// which may make more memory available to the uploader. If we fail for a second time with
/// 'NotEnoughUploadMemory' then we fail as retiring any in-flight commands should have freed
/// up enough memory for any one request.
pub fn try_allocate_buffer_range_for(
    loader: &AsyncResourceLoader<ResourceLoadHandle>,
    handle: BufferLoadHandle,
) -> io::Result<Option<BufferUploadRange<'_, ResourceLoadHandle>>> {
    let mut attempt = 0;
    'alloc: loop {
        match loader.allocate_range_for_buffer_load(handle, u64::MAX) {
            Ok(v) => break 'alloc Ok(Some(v)),
            Err(AllocateRangeError::UploadComplete) => {
                // In this case all our uploads are complete, and we should retire the upload if
                // it hasn't been already.
                return Ok(None);
            }
            Err(AllocateRangeError::OutstandingRange) => {
                // Our internal state machine should mean it's impossible to
                // hit this case. We never ask for more data before
                // submitting an existing block.
                log::error!("Upload memory allocation failed with 'OutstandingRange' error.");
                return Err(io::Error::from(io::ErrorKind::Other));
            }
            Err(AllocateRangeError::NotEnoughUploadMemory) => {
                // If we run out of memory in the internal pool then we must
                // wait for all the in-flight GPU work to finish to free up
                // space for the upload.
                //
                // Successfully waiting loops the alloc loop which will
                // retry getting the range. If it fails a second time then
                // we bail as we're stuck.
                if attempt == 0 {
                    match loader.wait_all_submissions() {
                        Ok(_) => {
                            attempt += 1;
                            continue 'alloc;
                        }
                        Err(RetireError::DeviceLost) => {
                            log::error!("GPU device lost.");
                            return Err(io::Error::from(io::ErrorKind::Other));
                        }
                        Err(RetireError::RendererDisconnected) => {
                            log::error!("Renderer disconnected.");
                            return Err(io::Error::from(io::ErrorKind::ConnectionAborted));
                        }
                        Err(RetireError::WaitFailure) => {
                            panic!("Failed to wait on the GPU.");
                        }
                    }
                } else {
                    log::error!("Not enough memory in async loader pool.");
                    return Err(io::Error::from(io::ErrorKind::OutOfMemory));
                }
            }
            Err(AllocateRangeError::LoadHandleInvalid) => {
                // We treat this the same as 'UploadComplete' as it largely means the same
                // thing. The upload with the given handle is complete so we should make
                // sure we have actually retired the request.
                return Ok(None);
            }
            Err(AllocateRangeError::DeviceLost) => {
                log::error!("GPU device lost.");
                return Err(io::Error::from(io::ErrorKind::Other));
            }
            Err(AllocateRangeError::WaitFailure) => {
                panic!("Failed to wait on the GPU.");
            }
            Err(AllocateRangeError::RendererDisconnected) => {
                log::error!("Renderer disconnected.");
                return Err(io::Error::from(io::ErrorKind::ConnectionAborted));
            }
        }
    }
}
