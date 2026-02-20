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

use std::collections::HashMap;

use crate::ObjectDescription;

/// This is the head of the object system registry. All objects that interact with the object system
/// will push an entry onto this list during the static init phase.
pub static __UNSAFE_REGISTRY_HEAD: init_list::InitList<&'static ObjectDescription> =
    init_list::InitList::new();

/// Utility function that will walk the list of registered types and assert if there are any
/// duplicate type IDs registered.
pub fn assert_no_duplicate_ids_registered() {
    let mut types = HashMap::new();
    for object in __UNSAFE_REGISTRY_HEAD.iter().copied() {
        let existing = types.insert(object.id, object);
        if let Some(existing) = existing {
            assert_eq!(object.id, existing.id); // Just being careful
            panic!(
                "Colliding IObject type IDs detected. '{}' and '{}' have the same ID of '{}'!",
                existing.name, object.name, existing.id
            );
        }
    }
}
