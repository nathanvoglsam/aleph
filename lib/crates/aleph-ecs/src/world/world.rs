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

use std::any::Any;
use std::cell::UnsafeCell;
use std::collections::HashMap;
use std::num::NonZeroU32;
use std::ptr::NonNull;

use crate::world::{
    Archetype, ArchetypeEntityIndex, ArchetypeIndex, Component, ComponentIdMap, ComponentQuery,
    ComponentRegistry, ComponentTypeDescription, ComponentTypeId, EntityId, EntityLayout,
    EntityLayoutBuf, EntityLocation, EntityStorage, Query, Resource, ResourceId,
};

/// Interface for converting one type into a type that implements `ComponentSource`.
///
/// # Safety
///
/// This trait is marked unsafe as `ComponentSource` is an unsafe trait as well. I have not spent
/// any thought into investigating how safe these operations actually are so they are marked as
/// unsafe pre-emptively until I can prove them as safe.
pub unsafe trait IntoComponentSource {
    type Source: ComponentSource;

    fn into_component_source(self) -> Self::Source;
}

/// Interface expected of a type that is a source of component data for inserting entities into
/// an ECS world.
///
/// # Safety
///
/// This trait is marked as unsafe because any non-trivial implementation is going to use a lot of
/// unsafe code anyway. The entire interface is based around type-erasure and copying data of
/// objects without dropping.
///
/// I have not put time into proving how safe this interface is so I mark it as unsafe
/// pre-emptively. The implementations provided are safe, but the trait remains unsafe for now.
pub unsafe trait ComponentSource {
    fn entity_layout(&self) -> &EntityLayout;

    fn data_for(&self, component: ComponentTypeId) -> &[u8];

    fn count(&self) -> u32;
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
/// # Implementation Details
///
/// The `World` consists of a collection of individual data structures that together form the whole
/// solution for storing entities and their components. These data structures are:
///
/// - A hash table that stores the set of registered component types and maps from the component
///   type's registered ID to the type's description (name, size, alignment, drop_fn, etc).
/// - A free-list based object pool for allocating entity IDs. Reuse of entity slots is made safe
///   with the use of a generational index.
/// - A hash table that maps entity layouts to an index in a set of arrays which form SoA storage
///   for all archetypes within the world. The SoA storage splits the archetype into two pieces:
///     - The core archetype data structure that handles the storage of the component data
///     - A set of graph edges that forms a graph from archetypes to other archetypes for
///       accelerating entity shape transitions.
///
/// ## Archetype
///
/// See [`Archetype`] for more detailed documentation.
///
/// ## Archetype Graph
///
/// The world also pairs archetypes with a set of links to other archetypes with similar layouts.
/// These links, taken together with all other archetypes, form a graph of all the archetypes. The
/// edges of the graph are defined by the addition or removal of a single component type to the
/// source archetype's layout. This forms a graph of neighbouring archetypes joined by the
/// transformation to the source layout required to create the destination layout.
///
/// This graph structure accelerates adding and removing components from entities. Changing an
/// entity's shape requires moving it to the archetype of the target shape. Without this graph in,
/// order to add/remove a component from an entity, it would be necessary to:
///   - Allocate a new [`EntityLayoutBuf`], which requires a heap allocation.
///   - Add the new component type to this [`EntityLayoutBuf`] so we know the layout of the
///     destination archetype.
///   - Use the layout to lookup the destination archetype, which requires a hash table lookup which
///     means we need to hash the layout.
///
/// This will need to done **for every individual entity transformation**. This would add insane
/// amounts of overhead.
///
/// By maintaining this graph it means we can add and remove components from entities without any
/// of the above work by simply following the correct link in the graph. Going from an archetype
/// with layout (A, B, C) to (A, B, C, D) we simply follow the edge for adding component D in the
/// graph. This reduces finding destination archetypes to chains of hash table lookups
/// (with a much smaller key to hash) without any heap allocations required.
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

