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

pub mod column;

use aleph_alloc::instrumentation::system;
use aleph_alloc::{BBox, BVec};
use aleph_gen_arena::HandleType;

use crate::EcsSystem;
use crate::archetype::column::Column;
use crate::entity::EntityHandle;
use crate::type_layout::{TypeLayout, TypeLayoutBuf};
use crate::world::component_index::ComponentIndex;

/// Archetype implements a dynamically sized table data structure.
///
/// This struct implements a table, where each column of the table stores data of the same type.
/// Each column stores data, densely packed in order for each row, where given some row `i` the data
/// for that row can be found for any column `j` as `columns[j][i]`.
///
/// Each column stores an array of elements of some size, the sizes are determined when the
/// archetype is constructed. [`Column`] implements the storage for a single column.
///
/// An extra column of data is provided that associates each row in the table with the live handle
/// that references the row in the table. This enables `Archetype` to identify the entity a row
/// is storing.
///
/// This type implements the memory management of this table, but makes no attempt at handling the
/// objects stored _within_ the table. `Archetype` will handle growing and shrinking the memory
/// allocations, as well as allocating and freeing rows in the table, but `Archetype` will not
/// handle constructors, destructors, or anything similar. This type concerns itself only with
/// managing memory allocations.
///
/// The API user is expected to allocate memory in the table via the functions exposed here, and
/// is left to their own devices on how to store and manage the data within the table itself.
///
/// # ECS?
///
/// This implements the backing storage for an 'archetype' in our ECS world. The logical model of
/// an entity is that an entity is a unique object that can have a set of 'component' objects
/// attached to it. An entity is merely an identifier, given form as the sum of its components.
///
/// An archetype based ECS stores entities together with other entities of the same shape (i.e.
/// those with exactly the same set of components). There are methods to implement the logical model
/// of an ECS, but we use archetypes.
///
/// Each column contains the data for a particular _component type_. An entity is a reference to a
/// specific _row_ within an _archetype_. An entity is fully located by the ID of its archetype and
/// the row index within. This API manages the allocations, the [`crate::world::World`] is
/// responsible for implementing the ECS semantics.
pub struct Archetype {
    /// The layout of this archetype
    pub(crate) type_layout: BBox<TypeLayout, EcsSystem>,

    /// A list that maps an entity's index in the archetype storage back to the ID it was allocated
    /// with.
    ///
    /// Typically used by iterators that yield an `EntityID` alongside the components.
    pub(crate) entity_handles: BVec<EntityHandle, EcsSystem>,

    /// A list of all the storages of each component type in the entity layout, indexed by the
    /// storage index.
    pub(crate) columns: BVec<Column<EcsSystem>, EcsSystem>,

    /// The number of live entities currently stored in this archetype
    len: usize,

    /// The current number of entities we have space for in the component storage
    capacity: usize,
}

impl Archetype {
    /// Constructs a new, empty [`Archetype`] based on the given component registry and type layout.
    ///
    /// The columns are sorted in order of their component ID using the same rules as
    /// [`TypeLayout`].
    ///
    /// ## Component Index
    ///
    /// This is a lookup table that matches a [`crate::component::ComponentId`] to a
    /// [`crate::component::ComponentDescription`]. This is very likely to be the index maintained
    /// by [`crate::world::World`]. This index is queried to get the size/align of a component so
    /// that we can construct [`Column`] instances that manage elements with the appropriate memory
    /// layout.
    ///
    /// The component types come from...
    ///
    /// ## Type Layout
    ///
    /// The `layout` parameter is a `TypeLayout` that defines the shape of an archetype.
    /// [`TypeLayout`] is just a list of component ids with some extra layout constraints. Each
    /// component id in the given `layout` will get a column in the `Archetype`, and they will be
    /// stored in the same order as they are found in `layout`.
    pub fn new(component_index: &ComponentIndex, layout: &TypeLayout) -> Self {
        let components = TypeLayoutBuf::from_layout_in(layout, system());
        let components = components.into_boxed_slice();

        // Create the buffer for mapping entity indices back to an entity id
        let entity_handles = BVec::new_in(system());

        let mut columns = BVec::new_in(system());
        columns.extend(layout.iter().map(|&v| {
            let v = &component_index[v].desc;
            Column::new_in(v.type_layout().unwrap(), system())
        }));

        Self {
            type_layout: components,
            entity_handles,
            columns,
            len: 0,
            capacity: 0,
        }
    }

