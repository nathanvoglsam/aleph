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
use crate::scheduler::system_schedule::system_cell::{ExclusiveSystemCell, SystemCell};
use crate::system::System;
use aleph_label::Label;
use std::collections::HashSet;

///
/// Internal container for pairing a boxed system with some metadata used to schedule the system
///
pub struct SystemBox<T> {
    /// The boxed system, stored in an atomic cell so it can be sent to other threads
    pub system: T,

    /// The accesses declared by the system
    pub access: SystemAccessDescriptor,

    /// The edges out of the system's node in the execution graph
    pub edges: GraphEdges,
}

impl SystemBox<SystemCell> {
    pub fn new<S: System<In = (), Out = ()> + Send + Sync>(
        label: Box<dyn Label>,
        system: S,
    ) -> Self {
        assert!(SystemCell::is_lock_free());
        Self {
            system: SystemCell::new(Some(Box::new(Box::new(system)))),
            access: SystemAccessDescriptor::new(label),
            edges: GraphEdges::default(),
        }
    }
}

impl SystemBox<ExclusiveSystemCell> {
    pub fn new_exclusive<S: System<In = (), Out = ()>>(label: Box<dyn Label>, system: S) -> Self {
        Self {
            system: ExclusiveSystemCell::new(Some(Box::new(Box::new(system)))),
            access: SystemAccessDescriptor::new(label),
            edges: GraphEdges::default(),
        }
    }
}
///
/// Internal container for the edges of execution dependency graph.
///
/// The graph will be constructed to respect parallel access as well as pure execution dependencies
///
#[derive(Default)]
pub struct GraphEdges {
    /// A set of indices to the systems that precede the execution of `system`
    pub predecessors: HashSet<usize>,

    /// A set of indices to the systems that execute after `system`
    pub successors: HashSet<usize>,
}
