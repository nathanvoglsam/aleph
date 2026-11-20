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
use std::alloc::{AllocError, Allocator, Global, Layout};
use std::cell::Cell;
use std::mem::size_of;
use std::ptr::NonNull;

use crate::blink_alloc::blink::Blink;
use crate::blink_alloc::local::BlinkAlloc;

#[test]
fn test_local_alloc() {
    let mut blink = BlinkAlloc::new();

    let ptr = blink
        .allocate(Layout::new::<usize>())
        .unwrap()
        .cast::<usize>();
    unsafe {
        std::ptr::write(ptr.as_ptr(), 42);
    }

    blink.reset();
}

#[test]
fn test_bad_iter() {
    struct OneTimeGlobal {
        served: Cell<bool>,
    }

    unsafe impl Allocator for OneTimeGlobal {
        fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
            if self.served.get() {
                Err(AllocError)
            } else {
                self.served.set(true);
                Global.allocate(layout)
            }
        }

        unsafe fn deallocate(&self, ptr: std::ptr::NonNull<u8>, layout: Layout) {
            unsafe { Global.deallocate(ptr, layout) }
        }
    }

    const ELEMENT_COUNT: usize = 2000;
    const ELEMENT_SIZE: usize = size_of::<u32>();

    let mut blink = Blink::new_in(BlinkAlloc::with_chunk_size_in(
        ELEMENT_SIZE * ELEMENT_COUNT,
        OneTimeGlobal {
            served: Cell::new(false),
        },
    ));

    blink
        .emplace()
        .from_iter((0..ELEMENT_COUNT as u32).filter(|_| true));

    blink.reset();
}

#[test]
fn test_reuse() {
    struct ControlledGlobal {
        enabled: Cell<bool>,
        last: Cell<bool>,
    }

    unsafe impl Allocator for ControlledGlobal {
        fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
            if !self.enabled.get() {
                return Err(AllocError);
            }
            if self.last.get() {
                self.enabled.set(false);
            }
            Global.allocate(layout)
        }

        unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
            unsafe { Global.deallocate(ptr, layout) }
        }
    }

    let allocator = ControlledGlobal {
        enabled: Cell::new(true),
        last: Cell::new(false),
    };

    let mut alloc = BlinkAlloc::with_chunk_size_in(0, &allocator);

    for _ in 0..123 {
        alloc.allocate(Layout::new::<u32>()).unwrap();
    }
    alloc.reset();

    allocator.last.set(false);

    for _ in 0..123 {
        alloc.allocate(Layout::new::<u32>()).unwrap();
    }
}

#[test]
fn test_emplace_no_drop() {
    use std::borrow::ToOwned;

    struct Foo<'a>(&'a String);

    impl Drop for Foo<'_> {
        fn drop(&mut self) {
            panic!("Dropped");
        }
    }

    let mut blink = Blink::new();
    let s = "Hello".to_owned();
    let foo = blink.emplace_no_drop().value(Foo(&s));
    assert_eq!(foo.0, "Hello");
    let world = blink.put("World".to_owned());
    // Would be unsound if `foo` could be dropped.
    foo.0 = world;
    blink.reset();
    // assert_eq!(foo.0, "Universe"); // Cannot compile. `foo` does not outlive reset.
}

#[test]
fn test_vec() {
    let mut blink_alloc = BlinkAlloc::new();
    let mut vec = Vec::new_in(&blink_alloc);
    vec.extend([1, 2, 3]);

    vec.push(4);
    vec.extend(5..6);
    vec.push(6);

    assert_eq!(vec, [1, 2, 3, 4, 5, 6]);
    drop(vec);
    blink_alloc.reset();
}
