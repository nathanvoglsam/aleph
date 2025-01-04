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

use std::fmt::{Debug, Formatter};
use std::num::NonZeroU64;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct Handle(NonZeroU64);

impl Handle {
    /// Takes the handle and produces a new handle pointing to the same slot and generation but with
    /// the given pool and type id values.
    pub(crate) fn with_pool_and_type_id(&self, pool_id: u8, type_id: u8) -> Self {
        let fields = self.to_fields();
        let fields = HandleFields {
            pool_id,
            type_id,
            generation: fields.generation,
            slot_index: fields.slot_index,
        };
        // SAFETY: We take a non-zero handle and don't modify the fields that guarantee the handle
        //         will be non-zero (generation will always be non-zero for a valid handle) so this
        //         new handle will always be a valid handle too.
        unsafe { Self::from_fields(fields).unwrap_unchecked() }
    }

    /// Handles creating Handle from the individual fields.
    ///
    /// # Warning
    ///
    /// Coerces any handle with a '0' generation to all zeroes to encode a null handle using the
    /// NonZeroU64 niche. An input [HandleFields] with a '0' generation will discard the other IDs
    /// to emit this null reference.
    pub(crate) const unsafe fn from_fields(v: HandleFields) -> Option<Self> {
        // Coerce the zero generation to a null handle
        if v.generation.0 == 0 {
            None
        } else {
            let pool_id: u64 = (v.pool_id as u64) << 48;
            let type_id: u64 = (v.type_id as u64) << 40;
            let generation: u64 = (v.generation.0 as u64) << 32;
            let resource: u64 = v.slot_index as u64;
            let word = pool_id | type_id | generation | resource;
            match NonZeroU64::new(word) {
                None => None,
                Some(v) => Some(Self(v)),
            }
        }
    }

    /// Unwraps the handle to its individual fields.
    pub(crate) const fn to_fields(&self) -> HandleFields {
        let pool_id = (self.0.get() >> 48) & 0xFF;
        let type_id = (self.0.get() >> 40) & 0xFF;
        let generation = (self.0.get() >> 32) & 0xFFFF;
        let slot_index = (self.0.get()) & 0xFFFFFFFF;
        HandleFields {
            pool_id: pool_id as u8,
            type_id: type_id as u8,
            generation: Generation(generation as u16),
            slot_index: slot_index as u32,
        }
    }

    /// Utility for implementing debug on newtype wrappers using the newtype's name
    pub(crate) fn debug_newtype(&self, f: &mut Formatter<'_>, name: &str) -> std::fmt::Result {
        let fields = self.to_fields();
        f.debug_struct(name)
            .field("pool_id", &fields.pool_id)
            .field("type_id", &fields.type_id)
            .field("generation", &fields.generation)
            .field("resource", &fields.slot_index)
            .finish()
    }
}

impl Debug for Handle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.debug_newtype(f, "Handle")
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct HandleFields {
    /// The pool the resource is allocated from
    pub pool_id: u8,

    /// The type of resource this handle refers to
    pub type_id: u8,

    /// The generation of the resource this handle refers to
    pub generation: Generation,

    /// The resource index this handle refers to in the given pool
    pub slot_index: u32,
}

/// Trait implemented by anything that is trivially convertible into a handle. Intended to be used
/// by handle newtype wrappers to generically unwrap to the inner handle type.
pub trait IntoHandle {
    /// Gets the [Handle] `self` represents
    fn into_handle(self) -> Handle;
}

macro_rules! handle_newtype {
    ($name:ident) => {
        #[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
        #[repr(transparent)]
        pub struct $name($crate::Handle);

        impl $name {
            /// Wraps the given handle in our newtype wrapper
            pub const unsafe fn from_handle(v: $crate::Handle) -> Self {
                Self(v)
            }

            /// Unwraps the given newtype handle to the underlying handle data
            pub const fn to_handle(&self) -> $crate::Handle {
                self.0
            }
        }

        impl ::core::fmt::Debug for $name {
            fn fmt(&self, f: &mut Formatter<'_>) -> ::core::fmt::Result {
                self.0.debug_newtype(f, stringify!($name))
            }
        }

        impl $crate::IntoHandle for $name {
            fn into_handle(self) -> $crate::Handle {
                self.to_handle()
            }
        }
    };
}

handle_newtype!(TextureHandle);
handle_newtype!(BufferHandle);
handle_newtype!(MeshHandle);

///
/// The base `Generation` type that represents an arbitrary generation index.
///
/// There are also some new-type wrappers `AliveGeneration` and `DeadGeneration` that reduce the
/// set of valid indices to alive or dead indices respectively.
///
#[repr(transparent)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
pub struct Generation(pub u16);

impl Generation {
    /// Returns a new generation with initial state of 0, which encodes a dead generation.
    pub const fn new() -> Self {
        Self(0)
    }

    /// Returns a new generation with initial state of 1, which encodes a live generation.
    pub const fn new_live() -> Self {
        Self(1)
    }

    /// Returns whether this generation marks a generation that is alive
    #[inline]
    pub const fn is_alive(self) -> bool {
        self.0 % 2 != 0
    }

    /// Returns whether this generation marks a generation that si dead
    #[inline]
    pub const fn is_dead(self) -> bool {
        !self.is_alive()
    }

    /// Increments the generation index.
    #[inline]
    pub const fn increment(self) -> Generation {
        Self(self.0.wrapping_add(1))
    }

    /// Increments the generation index and assigns the result to self
    #[inline]
    pub fn increment_assign(&mut self) {
        *self = self.increment()
    }
}
