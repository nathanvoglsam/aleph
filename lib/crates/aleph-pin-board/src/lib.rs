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

use aleph_arena_drop_list::DropLink;
use aleph_type_id_hasher::TypeIdHasher;
use bumpalo::Bump;
use parking_lot::Mutex;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::mem::needs_drop;
use std::ptr::NonNull;

/// A data structure for publishing data keyed by type that can be shared among a group of threads
/// or tasks. Used as a mechanism for dynamically publishing data from one place to another in a
/// thread-safe manner.
///
/// The API is in some sense like a real world pin board or bulletin board. The [PinBoard] object
/// represents some 'place' you can publish 'notices' (objects) for future tasks to come and read
/// later. New notices of the same type replace old notices, readers will get a stable reference to
/// whichever was the most recent notice published at the time they called [PinBoard::get].
///
/// Once an object is published on the board it is immutable. All published messages will remain on
/// the board (and safe to access) until the board is cleared.
///
/// # Note
///
/// There is no ordering guarantee of which object a reader may get. [PinBoard] simply keeps a
/// record of the most recently published item for a specific type. Do not use [PinBoard] if you
/// need to access all messages that were published, instead use a channel or other more appropriate
/// mechanism.
///
/// This data structure is designed for systems where strong publisher/reader coordination is either
/// not needed or already being provided by another system.
///
/// # Note for Naughty Unsafe Users
///
/// The object pointers given out by the [PinBoard] with [PinBoard::get] are stable until
/// [PinBoard::clear] is called. Moving [PinBoard] will _not_ move the stored objects as they are
/// stored inside a bump allocated that isn't stored inside the [PinBoard] itself. This means they
/// _can_ be used with [std::pin::Pin] and you _could_ throw pointers around and they will remain
/// live until [PinBoard::clear] is called. Whether you _should_ or not?
pub struct PinBoard {
    /// The underlying 'slot' data structure that takes [TypeId]s and will yield the stored value
    tables: Vec<Mutex<Table>>,
}

impl PinBoard {
    /// Constructs a new [PinBoard]. This is relatively expensive as it needs to create an array of
    /// 256 mutex wrapped hash tables. Don't call this too often, prefer to create a set upfront
    /// and reuse them with [PinBoard::clear].
    pub fn new() -> Self {
        let tables = Vec::from_iter((0..256).into_iter().map(|_| Mutex::new(Table::new())));

        debug_assert_eq!(tables.len(), 256);

        Self { tables }
    }

    /// Publishes a new item into the [PinBoard]. Once published all future calls to [PinBoard::get]
    /// will yield the item we just published, until another item is published in its place.
    pub fn publish<T: Send + Sync + Any>(&self, v: T) {
        debug_assert_eq!(self.tables.len(), 256);

        let i = &self.tables[Self::id_to_index(TypeId::of::<T>())];
        let mut i = i.lock();

        // Store the object we're storing into the arena and get the reference as a type-erased
        // pointer
        let v = i.arena.alloc(v);
        let v = NonNull::from(v);

        // Insert the reference to our object into the ID -> ptr table
        i.table.insert(TypeId::of::<T>(), v.cast());

        // Only append to the drop list if we actually need to drop the object
        if needs_drop::<T>() {
            // Create and store the link in the dropper linked list for this object
            let mut v_dropper = DropLink::new::<T>(v);
            v_dropper.prev = i.drop_head;
            let v_dropper = i.arena.alloc(v_dropper);

            // Update the linked-list head for this table
            i.drop_head = Some(NonNull::from(v_dropper));
        }
    }

    /// Look up a published item by its type. May return None if no value has been published yet.
    pub fn get<T: Send + Sync + Any>(&self) -> Option<&T> {
        debug_assert_eq!(self.tables.len(), 256);

        let t = TypeId::of::<T>();
        let i = &self.tables[Self::id_to_index(t)];
        let i = i.lock();
        let ptr = i.table.get(&t).copied();

        ptr.map(|v| {
            // Safety: The PinBoard guarantees that whatever is stored in a slot for 'T' is always
            //         a 'T'
            unsafe { v.cast::<T>().as_ref() }
        })
    }

