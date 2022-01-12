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

use crate::scheduler::system_schedule::access_descriptor::SystemAccessDescriptor;
use crate::system::System;
use crate::world::World;
use crossbeam::atomic::AtomicCell;
use std::cell::Cell;

/// Type alias for a boxed system trait object
pub type BoxedSystem = Box<dyn System<In = (), Out = ()> + Send + Sync>;

/// Type alias for a boxed system trait object
pub type BoxedExclusiveSystem = Box<dyn System<In = (), Out = ()>>;

/// Type alias for the thread safe slot a system is stored in. The type is very verbose to write.
///
/// We need to double box the system to make sizeof(Option<Box<BoxedSystem>>) == 8 so atomics can
/// be used. Otherwise global locks would be used to send the systems and that's bad
pub type SystemCell = AtomicCell<Option<Box<BoxedSystem>>>;

/// Type alias for the thread safe slot a system is stored in. The type is very verbose to write.
///
/// We need to double box the system to make sizeof(Option<Box<BoxedSystem>>) == 8 so atomics can
/// be used. Otherwise global locks would be used to send the systems and that's bad
pub type ExclusiveSystemCell = Cell<Option<Box<BoxedExclusiveSystem>>>;

/// Generic wrapper for allowing to schedule different system streams with the same implementations
pub trait GenericSystemCell {
    fn declare_access(&self, access: &mut SystemAccessDescriptor);

    unsafe fn execute(&self, world: &World);

    fn execute_safe(&self, world: &mut World);
}

impl GenericSystemCell for SystemCell {
    fn declare_access(&self, access: &mut SystemAccessDescriptor) {
        let mut system = self.take().unwrap();
        system.declare_access(access);
        self.store(Some(system));
    }

    unsafe fn execute(&self, world: &World) {
        let mut system = self.take().unwrap();
        system.execute((), world);
        self.store(Some(system));
    }

    fn execute_safe(&self, world: &mut World) {
        let mut system = self.take().unwrap();
        system.execute_safe((), world);
        self.store(Some(system));
    }
}

impl GenericSystemCell for ExclusiveSystemCell {
    fn declare_access(&self, access: &mut SystemAccessDescriptor) {
        let mut system = self.take().unwrap();
        system.declare_access(access);
        self.set(Some(system))
    }

    unsafe fn execute(&self, world: &World) {
        let mut system = self.take().unwrap();
        system.execute((), world);
        self.set(Some(system));
    }

    fn execute_safe(&self, world: &mut World) {
        let mut system = self.take().unwrap();
        system.execute_safe((), world);
        self.set(Some(system));
    }
}
