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

// Much of the implementation of this code is either copied or heavily based on code from the Bevy
// project. Available here: https://github.com/bevyengine/bevy and https://bevyengine.org/
//
// To respect the license terms I provide the license here.
//
// MIT License
//
// Copyright (c) 2020 bevyengine.org
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

//!
//! To be as honest as possible, I have no bloody idea how this works. I know just enough that I
//! could monkey wrench it out of [bevy_ecs](https://github.com/bevyengine/bevy) and adapt it to
//! work with my own code.
//!
//! This big web of traits, trait impls and macros allows users to define functions with a set of
//! parameters and use them as [`System`]s.
//!
//!
//!

use crate::scheduler::{AccessDescriptor, Resource};
use crate::system::{IntoSystem, System};
use crate::world::{ComponentQuery, Query, World};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

// ============================================================================================== //

pub trait SystemParam: Sized {
    type Fetch: for<'a> SystemParamFetch<'a>;
}

// ============================================================================================== //

pub unsafe trait SystemParamState: Send + Sync + 'static {
    fn init(access: &mut dyn AccessDescriptor) -> Self;
}

// ============================================================================================== //

pub trait SystemParamFetch<'a>: SystemParamState {
    type Item;

    unsafe fn get_param(state: &'a mut Self, world: &'a World) -> Self::Item;
}

// ============================================================================================== //

/// This marker/wrapper type provides the [`SystemParam`] implementation when a system wishes to get
/// shared access the [`Resource`] with type `T`.
pub struct Res<'w, T> {
    value: &'w T,
}

impl<'w, T: Resource> Deref for Res<'w, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.value
    }
}

/// An internal type that handles loading the pointer [`Res`] will provide to a system function.
pub struct ResState<T>(PhantomData<T>);

impl<'a, T: Resource> SystemParam for Res<'a, T> {
    type Fetch = ResState<T>;
}

unsafe impl<T: Resource> SystemParamState for ResState<T> {
    #[inline]
    fn init(access: &mut dyn AccessDescriptor) -> Self {
        access.reads_resource::<T>();
        Self(Default::default())
    }
}

impl<'a, T: Resource> SystemParamFetch<'a> for ResState<T> {
    type Item = Res<'a, T>;

    #[inline]
    unsafe fn get_param(_state: &'a mut Self, world: &'a World) -> Self::Item {
        Res {
            value: world.get_resource_ref_unchecked::<T>().unwrap(),
        }
    }
}

// ============================================================================================== //

/// This marker/wrapper type provides the [`SystemParam`] implementation when a system wishes to get
/// exclusive access the [`Resource`] with type `T`.
pub struct ResMut<'w, T> {
    value: &'w mut T,
}

impl<'w, T: Resource> Deref for ResMut<'w, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.value
    }
}

impl<'w, T: Resource> DerefMut for ResMut<'w, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.value
    }
}

/// An internal type that handles loading the pointer [`ResMut`] will provide to a system function.
pub struct ResMutState<T>(PhantomData<T>);

impl<'a, T: Resource> SystemParam for ResMut<'a, T> {
    type Fetch = ResMutState<T>;
}

unsafe impl<T: Resource> SystemParamState for ResMutState<T> {
    #[inline]
    fn init(access: &mut dyn AccessDescriptor) -> Self {
        access.reads_resource::<T>();
        Self(Default::default())
    }
}

impl<'a, T: Resource> SystemParamFetch<'a> for ResMutState<T> {
    type Item = ResMut<'a, T>;

    #[inline]
    unsafe fn get_param(_state: &'a mut Self, world: &'a World) -> Self::Item {
        ResMut {
            value: world.get_resource_mut_unchecked::<T>().unwrap(),
        }
    }
}

// ============================================================================================== //

/// An internal type that handles creating the [`Query`] object that is provided to a system
/// function when it is executed.
pub struct QueryState<Q: ComponentQuery + 'static>(PhantomData<Q>);

impl<'w, Q: ComponentQuery + 'static> SystemParam for Query<'w, Q> {
    type Fetch = QueryState<Q>;
}

unsafe impl<Q: ComponentQuery + 'static> SystemParamState for QueryState<Q> {
    #[inline]
    fn init(access: &mut dyn AccessDescriptor) -> Self {
        Q::declare_access(access);
        Self(Default::default())
    }
}

impl<'w, Q: ComponentQuery + 'static> SystemParamFetch<'w> for QueryState<Q> {
    type Item = Query<'w, Q>;

    #[inline]
    unsafe fn get_param(_state: &'w mut Self, world: &'w World) -> Self::Item {
        let world = &mut *(world as *const World as *mut World);
        Query::new(world)
    }
}

// ============================================================================================== //

/// This is the interface expected of a function that can be placed into a [`FunctionSystem`] and
/// called as the body of that system's execution.
///
/// This is essentially just a wrapper around `FnMut(Params)` that allows setting up the parameters
/// for the underlying function before calling said function.
pub trait SystemParamFunction<Param: SystemParam>: Send + Sync + 'static {
    unsafe fn run(&mut self, state: &mut Param::Fetch, world: &World);
}

