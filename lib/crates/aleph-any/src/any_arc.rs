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
use std::ops::Deref;
use std::ptr::NonNull;
use std::sync::{Arc, Weak};

///
/// AnyArc is a wrapper around [`std::sync::Arc`] that enables the ability to cast
/// `AnyArc<Trait> -> AnyArc<AnotherTrait>` so long as the underlying object supports both traits.
///
#[repr(transparent)]
pub struct AnyArc<T: IAny + ?Sized>(Arc<T>);

impl<T: IAny + Sized> AnyArc<T> {
    ///
    /// Takes the given concrete type and wraps it in an `AnyArc`
    ///
    /// `T` must be sized in this case as we're making a concrete heap allocation
    #[inline]
    pub fn new(v: T) -> AnyArc<T> {
        AnyArc::<T>(Arc::new(v))
    }

    ///
    /// Wrapper around [Arc::new_cyclic]
    ///
    #[inline]
    pub fn new_cyclic(data_fn: impl FnOnce(&AnyWeak<T>) -> T) -> AnyArc<T> {
        AnyArc::<T>(Arc::new_cyclic(|v| {
            // SAFETY: AnyWeak is a repr(transparent) alias for Weak so this is perfectly safe to
            //         do. The compiler just can't prove it.
            unsafe { data_fn(core::mem::transmute(v)) }
        }))
    }

    ///
    /// Takes the given `AnyArc` and converts it into a `AnyArc<dyn IAny` without going through
    /// `query_interface`
    ///
    #[inline]
    pub fn into_any(v: Self) -> AnyArc<dyn IAny> {
        let inner: Arc<dyn IAny> = v.0;
        AnyArc::from_arc(inner)
    }
}

impl<T: IAny + Send + Sized> AnyArc<T> {
    ///
    /// Takes the given `AnyArc` and converts it into a `AnyArc<dyn ISendSyncAny>` without going
    /// through `query_interface`
    ///
    #[inline]
    pub fn into_send_any(v: Self) -> AnyArc<dyn IAny + Send> {
        let inner: Arc<dyn IAny + Send> = v.0;
        AnyArc::from_arc(inner)
    }
}

impl<T: IAny + Send + Sync + Sized> AnyArc<T> {
    ///
    /// Takes the given `AnyArc` and converts it into a `AnyArc<dyn ISendSyncAny>` without going
    /// through `query_interface`
    ///
    #[inline]
    pub fn into_send_sync_any(v: Self) -> AnyArc<dyn IAny + Send + Sync> {
        let inner: Arc<dyn IAny + Send + Sync> = v.0;
        AnyArc::from_arc(inner)
    }
}

impl<T: IAny + ?Sized> AnyArc<T> {
    ///
    /// Construct an `AnyArc` from a `std::sync::Arc`.
    ///
    /// Useful for when you want to get access to the `query_interface` wrapper with a standard
    /// `Arc`.
    ///
    #[inline]
    pub fn from_arc(arc: Arc<T>) -> Self {
        Self(arc)
    }

    ///
    /// Unwraps the given `AnyArc` to the underlying `Arc` inside.
    ///
    /// Useful for when you want to access to things `Arc` provides like `CoerceUnsized` which isn't
    /// stabilized yet and so can't be implemented on `AnyArc` yet.
    ///
    #[inline]
    pub fn into_arc(this: Self) -> Arc<T> {
        this.0
    }

    ///
    /// Creates a new [`AnyWeak`] pointer to this allocation.
    ///
    /// # Info
    ///
    /// This is just a wrapper around `std::sync::Arc`'s `downgrade`
    ///
    #[inline]
    pub fn downgrade(this: &Self) -> AnyWeak<T> {
        AnyWeak(Arc::downgrade(&this.0))
    }

    ///
    /// Gets the number of [`AnyWeak`] pointers to this allocation.
    ///
    /// # Safety
    ///
    /// This method by itself is safe, but using it correctly requires extra care.
    /// Another thread can change the weak count at any time,
    /// including potentially between calling this method and acting on the result.
    ///
    /// # Info
    ///
    /// This is just a wrapper around `std::sync::Arc`'s `weak_count`
    ///
    #[inline]
    pub fn weak_count(this: &Self) -> usize {
        Arc::weak_count(&this.0)
    }

    ///
    /// Gets the number of strong (`AnyArc`) pointers to this allocation.
    ///
    /// # Safety
    ///
    /// This method by itself is safe, but using it correctly requires extra care.
    /// Another thread can change the strong count at any time,
    /// including potentially between calling this method and acting on the result.
    ///
    /// # Info
    ///
    /// This is just a wrapper around `std::sync::Arc`'s `strong_count`
    ///
    #[inline]
    pub fn strong_count(this: &Self) -> usize {
        Arc::strong_count(&this.0)
    }

