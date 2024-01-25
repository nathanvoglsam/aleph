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

mod system_access;

use std::any::Any;

use aleph_label::Label;
pub use system_access::{
    QueryState, Res, ResMut, ResMutState, ResState, SystemParam, SystemParamFetch,
    SystemParamFunction, SystemParamState,
};

use crate::scheduler::AccessDescriptor;
use crate::world::World;

// ============================================================================================== //

///
/// The interface expected of a [`System`] object.
///
pub trait System: Any + 'static {
    /// An arbitrary type that can be passed into [`System::execute`].
    type In;

    /// The return type of the [`System::execute`] function.
    type Out;

    /// Will be called by a scheduler once to retrieve the set of components and resources the
    /// system accesses. It **will** be called before [`System::execute`].
    ///
    /// When the function is called a [`AccessDescriptor`] is passed in. The implementation must
    /// use the [`AccessDescriptor`] interface to declare the components and resources the system
    /// want's to access.
    ///
    /// The [`AccessDescriptor`] is used to by a scheduler to order system execution. A scheduler
    /// *may* execute systems in parallel if their [`AccessDescriptor`] do not intersect with the
    /// exclusive accesses of other systems.
    fn declare_access(&mut self, access: &mut dyn AccessDescriptor);

    // /// This function will be called once by a scheduler before any call to [`System::execute`] to
    // /// allow a system to load whatever resources are needed from the world. This allows the system
    // /// to prepare itself for subsequent calls to [`System::execute`].
    // ///
    // /// # Safety
    // ///
    // /// This function is safe to call as it can not trigger UB on its own. Care must still be taken
    // /// to use a [`System`] correctly. The [`World`] **must not** be mutated between a call to
    // /// [`System::build`] and [`System::execute`].
    // ///
    // /// Implementations of [`System::execute`] are free to make this assumption, and **WILL** make
    // /// this assumption. Some implementations may cache pointers into [`World`] that could be made
    // /// dangling if the [`World`] is mutated and internal structures need to reallocate.
    // ///
    // /// [`System::build`] also **must not** mutate the [`World`] either.
    // fn build(&mut self, world: &mut World);

    /// This function will be called by a scheduler during the scheduler's execution cycle at a
    /// point the scheduler decides. This function is where the system's actual code should go.
    ///
    /// This function can return a value of type [`System::Out`]. This will typically be `()`, but
    /// in some cases (i.e. run criteria) a return value is useful.
    ///
    /// # Safety
    ///
    /// This might access world and resources in an unsafe manner. This should only be called in one
    /// of the following contexts:
    ///     1. This system is the only system running on the given world across all threads.
    ///     2. This system only runs in parallel with other systems that do not conflict with the
    ///        [`AccessDescriptor`]. It is the job of a scheduler to ensure at runtime that the
    ///        aliasing guarantees are upheld.
    ///
    /// It is an error for [`System::execute`] to access data in any way that does not match what
    /// was declared with [`System::declare_access`]. Doing so will almost certainly cause undefined
    /// behavior.
    unsafe fn execute(&mut self, input: Self::In, world: &World) -> Self::Out;

    /// A wrapper around [`System::execute`] that allows calling execute safely by enforcing that
    /// `world` is accessed through an exclusive borrow.
    fn execute_safe(&mut self, input: Self::In, world: &mut World) -> Self::Out {
        // SAFETY: This is safe per the requirements of context 1 as documented on the execute
        //         function. See the documentation of System::execute for more info.
        unsafe { self.execute(input, world) }
    }
}

// ============================================================================================== //

/// Generic trait that handles transforming one type into another that implements `System`
pub trait IntoSystem<In, Out, Params> {
    type System: System<In = In, Out = Out>;

    fn system(self) -> Self::System;
}

// ============================================================================================== //

pub struct AlreadyWasSystem;

// Systems implicitly implement IntoSystem
impl<In, Out, Sys: System<In = In, Out = Out>> IntoSystem<In, Out, AlreadyWasSystem> for Sys {
    type System = Sys;

    #[inline]
    fn system(self) -> Sys {
        self
    }
}

// ============================================================================================== //

/// This trait provides a generic interface for wrapping systems in one of the [`RunsBeforeSystem`]
/// or [`RunsAfterSystem`] wrappers.
///
/// This allows for providing explicit system ordering requirements by specifying the label of a
/// system that **must** run before or after another.
///
/// This is useful where the results of one system are required by another system but the order
/// they would otherwise be scheduled in is ambiguous due to non intersecting access masks.
pub trait ExplicitDependencies {
    type OutSystem: System;

    /// This adds an explicit dependency where the system `self` will finish executing before
    /// another system denoted by the [`Label`] `l` can start executing.
    fn runs_before<L: Label>(self, l: L) -> RunsBeforeSystem<Self::OutSystem, L>;

    /// This adds an explicit dependency where the system `self` will only begin executing after
    /// another system denoted by the [`Label`] `l` has finished executing.
    fn runs_after<L: Label>(self, l: L) -> RunsAfterSystem<Self::OutSystem, L>;
}

impl<S: System + Sized> ExplicitDependencies for S {
    type OutSystem = S;

    #[inline]
    fn runs_before<L: Label>(self, l: L) -> RunsBeforeSystem<S, L> {
        RunsBeforeSystem { s: self, l }
    }

    #[inline]
    fn runs_after<L: Label>(self, l: L) -> RunsAfterSystem<S, L> {
        RunsAfterSystem { s: self, l }
    }
}

// ============================================================================================== //

/// A wrapper for some other [`System`] implementation that injects an explicit "runs before"
/// dependency into [`System::declare_access`].
pub struct RunsBeforeSystem<S: System + Sized, L: Label> {
    s: S,
    l: L,
}

impl<S: System, L: Label> System for RunsBeforeSystem<S, L> {
    type In = S::In;
    type Out = S::Out;

    #[inline]
    fn declare_access(&mut self, access: &mut dyn AccessDescriptor) {
        access.runs_before_label(self.l.dyn_clone());
        self.s.declare_access(access);
    }

    #[inline]
    unsafe fn execute(&mut self, input: Self::In, world: &World) -> Self::Out {
        self.s.execute(input, world)
    }
}

// ============================================================================================== //

/// A wrapper for some other [`System`] implementation that injects an explicit "runs after"
/// dependency into [`System::declare_access`].
pub struct RunsAfterSystem<S: System + Sized, L: Label> {
    s: S,
    l: L,
}

impl<S: System, L: Label> System for RunsAfterSystem<S, L> {
    type In = S::In;
    type Out = S::Out;

    #[inline]
    fn declare_access(&mut self, access: &mut dyn AccessDescriptor) {
        access.runs_after_label(self.l.dyn_clone());
        self.s.declare_access(access);
    }

    #[inline]
    unsafe fn execute(&mut self, input: Self::In, world: &World) -> Self::Out {
        self.s.execute(input, world)
    }
}

// ============================================================================================== //
