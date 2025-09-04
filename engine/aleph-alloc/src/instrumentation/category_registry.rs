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

use std::collections::HashMap;
use std::ptr::NonNull;
use std::sync::LazyLock;
use std::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};

use aleph_nstr::NStr;

/// Trait associated with any type that uniquely identifies an allocation category.
pub unsafe trait IAllocationCategory: Sized {
    /// A stable, globally unique UUID. This must forever be stable and must always uniquely
    /// identify the category. It is the implementors responsibility to uphold the uniqueness and
    /// stability guarantees of this interface.
    const ID: uuid::Uuid;

    /// A name that can be used to identify the category. This name is not guaranteed to uniquely
    /// identify the type, only the ID may do that. This name should only be used for logging or
    /// other human visible use cases.
    const NAME: &'static NStr;

    /// A static reference to an [`CategoryInfo`] instance that describes the
    /// [`IAllocationCategory`].
    fn info() -> &'static CategoryInfo;
}

/// Utility function that allows checking if two different category types are the same, in a const
/// context.
pub const fn is_same_category<A: IAllocationCategory, B: IAllocationCategory>() -> bool {
    A::ID.as_u128() == B::ID.as_u128()
}

/// FFI portable category description table. Contains all the information exposed by
/// [`IAllocationCategory`] wrapped in a neat little struct that can be safely sent across FFI
/// boundaries.
#[repr(C)]
pub struct CategoryInfo {
    /// Type UUID that uniquely, globally identifies the category
    pub id: uuid::Uuid,

    /// Human-readable name of the category. Not guaranteed to be unique.
    pub name: &'static NStr,

    /// The number of bytes allocated into this category, as tracked by all tagged allocations.
    pub bytes_allocated: AtomicUsize,

    /// Pointer used to form a linked list of all registered [`CategoryInfo`] objects.
    ///
    /// This is an internal field and should never be accessed directly, but needs to be made
    /// public for macros to work.
    #[doc(hidden)]
    pub next: AtomicPtr<CategoryInfo>,
}

impl CategoryInfo {
    /// Constructs a [`CategoryData`] for a given type `T`
    #[inline(always)]
    pub fn get<T: IAllocationCategory>() -> &'static Self {
        T::info()
    }

    /// Utility for grabbing the number of bytes allocated
    #[inline(always)]
    pub fn allocated(&self) -> usize {
        self.bytes_allocated.load(Ordering::Relaxed)
    }
}

/// This macro can be used to implement [`IAllocationCategory`] on an object as a shorthand compared
/// to manually implementing it directly. This will correctly generate a safe implementation of
/// [`IAllocationCategory`] for the given type.
#[macro_export]
macro_rules! new_alloc_category {
    ($t: path, $id: literal) => {
        unsafe impl $crate::instrumentation::IAllocationCategory for $t {
            const ID: $crate::uuid::Uuid = $crate::uuid::uuid!($id);
            const NAME: &'static $crate::nstr::NStr = $crate::nstr::nstr!(stringify!($t));

            #[inline(always)]
            fn info() -> &'static $crate::instrumentation::CategoryInfo {
                #[$crate::ctor::ctor(crate_path = $crate::ctor)]
                fn internal_register_t() {
                    unsafe {
                        $crate::instrumentation::register_category(
                            <$t as $crate::instrumentation::IAllocationCategory>::info(),
                        );
                    }
                }

                static INFO: $crate::instrumentation::CategoryInfo =
                    $crate::instrumentation::CategoryInfo {
                        id: <$t as $crate::instrumentation::IAllocationCategory>::ID,
                        name: <$t as $crate::instrumentation::IAllocationCategory>::NAME,
                        bytes_allocated: ::std::sync::atomic::AtomicUsize::new(0),
                        next: ::std::sync::atomic::AtomicPtr::new(::std::ptr::null_mut()),
                    };
                &INFO
            }
        }
    };
}

/// A lazily initialized table of all types registered into the object system.
///
/// This is dervied from [`AllocationCategoryIter`] by using it to walk the internal type list. This
/// is the preferred API of [`AllocationCategoryIter`] as after the initial setup the hash map is
/// much more efficient to query.
pub static CATEGORIES: LazyLock<hashbrown::HashMap<uuid::Uuid, &'static CategoryInfo>> =
    LazyLock::new(|| {
        assert_no_duplicate_ids_registered();
        let map = hashbrown::HashMap::from_iter(AllocationCategoryIter::new().map(|v| (v.id, v)));
        map
    });

/// An object that integrates with the category system to iterate over all declared category types.
///
/// The list is filled out using '__attribute__((constructor))' functions generated by the macros.
/// All the list setup is done before main and so once main is entered the list will always be
/// available.
///
/// # Performance
///
/// In general please use [`CATEGORIES`]. This directly walks the linked list that gets built before
/// main. While this is safe, it's not particularly fast. [`CATEGORIES`] is much more efficient to
/// iterate.
pub struct AllocationCategoryIter {
    next: *mut CategoryInfo,
}

impl AllocationCategoryIter {
    /// Constructs a new [`AllocationCategoryIter`] instance.
    pub fn new() -> Self {
        // All accesses to the list pointer must leave it as null or a valid static pointer to a
        // 'AllocationCategoryIter' instance. All access to the head pointer is unsafe gated and so
        // it's impossible to break this expectation without other unsafe code.
        let next = CATEGORY_LIST_HEAD.load(Ordering::Relaxed);
        Self { next }
    }
}

impl Iterator for AllocationCategoryIter {
    type Item = &'static CategoryInfo;

    fn next(&mut self) -> Option<Self::Item> {
        let ptr = self.next;
        if ptr.is_null() {
            None
        } else {
            unsafe {
                let out: &'static CategoryInfo = ptr.as_ref().unwrap_unchecked();
                self.next = out.next.load(Ordering::Relaxed);
                Some(out)
            }
        }
    }
}

/// Utility function that will walk the list of registered types and assert if there are any
/// duplicate type IDs registered.
pub fn assert_no_duplicate_ids_registered() {
    let mut types = HashMap::new();
    for category in AllocationCategoryIter::new() {
        let existing = types.insert(category.id, category);
        if let Some(existing) = existing {
            assert_eq!(category.id, existing.id); // Just being careful
            panic!(
                "Colliding IAllocationCategory type IDs detected. '{}' and '{}' have the same ID of '{}'!",
                existing.name, category.name, existing.id
            );
        }
    }
}

/// Super-duper ultra unsafe do not access but needs to be public so macros can touch it.
///
/// Forms the head of the linked list of all declared category types. Will be setup before main.
///
/// Call to get a reference to the head of the list. Unsafe because if you call this outside of this
/// library (it's implementation detail) I will beat you.
#[doc(hidden)]
pub static CATEGORY_LIST_HEAD: AtomicPtr<CategoryInfo> = AtomicPtr::new(core::ptr::null_mut());

/// Super-duper ultra unsafe do not access but needs to be public so macros can touch it.
///
/// Utility function used by the unsafe_new_alloc_category macro for registering the type into the
/// global linked list of all categories.
#[doc(hidden)]
pub unsafe fn register_category(v: &'static CategoryInfo) {
    let ptr = NonNull::from(v).as_ptr();
    let next = CATEGORY_LIST_HEAD.swap(ptr, Ordering::SeqCst);
    v.next.store(next, Ordering::SeqCst);
}
