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

use std::collections::HashSet;

use aleph_label::Label;

use crate::scheduler::AccessDescriptor;
use crate::world::{ComponentTypeId, ResourceId};

///
/// Internal container for storing the sets of resource accesses of a system
///
pub struct SystemAccessDescriptor {
    /// The label of the system we're collecting access for currently
    pub label: Box<dyn Label>,

    /// Stores all component types that are read by a given system
    pub component_reads: HashSet<ComponentTypeId>,

    /// Stores all component types that are written by a given system
    pub component_writes: HashSet<ComponentTypeId>,

    /// Stores all resources that are read by a given system
    pub resource_reads: HashSet<ResourceId>,

    /// Stores all resources that are written by a given system
    pub resource_writes: HashSet<ResourceId>,

    /// Stores the labels of all systems that must run before the system this descriptor is for
    pub runs_before: HashSet<Box<dyn Label>>,

    /// Stores the labels of all systems that must run after the system this descriptor is for
    pub runs_after: HashSet<Box<dyn Label>>,
}

impl SystemAccessDescriptor {
    pub fn new(label: Box<dyn Label>) -> Self {
        Self {
            label,
            component_reads: Default::default(),
            component_writes: Default::default(),
            resource_reads: Default::default(),
            resource_writes: Default::default(),
            runs_before: Default::default(),
            runs_after: Default::default(),
        }
    }

    pub fn clear(&mut self) {
        self.component_reads.clear();
        self.component_writes.clear();
        self.resource_reads.clear();
        self.resource_writes.clear();
        self.runs_before.clear();
        self.runs_after.clear();
    }
}

impl AccessDescriptor for SystemAccessDescriptor {
    fn reads_component_with_id(&mut self, component: ComponentTypeId) {
        assert!(
            !self.component_writes.contains(&component),
            "System \"{:#?}\" wants shared for component \"{:?}\" that is already being used",
            self.label,
            component
        );
        assert!(
            self.component_reads.insert(component),
            "System \"{:#?}\" requested shared access for component \"{:?}\" more than once",
            self.label,
            component
        );
    }

    fn writes_component_with_id(&mut self, component: ComponentTypeId) {
        assert!(
            !self.component_reads.contains(&component),
            "System \"{:#?}\" wants exclusive for component \"{:?}\" that is already being used",
            self.label,
            component
        );
        assert!(
            self.component_writes.insert(component),
            "System \"{:#?}\" requested exclusive access for component \"{:?}\" more than once",
            self.label,
            component
        );
    }

    fn reads_resource_with_id(&mut self, resource: ResourceId) {
        assert!(
            !self.resource_writes.contains(&resource),
            "System \"{:#?}\" wants shared for resource \"{:?}\" that is already being used",
            self.label,
            resource
        );
        assert!(
            self.resource_reads.insert(resource),
            "System \"{:#?}\" requested shared access for resource \"{:?}\" more than once",
            self.label,
            resource
        );
    }

    fn writes_resource_with_id(&mut self, resource: ResourceId) {
        assert!(
            !self.resource_reads.contains(&resource),
            "System \"{:#?}\" wants exclusive for resource \"{:?}\" that is already being used",
            self.label,
            resource
        );
        assert!(
            self.resource_writes.insert(resource),
            "System \"{:#?}\" requested exclusive access for resource \"{:?}\" more than once",
            self.label,
            resource
        );
    }

    fn runs_before_label(&mut self, system: Box<dyn Label>) {
        self.runs_before.insert(system);
    }

    fn runs_after_label(&mut self, system: Box<dyn Label>) {
        self.runs_after.insert(system);
    }
}
