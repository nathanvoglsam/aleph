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

use std::sync::atomic::{AtomicPtr, Ordering};

use crossbeam::atomic::AtomicCell;

/// A single element in a [`InitList`]
pub struct ListItem<T: 'static> {
    object: T,
    next: AtomicCell<Option<&'static Self>>,
}

impl<T: 'static> ListItem<T> {
    /// Constructs a new list item containing the given object. It is expected that a caller will
    /// use [`InitList::push_entry`] to emplace the result into a list.
    pub const fn new(object: T) -> Self {
        Self {
            object,
            next: AtomicCell::new(None),
        }
    }

    /// Constructs a new list item containing the given object. It is expected that a caller will
    /// use [`InitList::push_entry`] to emplace the result into a list.
    ///
    /// This is an alternate form of [`ListItem::new`] that uses a leaked box to yield a `'static`
    /// lifetime. This is more expensive but allows making list items at runtime, and allows making
    /// items from types that don't have a `const` constructor.
    pub fn new_leaked(object: T) -> &'static Self {
        use allocator_api2::alloc::System;
        use allocator_api2::boxed::Box;

        let out = Box::new_in(Self::new(object), System);
        Box::leak(out)
    }
}

/// Forms the head of what is expected to be a linked list that is initialized during the init phase
/// of an application. This could be during early phases of an application's 'main' fn, but also
/// could be during static init using the 'ctor' crate.
///
/// An instance of this type presents a simple API.
///
/// 1. Construct a new list using [`InitList::new`].
/// 2. Push entries using [`InitList::push_entry`].
/// 3. Seal the list, making it readable [`InitList::seal`].
/// 4. Read the list using [`InitList::iter`].
///
/// These operations must be performed in order.
///
/// # Sealing
///
/// It is impossible to read an init list until after 'seal' has been called. Once 'seal' has been
/// called the state of the list will be snapshot and made available to readers. Calling 'iter' and
/// trying to read the list before calling 'seal' will yield an empty iterator.
///
/// The intended API is that once initialization of the list is complete it becomes immutable. This
/// prevents adding elements to the list after initialization is complete. This key constraint
/// allows making assumptions in unsafe code to create efficient, safe APIs.
pub struct InitList<T: 'static> {
    /// Pointer to the head of the list used during the construction phase.
    next: AtomicCell<Option<&'static ListItem<T>>>,

    /// Will eventually contain a pointer to the head of the list once the list has been sealed.
    sealed: AtomicPtr<ListItem<T>>,
}

impl<T: 'static> InitList<T> {
    /// Constructs a new, unsealed, empty list.
    pub const fn new() -> Self {
        Self {
            next: AtomicCell::new(None),
            sealed: AtomicPtr::new(std::ptr::null_mut()),
        }
    }

    /// Pushes a new entry into the init list.
    ///
    /// # Sealing
    ///
    /// This will never panic. However, once the list is sealed using [`InitList::seal`], it is
    /// impossible to read this entry from the list.
    pub fn push_entry(&self, node: &'static ListItem<T>) {
        let next = self.next.swap(Some(node));
        node.next.store(next);
    }

    /// Seals the list, taking the current state of the list as the final state and making it
    /// available to readers via [`InitList::iter`].
    ///
    /// # Sealing
    ///
    /// This function will take the current head of the list and copy a point into a second internal
    /// pointer. This second pointer can only be written to once. Any calls to 'seal' after the
    /// first will never change the cached 'seal' pointer.
    ///
    /// This function will return `None` for any call after the first\*
    ///
    /// \*Calling 'seal' on an empty init list does nothing. You may observe 'seal' return 'Some'
    /// multiple times if called on an empty init list.
    pub fn seal(&self) -> Option<()> {
        if let Some(sealed) = self.next.load() {
            let sealed_ptr = sealed as *const _ as *mut _;
            let result = self.sealed.compare_exchange(
                std::ptr::null_mut(),
                sealed_ptr,
                Ordering::SeqCst,
                Ordering::SeqCst,
            );
            match result {
                Ok(_) => Some(()),
                Err(_) => None,
            }
        } else {
            Some(())
        }
    }

    /// Returns an iterator over the entries push into the list.
    ///
    /// # Sealing
    ///
    /// This will always return an empty iterator unless 'seal' has been called at least once to
    /// lock the state of the list and make it available to readers.
    pub fn iter(&self) -> InitListIter<T> {
        let head = self.sealed.load(Ordering::Acquire);

        // Safety: Our implementation only ever writes 'null' or a valid static pointer to 'sealed'
        //         so it's impossible for this to be an invalid value.
        let head = unsafe { head.as_ref() };

        InitListIter::new_from_head(head)
    }
}

/// Simple iterator over a [`InitList`].
pub struct InitListIter<T: 'static> {
    next: Option<&'static ListItem<T>>,
}

impl<T: 'static> InitListIter<T> {
    /// Constructs a new [`InitListIter`] instance from the given list node.
    const fn new_from_head(v: Option<&'static ListItem<T>>) -> Self {
        Self { next: v }
    }
}

impl<T: 'static> Iterator for InitListIter<T> {
    type Item = &'static T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ptr) = self.next {
            self.next = ptr.next.load();
            Some(&ptr.object)
        } else {
            None
        }
    }
}