    /// Holds the edges of the archetype graph. Maps component ID to the links.
    archetype_edges: Vec<ComponentIdMap<ArchetypeEdge>>,

    /// The table of all resources registered with the world, keyed by the type id of the resource
    resources: HashMap<ResourceId, UnsafeCell<Box<dyn Any + Send + Sync + 'static>>>,
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
        let mut archetype_edges = Vec::new();
        let base_archetype = Archetype::new(1, EntityLayout::empty(), &component_registry);
        let base_archetype_edges = ComponentIdMap::with_hasher(Default::default());
        archetypes.push(base_archetype);
        archetype_edges.push(base_archetype_edges);

        // Creates the table that maps entity layouts to archetypes. Maps the empty layout to 0.
        let mut archetype_map = HashMap::new();
        archetype_map.insert(EntityLayoutBuf::new(), None);

        let resources = Default::default();

        let mut out = Self {
            options,
            component_registry,
            entities,
            archetype_map,
            archetypes,
            archetype_edges,
            resources,
        };

        // Insert dummy resource
        out.add_resource(());

        Ok(out)
    }

    /// Returns the number of entities allocated in the `World`
    pub fn len(&self) -> u32 {
        self.entities.len() as u32
    }

    /// Returns if there are no entities in the `World`
    pub fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }

    /// Register's a rust component type with this ECS world so that it can be used as a component
    #[inline]
    pub fn register<T: Component>(&mut self) -> ComponentTypeDescription {
        self.component_registry.register::<T>()
    }

    pub fn add_resource<T: Resource>(&mut self, r: T) {
        let cell = UnsafeCell::new(Box::new(r));
        assert!(self.resources.insert(ResourceId::of::<T>(), cell).is_none());
    }

    pub fn extend<T: IntoComponentSource>(&mut self, source: T) -> Vec<EntityId> {
        let source = source.into_component_source();
        let layout = source.entity_layout();

        let mut ids = Vec::new();
        ids.resize(source.count() as usize, EntityId::null());

        #[cfg(debug_assertions)]
        {
            // Debug assertion that checks that the buffer sizes for each component are exactly the
            // size and alignment needed.
            let layouts = layout.iter();
            let descs = layouts.map(|v| {
                let desc = self
                    .component_registry
                    .lookup(v)
                    .expect("Tried to insert an unregistered component type");
                let buffer = source.data_for(v);
                (desc, buffer)
            });
            for (desc, buffer) in descs {
                let required_bytes = ids.len() * desc.type_size;
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

        assert!(
            ids.len() < (u32::MAX - 1) as usize,
            "Can't allocate more than {} entities",
            u32::MAX - 1
        );

        assert!(
            !layout.is_empty(),
            "Tried to insert entity with 0 components"
        );

        // Locate the archetype and allocate space in the archetype for the new entities
        let archetype_index = self.find_or_create_archetype(layout);
        let archetype = &mut self.archetypes[archetype_index.0.get() as usize];
        let archetype_entity_base = archetype.allocate_entities(ids.len() as u32);

        // Copy the component data into the archetype buffers
        archetype.copy_from_source(archetype_entity_base, source);

        // Allocate the entity IDs and write them into the output slice
        for (i, v) in ids.iter_mut().enumerate() {
            // Calculate the final EntityLocation
            let entity = archetype_entity_base.0.get() + i as u32;
            let entity = ArchetypeEntityIndex(NonZeroU32::new(entity).unwrap());
            let location = EntityLocation {
                archetype: archetype_index,
                entity,
            };

            // Allocate the ID
            let id = self.entities.create(location);

            // Write the ID to the archetype and the output ID list
            *v = id;
            archetype.update_entity_id(entity, id);
        }

        ids
    }

    /// Adds the given component to the entity pointed to by the provided ID.
    ///
    /// If the component already existed on the entity then original component will be left
    /// unchanged and the provided component object will be dropped.
    ///
    /// Returns true if the component is successfully inserted, otherwise returns false.
    #[inline]
    pub fn add_component<T: Component>(&mut self, entity: EntityId, component: T) -> bool {
        // Construct a slice of the component data. This will be used by the underlying
        // implementation
        let data = unsafe {
            let data = &component as *const T as *const u8;
            let len = std::mem::size_of::<T>();
            std::slice::from_raw_parts(data, len)
        };

        // Perform the call, using mem::forget to not drop the component if ownership was
        // successfully transferred into the archetype
        unsafe {
            if self.add_component_dynamic(entity, ComponentTypeId::of::<T>(), data) {
                std::mem::forget(component);
                true
            } else {
                false
            }
        }
    }

    /// Removes the specified component from the provided entity.
    ///
    /// Returns true if the component is successfully removed, otherwise returns false.
    #[inline]
    pub fn remove_component<T: Component>(&mut self, entity: EntityId) -> bool {
        unsafe { self.remove_component_dynamic(entity, ComponentTypeId::of::<T>()) }
    }

    /// Erases the entity with the ID from the ECS.
    ///
    /// Returns true if the operation was successful, otherwise returns false.
    ///
    /// If the ID is invalid then this function does nothing and returns false.
    #[inline]
    pub fn remove_entity(&mut self, entity: EntityId) -> bool {
        if let Some(location) = self.entities.lookup(entity) {
            let archetype = &mut self.archetypes[location.archetype.0.get() as usize];

            // Remove the entity from the archetype, patching the `EntityLocation` if an entity
            // needed to be moved to keep the archetype storage dense.
            if let Some(needs_update) = archetype.remove_entity::<true>(location.entity) {
                unsafe {
                    let entry = self.entities.lookup_entry_mut(needs_update).unwrap();
                    let entry = entry.data.location.as_mut().unwrap();
                    entry.entity = location.entity;
                }
            }

            // Free's the entity ID slot (handles generation increment to invalidate the old IDs)
            self.entities.destroy(entity);

            true
        } else {
            false
        }
    }

    /// Returns whether the specified component has the component `T`.
    #[inline]
    pub fn has_component<T: Component>(&self, entity: EntityId) -> bool {
        self.get_component_ref::<T>(entity).is_some()
    }

    /// Returns a shared reference to the component `T` on the given entity, or `None` if no such
    /// component is attached to the entity.
    #[inline]
    pub fn get_component_ref<T: Component>(&self, entity: EntityId) -> Option<&T> {
        unsafe {
            self.get_component_ptr_dynamic(entity, ComponentTypeId::of::<T>())
                .map(|v| {
                    let ptr = v.as_ptr() as *const u8 as *const T;
                    &*ptr
                })
        }
    }

    /// Returns a mutable reference to the component `T` on the given entity, or `None` if no such
    /// component is attached to the entity.
    #[inline]
    pub fn get_component_mut<T: Component>(&mut self, entity: EntityId) -> Option<&mut T> {
        unsafe {
            self.get_component_ptr_dynamic(entity, ComponentTypeId::of::<T>())
                .map(|v| {
                    let ptr = v.as_ptr() as *mut T;
                    &mut *ptr
                })
        }
    }

    #[inline]
    pub fn query<Q: ComponentQuery>(&mut self) -> Query<Q> {
        Query::new(self)
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
    pub unsafe fn add_component_dynamic(
        &mut self,
        entity: EntityId,
        component: ComponentTypeId,
        data: &[u8],
    ) -> bool {
        // Lookup the entity location by the provided ID, returning false if the ID is invalid
        let location = if let Some(location) = self.entities.lookup(entity) {
            location
        } else {
            return false;
        };

        // Lookup the archetype to copy the entity from
        let source_archetype_index = location.archetype;

        // Find the destination archetype, returning false if the source and destination are the
        // same.
        let destination_archetype_index = if let Some(index) =
            self.follow_archetype_link::<true>(source_archetype_index, component)
        {
            index
        } else {
            return false;
        };

        // Move the entity into the destination archetype
        let new_index = self.move_entity_to_archetype::<false>(
            entity,
            source_archetype_index,
            destination_archetype_index,
        );

        self.archetypes[destination_archetype_index.0.get() as usize]
            .copy_component_data_into_slot(new_index, component, data);

        true
    }

    /// This function provides the raw implementation of removing a component from an entity.
    ///
    /// Returns true if the component existed on the entity and was removed, otherwise returns
    /// false.
    ///
    /// # Safety
    ///
    /// Marked unsafe until the function is proven to be safe, as it currently ambiguous whether
    /// this is safe to call.
    pub unsafe fn remove_component_dynamic(
        &mut self,
        entity: EntityId,
        component: ComponentTypeId,
    ) -> bool {
        // Lookup the entity location by the provided ID, returning false if the ID is invalid
        let location = if let Some(location) = self.entities.lookup(entity) {
            location
        } else {
            return false;
        };

        // Lookup the archetype to copy the entity from
        let source_archetype_index = location.archetype;

        // Find the destination archetype, returning false if the source and destination are the
        // same.
        let destination_archetype_index = if let Some(index) =
            self.follow_archetype_link::<false>(source_archetype_index, component)
        {
            index
        } else {
            return false;
        };

        // Move the entity into the destination archetype
        self.move_entity_to_archetype::<false>(
            entity,
            source_archetype_index,
            destination_archetype_index,
        );

        // Manually drop the component we're removing
        self.archetypes[source_archetype_index.0.get() as usize]
            .drop_component_in_slot(location.entity, component);

        true
    }

    /// This function provides a raw, untyped interface for looking up an individual component for
    /// a given entity.
    #[inline]
    pub fn get_component_ptr_dynamic(
        &self,
        entity: EntityId,
        component: ComponentTypeId,
    ) -> Option<NonNull<u8>> {
        if let Some(location) = self.entities.lookup(entity) {
            self.archetypes[location.archetype.0.get() as usize]
                .get_component_ptr(location.entity, component)
        } else {
            None
        }
    }
}

unsafe impl Send for World {}
unsafe impl Sync for World {}

/// Crate private function implementations
impl World {
    /// Returns the internal archetype list
    pub(crate) fn archetypes_ref(&self) -> &[Archetype] {
        &self.archetypes
    }

    /// Returns the internal archetype list
    pub(crate) fn archetypes_mut(&mut self) -> &mut [Archetype] {
        &mut self.archetypes
    }

    pub(crate) unsafe fn get_resource_ref_unchecked<T: Resource>(&self) -> Option<&T> {
        let resource = self.resources.get(&ResourceId::of::<T>())?;
        (*resource.get()).downcast_ref::<T>()
    }

    pub(crate) unsafe fn get_resource_mut_unchecked<T: Resource>(&self) -> Option<&mut T> {
        let resource = self.resources.get(&ResourceId::of::<T>())?;
        (*resource.get()).downcast_mut::<T>()
    }
}

/// Private function implementations
impl World {
    fn follow_archetype_link<const ADD: bool>(
        &mut self,
        source: ArchetypeIndex,
        component: ComponentTypeId,
    ) -> Option<ArchetypeIndex> {
        let source = source.0.get() as usize;

        // First check for an existing link in the graph
        if let Some(edge) = self.archetype_edges[source].get_mut(&component) {
            // Const switch between add or remove
            if ADD {
                if let Some(index) = edge.add {
                    return Some(index);
                }
            } else if let Some(index) = edge.remove {
                return Some(index);
            }
        }

        // If we get here then we failed to find an existing link so we'll need to lookup the target
        // archetype by layout, which requires an allocation to build the layout to lookup with.
        //
        // At least we'll only ever need to do this once

        // Create the destination layout, returning None if the component we're following a link
        // for doesn't change the layout (i.e trying to go from src->src).
        let source_layout = self.archetypes[source].entity_layout().to_owned();
        let mut destination_layout = source_layout;
        if ADD {
            if destination_layout.add_component_type(component) {
                return None;
            }
        } else if !destination_layout.remove_component_type(component) {
            return None;
        }

        // Lookup the archetype and update the graph edge in source
        let index = self.find_or_create_archetype(&destination_layout);
        let edge = self.archetype_edges[source].entry(component).or_default();
        if ADD {
            edge.add = Some(index);
        } else {
            edge.remove = Some(index);
        }

        Some(index)
    }

    fn find_or_create_archetype(&mut self, layout: &EntityLayout) -> ArchetypeIndex {
        if let Some(archetype) = self.archetype_map.get(layout).cloned() {
            archetype.expect("Tried to lookup the empty archetype")
        } else {
            let capacity = self.options.archetype_capacity;
            let archetype = Archetype::new(capacity, layout, &self.component_registry);
            let archetype_edges = ComponentIdMap::with_hasher(Default::default());
            let archetype_index = self.archetypes.len() as u32;
            let archetype_index = NonZeroU32::new(archetype_index).unwrap();
            self.archetype_map
                .insert(layout.to_owned(), Some(ArchetypeIndex(archetype_index)));
            self.archetypes.push(archetype);
            self.archetype_edges.push(archetype_edges);
            ArchetypeIndex(archetype_index)
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
                (&mut l[source], &mut r[dest])
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
        if let Some(needs_update) = source.remove_entity::<DROP>(old_index) {
            let entry = self.entities.lookup_entry_mut(needs_update).unwrap();
            let entry = entry.data.location.as_mut().unwrap();
            entry.entity = old_index;
        }

        new_index
    }
}

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

#[macro_export]
macro_rules! impl_component_source_for_tuple {
    ($($t: ident), *) => {
        #[allow(non_snake_case)]
        unsafe impl<$($t: $crate::world::Component),+> $crate::world::ComponentSource for (u32, $crate::world::EntityLayoutBuf, $(::std::vec::Vec<::std::mem::ManuallyDrop<$t>>,)+) {
            #[inline]
            fn entity_layout(&self) -> &$crate::world::EntityLayout {
                &self.1
            }

            #[inline(always)]
            fn data_for(&self, component: $crate::world::ComponentTypeId) -> &[u8] {
                let (_, _, $($t,)+) = self;
                $(
                    if component == $crate::world::ComponentTypeId::of::<$t>() {
                        let data = $t.as_ptr() as *const u8;
                        let len = $t.len() * ::std::mem::size_of::<$t>();
                        return unsafe {
                            ::std::slice::from_raw_parts(data, len)
                        };
                    }
                )+
                panic!()
            }

            #[inline(always)]
            fn count(&self) -> u32 {
                self.0
            }
        }

        #[allow(non_snake_case)]
        unsafe impl<$($t: $crate::world::Component),+ ,const SIZE: usize> $crate::world::ComponentSource for ($crate::world::EntityLayoutBuf, $([::std::mem::ManuallyDrop<$t>; SIZE],)+) {
            #[inline]
            fn entity_layout(&self) -> &$crate::world::EntityLayout {
                &self.0
            }

            #[inline(always)]
            fn data_for(&self, component: $crate::world::ComponentTypeId) -> &[u8] {
                let (_, $($t,)+) = self;
                $(
                    if component == $crate::world::ComponentTypeId::of::<$t>() {
                        let data = $t.as_ptr() as *const u8;
                        let len = $t.len() * ::std::mem::size_of::<$t>();
                        return unsafe {
                            ::std::slice::from_raw_parts(data, len)
                        };
                    }
                )+
                panic!()
            }

            #[inline(always)]
            fn count(&self) -> u32 {
                SIZE as u32
            }
        }
    }
}

#[macro_export]
macro_rules! impl_into_component_source_for_tuple {
    ($t0: ident, $($t: ident), *) => {
        #[allow(non_snake_case)]
        unsafe impl<$t0: $crate::world::Component, $($t: $crate::world::Component),+> $crate::world::IntoComponentSource for (::std::vec::Vec<$t0>, $(::std::vec::Vec<$t>,)+) {
            type Source = (u32, $crate::world::EntityLayoutBuf, ::std::vec::Vec<::std::mem::ManuallyDrop<$t0>>, $(::std::vec::Vec<::std::mem::ManuallyDrop<$t>>,)+);

            fn into_component_source(self) -> Self::Source {
                let (mut $t0, $(mut $t,)+) = self;

                let len = $t0.len();

                $(
                    assert_eq!(len, $t.len());
                    let len = $t.len();
                )+

                assert!(len < (u32::MAX - 1) as usize);
                let len = len as u32;

                let mut layout = $crate::world::EntityLayoutBuf::new();
                layout.add_component_type($crate::world::ComponentTypeId::of::<$t0>());
                $(
                    layout.add_component_type($crate::world::ComponentTypeId::of::<$t>());
                )+

                let $t0 = unsafe {
                    let ptr = $t0.as_mut_ptr() as *mut ::std::mem::ManuallyDrop<$t0>;
                    let length = $t0.len();
                    let capacity = $t0.capacity();
                    ::std::mem::forget($t0);
                    Vec::from_raw_parts(ptr, length, capacity)
                };

                $(
                    let $t = unsafe {
                        let ptr = $t.as_mut_ptr() as *mut ::std::mem::ManuallyDrop<$t>;
                        let length = $t.len();
                        let capacity = $t.capacity();
                        ::std::mem::forget($t);
                        ::std::vec::Vec::from_raw_parts(ptr, length, capacity)
                    };
                )+

                (len, layout, $t0, $($t,)+)
            }
        }

        #[allow(non_snake_case)]
        unsafe impl<$t0: $crate::world::Component, $($t: $crate::world::Component),+ , const SIZE: usize> $crate::world::IntoComponentSource for ([$t0; SIZE], $([$t; SIZE],)+) {
            type Source = ($crate::world::EntityLayoutBuf, [::std::mem::ManuallyDrop<$t0>; SIZE], $([::std::mem::ManuallyDrop<$t>; SIZE],)+);

            fn into_component_source(self) -> Self::Source {
                let ($t0, $($t,)+) = self;

                assert!(SIZE < (u32::MAX - 1) as usize);

                let mut layout = $crate::world::EntityLayoutBuf::new();
                layout.add_component_type($crate::world::ComponentTypeId::of::<$t0>());
                $(
                    layout.add_component_type($crate::world::ComponentTypeId::of::<$t>());
                )+

                let $t0 = unsafe {
                    let ptr = &$t0 as *const [$t0; SIZE] as *const [::std::mem::ManuallyDrop<$t0>; SIZE];
                    let value = ptr.read();
                    ::std::mem::forget($t0);
                    value
                };

                $(
                    let $t = unsafe {
                        let ptr = &$t as *const [$t; SIZE] as *const [::std::mem::ManuallyDrop<$t>; SIZE];
                        let value = ptr.read();
                        ::std::mem::forget($t);
                        value
                    };
                )+

                (layout, $t0, $($t,)+)
            }
        }
    };

    ($t0: ident) => {
        #[allow(non_snake_case)]
        unsafe impl<$t0: $crate::world::Component, > $crate::world::IntoComponentSource for (::std::vec::Vec<$t0>, ) {
            type Source = (u32, $crate::world::EntityLayoutBuf, ::std::vec::Vec<::std::mem::ManuallyDrop<$t0>>);

            fn into_component_source(self) -> Self::Source {
                let (mut $t0, ) = self;

                let len = $t0.len();

                assert!(len < (u32::MAX - 1) as usize);
                let len = len as u32;

                let mut layout = $crate::world::EntityLayoutBuf::new();
                layout.add_component_type($crate::world::ComponentTypeId::of::<$t0>());

                let $t0 = unsafe {
                    let ptr = $t0.as_mut_ptr() as *mut ::std::mem::ManuallyDrop<$t0>;
                    let length = $t0.len();
                    let capacity = $t0.capacity();
                    ::std::mem::forget($t0);
                    ::std::vec::Vec::from_raw_parts(ptr, length, capacity)
                };

                (len, layout, $t0)
            }
        }

        #[allow(non_snake_case)]
        unsafe impl<$t0: $crate::world::Component, const SIZE: usize> $crate::world::IntoComponentSource for ([$t0; SIZE], ) {
            type Source = ($crate::world::EntityLayoutBuf, [::std::mem::ManuallyDrop<$t0>; SIZE]);

            fn into_component_source(self) -> Self::Source {
                let ($t0, ) = self;

                assert!(SIZE < (u32::MAX - 1) as usize);

                let mut layout = $crate::world::EntityLayoutBuf::new();
                layout.add_component_type($crate::world::ComponentTypeId::of::<$t0>());

                let $t0 = unsafe {
                    let ptr = &$t0 as *const [$t0; SIZE] as *const [::std::mem::ManuallyDrop<$t0>; SIZE];
                    let value = ptr.read();
                    ::std::mem::forget($t0);
                    value
                };

                (layout, $t0)
            }
        }
    }
}

impl_into_component_source_for_tuple!(A);
impl_into_component_source_for_tuple!(A, B);
impl_into_component_source_for_tuple!(A, B, C);
impl_into_component_source_for_tuple!(A, B, C, D);
impl_into_component_source_for_tuple!(A, B, C, D, E);
impl_into_component_source_for_tuple!(A, B, C, D, E, F);
impl_into_component_source_for_tuple!(A, B, C, D, E, F, G);
impl_into_component_source_for_tuple!(A, B, C, D, E, F, G, H);
impl_into_component_source_for_tuple!(A, B, C, D, E, F, G, H, I);
impl_into_component_source_for_tuple!(A, B, C, D, E, F, G, H, I, J);
impl_into_component_source_for_tuple!(A, B, C, D, E, F, G, H, I, J, K);
impl_into_component_source_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L);
impl_into_component_source_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M);

impl_component_source_for_tuple!(A);
impl_component_source_for_tuple!(A, B);
impl_component_source_for_tuple!(A, B, C);
impl_component_source_for_tuple!(A, B, C, D);
impl_component_source_for_tuple!(A, B, C, D, E);
impl_component_source_for_tuple!(A, B, C, D, E, F);
impl_component_source_for_tuple!(A, B, C, D, E, F, G);
impl_component_source_for_tuple!(A, B, C, D, E, F, G, H);
impl_component_source_for_tuple!(A, B, C, D, E, F, G, H, I);
impl_component_source_for_tuple!(A, B, C, D, E, F, G, H, I, J);
impl_component_source_for_tuple!(A, B, C, D, E, F, G, H, I, J, K);
impl_component_source_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L);
impl_component_source_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M);

