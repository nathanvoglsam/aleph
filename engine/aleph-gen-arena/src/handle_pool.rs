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

use aleph_alloc::BVec;
use aleph_alloc::alloc::{Allocator, Global};
use thiserror::Error;

use super::handle::{Generation, HandleFields, RawHandle};

/// A _part_ of a generational arena.
///
/// HandlePool implements a free-list of handles, manages handle generation values and enables
/// mapping handles to generic 'u32' values. That 'u32' value is typically expected to be an index
/// into some other array. Some owning _object_ pool would use this as the core of how it implements
/// generation handles.
pub struct HandlePool<A: Allocator = Global> {
    /// The backing storage for the handles
    slots: BVec<HandleData, A>,
}

impl HandlePool<Global> {
    /// Constructs a new, empty [`HandlePool`].
    pub fn new() -> Self {
        // Construct the pool storage, with the zeroeth element containing the head of the free list
        // linked list. It's default initialized to empty (by pointing to itself)
        let mut slots = BVec::with_capacity(1);
        slots.push(HandleData {
            generation: Generation::new_dead(),
            data: 0,
        });

        Self { slots }
    }
}

impl<A: Allocator> HandlePool<A> {
    /// Constructs a new, empty [`HandlePool`].
    pub fn new_in(a: A) -> Self {
        // Construct the pool storage, with the zeroeth element containing the head of the free list
        // linked list. It's default initialized to empty (by pointing to itself)
        let mut slots = BVec::with_capacity_in(1, a);
        slots.push(HandleData {
            generation: Generation::new_dead(),
            data: 0,
        });

        Self { slots }
    }

    /// Create a new handle, associating and storing the given 'data' item with it.
    ///
    /// # Panic
    ///
    /// Will panic if there is not enough space in the pool. We chose a panic on OOM scenario
    /// because we expect our average use case would OOM on the system or GPU allocator long before
    /// hitting OOM on a handle pool.
    pub fn alloc(&mut self, data: u32) -> RawHandle {
        debug_assert!(
            self.slots.len() < u32::MAX as usize,
            "Can't allocate more than {} handles.",
            u32::MAX
        );

        let slot_index = self.pop_free_list();

        let fields = if slot_index != 0 {
            // We've got a handle from the free list, so we revive the generation and pass a handle
            // out referring to this slot.
            let slot = &mut self.slots[slot_index as usize];

            let generation = slot.generation.revive();

            slot.data = data;
            HandleFields {
                generation,
                slot_index,
            }
        } else {
            // Nothing in the free-list so we create a new slot
            let slot_index = u32::try_from(self.slots.len()).expect("Too many handles");

            let generation = Generation::new_live();
            self.slots.push(HandleData { generation, data });

            HandleFields {
                generation,
                slot_index,
            }
        };

        // SAFETY: Generation will always be non-zero as zero is a dead generation. This means
        //         the handle will always be non-zero and so will never be 'None'.
        unsafe { RawHandle::from_fields(fields).unwrap_unchecked() }
    }

