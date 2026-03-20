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

use std::ops::{Index, IndexMut};

use aleph_alloc::instrumentation::system;
use aleph_alloc::{BHashMap, BVec};

use crate::component::{ComponentDescription, ComponentId};
use crate::entity::EntityHandle;
use crate::world::component_index::alloc::ComponentIndexSystem;

/// This API maintains an append-only register of component types, indexable by the [`ComponentId`]
/// assigned when a type is registered.
///
/// This is basically just an append only `Vec`.
pub struct ComponentIndex {
    /// The list of all component types registered with some ECS world.
    components: BVec<ComponentEntry, ComponentIndexSystem>,
}

impl ComponentIndex {
    /// Construct a new, empty Self
    pub fn new() -> Self {
        Self {
            components: BVec::new_in(system()),
        }
    }

    /// Push a new type into the index. The type is described by the given [`ComponentDescription`].
    ///
    /// An ID is assigned to this type within this index and returned to the caller.
    ///
    /// # Panic
    ///
    /// This will panic if a component ID is generated that would overflow a `u32`. This would
    /// require pushing `u32::MAX` types into a single component index.
    ///
    /// # Auto-registration
    ///
    /// This type _does not_ integrate with component type auto-registration. That is handled by
    /// the ECS world. The IDs for auto-registered components is pre-assigned globally and the ECS
    /// world is implemented so that its internal component index matches the global table of
    /// component types. At least for __Rust__ component types.
    ///
    /// Dynamic component types are unaffected.
    pub fn push(&mut self, t: &ComponentDescription) -> ComponentId {
        let id = u32::try_from(self.components.len()).expect("Too many component types");
        let id = ComponentId(id);

        self.components.push(ComponentEntry {
            desc: t.clone(),
            archetypes: Default::default(),
            singleton: None,
        });

        id
    }

    /// Lookup a [`ComponentEntry`] using the given ID.
    #[inline]
    #[must_use]
    pub fn get(&self, index: ComponentId) -> Option<&ComponentEntry> {
        self.components.get(index.0 as usize)
    }

    /// Lookup a [`ComponentEntry`] using the given ID.
    #[inline]
    #[must_use]
    pub fn get_mut(&mut self, index: ComponentId) -> Option<&mut ComponentEntry> {
        self.components.get_mut(index.0 as usize)
    }
}

impl Index<ComponentId> for ComponentIndex {
    type Output = ComponentEntry;

    #[inline(always)]
    fn index(&self, index: ComponentId) -> &Self::Output {
        self.components.index(index.0 as usize)
    }
}

impl IndexMut<ComponentId> for ComponentIndex {
    #[inline(always)]
    fn index_mut(&mut self, index: ComponentId) -> &mut Self::Output {
        self.components.index_mut(index.0 as usize)
    }
}

/// An entry within a [`ComponentIndex`]. This stores both the description of the component type
/// as well as an additional per-component-type hash table that is used by the ECS world.
pub struct ComponentEntry {
    /// Description of the component type. Includes the physical size/alignment as well as other
    /// hooks and useful type metadata.
    pub desc: ComponentDescription,

    /// Encodes two things:
    /// - The keys form the set of all archetypes (indices) that some component type is a member of.
    /// - Maps the archetype index to the column index the component type is located in for that
    ///   archetype.
    pub archetypes: BHashMap<usize, ComponentArchetypeRecord, ComponentIndexSystem>,

    /// A handle to the current 'singleton' entity for the component type. This is backbone of the
    /// singleton API. A component singleton is a special entity that can be looked up for a
    /// component ID. It should always have a component of the associated id, and is used to store
    /// a single 'global' instance of a component.
    ///
    /// It is, otherwise, just a regular entity.
    pub singleton: Option<EntityHandle>,
}

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct ComponentArchetypeRecord {
    /// Index into an archetype's column list. Encodes which column in a given archetype a component
    /// can be found in.
    pub column: usize,
}

mod alloc {
    use crate::Ecs;

    pub struct ComponentIndex;
    aleph_alloc::new_child_alloc_category!(
        Ecs,
        ComponentIndex,
        "019c89bf-5ce3-78c1-ad5c-411442dd4091"
    );

    pub type ComponentIndexSystem = aleph_alloc::instrumentation::Instrumented<ComponentIndex>;
}
