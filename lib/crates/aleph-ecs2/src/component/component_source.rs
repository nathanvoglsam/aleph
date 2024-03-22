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

use crate::ComponentTypeId;
use crate::EntityLayout;

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
    /// The entity layout that describes the set of components the entities we're trying to insert
    /// have.
    ///
    /// It is the responsibility of the [ComponentSource] implementation to provide a buffer of
    /// [ComponentSource::count] components of each type specified here on request via
    /// [ComponentSource::data_for].
    fn entity_layout(&self) -> &EntityLayout;

    /// Returns a type erased data buffer that is valid storage for [ComponentSource::count]
    /// components of the requested type. That means valid size and alignment.
    ///
    /// The objects will be copied into the destination buffer. This is logically a move operation
    /// and the source objects should _not_ be dropped by the [ComponentSource] implementation as
    /// the caller of [ComponentSource::data_for] has taken ownership of these objects.
    fn data_for(&self, component: ComponentTypeId) -> &[u8];

    /// Returns the number of *entities* that this component source is storing data for. This
    /// describes the number of entities that need to be inserted, and also describes the required
    /// size of the buffers returned by [ComponentSource::data_for] as
    /// `entity_count' * size_of_component_type`.
    fn count(&self) -> u32;
}

#[macro_export]
macro_rules! impl_component_source_for_tuple {
    ($($t: ident), *) => {
        #[allow(non_snake_case)]
        unsafe impl<$($t: $crate::Component),+> $crate::ComponentSource for (u32, $crate::EntityLayoutBuf, $(::std::vec::Vec<::std::mem::ManuallyDrop<$t>>,)+) {
            #[inline]
            fn entity_layout(&self) -> &$crate::EntityLayout {
                &self.1
            }

            #[inline(always)]
            fn data_for(&self, component: $crate::ComponentTypeId) -> &[u8] {
                let (_, _, $($t,)+) = self;
                $(
                    if component == $crate::ComponentTypeId::of::<$t>() {
                        let data = $t.as_ptr() as *const u8;
                        let len = $t.len() * ::std::mem::size_of::<$t>();
                        return unsafe {
                            ::std::slice::from_raw_parts(data, len)
                        };
                    }
                )+
                panic!()
            }

            #[inline(always)]
            fn count(&self) -> u32 {
                self.0
            }
        }

        #[allow(non_snake_case)]
        unsafe impl<$($t: $crate::Component),+ ,const SIZE: usize> $crate::ComponentSource for ($crate::EntityLayoutBuf, $([::std::mem::ManuallyDrop<$t>; SIZE],)+) {
            #[inline]
            fn entity_layout(&self) -> &$crate::EntityLayout {
                &self.0
            }

            #[inline(always)]
            fn data_for(&self, component: $crate::ComponentTypeId) -> &[u8] {
                let (_, $($t,)+) = self;
                $(
                    if component == $crate::ComponentTypeId::of::<$t>() {
                        let data = $t.as_ptr() as *const u8;
                        let len = $t.len() * ::std::mem::size_of::<$t>();
                        return unsafe {
                            ::std::slice::from_raw_parts(data, len)
                        };
                    }
                )+
                panic!()
            }

            #[inline(always)]
            fn count(&self) -> u32 {
                SIZE as u32
            }
        }
    }
}

