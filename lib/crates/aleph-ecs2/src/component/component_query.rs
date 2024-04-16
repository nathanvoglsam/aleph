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

use std::num::NonZeroU32;
use std::ptr::NonNull;

use crate::{Archetype, ArchetypeEntityIndex, Component, ComponentTypeId, EntityLayoutBuf};

pub trait ComponentQuery: Send + Sync {
    type Fetch: for<'a> Fetch<'a>;

    fn add_to_layout(layout: &mut EntityLayoutBuf);

    /// Returns whether the component query asks for mutable access for _any_ component.
    fn wants_any_mutable_access() -> bool;
}

/// Type of values yielded by a query
///
/// Once rust offers generic associated types, this will be moved into [`Query`].
pub type ComponentQueryItem<'a, Q> = <<Q as ComponentQuery>::Fetch as Fetch<'a>>::Item;

#[allow(clippy::missing_safety_doc)]
/// Very thin interface over a pointer bumping based iterator.
///
/// This is a super unsafe library internal detail that must be public to allow for library users
/// to call `impl_query_for_tuple`.
pub unsafe trait Fetch<'a>: Sized {
    type Item;

    /// Attempts to acquire appropriate borrow access to the component storage for this 'fetch'.
    ///
    /// Will panic if appropriate access could not be obtained safely.
    fn acquire_borrow(archetype: &Archetype);

    /// Will release appropriate borrow access to the component storage for this 'fetch'.
    fn release_borrow(archetype: &Archetype);

    /// Constructs an instance of [`Fetch`] from the given archetype.
    ///
    /// Takes a pointer because borrow could mutable or shared depending on the implementation.
    fn create(archetype: &Archetype) -> Self {
        Self::create_at(archetype, ArchetypeEntityIndex(NonZeroU32::new(1).unwrap()))
    }

    /// Constructs an instance of [`Fetch`] from the given archetype.
    ///
    /// Takes a pointer because borrow could mutable or shared depending on the implementation.
    fn create_at(archetype: &Archetype, entity: ArchetypeEntityIndex) -> Self;

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

impl<'a, T: Component> ComponentQuery for &'a T {
    type Fetch = ComponentRead<T>;

    #[inline]
    fn add_to_layout(layout: &mut EntityLayoutBuf) {
        if layout.add_component_type(ComponentTypeId::of::<T>()) {
            panic!("Trying to lookup the same component multiple times within the same query");
        }
    }

    #[inline(always)]
    fn wants_any_mutable_access() -> bool {
        false
    }
}

/// Internal type that implements `Fetch` for shared references
pub struct ComponentRead<T>(NonNull<T>);

unsafe impl<'a, T: Component> Fetch<'a> for ComponentRead<T> {
    type Item = &'a T;

    #[inline]
    fn acquire_borrow(archetype: &Archetype) {
        let guard = archetype
            .get_component_guard(ComponentTypeId::of::<T>())
            .unwrap();
        assert!(
            guard.borrow(),
            "Colliding shared access to component type '{}' in query",
            std::any::type_name::<T>()
        );
    }

    #[inline]
    fn release_borrow(archetype: &Archetype) {
        let guard = archetype
            .get_component_guard(ComponentTypeId::of::<T>())
            .unwrap();
        guard.release();
    }

    #[inline]
    fn create_at(archetype: &Archetype, entity: ArchetypeEntityIndex) -> Self {
        let ptr = archetype
            .get_component_ptr(entity, ComponentTypeId::of::<T>())
            .unwrap()
            .cast::<T>();
        Self(ptr)
    }

    #[inline]
    unsafe fn next(&mut self) {
        self.0 = NonNull::new_unchecked(self.0.as_ptr().add(1));
    }

    #[inline]
    unsafe fn get(&self) -> Self::Item {
        self.0.as_ref()
    }
}

impl<'a, T: Component> ComponentQuery for &'a mut T {
    type Fetch = ComponentWrite<T>;

    #[inline]
    fn add_to_layout(layout: &mut EntityLayoutBuf) {
        if layout.add_component_type(ComponentTypeId::of::<T>()) {
            panic!("Trying to lookup the same component multiple times within the same query");
        }
    }

    #[inline(always)]
    fn wants_any_mutable_access() -> bool {
        true
    }
}

/// Internal type that implements `Fetch` for mutable references
pub struct ComponentWrite<T>(NonNull<T>);

unsafe impl<'a, T: Component> Fetch<'a> for ComponentWrite<T> {
    type Item = &'a mut T;

    #[inline]
    fn acquire_borrow(archetype: &Archetype) {
        let guard = archetype
            .get_component_guard(ComponentTypeId::of::<T>())
            .unwrap();
        assert!(
            guard.borrow_mut(),
            "Colliding exclusive access to component type '{}' in query",
            std::any::type_name::<T>()
        );
    }

    #[inline]
    fn release_borrow(archetype: &Archetype) {
        let guard = archetype
            .get_component_guard(ComponentTypeId::of::<T>())
            .unwrap();
        guard.release_mut();
    }

    #[inline]
    fn create_at(archetype: &Archetype, entity: ArchetypeEntityIndex) -> Self {
        let ptr = archetype
            .get_component_ptr(entity, ComponentTypeId::of::<T>())
            .unwrap()
            .cast::<T>();
        Self(ptr)
    }

    #[inline(always)]
    unsafe fn next(&mut self) {
        self.0 = NonNull::new_unchecked(self.0.as_ptr().add(1));
    }

    #[inline(always)]
    unsafe fn get(&self) -> Self::Item {
        &mut *self.0.as_ptr()
    }
}

#[macro_export]
macro_rules! impl_query_for_tuple {
    ($($name: ident),*) => {
        #[allow(non_snake_case, unused_variables, clippy::unused_unit)]
        unsafe impl<'a, $($name: $crate::Fetch<'a>),*> $crate::Fetch<'a> for ($($name,)*) {
            type Item = ($($name::Item,)*);

            #[inline]
            fn acquire_borrow(archetype: &$crate::Archetype) {
                ($($name::acquire_borrow(archetype),)*);
            }

            #[inline]
            fn release_borrow(archetype: &$crate::Archetype) {
                ($($name::release_borrow(archetype),)*);
            }

            #[inline]
            fn create_at(archetype: &$crate::Archetype, entity: $crate::archetype::ArchetypeEntityIndex) -> Self {
                ($($name::create_at(archetype, entity),)*)
            }

            #[inline]
            unsafe fn next(&mut self) {
                let ($($name,)*) = self;
                ($($name.next(),)*);
            }

            #[inline]
            unsafe fn get(&self) -> Self::Item {

                let ($($name,)*) = self;
                ($($name.get(),)*)
            }
        }

        impl<$($name: $crate::ComponentQuery),*> $crate::ComponentQuery for ($($name,)*) {
            type Fetch = ($($name::Fetch,)*);

            #[inline]
            fn add_to_layout(layout: &mut $crate::EntityLayoutBuf) {
                ($($name::add_to_layout(layout),)*);
            }

            #[inline]
            fn wants_any_mutable_access() -> bool {
                $($name::wants_any_mutable_access()|)* false
            }
        }
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
