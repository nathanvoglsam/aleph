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

use crate::scheduler::AccessDescriptor;
use crate::world::World;
use std::any::Any;

///
/// The interface expected of a [`System`] object.
///
pub trait System: Any + Send + Sync + 'static {
    /// An arbitrary type that can be passed into [`System::execute`]. Will almost always be `()`
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

    /// This function will be called once by a scheduler to allow a system to load whatever
    /// resources are needed from the world. This allows the system to prepare itself for any
    /// subsequent calls to [`System::execute`].
    fn build(&mut self, world: &mut World);

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
    unsafe fn execute(&mut self, input: Self::In, world: &World) -> Self::Out;

    /// A wrapper around [`System::execute`] that allows calling execute safely by enforcing that
    /// `world` is accessed through an exclusive borrow.
    fn execute_safe(&mut self, input: Self::In, world: &mut World) -> Self::Out {
        // SAFETY: This is safe per the requirements of context 1 as documented on the execute
        //         function. See the documentation of System::execute for more info.
        unsafe { self.execute(input, world) }
    }
}