    /// Create a batch of new handles, initializing from the 'datas' iterator and writing the
    /// constructed handles into the 'dst' [`Vec`].
    ///
    /// This function will push new entries onto the end of dst. The elements will be pushed in
    /// the same order as 'datas' yields 'data' items. That is: `datas[i] -> dst[base + i]` where
    /// 'base' is the existing len of the vector.
    ///
    /// # Panic
    ///
    /// This will panic like [`HandlePool::alloc`], see that function's docs for more info.
    #[allow(unused)]
    pub fn alloc_bulk(
        &mut self,
        dst: &mut Vec<RawHandle>,
        mut datas: impl ExactSizeIterator<Item = u32>,
    ) {
        // First we consume as many slots as we can from the free list
        loop {
            let data = if let Some(data) = datas.next() {
                data
            } else {
                // No more datas? We're done, exit.
                return;
            };

            let slot_index = self.pop_free_list();

            if slot_index != 0 {
                // We've got a handle from the free list, so we revive the generation and pass a handle
                // out referring to this slot.
                let slot = &mut self.slots[slot_index as usize];

                let generation = slot.generation.revive();

                slot.data = data;
                let fields = HandleFields {
                    generation,
                    slot_index,
                };
                // SAFETY: Generation will always be non-zero as zero is a dead generation.
                //         This means the handle will always be non-zero and so will never be
                //         'None'.
                let handle = unsafe { RawHandle::from_fields(fields).unwrap_unchecked() };
                dst.push(handle);
            } else {
                // If we've exhausted the free list then we break from the loop and try and
                // satisfy the remaining number of handles by allocating new ones.
                break;
            }
        }

        // Then we start consuming new slots once the free list is empty
        loop {
            let data = if let Some(data) = datas.next() {
                data
            } else {
                // No more datas? We're done, exit.
                return;
            };

            // Nothing in the free-list so we create a new slot
            let slot_index = u32::try_from(self.slots.len()).expect("Too many handles");

            let generation = Generation::new_live();
            self.slots.push(HandleData { generation, data });

            let fields = HandleFields {
                generation,
                slot_index,
            };

            // I hope the optimizer sees through this and skips the 'if generation == 0'
            let handle = RawHandle::from_fields(fields).unwrap();
            dst.push(handle);
        }
    }

    pub fn get_ref(&self, handle: RawHandle) -> Option<&u32> {
        let fields = handle.to_fields();

        let slot = &self.slots[fields.slot_index as usize];

        // If the generations don't match then the handle does not refer to a handle from this pool.
        if slot.generation != fields.generation || slot.generation.is_dead() {
            return None;
        }

        Some(&slot.data)
    }

    pub fn get(&self, handle: RawHandle) -> Option<u32> {
        self.get_ref(handle).copied()
    }

    pub fn get_mut(&mut self, handle: RawHandle) -> Option<&mut u32> {
        let fields = handle.to_fields();

        let slot = &mut self.slots[fields.slot_index as usize];

        // If the generations don't match then the handle does not refer to a handle from this pool.
        if slot.generation != fields.generation || slot.generation.is_dead() {
            return None;
        }

        Some(&mut slot.data)
    }

    pub fn free(&mut self, handle: RawHandle) -> Result<u32, HandleFreeError> {
        let fields = handle.to_fields();

        // The zeroth element is _always_ the head of the free list.
        // We need to store this as we may need it while we have the other slot borrowed.
        let free_list_next = self.slots[0].data;

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

        let data = slot.data;

        // Kill the generation to mark there's no data in the slot
        slot.generation.kill();

        // Add the slot to the free list. This works by making the slot we just freed point to the
        // old head (THIS CHANGES THE ACTIVE UNION VARIANT), which may be '0', and then making slot
        // zero point at the slot we just freed. This handles going from 0 to 1 entry in the list
        // and appending to a non-empty list with no branches.
        slot.data = free_list_next;
        self.slots[0].data = fields.slot_index;

        Ok(data)
    }

    /// Remove all handles from the pool and reset it back to the default state. After this returns
    /// the pool is logically empty.
    pub fn clear(&mut self) {
        self.slots.resize_with(1, || HandleData {
            generation: Generation::new_dead(),
            data: 0,
        });
    }

    /// Internal function that pops a free slot from the interwoven free list.
    ///
    /// If this returns zero then there is no entries in the free list and a new slot must be
    /// allocated.
    fn pop_free_list(&mut self) -> u32 {
        let slot_index = {
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
            let slot = self.slots[0].data;
            self.slots[0].data = self.slots[slot as usize].data;
            slot
        };
        slot_index
    }
}

#[derive(Error, Debug)]
pub enum HandleFreeError {
    #[error("The entity is not found in this pool")]
    NoEntity,
}

/// The data stored associated with a slot in a [HandlePool].
struct HandleData {
    /// The generation of the handle data
    generation: Generation,

    /// Stores either a free-list link or the handle payload
    data: u32,
}
