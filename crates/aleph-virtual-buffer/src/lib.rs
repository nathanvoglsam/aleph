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

#[cfg(test)]
mod tests;

mod implementation;

use std::ops::Range;
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
    /// Pages are assumed to always be 4096 bytes.
    ///
    pub fn reserve(pages: usize) -> Result<VirtualBuffer, ()> {
        unsafe { implementation::reserve_virtual_buffer(pages) }
    }

    ///
    /// Commits all pages that intersect the range of *bytes* inside the virtual buffer
    ///
    /// # SAFETY INFORMATION
    ///
    /// This is safe to use behind a shared reference as we can only ever change the state of memory
    ///
    ///
    pub fn commit(&self, range: Range<usize>) -> Result<(), ()> {
        let (base, pages) = Self::resolve_range(self.data, range);
        unsafe { implementation::commit_virtual_address_range(base, pages) }
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
    pub fn release(&mut self, range: Range<usize>) -> Result<(), ()> {
        let (base, pages) = Self::resolve_range(self.data, range);
        unsafe { implementation::release_virtual_address_range(base, pages) }
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
    /// Returns a slice over the whole address range
    ///
    /// # SAFETY
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
    /// # SAFETY
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

    ///
    /// This function returns whether the current platform requires committing pages explicitly
    /// before they can be used.
    ///
    /// When this returns false it can be assumed that the entire address space range is valid and
    /// can be de-referenced.
    ///
    pub const fn requires_committing() -> bool {
        implementation::requires_committing()
    }
}

impl Drop for VirtualBuffer {
    fn drop(&mut self) {
        unsafe { implementation::free_virtual_buffer(self.data, self.len / 4096).unwrap() }
    }
}

impl VirtualBuffer {
    #[inline]
    fn resolve_range(data: *mut u8, range: Range<usize>) -> (*mut u8, usize) {
        // Destructure the range as it's not a copy type
        let (start, end) = (range.start, range.end);

        // Find the base address for the first page the range intersects
        let base = start & !4095;

        // Find the number of pages the address intersects
        let pages = end - base; // Get the size of the range in bytes from the new base
        let pages = pages + 4095 & !4095; // Round up to the next page size
        let pages = pages / 4096; // Division should optimize to shift

        unsafe { (data.add(base), pages) }
    }
}
