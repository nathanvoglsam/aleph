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

use std::sync::atomic::{AtomicUsize, Ordering};

use aleph_nstr::NStr;
use crossbeam::atomic::AtomicCell;

use crate::instrumentation::with_category;

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

    /// Shortcut to [`with_category`].
    fn with<O>(f: impl FnOnce() -> O) -> O {
        with_category::<Self, O>(f)
    }
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
    pub(crate) id: uuid::Uuid,

    /// Human-readable name of the category. Not guaranteed to be unique.
    pub(crate) name: &'static NStr,

    /// The number of bytes allocated into this category, as tracked by all tagged allocations.
    pub(crate) bytes_allocated: AtomicUsize,

    /// The parent category, if one exists.
    pub(crate) parent: AtomicCell<Option<&'static CategoryInfo>>,

    /// Pointer used to form a linked list of all registered [`CategoryInfo`] objects.
    ///
    /// This is an internal field and should never be accessed directly, but needs to be made
    /// public for macros to work.
    pub(crate) next: AtomicCell<Option<&'static CategoryInfo>>,
}

impl CategoryInfo {
    pub const fn new(id: uuid::Uuid, name: &'static NStr) -> Self {
        Self {
            id,
            name,
            bytes_allocated: AtomicUsize::new(0),
            parent: AtomicCell::new(None),
            next: AtomicCell::new(None),
        }
    }

    /// Constructs a [`CategoryData`] for a given type `T`
    #[inline(always)]
    pub fn get<T: IAllocationCategory>() -> &'static Self {
        T::info()
    }

    /// Get the ID of the category
    pub const fn id(&self) -> &uuid::Uuid {
        &self.id
    }

    /// Get the name of the category
    pub const fn name(&self) -> &'static NStr {
        self.name
    }

    /// Utility for grabbing the number of bytes allocated
    #[inline(always)]
    pub fn allocated(&self) -> usize {
        self.bytes_allocated.load(Ordering::Relaxed)
    }

    /// Get the parent [`CategoryInfo`], if this category has one.
    #[inline(always)]
    pub fn parent(&self) -> Option<&'static CategoryInfo> {
        self.parent.load()
    }
}

/// This macro can be used to implement [`IAllocationCategory`] on an object as a shorthand compared
/// to manually implementing it directly. This will correctly generate a safe implementation of
/// [`IAllocationCategory`] for the given type.
#[macro_export]
macro_rules! new_alloc_category {
    ($t: path, $id: literal) => {
        $crate::new_alloc_category_inner!($t, $id, ::std::option::Option::None, stringify!($t));
    };
}

/// This macro can be used to implement [`IAllocationCategory`] on an object as a shorthand compared
/// to manually implementing it directly. This will correctly generate a safe implementation of
/// [`IAllocationCategory`] for the given type.
#[macro_export]
macro_rules! new_child_alloc_category {
    ($parent: path, $t: path, $id: literal) => {
        $crate::new_alloc_category_inner!(
            $t,
            $id,
            ::std::option::Option::Some(
                <$parent as $crate::instrumentation::IAllocationCategory>::info()
            ),
            $crate::const_format::concatc!(<$parent>::__NAME_SEGMENT, '.', stringify!($t))
        );
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! new_alloc_category_inner {
    ($t: path, $id: literal) => {
        $crate::new_alloc_category!($t, $id, ::std::option::Option::None, stringify!($t));
    };
    (child_of $parent: path, $t: path, $id: literal) => {
        $crate::new_alloc_category!(
            $t,
            $id,
            ::std::option::Option::Some(
                <$parent as $crate::instrumentation::IAllocationCategory>::info()
            ),
            $crate::const_format::concatc!(<$parent>::__NAME_SEGMENT, '.', stringify!($t))
        );
    };
    ($t: path, $id: literal, $parent_expr: expr, $name_seg: expr) => {
        impl $t {
            #[doc(hidden)]
            pub const __NAME_SEGMENT: &'static str = $name_seg;

            #[doc(hidden)]
            pub const fn __name_segment_nstr() -> &'static $crate::nstr::NStr {
                const PARENT_NAME: &'static str = <$t>::__NAME_SEGMENT;
                static NAME: &'static $crate::nstr::NStr =
                    $crate::nstr::NStr::new_str($crate::const_format::concatc!(PARENT_NAME, '\0'));
                NAME
            }
        }
        unsafe impl $crate::instrumentation::IAllocationCategory for $t {
            const ID: $crate::uuid::Uuid = $crate::uuid::uuid!($id);
            const NAME: &'static $crate::nstr::NStr = <$t>::__name_segment_nstr();

            #[inline(always)]
            fn info() -> &'static $crate::instrumentation::CategoryInfo {
                $crate::category_ctor!($t, $parent_expr);

                static __INFO: $crate::instrumentation::CategoryInfo =
                    $crate::instrumentation::CategoryInfo::new(
                        <$t as $crate::instrumentation::IAllocationCategory>::ID,
                        <$t as $crate::instrumentation::IAllocationCategory>::NAME,
                    );

                &__INFO
            }
        }
    };
}

