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
use std::hash::Hash;
use std::marker::PhantomData;
use std::num::NonZeroU64;

/// A generic generational handle that pairs a u32 index with a u32 generation.
///
/// This is expected to be new-typed and should not be exported from the crate directly.
///
/// This handle type packs a generation and index into the high/low halves of a [`NonZeroU64`]. We
/// exploit the fact that a generation with value 0 is not valid to access (dead generation) so we
/// can carve a niche from a completely zeroed handle.
///
/// This means `Option<RawHandle>` and `RawHandle` are the same size!
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct RawHandle(NonZeroU64);

impl RawHandle {
    /// Creates a Handle from the individual fields.
    ///
    /// # Warning
    ///
    /// Coerces any handle with a '0' generation to all zeroes to encode a null handle using the
    /// NonZeroU64 niche. An input [`HandleFields`] with a '0' generation will discard the other IDs
    /// to emit this null reference.
    pub const unsafe fn from_fields(v: HandleFields) -> Option<Self> {
        // Coerce the zero generation to a null handle
        if v.generation.0 == 0 {
            None
        } else {
            let generation: u64 = (v.generation.0 as u64) << 32;
            let resource: u64 = v.slot_index as u64;
            let word = generation | resource;
            match NonZeroU64::new(word) {
                None => None,
                Some(v) => Some(Self(v)),
            }
        }
    }

    /// Unwraps the handle to its individual fields.
    pub const fn to_fields(&self) -> HandleFields {
        let generation = (self.0.get() >> 32) & 0xFFFFFFFF;
        let slot_index = (self.0.get()) & 0xFFFFFFFF;
        HandleFields {
            generation: Generation(generation as u32),
            slot_index: slot_index as u32,
        }
    }

    /// Utility for implementing debug on newtype wrappers using the newtype's name
    pub fn debug_newtype(&self, f: &mut Formatter<'_>, name: &str) -> std::fmt::Result {
        let fields = self.to_fields();
        f.debug_struct(name)
            .field("generation", &fields.generation)
            .field("slot_index", &fields.slot_index)
            .finish()
    }
}

impl Debug for RawHandle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.debug_newtype(f, "Handle")
    }
}

impl HandleType for RawHandle {
    #[inline(always)]
    unsafe fn from_bare_handle(from: RawHandle) -> Self {
        from
    }

    #[inline(always)]
    fn to_bare_handle(self) -> RawHandle {
        self
    }
}

/// The unpacked fields from a [`RawHandle`]. A raw handle encodes this in the high/low word of a
/// u64.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct HandleFields {
    /// The generation of the resource this handle refers to
    pub generation: Generation,

    /// The resource index this handle refers to in the given pool
    pub slot_index: u32,
}

/// The base `Generation` type that represents an arbitrary generation index.
///
/// A generation encodes several things at once, and is the most important part of a generational
/// arena. When a handle is used to look up an object in some pool, the object's generation must
/// match the handle's generation. This prevents ABA reuse problems when adding/removing objects.
///
/// Secondly we use the lowest order bit to encode a 'live'/'dead' flag for objects inside an object
/// pool. In simpler terms all even generation values are considered 'dead' and all odd generation
/// values are considered 'live'. An arena just needs to increment a slot's generation whenever it
/// is allocated into or freed and handles will be invalidated correctly.
///
/// We also rely on the '0' value (conveniently an even number) being a 'dead' generation so we can
/// use [`NonZeroU64`] for our handles.
#[repr(transparent)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
pub struct Generation(pub(crate) u32);

impl Generation {
    /// Returns a new generation with initial state of 0, which encodes a dead generation.
    pub const fn new_dead() -> Self {
        Self(0)
    }

    /// Returns a new generation with initial state of 1, which encodes a live generation.
    pub const fn new_live() -> Self {
        Self(1)
    }

    /// Returns whether this generation marks a generation that is alive
    pub const fn is_alive(self) -> bool {
        self.0 % 2 != 0
    }

    /// Returns whether this generation marks a generation that is dead
    pub const fn is_dead(self) -> bool {
        !self.is_alive()
    }

    /// Revive a dead generation.
    ///
    /// # Note
    ///
    /// This may be called on a generation which is alive! In which case it will be changed to a
    /// dead generation. This can't cause problems on its own. It's up to callers to correctly use
    /// this type.
    ///
    /// This function _does_ contain a debug assert to check, but it will be stripped in a release
    /// build.
    pub const fn revive(&mut self) -> Generation {
        debug_assert!(self.is_dead(), "Tried to revive a live handle!");
        *self = self.increment();
        *self
    }

    /// Kill an alive generation.
    ///
    /// # Note
    ///
    /// This may be called on a generation which is dead! In which case it will be changed to an
    /// alive generation. This can't cause problems on its own. It's up to callers to correctly use
    /// this type.
    ///
    /// This function _does_ contain a debug assert to check, but it will be stripped in a release
    /// build.
    pub const fn kill(&mut self) -> Generation {
        debug_assert!(self.is_alive(), "Tried to kill a dead handle!");
        *self = self.increment();
        *self
    }

    /// Increments the generation index.
    const fn increment(self) -> Generation {
        Self(self.0.wrapping_add(1))
    }
}

/// Interface exposed on [`RawHandle`] newtype wrappers. Allows conversion to and from a newtype
/// in generic code.
pub trait HandleType: Copy + Clone {
    /// Takes a bare handle and wraps it into our newtype wrapper.
    ///
    /// # Safety
    ///
    /// There's no direct unsoundness caused by this interface, but logically this is similar to
    /// casting a pointer. To discourage people discarding handle's type information
    unsafe fn from_bare_handle(from: RawHandle) -> Self;

    /// Converts a wrapped handle back into the inner raw handle.
    fn to_bare_handle(self) -> RawHandle;
}

/// Trait used for constraining the types used with [`Handle`]. We require a type that at least
/// has a name we can use in the [`Debug`] impl.
///
/// Needs all the other traits so the derives work on [`Handle`]. This should only be implemented
/// on ZST structs like `struct Thing;` anyway. It's highly recommended to use [`make_handle_id!`]
/// to do this for you.
pub trait HandleId: Copy + Clone + Ord + PartialOrd + Eq + PartialEq + Hash {
    fn name() -> &'static str;
}

/// The base type of object handle. The generic 'T' identifies the type of resource the handle will
/// refer to.
///
/// [`HandleId`] must be implemented on the generic argument. It is highly recommended that you use
/// [`make_handle_id!`] to create the handle IDs.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct Handle<T: HandleId> {
    pub(crate) handle: RawHandle,
    pub(crate) phantom: PhantomData<T>,
}

impl<T: HandleId> Debug for Handle<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.handle.debug_newtype(f, T::name())
    }
}

impl<T: HandleId> HandleType for Handle<T> {
    #[inline(always)]
    unsafe fn from_bare_handle(from: RawHandle) -> Self {
        Self {
            handle: from,
            phantom: Default::default(),
        }
    }

    #[inline(always)]
    fn to_bare_handle(self) -> RawHandle {
        self.handle
    }
}

macro_rules! make_handle_id {
    ($name: ident) => {
        #[derive(
            ::core::marker::Copy,
            ::core::clone::Clone,
            ::core::cmp::Ord,
            ::core::cmp::PartialOrd,
            ::core::cmp::Eq,
            ::core::cmp::PartialEq,
            ::core::hash::Hash,
        )]
        pub struct $name;

        impl $crate::internal::handle::HandleId for $name {
            fn name() -> &'static str {
                stringify!($name)
            }
        }
    };
}
pub(crate) use make_handle_id;
