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

use std::marker::PhantomData;
use std::mem::ManuallyDrop;
use std::ops::Deref;

///
/// A wrapper type that allows sharing weak references to COM interfaces. This allows sharing
/// 'pseudo-owned' copies of a COM interface without incrementing the reference counter. The
/// lifetime invariant is upheld by a phantom borrow.
///
#[repr(transparent)]
pub struct WeakRef<'a, T> {
    pub(crate) v: ManuallyDrop<T>,
    pub(crate) phantom: PhantomData<&'a T>,
}

impl<'a, T: Clone> Clone for WeakRef<'a, T> {
    fn clone(&self) -> Self {
        Self {
            v: unsafe { std::mem::transmute_copy(&self.v) },
            phantom: Default::default(),
        }
    }
}

impl<'a, T: Clone> WeakRef<'a, T> {
    /// Promotes the weak COM reference to a strong reference
    pub fn to_strong(&self) -> T {
        self.v.deref().clone()
    }
}

impl<'a, T: Clone> Deref for WeakRef<'a, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.v.deref()
    }
}

///
/// Trait implemented on all COM wrapper types that allows producing [WeakRef] handles to the
/// underlying COM interface
///
/// # Safety
///
/// This trait only really makes sense when implemented on our COM object wrappers so we mark it as
/// unsafe. The implementations of this trait will almost always require unsafe code anyway.
///
/// Generally this will require making a copy of the object without using [Clone] and so it will
/// almost certainly be UB.
///
/// For COM stuff it isn't UB as the wrappers are just transparent wrappers around a single pointer.
/// They're perfectly safe to do copies without [Clone] as long as we make sure that the reference
/// count is incremented safely.
///
pub unsafe trait AsWeakRef: Sized {
    fn as_weak(&self) -> WeakRef<Self>;
}

#[macro_export]
macro_rules! as_weak_ref_impl {
    ($t:ident) => {
        unsafe impl $crate::utils::AsWeakRef for $t {
            fn as_weak(&self) -> $crate::utils::WeakRef<Self> {
                unsafe {
                    $crate::utils::WeakRef {
                        v: core::mem::ManuallyDrop::new(core::mem::transmute_copy::<$t, $t>(self)),
                        phantom: core::default::Default::default(),
                    }
                }
            }
        }
    };
}

#[macro_export]
macro_rules! device_child_impl {
    ($t:ident) => {
        impl $crate::D3D12DeviceChild for $t {
            #[inline]
            unsafe fn get_device(&self) -> $crate::windows::core::Result<$crate::Device> {
                type D = $crate::windows::Win32::Graphics::Direct3D12::ID3D12Device4;
                let mut device: Option<D> = None;
                self.0.GetDevice::<D>(&mut device)?;
                $crate::windows::core::Result::Ok($crate::Device(device.unwrap()))
            }
        }
    };
}

#[macro_export]
macro_rules! object_impl {
    ($t:ident) => {
        impl $crate::D3D12Object for $t {
            #[inline]
            unsafe fn set_name_raw(&self, name: &[u16]) -> $crate::windows::core::Result<()> {
                use $crate::windows::core::PCWSTR;
                self.0.SetName(PCWSTR(name.as_ptr()))
            }
        }
    };
}

#[macro_export]
macro_rules! owned_object {
    ($t:ident) => {
        unsafe impl Send for $t {}
    };
}

#[macro_export]
macro_rules! shared_object {
    ($t:ident) => {
        impl ::core::clone::Clone for $t {
            #[inline]
            fn clone(&self) -> Self {
                Self(self.0.clone())
            }
        }
        unsafe impl Send for $t {}
        unsafe impl Sync for $t {}
    };
}
