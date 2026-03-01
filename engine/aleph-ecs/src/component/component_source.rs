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

use std::mem::{ManuallyDrop, MaybeUninit};
use std::ptr::NonNull;

use crate::component::{Component, ComponentId};
use crate::{EcsSystem, EntityId, EntityLayoutBuf};

#[repr(C)]
pub struct UnsafeComponentSource {
    /// The number of entities in this component source
    pub count: u32,

    /// An optional pointer to an array of 'count' uninitialized entity IDs that should be filled
    /// out if provided.
    pub ids: Option<NonNull<MaybeUninit<EntityId>>>,

    /// An array of 'id + ptr' pairs that identifies the set of components that the entities will
    /// be created with, as well as pointers to buffers that contain the component data to copy into
    /// the ECS.
    pub components: NonNull<[UnsafeComponentSourceEntry]>,
}

impl UnsafeComponentSource {
    pub unsafe fn fill_layout(&self, target: &mut EntityLayoutBuf<EcsSystem>) {
        unsafe {
            for component in self.components.as_ref().iter() {
                assert!(
                    !target.add_component_type(component.id),
                    "UnsafeComponentSource contains duplicate component channels!"
                );
            }
        }
    }
}

#[repr(C)]
pub struct UnsafeComponentSourceEntry {
    /// The 'id' of the component type that is stored in this pair.
    pub id: ComponentId,

    /// An array of the component type identified by `id`. It is the user's responsibility to ensure
    /// the pointer is valid when it is read.
    pub ptr: NonNull<u8>,
}

pub struct CheckedSource<T>(pub T);

/// Interface for converting one type into a type that implements `ComponentSource`.
///
/// # Safety
///
/// This trait is marked unsafe as `ComponentSource` is an unsafe trait as well. I have not spent
/// any thought into investigating how safe these operations actually are so they are marked as
/// unsafe pre-emptively until I can prove them as safe.
pub unsafe trait IntoComponentSource {
    type Source: ComponentSource;

    fn into_component_source(self) -> Self::Source;
}

unsafe impl<T: ComponentSource> IntoComponentSource for T {
    type Source = T;

    fn into_component_source(self) -> Self::Source {
        self
    }
}

/// Specialization of [`IntoComponentSource`] that only contains components for a single entity
pub unsafe trait IntoOneComponentSource {
    type Source: OneComponentSource;

    fn into_one_component_source(self) -> Self::Source;
}

unsafe impl<T: OneComponentSource> IntoOneComponentSource for T {
    type Source = T;

    fn into_one_component_source(self) -> Self::Source {
        self
    }
}

unsafe impl<T: OneComponentSource> ComponentSource for T {
    fn with_unsafe_source<Out>(&self, callback: impl FnOnce(UnsafeComponentSource) -> Out) -> Out {
        <T as OneComponentSource>::with_unsafe_source(self, callback)
    }
}

/// Interface expected of a type that is a source of component data for inserting entities into
/// an ECS world.
///
/// # Safety
///
/// This trait is marked as unsafe because any non-trivial implementation is going to use a lot of
/// unsafe code anyway. The entire interface is based around type-erasure and copying data of
/// objects without dropping.
///
/// I have not put time into proving how safe this interface is so I mark it as unsafe
/// pre-emptively. The implementations provided are safe, but the trait remains unsafe for now.
pub unsafe trait ComponentSource {
    fn with_unsafe_source<Out>(&self, callback: impl FnOnce(UnsafeComponentSource) -> Out) -> Out;
}

/// Specialization of [`ComponentSource`] that only contains components for a single entity
pub unsafe trait OneComponentSource {
    fn with_unsafe_source<Out>(&self, callback: impl FnOnce(UnsafeComponentSource) -> Out) -> Out;
}

