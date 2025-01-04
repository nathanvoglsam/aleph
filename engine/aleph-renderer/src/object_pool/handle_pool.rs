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

use std::mem::ManuallyDrop;
use std::ops::{Deref, DerefMut};

use thiserror::Error;

use super::handle::{Generation, HandleFields};
use super::Handle;

/// A generic generational arena handle allocator.
pub struct HandlePool<T> {
    slots: Vec<HandleData<T>>,
}

impl<T> HandlePool<T> {
    pub fn new() -> Self {
        // Construct the pool storage, with the zeroeth element containing the head of the free list
        // linked list. It's default initialized to empty (by pointing to itself)
        let mut slots = Vec::with_capacity(1);
        slots.push(HandleData {
            generation: Generation::new(),
            variant: DataUnion { free_link: 0 },
        });

        Self { slots }
    }

    pub fn alloc(&mut self, data: T) -> Handle {
        debug_assert!(
            self.slots.len() < u32::MAX as usize,
            "Can't allocate more than {} handles.",
            u32::MAX
        );

        // SAFETY: This is safe because the first slot is always the head of the free list and will
        //         never contain an entity.
        let slot_index = unsafe {
            // Take an item from the free list
            //
            // When the free list is empty this is the equivalent of the following pseudo snippet:
            // `slot[0].free_link = slot[0].free_link`. This is a no-op, and 'slot_index' will just
            // contain 0.
            //
            // When the free list is _not_ empty then this pops the element off the head of the
            // list.
            //
            // This means popping off the list is branchless!
            let slot = self.slots[0].variant.free_link;
            self.slots[0].variant.free_link = self.slots[slot as usize].variant.free_link;
            slot
        };

        let fields = if slot_index != 0 {
            // We've got a handle from the free list, so we revive the generation and pass a handle
            // out referring to this slot.
            let slot = &mut self.slots[slot_index as usize];

            debug_assert!(
                slot.generation.is_dead(),
                "Tried to allocate a live handle a second time!"
            );

            slot.generation.increment_assign();
            slot.variant.data = ManuallyDrop::new(data);
            HandleFields {
                pool_id: 0,
                type_id: 0,
                generation: slot.generation,
                slot_index,
            }
        } else {
            // Nothing in the free-list so we create a new slot
            let slot_index = u32::try_from(self.slots.len()).expect("Too many handles");

            let generation = Generation::new_live();
            self.slots.push(HandleData {
                generation,
                variant: DataUnion {
                    data: ManuallyDrop::new(data),
                },
            });

            HandleFields {
                pool_id: 0,
                type_id: 0,
                generation,
                slot_index,
            }
        };

        // SAFETY: Generation will always be non-zero as zero is a dead generation. This means
        //         the handle will always be non-zero and so will never be 'None'.
        unsafe { Handle::from_fields(fields).unwrap_unchecked() }
    }

    pub fn get_ref(&self, handle: Handle) -> Option<&T> {
        let fields = handle.to_fields();

        let slot = &self.slots[fields.slot_index as usize];

        // If the generations don't match then the handle does not refer to a handle from this pool.
        if slot.generation != fields.generation || slot.generation.is_dead() {
            return None;
        }

        // Safety: A live generation guarantees that we're pointing at a live object so the data
        //         variant must be the active union variant.
        unsafe { Some(slot.variant.data.deref()) }
    }

    pub fn get_mut(&mut self, handle: Handle) -> Option<&mut T> {
        let fields = handle.to_fields();

        let slot = &mut self.slots[fields.slot_index as usize];

        // If the generations don't match then the handle does not refer to a handle from this pool.
        if slot.generation != fields.generation || slot.generation.is_dead() {
            return None;
        }

        // Safety: A live generation guarantees that we're pointing at a live object so the data
        //         variant must be the active union variant.
        unsafe { Some(slot.variant.data.deref_mut()) }
    }

