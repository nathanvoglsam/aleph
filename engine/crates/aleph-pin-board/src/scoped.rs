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
use std::collections::HashMap;
use std::ptr::NonNull;

pub trait ItemIdentifier: Any {
    type Output<'a>: Send + Sync + 'a;
}

pub struct ScopedParamBoard {
    /// The bump allocator arena used to allocate any newly inserted object
    arena: blink_alloc::Blink,

    /// The table that maps TypeId -> object
    table: HashMap<TypeId, NonNull<()>>,
}

impl Default for ScopedParamBoard {
    fn default() -> Self {
        Self::new()
    }
}

impl ScopedParamBoard {
    pub fn new() -> Self {
        Self {
            arena: Default::default(),
            table: Default::default(),
        }
    }

    pub fn scope(&mut self, f: impl FnOnce(&mut BoardScope)) {
        let mut scope = BoardScope { v: self };
        f(&mut scope)
    }
}

unsafe impl Send for ScopedParamBoard {}
unsafe impl Sync for ScopedParamBoard {}

pub struct BoardScope<'a> {
    v: &'a mut ScopedParamBoard,
}

impl<'a> BoardScope<'a> {
    /// Publishes a new item into the [PinBoard]. Once published all future calls to [PinBoard::get]
    /// will yield the item we just published, until another item is published in its place.
    pub fn publish<'s, 'i: 's, T: ItemIdentifier>(&'s mut self, v: T::Output<'i>) {
        // Store the object we're storing into the arena and get the reference as a type-erased
        // pointer
        let v = unsafe { self.v.arena.emplace_unchecked::<T::Output<'i>>().value(v) };
        let v = NonNull::from(v);

        // Insert the reference to our object into the ID -> ptr table
        self.v.table.insert(TypeId::of::<T>(), v.cast());
    }

    /// Look up a published item by its type. May return None if no value has been published yet.
    pub fn get<'s, T: ItemIdentifier>(&'s self) -> Option<&T::Output<'s>> {
        let t = TypeId::of::<T>();
        let ptr = self.v.table.get(&t).copied();

        ptr.map(|v| {
            // Safety: The PinBoard guarantees that whatever is stored in a slot for 'T' is always
            //         a 'T'
            unsafe { v.cast::<T::Output<'s>>().as_ref() }
        })
    }
}

impl<'a> Drop for BoardScope<'a> {
    fn drop(&mut self) {
        self.v.table.clear();
        self.v.arena.reset();
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    use crate::scoped::ItemIdentifier;
    use crate::scoped::ScopedParamBoard;

    struct IdentA;
    impl ItemIdentifier for IdentA {
        type Output<'a> = usize;
    }

    struct IdentB;
    impl ItemIdentifier for IdentB {
        type Output<'a> = &'a usize;
    }

    struct IdentC;
    impl ItemIdentifier for IdentC {
        type Output<'a> = &'a mut usize;
    }

    struct IdentD;
    impl ItemIdentifier for IdentD {
        type Output<'a> = D;
    }

    struct D(Arc<AtomicUsize>);
    impl Drop for D {
        fn drop(&mut self) {
            self.0.fetch_add(1, Ordering::SeqCst);
        }
    }

    /// This test will fail to compile if [PinBoard] does not implement Send + Sync
    #[test]
    fn param_board_is_send_sync() {
        fn send_sync_checker<T: Send + Sync>(v: T) -> T {
            v
        }

        let board = ScopedParamBoard::new();
        let _board = send_sync_checker(board);
    }

    #[test]
    fn test_param_board() {
        let mut board = ScopedParamBoard::new();

        let b = 3usize;
        let mut c = 420usize;
        board.scope(|board| {
            board.publish::<IdentA>(21usize);
            board.publish::<IdentB>(&b);

            let a = *board.get::<IdentA>().unwrap();
            assert_eq!(a, 21);

            let b = **board.get::<IdentB>().unwrap();
            assert_eq!(b, 3);

            board.publish::<IdentC>(&mut c);
            board.publish::<IdentA>(56);

            let a2 = board.get::<IdentA>().unwrap();
            assert_eq!(*a2, 56);
            assert_ne!(a, *a2);

            let c_ref = board.get::<IdentC>().unwrap();
            assert_eq!(**c_ref, 420);
        });
    }

    #[test]
    fn test_param_board_drop_1() {
        let mut board = ScopedParamBoard::new();

        let v = Arc::new(AtomicUsize::new(0));
        assert_eq!(Arc::strong_count(&v), 1);

        board.scope(|board| {
            let d = D(v.clone());
            assert_eq!(Arc::strong_count(&v), 2);
            board.publish::<IdentD>(d);

            assert_eq!(Arc::strong_count(&v), 2);
        });

        assert_eq!(Arc::strong_count(&v), 1);

        drop(board);

        assert_eq!(Arc::strong_count(&v), 1);
    }
}
