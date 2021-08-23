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

use crate::{
    Archetype, ComponentTypeDescription, ComponentTypeId, EntityId, EntityLayout,
    RawArchetypeQuery, World,
};
use std::ptr::NonNull;

pub unsafe extern "C" fn world_register(
    mut world: NonNull<World>,
    description: &ComponentTypeDescription,
) -> bool {
    world.as_mut().register_dynamic(description)
}

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
pub unsafe extern "C" fn world_get_component_ptr(
    world: NonNull<World>,
    entity: EntityId,
    component: ComponentTypeId,
) -> Option<NonNull<u8>> {
    world.as_ref().get_component_ptr_dynamic(entity, component)
}

pub unsafe extern "C" fn archetype_get_entity_layout(
    archetype: NonNull<Archetype>,
    out_len: &mut usize,
) -> NonNull<ComponentTypeId> {
    let layout = archetype.as_ref().entity_layout.as_inner();

    let ptr = layout.as_ptr() as *mut ComponentTypeId;
    let ptr = NonNull::new_unchecked(ptr);

    *out_len = layout.len();

    ptr
}

pub unsafe extern "C" fn archetype_get_component_descriptions(
    archetype: NonNull<Archetype>,
    out_len: &mut usize,
) -> NonNull<ComponentTypeDescription> {
    let descriptions = &archetype.as_ref().component_descriptions;

    let ptr = descriptions.as_ptr() as *mut ComponentTypeDescription;
    let ptr = NonNull::new_unchecked(ptr);

    *out_len = descriptions.len();

    ptr
}

pub unsafe extern "C" fn archetype_get_component_index(
    archetype: NonNull<Archetype>,
    component: ComponentTypeId,
    out_index: &mut usize,
) -> u32 {
    if let Some(index) = archetype.as_ref().storage_indices.get(&component).copied() {
        *out_index = index;
        1
    } else {
        0
    }
}

pub unsafe extern "C" fn archetype_get_storage_by_index(
    archetype: NonNull<Archetype>,
    index: usize,
) -> NonNull<u8> {
    let storage = archetype.as_ref().storages[index].as_slice();
    NonNull::new_unchecked(storage.as_ptr() as *mut u8)
}

/// `Archetype::len`
pub unsafe extern "C" fn archetype_get_len(archetype: NonNull<Archetype>) -> u32 {
    archetype.as_ref().len
}

/// `Archetype::len`
pub unsafe extern "C" fn archetype_get_capacity(archetype: NonNull<Archetype>) -> u32 {
    archetype.as_ref().capacity
}

/// `RawArchetypeQuery::new`
pub unsafe extern "C" fn archetype_query_new(
    matching: NonNull<ComponentTypeId>,
    matching_len: usize,
    excluding: NonNull<ComponentTypeId>,
    excluding_len: usize,
) -> NonNull<RawArchetypeQuery> {
    // Convert unpacked slice to EntityLayout
    let matching = core::slice::from_raw_parts(matching.as_ptr(), matching_len);
    let matching = EntityLayout::from_inner_unchecked(matching);

    // Convert unpacked slice to EntityLayout
    let excluding = core::slice::from_raw_parts(excluding.as_ptr(), excluding_len);
    let excluding = EntityLayout::from_inner_unchecked(excluding);

    // Construct and box query
    let query = RawArchetypeQuery::new(matching, excluding);
    let query = Box::new(query);

    // Leak the box to transfer lifetime ownership to caller
    let query = Box::leak(query);
    NonNull::from(query)
}

/// `RawArchetypeQuery::next`
pub unsafe extern "C" fn archetype_query_next(
    mut query: NonNull<RawArchetypeQuery>,
    world: NonNull<World>,
) -> u32 {
    // Call `next` converting the bool to a u32
    if query.as_mut().next(world.as_ref()) {
        1
    } else {
        0
    }
}

/// `RawArchetypeQuery::current_ptr`
pub unsafe extern "C" fn archetype_query_current(
    query: NonNull<RawArchetypeQuery>,
    world: NonNull<World>,
) -> Option<NonNull<Archetype>> {
    query.as_ref().current_ptr(world.as_ref())
}

/// `RawArchetypeQuery::drop`
pub unsafe extern "C" fn archetype_query_destroy(query: NonNull<RawArchetypeQuery>) {
    // Recreate and drop the box to call cleanup code
    Box::from_raw(query.as_ptr());
    drop(query)
}
