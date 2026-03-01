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

use std::borrow::Borrow;
use std::convert::TryFrom;
use std::iter::FromIterator;
use std::ops::Deref;

use aleph_alloc::alloc::{Allocator, Global};
use aleph_alloc::vec::IntoIter as BIntoIter;
use aleph_alloc::{BBox, BVec};

use crate::component::ComponentId;

/// A list of component IDs that make up a type/archetype layout.
///
/// This type is implemented as a sorted vector of component ids. The ordering of component
/// ids in this data structure has no semantic meaning so it is perfectly valid to use the sort
/// ordering for optimization.
///
/// By keeping the ids sorted we make the very rare insertion and removal operations O(log n)
/// while making important set operations O(n). Using an unsorted list would make most set
/// operations become O(n^2) while making the rare insertions and removals O(1). Fast comparisons
/// will dominate performance for our use-case so this is what we optimize for.
///
/// For example: subset checks. With an unsorted list, to intersect two layout sets would require
/// the worst case of comparing every element in one list with every element in the other list. With
/// a sorted list the operation becomes a single parallel iteration over both lists.
///
/// # Algorithmic Complexity
///
/// - Insertion: O(log N) where N = number of member components.
/// - Removal: O(log N) where N = number of member components.
/// - Subset Check: O(N) where N = number of member components.
/// - Disjoint Check: O(N log N) where N = number of member components.
///
#[derive(Debug)]
#[repr(transparent)]
pub struct TypeLayoutBuf<A: Allocator = Global> {
    components: BVec<ComponentId, A>,
}

impl<A: Allocator + Default> Default for TypeLayoutBuf<A> {
    fn default() -> Self {
        Self {
            components: BVec::new_in(Default::default()),
        }
    }
}

impl TypeLayoutBuf<Global> {
    /// Constructs a new, empty `TypeLayoutBuf`.
    #[inline]
    pub const fn new() -> TypeLayoutBuf {
        Self {
            components: BVec::new(),
        }
    }

    /// Constructs a new, empty `TypeLayoutBuf` with capacity for `capacity` elements.
    #[inline]
    pub fn with_capacity(capacity: usize) -> TypeLayoutBuf {
        Self {
            components: BVec::with_capacity(capacity),
        }
    }
}

impl<A: Allocator> TypeLayoutBuf<A> {
    /// Constructs a new, empty `TypeLayoutBuf`.
    #[inline]
    pub const fn new_in(alloc: A) -> Self {
        Self {
            components: BVec::new_in(alloc),
        }
    }

    /// Constructs a new, empty `TypeLayoutBuf` with capacity for `capacity` elements.
    #[inline]
    pub fn with_capacity_in(capacity: usize, alloc: A) -> Self {
        Self {
            components: BVec::with_capacity_in(capacity, alloc),
        }
    }

    /// Constructs a new [`TypeLayoutBuf`] from the given 'layout' using the given allocator.
    #[inline]
    pub fn from_layout_in(layout: &TypeLayout, alloc: A) -> Self {
        let mut components = BVec::with_capacity_in(layout.components.len(), alloc);
        components.extend(layout.components.iter().copied());
        Self { components }
    }

    /// Adds the given component ID to the `TypeLayoutBuf`.
    ///
    /// Returns true if the component is already present in the `TypeLayoutBuf`, and false if it is not.
    #[inline]
    pub fn add_component_type(&mut self, id: ComponentId) -> bool {
        match self.components.binary_search(&id) {
            Ok(_) => true,
            Err(index) => {
                self.components.insert(index, id);
                false
            }
        }
    }

    /// Removes the given component type from the `TypeLayoutBuf`.
    ///
    /// Returns true if the component was present in the `TypeLayoutBuf` and was removed, or false
    /// if the component was not present in the `TypeLayoutBuf`.
    #[inline]
    pub fn remove_component_type(&mut self, id: ComponentId) -> bool {
        match self.components.binary_search(&id) {
            Ok(index) => {
                self.components.remove(index);
                true
            }
            Err(_) => false,
        }
    }

    /// Converts the layout buf into a boxed [`TypeLayout`].
    ///
    /// If the buffer has excess capacity, its items will be moved into a
    /// newly-allocated buffer with exactly the right capacity.
    #[inline]
    pub fn into_boxed_slice(self) -> BBox<TypeLayout, A> {
        let v = self.components.into_boxed_slice();
        let (v, a) = BBox::<[ComponentId], A>::into_raw_with_allocator(v);
        let v = v as *mut TypeLayout;
        unsafe { BBox::<_, A>::from_raw_in(v, a) }
    }
}

impl<A: Allocator> Deref for TypeLayoutBuf<A> {
    type Target = TypeLayout;

    #[inline]
    fn deref(&self) -> &TypeLayout {
        let slice = self.components.as_slice();
        // SAFETY: All invariants of TypeLayout::from_inner are upheld by the TypeLayoutBuf
        //         safe interface
        unsafe { TypeLayout::from_inner_unchecked(slice) }
    }
}

impl<A: Allocator> AsRef<TypeLayout> for TypeLayoutBuf<A> {
    #[inline]
    fn as_ref(&self) -> &TypeLayout {
        self
    }
}

impl<A: Allocator> Borrow<TypeLayout> for TypeLayoutBuf<A> {
    #[inline]
    fn borrow(&self) -> &TypeLayout {
        self.deref()
    }
}

impl<A: Allocator> IntoIterator for TypeLayoutBuf<A> {
    type Item = ComponentId;

    type IntoIter = BIntoIter<ComponentId, A>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.components.into_iter()
    }
}

