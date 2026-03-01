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

use std::alloc::{Layout, LayoutError, handle_alloc_error};
use std::cmp::Ordering;
use std::ptr::NonNull;

use aleph_alloc::alloc::{Allocator, Global};

/// Basic 'stretchy buffer' data structure. Provides an aligned buffer allocated in multiples of
/// some element type.
///
/// Does not manage object life times. This just manages a growable byte buffer.
pub struct Column<A: Allocator = Global> {
    /// Pointer to the buffer that the column manages.
    ptr: NonNull<u8>,

    /// The layout of a single element in the column. The API allocates in multiples of this layout.
    element_layout: Layout,

    /// The current number of elements the column has allocated space for.
    count: usize,

    /// The allocator adaptor memory should be allocated from.
    allocator: A,
}

impl<A: Allocator> Column<A> {
    /// Constructs a new [`Column`] that will allocate in blocks of 'element_layout'.
    ///
    /// Takes an allocator parameter as the source of our memory allocations.
    pub const fn new_in(element_layout: Layout, allocator: A) -> Self {
        Self {
            ptr: NonNull::dangling(),
            element_layout,
            count: 0,
            allocator,
        }
    }

    /// Gets the column's buffer pointer, returning [`None`] if the column is currently empty.
    pub const fn get(&self) -> Option<NonNull<u8>> {
        if self.count > 0 { Some(self.ptr) } else { None }
    }

    /// Get a pointer to the `i'th` element of the column, returning [`None`] if the index is out
    /// of bounds.
    pub const fn get_at_index(&self, i: usize) -> Option<NonNull<u8>> {
        // Out-of-bounds index yields `None`. This also handles the self.count == 0 case because
        // all usize values are >= 0.
        if i >= self.count {
            return None;
        }

        // Safety: The implementation guarantees self.ptr is a valid allocation for self.count
        //         elements. All we need to check is that the index is in bounds, which we do above.
        unsafe { Some(self.ptr.byte_add(i * self.element_layout.size())) }
    }

    /// Resize the column so it has space for 'new_count' elements.
    ///
    /// This is a wrapper over [`Column::grow_to_fit`] and [`Column::shrink_to_fit`] that selects
    /// the appropriate call based on 'new_count' in relation to 'self.count'.
    pub fn resize(&mut self, new_count: usize) {
        match new_count.cmp(&self.count) {
            Ordering::Less => self.shrink_to_fit(new_count),
            Ordering::Equal => {}
            Ordering::Greater => self.grow_to_fit(new_count),
        }
    }

    /// Request the column to grow to provide enough space for 'new_count' elements.
    ///
    /// If 'new_count' is <= 'count' this will do nothing.
    pub fn grow_to_fit(&mut self, new_count: usize) {
        // If new_count does not actually increase the size of the column then we do nothing.
        if self.count >= new_count {
            return;
        }

        let new_layout = self.layout_for_count(new_count).unwrap();
        let buffer = if self.count == 0 {
            match self.allocator.allocate_zeroed(new_layout) {
                Ok(v) => v,
                Err(_) => handle_alloc_error(new_layout),
            }
        } else {
            let old_layout = self.layout_for_count(self.count).unwrap();
            unsafe {
                match self.allocator.grow_zeroed(self.ptr, old_layout, new_layout) {
                    Ok(v) => v,
                    Err(_) => handle_alloc_error(new_layout),
                }
            }
        };
        self.count = new_count;
        self.ptr = buffer.cast();
    }

    /// Request the column shrink to only provide enough space for 'new_count' elements.
    ///
    /// If 'new_count' is >= 'count' this will do nothing.
    pub fn shrink_to_fit(&mut self, new_count: usize) {
        // If new_count would not actually shrink the column then we do nothing.
        if self.count <= new_count {
            return;
        }

        let old_layout = self.layout_for_count(self.count).unwrap();
        let buffer = if new_count == 0 {
            // There's no situation where we can reach this code with self.count == 0, so we don't
            // need to handle the case where we don't have a valid allocation. We only need to
            // concern ourselves with deallocating an existing allocation.
            unsafe {
                self.allocator.deallocate(self.ptr, old_layout);
            }
            NonNull::dangling()
        } else {
            // There's no situation where we can reach this code with self.count == 0, so we don't
            // need to handle the case where we must create the initial allocation. We only need to
            // concern ourselves with shrinking an existing allocation.
            let new_layout = self.layout_for_count(new_count).unwrap();
            let old_layout = self.layout_for_count(self.count).unwrap();
            let buffer = unsafe {
                match self.allocator.shrink(self.ptr, old_layout, new_layout) {
                    Ok(v) => v,
                    Err(_) => handle_alloc_error(new_layout),
                }
            };
            buffer.cast()
        };

        self.count = new_count;
        self.ptr = buffer;
    }

    /// Gets the 'element_layout' the column was created with.
    pub const fn element_layout(&self) -> Layout {
        self.element_layout
    }

    /// Removes the element at the given 'index' using a 'swap-n-pop' algorithm. Removing any
    /// element other than the last will move the element at the end of the list into the place of
    /// the element to be removed.
    ///
    /// # Warning
    ///
    /// Does not drop the element, that is the caller's responsibility.
    pub fn swap_remove(&mut self, index: usize) {
        assert!(
            index < self.count,
            "Index '{index}' is out of bounds of column."
        );

        let last_index = self.count - 1; // Can't be 0 here so can't underflow.
        if index == last_index {
            // If we swap remove on the last element is just a pop operation.
            self.pop();
        } else {
            // Otherwise we copy the last element over the top of the one we want to remove.
            let remove_offset = index * self.element_layout.size();
            let last_offset = last_index * self.element_layout.size();

            // Safety: Bounds checked by the above assert. Column must always have an allocation
            //         in self.ptr valid for at least self.count elements. Mutable borrow means we
            //         have valid access and this branch is only taken if index != last index so
            //         the copy is always non-overlapping.
            unsafe {
                let remove = self.ptr.add(remove_offset);
                let last = self.ptr.add(last_offset);
                remove.copy_from_nonoverlapping(last, self.element_layout.size());
            };

            // Decrement count to 'pop' the swapped element off the end. It's impossible for count
            // to == 0 if we reach this code so this can't underflow.
            self.count -= 1;
        }
    }

    /// Pop the element at the end of the column off the end.
    ///
    /// # Warning
    ///
    /// Does not drop the element, that is the caller's responsibility as 'Column' doesn't know the
    /// type of what it holds.
    pub fn pop(&mut self) {
        // Saturating sub covers the case of popping from an empty column, which is a perfectly
        // valid no-op operation.
        self.count = self.count.saturating_sub(1);
    }

    /// Project the single element layout on self to provide a [`Layout`] for an array of 'count'
    /// elements.
    ///
    /// If 'count' is too large and produces an allocation size too large this will return an error.
    fn layout_for_count(&self, count: usize) -> Result<Layout, LayoutError> {
        let size = self.element_layout.size();
        let size = size.saturating_mul(count);
        Layout::from_size_align(size, self.element_layout.align())
    }
}

impl<A: Allocator> Drop for Column<A> {
    fn drop(&mut self) {
        if self.count > 0 {
            unsafe {
                self.allocator
                    .deallocate(self.ptr, self.layout_for_count(self.count).unwrap())
            }
        }
    }
}

unsafe impl<A: Allocator + Send> Send for Column<A> {}
unsafe impl<A: Allocator + Sync> Sync for Column<A> {}
