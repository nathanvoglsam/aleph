use crate::AnyRef;
use core::any::TypeId;
use core::marker::PhantomData;
use core::ptr::NonNull;

/// This struct is used for splitting a trait object (`&dyn SomeTrait`) into it's actual binary
/// representation so we can hack on it.
///
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

/// An end user of this crate should never have to interact with this type directly, but it needs
/// to be public as it is part of the `__query_interface` interface.
///
/// Rust's trait objects are fat pointers. That's what this type is, just with an explicit layout.
#[repr(C)]
pub struct TraitObject<'a> {
    /// The pointer to the underlying object
    pub data: NonNull<()>,

    /// The pointer to the vtable for this trait object. The type is erased by this struct so this
    /// could point to any vtable. It's up to the user to determine which one
    pub vtable: *const (),

    /// A phantom data to at least bind a lifetime to the pointer
    pub phantom: PhantomData<&'a ()>,
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
