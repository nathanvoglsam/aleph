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
    ArchetypeFilter, ComponentQuery, ComponentQueryItem, EntityId, EntityLayout, EntityLayoutBuf,
    Fetch, World,
};
use std::ptr::NonNull;

pub struct Query<'world, Q: ComponentQuery> {
    world: &'world mut World,
    archetype_filter: ArchetypeFilter,
    state: QueryState<Q>,
}

impl<'world, Q: ComponentQuery> Query<'world, Q> {
    #[inline]
    pub(crate) fn new(world: &'world mut World) -> Self {
        let mut matching = EntityLayoutBuf::new();
        Q::add_to_layout(&mut matching);

        let archetype_filter = ArchetypeFilter::new(&matching, EntityLayout::empty());

        Self {
            world,
            archetype_filter,
            state: QueryState::FindingArchetype,
        }
    }
}

enum QueryState<Q: ComponentQuery> {
    FindingArchetype,
    IteratingArchetype(u32, NonNull<EntityId>, Q::Fetch),
    Terminal,
}

impl<'world, Q: ComponentQuery> Iterator for Query<'world, Q> {
    type Item = (EntityId, ComponentQueryItem<'world, Q>);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match &mut self.state {
                QueryState::FindingArchetype => {
                    // In this state we need to try and find the next matching archetype in the
                    // world.
                    //
                    // If we fail to find another match with the `next` call then we move directly
                    // to the `Terminal` state.
                    if self.archetype_filter.next(self.world) {
                        let current = self.archetype_filter.current_ref(self.world).unwrap();
                        let remaining = current.len;
                        let ids = &current.entity_ids[1] as *const EntityId as *mut EntityId;
                        let ids = NonNull::new(ids).unwrap();
                        let fetch = unsafe { Q::Fetch::create(current) };
                        self.state = QueryState::IteratingArchetype(remaining, ids, fetch);
                    } else {
                        self.state = QueryState::Terminal;
                    }
                }
                QueryState::IteratingArchetype(remaining, ids, fetch) => {
                    // In this state we need to iterate through the entities inside
                    if *remaining == 0 {
                        self.state = QueryState::FindingArchetype;
                    } else {
                        *remaining -= 1;
                        unsafe {
                            let out_id = *ids.as_ref();
                            let out_fetch = fetch.get();
                            fetch.next();
                            *ids = NonNull::new_unchecked(ids.as_ptr().add(1));
                            return Some((out_id, out_fetch));
                        }
                    }
                }
                QueryState::Terminal => {
                    // When in the terminal state we will continue yielding `None` forever
                    return None;
                }
            }
        }
    }
}
