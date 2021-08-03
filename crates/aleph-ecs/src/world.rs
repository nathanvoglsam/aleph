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

        /// An array of pointers to buffers that contain `count` components for each component type.
        ///
        /// Each pointer must point to a buffer that holds `count` items for the given component.
        pub component_buffers: &'a [&'a [u8]],

        /// The buffer to write all the entity ids into
        pub ids: &'b mut [EntityId],

        /// The number of entities we want to create.
        pub count: u32,
    }

    ///
    /// The raw, FFI friendly struct that describes an entity removal operation.
    ///
    #[repr(C)]
    pub struct EntityRemovalDescription<'a> {
        /// The ids for each entity to remove
        pub ids: &'a [EntityId],
    }

    ///
    /// The raw, FFI friendly struct that describes an entity component removal operation.
    ///
    #[repr(C)]
    pub struct ComponentRemovalDescription<'a> {
        /// The layout of every entity pointed to with the `ids` list.
        pub soruce_layout: &'a EntityLayout,

        /// The ids of all the entities we want to operate on
        pub ids: &'a [EntityId],

        /// The ID of the component to remove from the given entities.
        pub component_id: ComponentTypeId,
    }

    ///
    /// The raw, FFI friendly struct that describes an entity component insertion operation.
    ///
    #[repr(C)]
    pub struct ComponentInsertionDescription<'a> {
        /// The layout of every entity pointed to with the `ids` list.
        pub source_layout: &'a EntityLayout,

        /// The ids of all the entities we want to operate on
        pub ids: &'a [EntityId],

        /// The ID of the component to insert into the given entities.
        pub component_id: ComponentTypeId,

        /// A buffer that holds the component for each entity
        pub buffer: &'a [u8],
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

impl World {
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

    /// The function provides the raw implementation of adding to the component registry using an
    /// arbitrary `ComponentTypeDescription`.
    ///
    /// # Safety
    ///
    /// This function is unsafe because there is no way to guarantee that the memory layout provided
    /// is valid for the provided ID. It is possible to provide the ID for a rust type but give an
    /// incorrect size and trigger UB.
    #[inline]
    pub unsafe fn register_dynamic(&mut self, description: ComponentTypeDescription) -> bool {
        self.component_registry.register_dynamic(description)
    }

    #[inline]
    pub fn insert_entities_dynamic(&mut self, description: operations::EntityInsertionDescription) {
        debug_assert_eq!(
            description.count as usize,
            description.ids.len(),
            "The length of the id slice and number of entities must match"
        );

        debug_assert_eq!(
            description.entity_layout.len(),
            description.component_buffers.len(),
            "The number of components in the layout and number of buffers provided must match"
        );

        // Debug assertion that checks that the buffer sizes for each component are exactly the size
        // and alignment needed.
        #[cfg(debug_assertions)]
        {
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
                let required_bytes = description.count as usize * desc.type_size;
                let actual_bytes = buffer.len();
                assert_eq!(
                    required_bytes, actual_bytes,
                    "The buffer provided for component {} was the wrong size",
                    desc.type_name
                );

                let buffer_base = buffer.as_ptr() as usize;
                if buffer_base & (desc.type_align - 1) != 0 {
                    panic!(
                        "The buffer provided for component {} was not sufficiently aligned",
                        desc.type_name
                    );
                }
            }
        }

        for id in description.entity_layout.iter() {
            if self.component_registry.lookup(id).is_none() {
                panic!("Tried to insert an unregistered component type");
            }
        }

        debug_assert!(
            !description.entity_layout.is_empty(),
            "Tried to insert entity with 0 components"
        );

        // Locate the archetype and allocate space in the archetype for the new entities
        let archetype_index = self.find_or_create_archetype(description.entity_layout);
        let archetype = &mut self.archetypes[archetype_index.0.get() as usize];
        let archetype_entity_base = archetype.allocate_entities(description.count);

        // Copy the component data into the archetype buffers
        for (i, (source, comp_id)) in description
            .component_buffers
            .iter()
            .cloned()
            .zip(description.entity_layout.iter())
            .enumerate()
        {
            let desc = self.component_registry.lookup(comp_id).unwrap();
            let base = desc.type_size * archetype_entity_base.0.get() as usize;
            let target = archetype.component_storage_mut_raw_index(i);
            let target = &mut target[base..];
            debug_assert_eq!(
                target.len(),
                source.len(),
                "Target and Source must be the same length"
            );
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

    #[inline]
    pub fn remove_entity(&mut self, entity: EntityId) -> bool {
        if let Some(entity) = self.entities.lookup(entity) {
            let archetype = &mut self.archetypes[entity.archetype.0.get() as usize];
            archetype.remove_entity(entity.entity);
            true
        } else {
            false
        }
    }

    #[inline]
    pub fn remove_entities_dynamic(&mut self, description: operations::EntityRemovalDescription) {
        for id in description.ids.iter().cloned() {
            self.remove_entity(id);
        }
    }
}

impl World {
    #[inline]
    fn find_or_create_archetype(&mut self, layout: &EntityLayout) -> ArchetypeIndex {
        if let Some(archetype) = self.archetype_map.get(layout).cloned() {
            archetype.unwrap()
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
}
