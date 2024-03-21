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

use std::num::NonZeroU32;
use std::ptr::NonNull;

use virtual_buffer::VirtualVec;

use crate::atomic_borrow::AtomicBorrow;
use crate::{
    ComponentIdMap, ComponentRegistry, ComponentSource, ComponentTypeDescription, ComponentTypeId,
    EntityId, EntityLayout, EntityLayoutBuf,
};

///
/// This index wrapper represents an index into the list of archetypes within a world.
///
/// This is used to better document the purpose of various indexes that would've otherwise been
/// plain `u32` fields.
///
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ArchetypeIndex(pub NonZeroU32);

impl ArchetypeIndex {
    /// Construct a new ArchetypeIndex from the given raw u32
    pub const fn new(index: u32) -> Option<ArchetypeIndex> {
        if let Some(v) = NonZeroU32::new(index) {
            Some(ArchetypeIndex(v))
        } else {
            None
        }
    }

    /// Returns the first valid ArchetypeIndex, which will be '1'
    pub const fn first() -> ArchetypeIndex {
        // Safety: uuuh, seems pretty obvious
        unsafe { Self(NonZeroU32::new_unchecked(1)) }
    }
}

///
/// This index wrapper represents an index into an archetype's component storage.
///
/// This is used to better document the purpose of various indexes that would've otherwise been
/// plain `u32` fields.
///
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ArchetypeEntityIndex(pub NonZeroU32);

pub struct ComponentStorage {
    /// The buffer that stores our components
    pub data: VirtualVec<u8>,

    /// A guard object that enables dynamic borrow checking of our component storages
    pub guard: AtomicBorrow,

    /// A description of the type being stored
    pub type_description: ComponentTypeDescription,
}

impl ComponentStorage {
    pub fn with_capacity(type_description: ComponentTypeDescription, capacity: usize) -> Self {
        let capacity = type_description.type_size * capacity;
        let mut data =
            VirtualVec::new(capacity).expect("Failed to reserve address space for components");

        // Pre-fill the first slot with zeroes, it will never be accessed
        data.resize(type_description.type_size, 0);

        Self {
            data,
            guard: AtomicBorrow::new(),
            type_description,
        }
    }
}

// TODO: State system based on linked list described here: https://ajmmertens.medium.com/why-storing-state-machines-in-ecs-is-a-bad-idea-742de7a18e59

///
/// The data structure that stores the components for a given entity. An archetype provides SoA
/// storage for all entities of the same "shape" (have the same components).
///
/// # Implementation Details
///
/// The storage implementation is very straight forward. An `Archetype` consists of an array of
/// buffers and a map that maps the `ComponentId` to the buffer that stores data for the matching
/// component type. There is also another buffer that stores the `EntityId` that the entity in the
/// given slot was allocated to so you can map from entity slot back to the ID (needed for
/// implementing component/entity removal and providing the ID of the entity in the slot when
/// iterating).
///
/// All buffers are stored type-erased as raw `u8` buffers. Size and alignment is checked on
/// construction to prevent unaligned access.
///
/// The components are stored using virtual memory allocations. When constructing an archetype a
/// maximum capacity is provided as the maximum number of entities that can be stored. This capacity
/// is then used to reserve enough address space to store the maximum number of entities and their
/// components. Virtual memory is only committed when the space is needed to store entities meaning
/// memory is only consumed for real entities and not the entire reserved range.
///
/// Using virtual memory like this has two important benefits:
///
/// Unlike `Vec<u8>` we do not copy the data when "growing" the buffer. A `Vec<u8>` will have to
/// allocate a new region of memory and copy the old data across to grow the buffer. By using a
/// virtual memory allocation the data never moves when growing, we simply commit more of the
/// address space to grow the buffer. The data never needs to be copied because it never needs to
/// move.
///
/// This also has the benefit of making pointers into the component storage stable. The backing
/// buffers will never move. This allows for holding onto pointers into the component buffers, and
/// they will never be invalidated by the buffer growing. This is still wildly unsafe but can be
/// a useful assumption for optimization purposes.
///
pub struct Archetype {
    /// The entity layout of this archetype
    entity_layout: EntityLayoutBuf,

