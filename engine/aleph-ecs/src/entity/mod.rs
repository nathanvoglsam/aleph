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

use aleph_gen_arena::{GenArena, Handle, make_handle_id};

use crate::EcsSystem;

/// Type tag used with [`Handle`] to create a unique 'entity handle' type.
pub struct Entity;
make_handle_id!(Entity);

/// Alias for [`Handle`] with the [`Entity`] tag. In general, use this instead of [`Handle`]
/// directly.
pub type EntityHandle = Handle<Entity>;

/// Specialized alias of [`GenArena`] that provides an arena for our entity handles.
pub type EntityHandleArena = GenArena<EntityLocation, EntityHandle, EcsSystem>;

/// Fully describes the storage location of a specific entity within an ECS world.
///
/// An entity is uniquely identified simply by the archetype and row.
///
/// # Stability
///
/// In general, this is _not_ a stable reference to a specific entity. Only an [`EntityHandle`] can
/// be used to identify the same entity instance. An ECS world may move the physical location of an
/// entity as part of its lifecycle. It may move between archetypes, or move within an archetype as
/// entities are added or removed.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct EntityLocation {
    /// Index into the archetype set that the entity is currently found in. This is also a 'type id'
    /// of sorts.
    pub archetype: usize,

    /// The row inside the linked archetype that the entity can be found. This is used in
    /// conjunction with the archetype index to find the exact location of all the entity's
    /// components.
    pub row: usize,
}
