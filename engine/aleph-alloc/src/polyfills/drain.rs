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

use std::alloc::{Allocator, Global};
use std::collections::Bound;
use std::fmt;
use std::iter::FusedIterator;
use std::mem::{self, size_of, ManuallyDrop};
use std::ops::RangeBounds;
use std::ptr::{self, NonNull};
use std::slice::{self};

/// A draining iterator for `Vec<T>`.
///
/// This `struct` is created by [`Vec::drain`].
/// See its documentation for more.
///
/// # Example
///
/// ```
/// use allocator_api2::vec;
///
/// let mut v = vec![0, 1, 2];
/// let iter: vec::Drain<_> = v.drain(..);
/// ```
pub struct Drain<'a, T: 'a, A: Allocator + 'a = Global> {
    /// Index of tail to preserve
    tail_start: usize,
    /// Length of tail
    tail_len: usize,
    /// Current remaining range to remove
    iter: slice::Iter<'a, T>,
    vec: NonNull<Vec<T, A>>,
}

impl<T: fmt::Debug, A: Allocator> fmt::Debug for Drain<'_, T, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Drain").field(&self.iter.as_slice()).finish()
    }
}

impl<'a, T, A: Allocator> Drain<'a, T, A> {
    /// Returns the remaining items of this iterator as a slice.
    ///
    /// # Examples
    ///
    /// ```
    /// use allocator_api2::vec;
    ///
    /// let mut vec = vec!['a', 'b', 'c'];
    /// let mut drain = vec.drain(..);
    /// assert_eq!(drain.as_slice(), &['a', 'b', 'c']);
    /// let _ = drain.next().unwrap();
    /// assert_eq!(drain.as_slice(), &['b', 'c']);
    /// ```
    #[must_use]
    #[inline(always)]
    pub fn as_slice(&self) -> &[T] {
        self.iter.as_slice()
    }

    /// Returns a reference to the underlying allocator.
    #[must_use]
    #[inline(always)]
    pub fn allocator(&self) -> &A {
        unsafe { self.vec.as_ref().allocator() }
    }

    /// Keep unyielded elements in the source `Vec`.
    ///
    /// # Examples
    ///
    /// ```
    /// use allocator_api2::vec;
    ///
    /// let mut vec = vec!['a', 'b', 'c'];
    /// let mut drain = vec.drain(..);
    ///
    /// assert_eq!(drain.next().unwrap(), 'a');
    ///
    /// // This call keeps 'b' and 'c' in the vec.
    /// drain.keep_rest();
    ///
    /// // If we wouldn't call `keep_rest()`,
    /// // `vec` would be empty.
    /// assert_eq!(vec, ['b', 'c']);
    /// ```
    #[inline(always)]
    pub fn keep_rest(self) {
        // At this moment layout looks like this:
        //
        // [head] [yielded by next] [unyielded] [yielded by next_back] [tail]
        //        ^-- start         \_________/-- unyielded_len        \____/-- self.tail_len
        //                          ^-- unyielded_ptr                  ^-- tail
        //
        // Normally `Drop` impl would drop [unyielded] and then move [tail] to the `start`.
        // Here we want to
        // 1. Move [unyielded] to `start`
        // 2. Move [tail] to a new start at `start + len(unyielded)`
        // 3. Update length of the original vec to `len(head) + len(unyielded) + len(tail)`
        //    a. In case of ZST, this is the only thing we want to do
        // 4. Do *not* drop self, as everything is put in a consistent state already, there is nothing to do
        let mut this = ManuallyDrop::new(self);

        unsafe {
            let source_vec = this.vec.as_mut();

            let start = source_vec.len();
            let tail = this.tail_start;

            let unyielded_len = this.iter.len();
            let unyielded_ptr = this.iter.as_slice().as_ptr();

            // ZSTs have no identity, so we don't need to move them around.
            let needs_move = mem::size_of::<T>() != 0;

            if needs_move {
                let start_ptr = source_vec.as_mut_ptr().add(start);

                // memmove back unyielded elements
                if unyielded_ptr != start_ptr {
                    let src = unyielded_ptr;
                    let dst = start_ptr;

                    ptr::copy(src, dst, unyielded_len);
                }

                // memmove back untouched tail
                if tail != (start + unyielded_len) {
                    let src = source_vec.as_ptr().add(tail);
                    let dst = start_ptr.add(unyielded_len);
                    ptr::copy(src, dst, this.tail_len);
                }
            }

            source_vec.set_len(start + unyielded_len + this.tail_len);
        }
    }
}

impl<'a, T, A: Allocator> AsRef<[T]> for Drain<'a, T, A> {
    #[inline(always)]
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

unsafe impl<T: Sync, A: Sync + Allocator> Sync for Drain<'_, T, A> {}

unsafe impl<T: Send, A: Send + Allocator> Send for Drain<'_, T, A> {}

impl<T, A: Allocator> Iterator for Drain<'_, T, A> {
    type Item = T;

    #[inline(always)]
    fn next(&mut self) -> Option<T> {
        self.iter
            .next()
            .map(|elt| unsafe { ptr::read(elt as *const _) })
    }

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<T, A: Allocator> DoubleEndedIterator for Drain<'_, T, A> {
    #[inline(always)]
    fn next_back(&mut self) -> Option<T> {
        self.iter
            .next_back()
            .map(|elt| unsafe { ptr::read(elt as *const _) })
    }
}

