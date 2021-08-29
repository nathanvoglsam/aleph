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

use crate::scheduler::{AccessDescriptor, Label, ResourceId, Stage};
use crate::system::System;
use crate::world::{ComponentTypeId, World};
use std::collections::{HashMap, HashSet};

#[derive(Default)]
pub struct SystemSchedule {
    /// Stores all systems in the schedule along with the label it was registered with
    systems: Vec<SystemBox>,

    /// Maps a label to the system it was registered with. Accelerates looking up a system by label
    /// as well as accelerating duplicate label checks.
    system_label_map: HashMap<Box<dyn Label>, usize>,

    /// A flag used to declare if the
    dirty: bool,
}

impl SystemSchedule {
    pub fn add_system<S: System<In = (), Out = ()>>(
        &mut self,
        label: impl Label,
        system: S,
    ) -> &mut Self {
        self.dirty = true;

        let label: Box<dyn Label> = Box::new(label);

        // Push the new system into the system list, capturing the index it will be inserted into
        let index = self.systems.len();
        self.systems.push(SystemBox::new(label.clone(), system));

        // Insert the label into the label->index map, checking if the label has already been
        // registered (triggers a panic)
        if self.system_label_map.insert(label.clone(), index).is_some() {
            panic!("System already exists: {:?}.", label);
        }
        self
    }

    pub fn run_once(&mut self, world: &mut World) {
        self.check_dirty(world);
    }
}

impl Stage for SystemSchedule {
    fn run(&mut self, world: &mut World) {
        self.run_once(world)
    }
}

impl SystemSchedule {
    /// Checks if the system set is marked as dirty. If so it will automatically rebuild the
    /// execution graph as it will now be out of date compared to the
    fn check_dirty(&mut self, world: &mut World) {
        if self.dirty {
            self.rebuild_graph(world);
        }
    }

    /// Handles rebuilding the execution graph
    fn rebuild_graph(&mut self, world: &mut World) {
        self.clear_graph_nodes();
        self.collect_access_descriptors();
        self.dirty = false;
    }

    /// Used for clearing all the edges from all the nodes prior to a graph rebuild
    fn clear_graph_nodes(&mut self) {
        self.systems.iter_mut().for_each(|v| {
            v.edges.predecessors.clear();
            v.edges.successors.clear();
        })
    }

    fn collect_access_descriptors(&mut self) {
        for i in 0..self.systems.len() {
            // First we call clear the access descriptor and re-populate it by calling
            // declare_access for each system
            {
                let v = &mut self.systems[i];
                v.access.clear();
                v.system.declare_access(&mut v.access);
            }

            // Next we write the explicit "runs before" execution dependencies into the graph
            let runs_before = std::mem::take(&mut self.systems[i].access.runs_before);
            for before in runs_before.iter() {
                // Get the index of the system that we wish to run before
                let before = self.system_label_map.get(before).copied().unwrap();

                // Mark ourselves as a predecessor to that system
                self.systems[before].edges.predecessors.insert(i);

                // Add the target system to our successor set
                self.systems[i].edges.successors.insert(before);
            }
            self.systems[i].access.runs_before = runs_before;

            // Next we write the explicit "runs after" execution dependencies into the graph
            let runs_after = std::mem::take(&mut self.systems[i].access.runs_after);
            for after in runs_after.iter() {
                // Get the index of the system that we wish to run after
                let after = self.system_label_map.get(after).copied().unwrap();

                // Mark ourselves as a successor to that system
                self.systems[after].edges.successors.insert(i);

                // Add the target system to our predecessor set
                self.systems[i].edges.predecessors.insert(after);
            }
            self.systems[i].access.runs_after = runs_after;
        }
    }
}

///
/// Internal container for pairing a boxed system with some metadata used to schedule the system
///
struct SystemBox {
    /// The label of the system
    label: Box<dyn Label>,

