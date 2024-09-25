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

pub extern crate uuid;

use std::mem::needs_drop;
use std::ptr::NonNull;

/// This trait represents the capability of a type to interact with the 'object system'
pub unsafe trait IObject: Sized {
    /// A stable, globally unique UUID. This must forever be stable and must always uniquely
    /// identify the type implementing [`IObject`]. It is the implementors responsibility to uphold
    /// the uniqueness and stability guarantees of this interface.
    const ID: uuid::Uuid;

    /// The size, in bytes, of the type implementing [`IObject`].
    const SIZE: usize = std::mem::size_of::<Self>();

    /// The alignment, in bytes, of the type implementing [`IObject`].
    const ALIGN: usize = std::mem::align_of::<Self>();

    /// A name that can be used to identify the type implementing [`IObject`]. This name is not
    /// guaranteed to uniquely identify the type, only the ID may do that. This name should only
    /// be used for logging or other human visible use cases.
    const NAME: &'static str;

    /// A type-erased destructor function that can be called on a pointer that is expected to be
    /// pointing to a non-aliased, owned instance of the type implementing [`IObject`].
    ///
    /// It is the _caller's_ (of [`IObject::destructor`], not the implementor of this trait)
    /// responsibility to ensure the pointer given to this function points to a valid, live 'Self',
    /// and that the access to this object is correctly synchronized.
    ///
    /// This also takes a 'count' parameter, for denoting the number of objects to destroy. This
    /// function will assume that the given pointer points to an array of 'count' objects and drop
    /// all of them. It is the caller's responsibility to ensure this is correct.
    unsafe extern "C" fn destructor(this: NonNull<()>, count: u64) {
        let mut base = this.cast::<Self>();
        for _ in 0..count {
            base.drop_in_place();
            base = base.add(1);
        }
    }
}

/// Short-hand for getting the ID of an object of type `T`
pub const fn object_id<T: IObject>() -> uuid::Uuid {
    T::ID
}

/// Short-hand for getting the name of an object of type `T`
pub const fn object_name<T: IObject>() -> &'static str {
    T::NAME
}

/// FFI portable object description table. Contains all the information exposed by [`IObject`]
/// wrapped in a neat little struct that can be safely sent across FFI boundaries.
#[repr(C)]
#[derive(Clone)]
pub struct ObjectDescription {
    pub id: uuid::Uuid,
    pub size: usize,
    pub align: usize,
    pub name: &'static str,
    pub destructor: Option<unsafe extern "C" fn(NonNull<()>, count: u64)>,
}

impl ObjectDescription {
    /// Constructs a [`ObjectDescription`] for a given type `T`
    pub const fn get<T: IObject>() -> Self {
        let destructor: Option<unsafe extern "C" fn(NonNull<()>, u64)> = if needs_drop::<T>() {
            Some(T::destructor)
        } else {
            None
        };

        Self {
            id: T::ID,
            size: T::SIZE,
            align: T::ALIGN,
            name: T::NAME,
            destructor,
        }
    }
}

/// This macro can be used to implement [`IObject`] on an object as a shorthand compared to manually
/// implementing it directly. This will correctly generate a (mostly) safe implementation of
/// [`IObject`] for the given type.
///
/// # Safety
///
/// This macro is still, strictly speaking, unsafe. We can't protect the caller from duplicate UUIDs
/// or if the caller changes the UUID.
#[macro_export]
macro_rules! unsafe_impl_iobject {
    ($t: path, $id: literal) => {
        unsafe impl $crate::IObject for $t {
            const ID: $crate::uuid::Uuid = $crate::uuid::uuid!($id);
            const NAME: &'static str = concat!(module_path!(), "::", stringify!($t));
        }
    };
}

// struct Test();
// unsafe_impl_iobject!(Test, "019212d8-f424-7221-8d34-97b9f318bf0b");