    /// Returns the number of live entities currently allocated in the archetype.
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Returns the number of entities the archetype has allocated storage to contain, regardless
    /// of the number of entities currently that are live.
    pub const fn capacity(&self) -> usize {
        self.capacity
    }

    /// Returns the [`TypeLayout`] of the archetype.
    #[inline(always)]
    pub fn type_layout(&self) -> &TypeLayout {
        &self.type_layout
    }

    /// Get a reference to the 'entity_handles' array. The slice will take [`Archetype::len`] as the
    /// size.
    #[inline(always)]
    pub fn entity_ids_ref(&self) -> &[EntityHandle] {
        &self.entity_handles[0..self.len]
    }

    /// Get a reference to the 'entity_handles' array. The slice will take [`Archetype::len`] as the
    /// size.
    #[inline(always)]
    pub fn entity_ids_mut(&mut self) -> &mut [EntityHandle] {
        &mut self.entity_handles[0..self.len]
    }

    /// Get a reference to the archetype's component storage columns. This array is the same size
    /// as [`Archetype::type_layout`], and associates such that a components column is found by
    /// `type_layout[i] -> columns[i]`.
    #[inline(always)]
    pub fn columns_ref(&self) -> &[Column<EcsSystem>] {
        &self.columns
    }

    /// Get a reference to the archetype's component storage columns. This array is the same size
    /// as [`Archetype::type_layout`], and associates such that a components column is found by
    /// `type_layout[i] -> columns[i]`.
    #[inline(always)]
    pub fn columns_mut(&mut self) -> &mut [Column<EcsSystem>] {
        &mut self.columns
    }

    /// Grow the archetype's storages to have capacity for at least 'count' additional entities and
    /// allocate them as 'live' by incrementing the archetype's len.
    ///
    /// Returns the index to the first new entity in the archetype. The archetype will be valid to
    /// access 'count' entities after the returned index for all related storages.
    ///
    /// # Note
    ///
    /// It is the caller's responsibility to update the following:
    /// - 'entity_ids' should be written for each new entity, as they will be zero initialized. The
    ///   archetype can't allocate entity IDs for itself.
    /// - Each entity must have all its components initialized in each column in the archetype. The
    ///   component data will be uninitialized until written by a caller.
    pub fn allocate_entities(&mut self, count: usize) -> usize {
        if count == 0 {
            // If you ask for 0 new entities then, strictly speaking, we _can_ give you the first
            // non-live index as perfectly valid behavior as a 0-sized array at that index _is_
            // valid.
            return self.len;
        }

        let out_idx = self.len;
        let new_size = self.len.checked_add(count).unwrap();

        // Expand the 'entity_ids' table to reserve enough space for the new number of live entities
        self.entity_handles
            .resize(new_size, EntityHandle::dangling());

        // Grow the capacity if we need to
        if new_size > self.capacity {
            // Get the minimum capacity we will use to store 'new_size' entities.
            let new_capacity = Self::minimum_capacity_for_entity_count(new_size);

            // Reserve enough memory in each column for 'new_capacity' entities.
            for column in self.columns.iter_mut() {
                column.grow_to_fit(new_capacity);
            }
            self.capacity = new_capacity;
        }

        // Bump the len now we've passed all the fallible conditions
        self.len = new_size;

        out_idx
    }

