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

pub mod query_filter;

use crate::ArchetypeIndex;
use crate::{
    ComponentQuery, ComponentQueryItem, EntityId, EntityLayout, EntityLayoutBuf, Fetch,
    QueryFilter, World,
};

pub struct Query<'world, Q: ComponentQuery, const CHECKED: bool> {
    world: &'world World,
    archetype_filter: QueryFilter,
    state: QueryState<Q>,
}

impl<'world, Q: ComponentQuery, const CHECKED: bool> Query<'world, Q, CHECKED> {
    pub(crate) fn new(world: &'world World) -> Self {
        let mut matching = EntityLayoutBuf::new();
        Q::add_to_layout(&mut matching);

        let archetype_filter = QueryFilter::new(&matching, EntityLayout::empty());

        Self {
            world,
            archetype_filter,
            state: QueryState::Entry,
        }
    }
}

// We implement the query iterator as a state machine.
enum QueryState<Q: ComponentQuery> {
    /// Initial state. Early bounds check+exit before moving to [QueryState::FindingArchetype]
    Entry,

    /// State where we filter archetypes. Transitions to [QueryState::IteratingArchetype] when we
    /// find a matching archetype.
    FindingArchetype(ArchetypeIndex),

    /// State where we iterate an archetype. Transitions to [QueryState::FindingArchetype] when we
    /// have yielded all entities in the archetype.
    IteratingArchetype(ArchetypeIndex, *mut EntityId, *mut EntityId, Q::Fetch),

    /// Terminal state once we've exhausted all archetypes. Any state can transition here if it can
    /// prove there's no archetypes left to match.
    Terminal,
}

impl<'world, Q: ComponentQuery, const CHECKED: bool> Iterator for Query<'world, Q, CHECKED> {
    type Item = (EntityId, ComponentQueryItem<'world, Q>);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match &mut self.state {
                // Initial state. This state just does an initial bounds check and then transitions
                // to archetype filtering.
                QueryState::Entry => {
                    // If there's only a single archetype then we haven't inserted any entities yet
                    // because the 0th archetype is intentionally invalid to give ArchetypeIndex
                    // a niche value.
                    if self.world.archetypes.len() <= 1 {
                        self.state = QueryState::Terminal;
                    } else {
                        self.state = QueryState::FindingArchetype(ArchetypeIndex::first());
                    }
                }

                // This state is entered when searching for the next archetype to iterate. Each
                // iteration of the loop we match against the filter and either iterate the
                // archetype if it matches, move to the next archetype if it doesn't, or move to
                // the terminal state if there's no archetypes left.
                QueryState::FindingArchetype(index) => {
                    // Safety: We never construct this state with an out of bounds index so this
                    //         unchecked index is safe to perform if the remains true.
                    let archetype =
                        unsafe { self.world.archetypes.get_unchecked(index.0.get() as usize) };

                    if self.archetype_filter.filter_archetype(archetype) {
                        let (ids, ids_end) = archetype.entity_id_ptr_range();
                        let (ids, ids_end) = (ids.as_ptr(), ids_end.as_ptr());
                        let fetch = Q::Fetch::create(archetype);

                        // If dynamic borrow checking is enabled then we must acquire our access.
                        // This _may_ panic.
                        if CHECKED {
                            Q::Fetch::acquire_borrow(archetype)
                        }

                        self.state = QueryState::IteratingArchetype(*index, ids, ids_end, fetch);
                    } else {
                        // If the next index is out of bounds then we've reached the end of the
                        // iterator
                        match bounds_check_archetype_index_increment(&self.world, *index) {
                            Some(i) => self.state = QueryState::FindingArchetype(i),
                            None => self.state = QueryState::Terminal,
                        }
                    }
                }

                // This state is entered when we've got a matching archetype and we want to yield
                // all the entities in that archetype. This will step through and yield each entity
                // individually before deciding whether to transition to the terminal state or try
                // to find more archetypes. If we've run out of archetypes we terminate, if we have
                // some we return to searching for archetypes.
                QueryState::IteratingArchetype(index, ids, ids_end, fetch) => {
                    if *ids != *ids_end {
                        // Safety: Borrow checking is handled at a higher layer, but we do bounds
                        //         check and ensure that our pointers are always in bounds and valid
                        //         to access assuming the caller has correctly borrow checked the
                        //         query's component access.
                        unsafe {
                            // Grab the ID and components
                            let out_id = ids.read();
                            let out_fetch = fetch.get();

                            // Increment the iterators
                            fetch.next();
                            *ids = ids.add(1);

                            return Some((out_id, out_fetch));
                        }
                    } else {
                        // Safety: We never construct this state with an out of bounds index so this
                        //         unchecked index is safe to perform if the remains true.
                        let archetype =
                            unsafe { self.world.archetypes.get_unchecked(index.0.get() as usize) };

                        // If dynamic borrow checking is enabled then we must release our access.
                        if CHECKED {
                            Q::Fetch::release_borrow(archetype)
                        }
                        match bounds_check_archetype_index_increment(&self.world, *index) {
                            Some(i) => self.state = QueryState::FindingArchetype(i),
                            None => self.state = QueryState::Terminal,
                        }
                    }
                }

                // Terminal state. Once we're here we've iterated the whole world. Nothing more to
                // yield.
                QueryState::Terminal => {
                    return None;
                }
            }
        }
    }
}

fn bounds_check_archetype_index_increment(
    world: &World,
    i: ArchetypeIndex,
) -> Option<ArchetypeIndex> {
    let next_index = i.0.get() as usize + 1;
    if next_index >= world.archetypes.len() {
        None
    } else {
        unsafe { Some(ArchetypeIndex::new(next_index as u32).unwrap_unchecked()) }
    }
}
