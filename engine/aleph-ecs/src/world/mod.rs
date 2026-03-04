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

pub mod component_index;
pub mod insertion;
pub mod query;
#[cfg(test)]
mod tests;

use std::mem::MaybeUninit;
use std::ptr::NonNull;

use aleph_alloc::instrumentation::system;
use aleph_alloc::{BBox, BHashMap, BHashSet, BVec};

use crate::EcsSystem;
use crate::archetype::Archetype;
use crate::component::internal::{COMPONENTS, ComponentIdMap};
use crate::component::{Component, ComponentId};
use crate::entity::{EntityHandle, EntityHandleArena, EntityLocation};
use crate::type_layout::{TypeLayout, TypeLayoutBuf};
use crate::world::component_index::{ComponentArchetypeRecord, ComponentIndex};
use crate::world::insertion::{
    ComponentInsertionInfo, EntityInsertionInfo, RustEntityInsertionInfo, SingleEntityInsertionInfo,
};
use crate::world::query::{
    ComponentQuery, ComponentQueryItem, Fetch, QueryMut, QueryRef, ReadOnlyComponentQuery,
    UnsafeQuery,
};

/// This API is the primary entry point to the `aleph-ecs` library. It is _the_ implementation of
/// the ECS data structure.
///
/// An in-depth explanation of an ECS is out of scope for this documentation. Briefly, ECS is
/// an overused term that describes a logical method of organizing game simulation code and data
/// that also gets lumped in to mean a few specific methods of implementing that logical model.
/// ECS deals with entities as the fundamental primitive object. An entity can have 0 or more
/// components (which are just structs) attached to it. You can identify an entity with a handle,
/// and that handle can be used to read/write the components as well as add/remove them dynamically.
///
/// The _data structures_ that get lumped in are usually some variant of a columnar database.
///
/// This library does not aim to enshrine the ECS programming model as a fundamental interface. This
/// library implements the data structures with no expectations of a 'pure' ECS model.
///
/// `World` implements an archetype based entity database. It exposes an API for managing a singular
/// pool of entities, referenced via [`EntityHandle`], and provides an interface for working with
/// [`Components`](Component) on those entities. Tools are provided to spawn entities, delete
/// entities, add and remove components, get references to those components, and most critically
/// query for all entities that match a filter.
///
/// # The Data
///
/// The data structures that `World` uses to implement this API are an important part of the
/// interface as they make a key set of tradeoffs to make certain operations efficient at the cost
/// of other operations. `World` implements an 'archetype' based ECS. What this means is that the
/// fundamental storage medium of the `World` organizes entities with the same set of components
/// so they are stored together in a single table. Each row of the table is an entity, and each
/// column stores the data for one of the component types. One of these tables is called an
/// 'archetype'.
///
/// When an entity is created, starting with an initial set of components, it is assigned to an
/// archetype that stores exactly those components. It  is stored in a row of the table along with
/// all the other entities of the same _shape_. The _shape_ of an entity is a fundamental term that
/// refers to the set of entities attached to it. The _shape_ of a new entity is determined from the
/// set of components, it takes a row in the table used for its shape, the components are copied
/// into the table, and finally a handle is given out. That handle can be used to find those
/// components again and identify the entity relative to all others within the same `World`.
///
/// An `archetype` is a compacting data structure. When adding/removing entities from an archetype
/// the other entities may be moved to compact the table so all columns are dense with no gaps
/// for empty rows.
///
/// This layout has a number of consequences:
///
/// 1. Iterating within an archetype is _very_ efficient, as each column is just a densely packed
///    array of structs.
/// 2. Finding an entity from its handle is also very efficient. The entity can be located with just
///    an _archetype id_ and a _row index_.
/// 3. Iterating over the whole world is also very efficient. It's just an extra level of iteration.
/// 4. Filtered iteration, searching for entities whose shape matches a filter, is also very
///    efficient. Because all entities of the same shape are stored together, filtering on shape is
///    performed at a much lower frequency by matching _archetypes_ instead of _individual
///    entities_.
/// 5. Adding and removing entities directly to/from an archetype by providing all components
///    upfront is also cheap.
/// 6. Adding/removing components dynamically is expensive. Changing the shape of an entity requires
///    moving it to a different archetype. This adds extra memory traffic beyond just the cost of
///    copying a new component in. The cost scales with the number of components on an entity too.
/// 7. Memory may become fragmented if large numbers of archetypes are created. If the shape of the
///    entities in the world are very inconsistent the entities get spread into a large number of
///    tables and the efficiency of the data structure suffers.
///
/// This library is designed to be most efficient for managing a large number of entities that
/// take on a small number of unique shapes. These entities will rarely change shape, and it is very
/// important that entities can be filtered based on their shape. It is expected that the cheap
/// filtering and iteration will outweigh the cost of the less efficient operations.
///
/// # The Logical Model
///
/// `World` is a data structure. It stores entities, and lets you query them. If you squint, it's
/// a database.
///
/// What this library isn't is an 'ECS framework'. We don't expose 'systems'. We expose entities and
/// queries. We expose enough of the internals that more complex frameworks can be built on top of
/// this fundamental data structure. We keep the library simple enough it could be used as just a
/// data structure.
///
/// The 'ECS framework' can be quite efficient. What it isn't is friendly to your average gameplay
/// programmer. This library is just data. You could expose systems and represent your logic as
/// data transformations. You could use this library as storage for a traditional OOP scripting
/// layer.
///
/// # Components
///
/// A component is a struct. Specifically it's a struct that has been correctly registered with the
/// library. A _Rust_ component can be implicitly 'auto-registered' using
/// [`crate::register_component`]. But ultimately a component is just a bag of bytes with a size
/// and alignment.
///
/// When a component is registered a [`ComponentId`] is given out that identifies that component
/// type within only the `World` it is registered with.
///
/// _Rust_ components that can be used with the safe APIs must be registered using our macro to
/// correctly implement the [`Component`] trait. _Dynamic_ components are those registered at
/// runtime and have no such constraint. Dynamic component types are very powerful as they allow
/// both Rust code or scripting languages loaded as data to create their own component types that
/// are treated as first class citizens next to the built-in Rust components. The `World` can become
/// a fully dynamic type system if you want it!
///
/// ## Auto Registration
///
/// It's worth talking about auto registration to explain what it means and why it's needed. Rust
/// types that implement [`Component`] correctly are auto registered with all `World` instances
/// created within the executable. No explicit registration step is required to retrieve a
/// [`ComponentId`].
///
/// How this works is that each `Component` type registers itself into a global list at startup.
/// When added to the list it is given a global id that is unique within that individual execution
/// of the program. That id is used as the component id for every `World` that gets constructed.
/// That id is also reserved so that no dynamic components can also be assigned that id.
///
/// This means a single, static [`ComponentId`] can be stored for each type `T` that implements
/// [`Component`], and that ID can be retrieved via `T::DESC.id`.
///
/// This system forms the backbone of Rust's safe, generic APIs. We can rely on the reserved ids
/// always referring to the _Rust_ type they were assigned, so we can use them like global type ids.
///
/// This is implemented using a mix of lazy statics and `__attribute__((constructor))` shenanigans.
/// The list of components is constructed before `fn main()` is ever called. The IDs are assigned
/// lazily, however in practice they will all be assigned on the first call to [`World::new`].
///
pub struct World {
    /// Holds all the components that have been registered with the World
    pub(crate) components: ComponentIndex,

