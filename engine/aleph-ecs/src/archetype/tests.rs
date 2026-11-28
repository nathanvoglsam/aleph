use std::alloc::Layout;

use crate::archetype::column::Column;

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
