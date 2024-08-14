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

use crate::scheduler::AccessDescriptor;
use crate::world::{Archetype, ArchetypeEntityIndex, Component, ComponentTypeId, EntityLayoutBuf};

pub trait ComponentQuery: Send + Sync {
    type Fetch: for<'a> Fetch<'a>;

    fn add_to_layout(layout: &mut EntityLayoutBuf);

    fn declare_access(access: &mut dyn AccessDescriptor);
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

    /// Constructs an instance of [`Fetch`] from the given archetype.
    ///
    /// Takes a pointer because borrow could mutable or shared depending on the implementation.
    unsafe fn create(archetype: NonNull<Archetype>) -> Self;

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

    fn declare_access(access: &mut dyn AccessDescriptor) {
        access.reads_component::<T>();
    }
}

/// Internal type that implements `Fetch` for shared references
pub struct ComponentRead<T>(NonNull<T>);

unsafe impl<'a, T: Component> Fetch<'a> for ComponentRead<T> {
    type Item = &'a T;

    #[inline]
    unsafe fn create(archetype: NonNull<Archetype>) -> Self {
        let slot = NonZeroU32::new(1).unwrap();
        let slot = ArchetypeEntityIndex(slot);
        let ptr = archetype
            .as_ref()
            .get_component_ptr(slot, ComponentTypeId::of::<T>())
            .unwrap();
        let ptr = NonNull::new(ptr.as_ptr() as *mut T).unwrap();

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

    fn declare_access(access: &mut dyn AccessDescriptor) {
        access.writes_component::<T>();
    }
}

/// Internal type that implements `Fetch` for mutable references
pub struct ComponentWrite<T>(NonNull<T>);

unsafe impl<'a, T: Component> Fetch<'a> for ComponentWrite<T> {
    type Item = &'a mut T;

    #[inline]
    unsafe fn create(archetype: NonNull<Archetype>) -> Self {
        let slot = NonZeroU32::new(1).unwrap();
        let slot = ArchetypeEntityIndex(slot);
        let ptr = archetype
            .as_ref()
            .get_component_ptr(slot, ComponentTypeId::of::<T>())
            .unwrap();
        let ptr = NonNull::new(ptr.as_ptr() as *mut T).unwrap();
        Self(ptr)
    }

    #[inline]
    unsafe fn next(&mut self) {
        self.0 = NonNull::new_unchecked(self.0.as_ptr().add(1));
    }

    #[inline]
    unsafe fn get(&self) -> Self::Item {
        &mut *self.0.as_ptr()
    }
}

#[macro_export]
macro_rules! impl_query_for_tuple {
    ($($name: ident),*) => {
        unsafe impl<'a, $($name: $crate::world::Fetch<'a>),*> $crate::world::Fetch<'a> for ($($name,)*) {
            type Item = ($($name::Item,)*);

            #[inline]
            #[allow(unused_variables, clippy::unused_unit)]
            unsafe fn create(archetype: ::std::ptr::NonNull<$crate::world::Archetype>) -> Self {
                #[allow(non_snake_case)]
                ($($name::create(archetype),)*)
            }

            #[inline]
            #[allow(unused_variables, clippy::unused_unit)]
            unsafe fn next(&mut self) {
                #[allow(non_snake_case)]
                let ($($name,)*) = self;
                ($($name.next(),)*);
            }

            #[inline]
            #[allow(unused_variables, clippy::unused_unit)]
            unsafe fn get(&self) -> Self::Item {
                #[allow(non_snake_case)]
                let ($($name,)*) = self;
                ($($name.get(),)*)
            }
        }

        impl<$($name: $crate::world::ComponentQuery),*> $crate::world::ComponentQuery for ($($name,)*) {
            type Fetch = ($($name::Fetch,)*);

            #[inline]
            fn add_to_layout(layout: &mut $crate::world::EntityLayoutBuf) {
                ($($name::add_to_layout(layout),)*);
            }

            #[inline]
            fn declare_access(access: &mut dyn $crate::scheduler::AccessDescriptor) {
                ($($name::declare_access(access),)*);
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