impl FromIterator<ComponentId> for TypeLayoutBuf<Global> {
    #[inline]
    fn from_iter<T: IntoIterator<Item = ComponentId>>(iter: T) -> Self {
        let mut components = BVec::from_iter(iter);
        components.sort();
        Self { components }
    }
}

impl<A: Allocator> TryFrom<BVec<ComponentId, A>> for TypeLayoutBuf<A> {
    type Error = ();

    fn try_from(components: BVec<ComponentId, A>) -> Result<Self, Self::Error> {
        if TypeLayout::from_inner(&components).is_some() {
            Ok(Self { components })
        } else {
            Err(())
        }
    }
}

impl<A: Allocator + Clone> Clone for TypeLayoutBuf<A> {
    fn clone(&self) -> Self {
        Self {
            components: self.components.clone(),
        }
    }
}

impl<A: Allocator> PartialEq for TypeLayoutBuf<A> {
    fn eq(&self, other: &Self) -> bool {
        self.components.eq(&other.components)
    }
}

impl<A: Allocator> Eq for TypeLayoutBuf<A> {}

impl<A: Allocator> std::hash::Hash for TypeLayoutBuf<A> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.components.hash(state);
    }
}

/// Non-owning slice form of [`TypeLayoutBuf`].
///
/// Follows all the same invariants. Can't be constructed without being in sorted order, and with
/// no duplicates.
///
/// This is an unsized type!
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Hash)]
pub struct TypeLayout {
    components: [ComponentId],
}

impl TypeLayout {
    /// An unsafe function similar to `Self::from_inner`. This function skips checking the
    /// requirements and so is marked as unsafe.
    ///
    /// # Safety
    ///
    /// It is up to the caller to check the constraints documented on [TypeLayout::from_inner].
    ///
    #[inline]
    pub unsafe fn from_inner_unchecked(components: &[ComponentId]) -> &Self {
        unsafe {
            // SAFETY: TypeLayout is just a wrapper of [ComponentTypeId],
            // therefore converting &[ComponentTypeId] to &TypeLayout is safe.
            &*(components as *const [ComponentId] as *const TypeLayout)
        }
    }

    /// Creates an TypeLayout wrapper over the given slice by first checking that the slice meets
    /// the required constraints.
    ///
    /// # Constraints
    ///
    /// The given slice must meet the following requirements:
    ///   - All elements are sorted in ascending order
    ///   - There are no duplicate entries
    ///
    /// If these requirements are not met then the function will return None.
    #[inline]
    pub fn from_inner(components: &[ComponentId]) -> Option<&Self> {
        // Check if the list is sorted, and contains no duplicates.
        let is_sorted = components.windows(2).all(|w| w[0] < w[1]);
        if !is_sorted {
            return None;
        }

        // SAFETY: We have just checked all the required constraints
        unsafe { Some(Self::from_inner_unchecked(components)) }
    }

    /// A utility that returns an empty TypeLayout. That is, a layout with no components.
    pub fn empty() -> &'static TypeLayout {
        static EMPTY: [ComponentId; 0] = [];

        // SAFETY: The list is empty so there is nothing to actually check so this is safe.
        unsafe { Self::from_inner_unchecked(&EMPTY) }
    }

    /// Returns whether the given component type is present in the `TypeLayout`.
    #[inline]
    pub fn contains_component_type(&self, id: ComponentId) -> bool {
        self.components.binary_search(&id).is_ok()
    }

    /// Returns the index of the component type in the `TypeLayout`, if it exists in the layout
    #[inline]
    pub fn index_of_component_type(&self, id: ComponentId) -> Option<usize> {
        self.components.binary_search(&id).ok()
    }

    /// Returns if the `TypeLayoutBuf` has no member component types.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.components.is_empty()
    }

    /// Returns the number of component types that the `TypeLayoutBuf` holds.
    #[inline]
    pub fn len(&self) -> usize {
        self.components.len()
    }

    /// Returns if `self` is a subset of `other`
    #[inline]
    pub fn is_subset_of(&self, other: &TypeLayout) -> bool {
        // Early exit if self is empty, as an empty set is always a subset of any other set.
        if self.is_empty() {
            return true;
        }

        // Create an iterator we can manually iterate over for checking if elements are present in
        // the other set.
        let mut self_iter = self.iter().copied();

        // Need to hold onto the current component to check between iterations over the other set.
        let mut current: ComponentId = self_iter.next().unwrap();

        // Now to check if every element of self can be found inside other
        for other_item in other.iter().copied() {
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
        // not a subset of 'other'.
        false
    }

    /// Returns if `self` contains no elements in common with other
    #[inline]
    pub fn is_disjoint_from(&self, other: &TypeLayout) -> bool {
        // An empty set is always disjoint from every other set
        if self.is_empty() {
            return true;
        }

        // Check if any of the IDs in self exist in other
        for &id in self.iter() {
            // If we find id in other we have proven self is not disjoint from 'other'.
            if other.contains_component_type(id) {
                return false;
            }
        }

        // If we reach the end of the function we have proven the sets disjoint
        true
    }

    /// An iterator over the components in this layout
    #[inline]
    pub fn iter(&'_ self) -> impl Iterator<Item = &'_ ComponentId> + '_ {
        self.into_iter()
    }

    /// Returns the wrapped slice directly
    #[inline]
    pub fn as_inner(&self) -> &[ComponentId] {
        &self.components
    }
}

impl<'a> IntoIterator for &'a TypeLayout {
    type Item = <&'a [ComponentId] as IntoIterator>::Item;
    type IntoIter = <&'a [ComponentId] as IntoIterator>::IntoIter;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.components.into_iter()
    }
}
