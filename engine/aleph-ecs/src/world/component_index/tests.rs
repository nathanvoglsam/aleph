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

use crate::component::{ComponentDescription, ComponentId};
use crate::world::component_index::ComponentIndex;

/// Builds a `ComponentDescription` directly (bypassing the global auto-registration counter, which
/// must not be touched from within tests) so the stored `id` can be set to an arbitrary value.
fn description_with_id(id: u32) -> ComponentDescription {
    ComponentDescription {
        id: ComponentId(id),
        size: 8,
        align: 8,
        name: crate::nstr::nstr!("test"),
        destructor: None,
    }
}

#[test]
fn push_assigns_index_matching_id() {
    let mut index = ComponentIndex::new();

    // The supplied descriptions carry IDs that do not match their eventual position in the index.
    let a = description_with_id(999);
    let b = description_with_id(999);

    // `push` assigns the ID from the index position and must return that assigned ID.
    let id_a = index.push(&a);
    let id_b = index.push(&b);

    assert_eq!(id_a, ComponentId(0));
    assert_eq!(id_b, ComponentId(1));

    // The stored descriptions must have their `id` field rewritten to match the assigned index,
    // so that lookups keyed on `desc.id` agree with the index position.
    assert_eq!(index.get(ComponentId(0)).unwrap().desc.id, ComponentId(0));
    assert_eq!(index.get(ComponentId(1)).unwrap().desc.id, ComponentId(1));
}