    /// A hash table that maps a component's id to the storage index. The storage index is used to
    /// index into the `component_descriptions` and `storages` fields.
    storage_indices: ComponentIdMap<usize>,

    /// A list of all the storages of each component type in the entity layout, indexed by the
    /// storage index
    storages: Vec<ComponentStorage>,

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
    /// Returns a reference to the entity layout this archetype stores entities for
    #[inline(always)]
    pub fn entity_layout(&self) -> &EntityLayout {
        &self.entity_layout
    }

    /// Returns the maximum number of entities that can be stored in this archetype
    pub const fn capacity(&self) -> u32 {
        self.capacity
    }

    /// Returns the current number of entities that can be stored in this archetype
    pub const fn len(&self) -> u32 {
        self.len
    }

    /// Returns if there are no entities in the archetype
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }
}

/// Internal implementations
impl Archetype {
    pub(crate) fn new(capacity: u32, layout: &EntityLayout, registry: &ComponentRegistry) -> Self {
        // Add 1 so there's space for the always empty 0th element
        let storage_capacity = capacity.checked_add(1).unwrap();

        // Produce the index map from the layout
        let storage_indices: ComponentIdMap<usize> =
            layout.iter().enumerate().map(|v| (v.1, v.0)).collect();

        // Create a virtual memory reservation for each component's storage
        let storages = layout
            .iter()
            .map(|v| {
                let type_description = registry
                    .lookup(v)
                    .expect("Tried to create an archetype with an unregistered component type")
                    .clone();
                ComponentStorage::with_capacity(type_description, storage_capacity as _)
            })
            .collect();

        // Create the buffer for mapping entity indices back to an entity id
        let mut entity_ids = VirtualVec::new(storage_capacity as usize)
            .expect("Failed to reserve address space for entity id list");
        entity_ids.push(EntityId::null());

        Self {
            entity_layout: layout.to_owned(),
            storage_indices,
            storages,
            entity_ids,
            capacity,
            len: 0,
        }
    }

    /// This function allocates spaces for `count` entities. This does not handle writing the
    /// entity components into the storages, this must be done separately.
    ///
    /// The function returns the base index where the first newly allocated entity can be found.
    /// All new entities will be contiguous.
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

        for storage in self.storages.iter_mut() {
            let bytes = count as usize * storage.type_description.type_size;
            storage.data.resize(storage.data.len() + bytes, 0);
        }

        self.entity_ids
            .resize(count as usize + self.entity_ids.len(), EntityId::null());

        self.len = new_size;