    /// Holds all the entity slots. This handles ID allocation and maps the IDs to their archetype
    pub(crate) entities: EntityHandleArena,

    /// The list of all archetypes in the ECS world
    pub(crate) archetypes: BVec<Archetype, EcsSystem>,

    /// Map that maps an entity layout to the index inside the archetypes list
    pub(crate) archetype_map: BHashMap<BBox<TypeLayout, EcsSystem>, usize, EcsSystem>,

    /// Holds the edges of the archetype graph. Maps component ID to the links.
    pub(crate) archetype_add_edges: BVec<ComponentIdMap<usize>, EcsSystem>,

    /// Holds the edges of the archetype graph. Maps component ID to the links.
    pub(crate) archetype_del_edges: BVec<ComponentIdMap<usize>, EcsSystem>,
}

impl World {
    /// Constructs a new, empty ECS world.
    ///
    /// All Rust component types declared using [`crate::register_component`] will be
    /// automatically registered with the ECS world. Any dynamically registered component types are
    /// only usable with the world they were registered from.
    pub fn new() -> Self {
        let mut components = ComponentIndex::new();
        COMPONENTS.iter().enumerate().for_each(|(i, v)| {
            assert_eq!(i, v.id.0 as usize);
            components.push(v);
        });

        let entities = EntityHandleArena::new_in();

        let mut archetypes = BVec::with_capacity_in(1, system());
        let mut archetype_map = BHashMap::new_in(system());
        let mut archetype_add_edges = BVec::with_capacity_in(1, system());
        let mut archetype_del_edges = BVec::with_capacity_in(1, system());

        archetypes.push(Archetype::new(&components, TypeLayout::empty()));
        archetype_map.insert(TypeLayoutBuf::new_in(system()).into_boxed_slice(), 0);
        archetype_add_edges.push(Default::default());
        archetype_del_edges.push(Default::default());

        let out = Self {
            components,
            entities,
            archetype_map,
            archetypes,
            archetype_add_edges,
            archetype_del_edges,
        };

        out
    }

