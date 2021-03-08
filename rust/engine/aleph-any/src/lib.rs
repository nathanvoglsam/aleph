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

#![no_std]

//!
//! # Example
//!
//! ```
//! use aleph_any::*;
//!
//! pub trait IInterfaceA: IAny {
//!     fn call_a(&self) -> u32;
//! }
//!
//! pub trait IInterfaceB: IAny {
//!     fn call_b(&self) -> u64;
//! }
//!
//! pub struct Concrete();
//!
//! impl IInterfaceA for Concrete {
//!     fn call_a(&self) -> u32 {
//!         1
//!     }
//! }
//!
//! impl IInterfaceB for Concrete {
//!     fn call_b(&self) -> u64 {
//!         2
//!     }
//! }
//!
//! pub fn to_concrete(interface: AnyRef<dyn IInterfaceB>) -> Option<&Concrete> {
//!     interface.query_interface::<Concrete>().map(|v| v.into_bare())
//! }
//!
//! pub fn to_interface(interface: AnyRef<dyn IInterfaceB>) -> Option<AnyRef<dyn IInterfaceA>> {
//!     interface.query_interface()
//! }
//!
//! aleph_any::declare_interfaces!(Concrete, [IInterfaceA, IInterfaceB]);
//! ```
//!

use core::any::TypeId;
use core::mem::size_of;
use core::ops::Deref;
use core::ptr::NonNull;

pub struct TraitObject {
    pub data: NonNull<()>,
    pub vtable: *const (),
}

///
/// This trait represents the core trait for the aleph interface system.
///
/// The is the core interface that all valid interface objects must implement. It provides the
/// central function `__query_interface` that is used to cast one interface to another.
///
/// You should not have to implement this trait directly. Instead use the `declare_interfaces!`
/// macro provided by this crate.
///
pub trait IAny: 'static {
    ///
    /// The `query_interface` function that should only be accessed through the `AnyRef` wrapper.
    ///
    /// This function should return a `TraitObject` for the given `TypeId` if, and only if, the
    /// concrete type behind the `IAny` implements (or actually is) the given type.
    ///
    /// This is *very* unsafe to implement manually, so don't. Unless there's a *very* good reason,
    /// just use `declare_interfaces!`.
    ///
    fn __query_interface(&self, _target: TypeId) -> Option<TraitObject> {
        None
    }
}

///
/// Trait for converting from something that implements IAny into an `AnyRef<dyn IAny>`
///
pub trait AsIAny: IAny {
    fn as_interface(&self) -> AnyRef<dyn IAny>;
}

/// A wrapper type for a shared reference to something that implements IAny. This is the preferred
/// way of using an interface. There is very little reason to have a bare trait object for an
/// interface as you lose access to the `query_interface` wrapper
///
/// This allows adding wrapper functions that will correctly autocomplete
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct AnyRef<'a, T: IAny + ?Sized> {
    inner: &'a T,
}

impl<'a, T: IAny + ?Sized> AnyRef<'a, T> {
    /// Function for converting a bare `IAny` trait object into an `AnyRef` wrapper
    pub fn new(inner: &'a T) -> Self {
        Self { inner }
    }

    /// Generic wrapper function over the bare implementation of `__query_interface` that makes
    /// using it safe and simple.
    pub fn query_interface<Into: IAny + ?Sized>(&self) -> Option<AnyRef<'a, Into>> {
        // Assert that trait object is the size of two pointers. Compiles to nothing
        assert_eq!(size_of::<TraitObject>(), size_of::<usize>() * 2);

        // Assert that the null pointer NonNull is correctly triggering size optimization. Compiles
        // to nothing
        assert_eq!(size_of::<Option<TraitObject>>(), size_of::<TraitObject>());

        unsafe {
            if let Some(obj) = self.__query_interface(TypeId::of::<Into>()) {
                let any_ref = *(&obj as *const TraitObject as *const &Into);
                Some(AnyRef::new(any_ref))
            } else {
                None
            }
        }
    }

    /// Unpacks the wrapper into the bare reference
    pub fn into_bare(self) -> &'a T {
        self.inner
    }
}

impl<'a, T: IAny + ?Sized> Deref for AnyRef<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.inner
    }
}

impl<'a, T: IAny + ?Sized> From<&'a T> for AnyRef<'a, T> {
    fn from(v: &'a T) -> Self {
        Self::new(v)
    }
}

/// This macro is used for implementing IAny for a concrete type. This will correctly generate the
/// required glue for casting to any of the provided interfaces that the concrete type implements.
///
/// # Example
///
/// ```
/// use aleph_any::*;
///
/// pub trait IInterfaceA: IAny {
///     fn call_a(&self);
/// }
///
/// pub trait IInterfaceB: IAny {
///     fn call_b(&self);
/// }
///
/// pub struct Concrete();
///
/// impl IInterfaceA for Concrete {
///     fn call_a(&self) {}
/// }
///
/// impl IInterfaceB for Concrete {
///     fn call_b(&self) {}
/// }
///
/// aleph_any::declare_interfaces!(Concrete, [IInterfaceA, IInterfaceB]);
/// ```
///
#[macro_export]
macro_rules! declare_interfaces (
    ( $typ: ident, [ $( $iface: ident ),* ]) => {
        impl $crate::IAny for $typ {
            fn __query_interface(&self, target: ::core::any::TypeId) -> Option<$crate::TraitObject> {
                unsafe {
                    $(
                    if target == ::core::any::TypeId::of::<$iface>() {
                        return Some(::core::mem::transmute(self as &$iface));
                    }
                    )*
                }
                unsafe {
                    if target == ::core::any::TypeId::of::<$typ>() {
                        Some($crate::TraitObject {
                            data: ::core::ptr::NonNull::new_unchecked(self as *const _ as *mut ()),
                            vtable: ::core::ptr::null_mut(),
                        })
                    } else {
                        None
                    }
                }
            }
        }
        impl $crate::AsIAny for $typ {
            fn as_interface(&self) -> $crate::AnyRef<dyn $crate::IAny> {
                $crate::AnyRef::new(self)
            }
        }
    }
);
