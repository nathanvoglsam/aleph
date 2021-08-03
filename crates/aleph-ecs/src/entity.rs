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

use crate::{ArchetypeEntityIndex, ArchetypeIndex, Generation};
use std::{
    io::{Error, ErrorKind},
    num::NonZeroU32,
};
use virtual_buffer::VirtualVec;

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

///
/// This represents a reference to the location an entity is stored within the set of all archetypes
/// and the components inside that archetype.
///
#[repr(C)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct EntityLocation {
    /// The index inside the set of all archetypes the entity is a member of
    pub archetype: ArchetypeIndex,

    /// The index within the archetype the entity can be found at
    pub entity: ArchetypeEntityIndex,
}

///
/// This index wrapper represents an index into an `EntityStorage`, but with a specific usecase for
/// the free list linked-list encoded in the de-allocated entity slots.
///
/// This is used to better document the purpose of various indexes that would've otherwise been
/// plain `u32` fields.
///
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub struct EntityFreeListLink(pub u32);

///
/// This union represents a single entry in the `EntityStorage` structure's backing storage. The
/// free-list allocator reuses the data field of empty slots as links in a linked list. This union
/// is used to implement that.
///
#[repr(C)]
#[derive(Clone, Copy)]
pub union EntityEntryData {
    /// The location of an entity
    pub location: Option<EntityLocation>,

    /// The next free element
    pub next: EntityFreeListLink,
}

impl Default for EntityEntryData {
    #[inline]
    fn default() -> EntityEntryData {
        Self { location: None }
    }
}

///
/// This represents an entry within the world's entity list, and is what an EntityId refers to.
///
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct EntityEntry {
    /// The current generation of this entity entry
    pub generation: Generation,

    /// The location this entity entry points to, if it is alive
    pub data: EntityEntryData,
}

///
/// A data-structure that manages efficiently
///
pub struct EntityStorage {
    /// The backing storage for the entities
    entities: VirtualVec<EntityEntry>,

    /// The number of entities that are currently live
    count: usize,
}

impl EntityStorage {
    /// This creates a fresh EntityStorage datastructure where `capacity` specifies the maximum
    /// number of entities that can be allocated.
    ///
    /// The list is backed by a dedicated virtual memory allocation so capacity specifies the size
    /// of the allocation. Physical memory usage will only grow when a page in the allocation is
    /// touched so the capacity is safe to be large.
    ///
    /// It is recommended to make `capacity` something large like 1,048,576 as this sets the upper
    /// bound on the maximum number of entities that can be alive at any one time.
    pub fn new(capacity: u32) -> std::io::Result<EntityStorage> {
        if capacity < (u32::MAX - 1) {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Can't have more than (u32::MAX - 1) entities as capacity",
            ));
        }

        // Create the backing storage with the given total capacity
        let mut entities = VirtualVec::new((capacity + 1) as usize)?;

        // Push the first element of the list. This first element must always exist and serves as
        // the head of the free list.
        entities.resize(1, EntityEntry::default());

        let out = Self { entities, count: 0 };
        Ok(out)
    }

    /// Returns the number of entities that are live in this storage.
    #[inline]
    pub fn len(&self) -> usize {
        self.count
    }

    /// Looks up the location of the entity with the given ID.
    ///
    /// Will return None if the ID is invalid (dangling)
    #[inline]
    pub fn lookup(&self, id: EntityId) -> Option<EntityLocation> {
        let index = id.index?.0.get() as usize;

        // If the generations do not match then this ID is dangling
        if self.entities[index].generation == id.generation && id.generation.is_alive() {
            // SAFETY: The location field will always be the current live field if the generation
            //         is alive so accessing this is sound.
            let location = unsafe { self.entities[index].data.location };
            location
        } else {
            None
        }
    }

    /// Allocates a new entity ID with the given location data and returns the ID.
    #[inline]
    pub fn create(&mut self, location: EntityLocation) -> EntityId {
        // SAFETY: This is safe because the first slot is always the head of the free list and will
        //         never contain an entity.
        let slot = unsafe {
            // Take an item from the free list
            let slot = self.entities[0].data.next;
            self.entities[0].data.next = self.entities[slot.0 as usize].data.next;
            slot
        };

        // If the freelist is empty, slot will be 0, because the header
        // item will point to itself.
        let out = if slot.0 != 0 {
            // Assert the generation was actually dead before reviving it
            debug_assert!(self.entities[slot.0 as usize].generation.is_dead());

            // Increment the generation to revive the slot
            self.entities[slot.0 as usize].generation.increment_assign();

            let index = NonZeroU32::new(slot.0).unwrap();
            EntityId {
                generation: self.entities[slot.0 as usize].generation,
                index: Some(EntityIndex(index)),
            }
        } else {
            // Add a new entry if there are no free ones in the free list
            let slot = self.entities.len();
            self.entities.push(EntityEntry {
                generation: Generation::default().increment(),
                data: EntityEntryData {
                    location: Some(location),
                },
            });

            // Debug assert the new entity is alive
            debug_assert!(self.entities[slot].generation.is_alive());

            let index = NonZeroU32::new(slot as u32).unwrap();
            EntityId {
                generation: Generation::default().increment(),
                index: Some(EntityIndex(index)),
            }
        };

        self.count += 1;
        out
    }

    /// Attempts to free the given entity slot, if the ID is valid. If the ID is invalid this
    /// function does nothing and returns None. If the ID is valid then the function will add the
    /// slot to the free list and return the location the entity pointed to before being marked
    /// as free.
    #[inline]
    pub fn destroy(&mut self, id: EntityId) -> Option<EntityLocation> {
        let index = id.index?.0.get() as usize;

        // Check if the generations match, if they don't match we don't have a valid ID to free
        if self.entities[index].generation == id.generation && id.generation.is_alive() {
            // SAFETY: The union access is safe as location will always be the live field of the
            //         union for live entity slots. Both members are also plain old data and so
            //         the access is still safe anyway as all bit patterns are valid in either
            //         member.
            let location = unsafe {
                // Capture the location the slot points to before we clobber it
                let location = self.entities[index].data.location;

                // Add this entity slot to the free list
                self.entities[index].data.next = self.entities[0].data.next;
                self.entities[0].data.next = EntityFreeListLink(index as u32);

                location
            };

            // Increment the generation to mark this slot as dead
            self.entities[index].generation.increment_assign();

            self.count -= 1;
            location
        } else {
            None
        }
    }
}
