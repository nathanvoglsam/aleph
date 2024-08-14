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
pub mod component_registry;
pub mod component_source;

use std::any::TypeId;
use std::collections::{HashMap, HashSet};
use std::hash::{BuildHasherDefault, Hash};

use aleph_identity_hasher::IdentityHasher;

///
/// This trait needs to be implemented by any type that wishes to be used as a component
///
pub trait Component: 'static + Sized + Send + Sync {
    fn get_type_description() -> ComponentTypeDescription;
}

impl<T: 'static + Sized + Send + Sync> Component for T {
    #[inline]
    fn get_type_description() -> ComponentTypeDescription {
        // Depending on whether or not `T` has a drop implementation we produce the virtual drop
        // functions
        let fn_drop = if std::mem::needs_drop::<T>() {
            unsafe extern "C" fn drop_fn<U: Component>(v: *mut u8) {
                (v as *mut U).drop_in_place()
            }

            let fn_drop: unsafe extern "C" fn(*mut u8) = drop_fn::<T>;

            Some(fn_drop)
        } else {
            None
        };

        ComponentTypeDescription {
            type_name: std::any::type_name::<T>(),
            type_id: ComponentTypeId::of::<T>(),
            type_size: std::mem::size_of::<T>(),
            type_align: std::mem::align_of::<T>(),
            fn_drop,
        }
    }
}

///
/// The type that is used for identifying a component type by ID
///
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
#[repr(transparent)]
pub struct ComponentTypeId(u64);

impl ComponentTypeId {
    /// Returns the ComponentTypeId of the given component type
    pub fn of<T: Component>() -> Self {
        let v = IdentityHasher::hash(TypeId::of::<T>());
        Self(v)
    }

    /// Returns the ComponentTypeId of the given component type by value. The value's type can be
    /// used to deduce the type without having to manually type it with `ComponentTypeId::of::<T>`
    pub fn of_val<T: Component>(_: &T) -> Self {
        Self::of::<T>()
    }
}

///
/// A struct that provides an FFI stable way to get a description of a component type, including the
/// name, size, alignment destructors and the ID.
///
#[repr(C)]
#[derive(Clone)]
pub struct ComponentTypeDescription {
    /// The name of the component type.
    pub type_name: &'static str,

    /// The ID of the component type
    pub type_id: ComponentTypeId,

    /// The size in bytes of the component type
    pub type_size: usize,

    /// The alignment in bytes of the component type
    pub type_align: usize,

    /// Optional function pointer that will drop a single component in place, type erased. This can
    /// be ommitted for types that don't need a destructor (POD types).
    ///
    /// # Safety
    /// The function this points to must treat the pointer given to it as a pointer to a single
    /// component and should call the corresponding drop function on it.
    pub fn_drop: Option<unsafe extern "C" fn(*mut u8)>,
}

/// A type alias for a configuration of `std::hash::HashMap` that efficiently uses `ComponentTypeId`
/// as a key. This alias is special as it skips hashing the `ComponentTypeId` and uses that id
/// directly as the key.
pub type ComponentIdMap<T> = HashMap<ComponentTypeId, T, BuildHasherDefault<IdentityHasher>>;

/// A type alias for a configuration of `std::hash::HashSet` that efficiently uses `ComponentTypeId`
/// as a key. This alias is special as it skips hashing the `ComponentTypeId` and uses that id
/// directly as the key.
pub type ComponentSet = HashSet<ComponentTypeId, BuildHasherDefault<IdentityHasher>>;
