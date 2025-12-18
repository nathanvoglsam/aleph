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

use std::ops::{Deref, DerefMut, Range};
use std::ptr::NonNull;

use aleph_alloc::BVec;
use aleph_alloc::offset_allocator::Allocation;

use crate::async_resource_loader::{AsyncResourceLoader, FlushError, TextureLoadHandle};
use crate::internal::async_resource_loader::MgAsyncLdrSystem;

pub struct TextureUploadRange<'a, C: Send + 'static> {
    /// Back-reference to the resource loader this range is allocated from.
    pub(crate) loader: &'a AsyncResourceLoader<C>,

    /// The resource load request this range is associated with.
    pub(crate) request: TextureLoadHandle,

    /// Flag that gets set on the first upload range. Captures whether this upload range will need
    /// a discard operation performed on the texture before it is copied in to for the first time.
    pub(crate) needs_discard: bool,

    /// An array of [`WantedTextureRows`] that marks up what data the caller should provide and
    /// where in the upload range to write it.
    pub(crate) wanted: BVec<WantedTextureRows, MgAsyncLdrSystem>,
}

impl<'a, C: Send + 'static> Drop for TextureUploadRange<'a, C> {
    fn drop(&mut self) {
        for wanted in self.wanted.drain(..) {
            // The 'allocation' field doubles as a canary for whether the upload range has been
            // submitted. This prevents us from double-freeing the allocation.
            //
            // We require the drop implementation to handle a user asking for an upload range but
            // never submitting it before it gets dropped. We need to return the memory and reset
            // the request to a valid state
            if !wanted.allocation.is_fail() {
                // Deallocate the upload range.
                self.loader
                    .upload_memory_manager
                    .free_upload_range(wanted.allocation);

                // "deallocate" the upload range from the soruce request too, if it's still valid.
                // The handle may be invalid in the event of a canceled request, in which case we
                // just do nothing.
                match self
                    .loader
                    .request_states
                    .borrow_mut()
                    .textures
                    .get_mut(self.request)
                {
                    None => {
                        // Intentionally do nothing
                    }
                    Some(load) => {
                        let level = &mut load.levels[wanted.level as usize];
                        level.rows_allocated = level.rows_submitted;
                    }
                }
            }
        }
    }
}

impl<'a, C: Send + 'static> TextureUploadRange<'a, C> {
    /// Returns the total number of bytes owned by this upload range across all the wanted mip level
    /// members.
    pub fn total_bytes(&self) -> usize {
        self.wanted.iter().map(|v| v.data.len()).sum()
    }

    /// Returns an array of [`WantedTextureRows`] that each own an upload range that should be
    /// filled with data. Each element describes the wanted data for one mip level.
    ///
    /// The upload range may desire data for multiple mip levels at once, so we need an array of
    /// upload blocks rather than a single one like for buffers.
    #[inline]
    pub fn wanted_levels(&self) -> &[WantedTextureRows] {
        self.wanted.as_slice()
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
            if let Some(load) = request_states.textures.get_mut(self.request) {
                // Update the request to reflect that we've submitted additional bytes to the queue.
                for wanted in &self.wanted {
                    let level = &mut load.levels[wanted.level as usize];
                    level.rows_submitted = level.rows_allocated;
                }

                // Also do the whole submit to the queue thing.
                loader
                    .queue_manager
                    .submit_texture_upload_range(self, load.is_complete());
            }
        }

        loader.maybe_flush()
    }
}

/// Describes the desired range of rows to be provided for a single mip level of a texture upload.
///
/// Closes over an upload memory range, as well as a description of the desired range of rows that
/// should be written into the upload range before calling [`TextureUploadRange::submit`].
///
/// The expected row stride will be aligned to the alignment requested when
/// [`AsyncResourceLoader::allocate_range_for_texture_load`] is called. It is the caller's
/// responsibility to keep track of this and write row data to the correct offsets within the
/// upload buffer.
pub struct WantedTextureRows {
    /// The allocation handle that was created to allocate the 'data' range of our upload buffer.
    pub(crate) allocation: Allocation,

    /// The mip level we want data to be provided into 'data' for.
    pub(crate) level: u32,

    /// The range of rows we want data to be copied into 'data'.
    pub(crate) wanted_rows: Range<u32>,

    /// An associated buffer -> texture copy region that will correctly upload the rows this struct
    /// encapsulates.
    pub(crate) region: rhi::BufferToTextureCopyRegion,

    /// Target buffer that we should write the requested rows into. The first row should be written
    /// starting at `data[0]`.
    pub(crate) data: NonNull<[u8]>,
}

impl Deref for WantedTextureRows {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl DerefMut for WantedTextureRows {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_slice_mut()
    }
}

impl WantedTextureRows {
    /// The mip level that data should be provided for
    pub const fn level(&self) -> u32 {
        self.level
    }

    /// The number of rows that should be copied into the upload block.
    pub const fn num_wanted_rows(&self) -> u32 {
        self.wanted_rows.end - self.wanted_rows.start
    }

    /// Range that describes the `begin..end` range of rows that should be provided.
    pub const fn wanted_rows(&self) -> Range<u32> {
        self.wanted_rows.start..self.wanted_rows.end
    }

    /// The extent of the dst mip level that data should be provided for.
    ///
    /// Typically, you should only need [`WantedTextureRows::num_wanted_rows`] and
    /// [`WantedTextureRows::wanted_rows`] to derive what data should be uploaded. This is provided
    /// for convenience.
    pub const fn wanted_extent(&self) -> rhi::Extent3D {
        self.region.dst.extent
    }

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
}
