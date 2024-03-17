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

use crate::{Archetype, EntityLayout, EntityLayoutBuf};

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
        }
    }

    pub fn filter_archetype(&self, archetype: &Archetype) -> bool {
        let all_matching = self
            .matching_components
            .is_subset_of(archetype.entity_layout());
        let no_excluded = self
            .excluded_components
            .is_disjoint_from(archetype.entity_layout());
        all_matching && no_excluded
    }
}
