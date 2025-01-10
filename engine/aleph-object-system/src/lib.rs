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

pub extern crate aleph_nstr as nstr;
pub extern crate uuid;

use std::mem::{needs_drop, ManuallyDrop};
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;
use std::sync::Arc;

use aleph_nstr::NStr;

/// This trait represents the capability of a type to interact with the 'object system'
pub unsafe trait IObject: Sized {
    /// A stable, globally unique UUID. This must forever be stable and must always uniquely
    /// identify the type implementing [`IObject`]. It is the implementors responsibility to uphold
    /// the uniqueness and stability guarantees of this interface.
    const ID: uuid::Uuid;

    /// The size, in bytes, of the type implementing [`IObject`].
    const SIZE: usize;

    /// The alignment, in bytes, of the type implementing [`IObject`].
    const ALIGN: usize;

    /// A name that can be used to identify the type implementing [`IObject`]. This name is not
    /// guaranteed to uniquely identify the type, only the ID may do that. This name should only
    /// be used for logging or other human visible use cases.
    const NAME: &'static NStr;

    /// A static reference to an [`ObjectDescription`] instance that describes the [`IObject`].
    const DESC: &'static ObjectDescription;
}

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
pub unsafe extern "C" fn object_destructor<T: Sized>(this: NonNull<()>, count: u64) {
    let mut base = this.cast::<T>();
    let mut count = count;
    while count != 0 {
        base.drop_in_place();
        base = base.add(1);
        count = count - 1;
    }
}

/// This is a 'const' wrapper function that will return a destructor function pointer if the given
/// 'T' returns true for [`needs_drop`].
pub const fn get_object_destructor_for<T: Sized>(
) -> Option<unsafe extern "C" fn(NonNull<()>, count: u64)> {
    if needs_drop::<T>() {
        Some(object_destructor::<T>)
    } else {
        None
    }
}

/// Short-hand for getting the ID of an object of type `T`
pub const fn object_id<T: IObject>() -> uuid::Uuid {
    T::ID
}

/// Short-hand for getting the name of an object of type `T`
pub const fn object_name<T: IObject>() -> &'static NStr {
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
    pub name: &'static NStr,
    pub destructor: Option<unsafe extern "C" fn(NonNull<()>, count: u64)>,
    pub destructor_arc: unsafe extern "C" fn(NonNull<()>, count: u64),
    // pub destructor_box: unsafe extern "C" fn(NonNull<()>, count: u64),
}

impl ObjectDescription {
    /// Constructs a [`ObjectDescription`] for a given type `T`
    pub const fn get<T: IObject>() -> Self {
        Self {
            id: T::ID,
            size: T::SIZE,
            align: T::ALIGN,
            name: T::NAME,
            destructor: get_object_destructor_for::<T>(),
            destructor_arc: object_destructor::<Arc<ArcedObject<T>>>,
            // destructor_box: object_destructor::<Box<T>>,
        }
    }
}

#[repr(transparent)]
#[derive(Clone)]
pub struct ArcObject {
    inner: ManuallyDrop<Arc<OpaqueArcedObject>>,
}

impl ArcObject {
    #[inline]
    pub fn from_object<T: IObject>(v: Arc<ArcedObject<T>>) -> Self {
        unsafe {
            let ptr = Arc::into_raw(v);
            let ptr = ptr as *const OpaqueArcedObject;
            ArcObject {
                inner: ManuallyDrop::new(Arc::from_raw(ptr)),
            }
        }
    }

    ///
    /// Gets the number of strong ([`ArcObject`]) pointers to this allocation.
    ///
    /// # Safety
    ///
    /// This method by itself is safe, but using it correctly requires extra care.
    /// Another thread can change the strong count at any time,
    /// including potentially between calling this method and acting on the result.
    ///
    /// # Info
    ///
    /// This is just a wrapper around [`std::sync::Arc::strong_count`]
    ///
    #[inline]
    #[must_use]
    pub fn strong_count(&self) -> usize {
        Arc::strong_count(&self.inner)
    }

