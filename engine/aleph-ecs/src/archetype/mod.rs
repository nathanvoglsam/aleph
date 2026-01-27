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

use std::alloc::{Layout, handle_alloc_error};
use std::num::NonZeroU32;
use std::ptr::NonNull;

use aleph_alloc::alloc::{Allocator, Global};
use aleph_alloc::instrumentation::system;
use aleph_alloc::{BBox, BVec};
use aleph_object_system::ObjectDescription;
use aleph_object_system::uuid::Uuid;

pub use crate::archetype::index::{ArchetypeEntityIndex, ArchetypeIndex};
use crate::{
    ComponentRegistry, EcsSystem, EntityId, EntityLayout, EntityLayoutBuf, UnsafeComponentSource,
};

mod index {
    use std::num::NonZeroU32;

    use crate::World;

    ///
    /// This index wrapper represents an index into the list of archetypes within a world.
    ///
    /// This is used to better document the purpose of various indexes that would've otherwise been
    /// plain `u32` fields.
    ///
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
    pub struct ArchetypeIndex(NonZeroU32);

    impl ArchetypeIndex {
        /// Construct a new ArchetypeIndex from the given u32. This assumes that the given 'index'
        /// has already been offset by 1
        pub const fn new(index: NonZeroU32) -> ArchetypeIndex {
            assert!((index.get() as usize) <= World::MAX_ARCHETYPES);
            Self(index)
        }

        /// Returns the first valid ArchetypeIndex, which will be '1'
        pub const fn first() -> ArchetypeIndex {
            match NonZeroU32::new(1) {
                None => unreachable!(),
                Some(v) => Self(v),
            }
        }

        pub const fn inner(&self) -> NonZeroU32 {
            self.0
        }

        pub const fn get_index(&self) -> usize {
            self.0.get() as usize - 1
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
    pub struct ArchetypeEntityIndex(NonZeroU32);

    impl ArchetypeEntityIndex {
        /// Construct a new ArchetypeEntityIndex from the given raw u32
        pub const fn new(index: NonZeroU32) -> ArchetypeEntityIndex {
            Self(index)
        }

        /// Returns the first valid ArchetypeEntityIndex, which will be '1'
        pub const fn first() -> ArchetypeEntityIndex {
            match NonZeroU32::new(1) {
                None => unreachable!(),
                Some(v) => Self(v),
            }
        }

        pub const fn inner(&self) -> NonZeroU32 {
            self.0
        }

        pub const fn get_index(&self) -> usize {
            self.0.get() as usize - 1
        }
    }
}

#[repr(C)]
pub struct ComponentStorage {
    /// Pointer to the start of the virtual memory region that stores a given type of component
    pub data: AlignedByteBuffer<EcsSystem>,

    /// A description of the type being stored
    pub desc: ObjectDescription,
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
    entity_layout: BBox<EntityLayout, EcsSystem>,

    /// A table that maps a component's id to the storage index. The storage index is used to
    /// index into the `component_descriptions` and `storages` fields.
    storage_indices: ComponentMapper,

    /// A list of all the storages of each component type in the entity layout, indexed by the
    /// storage index
    storages: BVec<ComponentStorage, EcsSystem>,

    /// A list that maps an entity's index in the archetype storage back to the ID it was allocated
    /// with.
    ///
    /// Typically used by iterators that yield an `EntityID` alongside the components.
    entity_ids: BVec<EntityId, EcsSystem>,

    /// The maximum number of entities that can be stored in this archetype
    capacity: u32,

    /// The current number of entities we have space commited in the virtual allocation
    allocated: u32,

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
        assert_ne!(capacity, 0);
        assert_ne!(layout.len(), 0);

        // Produce the index map from the layout
        let mut table = BVec::with_capacity_in(layout.len(), system());
        table.extend(layout.iter().enumerate().map(|v| (*v.1, v.0)));
        let storage_indices = ComponentMapper { table };

        // Create a virtual memory reservation for each component's storage
        let mut storages = BVec::new_in(system());
        storages.extend(layout.iter().map(|v| {
            let desc = registry
                .lookup(v)
                .expect("Tried to create an archetype with an unregistered component type")
                .clone();
            ComponentStorage {
                data: AlignedByteBuffer::new_in(desc.align, system()),
                desc,
            }
        }));

