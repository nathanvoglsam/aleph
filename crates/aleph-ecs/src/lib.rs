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
mod component;
mod component_query;
mod entity;
mod entity_layout;
mod entity_storage;
mod generation;
mod query;
mod world;

pub mod c_api;

#[cfg(test)]
mod tests;

pub use crate::archetype::Archetype;
pub use crate::archetype::ArchetypeEntityIndex;
pub use crate::archetype::ArchetypeIndex;
pub use crate::archetype_filter::ArchetypeFilter;
pub use crate::component::Component;
pub use crate::component::ComponentIdMap;
pub use crate::component::ComponentRegistry;
pub use crate::component::ComponentSet;
pub use crate::component::ComponentTypeDescription;
pub use crate::component::ComponentTypeId;
pub use crate::component::IdentityHasher;
pub use crate::component_query::ComponentQuery;
pub use crate::component_query::ComponentQueryItem;
pub use crate::component_query::ComponentRead;
pub use crate::component_query::ComponentWrite;
pub use crate::component_query::Fetch;
pub use crate::entity::EntityId;
pub use crate::entity::EntityIndex;
pub use crate::entity_layout::EntityLayout;
pub use crate::entity_layout::EntityLayoutBuf;
pub use crate::entity_storage::EntityEntry;
pub use crate::entity_storage::EntityEntryData;
pub use crate::entity_storage::EntityFreeListLink;
pub use crate::entity_storage::EntityLocation;
pub use crate::entity_storage::EntityStorage;
pub use crate::generation::Generation;
pub use crate::query::Query;
pub use crate::world::ComponentSource;
pub use crate::world::IntoComponentSource;
pub use crate::world::World;
pub use crate::world::WorldOptions;
