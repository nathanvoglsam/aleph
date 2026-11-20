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
use std::fmt;
use std::iter::FusedIterator;
use std::marker::PhantomData;
use std::mem::{self, ManuallyDrop, MaybeUninit, size_of};
use std::ptr::{self, NonNull};
use std::slice::{self};

/// An iterator that moves out of a vector.
///
/// This `struct` is created by the `into_iter` method on [`Vec`](super::Vec)
/// (provided by the [`IntoIterator`] trait).
///
/// # Example
///
/// ```
/// # use allocator_api2::vec;
/// let v = vec![0, 1, 2];
/// let iter: vec::IntoIter<_> = v.into_iter();
/// ```
pub struct IntoIter<T, A: Allocator = Global> {
    pub(super) buf: NonNull<T>,
    pub(super) phantom: PhantomData<T>,
    pub(super) cap: usize,
    // the drop impl reconstructs a RawVec from buf, cap and alloc
    // to avoid dropping the allocator twice we need to wrap it into ManuallyDrop
    pub(super) alloc: ManuallyDrop<A>,
    pub(super) ptr: *const T,
    pub(super) end: *const T,
}

impl<T: fmt::Debug, A: Allocator> fmt::Debug for IntoIter<T, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("IntoIter").field(&self.as_slice()).finish()
    }
}

impl<T, A: Allocator> IntoIter<T, A> {
    /// Returns the remaining items of this iterator as a slice.
    ///
    /// # Examples
    ///
    /// ```
    /// # use allocator_api2::vec;
    /// let vec = vec!['a', 'b', 'c'];
    /// let mut into_iter = vec.into_iter();
    /// assert_eq!(into_iter.as_slice(), &['a', 'b', 'c']);
    /// let _ = into_iter.next().unwrap();
    /// assert_eq!(into_iter.as_slice(), &['b', 'c']);
    /// ```
    pub fn as_slice(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.ptr, self.len()) }
    }

    /// Returns the remaining items of this iterator as a mutable slice.
    ///
    /// # Examples
    ///
    /// ```
    /// # use allocator_api2::vec;
    /// let vec = vec!['a', 'b', 'c'];
    /// let mut into_iter = vec.into_iter();
    /// assert_eq!(into_iter.as_slice(), &['a', 'b', 'c']);
    /// into_iter.as_mut_slice()[2] = 'z';
    /// assert_eq!(into_iter.next().unwrap(), 'a');
    /// assert_eq!(into_iter.next().unwrap(), 'b');
    /// assert_eq!(into_iter.next().unwrap(), 'z');
    /// ```
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { &mut *self.as_raw_mut_slice() }
    }

    /// Returns a reference to the underlying allocator.
    #[inline(always)]
    pub fn allocator(&self) -> &A {
        &self.alloc
    }

    fn as_raw_mut_slice(&mut self) -> *mut [T] {
        ptr::slice_from_raw_parts_mut(self.ptr as *mut T, self.len())
    }
}

impl<T, A: Allocator> AsRef<[T]> for IntoIter<T, A> {
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

unsafe impl<T: Send, A: Allocator + Send> Send for IntoIter<T, A> {}

unsafe impl<T: Sync, A: Allocator + Sync> Sync for IntoIter<T, A> {}

impl<T, A: Allocator> Iterator for IntoIter<T, A> {
    type Item = T;

    #[inline(always)]
    fn next(&mut self) -> Option<T> {
        if self.ptr == self.end {
            None
        } else if size_of::<T>() == 0 {
            // purposefully don't use 'ptr.offset' because for
            // vectors with 0-size elements this would return the
            // same pointer.
            self.ptr = self.ptr.cast::<u8>().wrapping_add(1).cast();

            // Make up a value of this ZST.
            Some(unsafe { mem::zeroed() })
        } else {
            let old = self.ptr;
            self.ptr = unsafe { self.ptr.add(1) };

            Some(unsafe { ptr::read(old) })
        }
    }

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let exact = if size_of::<T>() == 0 {
            self.end.addr().wrapping_sub(self.ptr.addr())
        } else {
            unsafe { self.end.offset_from(self.ptr) as usize }
        };
        (exact, Some(exact))
    }

