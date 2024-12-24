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

use std::mem::{needs_drop, ManuallyDrop};
use std::ptr::NonNull;

use bumpalo::Bump;

pub struct DropLink {
    pub ptr: NonNull<ManuallyDrop<()>>,
    pub dropper: unsafe fn(NonNull<ManuallyDrop<()>>),
    pub prev: Option<NonNull<DropLink>>,
}

impl DropLink {
    /// Constructs a new DropLink for the given value, with the given type.
    ///
    /// Will leave 'prev' as 'None', it is the caller's responsibility to initialize the 'prev'
    /// pointer.
    pub fn new<T>(v: NonNull<T>) -> Self {
        let ptr = v.cast::<ManuallyDrop<()>>();
        Self {
            ptr,
            dropper: dropper_impl::<T>,
            prev: None,
        }
    }

    /// Places another DropLink onto the list, only if T actually needs to be dropped.
    ///
    /// # Safety
    ///
    /// The implementation of this function isn't unsafe, but it is important to highlight that the
    /// links in the list must remain live until needed. They will be allocated into the given arena
    /// and so you must ensure that the arena is live when actually walking the drop list.
    pub fn append_drop_list<T>(arena: &Bump, head: &mut Option<NonNull<DropLink>>, v: NonNull<T>) {
        // Only append to the drop list if we actually need to drop the object
        if needs_drop::<T>() {
            // Create and store the link in the dropper linked list for this object
            let mut dropper_link = DropLink::new(v);
            dropper_link.prev = *head;
            let dropper_link = arena.alloc(dropper_link);

            // Update the linked-list head for this table
            *head = Some(NonNull::from(dropper_link));
        }
    }

    /// Utility wrapper for [DropLink::drop_all] that handles dropping from a head pointer instead
    /// of requiring a reference. This will null the head pointer given to prevent it being walked
    /// again.
    ///
    /// # Safety
    ///
    /// Has all the same safety requirements as 'drop_all', but with the added requirement that
    /// accessing the head of the list through the given pointer is sound.
    #[inline]
    pub unsafe fn drop_and_null(v: &mut Option<NonNull<DropLink>>) {
        // Call drop on all the inserted objects
        if let Some(dropper) = v {
            // Safety: implementation and API guarantees that dropper only gets called once per
            //         object, and always on the correct type.
            unsafe {
                let dropper = dropper.as_ref();
                dropper.drop_all();
            }
        }
        *v = None; // Null this just-in-case
    }

    /// Walks the linked list and calls the drop functions for all the entries in the list.
    ///
    /// # Safety
    ///
    /// It is the caller's responsibility to ensure that access to the objects in the drop list is
    /// sound.
    #[inline]
    pub unsafe fn drop_all(&self) {
        // Call drop on all the inserted objects
        let mut current = Some(NonNull::from(self));
        while let Some(v) = current {
            let v = v.as_ref();
            current = v.drop_object();
        }
    }

    /// Drops the object referenced by 'ptr' and returns a reference to the next link in the chain,
    /// or None if we've reached the end of the list.
    ///
    /// This _will_ lead to the referenced object being dropped.
    ///
    /// # Safety
    ///
    /// Synchronizing and validating access to the underlying object referenced by 'ptr' is the
    /// caller's responsibility. Links are immutable once created so the [DropLink] itself is
    /// thread safe.
    #[inline(always)]
    pub unsafe fn drop_object(&self) -> Option<NonNull<DropLink>> {
        (self.dropper)(self.ptr);
        self.prev
    }
}

unsafe fn dropper_impl<T>(v: NonNull<ManuallyDrop<()>>) {
    let mut v = v.cast::<ManuallyDrop<T>>();
    let v = v.as_mut();
    ManuallyDrop::drop(v);
}