    pub fn free(&mut self, handle: Handle) -> Result<T, HandleFreeError> {
        let fields = handle.to_fields();

        // Safety: The zeroth element is _always_ the head of the free list so free_link will always
        //         be the active union variant. Making this safe
        //
        // We need to store this as we may need it while we have the other slot borrowed.
        let free_list_next = unsafe { self.slots[0].variant.free_link };

        let slot = &mut self.slots[fields.slot_index as usize];

        // If the generations don't match then the handle does not refer to a handle from this pool.
        //
        // We also check if the generation in the slot is dead. If it is then we're trying to free
        // an already freed handle. We shouldn't even be able to _get_ a handle with a dead
        // generation in it, but it's trivial to guard against this by checking the slot's
        // generation is alive before trying to free it.
        if slot.generation != fields.generation || slot.generation.is_dead() {
            return Err(HandleFreeError::NoEntity);
        }

        // SAFETY: We've already confirmed the slot is alive by the above check, so we're guaranteed
        //         to have a valid T to take from the data field in slot.
        let data = unsafe { ManuallyDrop::take(&mut slot.variant.data) };

        // Kill the generation to mark there's no data in the slot
        slot.generation.increment_assign();

        // Add the slot to the free list. This works by making the slot we just freed point to the
        // old head (THIS CHANGES THE ACTIVE UNION VARIANT), which may be '0', and then making slot
        // zero point at the slot we just freed. This handles going from 0 to 1 entry in the list
        // and appending to a non-empty list with no branches.
        slot.variant.free_link = free_list_next;
        self.slots[0].variant.free_link = fields.slot_index;

        Ok(data)
    }
}

impl<T: Copy> HandlePool<T> {
    pub fn get_copied(&self, handle: Handle) -> Option<T> {
        self.get_ref(handle).copied()
    }
}

impl<T> Drop for HandlePool<T> {
    fn drop(&mut self) {
        if std::mem::needs_drop::<T>() {
            self.slots.drain(..).for_each(|mut v| {
                if v.generation.is_alive() {
                    // SAFETY: Inside the drop fn any slot with a live generation _must_ have a live
                    //         data field too. This means it is sound (and expected of us) to free
                    //         any data objects still live within the pool.
                    unsafe { ManuallyDrop::drop(&mut v.variant.data) }
                }
            });
        }
    }
}

#[derive(Error, Debug)]
pub enum HandleFreeError {
    #[error("The entity is not found in this pool")]
    NoEntity,
}

union DataUnion<T> {
    /// Variant that stores handle data
    data: ManuallyDrop<T>,

    /// Variant that stores a link in the linked list of free handles
    free_link: u32,
}

/// The data stored associated with a slot in a [HandlePool].
struct HandleData<T> {
    /// The generation of the handle data
    generation: Generation,

    /// Stores either a free-list link or the handle payload
    variant: DataUnion<T>,
}

#[cfg(test)]
mod tests {
    use crate::test_utils::DropCanary;
    use crate::HandlePool;

    #[test]
    pub fn test_handle_pool_alloc_free() {
        let mut pool = HandlePool::new();

        let canary = DropCanary::new();
        let handle = pool.alloc(canary.clone());

        assert_eq!(canary.strong_count(), 2);

        pool.free(handle).unwrap();

        assert_eq!(canary.strong_count(), 1);
    }

    #[test]
    pub fn test_handle_pool_drop_alloc_data() {
        let mut pool = HandlePool::new();

        let canary = DropCanary::new();
        let _handle = pool.alloc(canary.clone());

        assert_eq!(canary.strong_count(), 2);

        drop(pool);

        assert_eq!(canary.strong_count(), 1);
    }

    #[test]
    pub fn test_handle_pool_double_drop() {
        let mut pool = HandlePool::new();

        let canary = DropCanary::new();
        let handle = pool.alloc(canary.clone());

        assert_eq!(canary.strong_count(), 2);

        assert!(pool.free(handle).is_ok());
        assert_eq!(canary.strong_count(), 1);

        assert!(pool.free(handle).is_err());
        assert_eq!(canary.strong_count(), 1);
    }
}
