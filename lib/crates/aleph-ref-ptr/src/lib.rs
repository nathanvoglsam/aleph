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

pub extern crate once_cell;

pub mod layout;

#[cfg(test)]
mod tests;

use crate::layout::{IntrusiveData, TraitObject};
use std::any::TypeId;
use std::marker::PhantomData;
use std::ops::Deref;
use std::process::abort;
use std::ptr::NonNull;
use std::sync::atomic::Ordering;

///
/// RefPtr is a smart pointer type that wraps an intrusively ref-counted, heap allocated object.
///
/// RefPtr also wraps a custom v-table format that allows casting trait objects to other trait
/// objects directly without casting to a concrete type.
///
#[repr(transparent)]
pub struct RefPtr<T: ?Sized> {
    object: TraitObject,
    phantom: PhantomData<T>,
}

/// The maximum value possible for the reference count of a RefPtr
pub(crate) const MAX_REFCOUNT: usize = (isize::MAX) as usize;

impl<T: ?Sized> RefPtr<T> {
    /// Returns a new RefPtr that views the same underlying object through a different interface
    #[inline]
    pub fn query_interface<X: ?Sized + 'static>(&self) -> Option<RefPtr<X>> {
        unsafe {
            let id = TypeId::of::<X>();
            let vtable = self.inner().table().query_vtable(id);
            vtable.map(|v| {
                let mut cloned = self.clone();
                cloned.object.vtable = v;

                core::mem::transmute(cloned)
            })
        }
    }
}

impl<T: ?Sized> RefPtr<T> {
    /// Internal function for creating a RefPtr from a trait object
    #[inline(always)]
    unsafe fn from_inner(object: TraitObject) -> Self {
        Self {
            object,
            phantom: PhantomData,
        }
    }

    /// Internal function for getting a reference to the IntrusiveData in the object pointed to by
    /// the RefPtr
    #[inline(always)]
    fn inner(&self) -> &IntrusiveData<()> {
        // Safety: RefPtr assumes that all objects it points to contains an IntrusiveData field as
        //         the very first field in the object. It is not possible to construct a RefPtr
        //         that violates this requirement with *safe* rust.
        //
        //         This code is safe because it requires other unsafe code to break.
        unsafe { self.object.ptr.cast::<IntrusiveData<()>>().as_ref() }
    }
}

impl<T: RefPtrObject> RefPtr<T> {
    /// Creates a new RefPtr object based on the given `Initializer`
    pub fn new(initializer: T::Initializer) -> RefPtr<T> {
        T::make_ref_ptr(initializer)
    }
}

impl<T: ?Sized> Deref for RefPtr<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe {
            // Get a pointer to the trait object
            let ptr = &self.object as *const TraitObject;
            let ptr = ptr as *const *const T;
            ptr.read().as_ref().unwrap()
        }
    }
}

impl<T: ?Sized> Clone for RefPtr<T> {
    fn clone(&self) -> Self {
        // Using a relaxed ordering is alright here, as knowledge of the
        // original reference prevents other threads from erroneously deleting
        // the object.
        //
        // As explained in the [Boost documentation][1], Increasing the
        // reference counter can always be done with memory_order_relaxed: New
        // references to an object can only be formed from an existing
        // reference, and passing an existing reference from one thread to
        // another must already provide any required synchronization.
        //
        // [1]: (www.boost.org/doc/libs/1_55_0/doc/html/atomic/usage_examples.html)
        //
        // This is one of the only safe places to access the ref count
        let old_size = self.inner().count.fetch_add(1, Ordering::Relaxed);

        // However we need to guard against massive refcounts in case someone
        // is `mem::forget`ing Arcs. If we don't do this the count can overflow
        // and users will use-after free. We racily saturate to `isize::MAX` on
        // the assumption that there aren't ~2 billion threads incrementing
        // the reference count at once. This branch will never be taken in
        // any realistic program.
        //
        // We abort because such a program is incredibly degenerate, and we
        // don't care to support it.
        if old_size > MAX_REFCOUNT {
            abort();
        }

        unsafe { Self::from_inner(self.object) }
    }
}

