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

use crate::archetype::{Archetype, TypeLayout};
use crate::world::component_index::ComponentIndex;

#[test]
fn shrink_to_fit_updates_capacity() {
    let components = ComponentIndex::new();
    let mut archetype = Archetype::new(&components, TypeLayout::empty());

    // Allocate 5 live entities. `minimum_capacity_for_entity_count` rounds 5 up to 8.
    assert_eq!(archetype.allocate_entities(5), 0);
    assert_eq!(archetype.len(), 5);
    assert_eq!(archetype.capacity(), 8);

    // Remove down to a single live entity, leaving 4 stale (uninitialized) slots.
    for _ in 0..4 {
        archetype.remove_entity(0);
    }
    assert_eq!(archetype.len(), 1);

    // Shrinking must update `capacity` to reflect the shrunken columns. `shrink_to_fit` selects
    // the smallest capacity that fits `len`, which is 1 here.
    archetype.shrink_to_fit();
    assert_eq!(archetype.capacity(), 1);
}
