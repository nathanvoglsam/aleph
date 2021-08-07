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

use std::{collections::HashMap, num::NonZeroU32};

use crate::{
    Archetype, ArchetypeEntityIndex, ArchetypeIndex, Component, ComponentRegistry,
    ComponentTypeDescription, EntityId, EntityLayout, EntityLayoutBuf, EntityLocation,
    EntityStorage,
};

/// A module that groups all the operation descriptions
pub mod operations {
    use crate::{ComponentTypeId, EntityId, EntityLayout};

    ///
    /// The raw, FFI friendly struct that describes an entity insertion operation.
    ///
    #[repr(C)]
    pub struct EntityInsertionDescription<'a, 'b> {
        /// The layout of the entities to create. (Sorted de-duplicated list of component type ids)
        pub entity_layout: &'a EntityLayout,

        /// An array of pointers to buffers that contain `ids.len()` components for each component
        /// type.
        ///
        /// Each pointer must point to a buffer that holds `ids.len()` items for the given
        /// component.
        pub component_buffers: &'a [&'a [u8]],

        /// The buffer to write all the entity ids into
        pub ids: &'b mut [EntityId],
    }

    ///
    /// The raw, FFI friendly struct that describes an entity removal operation.
    ///
    #[repr(C)]
    #[derive(Clone)]
    pub struct EntityRemovalDescription {
        /// The ID of the entity we want to operate on
        pub id: EntityId,
    }

    ///
    /// The raw, FFI friendly struct that describes an entity component removal operation.
    ///
    #[repr(C)]
    #[derive(Clone)]
    pub struct ComponentRemovalDescription {
        /// The ID of the entity we want to operate on
        pub id: EntityId,

        /// The type ID of the component to remove
        pub component: ComponentTypeId,
    }

    ///
    /// The raw, FFI friendly struct that describes an entity component insertion operation.
    ///
    #[repr(C)]
    #[derive(Clone)]
    pub struct ComponentInsertionDescription<'a> {
        /// The ID of the entity we want to operate on
        pub id: EntityId,

        /// The type ID of the component to add
        pub component: ComponentTypeId,

        /// The data to copy the component data from
        pub data: &'a [u8],
    }
}

///
/// This struct packages the options for creating a `World`. The purpose is to provide an easy to
/// use "default options" via `Default::default()`.
///
#[repr(C)]
#[derive(Clone)]
pub struct WorldOptions {
    /// The maximum number of entities that can ever be allocated at one time in the ECS.
    pub entity_capacity: u32,

    /// The maximum number of entities that can ever be allocated within a single archetype at
    /// one time.
    pub archetype_capacity: u32,
}

impl Default for WorldOptions {
    fn default() -> Self {
        Self {
            // 1,048,576
            entity_capacity: 1024 * 1024,

            // 524,288
            archetype_capacity: 1024 * 512,
        }
    }
}

///
///
///
pub struct World {
    /// Configuration options the world was created with
    options: WorldOptions,

    /// Holds all the components that have been registered with the World
    component_registry: ComponentRegistry,

    /// Holds all the entity slots. This handles ID allocation and maps the IDs to their archetype
    entities: EntityStorage,

    /// Map that maps an entity layout to the index inside the archetypes list
    archetype_map: HashMap<EntityLayoutBuf, Option<ArchetypeIndex>>,

    /// The list of all archetypes in the ECS world
    archetypes: Vec<Archetype>,
}

///
/// Implementations for the rust friendly interface
///
impl World {
    ///
    pub fn new(options: WorldOptions) -> std::io::Result<Self> {
        let component_registry = ComponentRegistry::new();
        let entities = EntityStorage::new(options.entity_capacity)?;

        // Create the list of archetypes, with the first slot taken by an archetype with no
        // components.
        //
        // This allows using `ArchetypeIndex(0)` as a special value as its not possible to create
        // an entity with no components.
        let mut archetypes = Vec::new();
        let base_archetype = Archetype::new(1, EntityLayout::empty(), &component_registry);
        archetypes.push(base_archetype);

        // Creates the table that maps entity layouts to archetypes. Maps the empty layout to 0.
        let mut archetype_map = HashMap::new();
        archetype_map.insert(EntityLayoutBuf::new(), None);

        let out = Self {
            options,
            component_registry,
            entities,
            archetype_map,
            archetypes,
        };

        Ok(out)
    }

