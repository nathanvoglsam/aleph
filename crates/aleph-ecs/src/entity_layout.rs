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

use std::{iter::FromIterator, ops::Deref, vec::IntoIter};

use crate::ComponentTypeId;

///
/// A wrapper over a slice of an EntityLayoutBuf
///
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Hash)]
pub struct EntityLayout {
    components: [ComponentTypeId],
}

impl EntityLayout {
    /// An unsafe function similar to `Self::from_inner`. This function skips checking the
    /// requirements and so is marked as unsafe.
    pub unsafe fn from_inner_unchecked<'a>(components: &'a [ComponentTypeId]) -> &'a Self {
        // SAFETY: EntityLayout is just a wrapper of [ComponentTypeId],
        // therefore converting &[ComponentTypeId] to &EntityLayout is safe.
        &*(components as *const [ComponentTypeId] as *const EntityLayout)
    }

    /// Creates an EntityLayout wrapper over the given slice by first checking that the slice meets
    /// the required constraints.
    ///
    /// # Constraints
    ///
    /// The given slice must meet the following requirements:
    ///   - All elements are sorted in ascending order
    ///   - There are no duplicate entries
    ///
    /// If these requirements are not met then the function will return None.
    pub fn from_inner<'a>(components: &'a [ComponentTypeId]) -> Option<&'a Self> {
        // First we check if the given list is sorted
        let is_sorted = components.windows(2).all(|w| w[0] <= w[1]);
        if !is_sorted {
            return None;
        }

        // Then we check that the list contains no duplicates, which can be done as a single
        // iteration after confirming the list is sorted.
        //
        // Because the list is sorted we know that all duplicate items will be next to eachother so
        // to check this we only need to check the neighbouring elements for equality.
        let has_no_duplicates = components.windows(2).all(|w| w[0] != w[1]);
        if !has_no_duplicates {
            return None;
        }

        // SAFETY: We have just checked all the required constraints
        unsafe { Some(Self::from_inner_unchecked(components)) }
    }

    /// Returns whether the given component type is present in the `EntityLayoutBuf`.
    pub fn contains_component_type(&self, id: ComponentTypeId) -> bool {
        self.components.binary_search(&id).is_ok()
    }

    /// Returns if the `EntityLayoutBuf` has no member component types.
    pub fn is_empty(&self) -> bool {
        self.components.is_empty()
    }

    /// Returns the number of component types that the `EntityLayoutBuf` holds.
    pub fn len(&self) -> usize {
        self.components.len()
    }

    /// Returns if `self` is a subset of `other`
    pub fn is_subset_of(&self, other: &EntityLayout) -> bool {
        // Early exit if self is empty, as an empty set is always a subset of any other set.
        if self.is_empty() {
            return true;
        }

        // Create an iterator we can manually iterate over for checking if elements are present in
        // the other set.
        let mut self_iter = self.iter();

        // Need to hold onto the current component to check between iterations over the other set.
        let mut current: ComponentTypeId = self_iter.next().unwrap();

        // Now to check if every element of self can be found inside other
        for other_item in other.iter() {
            // Check the other_item with current to see if current can be found in the other set.
            // If it is found in the other set we then move current to the next element in self.
            if current == other_item {
                // Try to get the next element in self
                if let Some(i) = self_iter.next() {
                    // Update current and iter_count and continue iterating
                    current = i;
                    continue;
                } else {
                    // If we have iterated over the entirety of self then we have found every
                    // element of self in the other set, proving it to be a subset
                    return true;
                }
            }
        }

        // If we reach this point we've iterated over the entire other set without iterating over
        // the entirety of self. This means that not every element in self is in other making it
        // not a subset of other.
        false
    }

    /// Returns if `self` contains no elements in common with other
    pub fn is_disjoint_from(&self, other: &EntityLayoutBuf) -> bool {
        // An empty set is always jisjoint from every other set
        if self.is_empty() {
            return true;
        }

        // Check if any of the IDs in self exist in other
        for id in self.iter() {
            // If we find id in other we have proven self is not disjoint from other.
            if other.contains_component_type(id) {
                return false;
            }
        }

        // If we reach the end of the function we have proven the sets disjoint
        true
    }

    /// An iterator over the components in this layout
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = ComponentTypeId> + 'a {
        self.components.iter().cloned()
    }
}