        ArchetypeEntityIndex(base)
    }

    /// This function handles writing data from a generic component source into the correct buffers
    /// for each component type. It starts the copy from the given `base` entity slot.
    ///
    /// # Warning
    ///
    /// This will not perform any checks to ensure existing data isn't overwritten, this is
    /// effectively a wrapper around `memcpy`. It will, however, ensure that data is not written or
    /// read out of bounds.
    pub(crate) fn copy_from_source<T: ComponentSource>(
        &mut self,
        base: ArchetypeEntityIndex,
        source: T,
    ) {
        // Copy the component data into the archetype buffers
        for (i, comp) in source.entity_layout().iter().enumerate() {
            let source = source.data_for(comp);

            // Get the size of the type we're copying from the buffers
            let type_size = self.storages[i].type_description.type_size;

            // Calculate the base index for where to start copying into the buffer
            let base = base.0.get() as usize;
            let base = base * type_size;

            // Calculate the end of the region to copy into
            let end = base + source.len();

            // Get the target slice to copy into
            let target = self.storages[i].data.as_slice_mut();
            let target = &mut target[base..end];

            // Perform the actual copy
            target.copy_from_slice(source);
        }
    }

    /// This function will write the provided `data` into the storage for the given `component_type`
    /// at the given `slot` within the storage.
    pub(crate) fn copy_component_data_into_slot(
        &mut self,
        slot: ArchetypeEntityIndex,
        component_type: ComponentTypeId,
        data: &[u8],
    ) {
        // Get the index of the type inside the archetype and lookup the size of the type
        let type_index = self.storage_indices.get(&component_type).copied().unwrap();
        let type_size = self.storages[type_index].type_description.type_size;

        // Get the bounds of the component's data
        let dest_base = slot.0.get() as usize;
        let dest_base = dest_base * type_size;
        let dest_end = dest_base + type_size;

        // Create the slice to copy into, no dropping is needed as the data is uninitialized
        let dest_buffer = self.storages[type_index].data.as_slice_mut();
        let dest_buffer = &mut dest_buffer[dest_base..dest_end];

        // Perform the actual copy
        dest_buffer.copy_from_slice(data);
    }

    #[inline]
    pub(crate) unsafe fn drop_component_in_slot(
        &mut self,
        slot: ArchetypeEntityIndex,
        component_type: ComponentTypeId,
    ) {
        let type_index = self.storage_indices.get(&component_type).copied().unwrap();
        let type_size = self.storages[type_index].type_description.type_size;
        let drop_fn = self.storages[type_index].type_description.fn_drop;

        if let Some(drop_fn) = drop_fn {
            let base = slot.0.get() as usize;
            let base = base * type_size;
            let end = base + type_size;

            let slice = self.storages[type_index].data.as_slice_mut();
            let slice = &mut slice[base..end];

            drop_fn(slice.as_mut_ptr());
        }
    }

    pub(crate) fn get_component_guard(
        &self,
        component_type: ComponentTypeId,
    ) -> Option<&AtomicBorrow> {
        // Lookup the storage index for the borrow guard
        let storage_index = self.storage_indices.get(&component_type).copied()?;
        Some(&self.storages[storage_index].guard)
    }

    pub(crate) fn get_component_ptr(
        &self,
        slot: ArchetypeEntityIndex,
        component_type: ComponentTypeId,
    ) -> Option<NonNull<u8>> {
        // Lookup the storage index, load the size of the type and get the storage pointer
        let storage_index = self.storage_indices.get(&component_type).copied()?;
        let type_size = self.storages[storage_index].type_description.type_size;
        let storage = self.storages[storage_index].data.as_slice();

        // Get the bounds of the component's data
        let base = slot.0.get() as usize;
        let base = base * type_size;
        let end = base + type_size;

        // Get a pointer to the position in the buffer the component can be found
        let slice = &storage[base..end];
        let ptr = slice.as_ptr();

        NonNull::new(ptr as *mut u8)
    }

    /// Remove the entity at the given index.
    ///
    /// The const parameter chooses whether to call the drop function or not.
    ///
    /// Will return an optional `EntityId`. If an ID is yielded it means we had to move an entity
    /// within the archetype to perform the removal and keep the entities packed.
    ///
    /// If this function returns a value then the user (i.e. [`World`]) must update the
    /// `EntityLocation` field for that ID to prevent the ID from becoming a dangling reference
    /// (unsafe).
    pub(crate) fn remove_entity<const DROP: bool>(
        &mut self,
        index: ArchetypeEntityIndex,
    ) -> Option<EntityId> {
        // swap-remove the ID from the dense ID array
        //
        // Checks if we're popping from the end of th array. If we have to remove from the interior
        // of the dense list then we will need to move the ID at the end into the empty space. The
        // entity storage in the World will need to be updated to respect the entity being moved
        // inside the archetype
        //
        // This returns the entity that needs to be updated and whether an update is needed
        self.entity_ids.swap_remove(index.0.get() as usize);
        let out_index = if index.0.get() as usize == self.entity_ids.len() {
            None
        } else {
            Some(self.entity_ids[index.0.get() as usize])
        };

        for i in 0..self.storages.len() {
            self.swap_and_pop_for_storage::<DROP>(i, index);
        }

        self.len -= 1;

        out_index
    }

    /// Swap and pop the component in `storage_index` at `index`
    ///
    /// The const parameter chooses whether to call the drop function or not
    ///
    /// # Info
    ///
    /// DO NOT FORGET TO MANUALLY DECREMENT self.len
    pub(crate) fn swap_and_pop_for_storage<const DROP: bool>(
        &mut self,
        storage_index: usize,
        index: ArchetypeEntityIndex,
    ) {
        let index = index.0.get() as usize;
        let last_index = self.len as usize;
        if index == last_index {
            // Swap and pop at the end of the storage just decays to a regular pop operation.
            self.pop_for_storage::<DROP>(storage_index);
        } else {
            let storage = &mut self.storages[storage_index];
            let desc = &storage.type_description;

            let remove_offset = index * desc.type_size;
            let last_offset = last_index * desc.type_size;

            let (remove, last) = storage.data.split_at_mut(last_offset);
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
    pub(crate) fn pop_for_storage<const DROP: bool>(&mut self, storage_index: usize) {
        if self.len != 0 {
            let storage = &mut self.storages[storage_index];
            let desc = &storage.type_description;

            if DROP {
                if let Some(fn_drop) = desc.fn_drop {
                    let last_index = (self.len - 1) as usize;
                    let last_ptr = &mut storage.data[last_index * desc.type_size];

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

    pub(crate) fn copy_from_archetype(
        &mut self,
        target: ArchetypeEntityIndex,
        source: &Archetype,
    ) -> ArchetypeEntityIndex {
        // Allocate a new slot in self to copy the component from the other archetype into
        let new_index = self.allocate_entities(1);

        // Copy the entity ID slot across
        self.entity_ids[new_index.0.get() as usize] = source.entity_ids[target.0.get() as usize];

        for (source_index, source_id) in self.entity_layout.iter().enumerate() {
            // Get the size of the component to copy
            let type_size = self.storages[source_index].type_description.type_size;

            // Get the bounds of the data to copy
            let source_base = target.0.get() as usize;
            let source_base = source_base * type_size;
            let source_end = source_base + type_size;

            // Create a slice of the data to copy, exiting the loop if the component is not present
            // in the source archetype
            let source_buffer =
                if let Some(source_index) = source.storage_indices.get(&source_id).copied() {
                    source.storages[source_index].data.as_slice()
                } else {
                    continue;
                };
            let source_buffer = &source_buffer[source_base..source_end];

            // Get the bounds of the memory to copy the data to
            let dest_base = new_index.0.get() as usize;
            let dest_base = dest_base * type_size;
            let dest_end = dest_base + type_size;

            // Create a slice of the destination to copy into
            let dest_buffer = self.storages[source_index].data.as_slice_mut();
            let dest_buffer = &mut dest_buffer[dest_base..dest_end];

            // Perform the actual copy
            dest_buffer.copy_from_slice(source_buffer);
        }

        new_index
    }

    /// Writes the entity ID into the ID list at the given slot.
    ///
    /// Used for initializing the ID when entities are inserted.
    pub(crate) fn update_entity_id(&mut self, slot: ArchetypeEntityIndex, id: EntityId) {
        self.entity_ids[slot.0.get() as usize] = id;
    }

    /// Returns the start and end address for the entity id list so it can be used by query
    /// iterators
    pub(crate) fn entity_id_ptr_range(&self) -> (NonNull<EntityId>, NonNull<EntityId>) {
        unsafe {
            let ptr = self.entity_ids.as_ptr() as *mut EntityId;
            let ptr = ptr.add(1);
            let ptr_end = ptr.add(self.len() as usize);
            let ptr = NonNull::new_unchecked(ptr);
            let ptr_end = NonNull::new_unchecked(ptr_end);
            (ptr, ptr_end)
        }
    }
}

impl Drop for Archetype {
    fn drop(&mut self) {
        // Iterate over every component storage and call the drop function on all components
        for storage in self.storages.iter_mut() {
            // Lookup the size and drop fn so we can iterate over the components in the storage
            let desc = &storage.type_description;
            let type_size = desc.type_size;
            let drop_fn = desc.fn_drop;

            // Only need to iterate if the drop function is actually defined
            if let Some(drop_fn) = drop_fn {
                // SAFETY: This just iterates over each item in the storage while type erased, which
                //         is a sound operation. The drop function will never be invalid to call if
                //         there is no unsafe code interfacing with the world.
                unsafe {
                    let mut current = storage.data.as_mut_ptr().add(type_size);
                    for _ in 0..self.len {
                        drop_fn(current);
                        current = current.add(type_size);
                    }
                }
            }
        }
    }
}