    /// Register's a rust component type with this ECS world so that it can be used as a component
    #[inline]
    pub fn register<T: Component>(&mut self) -> ComponentTypeDescription {
        self.component_registry.register::<T>()
    }

    /// Erases the entity with the ID from the ECS.
    ///
    /// Returns true if the operation was successful, otherwise returns false.
    ///
    /// If the ID is invalid then this function does nothing and returns false.
    pub fn remove_entity(&mut self, entity: EntityId) -> bool {
        if let Some(entity) = self.entities.lookup(entity) {
            let archetype = &mut self.archetypes[entity.archetype.0.get() as usize];
            archetype.remove_entity::<true>(entity.entity);
            true
        } else {
            false
        }
    }
}

///
/// Implementations for the underlying FFI friendly API
///
impl World {
    /// The function provides the raw implementation of adding to the component registry using an
    /// arbitrary `ComponentTypeDescription`.
    ///
    /// # Safety
    ///
    /// This function is unsafe because there is no way to guarantee that the memory layout provided
    /// is valid for the provided ID. It is possible to provide the ID for a rust type but give an
    /// incorrect size and trigger UB.
    pub unsafe fn register_dynamic(&mut self, description: &ComponentTypeDescription) -> bool {
        self.component_registry.register_dynamic(description)
    }

    /// This function provides the raw implementation of adding a component to an existing entity.
    ///
    /// # Safety
    ///
    /// This function assumes the bytes provided for initializing the component encode a valid bit
    /// pattern for the component type. It also assumes that it takes ownership of the object it
    /// points to and that drop is not called on the underlying object.
    pub unsafe fn add_component_to_entity_dynamic(
        &mut self,
        description: &operations::ComponentInsertionDescription,
    ) -> bool {
        // Lookup the entity location by the provided ID, returning false if the ID is invalid
        let location = if let Some(location) = self.entities.lookup(description.id) {
            location
        } else {
            return false;
        };

        // Lookup the archetype to copy the entity from
        let source_archetype_index = location.archetype;

        // Add the new component to create our destination layout. If the source layout already
        // contains the new component (i.e we're adding a component that is already present) then
        // we return false to specify we did not add the component.
        let source_layout = self.archetypes[source_archetype_index.0.get() as usize]
            .entity_layout()
            .to_owned();
        let mut destination_layout = source_layout.clone();
        if destination_layout.add_component_type(description.component) {
            return false;
        }

        // Find or create the archetype to copy the modified entity into
        let destination_archetype_index = self.find_or_create_archetype(&destination_layout);

        // Move the entity into the destination archetype
        let new_index = self.move_entity_to_archetype::<false>(
            description.id,
            source_archetype_index,
            destination_archetype_index,
        );

        {
            let dest = &mut self.archetypes[destination_archetype_index.0.get() as usize];

            // Get the index of the type inside the archetype and lookup the size of the type
            let type_index = dest
                .entity_layout()
                .index_of_component_type(description.component)
                .unwrap();
            let type_size = dest.component_descriptions()[type_index].type_size;

            // Get the bounds of the component's data
            let dest_base = new_index.0.get() as usize;
            let dest_base = dest_base * type_size;
            let dest_end = dest_base + type_size;

            // Create the slice to copy into, no dropping is needed as the data is uninitialized
            let dest_buffer = dest.component_storage_mut_raw_index(type_index);
            let dest_buffer = &mut dest_buffer[dest_base..dest_end];

            // Perform the actual copy
            dest_buffer.copy_from_slice(description.data);
        }

        // Remove the entity from the previous archetype without dropping the components as they
        // were moved
        let source = &mut self.archetypes[source_archetype_index.0.get() as usize];
        source.remove_entity::<false>(location.entity);

        true
    }