// ============================================================================================== //

/// This type wraps another type that implements [`SystemParamFunction`].
///
/// This type provides an implementation of [`System`] for free functions and closures.
pub struct FunctionSystem<Param: SystemParam, F: SystemParamFunction<Param>> {
    f: F,
    state: Option<Param::Fetch>,
}

impl<Param: SystemParam + 'static, F: SystemParamFunction<Param>> System
    for FunctionSystem<Param, F>
{
    type In = ();
    type Out = ();

    #[inline]
    fn declare_access(&mut self, access: &mut dyn AccessDescriptor) {
        self.state = Some(Param::Fetch::init(access));
    }

    #[inline]
    unsafe fn execute(&mut self, _input: Self::In, world: &World) -> Self::Out {
        self.f.run(self.state.as_mut().unwrap(), world)
    }
}

impl<Param: SystemParam + 'static, F: SystemParamFunction<Param>> IntoSystem<(), (), Param> for F {
    type System = FunctionSystem<Param, F>;

    #[inline]
    fn system(self) -> FunctionSystem<Param, F> {
        FunctionSystem {
            f: self,
            state: None,
        }
    }
}

// ============================================================================================== //

#[macro_export]
macro_rules! impl_system_function {
    ($($param: ident),*) => {
        #[allow(non_snake_case)]
        impl<Func: ::std::marker::Send + ::std::marker::Sync + 'static, $($param: $crate::system::SystemParam),*> $crate::system::SystemParamFunction<($($param,)*)> for Func
        where
        for <'a> &'a mut Func:
                FnMut($($param),*) +
                FnMut($(<<$param as $crate::system::SystemParam>::Fetch as $crate::system::SystemParamFetch>::Item),*),
        {
            #[inline]
            unsafe fn run(&mut self, state: &mut <($($param,)*) as $crate::system::SystemParam>::Fetch, world: &$crate::world::World) {
                // Yes, this is strange, but rustc fails to compile this impl
                // without using this function.
                #[allow(clippy::too_many_arguments)]
                #[inline]
                fn call_inner<$($param,)*>(
                    mut f: impl FnMut($($param,)*),
                    $($param: $param,)*
                ) {
                    f($($param,)*)
                }
                let ($($param,)*) = <<($($param,)*) as $crate::system::SystemParam>::Fetch as $crate::system::SystemParamFetch>::get_param(state, world);
                call_inner(self, $($param),*)
            }
        }
    };
}

impl_system_function!(A);
impl_system_function!(A, B);
impl_system_function!(A, B, C);
impl_system_function!(A, B, C, D);
impl_system_function!(A, B, C, D, E);
impl_system_function!(A, B, C, D, E, F);
impl_system_function!(A, B, C, D, E, F, G);
impl_system_function!(A, B, C, D, E, F, G, H);
impl_system_function!(A, B, C, D, E, F, G, H, I);
impl_system_function!(A, B, C, D, E, F, G, H, I, J);
impl_system_function!(A, B, C, D, E, F, G, H, I, J, K);
impl_system_function!(A, B, C, D, E, F, G, H, I, J, K, L);
impl_system_function!(A, B, C, D, E, F, G, H, I, J, K, L, M);

// ============================================================================================== //

#[macro_export]
macro_rules! impl_param_for_tuple {
    ($($name: ident),*) => {
        impl<$($name: $crate::system::SystemParam),*> $crate::system::SystemParam for ($($name,)*) {
            type Fetch = ($($name::Fetch,)*);
        }

        #[allow(unused_variables)]
        #[allow(non_snake_case)]
        impl<'a, $($name: $crate::system::SystemParamFetch<'a>),*> $crate::system::SystemParamFetch<'a> for ($($name,)*) {
            type Item = ($($name::Item,)*);

            #[inline]
            unsafe fn get_param(state: &'a mut Self, world: &'a $crate::world::World) -> Self::Item {
                let ($($name,)*) = state;
                ($($name::get_param($name, world),)*)
            }
        }

        /// SAFE: implementors of each SystemParamState in the tuple have validated their impls
        #[allow(non_snake_case)]
        unsafe impl<$($name: $crate::system::SystemParamState),*> $crate::system::SystemParamState for ($($name,)*) {
            #[inline]
            fn init(access: &mut dyn $crate::scheduler::AccessDescriptor) -> Self {
                (($($name::init(access),)*))
            }
        }
    };
}

impl_param_for_tuple!(A);
impl_param_for_tuple!(A, B);
impl_param_for_tuple!(A, B, C);
impl_param_for_tuple!(A, B, C, D);
impl_param_for_tuple!(A, B, C, D, E);
impl_param_for_tuple!(A, B, C, D, E, F);
impl_param_for_tuple!(A, B, C, D, E, F, G);
impl_param_for_tuple!(A, B, C, D, E, F, G, H);
impl_param_for_tuple!(A, B, C, D, E, F, G, H, I);
impl_param_for_tuple!(A, B, C, D, E, F, G, H, I, J);
impl_param_for_tuple!(A, B, C, D, E, F, G, H, I, J, K);
impl_param_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L);
impl_param_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M);
