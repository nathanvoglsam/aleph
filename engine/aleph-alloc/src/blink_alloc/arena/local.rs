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
use super::*;

with_cursor!(Cell<*mut u8>);

/// Thread-local arena allocator.
pub struct ArenaLocal {
    root: Cell<Option<NonNull<ChunkHeader>>>,
    min_chunk_size: Cell<usize>,
}

/// It is safe to send `ArenaLocal` between threads.
unsafe impl Send for ArenaLocal {}

impl Drop for ArenaLocal {
    #[inline(always)]
    fn drop(&mut self) {
        debug_assert!(
            self.root.get().is_none(),
            "Owner must reset `ArenaLocal` with `keep_last` set to `false` before drop"
        );
    }
}

impl ArenaLocal {
    #[inline(always)]
    pub const fn new() -> Self {
        ArenaLocal {
            root: Cell::new(None),
            min_chunk_size: Cell::new(CHUNK_START_SIZE),
        }
    }

    #[inline(always)]
    pub const fn with_chunk_size(min_chunk_size: usize) -> Self {
        ArenaLocal {
            root: Cell::new(None),
            min_chunk_size: Cell::new(min_chunk_size),
        }
    }

    #[inline(always)]
    pub fn last_chunk_size(&self) -> usize {
        match self.root.get() {
            None => 0,
            Some(root) => {
                // Safety: `root` is a valid pointer to chunk allocation.
                unsafe { root.as_ref().cap() }
            }
        }
    }

    #[inline(always)]
    pub unsafe fn alloc_fast(&self, layout: Layout) -> Option<NonNull<[u8]>> {
        if let Some(root) = self.root.get() {
            return unsafe { ChunkHeader::alloc(root, layout) };
        }
        None
    }

    #[inline(always)]
    pub unsafe fn alloc_slow(
        &self,
        layout: Layout,
        allocator: impl Allocator,
    ) -> Result<NonNull<[u8]>, AllocError> {
        unsafe { alloc_slow(&self.root, self.min_chunk_size.get(), layout, allocator) }
    }

    #[inline(always)]
    pub unsafe fn resize_fast(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Option<NonNull<[u8]>> {
        if let Some(root) = self.root.get() {
            return unsafe { ChunkHeader::resize(root, ptr, old_layout, new_layout) };
        }
        None
    }

    #[inline(always)]
    pub unsafe fn resize_slow(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
        allocator: impl Allocator,
    ) -> Result<NonNull<[u8]>, AllocError> {
        unsafe {
            resize_slow(
                &self.root,
                self.min_chunk_size.get(),
                ptr,
                old_layout,
                new_layout,
                allocator,
            )
        }
    }

    #[inline(always)]
    pub unsafe fn dealloc(&self, ptr: NonNull<u8>, size: usize) {
        unsafe { dealloc(self.root.get(), ptr, size) }
    }

    #[inline(always)]
    pub unsafe fn reset(&mut self, keep_last: bool, allocator: impl Allocator) {
        unsafe { reset(&self.root, keep_last, allocator) }
    }

    #[inline(always)]
    pub unsafe fn reset_unchecked(&self, keep_last: bool, allocator: impl Allocator) {
        unsafe { reset(&self.root, keep_last, allocator) }
    }

    #[inline(always)]
    pub fn reset_leak(&mut self, keep_last: bool) {
        reset_leak(&self.root, keep_last)
    }
}
