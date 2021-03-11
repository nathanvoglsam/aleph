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

use crate::{IAny, TraitObject};
use core::any::TypeId;
use core::mem::size_of;
use core::ops::Deref;
use std::ops::DerefMut;

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

/// A wrapper type for a shared reference to something that implements IAny. This is the preferred
/// way of using an interface. There is very little reason to have a bare trait object for an
/// interface as you lose access to the `query_interface` wrapper
///
/// This allows adding wrapper functions that will correctly autocomplete
#[repr(transparent)]
pub struct AnyMut<'a, T: IAny + ?Sized> {
    inner: &'a mut T,
}

impl<'a, T: IAny + ?Sized> AnyMut<'a, T> {
    /// Function for converting a bare `IAny` trait object into an `AnyRef` wrapper
    pub fn new(inner: &'a mut T) -> Self {
        Self { inner }
    }

    /// Unpack the `AnyMut` into the underlying reference
    pub fn into_bare(this: Self) -> &'a mut T {
        this.inner
    }

    /// Generic wrapper function over the bare implementation of `__query_interface` that makes
    /// using it safe and simple.
    pub fn query_interface<Into: IAny + ?Sized>(self) -> Option<AnyMut<'a, Into>> {
        // Assert that trait object is the size of two pointers. Compiles to nothing
        assert_eq!(size_of::<TraitObject>(), size_of::<usize>() * 2);

        // Assert that the null pointer NonNull is correctly triggering size optimization. Compiles
        // to nothing
        assert_eq!(size_of::<Option<TraitObject>>(), size_of::<TraitObject>());

        unsafe {
            if let Some(obj) = self.__query_interface(TypeId::of::<Into>()) {
                let any_ref = (&obj as *const TraitObject as *const &mut Into).read();
                Some(AnyMut::new(any_ref))
            } else {
                None
            }
        }
    }
}

impl<'a, T: IAny + ?Sized> Deref for AnyMut<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.inner
    }
}

impl<'a, T: IAny + ?Sized> DerefMut for AnyMut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner
    }
}

impl<'a, T: IAny + ?Sized> From<&'a mut T> for AnyMut<'a, T> {
    fn from(v: &'a mut T) -> Self {
        Self::new(v)
    }
}