    /// This function provides the raw implementation of removing a component from an entity
    ///
    /// # Safety
    pub unsafe fn remove_component_from_entity_dynamic(
        &mut self,
        description: &operations::ComponentRemovalDescription,
    ) -> bool {
        // Lookup the entity location by the provided ID, returning false if the ID is invalid
        let location = if let Some(location) = self.entities.lookup(description.id) {
            location
        } else {
            return false;
        };

        // Lookup the archetype to copy the entity from
        let source_archetype_index = location.archetype;

        // Add the new component to create our destination layout. If the source layout already
        // contains the new component (i.e we're adding a component that is already present) then
        // we return false to specify we did not add the component.
        let source_layout = self.archetypes[source_archetype_index.0.get() as usize]
            .entity_layout()
            .to_owned();
        let mut destination_layout = source_layout.clone();
        if !destination_layout.remove_component_type(description.component) {
            return false;
        }

        // Find or create the archetype to copy the modified entity into
        let destination_archetype_index = self.find_or_create_archetype(&destination_layout);

        // Move the entity into the destination archetype
        self.move_entity_to_archetype::<false>(
            description.id,
            source_archetype_index,
            destination_archetype_index,
        );

        // Manually drop the component we're removing
        let source = &mut self.archetypes[source_archetype_index.0.get() as usize];
        let type_index = source_layout
            .index_of_component_type(description.component)
            .unwrap();
        let type_size = source.component_descriptions()[type_index].type_size;
        let drop_fn = source.component_descriptions()[type_index].fn_drop;

        if let Some(drop_fn) = drop_fn {
            let base = location.entity.0.get() as usize;
            let base = base * type_size;
            let end = base + type_size;

            let slice = source.component_storage_mut_raw_index(type_index);
            let slice = &mut slice[base..end];

            drop_fn(slice.as_mut_ptr());
        }

        // Remove the entity from the previous archetype without dropping the components as they
        // were moved
        source.remove_entity::<false>(location.entity);

        true
    }

    /// This function provides the raw implementation of inserting entities into the ECS world.
    ///
    /// # Safety
    ///
    /// The actual implementation of this function is safe. All operations on the data itself are
    /// sound and are built from safe interfaces.
    ///
    /// This function is unsafe because there is no sane way to verify the data provided by the
    /// `description` parameter is valid. The buffers for the components could be filled with
    /// garbage data and would then be later read back when querying the world. To prevent this we
    /// mark this function as unsafe.
    ///
    /// To use this function safely the contents of `component_buffers` in `description` must point
    /// to valid type-erased byte buffers for each component's data.
    pub unsafe fn insert_entities_dynamic(
        &mut self,
        description: &mut operations::EntityInsertionDescription,
    ) {
        #[cfg(debug_assertions)]
        {
            self.insert_entities_dynamic_debug_assertions(&description);
        }

        assert!(
            description.ids.len() < (u32::MAX - 1) as usize,
            "Can't allocate more than {} entities",
            (u32::MAX - 1)
        );

        assert!(
            !description.entity_layout.is_empty(),
            "Tried to insert entity with 0 components"
        );

        // Locate the archetype and allocate space in the archetype for the new entities
        let archetype_index = self.find_or_create_archetype(description.entity_layout);
        let archetype = &mut self.archetypes[archetype_index.0.get() as usize];
        let archetype_entity_base = archetype.allocate_entities(description.ids.len() as u32);

        // Copy the component data into the archetype buffers
        for (i, source) in description.component_buffers.iter().cloned().enumerate() {
            // Get the size of the type we're copying from the buffers
            let type_size = archetype.component_descriptions()[i].type_size;

            // Calculate the base index for where to start copying into the buffer
            let base = archetype_entity_base.0.get() as usize;
            let base = base * type_size;

            // Get the target slice to copy into
            let target = archetype.component_storage_mut_raw_index(i);
            let target = &mut target[base..];

            // Perform the actual copy
            target.copy_from_slice(source);
        }

        // Allocate the entity IDs and write them into the output slice
        description.ids.iter_mut().enumerate().for_each(|(i, v)| {
            let entity = archetype_entity_base.0.get() + i as u32;
            let entity = NonZeroU32::new(entity).unwrap();
            let location = EntityLocation {
                archetype: archetype_index,
                entity: ArchetypeEntityIndex(entity),
            };

            *v = self.entities.create(location);
        });
    }

    ///
    pub fn remove_entity_dynamic(
        &mut self,
        description: &operations::EntityRemovalDescription,
    ) -> bool {
        self.remove_entity(description.id)
    }
}