macro_rules! impl_component_source_for_tuple {
    ($($t: ident), *) => {
        #[allow(non_snake_case)]
        unsafe impl<$($t: Component),+> ComponentSource for CheckedSource<($(::std::vec::Vec<::std::mem::ManuallyDrop<$t>>,)+)> {
            #[inline]
            fn with_unsafe_source<Out>(&self, callback: impl ::std::ops::FnOnce(UnsafeComponentSource) -> Out) -> Out {
                let ($($t,)+) = &self.0;

                let components = [
                    $(UnsafeComponentSourceEntry {
                        id: <$t as Component>::DESC.id,
                        ptr: ::std::ptr::NonNull::new($t.as_ptr() as *mut std::mem::MaybeUninit<$t>).unwrap().cast::<u8>(),
                    }),+
                ];
                let unsafe_source = UnsafeComponentSource{
                    count: self.0.0.len().try_into().unwrap(),
                    ids: ::std::option::Option::None,
                    components: ::std::ptr::NonNull::from(&components),
                };
                callback(unsafe_source)
            }
        }

        #[allow(non_snake_case)]
        unsafe impl<$($t: Component),+ ,const SIZE: usize> ComponentSource for CheckedSource::<($([::std::mem::ManuallyDrop<$t>; SIZE],)+)> {
            #[inline]
            fn with_unsafe_source<Out>(&self, callback: impl ::std::ops::FnOnce(UnsafeComponentSource) -> Out) -> Out {
                let ($($t,)+) = &self.0;

                let components = [
                    $(UnsafeComponentSourceEntry {
                        id: <$t as Component>::DESC.id,
                        ptr: ::std::ptr::NonNull::new($t.as_ptr() as *mut std::mem::MaybeUninit<$t>).unwrap().cast::<u8>(),
                    }),+
                ];
                let unsafe_source = UnsafeComponentSource{
                    count: SIZE as u32,
                    ids: ::std::option::Option::None,
                    components: ::std::ptr::NonNull::from(&components),
                };
                callback(unsafe_source)
            }
        }

        #[allow(non_snake_case)]
        unsafe impl<$($t: Component),+> OneComponentSource for CheckedSource<($(::std::mem::ManuallyDrop<$t>,)+)> {
            #[inline]
            fn with_unsafe_source<Out>(&self, callback: impl ::std::ops::FnOnce(UnsafeComponentSource) -> Out) -> Out {
                let ($($t,)+) = &self.0;

                let components = [
                    $(UnsafeComponentSourceEntry {
                        id: <$t as Component>::DESC.id,
                        ptr: ::std::ptr::NonNull::from($t).cast::<u8>(),
                    }),+
                ];
                let unsafe_source = UnsafeComponentSource{
                    count: 1,
                    ids: ::std::option::Option::None,
                    components: ::std::ptr::NonNull::from(&components),
                };
                callback(unsafe_source)
            }
        }
    }
}

