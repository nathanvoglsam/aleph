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
//! Most structures in this module are repr(C) for FFI reasons
//!

use crate::world::Generation;
use std::num::NonZeroU32;

///
/// This index wrapper represents an index into an `EntityStorage`.
///
/// This is used to better document the purpose of various indexes that would've otherwise been
/// plain `u32` fields.
///
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct EntityIndex(pub NonZeroU32);

///
/// This represents an ID that refers to a specific entity.
///
/// # Info
///
/// Needs to be 8 byte aligned as this should have the same size and alignment as a u64
///
#[repr(C)]
#[repr(align(8))]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct EntityId {
    /// The generation of the slot in the entity list when this ID was allocated
    pub generation: Generation,

    /// The index inside the entity list this ID was allocated to
    pub index: Option<EntityIndex>,
}

impl EntityId {
    #[inline]
    pub const fn null() -> Self {
        Self {
            generation: Generation::new(),
            index: None,
        }
    }

    /// Returns whether this entity reference is a null reference.
    #[inline]
    pub const fn is_null(self) -> bool {
        self.generation.is_dead() && self.index.is_none()
    }
}

impl From<u64> for EntityId {
    /// Theoretically this whole thing should compile to a no-op as we're just manually spelling out
    /// the semantics of something that could be implemented with a mem::transmute.
    ///
    /// No point adding extra unsafe if it can be avoided
    #[inline]
    fn from(v: u64) -> Self {
        // Extract the high 32 bits of the u64 id, which we use as the entity index
        let first = v >> 32 & 0xFFFFFFFF;
        let first = first as u32;

        // Extract the low 32 bits of the u64 id, which we use as the generation
        let second = v & 0xFFFFFFFF;
        let second = second as u32;

        Self {
            generation: Generation::from_raw(second),
            index: NonZeroU32::new(first).map(|v| EntityIndex(v)),
        }
    }
}

impl Into<u64> for EntityId {
    /// Theoretically this whole thing should compile to a no-op as we're just manually spelling out
    /// the semantics of something that could be implemented with a mem::transmute.
    ///
    /// No point adding extra unsafe if it can be avoided
    #[inline]
    fn into(self) -> u64 {
        // Convert the generation index into the low 32 bits of a u64
        let first = self.generation.into_inner() as u64;

        // Convert the entity index into the high 32 bits of a u64
        let second = self.index.map(|v| v.0.get()).unwrap_or(0) as u64;
        let second = second << 32;

        // Combine the two haves to create a whole u64 id and return it
        first | second
    }
}