    /// Resets the [PinBoard] back to the default state, with no objects 'pinned' inside it. This
    /// internally resets the arena used for storing objects and clears the [TableSet] using
    /// [TableSet::clear].
    ///
    /// # Note
    ///
    /// This only frees memory from the arena allocator. The [TableSet] will still hold memory. To
    /// fully free all memory used by a [PinBoard] you must drop it.
    pub fn clear(&mut self) {
        debug_assert_eq!(self.tables.len(), 256);

        // Resets all the individual tables in the table set. This won't free any memory as we
        // internally use 'HashMap::clear'. This is intentional as the intended purpose is to reuse
        // the existing allocations to reduce how many times we have to hit the allocator.
        for i in self.tables.iter_mut() {
            let i = i.get_mut();
            i.table.clear();

            // Call drop on all the inserted objects
            let mut current = i.drop_head;
            while let Some(v) = current {
                // Safety: implementation and API guarantees that dropper only gets called once per
                //         object, and always on the correct type.
                unsafe {
                    let v = v.as_ref();
                    current = v.drop_object();
                }
            }
            i.drop_head = None;

            i.arena.reset();
        }
    }

    /// Internal function that converts a [TypeId] to an index into the internal table array.
    fn id_to_index(t: TypeId) -> usize {
        // Map a type id to a specific table by extracting the lower 64 bits of the type id modulo
        // 256. This will give us an index between 0-255.
        //
        // The distribution of TypeId should be uniform so this will reduce how frequently we will
        // collide on hash maps between threads and reduce lock contention.
        let t_hash = TypeIdHasher::hash(t);
        (t_hash % 256) as usize
    }
}

impl Drop for PinBoard {
    fn drop(&mut self) {
        for i in self.tables.drain(..) {
            let mut i = i.into_inner();

            // Safety: implementation and API guarantees that dropper only gets called once per
            //         object, and always on the correct type.
            unsafe {
                DropLink::drop_and_null(&mut i.drop_head);
            }
        }
    }
}

struct Table {
    /// The bump allocator arena used to allocate any newly inserted object
    pub arena: Bump,

    /// The table that maps TypeId -> object
    pub table: HashMap<TypeId, NonNull<()>>,

    /// The head of a linked list that contains a list of 'ptr + dropper' pairs used for dropping
    /// the objects stored inside this table (need virtual dispatch as they're stored type erased).
    pub drop_head: Option<NonNull<DropLink>>,
}

impl Table {
    pub fn new() -> Self {
        Self {
            arena: Default::default(),
            table: Default::default(),
            drop_head: Default::default(),
        }
    }
}

unsafe impl Send for Table {}
unsafe impl Sync for Table {}

#[cfg(test)]
mod tests {
    use crate::PinBoard;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    struct A(usize);
    struct B(u32);
    struct C(bool);

    struct D(Arc<AtomicUsize>);
    impl Drop for D {
        fn drop(&mut self) {
            self.0.fetch_add(1, Ordering::SeqCst);
        }
    }

    /// This test will fail to compile if [PinBoard] does not implement Send + Sync
    #[test]
    fn pin_board_is_send_sync() {
        fn send_sync_checker<T: Send + Sync>(v: T) -> T {
            v
        }

        let pin_board = PinBoard::new();
        let _pin_board = send_sync_checker(pin_board);

        assert!(true)
    }

    #[test]
    fn test_pin_board() {
        let mut pin_board = PinBoard::new();

        pin_board.publish(A(21));
        pin_board.publish(B(3));

        let a: &A = pin_board.get().unwrap();
        assert_eq!(a.0, 21);

        let b: &B = pin_board.get().unwrap();
        assert_eq!(b.0, 3);

        pin_board.publish(C(true));
        pin_board.publish(A(56));

        let a2: &A = pin_board.get().unwrap();
        assert_eq!(a2.0, 56);
        assert_ne!(a.0, a2.0);

        let c: &C = pin_board.get().unwrap();
        assert!(c.0);

        pin_board.clear();

        assert!(pin_board.get::<A>().is_none());
        assert!(pin_board.get::<B>().is_none());
        assert!(pin_board.get::<C>().is_none());
    }

    #[test]
    fn test_pin_board_drop_1() {
        let mut pin_board = PinBoard::new();

        let v = Arc::new(AtomicUsize::new(0));
        pin_board.publish(D(v.clone()));

        pin_board.clear();

        assert_eq!(v.load(Ordering::SeqCst), 1);

        pin_board.clear();

        assert_eq!(v.load(Ordering::SeqCst), 1);

        drop(pin_board);

        assert_eq!(v.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_pin_board_drop_2() {
        let pin_board = PinBoard::new();

        let v = Arc::new(AtomicUsize::new(0));
        pin_board.publish(D(v.clone()));

        drop(pin_board);

        assert_eq!(v.load(Ordering::SeqCst), 1);
    }
}