    #[inline]
    pub fn downcast<U: IObject>(&self) -> Option<Arc<ArcedObject<U>>> {
        unsafe {
            if U::ID == self.inner.vtable.id {
                let ptr = Arc::into_raw(self.inner.deref().clone());
                let ptr = ptr as *const ArcedObject<U>;
                let arc = Arc::from_raw(ptr);
                Some(arc)
            } else {
                None
            }
        }
    }

    #[inline]
    pub fn downcast_ref<U: IObject>(&self) -> Option<&U> {
        unsafe {
            if U::ID == self.inner.vtable.id {
                let ptr: NonNull<OpaqueArcedObject> = NonNull::from(self.inner.as_ref());
                Some(&ptr.cast::<ArcedObject<U>>().as_ref().object)
            } else {
                None
            }
        }
    }
}

impl Drop for ArcObject {
    fn drop(&mut self) {
        unsafe {
            let ptr = NonNull::from(&self.inner);
            let ptr = ptr.cast::<()>();
            let f = self.inner.vtable.destructor_arc;
            (f)(ptr, 1)
        }
    }
}

#[repr(C)]
pub struct ArcedObject<T: IObject> {
    vtable: &'static ObjectDescription,
    object: T,
}

impl<T: IObject> ArcedObject<T> {
    /// Constructs a new [`ArcedObject`] wrapping the given object.
    pub const fn new(object: T) -> Self {
        Self {
            vtable: T::DESC,
            object,
        }
    }

    /// Constructs a new `Arc<ArcedObject<T>>` wrapping the given object.
    #[inline]
    pub fn new_arc(object: T) -> Arc<ArcedObject<T>> {
        Arc::new(Self::new(object))
    }

    /// Constructs a new `Arc<ArcedObject<T>>` wrapping the given object that is then converted into
    /// an opaque [`ArcObject`].
    #[inline]
    pub fn new_arc_opaque(object: T) -> ArcObject {
        let arc = Self::new_arc(object);
        ArcObject::from_object(arc)
    }
}

impl<T: IObject> Deref for ArcedObject<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.object
    }
}

impl<T: IObject> DerefMut for ArcedObject<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.object
    }
}

/// Internal object. Represents the layout we use for [`ArcObject`]'s arc which is cast to only be
/// aware of the inner
#[repr(C)]
struct OpaqueArcedObject {
    vtable: &'static ObjectDescription,
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
        impl $t {
            #[doc(hidden)]
            const fn __internal_vtable() -> &'static $crate::ObjectDescription {
                static VTABLE: $crate::ObjectDescription = $crate::ObjectDescription {
                    id: $crate::uuid::uuid!($id),
                    size: ::std::mem::size_of::<$t>(),
                    align: ::std::mem::align_of::<$t>(),
                    name: $crate::nstr::nstr!(concat!(module_path!(), "::", stringify!($t))),
                    destructor: $crate::get_object_destructor_for::<$t>(),
                    destructor_arc: $crate::object_destructor::<
                        ::std::sync::Arc<$crate::ArcedObject<$t>>,
                    >,
                    // destructor_box: $crate::object_destructor::<::std::boxed::Box<$t>>,
                };
                &VTABLE
            }
        }
        unsafe impl $crate::IObject for $t {
            const ID: $crate::uuid::Uuid = <$t>::__internal_vtable().id;
            const SIZE: usize = <$t>::__internal_vtable().size;
            const ALIGN: usize = <$t>::__internal_vtable().align;
            const NAME: &'static $crate::nstr::NStr = <$t>::__internal_vtable().name;
            const DESC: &'static $crate::ObjectDescription = <$t>::__internal_vtable();
        }
    };
}

// struct Test();
// unsafe_impl_iobject!(Test, "019212d8-f424-7221-8d34-97b9f318bf0b");