    /// Returns the number of entities allocated in the `World`
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.entities.len()
    }

    /// Returns if there are no entities in the `World`
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }

    /// Spawn an empty entity into the world.
    ///
    /// The resulting entity will have no components, but the handle will be a valid handle.
    #[inline]
    pub fn spawn_entity(&mut self) -> EntityHandle {
        let row = self.archetypes[0].allocate_entities(1);
        let handle = self.entities.alloc(EntityLocation { archetype: 0, row });
        self.archetypes[0].entity_handles[row] = handle;
        handle
    }

    /// Performs a raw, fully dynamically typed bulk insertion operation.
    ///
    /// A table is formed by `types`. Each entry in `types` is a column. The rows a formed by the
    /// individual component items stored in each of our columns.
    ///
    /// As a whole, this table the input data for a collection of new entities to be spawned and
    /// initialized in the ECS world. The columns provide data for each component type, while the
    /// rows provide the data for each component to be added to a spawned entity.
    ///
    /// The types in `types` are dynamic. They declare their type as a `ComponentId`, and associate
    /// it with a type-erased pointer. For each column the attached pointer should be valid for
    /// `size_of(component) * count` bytes. The values stored in each column are _moved_ into the
    /// ECS.
    ///
    /// For a safe, Rust interface use [`World::insert`] or [`World::bulk_insert`].
    ///
    /// # Safety
    ///
    /// There are a few requirements the caller must satisfy for to not invoke UB.
    ///
    /// - Each 'ptr' in the list of types must be valid to read `size_of(component) * count` bytes.
    /// - The data copied from 'ptr' must represent a valid instance of some `T` if the component
    ///   is a Rust type. Garbage bytes could be re-interpreted and laundered as a `T`.
    ///   - This requirement is not relevant if the component type is _not_ a Rust type. Rust
    ///     considers foreign types as bags of bytes.
    /// - If 'out_ptr' is not `None`, it must be valid to write for `count` [`EntityHandle`]
    ///   objects.
    pub unsafe fn raw_bulk_insert(
        &mut self,
        count: usize,
        types: &[ComponentInsertionInfo],
        out_handles: Option<NonNull<EntityHandle>>,
    ) {
        if count == 0 {
            return;
        }

        unsafe {
            self.generic_bulk_insert(count, types, out_handles);
        }
    }

    /// A special case of [`World::bulk_insert`] that initializes exactly one entity.
    ///
    /// An extra generic constraint is added so that arrays of components can't be provided.
    ///
    /// This function allows skipping the heap allocation needed to return a dynamic number of
    /// entity IDs.
    pub fn insert<T: RustEntityInsertionInfo + SingleEntityInsertionInfo>(
        &mut self,
        types: T,
    ) -> EntityHandle {
        debug_assert_eq!(types.count(), 1);

        let mut out_handle = MaybeUninit::uninit();
        unsafe {
            self.generic_bulk_insert(1, types, Some(NonNull::from_mut(&mut out_handle).cast()));
            out_handle.assume_init()
        }
    }

    /// A Rusty, safe extension of [`World::raw_bulk_insert`].
    ///
    /// This uses a collection of marker traits and other unsafe internals to expose a safe version
    /// of the raw bulk insertion API.
    ///
    /// This interface ensures all requirements of the unsafe internals are followed.
    pub fn bulk_insert<T: RustEntityInsertionInfo>(&mut self, types: T) -> Vec<EntityHandle> {
        let count = types.count();
        if count == 0 {
            return Vec::new();
        }

        let mut out_handles = Vec::with_capacity(count);
        let out_handles_ptr = out_handles.spare_capacity_mut();
        unsafe {
            self.generic_bulk_insert(
                count,
                types,
                Some(NonNull::from_mut(out_handles_ptr).cast()),
            );
            out_handles.set_len(count);
        }

        out_handles
    }

    /// Returns `true` if the given entity is both live, and has a component of the given type.
    ///
    /// This is the fully dynamic interface. See [`World::has_component`] for the generic based
    /// version.
    #[inline]
    pub fn raw_has_component(&self, entity: EntityHandle, component: ComponentId) -> bool {
        // Lookup the entity, return None if the entity handle isn't live.
        if let Some(location) = self.entities.get_ref(entity) {
            // Then lookup for the resolved location
            self.entity_has_component(location.archetype, component)
                .is_some()
        } else {
            false
        }
    }

    /// Returns `true` if the given entity is both live, and has a component of type `T`.
    #[inline]
    pub fn has_component<T: Component>(&self, entity: EntityHandle) -> bool {
        self.raw_has_component(entity, T::DESC.id)
    }

    /// Returns `true` if the given entity is live, and accessible, through the given handle.
    #[inline]
    pub fn is_live(&self, entity: EntityHandle) -> bool {
        self.entities.get_ref(entity).is_some()
    }

    /// Returns `Some(ptr)` if the given entity is both live, and has a component of the given type.
    /// `ptr` will point to a component of the requested type.
    ///
    /// See [`World::get_component_ref`] or [`World::get_component_mut`] for a safer, generic
    /// interface.
    #[inline]
    pub fn raw_get_component(
        &self,
        entity: EntityHandle,
        component: ComponentId,
    ) -> Option<NonNull<u8>> {
        // Lookup the entity, return None if the entity handle isn't live.
        let location = self.entities.get_ref(entity)?;

        // Then grab the component pointer
        self.entity_get_component(location.archetype, location.row, component)
    }

    /// Returns `Some(ref)` if the given entity is both live, and has a component of the given type.
    /// `ref` will refer to the component of type `T` associated with the entity.
    #[inline]
    pub fn get_component_ref<T: Component>(&self, entity: EntityHandle) -> Option<&T> {
        let ptr = self.raw_get_component(entity, T::DESC.id)?;

        // Safety: The pointer should be valid for a single T, if raw_get_component is implemented
        //         correctly. We have an appropriate borrow to hand out a &T.
        unsafe { Some(ptr.cast::<T>().as_ref()) }
    }

    /// Returns `Some(ref)` if the given entity is both live, and has a component of the given type.
    /// `ref` will refer to the component of type `T` associated with the entity.
    #[inline]
    pub fn get_component_mut<T: Component>(&mut self, entity: EntityHandle) -> Option<&T> {
        let ptr = self.raw_get_component(entity, T::DESC.id)?;

        // Safety: The pointer should be valid for a single T, if raw_get_component is implemented
        //         correctly. We have an appropriate borrow to hand out a &T.
        unsafe { Some(ptr.cast::<T>().as_mut()) }
    }

    /// Find the index in the archetype table where the archetype for the given type layout can be
    /// found.
    ///
    /// May return `None` if that archetype does not exist yet. An archetype is only created once
    /// an entity with that shape is created.
    #[inline]
    pub fn get_archetype_index(&self, type_layout: &TypeLayout) -> Option<&usize> {
        self.archetype_map.get(type_layout)
    }

    /// The raw, unsafe form of [`World::add_component`]. This allows adding a component with a
    /// dynamic type. Reads the component data from 'src_ptr'.
    ///
    /// # Safety
    ///
    /// This has the same safety issues as [`World::raw_bulk_insert`]. The data referred to
    /// 'src_ptr' must be a valid instance of the component type. This is only required for Rust
    /// types, but the type is dynamic so we can't prove it.
    ///
    /// Additionally, this function assumes 'src_ptr' is valid to read for `size_of(component)`
    /// bytes.
    #[inline]
    pub unsafe fn raw_add_component(
        &mut self,
        entity: EntityHandle,
        component: ComponentId,
        src_ptr: NonNull<u8>,
    ) -> Option<()> {
        unsafe {
            self.generic_add_component(entity, component, |dst_ptr, dst_size| {
                dst_ptr.copy_from_nonoverlapping(src_ptr, dst_size)
            })
        }
    }

    /// Add a component of type `T` to the entity referenced by the given handle.
    ///
    /// If the entity already has a component of type `T` then the old entity will be destructed
    /// and the new component will replace it.
    ///
    /// If the entity did not already have a component of type `T` it will be moved into the
    /// appropriate archetype, freeing its space on the old location.
    ///
    /// Returns `Err` if the handle is invalid, with the `Err` containing the component that
    /// would've been moved into the world.
    #[inline]
    pub fn add_component<T: Component>(&mut self, entity: EntityHandle, v: T) -> Result<(), T> {
        // Wrap in manually drop as the component will be type erased and moved into another
        // storage. We don't want to drop it after it's been moved.
        let mut slot = MaybeUninit::uninit();
        slot.write(v);

        let src_fn = |dst_ptr: NonNull<u8>, _: usize| unsafe {
            let src_ptr = NonNull::from(&slot);
            let dst_ptr = dst_ptr.cast::<MaybeUninit<T>>();
            dst_ptr.copy_from_nonoverlapping(src_ptr, 1);
        };
        unsafe {
            match self.generic_add_component(entity, T::DESC.id, src_fn) {
                None => {
                    // If the add operation fails by returning `None` it means we failed in a way
                    // where the component 'v' is still valid. We pass it back out to the caller so
                    // they may do something with it (very likely drop it).
                    Err(slot.assume_init())
                }
                Some(_) => Ok(()),
            }
        }
    }

    /// The raw, unsafe form of [`World::remove_component`]. Allows removing a component with a
    /// dynamic type. If 'dst_ptr' is `Some`, the removed component will be moved and copied to
    /// 'dst_ptr' instead of destructed in place.
    ///
    /// # Safety
    ///
    /// If a 'dst_ptr' is non-null, 'dst_ptr' must be valid to write `size_of(component)` bytes.
    #[inline]
    pub unsafe fn raw_remove_component(
        &mut self,
        entity: EntityHandle,
        component: ComponentId,
        dst_ptr: Option<NonNull<u8>>,
    ) -> Option<()> {
        match dst_ptr {
            None => unsafe {
                // A type can't be inferred, so just stub it out with a basic function.
                let dst_fn: Option<fn(NonNull<u8>, usize)> = None;
                self.generic_remove_component(entity, component, dst_fn)
            },
            Some(dst_ptr) => unsafe {
                let dst_fn = move |src_ptr: NonNull<u8>, src_size: usize| {
                    dst_ptr.copy_from_nonoverlapping(src_ptr, src_size);
                };
                self.generic_remove_component(entity, component, Some(dst_fn))
            },
        }
    }

    /// Removes a component of type `T` from the entity referenced by the given handle.
    ///
    /// If the entity does not have a component of type `T` this will return `None`.
    ///
    /// If the entity does have a component of type `T` it will be returned as `Some(t)`. The entity
    /// will move archetype, and it's old space will be recycled.
    ///
    /// If the handle is invalid, then `None` will also be returned.
    #[inline]
    pub fn remove_component<T: Component>(&mut self, entity: EntityHandle) -> Option<T> {
        let mut v = MaybeUninit::<T>::uninit();
        let dst_ptr = NonNull::from(&mut v);

        let dst_fn = move |src_ptr: NonNull<u8>, _: usize| unsafe {
            let src_ptr = src_ptr.cast::<MaybeUninit<T>>();
            dst_ptr.copy_from_nonoverlapping(src_ptr, 1);
        };

        unsafe {
            self.generic_remove_component(entity, T::DESC.id, Some(dst_fn))?;
            Some(v.assume_init())
        }
    }

    /// Given a handle 'entity', remove that entity from the world. This will drop all the
    /// components and invalidate the handle.
    ///
    /// Returns `Some` if the handle was valid and an entity was successfully removed from the
    /// world.
    ///
    /// Returns `None` if the handle was invalid and no entity was successfully removed.
    pub fn remove_entity(&mut self, entity: EntityHandle) -> Option<()> {
        let location = self.entities.free(entity)?;

        unsafe {
            // location.archetype should never be out of bounds
            let archetype = self
                .archetypes
                .get_mut(location.archetype)
                .unwrap_unchecked();

            for c in archetype.type_layout.iter().copied() {
                // an archetype should never have a component that isn't registered with the world
                let type_info = self.components.get(c).unwrap_unchecked();

                // and a component should never not know of the archetype and the column it's in.
                let column = type_info
                    .archetypes
                    .get(&location.archetype)
                    .unwrap_unchecked()
                    .column;

                // only need to drop the component if there's a destructor to call
                if let Some(drop) = type_info.desc.destructor {
                    // column should never be out of bounds
                    let column = archetype.columns.get_mut(column).unwrap_unchecked();

                    // location.row should never be out of bounds
                    let row = column.get_at_index(location.row).unwrap_unchecked();

                    // finally, we can drop.
                    drop(row.cast(), 1);
                }
            }
        }

        if let Some(moved) = self.archetypes[location.archetype].remove_entity(location.row) {
            // Patch the location of the moved entity if a swap-remove operation was performed.
            unsafe {
                self.entities.get_mut(moved).unwrap_unchecked().row = location.row;
            }
        }

        Some(())
    }

    /// Fetches the components specified in the query `Q` for the given entity. Returns `None` if
    /// all the query terms could not be satisfied.
    ///
    /// This applies the query fetcher machinery to load the data for a single entity, but does not
    /// need to perform a full query operation. The archetype is known from the entity, we only need
    /// to check that the individual entity passes the query.
    ///
    /// If the entity handle is invalid then we return `None` without ever needing to test the
    /// filter.
    pub fn query_one<Q: ReadOnlyComponentQuery>(
        &self,
        entity: EntityHandle,
    ) -> Option<ComponentQueryItem<'_, Q>> {
        let location = self.entities.get_ref(entity)?;

        // We check if the entity matches the query.
        for c in Q::query_info() {
            let entity_has = self.components[c.id]
                .archetypes
                .get(&location.archetype)
                .is_some();
            match (entity_has, c.required) {
                (true, true) => {}
                (false, false) => {}
                _ => return None,
            }
        }

        unsafe {
            let fetch =
                Q::Fetch::create_at(self, location.archetype, location.row).unwrap_unchecked();
            Some(fetch.get())
        }
    }

    /// Mutable version of [`World::query_one`].
    ///
    /// Has the additional constraint that a component type can only be referenced once across all
    /// terms in the query. Otherwise, it would be possible to create aliasing mutable references
    /// via this interface. For example: the query (Read<T>, Write<T>) would match and fetch the
    /// references for the same T twice, once as mutable and as a shared reference. This is UB in
    /// Rust.
    ///
    /// Rather than do a more in-depth borrow check that fully checks shared-xor-mutable, we just
    /// deny listing the same type in multiple terms. There's no good reason to do it anyway.
    ///
    /// # What about `query_one`?
    ///
    /// [`World::query_one`] skips this check. `query_one` is only allowed to make shared
    /// references, which it is perfectly sound to alias. Using the same twice is nonsense, but it
    /// isn't unsound for read-only access.
    ///
    pub fn query_one_mut<Q: ComponentQuery>(
        &mut self,
        entity: EntityHandle,
    ) -> Option<ComponentQueryItem<'_, Q>> {
        // First verify the entity is live before we do our more expensive validation
        let location = self.entities.get_ref(entity)?;

        // Mutable queries must ensure that each component ID is only referenced once within a
        // single query. There's no compile time machinery to prevent adding two write bounds on
        // the same component. Without this validation it would be possible to construct aliasing
        // mutable references, which would be instant UB.
        //
        // I have yet to find a good way to validate this at compile time, unfortunately, so we're
        // going to have to do it at runtime for now.
        //
        // We make a pragmatic choice to use a 64 element, stack allocated buffer to do the
        // duplicate checks. We should not heap allocate here. Rust isn't capable of right-sizing
        // the buffer currently so this is the best we can do. It's likely 64 components is enough
        // for any sane use of this function, and avoiding a heap allocation is worth the
        // limitation.
        //
        // We then produce a 'TypeLayout' using the validated path. This provides exactly the
        // validation we need.
        {
            let mut components = [ComponentId(0); 64];

            let mut count = 0;
            for (i, v) in Q::query_info().enumerate() {
                count = usize::max(count, i);
                let i = components
                    .get_mut(i)
                    .expect("Must have less than 64 query items");
                *i = v.id;
            }

            let components = &mut components[0..=count];
            components.sort_unstable();
            let _layout = TypeLayout::from_inner(&components)
                .expect("Must be no duplicate components referenced in query");
        }

        // We check if the entity matches the query. We don't need to filter archetypes because we
        // know the one we're looking for, instead we need to check if the archetype matches the
        // query to determine if we can yield a value or not.
        //
        // So instead of the set operations we just check the archetype
        for c in Q::query_info() {
            let entity_has = self.components[c.id]
                .archetypes
                .get(&location.archetype)
                .is_some();
            match (entity_has, c.required) {
                (true, true) => {}
                (false, false) => {}
                _ => return None,
            }
        }

        // Safety: It's a bug for any of the indices to be wrong so we can assume they're valid here
        //         as they all come from within the ECS world. The other constraint, on mutable
        //         references, is checked above.
        unsafe {
            let fetch =
                Q::Fetch::create_at(self, location.archetype, location.row).unwrap_unchecked();
            Some(fetch.get())
        }
    }

    /// Given a query `Q`, yield an iterator that will yield all entities in the world that match
    /// that query filter.
    ///
    /// `Q` is bounded with [`ReadOnlyComponentQuery`], which means this interface can only be
    /// used with queries made entirely of read-only terms. This is checked at compile time using
    /// the restricted trait bound.
    ///
    /// # Duplicate Terms
    ///
    /// [`World::query_one`] allows the same component type to appear in multiple terms of a query.
    /// [`World::query_one_mut`] does not because that would allow unsound aliasing of mutable
    /// references.
    ///
    /// [`World::query`] __also denies__ the same component type appearing in multiple terms of
    /// the query. This wouldn't be unsound, but it is somewhat pointless and likely an error.
    ///
    /// This function re-uses the same machinery as [`World::query_mut`], which _must_ do the
    /// duplicate component term check. We don't think it's worth having the extra machinery to
    /// skip the check here.
    pub fn query<Q: ReadOnlyComponentQuery>(&self) -> QueryRef<'_, Q> {
        let matches = self.find_query_matches::<Q>();

        unsafe {
            QueryRef::<Q> {
                world: self,
                inner: UnsafeQuery::new(matches.into_iter()),
            }
        }
    }

    /// Given a query `Q`, yield an iterator that will yield all entities in the world that match
    /// that query filter.
    ///
    /// `Q` is bounded with [`ComponentQuery`] which allows any query term to be used. Mutable or
    /// read-only.
    ///
    /// Much like [`World::query_one_mut`], the same component type can not appear in more than one
    /// query term. This is to prevent unsound mutable aliasing by trying to read and write a
    /// component in the same query.
    pub fn query_mut<Q: ComponentQuery>(&mut self) -> QueryMut<'_, Q> {
        let matches = self.find_query_matches::<Q>();

        unsafe {
            QueryMut::<Q> {
                world: self,
                inner: UnsafeQuery::new(matches.into_iter()),
            }
        }
    }

    /// Delete all entities in the world that match the given query `Q`.
    ///
    /// Queries match on types, so what this function does in practice is finds every _archetype_
    /// that matches `Q` and clears it, dropping all components and releasing all entity handles.
    pub fn remove_matching<Q: ReadOnlyComponentQuery>(&mut self) {
        let matches = self.find_query_matches::<Q>();
        for i in matches {
            // First we free all the entity handles so the entities in the matching archetype will
            // become unavailable as they will be considered dead.
            //
            // We do this first so if we panic mid-call we just leak entities.
            self.free_all_entities_in_archetype(i);

            // Then we drop all the entities in the archetype. This also internally has ordering
            // constraints to keep this code panic safe.
            //
            // This also resets the length of the archetype to 0.
            self.clear_archetype(i);
        }
    }
}

