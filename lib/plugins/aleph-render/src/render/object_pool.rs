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

use crate::render::{Handle, HandlePool};
use std::num::NonZeroU8;

pub struct ObjectPool<T> {
    /// A pool_id nonce that gets included in the handle, helps verify if the handle came from this
    /// pool or not. It's not a guarantee, but defense in depth.
    pool_id: NonZeroU8,

    /// Backing storage for the object pool. Uses a swap'n'pop scheme for allocating and freeing
    /// new objects as they are needed.
    objects: Vec<T>,

    /// SoA storage associated with `Self::objects` that holds whatever handle is currently pointing
    /// at the object in the slot. Enables patching the handles when we move objects around in the
    /// pool.
    back_references: Vec<Handle>,

    /// The handle pool, which we use for allocating the handles themselves. These add an extra
    /// indirection to accessing the object pool items, but it allows us to move the objects
    /// underneath the stable handles. Critical for swap'n'pop! Means we keep our objects densely
    /// packed!
    handles: HandlePool<u32>,
}

impl<T> ObjectPool<T> {
    pub fn new(pool_id: NonZeroU8) -> Self {
        Self {
            pool_id,
            objects: Vec::new(),
            back_references: Vec::new(),
            handles: HandlePool::new(),
        }
    }

    pub fn alloc(&mut self, data: T) -> Handle {
        let index = u32::try_from(self.objects.len()).expect("Too many objects!");
        let handle = self.handles.alloc(index);
        self.objects.push(data);
        self.back_references.push(handle);
        handle.with_pool_and_type_id(self.pool_id.get(), 0)
    }

    pub fn get_ref(&self, handle: Handle) -> Option<&T> {
        // Pre-filter if the pool_id is wrong
        if handle.to_fields().pool_id != self.pool_id.get() {
            return None;
        }

        if let Some(index) = self.handles.get_copied(handle) {
            Some(&self.objects[index as usize])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, handle: Handle) -> Option<&mut T> {
        // Pre-filter if the pool_id is wrong
        if handle.to_fields().pool_id != self.pool_id.get() {
            return None;
        }

        if let Some(index) = self.handles.get_copied(handle) {
            Some(&mut self.objects[index as usize])
        } else {
            None
        }
    }

    pub fn free(&mut self, handle: Handle) -> Option<T> {
        // Pre-filter if the pool_id is wrong
        if handle.to_fields().pool_id != self.pool_id.get() {
            return None;
        }

        if let Some(index) = self.handles.get_copied(handle) {
            let last_index = self.objects.len() - 1;

            let out = if index as usize != last_index {
                // If the object is _not_ the last then we do the old swap'n'pop where we swap with
                // the last element and pop. The only hitch is we need to patch the index stored in
                // the swapped item's handle.

                // Use the handle back-reference to patch the handle of the current last item to
                // point to the place it _will_ be after we do the swap and pop
                let last_item_handle = self.back_references[last_index];
                let last_item_index = self.handles.get_mut(last_item_handle).unwrap();
                *last_item_index = index;

                // Perform the swap'n'pop. Whatever was at 'last_index' will now be stored at
                // 'index' and whatever was at 'index' before has been removed from the pool
                self.objects.swap_remove(index as usize)
            } else {
                // If the object is the last we can just pop it off the end of the pool
                self.objects.pop().unwrap()
            };

            // Now that we've done the swap'n'pop (or just regular pop) we free the handle we just
            // asked to free.
            self.handles
                .free(handle)
                .expect("Someone else freed the handle first (how?)");

            // And we can hand the old object out, drop is up to the caller.
            Some(out)
        } else {
            None
        }
    }
}

impl<T: Copy> ObjectPool<T> {
    pub fn get_copied(&self, handle: Handle) -> Option<T> {
        self.get_ref(handle).copied()
    }
}

#[cfg(test)]
mod tests {
    use crate::render::test_utils::DropCanary;
    use crate::render::ObjectPool;
    use std::num::NonZeroU8;

    #[test]
    pub fn test_object_pool_alloc_free() {
        let mut pool = ObjectPool::new(NonZeroU8::MIN);

        let canary = DropCanary::new();
        let handle = pool.alloc(canary.clone());

        assert_eq!(canary.strong_count(), 2);

        pool.free(handle).unwrap();

        assert_eq!(canary.strong_count(), 1);
    }

    #[test]
    pub fn test_object_pool_drop_alloc_data() {
        let mut pool = ObjectPool::new(NonZeroU8::MIN);

        let canary = DropCanary::new();
        let _handle = pool.alloc(canary.clone());

        assert_eq!(canary.strong_count(), 2);

        drop(pool);

        assert_eq!(canary.strong_count(), 1);
    }

    #[test]
    pub fn test_object_pool_double_drop() {
        let mut pool = ObjectPool::new(NonZeroU8::MIN);

        let canary = DropCanary::new();
        let handle = pool.alloc(canary.clone());

        assert_eq!(canary.strong_count(), 2);

        assert!(pool.free(handle).is_some());
        assert_eq!(canary.strong_count(), 1);

        assert!(pool.free(handle).is_none());
        assert_eq!(canary.strong_count(), 1);
    }
}
