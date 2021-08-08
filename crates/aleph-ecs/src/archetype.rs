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

use std::{borrow::Borrow, num::NonZeroU32};

use crate::{
    ComponentIdMap, ComponentRegistry, ComponentTypeDescription, ComponentTypeId, EntityId,
    EntityLayout, EntityLayoutBuf,
};
use virtual_buffer::VirtualVec;

///
/// This index wrapper represents an index into the list of archetypes within a world.
///
/// This is used to better document the purpose of various indexes that would've otherwise been
/// plain `u32` fields.
///
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ArchetypeIndex(pub NonZeroU32);

///
/// This index wrapper represents an index into an archetype's component storage.
///
/// This is used to better document the purpose of various indexes that would've otherwise been
/// plain `u32` fields.
///
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ArchetypeEntityIndex(pub NonZeroU32);

///
/// The structure that holds the links to other archetypes based on whether a specific component is
/// added or removed
///
#[repr(C)]
#[repr(align(8))]
#[derive(Clone, Copy, Hash, Debug, Default)]
pub struct ArchetypeEdge {
    /// Links to the archetype to move to if the specific component is added
    pub add: Option<ArchetypeIndex>,

    /// Links to the archetype to move to if the specific component is removed
    pub remove: Option<ArchetypeIndex>,
}

// TODO: State system based on linked list described here: https://ajmmertens.medium.com/why-storing-state-machines-in-ecs-is-a-bad-idea-742de7a18e59

pub struct Archetype {
    /// The entity layout of this archetype
    entity_layout: EntityLayoutBuf,

    /// A hash table that maps a component's id to the storage index. The storage index is used to
    /// index into the `component_descriptions` and `storages` fields.
    storage_indices: ComponentIdMap<usize>,

    /// A list of the description of each component type in the entity layout, indexed by the
    /// storage index
    component_descriptions: Vec<ComponentTypeDescription>,

    /// A list of all the storages of each component type in the entity layout, indexed by the
    /// storage index
    storages: Vec<VirtualVec<u8>>,

    /// A list that maps an entity's index in the archetype storage back to the ID it was allocated
    /// with.
    ///
    /// Typically used by iterators that yield an `EntityID` alongside the components.
    entity_ids: VirtualVec<EntityId>,

    /// Holds the edges of the archetype graph. Maps component ID to the links.
    edges: ComponentIdMap<ArchetypeEdge>,

    /// The maximum number of entities that can be stored in this archetype
    capacity: u32,

    /// The number of entities currently stored in this archetype
    len: u32,
}

impl Archetype {
    pub fn new(capacity: u32, layout: &EntityLayout, registry: &ComponentRegistry) -> Self {
        // Add 1 so there's space for the always empty 0th element
        let storage_capacity = capacity.checked_add(1).unwrap();

        // Produce the indice map from the layout
        let storage_indices: ComponentIdMap<usize> =
            layout.iter().enumerate().map(|v| (v.1, v.0)).collect();

        // Lookup the list of descriptions in the registry
        let component_descriptions: Vec<ComponentTypeDescription> = layout
            .iter()
            .map(|v| {
                registry
                    .lookup(v)
                    .expect("Tried to create an archetype with an unregistered component type")
            })
            .cloned()
            .collect();

        // Create a virtual memory reservation for each component's storage
        let storages = component_descriptions
            .iter()
            .map(|v| {
                let mut buffer = VirtualVec::new(v.type_size * storage_capacity as usize)
                    .expect("Failed to reserve address space for components");
                // Pre-fill the first slot with zeroes, it will never be accessed
                buffer.resize(v.type_size, 0);

                buffer
            })
            .collect();

        // Create the buffer for mapping entity indices back to an entity id
        let mut entity_ids = VirtualVec::new(storage_capacity as usize)
            .expect("Failed to reserve address space for entity id list");
        entity_ids.push(EntityId::null());

        // Create graph links as empty
        let edges = ComponentIdMap::with_hasher(Default::default());

        Self {
            entity_layout: layout.to_owned(),
            storage_indices,
            component_descriptions,
            storages,
            entity_ids,
            edges,
            capacity,
            len: 0,
        }
    }

    /// Returns the number of entities allocated in this archetype
    #[inline]
    pub fn len(&self) -> u32 {
        self.len
    }