#[macro_export]
macro_rules! impl_into_component_source_for_tuple {
    ($t0: ident, $($t: ident), *) => {
        #[allow(non_snake_case)]
        unsafe impl<$t0: $crate::Component, $($t: $crate::Component),+> $crate::IntoComponentSource for (::std::vec::Vec<$t0>, $(::std::vec::Vec<$t>,)+) {
            type Source = (u32, $crate::EntityLayoutBuf, ::std::vec::Vec<::std::mem::ManuallyDrop<$t0>>, $(::std::vec::Vec<::std::mem::ManuallyDrop<$t>>,)+);

            fn into_component_source(self) -> Self::Source {
                let (mut $t0, $(mut $t,)+) = self;

                let len = $t0.len();

                $(
                    assert_eq!(len, $t.len());
                    let len = $t.len();
                )+

                assert!(len < (u32::MAX - 1) as usize);
                let len = len as u32;

                let mut layout = $crate::EntityLayoutBuf::new();
                layout.add_component_type($crate::ComponentTypeId::of::<$t0>());
                $(
                    layout.add_component_type($crate::ComponentTypeId::of::<$t>());
                )+

                let $t0 = unsafe {
                    let ptr = $t0.as_mut_ptr() as *mut ::std::mem::ManuallyDrop<$t0>;
                    let length = $t0.len();
                    let capacity = $t0.capacity();
                    ::std::mem::forget($t0);
                    Vec::from_raw_parts(ptr, length, capacity)
                };

                $(
                    let $t = unsafe {
                        let ptr = $t.as_mut_ptr() as *mut ::std::mem::ManuallyDrop<$t>;
                        let length = $t.len();
                        let capacity = $t.capacity();
                        ::std::mem::forget($t);
                        ::std::vec::Vec::from_raw_parts(ptr, length, capacity)
                    };
                )+

                (len, layout, $t0, $($t,)+)
            }
        }

        #[allow(non_snake_case)]
        unsafe impl<$t0: $crate::Component, $($t: $crate::Component),+ , const SIZE: usize> $crate::IntoComponentSource for ([$t0; SIZE], $([$t; SIZE],)+) {
            type Source = ($crate::EntityLayoutBuf, [::std::mem::ManuallyDrop<$t0>; SIZE], $([::std::mem::ManuallyDrop<$t>; SIZE],)+);

            fn into_component_source(self) -> Self::Source {
                let ($t0, $($t,)+) = self;

                assert!(SIZE < (u32::MAX - 1) as usize);

                let mut layout = $crate::EntityLayoutBuf::new();
                layout.add_component_type($crate::ComponentTypeId::of::<$t0>());
                $(
                    layout.add_component_type($crate::ComponentTypeId::of::<$t>());
                )+

                let $t0 = unsafe {
                    let ptr = &$t0 as *const [$t0; SIZE] as *const [::std::mem::ManuallyDrop<$t0>; SIZE];
                    let value = ptr.read();
                    ::std::mem::forget($t0);
                    value
                };

                $(
                    let $t = unsafe {
                        let ptr = &$t as *const [$t; SIZE] as *const [::std::mem::ManuallyDrop<$t>; SIZE];
                        let value = ptr.read();
                        ::std::mem::forget($t);
                        value
                    };
                )+

                (layout, $t0, $($t,)+)
            }
        }
    };

    ($t0: ident) => {
        #[allow(non_snake_case)]
        unsafe impl<$t0: $crate::Component, > $crate::IntoComponentSource for (::std::vec::Vec<$t0>, ) {
            type Source = (u32, $crate::EntityLayoutBuf, ::std::vec::Vec<::std::mem::ManuallyDrop<$t0>>);

            fn into_component_source(self) -> Self::Source {
                let (mut $t0, ) = self;

                let len = $t0.len();

                assert!(len < (u32::MAX - 1) as usize);
                let len = len as u32;

                let mut layout = $crate::EntityLayoutBuf::new();
                layout.add_component_type($crate::ComponentTypeId::of::<$t0>());

                let $t0 = unsafe {
                    let ptr = $t0.as_mut_ptr() as *mut ::std::mem::ManuallyDrop<$t0>;
                    let length = $t0.len();
                    let capacity = $t0.capacity();
                    ::std::mem::forget($t0);
                    ::std::vec::Vec::from_raw_parts(ptr, length, capacity)
                };

                (len, layout, $t0)
            }
        }

        #[allow(non_snake_case)]
        unsafe impl<$t0: $crate::Component, const SIZE: usize> $crate::IntoComponentSource for ([$t0; SIZE], ) {
            type Source = ($crate::EntityLayoutBuf, [::std::mem::ManuallyDrop<$t0>; SIZE]);

            fn into_component_source(self) -> Self::Source {
                let ($t0, ) = self;

                assert!(SIZE < (u32::MAX - 1) as usize);

                let mut layout = $crate::EntityLayoutBuf::new();
                layout.add_component_type($crate::ComponentTypeId::of::<$t0>());

                let $t0 = unsafe {
                    let ptr = &$t0 as *const [$t0; SIZE] as *const [::std::mem::ManuallyDrop<$t0>; SIZE];
                    let value = ptr.read();
                    ::std::mem::forget($t0);
                    value
                };

                (layout, $t0)
            }
        }
    }
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
