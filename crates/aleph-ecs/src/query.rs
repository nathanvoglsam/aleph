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

use crate::{Archetype, ArchetypeIndex, EntityLayout, EntityLayoutBuf, World};
use std::num::NonZeroU32;
use std::ptr::NonNull;

///
/// The raw implementation of a world query that provides the implementation for an iteration over
/// a subset of the archetypes within a world.
///
/// This provides an iterator over archetypes. Entity iteration within the archetypes is out of
/// scope for this object.
///
/// # Warning
///
/// This interface is implemented with 100% safe code but requires care to be used safely. There
/// is no guarantee that the world isn't mutated between calls to `Self::next` and
/// `Self::current_*`. This means the iterator can be invalidated by incorrectly using this
/// interface.
///
/// Archetypes are type-erased over the components they store. Iterator invalidation *will* cause
/// UB in unsafe code that un-erases the types as the Archetype being pointed to will change.
///
/// This struct exists as an FFI compatible implementation layer that safe wrappers should
/// abstract over
pub struct ArchetypeFilter {
    /// A list of components that *must* be present on a component for the query to match
    ///
    /// We create an owned copy to simplify FFI usage
    matching_components: EntityLayoutBuf,

    /// A list of components that *must not* be present on a component for the query to match
    ///
    /// We create an owned copy to simplify FFI usage
    excluded_components: EntityLayoutBuf,

    /// The current index of the archetype we're looking at
    current: Option<ArchetypeIndex>,
}

impl ArchetypeFilter {
    /// Constructs a new instance of `RawArchetypeQuery` in the default state.
    ///
    /// This struct will only yield archetypes that match the pattern specified when it was created
    ///
    /// # Arguments
    ///
    /// - `matching`: This specifies the list of components that **must** be present in an
    ///               archetype for it to match the query.
    /// - `excluding`: This specifies the list of components that **must not** be present in an
    ///                archetype for it to match the query.
    ///
    pub fn new(matching: &EntityLayout, excluding: &EntityLayout) -> Self {
        Self {
            matching_components: matching.to_owned(),
            excluded_components: excluding.to_owned(),
            current: None,
        }
    }

    /// This function will try to find the next matching archetype in the given `World`, starting
    /// from the last archetype that was found.
    ///
    /// This function needs to be called at least once before any of the `Self::current_*` functions
    /// will yield a value.
    ///
    /// This function will always be valid to call. The `RawArchetypeQuery` will always return to
    /// the default state after a call to `Self::next` fails. What this means is:
    ///
    /// - *When there are no matches in the world:* `Self::next` will endlessly return `false`
    /// - *When there is at least one match in the world:* `Self::next` will return `true` every
    ///   time it finds a matching archetype. Once all matching archetypes have been found the call
    ///   to `Self::next` will fail and `Self` will return to the default state. This means that
    ///   once a full iteration has been performed the iterator will "wrap" and start yielding
    ///   archetypes from the start of the world again. This makes an instance of this iterator
    ///   re-usable.
    pub fn next(&mut self, world: &World) -> bool {
        // Chose the candidate subset of archetypes that should be used to search for the next
        // archetype
        let candidates = match &mut self.current {
            None => world.archetypes.as_slice(),
            Some(current) => {
                // Search for any more archetypes after the current one that matches the pattern
                let current = current.0.get() as usize;
                let next = current.checked_add(1).unwrap();
                &world.archetypes[next..]
            }
        };

        // Search our candidate set for the next matching archetype
        let next_match = candidates.iter().enumerate().find(|(_, v)| {
            self.matching_components.is_subset_of(&v.entity_layout)
                && self.excluded_components.is_disjoint_from(&v.entity_layout)
        });

        // Based on our search result update the iterator and flag if we've found another matching
        // archetype
        if let Some((index, _)) = next_match {
            let index = NonZeroU32::new(index as u32);
            let index = index.map(|v| ArchetypeIndex(v));
            self.current = index;
            true
        } else {
            self.current = None;
            false
        }
    }

    /// Returns a shared reference to the archetype currently being pointed to by the query.
    ///
    /// Will yield none if no archetype is being pointed to
    pub fn current_ref<'a>(&self, world: &'a World) -> Option<&'a Archetype> {
        world.archetypes.get(self.current?.0.get() as usize)
    }

    /// Returns a mutable reference to the archetype currently being pointed to by the query.
    ///
    /// Will yield none if no archetype is being pointed to
    pub fn current_mut<'a>(&self, world: &'a mut World) -> Option<&'a mut Archetype> {
        world.archetypes.get_mut(self.current?.0.get() as usize)
    }

    /// Returns a pointer to the archetype currently being pointed to by the query.
    ///
    /// Will yield none if no archetype is being pointed to
    pub fn current_ptr(&self, world: &World) -> Option<NonNull<Archetype>> {
        let archetype = world.archetypes.get(self.current?.0.get() as usize)?;
        let ptr = NonNull::from(archetype);
        Some(ptr)
    }
}
