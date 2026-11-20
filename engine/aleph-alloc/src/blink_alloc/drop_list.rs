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
//! This crate provides `DropList` type which is
//! an intrusive linked list of drop functions.
//!

use std::cell::Cell;
use std::mem::MaybeUninit;
use std::ptr::{self, NonNull, addr_of_mut, slice_from_raw_parts_mut};

/// Single drop item.
/// Drops associated value when invoked.
struct Drops {
    /// Number of values pointed by `ptr`.
    count: usize,

    /// Drop function.
    drop: unsafe fn(NonNull<Drops>, usize),

    /// Next item in the list.
    next: Option<NonNull<Self>>,
}

impl Drops {
    unsafe fn drop(ptr: NonNull<Self>) -> Option<NonNull<Self>> {
        let Drops { count, drop, next } = *unsafe { ptr.as_ref() };
        // Safety: `DropItem` constructed as part of `DropItemValue<T>`.
        // And `drop` is set to `drop_in_place::<T>`.
        unsafe { (drop)(ptr, count) };
        next
    }
}

#[repr(C)]
pub struct DropItem<T: ?Sized> {
    drops: Drops,
    pub value: T,
}

impl<T> DropItem<T> {
    pub unsafe fn init_value<'a, I>(
        mut ptr: NonNull<DropItem<T>>,
        init: I,
        f: impl FnOnce(&mut MaybeUninit<T>, I),
    ) -> &'a mut Self {
        let drops_ptr = unsafe { addr_of_mut!((*ptr.as_ptr()).drops) };
        unsafe { f(&mut *addr_of_mut!((*ptr.as_ptr()).value).cast(), init) };
        unsafe {
            ptr::write(
                drops_ptr,
                Drops {
                    count: 1,
                    drop: drop_from_item::<T>,
                    next: None,
                },
            );
        }
        unsafe { ptr.as_mut() }
    }
}

impl<T> DropItem<[T; 0]> {
    pub unsafe fn init_slice<'a>(
        mut ptr: NonNull<DropItem<[T; 0]>>,
        count: usize,
    ) -> (&'a mut Self, &'a mut [T]) {
        debug_assert_ne!(
            count, 0,
            "DropItem<[T]> should not be constructed with count 0"
        );
        unsafe {
            ptr::write(
                ptr.as_ptr().cast(),
                Drops {
                    count,
                    drop: drop_from_item::<T>,
                    next: None,
                },
            );
        }
        let slice = unsafe { std::slice::from_raw_parts_mut(ptr.as_ptr().add(1).cast(), count) };
        (unsafe { ptr.as_mut() }, slice)
    }
}

/// Intrusive linked list of drop functions.
pub struct DropList {
    // Root item of the list.
    // Contains `None` if list is empty.
    // Lifetime of the items is bound to `DropList::reset` method calls.
    root: Cell<Option<NonNull<Drops>>>,
}

impl DropList {
    pub const fn new() -> Self {
        DropList {
            root: Cell::new(None),
        }
    }

    /// Adds new drop item for given typed pointer.
    ///
    /// # Safety
    ///
    /// `item` reference must be valid until next call to [`DropList::reset`].
    #[allow(clippy::mut_from_ref)]
    pub unsafe fn add<'a, 'b: 'a, T: ?Sized>(&'a self, item: &'b mut DropItem<T>) -> &'a mut T {
        item.drops.next = self.root.take();
        let item = NonNull::from(item);
        self.root.set(Some(item.cast()));
        unsafe { &mut *addr_of_mut!((*item.as_ptr()).value) }
    }

    /// Drops all items in the list.
    pub fn reset(&mut self) {
        let mut next = self.root.take();

        while let Some(item_ptr) = next {
            // Safety: `item` is a valid pointer to `DropItem`.
            // And it didn't move since it was added to the list.
            unsafe {
                next = Drops::drop(item_ptr);
            }
        }
    }
}

/// Type-erased `std::ptr::drop_in_place` wrapper.
unsafe fn drop_from_item<T>(ptr: NonNull<Drops>, count: usize) {
    let ptr = ptr.cast::<DropItem<T>>();
    let value_ptr = unsafe { addr_of_mut!((*ptr.as_ptr()).value) };
    unsafe { std::ptr::drop_in_place(slice_from_raw_parts_mut(value_ptr, count)) }
}
