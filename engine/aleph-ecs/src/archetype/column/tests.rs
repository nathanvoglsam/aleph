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
use std::ptr::NonNull;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::archetype::column::Column;

#[derive(Default)]
struct AllocCounters {
    alloc_count: AtomicUsize,
    dealloc_count: AtomicUsize,
}

/// Allocator that delegates to [`Global`] while counting allocation/deallocation calls, so tests
/// can verify that [`Column`] actually frees its memory on drop.
#[derive(Clone)]
struct CountingAllocator {
    counters: Arc<AllocCounters>,
}

impl CountingAllocator {
    fn new() -> (Self, Arc<AllocCounters>) {
        let counters = Arc::new(AllocCounters::default());
        (
            Self {
                counters: counters.clone(),
            },
            counters,
        )
    }
}

unsafe impl Allocator for CountingAllocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        self.counters.alloc_count.fetch_add(1, Ordering::Relaxed);
        Global.allocate(layout)
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        self.counters.dealloc_count.fetch_add(1, Ordering::Relaxed);
        unsafe { Global.deallocate(ptr, layout) }
    }
}

#[test]
fn column_resize_preserves_stored_elements() {
    let mut column = Column::new_in(Layout::new::<u64>(), std::alloc::Global);
    column.resize(4);

    unsafe {
        for (index, value) in [10_u64, 20, 30, 40].into_iter().enumerate() {
            column
                .get_at_index(index)
                .unwrap()
                .cast::<u64>()
                .write(value);
        }
    }

    column.resize(2);
    unsafe {
        assert_eq!(*column.get_at_index(0).unwrap().cast::<u64>().as_ref(), 10);
        assert_eq!(*column.get_at_index(1).unwrap().cast::<u64>().as_ref(), 20);
    }
    assert!(column.get_at_index(2).is_none());

    column.resize(4);
    unsafe {
        assert_eq!(*column.get_at_index(0).unwrap().cast::<u64>().as_ref(), 10);
        assert_eq!(*column.get_at_index(1).unwrap().cast::<u64>().as_ref(), 20);
        column.get_at_index(3).unwrap().cast::<u64>().write(50);
    }
}

#[test]
fn column_swap_remove_uses_live_len_not_capacity() {
    let mut column = Column::new_in(Layout::new::<u64>(), std::alloc::Global);

    // Allocate capacity for 4 elements, but only treat 3 as live.
    column.resize(4);

    unsafe {
        for (index, value) in [10_u64, 20, 30].into_iter().enumerate() {
            column
                .get_at_index(index)
                .unwrap()
                .cast::<u64>()
                .write(value);
        }
    }

    // Removing the first live element must copy the last *live* element (index 2, value 30) into
    // its place, not the last *allocated* element (index 3, uninitialized garbage).
    column.swap_remove(0, 3);

    unsafe {
        assert_eq!(*column.get_at_index(0).unwrap().cast::<u64>().as_ref(), 30);
        assert_eq!(*column.get_at_index(1).unwrap().cast::<u64>().as_ref(), 20);
    }

    // Capacity must be unchanged by a removal.
    assert!(column.get_at_index(3).is_some());

    // Removing the last live element is a no-op copy-wise; capacity is still untouched.
    column.swap_remove(2, 3);
    assert!(column.get_at_index(3).is_some());
}

#[test]
fn column_new_is_empty() {
    let column = Column::new_in(Layout::new::<u64>(), Global);

    // A freshly constructed column has no allocation and reports no elements.
    assert!(column.get().is_none());
    assert!(column.get_at_index(0).is_none());
}

#[test]
fn column_element_layout_round_trips() {
    let layout = Layout::from_size_align(16, 32).unwrap();
    let column = Column::new_in(layout, Global);

    let got = column.element_layout();
    assert_eq!(got.size(), layout.size());
    assert_eq!(got.align(), layout.align());
}

#[test]
fn column_grow_zero_initializes_new_memory() {
    let mut column = Column::new_in(Layout::new::<u64>(), Global);

    // Pre-populate the first two elements with non-zero values.
    column.grow_to_fit(2);
    unsafe {
        column.get_at_index(0).unwrap().cast::<u64>().write(1);
        column.get_at_index(1).unwrap().cast::<u64>().write(2);
    }

    // Growing must leave the existing elements untouched and zero the new region.
    column.grow_to_fit(4);
    unsafe {
        assert_eq!(*column.get_at_index(0).unwrap().cast::<u64>().as_ref(), 1);
        assert_eq!(*column.get_at_index(1).unwrap().cast::<u64>().as_ref(), 2);
        assert_eq!(*column.get_at_index(2).unwrap().cast::<u64>().as_ref(), 0);
        assert_eq!(*column.get_at_index(3).unwrap().cast::<u64>().as_ref(), 0);
    }
}

