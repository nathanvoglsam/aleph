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

#![allow(clippy::module_inception)]

pub(crate) mod archetype;
pub(crate) mod archetype_filter;
pub(crate) mod component;
pub(crate) mod component_query;
pub(crate) mod component_registry;
pub(crate) mod entity;
pub(crate) mod entity_layout;
pub(crate) mod entity_storage;
pub(crate) mod generation;
pub(crate) mod query;
pub(crate) mod resource;
pub(crate) mod world;

pub use self::archetype::Archetype;
pub use self::archetype::ArchetypeEntityIndex;
pub use self::archetype::ArchetypeIndex;
pub use self::archetype_filter::ArchetypeFilter;
pub use self::component::Component;
pub use self::component::ComponentIdMap;
pub use self::component::ComponentSet;
pub use self::component::ComponentTypeDescription;
pub use self::component::ComponentTypeId;
pub use self::component::IdentityHasher;
pub use self::component_query::ComponentQuery;
pub use self::component_query::ComponentQueryItem;
pub use self::component_query::ComponentRead;
pub use self::component_query::ComponentWrite;
pub use self::component_query::Fetch;
pub use self::component_registry::ComponentRegistry;
pub use self::entity::EntityId;
pub use self::entity::EntityIndex;
pub use self::entity_layout::EntityLayout;
pub use self::entity_layout::EntityLayoutBuf;
pub use self::entity_storage::EntityEntry;
pub use self::entity_storage::EntityEntryData;
pub use self::entity_storage::EntityFreeListLink;
pub use self::entity_storage::EntityLocation;
pub use self::entity_storage::EntityStorage;
pub use self::generation::Generation;
pub use self::query::Query;
pub use self::resource::Resource;
pub use self::resource::ResourceId;
pub use self::world::ComponentSource;
pub use self::world::IntoComponentSource;
pub use self::world::World;
pub use self::world::WorldOptions;
