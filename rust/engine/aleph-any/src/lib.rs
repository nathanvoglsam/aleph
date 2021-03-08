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

use core::any::{Any, TypeId};
use core::mem::size_of;
use core::ptr::NonNull;

pub type TraitObject = (NonNull<()>, *const ());

///
/// This trait represents the core trait for the aleph interface system.
///
/// The trait represents the core interface that all valid interface objects must implement.
///
pub trait IAny: Any {
    fn __query_interface(&self, _target: TypeId) -> Option<TraitObject> {
        None
    }
}

impl dyn IAny {
    pub fn query_interface<T: ?Sized + 'static>(&self) -> Option<&T> {
        // Assert that trait object is the size of two pointers
        assert_eq!(size_of::<TraitObject>(), size_of::<usize>() * 2);

        // Assert that the null pointer NonNull is correctly triggering size optimization
        assert_eq!(size_of::<Option<TraitObject>>(), size_of::<TraitObject>());

        unsafe {
            if let Some(obj) = self.__query_interface(TypeId::of::<T>()) {
                Some(*(&obj as *const TraitObject as *const &T))
            } else {
                None
            }
        }
    }
}

pub trait AsIAny: IAny {
    fn as_interface_ref(&self) -> &dyn IAny;
    fn as_interface_mut(&mut self) -> &mut dyn IAny;
}

impl<T: IAny> AsIAny for T {
    fn as_interface_ref(&self) -> &dyn IAny {
        self
    }

    fn as_interface_mut(&mut self) -> &mut dyn IAny {
        self
    }
}

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
                if target == ::core::any::TypeId::of::<$typ>() {
                    Some($crate::TraitObject {
                        data: self as *const _ as *mut (),
                        vtable: ::core::ptr::null_mut(),
                    })
                } else {
                    None
                }
            }
        }
    }
);
