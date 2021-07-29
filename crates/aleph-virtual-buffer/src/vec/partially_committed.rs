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

use crate::VirtualBuffer;
use std::io::{Error, ErrorKind};
use std::marker::PhantomData;
use std::mem::{align_of, needs_drop, size_of};
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::slice::{from_raw_parts, from_raw_parts_mut, SliceIndex};

pub struct VirtualVec<T> {
    /// The backing buffer the `VirtualVec` uses as storage
    buffer: VirtualBuffer,

    /// The number of `T` items stored inside the vector
    len: usize,

    /// The number of `T` items that can be stored without having to commit more pages.
    committed: usize,

    /// Phantom data
    phantom: PhantomData<T>,
}

impl<T> VirtualVec<T> {
    ///
    #[inline]
    pub fn new(capacity: usize) -> std::io::Result<Self> {
        let buffer = VirtualBuffer::reserve_bytes(capacity * size_of::<T>())?;

        // Check that we have the required alignment
        //
        // This should almost always pass as the buffers should always be aligned to a page boundary
        // which are aligned to 4096. Types that need higher alignment than that are very rare.
        let wanted_alignment = align_of::<T>();
        let buffer_base = buffer.as_ptr() as usize;
        if buffer_base % wanted_alignment != 0 {
            Err(Error::new(
                ErrorKind::Other,
                "The allocated buffer was not sufficiently aligned",
            ))
        } else {
            Ok(VirtualVec {
                buffer,
                len: 0,
                committed: 0,
                phantom: Default::default(),
            })
        }
    }

    /// Returns the number of items that the `VirtualVec` holds.
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns the number of items that the `VirtualVec` has memory committed for.
    #[inline]
    pub fn committed(&self) -> usize {
        self.committed
    }

    /// Returns the number of items that the `VirtualVec` has address space reserved for. This is
    /// the maximum number of elements that can be held in a `VirtualVec`.
    #[inline]
    pub fn capacity(&self) -> usize {
        self.buffer.len() / size_of::<T>()
    }

    /// Place an element onto the end of the vec.
    ///
    /// # Panics
    /// * If the length of the vec would overflow the capacity.
    #[inline]
    pub fn push(&mut self, v: T) {
        // Check if the vector needs to grow the committed range
        if self.len == self.committed {
            self.grow().expect("Ran out of space in `VirtualVec`");
        }

        // SAFETY:
        // The type's safe interface ensures that the pointer is valid to write to, and we ensure
        // that memory is committed at the address with the above growth check.
        unsafe {
            self.ptr_mut().add(self.len).write(v);
            self.len += 1;
        }
    }

    /// Remove and return the last element of the vec, if there is one.
    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;

