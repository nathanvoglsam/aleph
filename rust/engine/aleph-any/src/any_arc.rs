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

use crate::{IAny, TraitObject};
use std::any::TypeId;
use std::mem::ManuallyDrop;
use std::ops::Deref;
use std::ptr::NonNull;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

/// Internal struct for creating the heap storage for the object behind the refcount
struct AnyArcInner<T: IAny + ?Sized> {
    /// The reference count
    count: AtomicUsize,

    /// The object we're storing. The outermost refcount decides when we drop this so we wrap it in
    /// ManuallyDrop so we just use `Box's` drop function to free the memory
    object: ManuallyDrop<T>,
}

/// Represents a custom `Arc` like smart pointer that allows for easy ref counted access to the same
/// object through multiple interfaces.
pub struct AnyArc<T: IAny + ?Sized> {
    inner: NonNull<AnyArcInner<T>>,
}

impl<T: IAny + Sized> AnyArc<T> {
    /// Takes the given concrete type and wraps it in an `AnyArc`
    ///
    /// `T` must be sized in this case as we're making a concrete heap allocation
    pub fn new(v: T) -> AnyArc<T> {
        let inner: Box<_> = Box::new(AnyArcInner {
            count: AtomicUsize::new(1),
            object: ManuallyDrop::new(v),
        });
        let inner = Box::leak(inner);

        AnyArc::<T> {
            inner: inner.into(),
        }
    }
}

impl<T: IAny + ?Sized> AnyArc<T> {
    pub fn get_mut(&mut self) -> Option<&mut T> {
        // If there is only a single reference to the underlying object then it is safe to hand out
        // a mutable reference to the underlying object.
        //
        // By requiring mutable access to the `AnyArc` container we prevent any more references
        // being created until the mutable borrow ends making this safe to do.
        if self.inner().count.load(Ordering::Relaxed) == 1 {
            Some(&mut self.inner_mut().object)
        } else {
            None
        }
    }

    pub fn query_interface<Into: IAny + ?Sized>(&self) -> Option<AnyArc<Into>> {
        unsafe {
            // Lookup whether the underlying object implements the requested interface
            if let Some(casted) = self.inner().object.__query_interface(TypeId::of::<Into>()) {
                // Increment the ref count
                let _sink = self.inner().count.fetch_add(1, Ordering::Relaxed);

                // We build a trait object with a potentially null vtable and use some pointer copy
                // wizardry to allow this to work for sized and unsized types
                let arc_trait_object = TraitObject::<'static> {
                    data: self.inner.cast(),
                    vtable: casted.vtable,
                    phantom: Default::default(),
                };

                // We reinterpret the pointer to our unpacked `TraitObject` as a pointer to an
                // `AnyArc`.
                //
                // When `T` is unsized then size_of `AnyArc<T>` is 16 bytes (2 * usize).
                //
                // When `T` is sized then size_of `AnyArc<T>` is 8 bytes (single pointer).
                //
                // `AnyArc<T>` has the same layout as `AnyRef<T>` so we can use the same pointer
                // casting trick to transmute to either the sized or unsized versions.
                //
                // We just need to do a C style pointer cast to reinterpret rather than using
                // transmute
                let out_ptr = &arc_trait_object as *const TraitObject as *const AnyArc<Into>;

                // Now we can just read out the reinterpreted data
                Some(out_ptr.read())
            } else {
                None
            }
        }
    }

    fn inner(&self) -> &AnyArcInner<T> {
        // This unsafety is ok because while this arc is alive we're guaranteed
        // that the inner pointer is valid. Furthermore, we know that the
        // `ArcInner` structure itself is `Sync` because the inner data is
        // `Sync` as well, so we're ok loaning out an immutable pointer to these
        // contents.
        unsafe { self.inner.as_ref() }
    }

    fn inner_mut(&mut self) -> &mut AnyArcInner<T> {
        // This unsafety is ok because while this arc is alive we're guaranteed
        // that the inner pointer is valid. Furthermore, we know that the
        // `ArcInner` structure itself is `Sync` because the inner data is
        // `Sync` as well, so we're ok loaning out an immutable pointer to these
        // contents.
        unsafe { self.inner.as_mut() }
    }
}

impl<T: IAny + ?Sized> Clone for AnyArc<T> {
    fn clone(&self) -> Self {
        let _ = self.inner().count.fetch_add(1, Ordering::Relaxed);
        Self { inner: self.inner }
    }
}

impl<T: IAny + ?Sized> Drop for AnyArc<T> {
    fn drop(&mut self) {
        unsafe {
            // If this yields 1 that means the last `AnyArc` that points to the underlying object is
            // being dropped, so we should Drop the object and free the memory.
            if self.inner().count.fetch_sub(1, Ordering::Release) == 1 {
                // Atomic synchronization stuff I stole from std::arc::Arc
                let _sink = self.inner().count.load(Ordering::Acquire);

                ManuallyDrop::drop(&mut self.inner_mut().object);

                // "un-leak" the box and drop it to free the memory
                drop(Box::from_raw(self.inner.as_ptr()))
            }
        }
    }
}

impl<T: IAny + ?Sized> Deref for AnyArc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner().object
    }
}
