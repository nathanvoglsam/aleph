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

use std::any::Any;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};

///
/// The generic interface expected of types that can be used as a label. A label is just a generic
/// identifier or name that can be used in some context to identify something.
///
/// For example, the scheduler uses labels to identify execution stages.
///
/// A dynamic, generic system is more friendly to FFI bindings where special FFI friendly
/// implementations of [`Label`] can be created while rust friendly interfaces can be used by pure
/// rust code.
///
pub trait Label: DynHash + Debug + Send + Sync + 'static {
    #[doc(hidden)]
    fn dyn_clone(&self) -> Box<dyn Label>;
}

impl PartialEq for dyn Label {
    fn eq(&self, other: &Self) -> bool {
        self.dyn_eq(other.as_dyn_eq())
    }
}

impl Eq for dyn Label {}

impl Hash for dyn Label {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.dyn_hash(state);
    }
}

impl Clone for Box<dyn Label> {
    fn clone(&self) -> Self {
        self.dyn_clone()
    }
}

impl Label for &'static str {
    fn dyn_clone(&self) -> Box<dyn Label> {
        Box::new(<&str>::clone(self))
    }
}

///
/// Support trait used by [`Label`]
///
pub trait DynEq: Any {
    /// Get `self` as [`Any`]
    fn as_any(&self) -> &dyn Any;

    /// Compare `self` with `other`.
    fn dyn_eq(&self, other: &dyn DynEq) -> bool;
}

impl<T: Any + Eq> DynEq for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn dyn_eq(&self, other: &dyn DynEq) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<T>() {
            return self == other;
        }
        false
    }
}

///
/// Support trait used by [`Label`]
///
pub trait DynHash: DynEq {
    /// Get `self` as [`DynEq`]
    fn as_dyn_eq(&self) -> &dyn DynEq;

    /// Dynamic version of [`Hash::hash`]
    fn dyn_hash(&self, state: &mut dyn Hasher);
}

impl<T: DynEq + Hash> DynHash for T {
    fn as_dyn_eq(&self) -> &dyn DynEq {
        self
    }

    fn dyn_hash(&self, mut state: &mut dyn Hasher) {
        T::hash(self, &mut state);
        self.type_id().hash(&mut state);
    }
}