    /// Returns the maximum number of entities that this archetype can hold
    #[inline]
    pub fn capacity(&self) -> u32 {
        self.capacity
    }

    /// Returns the layout for the set of components this archetype holds
    #[inline]
    pub fn entity_layout(&self) -> &EntityLayout {
        self.entity_layout.borrow()
    }
}

impl Archetype {
    ///
    #[inline]
    pub fn component_descriptions(&self) -> &[ComponentTypeDescription] {
        &self.component_descriptions
    }

    /// Given a component id, returns the raw bytes for the backing storage
    #[inline]
    pub fn component_storage_raw(&self, id: ComponentTypeId) -> Option<&[u8]> {
        // Map the component ID to the storage index
        let storage_index = self.storage_indices.get(&id).cloned()?;

        // Lookup the storage
        Some(self.storages[storage_index].as_slice())
    }

    /// Given a component id, returns the raw bytes for the backing storage
    #[inline]
    pub fn component_storage_mut_raw(&mut self, id: ComponentTypeId) -> Option<&mut [u8]> {
        // Map the component ID to the storage index
        let storage_index = self.storage_indices.get(&id).cloned()?;

        // Lookup the storage
        Some(self.storages[storage_index].as_slice_mut())
    }

    /// Given a storage index, returns the raw bytes for the backing storage
    #[inline]
    pub fn component_storage_raw_index(&self, index: usize) -> &[u8] {
        &self.storages[index]
    }

    /// Given a storage index, returns the raw bytes for the backing storage
    #[inline]
    pub fn component_storage_mut_raw_index(&mut self, index: usize) -> &mut [u8] {
        &mut self.storages[index]
    }

    ///
    #[inline]
    pub fn entity_ids(&self) -> &[EntityId] {
        &self.entity_ids
    }

    ///
    #[inline]
    pub fn entity_ids_mut(&mut self) -> &mut [EntityId] {
        &mut self.entity_ids
    }
}

/// Internal implementations
impl Archetype {
    #[inline]
    pub(crate) fn edges_mut(&mut self) -> &mut ComponentIdMap<ArchetypeEdge> {
        &mut self.edges
    }

    /// This function allocates spaces for `count` entities. This does not handle writting the
    /// entity components into the storages, this must be done separately.
    ///
    /// The function returns the base index where the first newly allocated entity can be found.
    /// All new entities will be contiguous.
    #[inline]
    pub(crate) fn allocate_entities(&mut self, count: u32) -> ArchetypeEntityIndex {
        // The base will be the index of the first slot after the end of the densely packed section
        //
        // We need to offset by 1 because the 1st slot in memory is skipped to allow using a 0 index
        // as a niche value.
        let base = NonZeroU32::new(self.len.checked_add(1).unwrap()).unwrap();

        let new_size = self.len.checked_add(count).unwrap();
        if new_size > self.capacity {
            panic!(
                "Adding {} entities would overflow capacity \"{}\"",
                count, self.capacity
            );
        }

        for (desc, storage) in self
            .component_descriptions
            .iter()
            .zip(self.storages.iter_mut())
        {
            let bytes = count as usize * desc.type_size;
            storage.resize(storage.len() + bytes, 0);
        }

        self.entity_ids
            .resize(count as usize + self.entity_ids.len(), EntityId::null());

        self.len = new_size;

        ArchetypeEntityIndex(base)
    }

    /// Remove the entity at the given index.
    ///
    /// The const parameter chooses whether to call the drop function or not
    #[inline]
    pub(crate) fn remove_entity<const DROP: bool>(&mut self, index: ArchetypeEntityIndex) {
        self.entity_ids.swap_remove(index.0.get() as usize);
        for i in 0..self.storages.len() {
            self.swap_and_pop_for_storage::<DROP>(i, index);
        }
        self.len -= 1;
    }

    /// Swap and pop the component in `storage_index` at `index`
    ///
    /// The const parameter chooses whether to call the drop function or not
    ///
    /// # Info
    ///
    /// DO NOT FORGET TO MANUALLY DECREMENT self.len
    #[inline]
    pub(crate) fn swap_and_pop_for_storage<const DROP: bool>(
        &mut self,
        storage_index: usize,
        index: ArchetypeEntityIndex,
    ) {
        let index = index.0.get() as usize;
        let last_index = (self.len - 1) as usize;
        if index == last_index {
            // Swap and pop at the end of the storage just decays to a regular pop operation.
            self.pop_for_storage::<DROP>(storage_index);
        } else {
            let storage = &mut self.storages[storage_index];
            let desc = &self.component_descriptions[storage_index];

            let remove_offset = index * desc.type_size;
            let last_offset = last_index * desc.type_size;

            let (remove, last) = storage.split_at_mut(last_offset);
            let remove = &mut remove[remove_offset..];

            remove.swap_with_slice(last);

            // Pop off the end, which destroys the element we wanted to remove
            self.pop_for_storage::<DROP>(storage_index);
        }
    }