    ///
    /// Returns a mutable reference into the given `AnyArc`, if there are
    /// no other `AnyArc` or [`AnyWeak`] pointers to the same allocation.
    ///
    /// Returns [`None`] otherwise, because it is not safe to
    /// mutate a shared value.
    ///
    /// # Info
    ///
    /// This is just a wrapper around `std::sync::Arc`'s `get_mut`
    ///
    #[inline]
    pub fn get_mut(this: &mut Self) -> Option<&mut T> {
        Arc::get_mut(&mut this.0)
    }

    ///
    /// Returns another `AnyArc` to the underlying object but with the `Into` interface. This
    /// function enables casting from one trait object type to another.
    ///
    /// Returns [`None`] if the underlying object does not implement the requested interface.
    ///
    pub fn query_interface<Into: IAny + ?Sized>(&self) -> Option<AnyArc<Into>> {
        unsafe {
            // Lookup whether the underlying object implements the requested interface
            if let Some(casted) = self.0.__query_interface(TypeId::of::<Into>()) {
                // Clone the internal Arc
                let cloned = self.0.clone();

                // We need to do some pointer casting stuff to get the pointer to the control block
                // without using transmute
                let cloned_ptr = &cloned as *const Arc<T> as *const *mut ();
                let cloned_ptr = cloned_ptr.read();

                // We build a trait object with a potentially null vtable and use some pointer copy
                let queried_trait_object = TraitObject::<'static> {
                    data: NonNull::new_unchecked(cloned_ptr),
                    vtable: casted.vtable,
                    phantom: Default::default(),
                };

                // We reinterpret the pointer to our unpacked `TraitObject` as a pointer to an
                // `Arc`.
                //
                // When `T` is unsized then size_of `Arc<T>` is 16 bytes (2 * usize).
                //
                // When `T` is sized then size_of `Arc<T>` is 8 bytes (single pointer).
                //
                // `Arc<T>` has the same layout as `AnyRef<T>` so we can use the same pointer
                // casting trick to transmute to either the sized or unsized versions.
                //
                // We just need to do a C style pointer cast to reinterpret rather than using
                // transmute
                let out_ptr = &queried_trait_object as *const TraitObject as *const Arc<Into>;
                let out = AnyArc(out_ptr.read());

                // Forget our original clone so we don't decrement the ref counter
                std::mem::forget(cloned);

                // Now we can just read out the reinterpreted data
                Some(out)
            } else {
                None
            }
        }
    }
}

impl<T: IAny + Default> Default for AnyArc<T> {
    #[inline]
    fn default() -> Self {
        Self(Arc::new(Default::default()))
    }
}

impl<T: IAny + ?Sized> Clone for AnyArc<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: IAny + ?Sized> Deref for AnyArc<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

#[repr(transparent)]
pub struct AnyWeak<T: IAny + ?Sized>(Weak<T>);

impl<T: IAny + ?Sized> AnyWeak<T> {
    ///
    /// Attempts to upgrade the `AnyWeak` pointer to an [`AnyArc`], delaying
    /// dropping of the inner value if successful.
    ///
    /// Returns [`None`] if the inner value has since been dropped.
    ///
    /// # Info
    ///
    /// This is just a wrapper around `std::sync::Weak`'s `upgrade`
    ///
    #[inline]
    pub fn upgrade(&self) -> Option<AnyArc<T>> {
        self.0.upgrade().map(|v| AnyArc(v))
    }

    ///
    /// Gets the number of strong (`Arc`) pointers pointing to this allocation.
    ///
    /// # Info
    ///
    /// This is just a wrapper around `std::sync::Weak`'s `strong_count`
    ///
    #[inline]
    pub fn strong_count(&self) -> usize {
        self.0.strong_count()
    }

    ///
    /// Gets an approximation of the number of `Weak` pointers pointing to this
    /// allocation.
    ///
    /// # Accuracy
    ///
    /// Due to implementation details, the returned value can be off by 1 in
    /// either direction when other threads are manipulating any `Arc`s or
    /// `Weak`s pointing to the same allocation.
    ///
    /// # Info
    ///
    /// This is just a wrapper around `std::sync::Weak`'s `weak_count`
    ///
    #[inline]
    pub fn weak_count(&self) -> usize {
        self.0.weak_count()
    }
}

impl<T: IAny + ?Sized> Clone for AnyWeak<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