    /// Remove the entity at the given index from the archetype.
    ///
    /// This will perform a swap-n-pop operation, potentially moving the entity currently at the
    /// end of live region into the position of 'index' to replace the element being removed. This
    /// keeps entity storage dense.
    ///
    /// This will return an [`EntityHandle`] in the likely event that an entity was moved in order
    /// to keep the storage dense. You should use this to update the entity handle table.
    ///
    /// # Note
    ///
    /// It is the caller's responsibility to drop all components for the given entity _before_
    /// calling this function. Archetype implements storage, not the component lifecycle.
    pub fn remove_entity(&mut self, index: usize) -> Option<EntityHandle> {
        assert!(
            index < self.len,
            "Index '{index}' is out of bounds of archetype"
        );

        // swap-remove the ID from the dense ID array
        //
        // Checks if we're popping from the end of th array. If we have to remove from the interior
        // of the dense list then we will need to move the ID at the end into the empty space. The
        // entity storage in the World will need to be updated to respect the entity being moved
        // inside the archetype
        //
        // This returns the entity that needs to be updated and whether an update is needed

        // Do the initial swap_remove on the entity id table.
        self.entity_handles.swap_remove(index);

        // If we only need to pop off the end of the dense block of live entities then we should not
        // yield an ID to the caller. If we have to move an entity to keep the list dense then we
        // emit the ID so the caller can do any fixups needed to keep entity handles valid.
        let out_handle = if index == self.len - 1 {
            None
        } else {
            // Whatever is now in 'index' is what _was_ at the end of the list.
            Some(self.entity_handles[index])
        };

        // Now perform the swap-remove operation on each component column in the archetype.
        for column in self.columns.iter_mut() {
            column.swap_remove(index);
        }

        self.len -= 1;

        out_handle
    }

    /// Re-allocate all the different component columns/ID tables to the smallest capacity we can
    /// use to fit the current number of live entities.
    ///
    /// # Note
    ///
    /// Unlike [`Vec::shrink_to_fit`] this _does not_ shrink capacity to == len. We instead select
    /// the smallest capacity that will fit len according to
    /// [`Self::minimum_capacity_for_entity_count`].
    ///
    /// ## Why?
    ///
    /// The `shrink_to_fit` behavior on `Vec` is a sane default for a general purpose stretchy
    /// buffer. However, for an ECS archetype this is probably the wrong choice.
    ///
    /// Shrinking to the smallest size will _guarantee_ the next entity push operation will
    /// reallocate. In a game this is undesirable as we lose our amortized allocation costs when
    /// reclaiming memory.
    ///
    /// The more loose shrink behavior will spread allocation cost out over time in practice while
    /// still effectively reclaiming memory.
    ///
    pub fn shrink_to_fit(&mut self) {
        // Get the smallest allowed value for self.capacity that will fit self.len entities.
        let new_capacity = Self::minimum_capacity_for_entity_count(self.len);

        // If capacity is already correctly (minimally) sized then we don't need to do anything.
        if self.capacity <= new_capacity {
            return;
        }

        // These lengths should always match, but the capacities can get out of sync.
        assert_eq!(self.entity_handles.len(), self.len);

        // Once the lengths are known good we shrink the IDs array capacity too!
        self.entity_handles.shrink_to(new_capacity);

        // Then... shrink the allocations for all the columns too!
        for column in self.columns.iter_mut() {
            column.shrink_to_fit(new_capacity);
        }
    }

    /// Clear the archetype of all entities, returning length to 0.
    ///
    /// Does not free any memory, the capacity of all internal buffers remains unchanged.
    pub fn clear(&mut self) {
        self.len = 0;
        self.entity_handles.clear();
    }

    /// Find the minimum, padded value for 'capacity' that is valid for 'count' number of entities.
    ///
    /// This just rounds up to the next power of two after 'count' so we get amortized constant
    /// insertion complexity (`O(1)`).
    pub fn minimum_capacity_for_entity_count(count: usize) -> usize {
        if count == 0 {
            0
        } else if count.is_power_of_two() {
            count
        } else {
            let capacity = count.checked_next_power_of_two().unwrap();
            assert!(capacity >= count);
            capacity
        }
    }
}