        // Create the buffer for mapping entity indices back to an entity id
        let entity_ids = BVec::new_in(system());

        let entity_layout = EntityLayoutBuf::from_layout_in(layout, system());
        Self {
            entity_layout: entity_layout.into_boxed_slice(),
            storage_indices,
            storages,
            entity_ids,
            capacity,
            allocated: 0,
            len: 0,
        }
    }

    /// This function allocates spaces for `count` entities. This does not handle writing the
    /// entity components into the storages, this must be done separately.
    ///
    /// The function returns the base index where the first newly allocated entity can be found.
    /// All new entities will be contiguous.
    pub(crate) fn allocate_entities(&mut self, count: u32) -> ArchetypeEntityIndex {
        let base = NonZeroU32::new(self.len.checked_add(1).unwrap()).unwrap();

        let new_size = self.len.checked_add(count).unwrap();
        if new_size > self.capacity {
            panic!(
                "Adding {} entities would overflow capacity \"{}\"",
                count, self.capacity
            );
        }

        if new_size > self.allocated {
            // Minimum number of entities to have space for is 64 and max is capacity
            let mut new_allocated = self.allocated.checked_mul(2).unwrap().max(1);
            while new_allocated < new_size {
                new_allocated = new_allocated.checked_mul(2).unwrap();
            }
            let new_allocated = u32::clamp(new_allocated, 64, self.capacity);
            for storage in self.storages.iter_mut() {
                storage
                    .data
                    .grow(new_allocated as usize * storage.desc.size);
            }
            self.allocated = new_allocated;
        }

        self.entity_ids.resize(new_size as usize, EntityId::null());

        self.len = new_size;

        ArchetypeEntityIndex::new(base)
    }

    /// This function handles writing data from a generic component source into the correct buffers
    /// for each component type. It starts the copy from the given `base` entity slot.
    ///
    /// # Warning
    ///
    /// This will not perform any checks to ensure existing data isn't overwritten, this is
    /// effectively a wrapper around `memcpy`. It will, however, ensure that data is not written or
    /// read out of bounds.
    pub(crate) unsafe fn unsafe_copy_from_source(
        &mut self,
        base: ArchetypeEntityIndex,
        source: &UnsafeComponentSource,
    ) {
        unsafe {
            // Copy the component data into the archetype buffers
            for (i, comp) in self.entity_layout.iter().enumerate() {
                // Safety: Caller's job to ensure components is valid to read
                let components = source.components.as_ref();

                // Safety: Caller's job to ensure that all IDs in 'layout' are also in the components
                //         list.
                let data = components.iter().find(|v| v.id == *comp).unwrap_unchecked();

                // Get the size of the type we're copying from the buffers
                let type_size = self.storages[i].desc.size;

                // Calculate the base index for where to start copying into the buffer
                let base = base.get_index();
                let base = base * type_size;

                // Get the target slice to copy into
                let target = self.storages[i].data.get().unwrap_unchecked().add(base);

                // Perform the actual copy
                target.copy_from_nonoverlapping(data.ptr, source.count as usize * type_size);
            }
        }
    }

    /// This function will write the provided `data` into the storage for the given `component_type`
    /// at the given `slot` within the storage.
    pub(crate) unsafe fn copy_component_data_into_slot(
        &mut self,
        slot: ArchetypeEntityIndex,
        component_type: &Uuid,
        data: NonNull<u8>,
    ) {
        // Get the index of the type inside the archetype and lookup the size of the type
        let type_index = self.storage_indices.get(component_type).unwrap();
        let type_size = self.storages[type_index].desc.size;

        // Get the bounds of the component's data
        let dest_base = slot.get_index();
        let dest_base = dest_base * type_size;

        unsafe {
            // Create the slice to copy into, no dropping is needed as the data is uninitialized
            let dest_buffer = self.storages[type_index]
                .data
                .get()
                .unwrap_unchecked()
                .add(dest_base);

            // Perform the actual copy
            dest_buffer.copy_from_nonoverlapping(data, type_size);
        }
    }

