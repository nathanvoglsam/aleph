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

pub mod component_query;
pub mod component_source;

use std::mem::needs_drop;
use std::ptr::NonNull;
use std::sync::LazyLock;
use std::sync::atomic::{AtomicU32, Ordering};

use aleph_alloc::instrumentation::system;
use aleph_alloc::nstr::NStr;
use aleph_alloc::{BHashMap, BVec};
use init_list::{InitList, ListItem};

use crate::EcsSystem;

///
/// This trait needs to be implemented by any type that wishes to be used as a component
///
pub unsafe trait Component: Send + Sync + 'static {
    /// A name that can be used to identify the type implementing [`Component`]. This name is not
    /// guaranteed to uniquely identify the type, only the ID may do that. This name should only
    /// be used for logging or other human visible use cases.
    const NAME: &'static NStr;

    /// A static reference to an [`TypeDescription`] instance that describes the [`Component`].
    const DESC: &'static LazyLock<TypeDescription>;
}

#[repr(transparent)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct ComponentId(pub(crate) u32);

/// A type alias for a configuration of `std::hash::HashMap` that efficiently uses `ComponentTypeId`
/// as a key. This alias is special as it skips hashing the `ComponentTypeId` and uses that id
/// directly as the key.
pub type ComponentIdMap<T> = BHashMap<ComponentId, T, EcsSystem>;

/// FFI portable type description table. Contains all the information exposed by [`Component`]
/// wrapped in a neat little struct that can be safely sent across FFI boundaries.
#[repr(C)]
pub struct TypeDescription {
    /// The auto-increment assigned ID.
    pub id: ComponentId,

    /// Size, in bytes, of the underlying object type.
    pub size: usize,

    /// Alignment, in bytes, of the underlying object type.
    pub align: usize,

    /// Human-readable name of the underlying type. Not guaranteed to be unique.
    pub name: &'static NStr,

    /// Opaque fn-ptr to a drop wrapper that will drop a packed array of 'count' objects. This can
    /// be set to 'None' if the underlying type returns false for [`needs_drop`].
    pub destructor: Option<unsafe extern "C" fn(NonNull<()>, count: u64)>,
}

impl TypeDescription {
    pub fn new<T: Sized>(name: &'static NStr) -> Self {
        unsafe extern "C" fn object_destructor<T: Sized>(this: NonNull<()>, count: u64) {
            unsafe {
                let mut base = this.cast::<T>();
                let mut count = count;
                while count != 0 {
                    base.drop_in_place();
                    base = base.add(1);
                    count -= 1;
                }
            }
        }

        static __COMPONENT_COUNTER: AtomicU32 = AtomicU32::new(0);

        // If ID == u32::MAX then we just overflowed the counter and we're in for sadness. Panic
        // instead.
        let id = __COMPONENT_COUNTER.fetch_add(1, Ordering::SeqCst);
        assert!(id < u32::MAX);

        Self {
            id: ComponentId(id),
            size: size_of::<T>(),
            align: align_of::<T>(),
            name,
            destructor: if needs_drop::<T>() {
                Some(object_destructor::<T>)
            } else {
                None
            },
        }
    }
}

#[macro_export]
macro_rules! register_component {
    ($t: path) => {
        impl $t {
            #[doc(hidden)]
            const fn __internal_component_type_desc()
            -> &'static ::std::sync::LazyLock<$crate::TypeDescription> {
                fn make() -> $crate::TypeDescription {
                    $crate::TypeDescription::new::<$t>($crate::nstr::nstr!(concat!(
                        module_path!(),
                        "::",
                        stringify!($t)
                    )))
                }
                static TYPE_DESC: ::std::sync::LazyLock<$crate::TypeDescription> =
                    ::std::sync::LazyLock::new(make);
                &TYPE_DESC
            }
        }
        impl $t {
            #[doc(hidden)]
            const fn __internal_component_node() -> &'static $crate::init_list::ListItem<
                &'static ::std::sync::LazyLock<$crate::TypeDescription>,
            > {
                static ENTRY: $crate::init_list::ListItem<
                    &'static ::std::sync::LazyLock<$crate::TypeDescription>,
                > = $crate::init_list::ListItem::new(<$t>::__internal_component_type_desc());
                &ENTRY
            }

            #[doc(hidden)]
            const fn __internal_register_component_node_scope() -> bool {
                #[$crate::ctor::ctor(crate_path = $crate::ctor)]
                fn internal_register_t() {
                    unsafe {
                        $crate::register_component_type(<$t>::__internal_component_node());
                    }
                }
                true
            }
        }
        unsafe impl $crate::Component for $t {
            const NAME: &'static $crate::nstr::NStr =
                $crate::nstr::nstr!(concat!(module_path!(), "::", stringify!($t)));
            const DESC: &'static ::std::sync::LazyLock<$crate::TypeDescription> =
                <$t>::__internal_component_type_desc();
        }
    };
}

/// A lazily initialized table of all types registered into the object system.
pub static COMPONENTS: LazyLock<BVec<&'static TypeDescription, EcsSystem>> = LazyLock::new(|| {
    // We don't care if someone's sealed the list before, only that it has been sealed.
    let _ = __UNSAFE_COMPONENT_REGISTRY_HEAD.seal();

    // Pull all the type descriptions into a list
    let mut list = BVec::new_in(system());
    for t in __UNSAFE_COMPONENT_REGISTRY_HEAD.iter().copied() {
        list.push(LazyLock::force(t));
    }

    // Sort by the ID
    list.sort_by_key(|v| v.id);

    // Assert that there are no duplicates, and the list is fully dense.
    //
    // COMPONENTS[i] should now map to a type with ID i.
    for (i, t) in list.iter().enumerate() {
        assert_eq!(i, t.id.0 as usize);
    }

    list
});

/// INTERNAL
///
/// DO NOT USE
#[doc(hidden)]
pub unsafe fn register_component_type(node: &'static ListItem<&'static LazyLock<TypeDescription>>) {
    __UNSAFE_COMPONENT_REGISTRY_HEAD.push_entry(node);
}

/// This is the head of the object system registry. All objects that interact with the object system
/// will push an entry onto this list during the static init phase.
#[doc(hidden)]
static __UNSAFE_COMPONENT_REGISTRY_HEAD: InitList<&'static LazyLock<TypeDescription>> =
    InitList::new();