impl<T, A: Allocator> Drop for Drain<'_, T, A> {
    #[inline]
    fn drop(&mut self) {
        /// Moves back the un-`Drain`ed elements to restore the original `Vec`.
        struct DropGuard<'r, 'a, T, A: Allocator>(&'r mut Drain<'a, T, A>);

        impl<'r, 'a, T, A: Allocator> Drop for DropGuard<'r, 'a, T, A> {
            fn drop(&mut self) {
                if self.0.tail_len > 0 {
                    unsafe {
                        let source_vec = self.0.vec.as_mut();
                        // memmove back untouched tail, update to new length
                        let start = source_vec.len();
                        let tail = self.0.tail_start;
                        if tail != start {
                            let src = source_vec.as_ptr().add(tail);
                            let dst = source_vec.as_mut_ptr().add(start);
                            ptr::copy(src, dst, self.0.tail_len);
                        }
                        source_vec.set_len(start + self.0.tail_len);
                    }
                }
            }
        }

        let iter = mem::replace(&mut self.iter, [].iter());
        let drop_len = iter.len();

        let mut vec = self.vec;

        if size_of::<T>() == 0 {
            // ZSTs have no identity, so we don't need to move them around, we only need to drop the correct amount.
            // this can be achieved by manipulating the Vec length instead of moving values out from `iter`.
            unsafe {
                let vec = vec.as_mut();
                let old_len = vec.len();
                vec.set_len(old_len + drop_len + self.tail_len);
                vec.truncate(old_len + self.tail_len);
            }

            return;
        }

        // ensure elements are moved back into their appropriate places, even when drop_in_place panics
        let _guard = DropGuard(self);

        if drop_len == 0 {
            return;
        }

        // as_slice() must only be called when iter.len() is > 0 because
        // vec::Splice modifies vec::Drain fields and may grow the vec which would invalidate
        // the iterator's internal pointers. Creating a reference to deallocated memory
        // is invalid even when it is zero-length
        let drop_ptr = iter.as_slice().as_ptr();

        unsafe {
            // drop_ptr comes from a slice::Iter which only gives us a &[T] but for drop_in_place
            // a pointer with mutable provenance is necessary. Therefore we must reconstruct
            // it from the original vec but also avoid creating a &mut to the front since that could
            // invalidate raw pointers to it which some unsafe code might rely on.
            let vec_ptr = vec.as_mut().as_mut_ptr();
            let drop_offset = drop_ptr.offset_from(vec_ptr) as usize;
            let to_drop = ptr::slice_from_raw_parts_mut(vec_ptr.add(drop_offset), drop_len);
            ptr::drop_in_place(to_drop);
        }
    }
}

impl<T, A: Allocator> ExactSizeIterator for Drain<'_, T, A> {}

impl<T, A: Allocator> FusedIterator for Drain<'_, T, A> {}

// TODO: remove this when the standard library API is stable
/// Removes the specified range from the vector in bulk, returning all
/// removed elements as an iterator. If the iterator is dropped before
/// being fully consumed, it drops the remaining removed elements.
///
/// The returned iterator keeps a mutable borrow on the vector to optimize
/// its implementation.
///
/// # Panics
///
/// Panics if the starting point is greater than the end point or if
/// the end point is greater than the length of the vector.
///
/// # Leaking
///
/// If the returned iterator goes out of scope without being dropped (due to
/// [`mem::forget`], for example), the vector may have lost and leaked
/// elements arbitrarily, including elements outside the range.
///
/// # Examples
///
/// ```
/// use allocator_api2::{vec, vec::Vec};
///
/// let mut v = vec![1, 2, 3];
/// let u: Vec<_> = v.drain(1..).collect();
/// assert_eq!(v, &[1]);
/// assert_eq!(u, &[2, 3]);
///
/// // A full range clears the vector, like `clear()` does
/// v.drain(..);
/// assert_eq!(v, &[]);
/// ```
#[inline(always)]
pub fn drain_vec<T, A, R>(this: &mut Vec<T, A>, range: R) -> Drain<'_, T, A>
where
    A: Allocator,
    R: RangeBounds<usize>,
{
    // Memory safety
    //
    // When the Drain is first created, it shortens the length of
    // the source vector to make sure no uninitialized or moved-from elements
    // are accessible at all if the Drain's destructor never gets to run.
    //
    // Drain will ptr::read out the values to remove.
    // When finished, remaining tail of the vec is copied back to cover
    // the hole, and the vector length is restored to the new length.
    //
    let len = this.len();

    // Replaced by code below
    // let Range { start, end } = slice::range(range, ..len);

    // Panics if range is out of bounds
    let _ = &this.as_slice()[(range.start_bound().cloned(), range.end_bound().cloned())];

    let start = match range.start_bound() {
        Bound::Included(&n) => n,
        Bound::Excluded(&n) => n + 1,
        Bound::Unbounded => 0,
    };
    let end = match range.end_bound() {
        Bound::Included(&n) => n + 1,
        Bound::Excluded(&n) => n,
        Bound::Unbounded => len,
    };

    unsafe {
        // set self.vec length's to start, to be safe in case Drain is leaked
        this.set_len(start);
        let range_slice = slice::from_raw_parts(this.as_ptr().add(start), end - start);
        Drain {
            tail_start: end,
            tail_len: len - end,
            iter: range_slice.iter(),
            vec: NonNull::from(this),
        }
    }
}