    /// The boxed system
    system: Box<dyn System<In = (), Out = ()>>,

    /// The accesses declared by the system
    access: SystemAccessDescriptor,

    /// The edges out of the system's node in the execution graph
    edges: GraphEdges,
}

impl SystemBox {
    pub fn new<S: System<In = (), Out = ()>>(label: Box<dyn Label>, system: S) -> Self {
        Self {
            label,
            system: Box::new(system),
            access: SystemAccessDescriptor::default(),
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
struct GraphEdges {
    /// A set of indices to the systems that precede the execution of `system`
    predecessors: HashSet<usize>,

    /// A set of indices to the systems that execute after `system`
    successors: HashSet<usize>,
}

///
/// Internal container for storing the sets of resource accesses of a system
///
#[derive(Default)]
struct SystemAccessDescriptor {
    /// Stores all component types that are read by a given system
    component_reads: HashSet<ComponentTypeId>,

    /// Stores all component types that are written by a given system
    component_writes: HashSet<ComponentTypeId>,

    /// Stores all resources that are read by a given system
    resource_reads: HashSet<ResourceId>,

    /// Stores all resources that are written by a given system
    resource_writes: HashSet<ResourceId>,

    /// Stores the labels of all systems that must run before the system this descriptor is for
    runs_before: HashSet<Box<dyn Label>>,

    /// Stores the labels of all systems that must run after the system this descriptor is for
    runs_after: HashSet<Box<dyn Label>>,
}

impl SystemAccessDescriptor {
    fn clear(&mut self) {
        self.component_reads.clear();
        self.component_writes.clear();
        self.resource_reads.clear();
        self.resource_writes.clear();
        self.runs_before.clear();
        self.runs_after.clear();
    }
}

impl SystemAccessDescriptor {
    fn is_access_disjoint(&self, other: &Self) -> bool {
        self.is_component_access_disjoint(other) && self.is_resource_access_disjoint(other)
    }

    fn is_component_access_disjoint(&self, other: &Self) -> bool {
        // Parallel access is only safe if self does not read any components other is writing
        let a = self.component_reads.is_disjoint(&other.component_writes);

        // Parallel access is only safe if self does not write any components other is writing
        let b = self.component_writes.is_disjoint(&other.component_writes);

        // Parallel access is only safe if self does not write any components other is reading
        let c = self.component_writes.is_disjoint(&other.component_reads);

        // It is safe for self.component_reads and other.component_reads to intersect

        // Parallel access is only safe if all the above conditions are met
        a && b && c
    }

    fn is_resource_access_disjoint(&self, other: &Self) -> bool {
        // Parallel access is only safe if self does not read any resources other is writing
        let a = self.resource_reads.is_disjoint(&other.resource_writes);

        // Parallel access is only safe if self does not write any resources other is writing
        let b = self.resource_writes.is_disjoint(&other.resource_writes);

        // Parallel access is only safe if self does not write any resources other is reading
        let c = self.resource_writes.is_disjoint(&other.resource_reads);

        // It is safe for self.resource_reads and other.resource_reads to intersect

        // Parallel access is only safe if all the above conditions are met
        a && b && c
    }
}

impl AccessDescriptor for SystemAccessDescriptor {
    fn reads_component_with_id(&mut self, component: ComponentTypeId) {
        self.component_reads.insert(component);
    }

    fn writes_component_with_id(&mut self, component: ComponentTypeId) {
        self.component_writes.insert(component);
    }

    fn reads_resource_with_id(&mut self, resource: ResourceId) {
        self.resource_reads.insert(resource);
    }

    fn writes_resource_with_id(&mut self, resource: ResourceId) {
        self.resource_writes.insert(resource);
    }

    fn runs_before_label(&mut self, system: Box<dyn Label>) {
        self.runs_before.insert(system);
    }

    fn runs_after_label(&mut self, system: Box<dyn Label>) {
        self.runs_after.insert(system);
    }
}