            // SAFETY:
            // The slot is guaranteed to be committed and contain a valid object by the type's safe
            // interface and so we can just read it here.
            //
            // By decrementing the length we can just forget about the contents of the slot, and by
            // returning the value we don't need to care about
            unsafe { Some(self.ptr().add(self.len).read()) }
        }
    }

    /// Removes the item at `index`, shifting all others down by one index.
    ///
    /// Returns the removed element.
    ///
    /// # Panics
    ///
    /// * If the index is out of bounds.
    #[inline]
    pub fn remove(&mut self, index: usize) -> T {
        if self.len == 0 {
            panic!("VirtualVec::remove> Tried to remove an element from empty vec");
        } else {
            let targets: &mut [T] = &mut self.as_slice_mut()[index..];

            // SAFETY:
            // This is safe as the dangling data for the item will be inaccessible so `item` still
            // upholds correct ownership semantics. The item in the array will end up at the end before
            // being "popped" off by decrementing `self.len` which prevents drop from being called
            // twice on the same item (when the vec is destroyed, and when the item we yield is
            // dropped).
            let item = unsafe { targets.as_ptr().read() };
            targets.rotate_left(1);
            self.len -= 1;
            item
        }
    }

    /// As [`resize_with`](VirtualVec::resize_with)
    /// and it clones the value as the closure.
    #[inline]
    pub fn resize(&mut self, new_len: usize, new_val: T)
    where
        T: Clone,
    {
        self.resize_with(new_len, || new_val.clone())
    }

    /// Resize the vec to the new length.
    ///
    /// * If it needs to be longer, it's filled with repeated calls to the
    ///   provided function.
    /// * If it needs to be shorter, it's truncated.
    ///
    #[inline]
    pub fn resize_with<F: FnMut() -> T>(&mut self, new_len: usize, mut f: F) {
        match new_len.checked_sub(self.len) {
            None => {
                if needs_drop::<T>() {
                    while self.len() > new_len {
                        self.len -= 1;
                        // SAFETY:
                        // This is safe as the pointer is guaranteed to point to a valid object.
                        //
                        // We then just repeatedly pop elements off the end, dropping them as we
                        // go.
                        //
                        // The object is already inaccessible outside this function by the time
                        // the drop is actually called.
                        unsafe {
                            self.ptr_mut().add(self.len).drop_in_place();
                        }
                    }
                } else {
                    self.len = new_len;
                }
            }
            Some(new_elements) => {
                for _ in 0..new_elements {
                    self.push(f());
                }
            }
        }
    }

    /// Remove an element, swapping the end of the vec into its place.
    ///
    /// ## Panics
    /// * If the index is out of bounds.
    #[inline]
    pub fn swap_remove(&mut self, index: usize) -> T {
        assert!(
            index < self.len,
            "VirtualVec::swap_remove> index {} is out of bounds {}",
            index,
            self.len
        );
        if self.len == 0 {
            panic!("VirtaulVec::swap_remove> tried to remove from empty vec");
        } else if index == self.len - 1 {
            self.pop().unwrap()
        } else {
            // SAFETY:
            // The pointers are all guaranteed and checked to be valid for both alignment, validity
            // and if the overlap.
            unsafe {
                // Swap the value we want to remove so it's the last item in the array
                let a = self.ptr_mut().add(index);
                let b = self.ptr_mut().add(self.len - 1);
                std::ptr::swap_nonoverlapping(a, b, 1);
            }
            // Pop the removed element off the end
            self.pop().unwrap()
        }
    }

    /// Reduces the vec's length to the given value.
    ///
    /// If the vec is already shorter than the input, nothing happens.
    #[inline]
    pub fn truncate(&mut self, new_len: usize) {
        // Check if we have anything to truncate off the end
        if new_len < self.len {
            // Only drop if we actually need to
            if needs_drop::<T>() {
                // Get the range of values we need to drop
                let start = new_len;
                let end = self.len;

                for i in start..end {
                    unsafe {
                        self.ptr_mut().add(i).drop_in_place();
                    }
                }
            }

            // Update the length
            self.len = new_len;
        }
    }

    /// Truncates the `SliceVec` down to length 0.
    #[inline(always)]
    pub fn clear(&mut self) {
        self.truncate(0)
    }

    /// Reserves (commits) enough address space for at least `count` items
    #[inline]
    pub fn reserve(&mut self, count: usize) -> std::io::Result<()> {
        if count > self.capacity() {
            panic!(
                "VirtualVec::reserve> total length {} exceeds capacity {}",
                count,
                self.capacity()
            )
        } else if count <= self.committed {
            // We already have enough space committed
            Ok(())
        } else {
            // Keep doubling the space until it fits `count`
            //
            // We do this so we only need one `VirtualAlloc` / `mmap` call
            let mut to_commit = self.committed;
            while count < to_commit {
                to_commit = to_commit
                    .checked_mul(2)
                    .expect("VirtualVec::reserve> overflowed calculating growth");
            }

            // While it shouldn't be possible for this to be reachable I will check anyway
            debug_assert!(
                to_commit <= self.capacity(),
                "VirtualVec::reserve> total length {} exceeds capacity {}",
                to_commit,
                self.capacity()
            );

            // The start of the range to commit is after the last committed item
            let uncommitted_start = self
                .committed
                .checked_mul(size_of::<T>())
                .expect("VirtualVec::reserve> overflow getting commit range start");

            // We commit up to the `to_commit`
            let uncommitted_end = to_commit
                .checked_mul(size_of::<T>())
                .expect("VirtualVec::reserve> overflow getting commit range end");

            // Commit the range
            self.buffer.commit(uncommitted_start..uncommitted_end)?;

            // Update the number of committed entries
            self.committed = to_commit;

            Ok(())
        }
    }

    #[inline]
    pub fn extend_from_slice(&mut self, sli: &[T])
    where
        T: Clone,
    {
        // No-op on empty slice
        if sli.is_empty() {
            return;
        }

        // Check if we have enough capacity
        let new_len = self
            .len
            .checked_add(sli.len())
            .expect("VirtualVec::extend_from_slice> overflow adding lengths");
        if new_len > self.capacity() {
            panic!(
                "VirtualVec::extend_from_slice> total length {} exceeds capacity {}",
                new_len,
                self.capacity()
            )
        }

        // Pre-reserve space for the new entries
        self.reserve(new_len).unwrap();

        // SAFETY:
        // Safe as we've guaranteed the memory to be accessible with the above reserve call
        unsafe {
            let base = self.ptr_mut().add(self.len);
            for i in 0..sli.len() {
                base.add(i).write(sli[i].clone())
            }
        };

        // Update the length
        self.len = new_len;
    }

    // TODO: retain

    // TODO: drain

    /// Returns a slice over all the elements stored inside `self`
    #[inline(always)]
    pub fn as_slice(&self) -> &[T] {
        unsafe { from_raw_parts(self.ptr(), self.len) }
    }

    /// Returns a slice over all the elements stored inside `self`
    #[inline(always)]
    pub fn as_slice_mut(&mut self) -> &mut [T] {
        unsafe { from_raw_parts_mut(self.ptr_mut(), self.len) }
    }

    // Grows the committed range geometrically
    #[inline]
    fn grow(&mut self) -> std::io::Result<()> {
        if let Some(committed) = self.committed.checked_mul(2) {
            if committed > self.capacity() {
                panic!(
                    "VirtualVec::grow> total length {} exceeds capacity {}",
                    self.committed * 2,
                    self.capacity()
                )
            }
        } else {
            panic!(
                "VirtualVec::grow> overflow on length {}, doubling overflows",
                self.committed,
            )
        }

        let (new_committed, uncommitted_start, uncommitted_end) = if self.committed == 0 {
            let new_committed = 1;
            let uncommitted_start = 0;
            let uncommitted_end = 1;

            (new_committed, uncommitted_start, uncommitted_end)
        } else {
            let new_committed = self.committed * 2;

            // The start of the range to commit is after the last committed item
            let uncommitted_start = self
                .committed
                .checked_mul(size_of::<T>())
                .expect("VirtualVec::grow> overflow on byte size");

            // We double the amount of committed space to grow geometrically
            let uncommitted_end = uncommitted_start
                .checked_mul(2)
                .expect("VirtualVec::grow> overflow on byte size");

            (new_committed, uncommitted_start, uncommitted_end)
        };

        // Commit the range
        self.buffer.commit(uncommitted_start..uncommitted_end)?;

        // Update the number of committed entries
        self.committed = new_committed;

        Ok(())
    }

    #[inline(always)]
    fn ptr(&self) -> *const T {
        self.buffer.data() as *const u8 as *const T
    }

    #[inline(always)]
    fn ptr_mut(&mut self) -> *mut T {
        self.buffer.data() as *mut T
    }
}

impl<T> Deref for VirtualVec<T> {
    type Target = [T];
    #[inline(always)]
    #[must_use]
    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<T> DerefMut for VirtualVec<T> {
    #[inline(always)]
    #[must_use]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_slice_mut()
    }
}

impl<'a, T: 'a, I> Index<I> for VirtualVec<T>
where
    I: SliceIndex<[T]>,
{
    type Output = <I as SliceIndex<[T]>>::Output;
    #[inline(always)]
    #[must_use]
    fn index(&self, index: I) -> &Self::Output {
        &self.deref()[index]
    }
}

impl<T, I> IndexMut<I> for VirtualVec<T>
where
    I: SliceIndex<[T]>,
{
    #[inline(always)]
    #[must_use]
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.deref_mut()[index]
    }
}

impl<T> Drop for VirtualVec<T> {
    #[inline]
    fn drop(&mut self) {
        if needs_drop::<T>() {
            for i in 0..self.len {
                // SAFETY:
                // This just iterates over all valid elements and calls their drop function, which
                // is a perfectly valid operation.
                unsafe {
                    let ptr = self.ptr_mut();
                    ptr.add(i).drop_in_place();
                }
            }
        }
    }
}
