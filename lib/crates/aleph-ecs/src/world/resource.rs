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

use std::any::{Any, TypeId};
use std::hash::Hash;

use aleph_identity_hasher::IdentityHasher;

///
/// This trait specifies the requirements of a type that will be used as a [`Resource`] within the
/// scheduler.
///
/// Resources will be scheduled for parallel access so the must implement [`Send`] and [`Sync`] as
/// they **will** be shared between threads. Resources will also be type-erased and so [`Any`] is
/// needed so the concrete type can be recovered safely at runtime.
///
/// This trait will be automatically implemented on any type that meets these requirements.
///
pub trait Resource: Any + Send + Sync + 'static {}

impl<T: Any + Send + Sync + 'static> Resource for T {}

///
/// The type that is used for identifying a component type by ID.
///
/// The Rust friendly interface uses the [`TypeId`] as the [`ResourceId`] as this built-in feature
/// meets the requirements of an ephemeral ID (doesn't need to be stable between compilations) that
/// uniquely identifies a [`Resource`]. We unwrap the internal `u64` in [`TypeId`] for FFI purposes.
/// External resources can be provided via FFI, where the FFI caller provides their own
/// [`ResourceId`] too. Therefore we need to pin down the memory layout of this type.
///
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
#[repr(transparent)]
pub struct ResourceId(u64);

impl ResourceId {
    /// Returns the [`ResourceId`] of the given resource.
    #[inline]
    pub fn of<T: Resource>() -> Self {
        let v = IdentityHasher::hash(TypeId::of::<T>());
        Self(v)
    }

    /// Returns the [`ResourceId`] of the given resource by value using generics. This function will
    /// still return a compile time constant, unlike [`ResourceId::of_any`] which performs a runtime
    /// lookup of the [`ResourceId`].
    pub fn of_val<T: Resource>(_: &T) -> Self {
        Self::of::<T>()
    }

    /// Returns the [`ResourceId`] of the given object. The lookup is performed at runtime. If the
    /// type is known at compile-time use [`ResourceId::of`] of [`ResourceId::of_val`].
    pub fn of_any(val: &dyn Resource) -> Self {
        let id = val.type_id();
        let v = IdentityHasher::hash(id);
        Self(v)
    }
}
