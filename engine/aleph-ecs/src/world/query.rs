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

use std::marker::PhantomData;
use std::ptr::NonNull;

use aleph_alloc::BHashSet;

use crate::EcsSystem;
use crate::component::{Component, ComponentId};
use crate::entity::EntityHandle;
use crate::world::World;

// =================================================================================================
// Traits
// =================================================================================================

pub trait ComponentQuery: Send + Sync {
    type Fetch: for<'a> Fetch<'a>;

    /// An iterator that enumerates all the components being queried, and any extra information
    /// included in [`ComponentQueryInfo`].
    fn query_info() -> impl Iterator<Item = ComponentQueryInfo>;
}

/// Marker trait that extends over [`ComponentQuery`] that should only be implemented for queries
/// that guarantee read-only access.
///
/// To safely implement this, Read-only access across all component matches in the query is
/// required.
///
/// This is needed so we can allow using a shared borrow of the world for read only queries as there
/// wouldn't be a way to determine if a query is purely read-only at compile time otherwise.
pub unsafe trait ReadOnlyComponentQuery: ComponentQuery {}

#[allow(clippy::missing_safety_doc)]
/// Very thin interface over a pointer bumping based iterator.
///
/// This is a super unsafe library internal detail that must be public to allow for library users
/// to call `impl_query_for_tuple`.
pub unsafe trait Fetch<'a>: Clone + Sized {
    type Item;

    /// Constructs an instance of [`Fetch`] from the given archetype.
    ///
    /// Takes a pointer because borrow could mutable or shared depending on the implementation.
    fn create_at(world: &World, archetype: usize, row: usize) -> Option<Self>;

    /// Skip to the next item in the stream
    ///
    /// There is no requirement for this function to do any bounds checks whatsoever. Bounds
    /// checking should be done externally.
    unsafe fn next(&mut self);

    /// Yields the item at the current position
    ///
    /// This *will* trigger UB if called when `Self` is out of bounds. To use this safely bounds
    /// checks must be implemented by users of this trait.
    unsafe fn get(&self) -> Self::Item;
}

// =================================================================================================
// Structs
// =================================================================================================

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct ComponentQueryInfo {
    /// The component ID the query is looking up.
    pub id: ComponentId,

    /// Flags if the component type is required, or denies.
    ///
    /// - `true` = positive bound, component must be present for the query to match.
    /// - `false` = negative bound, component must _not be_ present for the query to match.
    pub required: bool,

    /// Whether the query wants to be able to write to the component.
    pub mutable: bool,
}

/// Type of values yielded by a query
///
/// Once rust offers generic associated types, this will be moved into [`ComponentQuery`].
pub type ComponentQueryItem<'a, Q> = <<Q as ComponentQuery>::Fetch as Fetch<'a>>::Item;

/// Query parameter that declares a query wants to read the given component type `T`.
pub struct Read<T>(PhantomData<T>);

/// Query parameter that declares a query wants to write the given component type `T`.
pub struct Write<T>(PhantomData<T>);

/// Special query parameter that provides a negative bound on a component type. Use this to query
/// for entities that _do not_ have the component type `T`.
///
/// Data for component `T` can not be accessed through this type.
pub struct Not<T>(PhantomData<T>);

/// Internal type that implements `Fetch` for shared references
pub struct ComponentRead<T>(NonNull<T>);

impl<T> Clone for ComponentRead<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl<T> Copy for ComponentRead<T> {}

/// Internal type that implements `Fetch` for mutable references
pub struct ComponentWrite<T>(NonNull<T>);

impl<T> Clone for ComponentWrite<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl<T> Copy for ComponentWrite<T> {}

/// Internal type that implements a dummy `Fetch` that performs no access
pub struct NoFetch<T>(PhantomData<T>);

impl<T> Clone for NoFetch<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl<T> Copy for NoFetch<T> {}

// =================================================================================================
// Query Implementations
// =================================================================================================

impl<T: Component> ComponentQuery for Read<T> {
    type Fetch = ComponentRead<T>;

    #[inline]
    fn query_info() -> impl Iterator<Item = ComponentQueryInfo> {
        std::iter::once(ComponentQueryInfo {
            id: T::DESC.id,
            required: true,
            mutable: false,
        })
    }
}

unsafe impl<T: Component> ReadOnlyComponentQuery for Read<T> {}

impl<T: Component> ComponentQuery for Write<T> {
    type Fetch = ComponentWrite<T>;