impl<T: ?Sized> Drop for RefPtr<T> {
    fn drop(&mut self) {
        // Because `fetch_sub` is already atomic, we do not need to synchronize
        // with other threads unless we are going to delete the object. This
        // same logic applies to the below `fetch_sub` to the `weak` count.
        if self.inner().count.fetch_sub(1, Ordering::Release) != 1 {
            return;
        }

        // This fence is needed to prevent reordering of use of the data and
        // deletion of the data.  Because it is marked `Release`, the decreasing
        // of the reference count synchronizes with this `Acquire` fence. This
        // means that use of the data happens before decreasing the reference
        // count, which happens before this fence, which happens before the
        // deletion of the data.
        //
        // As explained in the [Boost documentation][1],
        //
        // > It is important to enforce any possible access to the object in one
        // > thread (through an existing reference) to *happen before* deleting
        // > the object in a different thread. This is achieved by a "release"
        // > operation after dropping a reference (any access to the object
        // > through this reference must obviously happened before), and an
        // > "acquire" operation before deleting the object.
        //
        // In particular, while the contents of an Arc are usually immutable, it's
        // possible to have interior writes to something like a Mutex<T>. Since a
        // Mutex is not acquired when it is deleted, we can't rely on its
        // synchronization logic to make writes in thread A visible to a destructor
        // running in thread B.
        //
        // Also note that the Acquire fence here could probably be replaced with an
        // Acquire load, which could improve performance in highly-contended
        // situations. See [2].
        //
        // [1]: (www.boost.org/doc/libs/1_55_0/doc/html/atomic/usage_examples.html)
        // [2]: (https://github.com/rust-lang/rust/pull/41714)
        core::sync::atomic::fence(Ordering::Acquire);

        unsafe {
            // This is 100% intentional. We *must* capture a reference to the table as it will be
            // UB to access again after we call the Drop implementation
            let table = self.inner().table();

            // Call the drop implementation on the underlying object
            let object = self.object.ptr;
            let intrusive = NonNull::from(self.inner());
            (table.drop)(object, intrusive.cast());

            // Drop the actual allocation
            (table.dealloc)(object);
        }
    }
}

unsafe impl<T: ?Sized + Sync + Send> Send for RefPtr<T> {}
unsafe impl<T: ?Sized + Sync + Send> Sync for RefPtr<T> {}

/// A semi-internal trait used for marking types that have a RefPtr compatible v-table available.
///
/// This trait must be implemented on an object for it to be stored in a RefPtr.
///
/// # Safety
///
/// This trait is very unsafe. RefPtr relies on the memory layout of [Table] as well as the object
/// it is pointing to. There is no way to verify the v-table provided is valid.
pub unsafe trait RefPtrObject: Sized + 'static {
    /// Associated type for a struct that is used for populating the fields of a [RefPtrObject]
    /// implementation with [RefOtrObject::make_ref_ptr].
    type Initializer: Sized;

    /// Creates a new RefPtr<Self> that points to `self`. This allows promoting bare references to
    /// the underlying object to a [RefPtr].
    fn as_ref_ptr(&self) -> RefPtr<Self>;

    /// The constructor implementation for constructing a RefPtr<Self> from an initializer struct.
    ///
    /// This is the only way to safely initialize a RefPtr with a new object.
    fn make_ref_ptr(initializer: Self::Initializer) -> RefPtr<Self>;

    /// This function constructs a fully initialized IntrusiveData instance for the implementing
    /// type.
    ///
    /// # Safety
    ///
    /// This function is not safe to call directly because it would allow initializing a
    /// RefPtrObject manually, on the stack. All RefPtrObject objects must be heap allocated for the
    /// ref counting to work.
    ///
    /// By making this unsafe it becomes impossible
    unsafe fn new_intrusive() -> IntrusiveData<Self>;
}