    #[inline]
    pub(crate) unsafe fn drop_component_in_slot(
        &mut self,
        slot: ArchetypeEntityIndex,
        component_type: &Uuid,
    ) {
        let type_index = self.storage_indices.get(component_type).unwrap();
        let type_size = self.storages[type_index].desc.size;
        let drop_fn = self.storages[type_index].desc.destructor;

        if let Some(drop_fn) = drop_fn {
            let slot = slot.get_index();
            unsafe {
                let component = self.storages[type_index]
                    .data
                    .get()
                    .unwrap_unchecked()
                    .add(slot * type_size);
                drop_fn(component.cast(), 1);
            };
        }
    }

    pub(crate) unsafe fn get_component_ptr(
        &self,
        slot: ArchetypeEntityIndex,
        component_type: &Uuid,
    ) -> Option<NonNull<u8>> {
        let slot = slot.get_index();

        // Lookup the storage index, load the size of the type and get the storage pointer
        let storage_index = self.storage_indices.get(component_type)?;
        let storage = &self.storages[storage_index];

        let type_size = storage.desc.size;
        unsafe {
            let data = storage.data.get().unwrap_unchecked().add(slot * type_size);

            // Get a pointer to the position in the buffer the component can be found
            Some(data)
        }
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
    pub(crate) unsafe fn remove_entity<const DROP: bool>(
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
        self.entity_ids.swap_remove(index.get_index());
        let out_index = if index.get_index() == self.entity_ids.len() {
            None
        } else {
            Some(self.entity_ids[index.get_index()])
        };

        for i in 0..self.storages.len() {
            unsafe {
                self.swap_and_pop_for_storage::<DROP>(i, index);
            };
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
    pub(crate) unsafe fn swap_and_pop_for_storage<const DROP: bool>(
        &mut self,
        storage_index: usize,
        index: ArchetypeEntityIndex,
    ) {
        if self.len == 0 {
            return;
        }

        let index = index.get_index();
        let last_index = (self.len - 1) as usize;
        if index == last_index {
            // Swap and pop at the end of the storage just decays to a regular pop operation.
            self.pop_for_storage::<DROP>(storage_index);
        } else {
            let storage = &mut self.storages[storage_index];
            let desc = &storage.desc;

            let remove_offset = index * desc.size;
            let last_offset = last_index * desc.size;

            unsafe {
                let base = storage.data.get().unwrap_unchecked();
                let remove = base.add(remove_offset);
                let last = base.add(last_offset);

                if DROP && let Some(fn_drop) = desc.destructor {
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
                    fn_drop(remove.cast(), 1);
                }

                remove.copy_from_nonoverlapping(last, desc.size);
            };
        }
    }

    /// The const parameter chooses whether to call the drop function or not
    ///
    /// # Info
    ///
    /// DO NOT FORGET TO MANUALLY DECREMENT `self.len`
    pub(crate) fn pop_for_storage<const DROP: bool>(&mut self, storage_index: usize) {
        if self.len == 0 {
            return;
        }

        let storage = &mut self.storages[storage_index];
        let desc = &storage.desc;

        if DROP && let Some(fn_drop) = desc.destructor {
            let last_index = (self.len - 1) as usize;

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
                let last_ptr = storage
                    .data
                    .get()
                    .unwrap_unchecked()
                    .add(last_index * desc.size);
                fn_drop(last_ptr.cast(), 1);
            }
        }
    }

    pub(crate) unsafe fn copy_from_archetype(
        &mut self,
        target: ArchetypeEntityIndex,
        source: &Archetype,
    ) -> ArchetypeEntityIndex {
        // Allocate a new slot in self to copy the component from the other archetype into
        let new_index = self.allocate_entities(1);

        // Copy the entity ID slot across
        self.entity_ids[new_index.get_index()] = source.entity_ids[target.get_index()];

        for (source_index, source_id) in self.entity_layout.iter().enumerate() {
            // Get the size of the component to copy
            let type_size = self.storages[source_index].desc.size;

            // Get the bounds of the data to copy
            let source_base = target.get_index();
            let source_base = source_base * type_size;

            // Create a slice of the data to copy, exiting the loop if the component is not present
            // in the source archetype
            let source_buffer = if let Some(source_index) = source.storage_indices.get(source_id) {
                unsafe {
                    source.storages[source_index]
                        .data
                        .get()
                        .unwrap_unchecked()
                        .add(source_base)
                }
            } else {
                continue;
            };

            // Get the bounds of the memory to copy the data to
            let dest_base = new_index.get_index();
            let dest_base = dest_base * type_size;

            unsafe {
                // Create a slice of the destination to copy into
                let dest_buffer = self.storages[source_index]
                    .data
                    .get()
                    .unwrap_unchecked()
                    .add(dest_base);

                // Perform the actual copy
                dest_buffer.copy_from_nonoverlapping(source_buffer, type_size);
            }
        }

        new_index
    }

    /// Writes the entity ID into the ID list at the given slot.
    ///
    /// Used for initializing the ID when entities are inserted.
    pub(crate) fn update_entity_id(&mut self, slot: ArchetypeEntityIndex, id: EntityId) {
        self.entity_ids[slot.get_index()] = id;
    }

    /// Returns the start and end address for the entity id list so it can be used by query
    /// iterators
    pub(crate) fn entity_id_ptr_range(&self) -> (NonNull<EntityId>, NonNull<EntityId>) {
        unsafe {
            let ptr = self.entity_ids.as_ptr() as *mut EntityId;
            let ptr_end = ptr.add(self.len() as usize);
            let ptr = NonNull::new_unchecked(ptr);
            let ptr_end = NonNull::new_unchecked(ptr_end);
            (ptr, ptr_end)
        }
    }
}

impl Drop for Archetype {
    fn drop(&mut self) {
        // Early exit if we don't contain any entities in this archetype
        if self.len == 0 {
            return;
        }

        // Iterate over every component storage and call the drop function on all components
        for storage in self.storages.iter_mut() {
            // Lookup the size and drop fn so we can iterate over the components in the storage
            let desc = &storage.desc;

            // Only need to iterate if the drop function is actually defined
            if let Some(drop_fn) = desc.destructor {
                // SAFETY: This just iterates over each item in the storage while type erased, which
                //         is a sound operation. The drop function will never be invalid to call if
                //         there is no unsafe code interfacing with the world.
                unsafe {
                    if let Some(ptr) = storage.data.get() {
                        drop_fn(ptr.cast(), self.len as _);
                    }
                }
            }
        }
    }
}

#[repr(C)]
struct ComponentMapper {
    table: BVec<(Uuid, usize), EcsSystem>,
}

impl ComponentMapper {
    fn get(&self, component_id: &Uuid) -> Option<usize> {
        for v in self.table.iter() {
            if v.0 == *component_id {
                return Some(v.1);
            }
        }
        None
    }
}

pub struct AlignedByteBuffer<A: Allocator = Global> {
    buffer: NonNull<u8>,
    align: usize,
    len: usize,
    allocator: A,
}

impl<A: Allocator> AlignedByteBuffer<A> {
    pub const fn new_in(align: usize, allocator: A) -> Self {
        Self {
            buffer: NonNull::dangling(),
            align,
            len: 0,
            allocator,
        }
    }

    pub const fn get(&self) -> Option<NonNull<u8>> {
        if self.len > 0 {
            Some(self.buffer)
        } else {
            None
        }
    }

    pub fn grow(&mut self, new_size: usize) {
        assert!(new_size > self.len);

        let new_layout = Layout::from_size_align(new_size, self.align).unwrap();
        if self.len == 0 {
            let buffer = match self.allocator.allocate(new_layout) {
                Ok(v) => v,
                Err(_) => handle_alloc_error(new_layout),
            };

            self.len = new_size;
            self.buffer = buffer.cast();
        } else {
            let old_layout = self.layout();
            let buffer = unsafe {
                match self.allocator.grow(self.buffer, old_layout, new_layout) {
                    Ok(v) => v,
                    Err(_) => handle_alloc_error(new_layout),
                }
            };

            self.len = new_size;
            self.buffer = buffer.cast();
        }
    }

    fn layout(&self) -> Layout {
        Layout::from_size_align(self.len, self.align).unwrap()
    }
}

impl<A: Allocator> Drop for AlignedByteBuffer<A> {
    fn drop(&mut self) {
        unsafe { self.allocator.deallocate(self.buffer, self.layout()) }
    }
}

unsafe impl<A: Allocator> Send for AlignedByteBuffer<A> {}
unsafe impl<A: Allocator> Sync for AlignedByteBuffer<A> {}
