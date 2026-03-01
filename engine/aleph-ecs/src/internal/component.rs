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

use std::sync::LazyLock;

use aleph_alloc::instrumentation::system;
use aleph_alloc::{BHashMap, BVec};
use init_list::{InitList, ListItem};

use crate::EcsSystem;
use crate::component::{ComponentDescription, ComponentId};

/// A type alias for a configuration of `std::hash::HashMap` that efficiently uses `ComponentTypeId`
/// as a key. This alias is special as it skips hashing the `ComponentTypeId` and uses that id
/// directly as the key.
pub type ComponentIdMap<T> = BHashMap<ComponentId, T, EcsSystem>;

/// A lazily initialized table of all types registered into the object system.
pub static COMPONENTS: LazyLock<BVec<&'static ComponentDescription, EcsSystem>> =
    LazyLock::new(|| {
        // We don't care if someone's sealed the list before, only that it has been sealed.
        let _ = __UNSAFE_COMPONENT_REGISTRY_HEAD.seal();

        // Pull all the type descriptions into a list
        let mut list = BVec::new_in(system());
        for t in __UNSAFE_COMPONENT_REGISTRY_HEAD.iter().copied() {
            list.push(LazyLock::force(t));
        }

        // Sort by the ID
        list.sort_by_key(|v| v.id);

        // Assert that there are no duplicates, and the list is fully dense.
        //
        // COMPONENTS[i] should now map to a type with ID i.
        for (i, t) in list.iter().enumerate() {
            assert_eq!(i, t.id.0 as usize);
        }

        list
    });

/// INTERNAL
///
/// DO NOT USE
#[doc(hidden)]
pub unsafe fn register_component_type(
    node: &'static ListItem<&'static LazyLock<ComponentDescription>>,
) {
    __UNSAFE_COMPONENT_REGISTRY_HEAD.push_entry(node);
}

/// This is the head of the object system registry. All objects that interact with the object system
/// will push an entry onto this list during the static init phase.
#[doc(hidden)]
static __UNSAFE_COMPONENT_REGISTRY_HEAD: InitList<&'static LazyLock<ComponentDescription>> =
    InitList::new();