    #[inline(always)]
    fn count(self) -> usize {
        self.len()
    }
}

impl<T, A: Allocator> DoubleEndedIterator for IntoIter<T, A> {
    #[inline(always)]
    fn next_back(&mut self) -> Option<T> {
        if self.end == self.ptr {
            None
        } else if size_of::<T>() == 0 {
            // See above for why 'ptr.offset' isn't used
            self.end = self.end.cast::<u8>().wrapping_add(1).cast();

            // Make up a value of this ZST.
            Some(unsafe { mem::zeroed() })
        } else {
            self.end = unsafe { self.end.sub(1) };

            Some(unsafe { ptr::read(self.end) })
        }
    }
}

impl<T, A: Allocator> ExactSizeIterator for IntoIter<T, A> {}

impl<T, A: Allocator> FusedIterator for IntoIter<T, A> {}

impl<T: Clone, A: Allocator + Clone> Clone for IntoIter<T, A> {
    fn clone(&self) -> Self {
        let mut vec = Vec::<T, A>::with_capacity_in(self.len(), (*self.alloc).clone());
        vec.extend(self.as_slice().iter().cloned());
        vec_into_iter(vec)
    }
}

impl<T, A: Allocator> Drop for IntoIter<T, A> {
    fn drop(&mut self) {
        struct DropGuard<'a, T, A: Allocator>(&'a mut IntoIter<T, A>);

        impl<T, A: Allocator> Drop for DropGuard<'_, T, A> {
            fn drop(&mut self) {
                unsafe {
                    // `IntoIter::alloc` is not used anymore after this and will be dropped by RawVec
                    let alloc = ManuallyDrop::take(&mut self.0.alloc);
                    // RawVec handles deallocation. Use MaybeUninit to ensure no T is dropped
                    let ptr = self.0.buf.cast::<MaybeUninit<T>>();
                    let _ = Vec::from_raw_parts_in(ptr.as_ptr(), self.0.cap, self.0.cap, alloc);
                }
            }
        }

        let guard = DropGuard(self);
        // destroy the remaining elements
        unsafe {
            ptr::drop_in_place(guard.0.as_raw_mut_slice());
        }
        // now `guard` will be dropped and do the rest
    }
}

impl<T, A> Default for IntoIter<T, A>
where
    A: Allocator + Default,
{
    /// Creates an empty `vec::IntoIter`.
    ///
    /// ```
    /// # use allocator_api2::vec;
    /// let iter: vec::IntoIter<u8> = Default::default();
    /// assert_eq!(iter.len(), 0);
    /// assert_eq!(iter.as_slice(), &[]);
    /// ```
    fn default() -> Self {
        vec_into_iter(Vec::new_in(Default::default()))
    }
}

// TODO: remove this once into_iter is stable with custom allocator
/// Creates a consuming iterator, that is, one that moves each value out of
/// the vector (from start to end). The vector cannot be used after calling
/// this.
///
/// # Examples
///
/// ```
/// use allocator_api2::vec;
///
/// let v = vec!["a".to_string(), "b".to_string()];
/// let mut v_iter = v.into_iter();
///
/// let first_element: Option<String> = v_iter.next();
///
/// assert_eq!(first_element, Some("a".to_string()));
/// assert_eq!(v_iter.next(), Some("b".to_string()));
/// assert_eq!(v_iter.next(), None);
/// ```
#[inline(always)]
pub fn vec_into_iter<T, A: Allocator>(this: Vec<T, A>) -> IntoIter<T, A> {
    unsafe {
        let mut me = ManuallyDrop::new(this);
        let alloc = ManuallyDrop::new(ptr::read(me.allocator()));
        let begin = me.as_mut_ptr();
        let end = if size_of::<T>() == 0 {
            begin.cast::<u8>().wrapping_add(me.len()).cast()
        } else {
            begin.add(me.len()) as *const T
        };
        let cap = me.capacity();
        IntoIter {
            buf: NonNull::new_unchecked(begin),
            phantom: PhantomData,
            cap,
            alloc,
            ptr: begin,
            end,
        }
    }
}
