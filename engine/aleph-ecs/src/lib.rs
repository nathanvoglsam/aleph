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

pub extern crate aleph_object_system as object_system;

mod archetype;
mod component;
mod entity;
mod query;
mod world;

mod utils;

pub use self::archetype::{Archetype, ArchetypeEntityIndex, ArchetypeIndex};
pub use self::component::component_query::{
    ComponentQuery, ComponentQueryItem, ComponentRead, ComponentWrite, Fetch,
    ReadOnlyComponentQuery,
};
pub use self::component::component_registry::ComponentRegistry;
pub use self::component::component_source::{
    CheckedSource, ComponentSource, IntoComponentSource, IntoOneComponentSource,
    OneComponentSource, UnsafeComponentSource, UnsafeComponentSourceEntry,
};
pub use self::component::{Component, ComponentIdMap};
pub use self::entity::entity_layout::{EntityLayout, EntityLayoutBuf};
pub use self::entity::entity_storage::{
    EntityEntry, EntityEntryData, EntityFreeListLink, EntityLocation, EntityStorage,
};
pub use self::entity::{EntityId, EntityIndex};
pub use self::query::query_filter::QueryFilter;
pub use self::query::{QueryMut, QueryRef, UnsafeQuery};
pub use self::utils::Generation;
pub use self::world::{World, WorldOptions};

#[cfg(test)]
mod tests;

pub struct Ecs;
aleph_alloc::new_alloc_category!(Ecs, "01996aaa-df23-7790-ad3f-47f1b2420ee2");

pub type EcsSystem = aleph_alloc::instrumentation::Instrumented<Ecs>;

// TODO: CommandBuffers so that world modification commands can be queued by jobs and then resolved
//       when the execution phase has completed. This completes the functionality of the ECS as
//       currently it's not really possible to modify the world from within the task graph.
