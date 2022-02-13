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

use crate::MAX_REFCOUNT;
use std::any::TypeId;
use std::marker::PhantomData;
use std::mem::ManuallyDrop;
use std::ptr::NonNull;
use std::sync::atomic::{AtomicUsize, Ordering};

/// Internal struct used for describing the layout of the intrusive ref count data
#[repr(C)]
pub struct IntrusiveData<T> {
    pub(crate) count: AtomicUsize,
    table: &'static Table,
    phantom: PhantomData<T>,
}

impl<T> IntrusiveData<T> {
    /// Internal function for constructing a new IntrusiveData instance
    ///
    /// # Safety
    ///
    /// This is marked unsafe as it should only be used as an API internal. It is a measure to
    /// prevent creating stack allocated RefPtrObject objects by making it impossible to construct
    /// the `__internal_intrusive` member with safe code.
    pub unsafe fn new(table: &'static Table) -> Self {
        Self {
            count: AtomicUsize::new(1),
            table,
            phantom: Default::default(),
        }
    }

    /// Implementation detail for accessing the v-table
    ///
    /// # Safety
    ///
    /// Any access to the v-table is probably UB, don't do it.
    #[inline(always)]
    pub fn table(&self) -> &'static Table {
        self.table
    }
}

/// Internal struct used for describing the layout of a RefPtr vtable (not a rust trait object
/// vtable)
#[repr(C)]
#[derive(Clone)]
pub struct Table {
    pub this_id: TypeId,
    pub size: usize,
    pub align: usize,
    pub drop: unsafe fn(NonNull<()>, NonNull<()>),
    pub dealloc: unsafe fn(NonNull<()>),
    pub table: &'static [PointerTypePair],
}

impl Table {
    /// Initializes a new Table for the given type, leaving the internal vtable empty for the caller
    /// too manually populate.
    ///
    /// # Safety
    ///
    /// This is marked unsafe as it should only be used as an API internal. It is a measure to
    /// prevent creating stack allocated RefPtrObject objects by making it impossible to construct
    /// the `__internal_intrusive` member with safe code.
    pub unsafe fn new_for<T: Sized + 'static>(table: &'static [PointerTypePair]) -> Self {
        unsafe fn drop_fn<X: Sized + 'static>(object: NonNull<()>, intrusive: NonNull<()>) {
            // Push the reference count past the maximum value. Because of the use of an intrusive
            // refcount it is possible for a user to try and 'revive' the object while inside of
            // its Drop impl. This would allow use after free in safe code.
            //
            // To prevent this we set the refcount to be > MAX_REFCOUNT, which means if the user
            // attempts to 'revive' the object then the program will abort as per the implementation
            // of RefPtr::clone. This demotes a use after free to an abort, which is safe. Only
            // degenerate programs will trigger this behavior. We do not want to support this.
            let intrusive = intrusive.cast::<IntrusiveData<X>>();
            intrusive
                .as_ref()
                .count
                .store(MAX_REFCOUNT + 1, Ordering::Relaxed);

            // Call the object's Drop impl, now in a state where we can't trigger use-after-free
            let object = object.cast::<X>();
            core::ptr::drop_in_place(object.as_ptr());
        }

        unsafe fn dealloc_fn<X: Sized + 'static>(object: NonNull<()>) {
            drop(Box::<ManuallyDrop<X>>::from_raw(object.cast().as_ptr()))
        }

        Self {
            this_id: TypeId::of::<T>(),
            size: core::mem::size_of::<T>(),
            align: core::mem::align_of::<T>(),
            drop: drop_fn::<T>,
            dealloc: dealloc_fn::<T>,
            table,
        }
    }

    /// Internal function used for finding a v-pointer inside a RefPtr vtable for a given type-id.
    pub fn query_vtable(&self, id: TypeId) -> Option<NonNull<()>> {
        for pair in self.table {
            if pair.id == id {
                return Some(pair.ptr);
            }
        }

        None
    }
}

/// Internal (type-id + v-pointer) pair
#[repr(C)]
pub struct PointerTypePair {
    /// The TypeId of some trait that this pair is providing the v-ptr for
    pub id: TypeId,

    /// The v-ptr for the trait identified by id
    pub ptr: NonNull<()>,
}

impl PointerTypePair {
    /// Constructs a new PointerTypePair from the given parts
    pub const fn new(id: TypeId, ptr: NonNull<()>) -> Self {
        Self { id, ptr }
    }
}

// PointerTypePair is POD. PointerTypePair.ptr is a pointer to 'static data and is only ever used
// immutably
unsafe impl Send for PointerTypePair {}
unsafe impl Sync for PointerTypePair {}

/// Internal representation of a rust fat pointer
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TraitObject {
    pub ptr: NonNull<()>,
    pub vtable: NonNull<()>,
}