    /// The const parameter chooses whether to call the drop function or not
    ///
    /// # Info
    ///
    /// DO NOT FORGET TO MANUALLY DECREMENT `self.len`
    #[inline]
    pub(crate) fn pop_for_storage<const DROP: bool>(&mut self, storage_index: usize) {
        if self.len != 0 {
            let storage = &mut self.storages[storage_index];
            let desc = &self.component_descriptions[storage_index];

            if DROP {
                if let Some(fn_drop) = desc.fn_drop {
                    let last_index = (self.len - 1) as usize;
                    let last_ptr = &mut storage[last_index * desc.type_size] as *mut u8;

                    // SAFETY: This handles calling the drop function for a component through a raw
                    //         pointer. The signature is type erased so the interface is unsafe.
                    //
                    //         This is just a type-erased call to `drop::<T>()` where T is the type of
                    //         the component. The `Archetype` data structure's safe interface ensures
                    //         the drop function is only called with valid data.
                    //
                    //         UB can be triggered if `fn_drop` is not implemented correctly, but this
                    //         is impossible from safe code as the implementation for each component is
                    //         auto generated from a generic implementation. The function can only be
                    //         incorrect by providing an incorrect ComponentTypeDescription using an
                    //         unsafe function.
                    unsafe {
                        fn_drop(last_ptr);
                    }
                }
            }
        }
    }

    #[inline]
    pub(crate) fn copy_from_archetype(
        &mut self,
        target: ArchetypeEntityIndex,
        source: &Archetype,
    ) -> ArchetypeEntityIndex {
        // Allocate a new slot in self to copy the component from the other archetype into
        let new_index = self.allocate_entities(1);

        // Copy the entity ID slot across
        self.entity_ids[new_index.0.get() as usize] = source.entity_ids()[target.0.get() as usize];

        for (source_index, source_id) in self.entity_layout.iter().enumerate() {
            // Get the size of the component to copy
            let type_size = self.component_descriptions[source_index].type_size;

            // Get the bounds of the data to copy
            let source_base = target.0.get() as usize;
            let source_base = source_base * type_size;
            let source_end = source_base + type_size;

            // Create a slice of the data to copy, exiting the loop if the component is not present
            // in the source archetype
            let source_buffer = if let Some(source_buffer) = source.component_storage_raw(source_id)
            {
                source_buffer
            } else {
                continue;
            };
            let source_buffer = &source_buffer[source_base..source_end];

            // Get the bounds of the memory to copy the data to
            let dest_base = new_index.0.get() as usize;
            let dest_base = dest_base * type_size;
            let dest_end = dest_base + type_size;

            // Create a slice of the destination to copy into
            let dest_buffer = self.storages[source_index].as_slice_mut();
            let dest_buffer = &mut dest_buffer[dest_base..dest_end];

            // Perform the actual copy
            dest_buffer.copy_from_slice(source_buffer);
        }

        new_index
    }
}

impl Drop for Archetype {
    fn drop(&mut self) {
        // Iterate over every component storage and call the drop function on all components
        for (index, storage) in self.storages.iter_mut().enumerate() {
            // Lookup the size and drop fn so we can iterate over the components in the storage
            let type_size = self.component_descriptions[index].type_size;
            let drop_fn = self.component_descriptions[index].fn_drop;

            // Only need to iterate if the drop function is actually defined
            if let Some(drop_fn) = drop_fn {
                // SAFETY: This just iterates over each item in the storage while type erased, which
                //         is a sound operation. The drop function will never be invalid to call if
                //         there is no unsafe code interfacing with the world.
                unsafe {
                    let mut current = storage.as_mut_ptr();
                    for _ in 0..self.len {
                        drop_fn(current);
                        current = current.add(type_size);
                    }
                }
            }
        }
    }
}