#[test]
fn column_grow_is_no_op_when_not_growing() {
    let (allocator, counters) = CountingAllocator::new();
    let mut column = Column::new_in(Layout::new::<u64>(), allocator);

    column.grow_to_fit(4);
    unsafe {
        column.get_at_index(0).unwrap().cast::<u64>().write(10);
    }
    let alloc_count_after_grow = counters.alloc_count.load(Ordering::Relaxed);
    assert!(alloc_count_after_grow > 0);

    // Asking to grow to the same, or a smaller, count must not allocate.
    column.grow_to_fit(4);
    column.grow_to_fit(1);
    assert_eq!(
        counters.alloc_count.load(Ordering::Relaxed),
        alloc_count_after_grow
    );

    // Data is still intact.
    unsafe {
        assert_eq!(*column.get_at_index(0).unwrap().cast::<u64>().as_ref(), 10);
    }
}

#[test]
fn column_shrink_is_no_op_when_not_shrinking() {
    let (allocator, counters) = CountingAllocator::new();
    let mut column = Column::new_in(Layout::new::<u64>(), allocator);

    column.grow_to_fit(4);
    let alloc_count = counters.alloc_count.load(Ordering::Relaxed);

    // Asking to shrink to the same, or a larger, count must not deallocate.
    column.shrink_to_fit(4);
    column.shrink_to_fit(8);
    assert_eq!(counters.dealloc_count.load(Ordering::Relaxed), 0);
    assert_eq!(counters.alloc_count.load(Ordering::Relaxed), alloc_count);

    // Capacity is still 4.
    assert!(column.get_at_index(3).is_some());
}

#[test]
fn column_shrink_to_zero_frees_and_empties() {
    let (allocator, counters) = CountingAllocator::new();
    let mut column = Column::new_in(Layout::new::<u64>(), allocator);

    column.grow_to_fit(4);
    assert_eq!(counters.alloc_count.load(Ordering::Relaxed), 1);
    assert_eq!(counters.dealloc_count.load(Ordering::Relaxed), 0);

    column.shrink_to_fit(0);

    // The allocation is released and the column reports as empty.
    assert_eq!(counters.dealloc_count.load(Ordering::Relaxed), 1);
    assert!(column.get().is_none());
    assert!(column.get_at_index(0).is_none());

    // The column can be grown again afterwards.
    column.grow_to_fit(2);
    assert!(column.get_at_index(1).is_some());
}

#[test]
fn column_resize_dispatch_grow_and_shrink() {
    let mut column = Column::new_in(Layout::new::<u64>(), Global);

    // Grow from empty.
    column.resize(3);
    assert!(column.get_at_index(2).is_some());
    assert!(column.get_at_index(3).is_none());

    // Shrink.
    column.resize(1);
    assert!(column.get_at_index(0).is_some());
    assert!(column.get_at_index(1).is_none());

    // Equal resize is a no-op.
    column.resize(1);
    assert!(column.get_at_index(0).is_some());
    assert!(column.get_at_index(1).is_none());

    // Shrink to zero then re-grow.
    column.resize(0);
    assert!(column.get().is_none());
    column.resize(2);
    assert!(column.get_at_index(1).is_some());
}

#[test]
fn column_indexing_out_of_bounds_returns_none() {
    let mut column = Column::new_in(Layout::new::<u64>(), Global);
    column.resize(2);

    assert!(column.get_at_index(0).is_some());
    assert!(column.get_at_index(1).is_some());
    assert!(column.get_at_index(2).is_none());
    assert!(column.get_at_index(usize::MAX).is_none());
}

#[repr(align(64))]
#[allow(dead_code)]
struct OverAligned([u8; 16]);

#[test]
fn column_elements_are_aligned() {
    let layout = Layout::new::<OverAligned>();
    let mut column = Column::new_in(layout, Global);
    column.resize(8);

    for i in 0..8 {
        let ptr = column.get_at_index(i).unwrap();
        assert_eq!(ptr.as_ptr() as usize % layout.align(), 0);
    }
}

#[test]
#[should_panic(expected = "Live length")]
fn column_swap_remove_panics_when_len_exceeds_capacity() {
    let mut column = Column::new_in(Layout::new::<u64>(), Global);
    column.resize(2);

    // `len` must never exceed the allocated capacity.
    column.swap_remove(0, 3);
}

#[test]
#[should_panic(expected = "Index")]
fn column_swap_remove_panics_when_index_out_of_bounds() {
    let mut column = Column::new_in(Layout::new::<u64>(), Global);
    column.resize(2);

    // `index` must be within the live region.
    column.swap_remove(2, 2);
}

#[test]
fn column_drop_frees_non_empty_allocation() {
    let (allocator, counters) = CountingAllocator::new();

    {
        let mut column = Column::new_in(Layout::new::<u64>(), allocator);
        column.grow_to_fit(4);
        assert_eq!(counters.alloc_count.load(Ordering::Relaxed), 1);
        assert_eq!(counters.dealloc_count.load(Ordering::Relaxed), 0);
    }

    // The allocation is released exactly once when the column drops.
    assert_eq!(counters.dealloc_count.load(Ordering::Relaxed), 1);
}

#[test]
fn column_drop_is_no_op_when_empty() {
    let (allocator, counters) = CountingAllocator::new();

    {
        let _column = Column::new_in(Layout::new::<u64>(), allocator);
    }

    // An empty column never allocated, so it must not deallocate either.
    assert_eq!(counters.alloc_count.load(Ordering::Relaxed), 0);
    assert_eq!(counters.dealloc_count.load(Ordering::Relaxed), 0);
}