macro_rules! impl_into_component_source_for_tuple {
    ($($t: ident), *) => {
        #[allow(non_snake_case)]
        unsafe impl<$($t: Component),+> IntoComponentSource for ($(::std::vec::Vec<$t>,)+) {
            type Source = CheckedSource::<($(::std::vec::Vec<::std::mem::ManuallyDrop<$t>>,)+)>;

            fn into_component_source(self) -> Self::Source {
                let len = self.0.len();
                assert!(len < (u32::MAX as usize - 1));
                assert!(len > 0);

                let ($(mut $t,)+) = self;

                // Check all array lengths are the same
                let lens = [
                    $($t.len(),)+
                ];
                assert!(lens.iter().all(|v| *v == len), "All component channels must be the same length!");

                $(
                    let $t = unsafe {
                        let ptr = $t.as_mut_ptr() as *mut ::std::mem::ManuallyDrop<$t>;
                        let length = $t.len();
                        let capacity = $t.capacity();
                        ::std::mem::forget($t);
                        ::std::vec::Vec::from_raw_parts(ptr, length, capacity)
                    };
                )+

                CheckedSource(($($t,)+))
            }
        }

        #[allow(non_snake_case)]
        unsafe impl<$($t: Component),+ , const SIZE: usize> IntoComponentSource for ($([$t; SIZE],)+) {
            type Source = CheckedSource::<($([::std::mem::ManuallyDrop<$t>; SIZE],)+)>;

            fn into_component_source(self) -> Self::Source {
                assert!(SIZE < (u32::MAX - 1) as usize);
                assert!(SIZE > 0);

                let ($($t,)+) = self;

                $(
                    let $t: [::std::mem::ManuallyDrop<$t>; SIZE] = $t.map(|v| ::std::mem::ManuallyDrop::new(v));
                )+

                CheckedSource(($($t,)+))
            }
        }

        #[allow(non_snake_case)]
        unsafe impl<$($t: Component),+> IntoOneComponentSource for ($($t,)+) {
            type Source = CheckedSource::<($(::std::mem::ManuallyDrop<$t>,)+)>;

            fn into_one_component_source(self) -> Self::Source {
                let ($($t,)+) = self;

                $(
                    let $t: ::std::mem::ManuallyDrop<$t> = ::std::mem::ManuallyDrop::new($t);
                )+

                CheckedSource(($($t,)+))
            }
        }
    };
}

impl_into_component_source_for_tuple!(A);
impl_into_component_source_for_tuple!(A, B);
impl_into_component_source_for_tuple!(A, B, C);
impl_into_component_source_for_tuple!(A, B, C, D);
impl_into_component_source_for_tuple!(A, B, C, D, E);
impl_into_component_source_for_tuple!(A, B, C, D, E, F);
impl_into_component_source_for_tuple!(A, B, C, D, E, F, G);
impl_into_component_source_for_tuple!(A, B, C, D, E, F, G, H);
impl_into_component_source_for_tuple!(A, B, C, D, E, F, G, H, I);
impl_into_component_source_for_tuple!(A, B, C, D, E, F, G, H, I, J);
impl_into_component_source_for_tuple!(A, B, C, D, E, F, G, H, I, J, K);
impl_into_component_source_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L);
impl_into_component_source_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M);

impl_component_source_for_tuple!(A);
impl_component_source_for_tuple!(A, B);
impl_component_source_for_tuple!(A, B, C);
impl_component_source_for_tuple!(A, B, C, D);
impl_component_source_for_tuple!(A, B, C, D, E);
impl_component_source_for_tuple!(A, B, C, D, E, F);
impl_component_source_for_tuple!(A, B, C, D, E, F, G);
impl_component_source_for_tuple!(A, B, C, D, E, F, G, H);
impl_component_source_for_tuple!(A, B, C, D, E, F, G, H, I);
impl_component_source_for_tuple!(A, B, C, D, E, F, G, H, I, J);
impl_component_source_for_tuple!(A, B, C, D, E, F, G, H, I, J, K);
impl_component_source_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L);
impl_component_source_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M);

unsafe impl<A: Component> IntoComponentSource for Vec<A> {
    type Source = CheckedSource<(Vec<ManuallyDrop<A>>,)>;
    fn into_component_source(mut self) -> Self::Source {
        assert!(self.len() < (u32::MAX - 1) as usize);

        let out = unsafe {
            let ptr = self.as_mut_ptr() as *mut ManuallyDrop<A>;
            let length = self.len();
            let capacity = self.capacity();
            std::mem::forget(self);
            Vec::from_raw_parts(ptr, length, capacity)
        };
        CheckedSource((out,))
    }
}

unsafe impl<A: Component, const SIZE: usize> IntoComponentSource for [A; SIZE] {
    type Source = CheckedSource<([ManuallyDrop<A>; SIZE],)>;
    fn into_component_source(self) -> Self::Source {
        assert!(SIZE < (u32::MAX - 1) as usize);

        let out: [ManuallyDrop<A>; SIZE] = self.map(|v| ManuallyDrop::new(v));
        CheckedSource((out,))
    }
}
