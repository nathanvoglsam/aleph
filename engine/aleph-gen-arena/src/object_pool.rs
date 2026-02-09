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

use aleph_alloc::BVec;
use aleph_alloc::allocator_global_handle::AllocatorGlobalHandle;
use allocator_api2::alloc::{Allocator, Global};

use crate::handle::{HandleType, RawHandle};
use crate::handle_pool::HandlePool;

/// A generic generational arena based object pool. Can store objects of type 'T' and associate them
/// with a generational handle value, allowing them to be retrieved with that handle. The handle
/// will become invalid if the object is later removed from the pool, preventing access to unrelated
/// objects that happen to reuse the slot the handle refers to.
///
/// The handle type 'H' is generic, and is intended so handle newtypes can be used so extra type
/// information can be attached to the otherwise POD handles. This can help prevent using handles
/// with the wrong pools.
///
/// This combines [`HandlePool`] with the object managing logic to construct a complete generational
/// arena implementation.
pub struct GenArena<T, H = RawHandle, A: Allocator = Global> {
    /// Backing storage for the object pool. Uses a swap'n'pop scheme for allocating and freeing
    /// new objects as they are needed.
    objects: BVec<T, A>,

    /// SoA storage associated with `Self::objects` that holds whatever handle is currently pointing
    /// at the object in the slot. Enables patching the handles when we move objects around in the
    /// pool.
    back_references: BVec<H, A>,

    /// The handle pool, which we use for allocating the handles themselves. These add an extra
    /// indirection to accessing the object pool items, but it allows us to move the objects
    /// underneath the stable handles. Critical for swap'n'pop! Means we keep our objects densely
    /// packed!
    handles: HandlePool<A>,
}

impl<T, H: HandleType> GenArena<T, H, Global> {
    /// Constructs a new, empty object pool backed by the global allocator
    #[allow(unused)]
    pub fn new() -> Self {
        Self {
            objects: BVec::new(),
            back_references: BVec::new(),
            handles: HandlePool::new(),
        }
    }
}

impl<T, H: HandleType, A: AllocatorGlobalHandle> GenArena<T, H, A> {
    /// Constructs a new, empty object pool backed by a custom allocator that implements
    /// [`AllocatorGlobalHandle`].
    ///
    /// This interface is intended for use with allocator instrumentation to track memory
    /// consumption of different object pools.
    pub fn new_in() -> Self {
        Self {
            objects: BVec::new_in(A::make_handle()),
            back_references: BVec::new_in(A::make_handle()),
            handles: HandlePool::new_in(A::make_handle()),
        }
    }
}

impl<T, H: HandleType, A: Allocator> GenArena<T, H, A> {
    /// Insert the given object of type 'T' into the pool, returning a handle that identifies that
    /// object in the pool. The object may be queried again from the pool with
    /// [`GenArena::get_ref`] or [`GenArena::get_mut`].
    ///
    /// # Panics
    ///
    /// This function may panic if the pool runs out of space for new objects. We only allow
    /// [`u32::MAX`] total objects in a single pool. Exceeding this panics.
    ///
    /// This is very likely to ever happen. Any object of non-trivial size will likely trigger OOM
    /// before that many objects could even be constructed.
    pub fn alloc(&mut self, data: T) -> H {
        let index = u32::try_from(self.objects.len()).expect("Too many objects!");
        let handle = self.handles.alloc(index);
        let handle = unsafe { H::from_bare_handle(handle) };
        self.objects.push(data);
        self.back_references.push(handle);
        handle
    }

    /// Retreive a reference to an object identified by the provided handle. This may return
    /// [`None`] if the handle is no longer valid, such as if the object was removed from the pool
    /// with [`GenArena::free`].
    ///
    /// In general\*, it should not be possible this function to return a reference to an object
    /// other than object that the handle was created with.
    ///
    /// # The asterisk
    ///
    /// It is, strictly, possible for a handle to end up pointing at the wrong object. However, it
    /// requires the handle's slot to be reused [`u32::MAX`] times, overflowing and wrapping back
    /// to the generation value the handle stores. The probability of this happening is
    /// astronomically low. The results are still sound w.r.t. Rust's safety rules too, so it's
    /// not unsafe to allow this to happen.
    pub fn get_ref(&self, handle: H) -> Option<&T> {
        let handle = handle.to_bare_handle();
        if let Some(index) = self.handles.get(handle) {
            Some(&self.objects[index as usize])
        } else {
            None
        }
    }

    /// Retreive a reference to an object identified by the provided handle. This may return
    /// [`None`] if the handle is no longer valid, such as if the object was removed from the pool
    /// with [`GenArena::free`].
    ///
    /// In general\*, it should not be possible this function to return a reference to an object
    /// other than object that the handle was created with.
    ///
    /// # The asterisk
    ///
    /// It is, strictly, possible for a handle to end up pointing at the wrong object. However, it
    /// requires the handle's slot to be reused [`u32::MAX`] times, overflowing and wrapping back
    /// to the generation value the handle stores. The probability of this happening is
    /// astronomically low. The results are still sound w.r.t. Rust's safety rules too, so it's
    /// not unsafe to allow this to happen.
    pub fn get_mut(&mut self, handle: H) -> Option<&mut T> {
        let handle = handle.to_bare_handle();
        if let Some(index) = self.handles.get(handle) {
            Some(&mut self.objects[index as usize])
        } else {
            None
        }
    }

