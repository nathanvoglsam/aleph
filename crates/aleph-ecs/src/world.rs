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
        pub entity_layout: &'a EntityLayout,

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
        pub entity_layout: &'a EntityLayout,

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
    archetype_map: HashMap<EntityLayoutBuf, u32>,

    /// The list of all archetypes in the ECS world
    archetypes: Vec<Archetype>,
}

impl World {
    pub fn new(options: WorldOptions) -> std::io::Result<Self> {
        let out = Self {
            options: options.clone(),
            component_registry: ComponentRegistry::new(),
            entities: EntityStorage::new(options.entity_capacity)?,
            archetype_map: HashMap::new(),
            archetypes: Vec::new(),
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
    pub fn insert_entities_dynamic(&mut self, payload: operations::EntityInsertionDescription) {
        debug_assert_eq!(
            payload.count as usize,
            payload.ids.len(),
            "The length of the id slice and number of entities must match"
        );

        debug_assert_eq!(
            payload.entity_layout.len(),
            payload.component_buffers.len(),
            "The number of components in the layout and number of buffers provided must match"
        );

        // Debug assertion that checks that the buffer sizes for each component are exactly the size
        // and alignment needed.
        #[cfg(debug_assertions)]
        {
            let layouts = payload.entity_layout.iter();
            let descs = layouts.map(|v| {
                let desc = self
                    .component_registry
                    .lookup(v)
                    .expect("Tried to insert an unregistered component type");
                desc
            });
            let buffers = payload.component_buffers.iter().cloned();
            for (desc, buffer) in descs.zip(buffers) {
                let required_bytes = payload.count as usize * desc.type_size;
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

        for id in payload.entity_layout.iter() {
            if self.component_registry.lookup(id).is_none() {
                panic!("Tried to insert an unregistered component type");
            }
        }

        // Locate the archetype and allocate space in the archetype for the new entities
        let archetype_index = self.find_or_create_archetype(payload.entity_layout);
        let archetype = &mut self.archetypes[archetype_index as usize];
        let archetype_entity_base = archetype.allocate_entities(payload.count);

        // Copy the component data into the archetype buffers
        for (i, (source, comp_id)) in payload
            .component_buffers
            .iter()
            .cloned()
            .zip(payload.entity_layout.iter())
            .enumerate()
        {
            let desc = self.component_registry.lookup(comp_id).unwrap();
            let base = desc.type_size * archetype_entity_base as usize;
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
        payload.ids.iter_mut().enumerate().for_each(|(i, v)| {
            let location = EntityLocation {
                archetype: ArchetypeIndex(archetype_index),
                entity: ArchetypeEntityIndex(archetype_entity_base + i as u32),
            };

            *v = self.entities.create(location);
        });
    }

    #[inline]
    pub fn remove_entity(&mut self, entity: EntityId) -> bool {
        if let Some(entity) = self.entities.lookup(entity) {
            let archetype = &mut self.archetypes[entity.archetype.0 as usize];
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
    fn find_or_create_archetype(&mut self, layout: &EntityLayout) -> u32 {
        if let Some(archetype) = self.archetype_map.get(layout).cloned() {
            archetype
        } else {
            let capacity = self.options.archetype_capacity;
            let archetype = Archetype::new(capacity, layout, &self.component_registry);
            let archetype_index = self.archetypes.len() as u32;
            self.archetype_map
                .insert(layout.to_owned(), archetype_index);
            self.archetypes.push(archetype);
            archetype_index
        }
    }
}