/// Private function implementations
impl World {
    fn find_or_create_archetype(&mut self, layout: &EntityLayout) -> ArchetypeIndex {
        if let Some(archetype) = self.archetype_map.get(layout).cloned() {
            archetype.expect("Tried to lookup the empty archetype")
        } else {
            let capacity = self.options.archetype_capacity;
            let archetype = Archetype::new(capacity, layout, &self.component_registry);
            let archetype_index = self.archetypes.len() as u32;
            let archetype_index = NonZeroU32::new(archetype_index).unwrap();
            self.archetype_map
                .insert(layout.to_owned(), Some(ArchetypeIndex(archetype_index)));
            self.archetypes.push(archetype);
            ArchetypeIndex(archetype_index)
        }
    }

    fn insert_entities_dynamic_debug_assertions(
        &self,
        description: &operations::EntityInsertionDescription,
    ) {
        debug_assert_eq!(
            description.entity_layout.len(),
            description.component_buffers.len(),
            "The number of components in the layout and number of buffers provided must match"
        );

        // Debug assertion that checks that the buffer sizes for each component are exactly the size
        // and alignment needed.
        let layouts = description.entity_layout.iter();
        let descs = layouts.map(|v| {
            let desc = self
                .component_registry
                .lookup(v)
                .expect("Tried to insert an unregistered component type");
            desc
        });
        let buffers = description.component_buffers.iter().cloned();
        for (desc, buffer) in descs.zip(buffers) {
            let required_bytes = description.ids.len() * desc.type_size;
            let actual_bytes = buffer.len();
            debug_assert_eq!(
                required_bytes, actual_bytes,
                "The buffer provided for component {} was the wrong size",
                desc.type_name
            );

            let buffer_base = buffer.as_ptr() as usize;
            debug_assert!(
                buffer_base & (desc.type_align - 1) == 0,
                "The buffer provided for component {} was not sufficiently aligned",
                desc.type_name
            );
        }
    }

    /// # Safety
    ///
    /// This function doesn't check what components intersect from the source and destination
    /// archetypes. If dest is a superset of source then this will leave some component's data
    /// uninitialized.
    ///
    /// To use this safely the data must be initialized manually outside this function in a higher
    /// level wrapper.
    unsafe fn move_entity_to_archetype<const DROP: bool>(
        &mut self,
        target: EntityId,
        source_index: ArchetypeIndex,
        dest_index: ArchetypeIndex,
    ) -> ArchetypeEntityIndex {
        // Use our split_at_mut wrapper to get access to both archetypes mutably
        //
        // Unfortunately this has to be vendored into each function to satisfy the borrow checker
        let (source, dest) = {
            let source: usize = source_index.0.get() as usize;
            let dest: usize = dest_index.0.get() as usize;
            // Handles all cases: <, >, and ==. Will panic from underflow in the == case as that
            // would lead to mutable aliasing.
            if source < dest {
                // Select the pivot based on the lowest of the two indices and split the array
                let pivot = source.checked_add(1).unwrap();
                let (l, r) = self.archetypes.split_at_mut(pivot);

                // Rebase the destination index in the second of the splits
                let dest = dest.checked_sub(pivot).unwrap();

                // Get the references to the target indices
                (&mut l[source as usize], &mut r[dest as usize])
            } else {
                // Select the pivot based on the lowest of the two indices and split the array
                let pivot = dest.checked_add(1).unwrap();
                let (l, r) = self.archetypes.split_at_mut(pivot);

                // Rebase the source index in the second of the splits
                let source = source.checked_sub(pivot).unwrap();

                // Get the references to the target indices
                (&mut l[source], &mut r[dest])
            }
        };

        // Allocate space for the entity in the destination archetype and construct the new
        // location while updating the entity slot
        let entry = self.entities.lookup_entry_mut(target).unwrap();
        let old_index = entry.data.location.unwrap().entity;
        let new_index = dest.copy_from_archetype(old_index, source);
        entry.data.location = Some(EntityLocation {
            archetype: dest_index,
            entity: new_index,
        });

        // Remove the entity from the previous archetype without dropping the components as they
        // were moved
        source.remove_entity::<DROP>(old_index);

        new_index
    }
}