    #[inline]
    fn query_info() -> impl Iterator<Item = ComponentQueryInfo> {
        std::iter::once(ComponentQueryInfo {
            id: T::DESC.id,
            required: true,
            mutable: false,
        })
    }
}

impl<T: Component> ComponentQuery for Not<T> {
    type Fetch = NoFetch<T>;

    #[inline]
    fn query_info() -> impl Iterator<Item = ComponentQueryInfo> {
        std::iter::once(ComponentQueryInfo {
            id: T::DESC.id,
            required: false,
            mutable: false,
        })
    }
}

unsafe impl<T: Component> ReadOnlyComponentQuery for Not<T> {}

// =================================================================================================
// Fetch Implementations
// =================================================================================================

unsafe impl<'a, T: Component> Fetch<'a> for ComponentRead<T> {
    type Item = &'a T;

    #[inline]
    fn create_at(world: &World, archetype: usize, row: usize) -> Option<Self> {
        let column = world.components[T::DESC.id]
            .archetypes
            .get(&archetype)?
            .column;
        let ptr = world.archetypes[archetype].columns[column].get_at_index(row)?;
        Some(Self(ptr.cast::<T>()))
    }

    #[inline]
    unsafe fn next(&mut self) {
        unsafe {
            self.0 = self.0.add(1);
        }
    }

    #[inline]
    unsafe fn get(&self) -> Self::Item {
        unsafe { self.0.as_ref() }
    }
}

unsafe impl<'a, T: Component> Fetch<'a> for ComponentWrite<T> {
    type Item = &'a mut T;

    #[inline]
    fn create_at(world: &World, archetype: usize, row: usize) -> Option<Self> {
        let column = world.components[T::DESC.id]
            .archetypes
            .get(&archetype)?
            .column;
        let ptr = world.archetypes[archetype].columns[column].get_at_index(row)?;
        Some(Self(ptr.cast::<T>()))
    }

    #[inline(always)]
    unsafe fn next(&mut self) {
        unsafe {
            self.0 = NonNull::new_unchecked(self.0.as_ptr().add(1));
        }
    }

    #[inline(always)]
    unsafe fn get(&self) -> Self::Item {
        unsafe { &mut *self.0.as_ptr() }
    }
}

unsafe impl<'a, T: Component> Fetch<'a> for NoFetch<T> {
    type Item = ();

    #[inline]
    fn create_at(_world: &World, _archetype: usize, _row: usize) -> Option<Self> {
        Some(Self(Default::default()))
    }

    #[inline(always)]
    unsafe fn next(&mut self) {}

    #[inline(always)]
    unsafe fn get(&self) -> Self::Item {
        ()
    }
}

// =================================================================================================
// Tuple Impl Macro
// =================================================================================================

macro_rules! impl_query_for_tuple {
    ($($t: ident),+) => {
        #[allow(non_snake_case, unused_variables, clippy::unused_unit)]
        unsafe impl<'a, $($t: Fetch<'a>),+> Fetch<'a> for ($($t,)+) {
            type Item = ($($t::Item,)+);

            #[inline]
            fn create_at(world: &World, archetype: usize, row: usize) -> Option<Self> {
                Some(($(<$t as Fetch>::create_at(world, archetype, row)?,)+))
            }

            #[inline]
            unsafe fn next(&mut self) {
                unsafe {
                    let ($($t,)+) = self;
                    ($($t.next(),)+);
                }
            }

            #[inline]
            unsafe fn get(&self) -> Self::Item {
                unsafe {
                    let ($($t,)+) = self;
                    ($($t.get(),)+)
                }
            }
        }

        impl<$($t: ComponentQuery),+> ComponentQuery for ($($t,)+) {
            type Fetch = ($($t::Fetch,)+);

            #[inline]
            fn query_info() -> impl Iterator<Item=ComponentQueryInfo> {
                let _i = std::iter::empty();
                $(let _i = _i.chain(<$t as ComponentQuery>::query_info());)+
                _i
            }
        }

        unsafe impl<$($t: ReadOnlyComponentQuery),*> ReadOnlyComponentQuery for ($($t,)+) {}
    };
}