impl World {
    fn push_new_archetype(&mut self, layout: &TypeLayout) -> Option<usize> {
        // Verify that each type in the layout actually exists before we do anything. We don't want
        // to find this out halfway through and leave the world in a bad state.
        for &c in layout {
            self.components.get(c)?;
        }

        // Construct a new archetype and insert it into the list of archetypes.
        let new_archetype = Archetype::new(&self.components, layout);
        let new_index = self.archetypes.len();
        self.archetypes.push(new_archetype);

        // Insert an entry into the archetype map so you can look up the archetype by the
        // type layout.
        let boxed_layout = TypeLayoutBuf::from_layout_in(layout, system());
        let boxed_layout = boxed_layout.into_boxed_slice();
        self.archetype_map.insert(boxed_layout, new_index);

        // Register the archetype with each component in the component index, providing the column
        // index of the components.
        for (c, t) in layout.iter().copied().enumerate() {
            self.components[t]
                .archetypes
                .insert(new_index, ComponentArchetypeRecord { column: c });
        }

        self.archetype_add_edges.push(Default::default());
        self.archetype_del_edges.push(Default::default());

        Some(new_index)
    }

    #[inline(always)]
    fn entity_has_component(
        &self,
        archetype: usize,
        component: ComponentId,
    ) -> Option<&ComponentArchetypeRecord> {
        let component_info = self.components.get(component)?;
        component_info.archetypes.get(&archetype)
    }