#[macro_export]
macro_rules! ref_ptr_object {
    (
        $obj_vis:vis struct $obj_name:ident $(: $($impl_name:ident),* $(,)?)? {
            $($field_vis:vis $field_name:ident : $field_type:ty),* $(,)?
        }
    ) => {
        #[repr(C)]
        $obj_vis struct $obj_name {
            __internal_intrusive: $crate::layout::IntrusiveData::<$obj_name>,
            $($field_vis $field_name : $field_type),*
        }

        pub struct Initializer {
            $(pub $field_name : $field_type),*
        }

        unsafe impl $crate::RefPtrObject for $obj_name {
            type Initializer = Initializer;

            fn as_ref_ptr(&self) -> $crate::RefPtr<$obj_name> {
                use std::ptr::NonNull;

                unsafe {
                    let ptr = NonNull::from(self).cast();
                    let object = crate::layout::TraitObject {
                        ptr,
                        vtable: NonNull::dangling(),
                    };
                    let object = core::mem::transmute::<_, $crate::RefPtr<$obj_name>>(object);
                    let out = object.clone();
                    core::mem::forget(object);
                    out
                }

            }

            fn make_ref_ptr(v: Self::Initializer) -> $crate::RefPtr<$obj_name> {
                use std::mem::ManuallyDrop;

                let obj = $obj_name {
                    __internal_intrusive: unsafe { Test::new_intrusive() },
                    $($field_name : v.$field_name),*
                };
                let obj = ManuallyDrop::new(obj);

                unsafe {
                    use std::ptr::NonNull;

                    let ptr = Box::leak(Box::<ManuallyDrop<$obj_name>>::new(obj));
                    let ptr = NonNull::from(ptr).cast::<()>();

                    let object = crate::layout::TraitObject {
                        ptr,
                        vtable: NonNull::dangling(),
                    };

                    core::mem::transmute(object)
                }
            }

            unsafe fn new_intrusive() -> $crate::layout::IntrusiveData<$obj_name> {
                use $crate::layout::{PointerTypePair, Table};
                use $crate::once_cell::race::OnceBox;
                use $crate::once_cell::sync::OnceCell;

                static VIRTUAL_TABLE: OnceCell<Vec<PointerTypePair>> = OnceCell::new();
                static TABLE: OnceBox<Table> = OnceBox::new();

                let table = TABLE.get_or_init(|| {
                    let table = VIRTUAL_TABLE.get_or_init(|| {
                        #[allow(unused_mut)]
                        let mut table = Vec::new();

                        $(
                        $(
                        unsafe {
                            use $crate::layout::TraitObject;
                            use std::ptr::NonNull;
                            use std::any::TypeId;

                            let trait_object = NonNull::<$obj_name>::dangling().as_ref() as &dyn $impl_name;
                            let trait_object = &trait_object as *const &dyn $impl_name;
                            let trait_object = trait_object as *const TraitObject;
                            let trait_object = trait_object.read();

                            let id = TypeId::of::<dyn $impl_name>();
                            let ptr = trait_object.vtable;
                            let pair = PointerTypePair::new(id, ptr);
                            table.push(pair);
                        }
                        )*
                        )?

                        table
                    });

                    let table = Table::new_for::<$obj_name>(table.as_slice());
                    Box::new(table)
                });
                $crate::layout::IntrusiveData::<$obj_name>::new(table)
            }
        }
    };
}

#[macro_export]
macro_rules! ref_ptr_init {
    (
        $obj_name:ident {
            $($field_name:ident : $source_expr:expr),* $(,)?
        }
    ) => {
        {
            type Init = <$obj_name as $crate::RefPtrObject>::Initializer;
            Init {
                $($field_name : $source_expr),*
            }
        }
    }
}