#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn world_register(
    mut world: NonNull<World>,
    description: &ComponentTypeDescription,
) -> bool {
    world.as_mut().register_dynamic(description)
}

#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn world_add_component(
    mut world: NonNull<World>,
    entity: EntityId,
    component: ComponentTypeId,
    data: NonNull<u8>,
    data_len: usize,
) -> u32 {
    let data = core::slice::from_raw_parts(data.as_ptr() as *const u8, data_len);
    if world
        .as_mut()
        .add_component_dynamic(entity, component, data)
    {
        1
    } else {
        0
    }
}

#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn world_remove_component(
    mut world: NonNull<World>,
    entity: EntityId,
    component: ComponentTypeId,
) -> u32 {
    if world.as_mut().remove_component_dynamic(entity, component) {
        1
    } else {
        0
    }
}

#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn world_has_component(
    world: NonNull<World>,
    entity: EntityId,
    component: ComponentTypeId,
) -> u32 {
    if world
        .as_ref()
        .get_component_ptr_dynamic(entity, component)
        .is_some()
    {
        1
    } else {
        0
    }
}

#[inline]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn world_get_component_ptr(
    world: NonNull<World>,
    entity: EntityId,
    component: ComponentTypeId,
) -> Option<NonNull<u8>> {
    world.as_ref().get_component_ptr_dynamic(entity, component)
}