    #[inline(always)]
    fn entity_get_component(
        &self,
        archetype: usize,
        row: usize,
        component: ComponentId,
    ) -> Option<NonNull<u8>> {
        let arch = self.archetypes.get(archetype)?;
        let column = self.entity_has_component(archetype, component)?.column;
        let ptr = arch.columns[column].get_at_index(row)?;
        Some(ptr)
    }

    #[inline]
    fn copy_component_from_to_archetype(
        &mut self,
        src_archetype: usize,
        src_row: usize,
        dst_archetype: usize,
        dst_row: usize,
        component: ComponentId,
    ) -> Option<()> {
        let component_info = self.components.get(component)?;

        let src_column = component_info.archetypes.get(&src_archetype)?.column;
        let dst_column = component_info.archetypes.get(&dst_archetype)?.column;

        let [src_archetype, dst_archetype] = self
            .archetypes
            .get_disjoint_mut([src_archetype, dst_archetype])
            .ok()?;

        let src_ptr = src_archetype.columns[src_column].get_at_index(src_row)?;
        let dst_ptr = dst_archetype.columns[dst_column].get_at_index(dst_row)?;

        // Safety: Implementation should guarantee that if all of the above operations succeed then
        //         this access is valid
        unsafe {
            dst_ptr.copy_from_nonoverlapping(src_ptr, component_info.desc.size);
        }

        Some(())
    }

