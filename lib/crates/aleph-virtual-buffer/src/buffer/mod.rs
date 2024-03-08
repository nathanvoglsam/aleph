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

mod implementation;

use std::ops::{Deref, DerefMut, Range};
use std::slice::{from_raw_parts, from_raw_parts_mut};

///
/// An abstraction over an owned region of address space that can be committed and released
///
pub struct VirtualBuffer {
    data: *mut u8,
    len: usize,
}

impl VirtualBuffer {
    ///
    /// Reserves the given number of pages somewhere in the address space.
    ///
    pub fn reserve(pages: usize) -> std::io::Result<VirtualBuffer> {
        unsafe { implementation::reserve_virtual_buffer(pages) }
    }

    ///
    /// Reserves the number of bytes somewhere in the address space.
    ///
    /// # Warning
    ///
    /// This will round the number of bytes requested up to the next page boundary. As such
    /// `VirtualBuffer::len` may return a value greater than `bytes`.
    ///
    pub fn reserve_bytes(bytes: usize) -> std::io::Result<VirtualBuffer> {
        // Calculate the number of pages we need to store the given number of bytes
        let (_, pages) = Self::page_range_for_offset_len(0, bytes);

        // Reserve the required number of pages
        Self::reserve(pages)
    }

    ///
    /// Commits all pages that intersect the range of *bytes* inside the virtual buffer
    ///
    pub fn commit(&self, range: Range<usize>) -> std::io::Result<()> {
        if range.end > self.len() {
            Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Range out of bounds",
            ))
        } else {
            let (offset, pages) = Self::page_range_for_byte_range(range);

            let ptr = unsafe { self.data.add(offset) };
            unsafe { implementation::commit_virtual_address_range(ptr, pages) }
        }
    }

    ///
    /// Releases all pages (but does not un-reserve them)
    ///
    /// # SAFETY INFORMATION
    ///
    /// Typically this kind of operation would by wildly unsafe as in C or C++ we could release
    /// the memory while someone else has a pointer into it, leaving behind a dangling pointer.
    ///
    /// Thankfully in rust the ownership and lifetime rules can prevent this from happening by
    /// requiring exclusive access to the virtual buffer. There can be no outstanding borrows to
    /// the underlying memory for a call to this function to compile.
    ///
    pub fn release(&mut self, range: Range<usize>) -> std::io::Result<()> {
        if range.end > self.len() {
            Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Range out of bounds",
            ))
        } else {
            let (offset, pages) = Self::page_range_for_byte_range(range);

            let ptr = unsafe { self.data.add(offset) };
            unsafe { implementation::release_virtual_address_range(ptr, pages) }
        }
    }

    ///
    /// Returns the pointer to base address for the buffer.
    ///
    /// # SAFETY
    ///
    /// This function itself is safe, but using the pointer can be very unsafe and should be done
    /// with care.
    ///
    /// Because the address space is only reserved by default it will not always be safe to
    /// dereference every address within the buffer. Only regions that are committed can be accessed
    /// safely.
    ///
    /// This must be upheld by the caller, hence returning a pointer rather than a slice. We wash
    /// our hands of the safety problem from here after.
    ///
    pub fn data(&self) -> *mut u8 {
        self.data
    }

    ///
    /// Returns the length of the buffer
    ///
    pub fn len(&self) -> usize {
        self.len
    }

    ///
    /// Returns whether the len is 0
    ///
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    ///
    /// Commits the entire address range and emits a new-type wrapper that can allow for more
    /// powerful operations to be done entirely in safe code
    ///
    pub fn commit_whole(self) -> std::io::Result<CommittedVirtualBuffer> {
        self.commit(0..self.len)?;
        Ok(CommittedVirtualBuffer(self))
    }

    ///
    /// Returns a slice over the whole address range
    ///
    /// # Safety
    ///
    /// The entire address range is not guaranteed to be committed so creating a slice of it is
    /// not safe as it could lead to safe code de-referencing un-commited memory.
    ///
    /// Commit all memory before using, or check if the platform requires it with
    /// `Self::requires_committing`.
    ///
    pub unsafe fn as_slice(&self) -> &[u8] {
        from_raw_parts(self.data, self.len)
    }

    ///
    /// Returns a slice over the whole address range
    ///
    /// # Safety
    ///
    /// The entire address range is not guaranteed to be committed so creating a slice of it is
    /// not safe as it could lead to safe code de-referencing un-commited memory.
    ///
    /// Commit all memory before using, or check if the platform requires it with
    /// `Self::requires_committing`.
    ///
    pub unsafe fn as_slice_mut(&mut self) -> &mut [u8] {
        from_raw_parts_mut(self.data, self.len)
    }

    /// Returns a pointer to the base address of the virtual address range.
    pub fn as_ptr(&self) -> *const u8 {
        self.data as *const u8
    }

    /// Returns a pointer to the base address of the virtual address range.
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.data
    }

    ///
    /// Returns the page size for the platform
    ///
    pub const fn page_size() -> usize {
        implementation::page_size()
    }
}

impl Drop for VirtualBuffer {
    fn drop(&mut self) {
        unsafe {
            implementation::free_virtual_buffer(self.data, self.len / Self::page_size()).unwrap()
        }
    }
}

impl VirtualBuffer {
    /// This function will calculate the number of pages for any pages that are in
    /// the range specified by the given byte range.
    const fn page_range_for_byte_range(range: Range<usize>) -> (usize, usize) {
        let offset = range.start;
        let len = range.end - offset;
        Self::page_range_for_offset_len(offset, len)
    }

    /// This function will calculate the number of pages for any pages that are in
    /// the range specified by the given byte range.
    const fn page_range_for_offset_len(offset: usize, len: usize) -> (usize, usize) {
        let mask = implementation::page_size() - 1;

        // Find the base address for the first page the range intersects
        let base = offset & !mask;

        // Find the number of pages the address intersects
        let extended_len = (offset + len) - base; // Get the size of the range in bytes from the new base
        let pages = extended_len.div_ceil(implementation::page_size());

        (base, pages)
    }
}

unsafe impl Send for VirtualBuffer {}
unsafe impl Sync for VirtualBuffer {}

///
/// A new-type wrapper around `VirtualBuffer` that requires that the entire buffer is committed.
///
/// By requiring the whole virtual address range to be committed we can safely treat the entire
/// buffer as a contiguous slice.
///
pub struct CommittedVirtualBuffer(VirtualBuffer);

impl Deref for CommittedVirtualBuffer {
    type Target = VirtualBuffer;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CommittedVirtualBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