impl_query_for_tuple!(A);
impl_query_for_tuple!(A, B);
impl_query_for_tuple!(A, B, C);
impl_query_for_tuple!(A, B, C, D);
impl_query_for_tuple!(A, B, C, D, E);
impl_query_for_tuple!(A, B, C, D, E, F);
impl_query_for_tuple!(A, B, C, D, E, F, G);
impl_query_for_tuple!(A, B, C, D, E, F, G, H);
impl_query_for_tuple!(A, B, C, D, E, F, G, H, I);
impl_query_for_tuple!(A, B, C, D, E, F, G, H, I, J);
impl_query_for_tuple!(A, B, C, D, E, F, G, H, I, J, K);
impl_query_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L);
impl_query_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M);

// =================================================================================================
// Query Iterator
// =================================================================================================

pub struct QueryRef<'world, Q: ReadOnlyComponentQuery> {
    pub(crate) world: &'world World,
    pub(crate) inner: UnsafeQuery<<BHashSet<usize, EcsSystem> as IntoIterator>::IntoIter, Q>,
}

impl<'world, Q: ReadOnlyComponentQuery> Iterator for QueryRef<'world, Q> {
    type Item = (EntityHandle, ComponentQueryItem<'world, Q>);

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let (id, fetch) = self.inner.next(self.world)?;
            Some((id.read(), fetch.get()))
        }
    }
}

pub struct QueryMut<'world, Q: ComponentQuery> {
    pub(crate) world: &'world mut World,
    pub(crate) inner: UnsafeQuery<<BHashSet<usize, EcsSystem> as IntoIterator>::IntoIter, Q>,
}

impl<'world, Q: ComponentQuery> Iterator for QueryMut<'world, Q> {
    type Item = (EntityHandle, ComponentQueryItem<'world, Q>);

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let (id, fetch) = self.inner.next(self.world)?;
            Some((id.read(), fetch.get()))
        }
    }
}

pub struct UnsafeQuery<I: Iterator<Item = usize>, Q: ComponentQuery> {
    matches: I,
    state: QueryState<Q>,
}

impl<I: Iterator<Item = usize>, Q: ComponentQuery> UnsafeQuery<I, Q> {
    pub(crate) unsafe fn new(matches: I) -> Self {
        Self {
            matches,
            state: QueryState::FindingArchetype,
        }
    }
}

// We implement the query iterator as a state machine.
enum QueryState<Q: ComponentQuery> {
    /// State where we filter archetypes. Transitions to [QueryState::IteratingArchetype] when we
    /// find a matching archetype.
    FindingArchetype,

    /// State where we iterate an archetype. Transitions to [QueryState::FindingArchetype] when we
    /// have yielded all entities in the archetype.
    IteratingArchetype(NonNull<EntityHandle>, NonNull<EntityHandle>, Q::Fetch),
}

impl<I: Iterator<Item = usize>, Q: ComponentQuery> UnsafeQuery<I, Q> {
    /// The actual component query iterator implementation that is wrapped by [`QueryRef`] and
    /// [`QueryMut`].
    pub fn next<'b>(&mut self, world: &World) -> Option<(NonNull<EntityHandle>, Q::Fetch)> {
        loop {
            match &mut self.state {
                QueryState::FindingArchetype => {
                    let next = self.matches.next()?;

                    let ids = NonNull::from_ref(world.archetypes[next].entity_handles.as_slice());
                    let ids_start = ids.cast::<EntityHandle>();

                    // Safety: Safe by construction, pointer comes from a slice.
                    let ids_end = unsafe { ids_start.add(ids.len()) };

                    // Safety: It's the caller's responsibility to ensure that all archetypes in
                    //         the 'matches' correctly match the filter 'Q'. This is enforced by
                    //         UnsafeQuery's constructor.
                    let fetch = unsafe { Q::Fetch::create_at(world, next, 0).unwrap_unchecked() };

                    // Hand off to the next state in the chain
                    self.state = QueryState::IteratingArchetype(ids_start, ids_end, fetch);
                }

                QueryState::IteratingArchetype(ids, ids_end, fetch) => {
                    if ids != ids_end {
                        // We send out pointers for outer callers to deref so we don't need to
                        // handle borrows in this iterator logic.
                        let out_id = *ids;
                        let out_fetch = fetch.clone();

                        // Safety: Safe by construction. We're iterating over an entire archetype.
                        //         These values are constructed such that we can never make ids
                        //         or fetch invalid.
                        unsafe {
                            *ids = ids.add(1);
                            fetch.next();
                        }

                        return Some((out_id, out_fetch));
                    }

                    // If we reach here we have exhausted the archetype, so we try and find the
                    // next one in the matches set.
                    self.state = QueryState::FindingArchetype;
                }
            }
        }
    }
}
