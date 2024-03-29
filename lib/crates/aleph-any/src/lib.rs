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
//! pub fn to_concrete(interface: &dyn IInterfaceB) -> Option<&Concrete> {
//!     interface.query_interface::<Concrete>()
//! }
//!
//! pub fn to_interface(interface: &dyn IInterfaceB) -> Option<&dyn IInterfaceA> {
//!     interface.query_interface()
//! }
//!
//! aleph_any::declare_interfaces!(Concrete, [IInterfaceA, IInterfaceB]);
//! ```
//!

mod any;
mod any_arc;

#[cfg(test)]
mod tests;

pub use any::{box_downcast, box_downcast_unchecked, IAny, QueryInterface, TraitObject};
pub use any_arc::{AnyArc, AnyWeak};

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
            #[allow(bare_trait_objects)]
            fn __query_interface(&self, target: ::core::any::TypeId) -> Option<$crate::TraitObject> {
                unsafe {
                    $(
                    if target == ::core::any::TypeId::of::<dyn $iface>() {
                        return Some(::core::mem::transmute(self as &dyn $iface));
                    }
                    )*
                    if target == ::core::any::TypeId::of::<dyn $crate::IAny>() {
                        return Some(::core::mem::transmute(self as &dyn $crate::IAny));
                    }
                }
                unsafe {
                    if target == ::core::any::TypeId::of::<$typ>() {
                        Some($crate::TraitObject {
                            data: ::core::ptr::NonNull::new_unchecked(self as *const _ as *mut ()),
                            vtable: ::core::ptr::null_mut(),
                            phantom: ::core::default::Default::default(),
                        })
                    } else {
                        None
                    }
                }
            }
        }
    }
);