    /// Returns a [`HashSet`] of archetype indices that match a given filter.
    ///
    /// Only archetypes that have a column for every component ID in the given list will be returned
    /// in the set.
    fn find_archetypes_with_components(
        &self,
        components: &[ComponentId],
    ) -> BHashSet<usize, EcsSystem> {
        let mut matches = BHashSet::new_in(system());

        // We first seed the matches with all archetypes that contain the first component in the
        // list. We want the intersection of 'archetypes' for each component types so we start with
        // simply the set for the first component.
        let mut iter = components.iter();
        if let Some(&first) = iter.next() {
            for (&a, _) in self.components[first].archetypes.iter() {
                matches.insert(a);
            }
        }

        // We then iterate the remaining components in the filter and only retain archetypes that
        // also contain each component type.
        //
        // The end result will have 'matches' contain indices for all archetypes that contain at
        // least every component in the input filter.
        for &rest in iter {
            matches.retain(|a| self.components[rest].archetypes.contains_key(a));
        }

        matches
    }

    /// Attempts to follow an 'add' component edge in the archetype graph, returning the archetype
    /// index pointing to the matching archetype.
    ///
    /// This will construct missing edges using a more expensive fallback path if the edge hasn't
    /// been formed yet.
    ///
    /// If the archetype doesn't exist yet this will create a new, empty archetype containing all
    /// components from the source archetype plus the additional component provided by the caller.
    fn follow_add_component_edge(
        &mut self,
        src_archetype_index: usize,
        component: ComponentId,
    ) -> Option<usize> {
        // First we try the graph, looking if there is an existing edge for adding the given
        // component type.
        match self.archetype_add_edges[src_archetype_index]
            .get(&component)
            .copied()
        {
            None => {
                // If the edge hasn't been formed we must build a type layout (allocates) and
                // look the archetype up in the map.
                //
                // The slow-path requires us to materialize the dst layout so we can find it in
                // the archetype map.
                let mut dst_layout: TypeLayoutBuf<EcsSystem> = TypeLayoutBuf::from_layout_in(
                    self.archetypes[src_archetype_index].type_layout(),
                    system(),
                );
                dst_layout.add_component_type(component);

                let idx = match self.archetype_map.get(dst_layout.as_ref()) {
                    None => {
                        // If we get here it's even more dire. The archetype doesn't exist yet
                        // so we have to make it.
                        //
                        // This is the worst possible case. It is, however, extremely rare.
                        self.push_new_archetype(&dst_layout)?
                    }
                    Some(&v) => v,
                };

                // In any case the index is missing from the graph so we add it. We also update
                // the del_edges side while we're here.
                self.archetype_add_edges[src_archetype_index].insert(component, idx);
                self.archetype_del_edges[idx].insert(component, src_archetype_index);

                Some(idx)
            }
            v => v,
        }
    }

    /// Attempts to follow a 'del' component edge in the archetype graph, returning the archetype
    /// index pointing to the matching archetype.
    ///
    /// This will construct missing edges using a more expensive fallback path if the edge hasn't
    /// been formed yet.
    ///
    /// If the archetype doesn't exist yet this will create a new, empty archetype containing all
    /// components from the source archetype plus the additional component provided by the caller.
    fn follow_del_component_edge(
        &mut self,
        src_archetype_index: usize,
        component: ComponentId,
    ) -> Option<usize> {
        // First we try the graph, looking if there is an existing edge for adding the given
        // component type.
        match self.archetype_del_edges[src_archetype_index]
            .get(&component)
            .copied()
        {
            None => {
                // If the edge hasn't been formed we must build a type layout (allocates) and
                // look the archetype up in the map.
                //
                // The slow-path requires us to materialize the dst layout so we can find it in
                // the archetype map.
                let mut dst_layout: TypeLayoutBuf<EcsSystem> = TypeLayoutBuf::from_layout_in(
                    &self.archetypes[src_archetype_index].type_layout,
                    system(),
                );
                dst_layout.remove_component_type(component);

                let idx = match self.archetype_map.get(dst_layout.as_ref()) {
                    None => {
                        // If we get here it's even more dire. The archetype doesn't exist yet
                        // so we have to make it.
                        //
                        // This is the worst possible case. It is, however, extremely rare.
                        self.push_new_archetype(&dst_layout)?
                    }
                    Some(&v) => v,
                };

                // In any case the index is missing from the graph so we add it. We also update
                // the add_edges side while we're here.
                self.archetype_del_edges[src_archetype_index].insert(component, idx);
                self.archetype_add_edges[idx].insert(component, src_archetype_index);

                Some(idx)
            }
            v => v,
        }
    }

    /// Given some set of component ids in 'types', validate that each type exists
    fn find_archetype_for_component_set(
        &mut self,
        types: impl ExactSizeIterator<Item = ComponentId>,
    ) -> Option<usize> {
        // Use a stack allocated buffer for low numbers of components. This saves allocating
        // by using the stack for scratch space.
        let mut indices_stack = [ComponentId(0); 32];

        // If there's too many component types we fall back to a heap allocation for our
        // layout scratch space.
        let mut indices_heap: BVec<_, EcsSystem> = BVec::new_in(system());

        let indices = if types.len() <= indices_stack.len() {
            // Take a sub-slice of the stack space to use as scratch space.
            let indices_stack = &mut indices_stack[0..types.len()];

            // Copy the IDs for each component in the input set into the scratch space. Both slices
            // will be the same length.
            indices_stack.iter_mut().zip(types).for_each(|(dst, src)| {
                *dst = src;
            });

            indices_stack
        } else {
            // Copy the IDs from the info into the heap indices list.
            indices_heap.extend(types);
            indices_heap.as_mut_slice()
        };

        // Then sort the IDs to make it a valid TypeLayout (almost).
        indices.sort_unstable();

        // Then finally we can convert to a 'TypeLayout'. We can't use the unchecked version
        // because we still need to assert there aren't any duplicates.
        let layout = TypeLayout::from_inner(&indices)?;

        let archetype_index = match self.get_archetype_index(layout) {
            None => {
                let new_index = self.push_new_archetype(layout)?;
                new_index
            }
            Some(&index) => index,
        };

        Some(archetype_index)
    }

