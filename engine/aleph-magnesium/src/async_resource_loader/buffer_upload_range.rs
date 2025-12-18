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

use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;

use aleph_alloc::offset_allocator::Allocation;

use crate::async_resource_loader::{AsyncResourceLoader, BufferLoadHandle, FlushError};

pub struct BufferUploadRange<'a, C: Send + 'static> {
    /// Back-reference to the resource loader this range is allocated from.
    pub(crate) loader: &'a AsyncResourceLoader<C>,

    /// The resource load request this range is associated with.
    pub(crate) request: BufferLoadHandle,

    /// Pointer to the sub-allocated data this upload range owns.
    pub(crate) data: NonNull<[u8]>,

    /// The allocation handle that was created to allocate the 'data' range of our upload buffer.
    pub(crate) allocation: Allocation,

    /// Offset into the destination resource that this upload range should be copied into once
    /// it is submitted.
    pub(crate) dst_offset: u64,
}

impl<'a, C: Send + 'static> Drop for BufferUploadRange<'a, C> {
    fn drop(&mut self) {
        // The 'allocation' field doubles as a canary for whether the upload range has been
        // submitted. This prevents us from double-freeing the allocation.
        //
        // We require the drop implementation to handle a user asking for an upload range but
        // never submitting it before it gets dropped. We need to return the memory and reset the
        // request to a valid state
        if !self.allocation.is_fail() {
            // Deallocate the upload range.
            self.loader
                .upload_memory_manager
                .free_upload_range(self.allocation);

            // "deallocate" the upload range from the soruce request too, if it's still valid. The
            // handle may be invalid in the event of a canceled request, in which case we just do
            // nothing.
            match self
                .loader
                .request_states
                .borrow_mut()
                .buffers
                .get_mut(self.request)
            {
                None => {
                    // Intentionally do nothing
                }
                Some(load) => load.bytes_allocated = load.bytes_submitted,
            }
        }
    }
}

impl<'a, C: Send + 'static> Deref for BufferUploadRange<'a, C> {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<'a, C: Send + 'static> DerefMut for BufferUploadRange<'a, C> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_slice_mut()
    }
}

impl<'a, C: Send + 'static> BufferUploadRange<'a, C> {
    /// Get a slice over the upload memory owned by self.
    ///
    /// # Performance
    ///
    /// This is almost certainly write-combine memory from an upload heap. Reading this memory is
    /// highly discouraged, as it is very expensive. Usage should be restricted to sequential
    /// writes only.
    pub const fn as_slice(&self) -> &[u8] {
        unsafe { self.data.as_ref() }
    }

    /// Get a slice over the upload memory owned by self.
    ///
    /// # Performance
    ///
    /// This is almost certainly write-combine memory from an upload heap. Reading this memory is
    /// highly discouraged, as it is very expensive. Usage should be restricted to sequential
    /// writes only.
    pub const fn as_slice_mut(&mut self) -> &mut [u8] {
        unsafe { self.data.as_mut() }
    }

    /// Get a slice over the upload memory owned by self, as a [`NonNull`] instead of a slice.
    ///
    /// # Performance
    ///
    /// This is almost certainly write-combine memory from an upload heap. Reading this memory is
    /// highly discouraged, as it is very expensive. Usage should be restricted to sequential
    /// writes only.
    pub const fn as_ptr(&self) -> NonNull<[u8]> {
        self.data
    }

    /// Submit the upload range to the loader it was allocated from.
    ///
    /// This should be called once the caller has fully written all the requested data into the
    /// upload memory range. The block is then placed onto the queue that the loader pulls from to
    /// record and submit upload work to the GPU queue.
    ///
    /// This function will also call [`AsyncResourceLoader::maybe_flush`], returning any errors that
    /// bubble up. We call 'maybe_flush' to prevent the queue getting too full, either starving
    /// the managed upload memory or starving the GPU of upload work for no reason.
    pub fn submit(self) -> Result<(), FlushError> {
        let loader = self.loader;

        {
            let mut request_states = loader.request_states.borrow_mut();

            // The request handle may be invalid if the request was canceled. If so we just do
            // nothing and drop the upload range.
            if let Some(load) = request_states.buffers.get_mut(self.request) {
                // Update the request to reflect that we've submitted additional bytes to the queue.
                load.bytes_submitted = load.bytes_allocated;

                // Also do the whole submit to the queue thing.
                loader
                    .queue_manager
                    .submit_buffer_upload_range(self, load.is_complete());
            }
        }

        loader.maybe_flush()
    }
}
