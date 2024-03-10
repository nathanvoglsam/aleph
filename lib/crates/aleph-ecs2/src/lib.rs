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

extern crate aleph_virtual_buffer as virtual_buffer;

mod archetype;
mod archetype_filter;
mod atomic_borrow;
mod component;
mod component_query;
mod component_registry;
mod entity;
mod entity_layout;
mod entity_storage;
mod generation;
mod query;
mod world;

pub use self::archetype::{Archetype, ArchetypeEntityIndex, ArchetypeIndex};
pub use self::archetype_filter::ArchetypeFilter;
pub use self::component::{
    Component, ComponentIdMap, ComponentSet, ComponentTypeDescription, ComponentTypeId,
};
pub use self::component_query::{
    ComponentQuery, ComponentQueryItem, ComponentRead, ComponentWrite, Fetch,
};
pub use self::component_registry::ComponentRegistry;
pub use self::entity::{EntityId, EntityIndex};
pub use self::entity_layout::{EntityLayout, EntityLayoutBuf};
pub use self::entity_storage::{
    EntityEntry, EntityEntryData, EntityFreeListLink, EntityLocation, EntityStorage,
};
pub use self::generation::Generation;
pub use self::query::Query;
pub use self::world::{ComponentSource, IntoComponentSource, World, WorldOptions};

#[cfg(test)]
mod tests;

pub mod c_api {
    //!
    //! This module exposes a C friendly API for working with the ECS. Most functionality is unsafe as
    //! none of the Rust type system tricks are available to C.
    //!
    //! These functions are intended to be utilized by language or script bindings to enable languages
    //! other than Rust to interface with an ECS world. These provide the low-level building blocks for
    //! building a more ergonomic interface in whichever language is being used.
    //!
    //! # Code Organization Details
    //!
    //! The functions are defined within private modules and re-exported here to avoid having to expose
    //! data structure internals.
    //!

    pub use crate::archetype::{
        archetype_get_capacity, archetype_get_component_descriptions,
        archetype_get_component_index, archetype_get_entity_layout, archetype_get_len,
        archetype_get_storage_by_index,
    };
    pub use crate::archetype_filter::{archetype_filter_destroy, archetype_filter_new};
    pub use crate::world::{
        world_add_component, world_get_component_ptr, world_has_component, world_register,
        world_remove_component,
    };
}

// TODO: CommandBuffers so that world modification commands can be queued by jobs and then resolved
//       when the execution phase has completed. This completes the functionality of the ECS as
//       currently it's not really possible to modify the world from within the task graph.