    /// # Safety
    ///
    /// This has the same safety issues as [`World::raw_bulk_insert`]. The data referred to
    /// 'src_ptr' must be a valid instance of the component type. This is only required for Rust
    /// types, but the type is dynamic so we can't prove it.
    ///
    /// Additionally, this function assumes 'src_ptr' is valid to read for `size_of(component)`
    /// bytes.
    unsafe fn generic_add_component(
        &mut self,
        entity: EntityHandle,
        component: ComponentId,
        src_fn: impl FnOnce(NonNull<u8>, usize),
    ) -> Option<()> {
        // Get the physical location of the entity so we know where to copy from.
        //
        // Bail if the entity handle isn't valid.
        let src_location = self.entities.get_ref(entity).cloned()?;
        let src_archetype = src_location.archetype;
        let src_row = src_location.row;

        // Grab the metadata for the component we're adding.
        //
        // Bail if the component id isn't valid.
        let component_info = self.components.get(component)?;
        let component_size = component_info.desc.size;

        if let Some(dst_ptr) = self.entity_get_component(src_archetype, src_row, component) {
            // If the entity already contains a component of the given type we overwrite the
            // existing component.
            unsafe {
                // We're overwriting an existing component so we drop the old one before replacing
                // it with the new one.
                if let Some(destructor) = component_info.desc.destructor {
                    destructor(dst_ptr.cast(), 1);
                }
                src_fn(dst_ptr, component_size);
            }
        } else {
            // If the entity does not already have a component of the given type then we must add
            // it by moving the entity to a different archetype.
            //
            // We use the archetype graph to efficiently find the destination archetype.
            let dst_archetype = self.follow_add_component_edge(src_archetype, component)?;

            // Allocate space in the destination archetype
            //
            // Then write the ID so the row knows the handle that points to it.
            let dst_row = self.archetypes[dst_archetype].allocate_entities(1);
            self.archetypes[dst_archetype].entity_handles[dst_row] = entity;

            // Copy all the existing components from the source archetype into the destination
            for i in 0..self.archetypes[src_archetype].type_layout.len() {
                let c = self.archetypes[src_archetype].type_layout.as_inner()[i];
                // Safety: The list of conditions is long...
                //
                // - src and dst must be different archetypes
                // - src and dst archetypes must have initialized components of type c
                // - src and dst row must be initialized and valid for access
                // - c must be valid component
                //
                // These are either explicitly checked or implicitly enforced by the implementation.
                unsafe {
                    self.copy_component_from_to_archetype(
                        src_archetype,
                        src_row,
                        dst_archetype,
                        dst_row,
                        c,
                    )
                    .unwrap_unchecked();
                }
            }

            // Now that the entity has been fully moved to the destination archetype we remove it
            // from the source archetype.
            //
            // We _don't_ drop any of the components because they were _moved_.
            if let Some(moved) = self.archetypes[src_archetype].remove_entity(src_row) {
                // Patch the location of the moved entity if a swap-remove operation was performed.
                unsafe {
                    self.entities.get_mut(moved).unwrap_unchecked().row = src_row;
                }
            }

            // Copy the new component into the destination from the source data provided by the
            // caller.
            unsafe {
                let dst_ptr = self
                    .entity_get_component(dst_archetype, dst_row, component)
                    .unwrap_unchecked();
                src_fn(dst_ptr, component_size);
            }

            // And, finally, patch the location of the entity that we added the component to.
            unsafe {
                let location = self.entities.get_mut(entity).unwrap_unchecked();
                location.archetype = dst_archetype;
                location.row = dst_row;
            }
        }

        Some(())
    }

    unsafe fn generic_remove_component(
        &mut self,
        entity: EntityHandle,
        component: ComponentId,
        dst_fn: Option<impl FnOnce(NonNull<u8>, usize)>,
    ) -> Option<()> {
        // Get the physical location of the entity so we know where to copy from.
        //
        // Bail if the entity handle isn't valid.
        let src_location = self.entities.get_ref(entity).cloned()?;
        let src_archetype = src_location.archetype;
        let src_row = src_location.row;

        // Grab the metadata for the component we're removing.
        //
        // Bail if the component id isn't valid.
        let component_info = self.components.get(component)?;
        let component_size = component_info.desc.size;
        let component_drop = component_info.desc.destructor;

        // Lookup the component in the archetype it is currently found in. If this fails then
        let src_ptr = self.entity_get_component(src_archetype, src_row, component)?;

        let dst_archetype = self.follow_del_component_edge(src_archetype, component)?;
        debug_assert_ne!(src_archetype, dst_archetype);

        // Allocate space in the destination archetype
        //
        // Then write the ID so the row knows the handle that points to it.
        let dst_row = self.archetypes[dst_archetype].allocate_entities(1);
        self.archetypes[dst_archetype].entity_handles[dst_row] = entity;

        // Copy all components, except for the component we're removing. This is done by using the
        // dst layout which is guaranteed to a subset of the src layout.
        for i in 0..self.archetypes[dst_archetype].type_layout.len() {
            let c = self.archetypes[dst_archetype].type_layout.as_inner()[i];
            // Safety: The list of conditions is long...
            //
            // - src and dst must be different archetypes
            // - src and dst archetypes must have initialized components of type c
            // - src and dst row must be initialized and valid for access
            // - c must be valid component
            //
            // These are either explicitly checked or implicitly enforced by the implementation.
            unsafe {
                self.copy_component_from_to_archetype(
                    src_archetype,
                    src_row,
                    dst_archetype,
                    dst_row,
                    c,
                )
                .unwrap_unchecked();
            }
        }

        if let Some(dst_fn) = dst_fn {
            dst_fn(src_ptr, component_size);
        } else {
            unsafe {
                // We're removing the component and not handing it to the caller so we must call
                // drop on it ourselves.
                if let Some(destructor) = component_drop {
                    destructor(src_ptr.cast(), 1);
                }
            }
        }

        // Now that the entity has been fully moved to the destination archetype we remove it
        // from the source archetype.
        //
        // We _don't_ drop any of the components because they were _moved_.
        if let Some(moved) = self.archetypes[src_archetype].remove_entity(src_row) {
            // Patch the location of the moved entity if a swap-remove operation was performed.
            unsafe {
                self.entities.get_mut(moved).unwrap_unchecked().row = src_row;
            }
        }

        // And, finally, patch the location of the entity that we added the component to.
        unsafe {
            let location = self.entities.get_mut(entity).unwrap_unchecked();
            location.archetype = dst_archetype;
            location.row = dst_row;
        }

        Some(())
    }