    /// Removes the object associated with the provided handle from the pool. This will invalidate
    /// the handle, take the object from the pool and return it to the caller.
    ///
    /// This may return [`None`] if the handle has been invalidated, likely because another client
    /// of the pool has already freed the handle.
    ///
    /// A successful call to this function will invalidate\* all outstanding handles to the
    /// object those handles were created with.
    ///
    /// # The asterisk
    ///
    /// See [`GenArena::get_ref`].
    pub fn free(&mut self, handle: H) -> Option<T> {
        let handle = handle.to_bare_handle();
        if let Some(index) = self.handles.get(handle) {
            let last_index = self.objects.len() - 1;

            let out = if index as usize != last_index {
                // If the object is _not_ the last then we do the old swap'n pop where we swap with
                // the last element and pop. The only hitch is we need to patch the index stored in
                // the swapped item's handle.

                // Use the handle back-reference to patch the handle of the current last item to
                // point to the place it _will_ be after we do the swap and pop
                let last_item_handle = self.back_references[last_index].to_bare_handle();
                let last_item_index = self.handles.get_mut(last_item_handle).unwrap();
                *last_item_index = index;

                // Perform the swap'n'pop. Whatever was at 'last_index' will now be stored at
                // 'index' and whatever was at 'index' before has been removed from the pool
                self.back_references.swap_remove(index as usize);
                self.objects.swap_remove(index as usize)
            } else {
                // If the object is the last we can just pop it off the end of the pool
                self.back_references.pop();
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

    /// Removes all objects from the pool, invalidates all handles and returns the pool to fully
    /// empty state.
    ///
    /// # Memory
    ///
    /// This does _not_ free any memory. It simply calls 'clear' on all the internal [`Vec`]s
    /// maintained inside the pool.
    pub fn clear(&mut self) {
        self.handles.clear();
        self.back_references.clear();
        self.objects.clear();
    }

    /// Removes all objects from the pool, invalidates all handles and returns the pool to the fully
    /// empty state. Yields each (handle, object) pair stored in the object pool.
    ///
    /// None of the handles yielded by the iterator will be valid, as all handles are immediately
    /// invalidated before returning the iterator to the caller.
    pub fn drain(&mut self) -> impl Iterator<Item = (H, T)> + '_ {
        // This shouldn't be possible
        assert_eq!(self.objects.len(), self.back_references.len());

        // First we clear the handles. None of the handles we emit when iterating will be valid, but
        // they may still be useful to yield
        self.handles.clear();

        // Drain our back_references and objects storage, yielding the appropriate iterator
        let back_references = self.back_references.drain(..);
        let objects = self.objects.drain(..);
        back_references.zip(objects)
    }
}

#[cfg(test)]
mod tests {
    use crate::{GenArena, RawHandle};

    /// Shorthand wrapper over an [`std::rc::Rc`] that we use to track if objects stored inside
    /// containers are being dropped correctly. We don't care about what's being stored, only that the
    /// reference count is maintained correctly.
    #[cfg(test)]
    #[derive(Clone)]
    pub struct DropCanary(std::rc::Rc<()>);

    #[cfg(test)]
    impl DropCanary {
        pub fn new() -> Self {
            Self(std::rc::Rc::new(()))
        }

        pub fn strong_count(&self) -> usize {
            std::rc::Rc::strong_count(&self.0)
        }
    }

    #[test]
    pub fn test_object_pool_alloc_free() {
        let mut pool = GenArena::new();

        let canary = DropCanary::new();
        let handle: RawHandle = pool.alloc(canary.clone());

        assert_eq!(canary.strong_count(), 2);

        pool.free(handle).unwrap();

        assert_eq!(canary.strong_count(), 1);
    }

    #[test]
    pub fn test_object_pool_drop_alloc_data() {
        let mut pool = GenArena::new();

        let canary = DropCanary::new();
        let _handle: RawHandle = pool.alloc(canary.clone());

        assert_eq!(canary.strong_count(), 2);

        drop(pool);

        assert_eq!(canary.strong_count(), 1);
    }

    #[test]
    pub fn test_object_pool_double_drop() {
        let mut pool = GenArena::new();

        let canary = DropCanary::new();
        let handle: RawHandle = pool.alloc(canary.clone());

        assert_eq!(canary.strong_count(), 2);

        assert!(pool.free(handle).is_some());
        assert_eq!(canary.strong_count(), 1);

        assert!(pool.free(handle).is_none());
        assert_eq!(canary.strong_count(), 1);
    }
}
