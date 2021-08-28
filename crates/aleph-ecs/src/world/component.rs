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

use std::any::TypeId;
use std::collections::{HashMap, HashSet};
use std::hash::{BuildHasherDefault, Hasher};

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
    #[inline]
    pub fn of<T: Component>() -> Self {
        // SAFETY: Just a bitcast from one wrapped u64 to another wrapped u64
        unsafe { std::mem::transmute(TypeId::of::<T>()) }
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

/// A hasher optimized for hashing a single u64.
///
/// Given something like `TypeId`, which is already a hash, to use as a key in a HashMap can be
/// less efficient than it needs to be. For `u64` keys which already hold sufficiently scrambled
/// data (like `TypeId` which is already a hash) we can skip the cost of the hash function in a
/// HashMap by just using the integer directly.
#[derive(Clone, Default)]
pub struct IdentityHasher {
    hash: u64,
}

impl Hasher for IdentityHasher {
    #[inline]
    fn write_u64(&mut self, n: u64) {
        // Only a single value can be hashed, so the old hash should be zero.
        debug_assert_eq!(self.hash, 0);
        self.hash = n;
    }

    // This should never be called as this struct only supports hashing u64
    #[inline]
    fn write(&mut self, _: &[u8]) {
        unimplemented!()
    }

    #[inline]
    fn finish(&self) -> u64 {
        self.hash
    }
}

/// A type alias for a configuration of `std::hash::HashMap` that efficiently uses `ComponentTypeId`
/// as a key. This alias is special as it skips hashing the `ComponentTypeId` and uses that id
/// directly as the key.
pub type ComponentIdMap<T> = HashMap<ComponentTypeId, T, BuildHasherDefault<IdentityHasher>>;

/// A type alias for a configuration of `std::hash::HashSet` that efficiently uses `ComponentTypeId`
/// as a key. This alias is special as it skips hashing the `ComponentTypeId` and uses that id
/// directly as the key.
pub type ComponentSet = HashSet<ComponentTypeId, BuildHasherDefault<IdentityHasher>>;