    /// # Safety
    ///
    /// There are a few requirements the caller must satisfy for to not invoke UB.
    ///
    /// - Each 'ptr' in the list of types must be valid to read `size_of(component) * count` bytes.
    /// - The data copied from 'ptr' must represent a valid instance of some `T` if the component
    ///   is a Rust type. Garbage bytes could be re-interpreted and laundered as a `T`.
    ///   - This requirement is not relevant if the component type is _not_ a Rust type. Rust
    ///     considers foreign types as bags of bytes.
    /// - If 'out_ptr' is not `None`, it must be valid to write for `count` [`EntityHandle`]
    ///   objects.
    #[inline]
    unsafe fn generic_bulk_insert(
        &mut self,
        count: usize,
        info: impl EntityInsertionInfo,
        out_handles: Option<NonNull<EntityHandle>>,
    ) {
        if count == 0 {
            return;
        }

        // Check if attempting to allocate info.count additional entities will exhaust the capacity
        // of the handle arena.
        let new_entity_count = self.entities.len().saturating_add(count);
        assert!(new_entity_count <= u32::MAX as usize, "Too many entities!");

        let archetype_index = self
            .find_archetype_for_component_set(info.types())
            .expect("Insertion info can't contain duplicate component channels");

        // Allocate space for 'info.count' additional entities in the destination archetype.
        //
        // 'base_index' is the index of the first entity, with 'info.count' new entities after
        // this index forming the full set to initialize.
        let base_row = self.archetypes[archetype_index].allocate_entities(count);

        // Initialize the components (each type, for each new entity) by copying from the source
        // data into the appropriate column.
        unsafe {
            info.copy_into_columns(self, archetype_index, base_row, count);
        }

        // Once we've copied the component data we allocate entity handles for each new entity and
        // update the id back-reference entries in the archetype.
        let ids = &mut self.archetypes[archetype_index].entity_handles;
        match out_handles {
            None => {
                let mut row = base_row;
                for id in &mut ids[base_row..] {
                    *id = self.entities.alloc(EntityLocation {
                        archetype: archetype_index,
                        row,
                    });

                    row += 1;
                }
            }
            Some(mut out_handles) => {
                let mut row = base_row;
                for id in &mut ids[base_row..] {
                    *id = self.entities.alloc(EntityLocation {
                        archetype: archetype_index,
                        row,
                    });

                    unsafe {
                        out_handles.write(*id);
                        out_handles = out_handles.add(1);
                    }

                    row += 1;
                }
            }
        }
    }

    fn find_query_matches<Q: ComponentQuery>(&self) -> BHashSet<usize, EcsSystem> {
        let mut matches = [ComponentId(0); 64];
        let mut required = [ComponentId(0); 64];
        let mut denied = [ComponentId(0); 64];

        let (required, denied) = {
            let mut matches_count = 0;
            let mut required_count = 0;
            let mut denied_count = 0;
            for v in Q::query_info() {
                matches[matches_count] = v.id;
                matches_count += 1;
                if v.required {
                    required[required_count] = v.id;
                    required_count += 1;
                } else {
                    denied[required_count] = v.id;
                    denied_count += 1;
                }
            }

            // This validates that we don't match the same component multiple times in the query.
            //
            // If we didn't check this we could make aliasing mutable references (UB).
            let matches = &mut matches[0..matches_count];
            matches.sort_unstable();
            let _matches_layout = TypeLayout::from_inner(&matches)
                .expect("Must be no duplicate components referenced in query");

            // Now we can make the required and denied subset layouts.
            //
            // These don't need to be de-duplicated or sorted!
            let required = &mut required[0..required_count];
            let denied = &mut denied[0..denied_count];

            (required, denied)
        };

        let mut required_archetypes = self.find_archetypes_with_components(required);
        let denied_archetypes = self.find_archetypes_with_components(denied);
        required_archetypes.retain(|v| !denied_archetypes.contains(v));

        required_archetypes
    }

    /// Internal function that frees all entity handles for the live entities stored in the given
    /// archetype.
    fn free_all_entities_in_archetype(&mut self, archetype: usize) {
        let archetype = &mut self.archetypes[archetype];
        let count = archetype.len();
        for &handle in &archetype.entity_handles[0..count] {
            let _ = self.entities.free(handle);
        }
    }

    /// Internal function that calls drop on every live component in the given archetype. This will
    /// also clear the archetype (set the length to 0).
    fn clear_archetype(&mut self, archetype: usize) {
        let archetype = &mut self.archetypes[archetype];

        // Capture the number of live components right before we...
        let count = archetype.len();

        // 'Clear' the archetype. It is now logically empty. len = 0
        //
        // We do this _before_ dropping the components so that if we panic while dropping them then
        // we just leak components. Otherwise, we would try and free components a second time in the
        // World destructor while unwinding.
        archetype.clear();

        unsafe {
            for (column, &component) in archetype.type_layout.iter().enumerate() {
                let type_info = self.components.get(component).unwrap_unchecked();
                if let Some(drop) = type_info.desc.destructor {
                    let column = archetype.columns.get(column).unwrap_unchecked();
                    if let Some(column) = column.get() {
                        drop(column.cast(), count as u64);
                    }
                }
            }
        }
    }
}

impl Drop for World {
    fn drop(&mut self) {
        // Walk through all the columns for all the archetypes and drop any components that are
        // still left alive in the world when it was dropped.
        for archetype in 0..self.archetypes.len() {
            self.clear_archetype(archetype);
        }
    }
}
