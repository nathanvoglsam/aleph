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
use std::mem::MaybeUninit;

///
/// A trait exposed by API objects that allows querying platform specific objects and interfaces
/// for accessing details specific to underlying implementations.
///
/// This allows for intentionally 'leaking' backend objects.
///
pub trait IGetPlatformInterface {
    /// The dynamic interface for looking up an interface/object by type-id. Writes the resulting
    /// object into a prepared place im memory though the provided 'out' pointer. 'out' must provide
    /// valid storage for an object of the requested type.
    ///
    /// It is not recommended to use this interface directly. Instead use this via the
    /// [GetPlatformInterface::query_platform_interface] wrapper.
    ///
    /// # Safety
    ///
    /// The caller has a responsibility to ensure that 'out' points to a valid region of memory that
    /// is of sufficient size, alignment and ownership to initialize a new object of the requested
    /// type into. There is not type safety in this interface, all responsibility lies on the caller
    /// to ensure this.
    ///
    /// The implementation is required to, if the target can be provided, clone or construct a new
    /// object of the expected type at the address given by 'out' and return Some to signify
    /// success. If the object of the requested type *can not be* provided then 'out' must remain
    /// untouched and None must be returned.
    ///
    /// These requirements are important as they allow implementing the interface without any heap
    /// allocations while keeping the [IGetPlatformInterface] trait-object safe. We can't use
    /// generics in object-safe traits so we must do this instead.
    unsafe fn __query_platform_interface(&self, target: TypeId, out: *mut ()) -> Option<()>;
}

///
/// A wrapper over [IGetPlatformInterface] that provides a type safe interface for using
/// `__query_platform_interface`.
///
pub trait GetPlatformInterface {
    /// A type-safe wrapper over [IGetPlatformInterface::__query_platform_interface] that
    /// automatically handles sending the correct type-id to the dynamic interface and casting back
    /// to the requested type.
    fn query_platform_interface<T: Sized + 'static>(&self) -> Option<T>;
}

impl<T: IGetPlatformInterface + ?Sized> GetPlatformInterface for T {
    #[inline]
    fn query_platform_interface<R: Sized + 'static>(&self) -> Option<R> {
        let mut stack_slot: MaybeUninit<R> = MaybeUninit::uninit();

        // Safety: It is our responsibility to ensure 'out' points to a valid memory region for an
        //         object of type R. We do that via 'stack_slot.
        //
        //         The caller is expected to initialize 'stack_slot' if it has returned 'Some' so
        //         it is safe for us to assume_init in that case.
        unsafe {
            if self
                .__query_platform_interface(TypeId::of::<R>(), stack_slot.as_mut_ptr() as *mut ())
                .is_some()
            {
                Some(MaybeUninit::assume_init(stack_slot))
            } else {
                None
            }
        }
    }
}