/// An entity layout description that describes the member components of an entity layout.
///
/// This type is implemented as a sorted vector of component type ids. The ordering of component
/// ids in this data structure has no semantic meaning so it is perfectly valid to use the sort
/// ordering for optimization.
///
/// By keeping the ids sorted we make the very uncommon insertion and removal operations O(log n)
/// while making important set operations O(n). By using an unsorted list most set operations would
/// become O(n^2) while making the rare insertions and removals O(1). Fast comparisons will dominate
/// performance for our use-case so this is what we optimize for.
///
/// For example: subset checks. With an unsorted list, to intersect two layout sets would require
/// a worst case of comparing every element in one list with every element in the other list. With
/// a sorted list the operation becomes a single iteration over both lists.
///
/// # Algorithmic Complexity
///
/// - Insertion: O(log N) where N = number of member components.
/// - Removal: O(log N) where N = number of member components.
/// - Subset Check: O(N) where N = number of member components.
/// - Disjoint Check: O(N log N) where N = number of member components.
///
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct EntityLayoutBuf {
    components: Vec<ComponentTypeId>,
}

impl EntityLayoutBuf {
    /// Constructs a new, empty `EntityLayoutBuf`.
    pub fn new() -> EntityLayoutBuf {
        Self {
            components: Vec::new(),
        }
    }

    /// Constructs a new, empty `EntityLayoutBuf` with capacity for `capacity` elements.
    pub fn with_capacity(capacity: usize) -> EntityLayoutBuf {
        Self {
            components: Vec::with_capacity(capacity),
        }
    }

    /// Adds the given component ID to the `EntityLayoutBuf`.
    ///
    /// Returns true if the component is already present in the `EntityLayoutBuf`, and false if it is not.
    pub fn add_component_type(&mut self, id: ComponentTypeId) -> bool {
        match self.components.binary_search(&id) {
            Ok(_) => true,
            Err(index) => {
                self.components.insert(index, id);
                false
            }
        }
    }

    /// Removes the given component type from the `EntityLayoutBuf`.
    ///
    /// Returns true if the component was present in the `EntityLayoutBuf` and was removed, or false if
    /// the component was not present in the `EntityLayoutBuf`.
    pub fn remove_component_type(&mut self, id: ComponentTypeId) -> bool {
        match self.components.binary_search(&id) {
            Ok(index) => {
                self.components.remove(index);
                true
            }
            Err(_) => false,
        }
    }
}

impl Deref for EntityLayoutBuf {
    type Target = EntityLayout;

    fn deref(&self) -> &EntityLayout {
        let slice = self.components.as_slice();
        // SAFETY: All invariants of EntityLayout::from_inner are upheld by EntityLayoutBuf's
        //         safe interface
        unsafe { EntityLayout::from_inner_unchecked(slice) }
    }
}

impl AsRef<EntityLayout> for EntityLayoutBuf {
    fn as_ref(&self) -> &EntityLayout {
        self
    }
}

impl IntoIterator for EntityLayoutBuf {
    type Item = ComponentTypeId;

    type IntoIter = IntoIter<ComponentTypeId>;

    fn into_iter(self) -> Self::IntoIter {
        self.components.into_iter()
    }
}

impl FromIterator<ComponentTypeId> for EntityLayoutBuf {
    fn from_iter<T: IntoIterator<Item = ComponentTypeId>>(iter: T) -> Self {
        Self {
            components: Vec::from_iter(iter),
        }
    }
}