#[cfg(feature = "instrumentation-enabled")]
#[doc(hidden)]
#[macro_export]
macro_rules! category_ctor {
    ($t: path, $parent_expr: expr) => {
        #[$crate::ctor::ctor(crate_path = $crate::ctor)]
        fn __internal_register_t() {
            $crate::instrumentation::register_category(
                <$t as $crate::instrumentation::IAllocationCategory>::info(),
                $parent_expr,
            );
        }
    };
}

#[cfg(not(feature = "instrumentation-enabled"))]
#[doc(hidden)]
#[macro_export]
macro_rules! category_ctor {
    ($t: path, $parent_expr: expr) => {};
}

/// An object that integrates with the category system to iterate over all declared category types.
///
/// The list is filled out using '__attribute__((constructor))' functions generated by the macros.
/// All the list setup is done before main and so once main is entered the list will always be
/// available.
pub struct AllocationCategoryIter {
    next: Option<&'static CategoryInfo>,
}

impl AllocationCategoryIter {
    /// Constructs a new [`AllocationCategoryIter`] instance.
    pub fn new() -> Self {
        let next = CATEGORY_LIST_HEAD.load();
        Self { next }
    }
}

impl Iterator for AllocationCategoryIter {
    type Item = &'static CategoryInfo;

    fn next(&mut self) -> Option<Self::Item> {
        let ptr = self.next;
        if let Some(ptr) = ptr {
            self.next = ptr.next.load();
            Some(ptr)
        } else {
            None
        }
    }
}

/// Utility function that will walk the list of registered types and assert if there are any
/// duplicate type IDs registered.
pub fn assert_no_duplicate_ids_registered() {
    #[cfg(feature = "instrumentation-enabled")]
    {
        use std::collections::HashMap;
        let mut types = HashMap::new();
        for category in AllocationCategoryIter::new() {
            let existing = types.insert(category.id, category);
            if let Some(existing) = existing {
                assert_eq!(category.id, existing.id); // Just being careful
                panic!(
                    "Colliding IAllocationCategory type IDs detected. '{}' and '{}' have the same ID of '{}'!",
                    existing.name(),
                    category.name(),
                    existing.id()
                );
            }
        }
    }
}

/// Do not access but needs to be public so macros can touch it.
///
/// Utility function used by the new_alloc_category macro for registering the type into the
/// global linked list of all categories.
#[doc(hidden)]
pub fn register_category(v: &'static CategoryInfo, parent: Option<&'static CategoryInfo>) {
    let next = CATEGORY_LIST_HEAD.swap(Some(v));
    v.next.store(next);
    v.parent.store(parent);
}

/// Forms the head of the linked list of all declared category types. Will be setup before main.
///
/// Call to get a reference to the head of the list.
static CATEGORY_LIST_HEAD: AtomicCell<Option<&'static CategoryInfo>> = AtomicCell::new(None);
