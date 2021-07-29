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

use std::borrow::Borrow;

use crate::{
    Component, ComponentIdMap, ComponentRegistry, ComponentTypeDescription, ComponentTypeId,
    EntityId, EntityLayout, EntityLayoutBuf,
};
use virtual_buffer::VirtualVec;

///
/// This index wrapper represents an index into the list of archetypes within a world.
///
/// This is used to better document the purpose of various indexes that would've otherwise been
/// plain `u32` fields.
///
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub struct ArchetypeIndex(pub u32);

///
/// This index wrapper represents an index into an archetype's component storage.
///
/// This is used to better document the purpose of various indexes that would've otherwise been
/// plain `u32` fields.
///
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub struct ArchetypeEntityIndex(pub u32);

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

    /// The maximum number of entities that can be stored in this archetype
    capacity: u32,

    /// The number of entities currently stored in this archetype
    len: u32,
}

impl Archetype {
    pub fn new(capacity: u32, layout: &EntityLayout, registry: &ComponentRegistry) -> Self {
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
                let buffer = VirtualVec::new(v.type_size * capacity as usize)
                    .expect("Failed to reserve address space for components");

                buffer
            })
            .collect();

        // Create the buffer for mapping entity indices back to an entity id
        let entity_ids = VirtualVec::new(capacity as usize)
            .expect("Failed to reserve address space for entity id list");

        Self {
            entity_layout: layout.to_owned(),
            storage_indices,
            component_descriptions,
            storages,
            entity_ids,
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
    pub fn component_storage<T: Component>(&self) -> Option<&[T]> {
        let ptr = self.component_storage_raw(T::get_type_description().type_id)?;

        let slice = unsafe {
            std::slice::from_raw_parts(ptr.as_ptr() as *mut T as *const T, self.len as usize)
        };

        Some(slice)
    }

    ///
    #[inline]
    pub fn component_storage_mut<T: Component>(&mut self) -> Option<&mut [T]> {
        let ptr = self.component_storage_mut_raw(T::get_type_description().type_id)?;

        let slice = unsafe {
            std::slice::from_raw_parts_mut(ptr.as_mut_ptr() as *mut T, self.len as usize)
        };

        Some(slice)
    }

    /// Given a component id, returns the raw bytes for the backing storage
    #[inline]
    pub fn component_storage_raw(&self, id: ComponentTypeId) -> Option<&[u8]> {
        // Map the component ID to the storage index
        let storage_index = self.storage_indices.get(&id).cloned()?;

        // Lookup the storage
        let storage = &self.storages[storage_index];

        Some(storage.as_slice())
    }

    /// Given a component id, returns the raw bytes for the backing storage
    #[inline]
    pub fn component_storage_mut_raw(&mut self, id: ComponentTypeId) -> Option<&mut [u8]> {
        // Map the component ID to the storage index
        let storage_index = self.storage_indices.get(&id).cloned()?;

        // Lookup the storage
        let storage = &mut self.storages[storage_index];

        Some(storage.as_slice_mut())
    }

    /// Given a storage index, returns the raw bytes for the backing storage
    #[inline]
    pub fn component_storage_raw_index(&self, index: usize) -> &[u8] {
        // Lookup the storage
        let storage = &self.storages[index];

        storage.as_slice()
    }

    /// Given a storage index, returns the raw bytes for the backing storage
    #[inline]
    pub fn component_storage_mut_raw_index(&mut self, index: usize) -> &mut [u8] {
        // Lookup the storage
        let storage = &mut self.storages[index];

        storage.as_slice_mut()
    }

    ///
    #[inline]
    pub fn entity_ids(&self) -> &[EntityId] {
        self.entity_ids.as_slice()
    }

    ///
    #[inline]
    pub fn entity_ids_mut(&mut self) -> &mut [EntityId] {
        self.entity_ids.as_slice_mut()
    }
}

/// Internal implementations
impl Archetype {
    /// This function allocates spaces for `count` entities. This does not handle writting the
    /// entity components into the storages, this must be done separately.
    ///
    /// The function returns the base index where the first newly allocated entity can be found.
    /// All new entities will be contiguous.
    #[inline]
    pub(crate) fn allocate_entities(&mut self, count: u32) -> u32 {
        let base = self.len;

        let new_size = self.len + count;
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

        self.len = new_size;

        base
    }

    #[inline]
    pub(crate) fn remove_entity(&mut self, index: ArchetypeEntityIndex) {
        self.entity_ids.swap_remove(index.0 as usize);
        for i in 0..self.storages.len() {
            self.swap_and_pop_for_storage(i, index);
        }
    }

    #[inline]
    pub(crate) fn swap_and_pop_for_storage(
        &mut self,
        storage_index: usize,
        index: ArchetypeEntityIndex,
    ) {
        let index = index.0 as usize;
        let last_index = (self.len - 1) as usize;
        if index == last_index {
            // Swap and pop at the end of the storage just decays to a regular pop operation.
            self.pop_for_storage(storage_index);
        } else {
            let storage = &mut self.storages[storage_index];
            let desc = &self.component_descriptions[storage_index];

            let remove_offset = index * desc.type_size;
            let last_offset = last_index * desc.type_size;

            let (remove, last) = storage.split_at_mut(last_offset);
            let remove = &mut remove[remove_offset..];

            remove.swap_with_slice(last);

            // Pop off the end, which destroys the element we wanted to remove
            self.pop_for_storage(storage_index);
        }
    }

    #[inline]
    pub(crate) fn pop_for_storage(&mut self, storage_index: usize) {
        if self.len != 0 {
            let storage = &mut self.storages[storage_index];
            let desc = &self.component_descriptions[storage_index];

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

            self.len -= 1;
        }
    }
}

impl Drop for Archetype {
    fn drop(&mut self) {
        todo!("Drop all live components in their type erased storages");
    }
}