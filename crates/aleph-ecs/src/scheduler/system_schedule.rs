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

use crate::scheduler::{AccessDescriptor, Stage};
use crate::system::{IntoSystem, System};
use crate::world::{ComponentTypeId, ResourceId, World};
use aleph_label::Label;
use crossbeam::atomic::AtomicCell;
use crossbeam::sync::WaitGroup;
use rayon::prelude::*;
use std::cell::Cell;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Default)]
pub struct SystemSchedule {
    /// Stores all exclusive systems in the schedule
    exclusive_systems: Vec<SystemBox<ExclusiveSystemCell>>,

    /// Maps a label to the system it was registered with. Accelerates looking up a system by label
    /// as well as accelerating duplicate label checks.
    exclusive_system_label_map: HashMap<Box<dyn Label>, usize>,

    /// This caches the list of root tasks where execution should start from
    exclusive_root_systems: Vec<usize>,

    /// Stores all systems in the schedule
    systems: Vec<SystemBox<SystemCell>>,

    /// Maps a label to the system it was registered with. Accelerates looking up a system by label
    /// as well as accelerating duplicate label checks.
    system_label_map: HashMap<Box<dyn Label>, usize>,

    /// This caches the list of root tasks where execution should start from
    root_systems: Vec<usize>,

    /// A flag used to declare if the
    dirty: bool,
}

impl SystemSchedule {
    pub fn add_exclusive_at_start_system<
        Param,
        T: System<In = (), Out = ()>,
        S: IntoSystem<(), (), Param, System = T>,
    >(
        &mut self,
        label: impl Label,
        system: S,
    ) -> &mut Self {
        self.dirty = true;

        let label: Box<dyn Label> = Box::new(label);

        // Push the new system into the system list, capturing the index it will be inserted into
        let index = self.systems.len();
        self.exclusive_systems
            .push(SystemBox::<ExclusiveSystemCell>::new_exclusive(
                label.clone(),
                system.system(),
            ));

        // Insert the label into the label->index map, checking if the label has already been
        // registered (triggers a panic)
        if self
            .exclusive_system_label_map
            .insert(label.clone(), index)
            .is_some()
        {
            panic!("System already exists: {:?}.", label);
        }
        self
    }

    pub fn add_system<
        Param,
        T: System<In = (), Out = ()> + Send + Sync,
        S: IntoSystem<(), (), Param, System = T>,
    >(
        &mut self,
        label: impl Label,
        system: S,
    ) -> &mut Self {
        self.dirty = true;

        let label: Box<dyn Label> = Box::new(label);

        // Push the new system into the system list, capturing the index it will be inserted into
        let index = self.systems.len();
        self.systems
            .push(SystemBox::new(label.clone(), system.system()));

        // Insert the label into the label->index map, checking if the label has already been
        // registered (triggers a panic)
        if self.system_label_map.insert(label.clone(), index).is_some() {
            panic!("System already exists: {:?}.", label);
        }
        self
    }

    pub fn run_once(&mut self, world: &mut World) {
        self.check_dirty();
        self.execute_exclusive_at_start_systems(world);
        self.execute_parallel_systems(world);
    }
}

impl Stage for SystemSchedule {
    fn run(&mut self, world: &mut World) {
        self.run_once(world)
    }
}

impl SystemSchedule {
    fn execute_exclusive_at_start_systems(&mut self, world: &mut World) {
        /// Struct that holds data that needs ownership transferred to the thread that executes the
        /// matching system
        struct WorkerPayload {
            wg: WaitGroup,
        }

        /// Alias for the container a payload is sent to other threads in
        ///
        /// A Box is used to ensure the time in the AtomicCell is pointer sized so it can be sent
        /// using atomic instructions instead of locks
        type PayloadCell = AtomicCell<Option<Box<WorkerPayload>>>;

        // Treat a non lock free implementation as an error
        assert!(PayloadCell::is_lock_free());

        // Root wait group that forces the function to wait for all systems to complete for exiting
        let wg = WaitGroup::new();

        // SoA list of flags that denote whether the matching task has completed, indexed in
        // parallel with self.systems
        let done: Vec<AtomicBool> = (0..self.exclusive_systems.len())
            .into_iter()
            .map(|_| AtomicBool::new(false))
            .collect();

        // SoA list of worker payloads, indexed in parallel with self.systems
        let payloads: Vec<PayloadCell> = (0..self.exclusive_systems.len())
            .into_iter()
            .map(|_| {
                let payload = WorkerPayload { wg: wg.clone() };
                AtomicCell::new(Some(Box::new(payload)))
            })
            .collect();

        // This handles executing a system, then recursively executing the successive tasks
        fn exec_task(
            systems: &[SystemBox<ExclusiveSystemCell>],
            done: &[AtomicBool],
            payloads: &[PayloadCell],
            world: &World,
            system_index: usize,
        ) {
            // Unpack the payload
            let payload = if let Some(payload) = payloads[system_index].take() {
                payload
            } else {
                return;
            };

            // Unpack the wait group to explicitly drop it to "use" it
            let wg = payload.wg;

            // Pull the system from the cell, execute it, then return it to the cell
            let mut system = systems[system_index].system.take().unwrap();
            unsafe {
                // SAFETY: This is unsafe to call in the event of unsafe implementations of System
                //         that do not access world according to their access flags. If a System
                //         does correctly respect its access declarations then the work scheduler
                //         ensures that aliasing requirements will be upheld, making this safe to
                //         call. This is only unsafe in the presence of other unsafe code.
                system.execute((), world);
            }
            systems[system_index].system.set(Some(system));

            // Update the "done" flag now that the system has executed
            done[system_index].store(true, Ordering::Relaxed);

            // Spawn new tasks for each successor system and execute it, if all of its predecessors
            // have completed.
            systems[system_index]
                .edges
                .successors
                .iter()
                .copied()
                .for_each(|successor| {
                    let successor: usize = successor;
                    if systems[successor]
                        .edges
                        .predecessors
                        .iter()
                        .copied()
                        .all(|predecessor| done[predecessor].load(Ordering::Relaxed))
                    {
                        exec_task(systems, done, payloads, world, successor);
                    }
                });

            // Explicitly drop the wait group to "use" it according to the compiler.
            drop(wg);
        }

        let systems = std::mem::take(&mut self.exclusive_systems);

        // Kick off parallel tasks for each of the root systems
        self.exclusive_root_systems
            .iter()
            .copied()
            .for_each(|system_index| {
                exec_task(&systems, &done, &payloads, world, system_index);
            });

        self.exclusive_systems = systems;

        // Wait for all of the systems to complete their execution
        wg.wait();
    }

    fn execute_parallel_systems(&mut self, world: &mut World) {
        /// Struct that holds data that needs ownership transferred to the thread that executes the
        /// matching system
        struct WorkerPayload {
            wg: WaitGroup,
        }

        /// Alias for the container a payload is sent to other threads in
        ///
        /// A Box is used to ensure the time in the AtomicCell is pointer sized so it can be sent
        /// using atomic instructions instead of locks
        type PayloadCell = AtomicCell<Option<Box<WorkerPayload>>>;

        // Treat a non lock free implementation as an error
        assert!(PayloadCell::is_lock_free());

        // Root wait group that forces the function to wait for all systems to complete for exiting
        let wg = WaitGroup::new();

        // SoA list of flags that denote whether the matching task has completed, indexed in
        // parallel with self.systems
        let done: Vec<AtomicBool> = (0..self.systems.len())
            .into_iter()
            .map(|_| AtomicBool::new(false))
            .collect();

        // SoA list of worker payloads, indexed in parallel with self.systems
        let payloads: Vec<PayloadCell> = (0..self.systems.len())
            .into_iter()
            .map(|_| {
                let payload = WorkerPayload { wg: wg.clone() };
                AtomicCell::new(Some(Box::new(payload)))
            })
            .collect();

        // This handles executing a system, then recursively executing the successive tasks
        fn exec_task(
            systems: &[SystemBox<SystemCell>],
            done: &[AtomicBool],
            payloads: &[PayloadCell],
            world: &World,
            system_index: usize,
        ) {
            // Unpack the payload
            let payload = if let Some(payload) = payloads[system_index].take() {
                payload
            } else {
                return;
            };

            // Unpack the wait group to explicitly drop it to "use" it
            let wg = payload.wg;

            // Pull the system from the cell, execute it, then return it to the cell
            let mut system = systems[system_index].system.take().unwrap();
            unsafe {
                // SAFETY: This is unsafe to call in the event of unsafe implementations of System
                //         that do not access world according to their access flags. If a System
                //         does correctly respect its access declarations then the work scheduler
                //         ensures that aliasing requirements will be upheld, making this safe to
                //         call. This is only unsafe in the presence of other unsafe code.
                system.execute((), world);
            }
            systems[system_index].system.store(Some(system));

            // Update the "done" flag now that the system has executed
            done[system_index].store(true, Ordering::Relaxed);

            // Spawn new tasks for each successor system and execute it, if all of its predecessors
            // have completed.
            systems[system_index]
                .edges
                .successors
                .par_iter()
                .copied()
                .for_each(|successor| {
                    let successor: usize = successor;
                    if systems[successor]
                        .edges
                        .predecessors
                        .iter()
                        .copied()
                        .all(|predecessor| done[predecessor].load(Ordering::Relaxed))
                    {
                        exec_task(systems, done, payloads, world, successor);
                    }
                });

            // Explicitly drop the wait group to "use" it according to the compiler.
            drop(wg);
        }

        let systems = std::mem::take(&mut self.systems);

        // Kick off parallel tasks for each of the root systems
        self.root_systems
            .par_iter()
            .copied()
            .for_each(|system_index| {
                exec_task(&systems, &done, &payloads, world, system_index);
            });

        self.systems = systems;

        // Wait for all of the systems to complete their execution
        wg.wait();
    }

    /// Checks if the system set is marked as dirty. If so it will automatically rebuild the
    /// execution graph as it will now be out of date compared to the
    fn check_dirty(&mut self) {
        if self.dirty {
            self.rebuild_graph();
        }
    }

    /// Handles rebuilding the execution graph
    fn rebuild_graph(&mut self) {
        Self::clear_graph_nodes(&mut self.systems);
        Self::clear_graph_nodes(&mut self.exclusive_systems);
        self.root_systems.clear();
        self.exclusive_root_systems.clear();

        Self::collect_access_descriptors(&mut self.systems, &self.system_label_map);
        Self::collect_access_descriptors(
            &mut self.exclusive_systems,
            &self.exclusive_system_label_map,
        );

        Self::build_graph_nodes(&mut self.systems, &mut self.root_systems);
        Self::build_graph_nodes(
            &mut self.exclusive_systems,
            &mut self.exclusive_root_systems,
        );

        self.dirty = false;
    }

    /// Used for clearing all the edges from all the nodes prior to a graph rebuild
    fn clear_graph_nodes<T>(systems: &mut [SystemBox<T>]) {
        systems.iter_mut().for_each(|v| {
            v.edges.predecessors.clear();
            v.edges.successors.clear();
        });
    }

    fn collect_access_descriptors<T: GenericSystemCell>(
        systems: &mut [SystemBox<T>],
        system_label_map: &HashMap<Box<dyn Label>, usize>,
    ) {
        for i in 0..systems.len() {
            // First we call clear the access descriptor and re-populate it by calling
            // declare_access for each system
            {
                let v = &mut systems[i];
                v.access.clear();
                v.system.declare_access(&mut v.access);
            }

            // Next we write the explicit "runs before" execution dependencies into the graph
            let runs_before = std::mem::take(&mut systems[i].access.runs_before);
            for before in runs_before.iter() {
                // Get the index of the system that we wish to run before
                let before = system_label_map.get(before).copied().unwrap();

                // Mark ourselves as a predecessor to that system
                systems[before].edges.predecessors.insert(i);

                // Add the target system to our successor set
                systems[i].edges.successors.insert(before);
            }
            systems[i].access.runs_before = runs_before;

            // Next we write the explicit "runs after" execution dependencies into the graph
            let runs_after = std::mem::take(&mut systems[i].access.runs_after);
            for after in runs_after.iter() {
                // Get the index of the system that we wish to run after
                let after = system_label_map.get(after).copied().unwrap();

                // Mark ourselves as a successor to that system
                systems[after].edges.successors.insert(i);

                // Add the target system to our predecessor set
                systems[i].edges.predecessors.insert(after);
            }
            systems[i].access.runs_after = runs_after;
        }
    }

    fn build_graph_nodes<T>(systems: &mut [SystemBox<T>], root_systems: &mut Vec<usize>) {
        let mut last_component_write: HashMap<ComponentTypeId, usize> = HashMap::new();
        let mut last_component_reads: HashMap<ComponentTypeId, Vec<usize>> = HashMap::new();
        let mut last_resource_write: HashMap<ResourceId, usize> = HashMap::new();
        let mut last_resource_reads: HashMap<ResourceId, Vec<usize>> = HashMap::new();

        for system_index in 0..systems.len() {
            Self::handle_writes(
                systems,
                &mut last_component_write,
                &mut last_component_reads,
                &mut last_resource_write,
                &mut last_resource_reads,
                system_index,
            );

            Self::handle_reads(
                systems,
                &mut last_component_write,
                &mut last_component_reads,
                &mut last_resource_write,
                &mut last_resource_reads,
                system_index,
            );
        }

        for (i, system) in systems.iter().enumerate() {
            if system.edges.predecessors.is_empty() {
                root_systems.push(i);
            }
        }
    }

    fn handle_writes<T>(
        systems: &mut [SystemBox<T>],
        last_component_write: &mut HashMap<ComponentTypeId, usize>,
        last_component_reads: &mut HashMap<ComponentTypeId, Vec<usize>>,
        last_resource_write: &mut HashMap<ResourceId, usize>,
        last_resource_reads: &mut HashMap<ResourceId, Vec<usize>>,
        system_index: usize,
    ) {
        let writes = std::mem::take(&mut systems[system_index].access.component_writes);
        Self::handle_writes_generic(
            systems,
            writes.iter().copied(),
            last_component_write,
            last_component_reads,
            system_index,
        );
        systems[system_index].access.component_writes = writes;

        let writes = std::mem::take(&mut systems[system_index].access.resource_writes);
        Self::handle_writes_generic(
            systems,
            writes.iter().copied(),
            last_resource_write,
            last_resource_reads,
            system_index,
        );
        systems[system_index].access.resource_writes = writes;
    }

    fn handle_writes_generic<T: Copy + Eq + Hash, X>(
        systems: &mut [SystemBox<X>],
        writes: impl Iterator<Item = T>,
        last_write: &mut HashMap<T, usize>,
        last_reads: &mut HashMap<T, Vec<usize>>,
        system_index: usize,
    ) {
        for write in writes {
            last_write.insert(write.clone(), system_index);

            match last_reads.get_mut(&write) {
                None => {}
                Some(reads) => {
                    for read in reads.iter().copied() {
                        if read != system_index {
                            systems[system_index].edges.predecessors.insert(read);
                            systems[read].edges.successors.insert(system_index);
                        }
                    }
                    reads.clear();
                }
            }
        }
    }

    fn handle_reads<T>(
        systems: &mut [SystemBox<T>],
        last_component_write: &mut HashMap<ComponentTypeId, usize>,
        last_component_reads: &mut HashMap<ComponentTypeId, Vec<usize>>,
        last_resource_write: &mut HashMap<ResourceId, usize>,
        last_resource_reads: &mut HashMap<ResourceId, Vec<usize>>,
        system_index: usize,
    ) {
        let reads = std::mem::take(&mut systems[system_index].access.component_reads);
        Self::handle_reads_generic(
            systems,
            reads.iter().copied(),
            last_component_write,
            last_component_reads,
            system_index,
        );
        systems[system_index].access.component_reads = reads;

        let reads = std::mem::take(&mut systems[system_index].access.resource_reads);
        Self::handle_reads_generic(
            systems,
            reads.iter().copied(),
            last_resource_write,
            last_resource_reads,
            system_index,
        );
        systems[system_index].access.resource_reads = reads;
    }

    fn handle_reads_generic<T: Copy + Eq + Hash, X>(
        systems: &mut [SystemBox<X>],
        reads: impl Iterator<Item = T>,
        last_write: &mut HashMap<T, usize>,
        last_reads: &mut HashMap<T, Vec<usize>>,
        system_index: usize,
    ) {
        for read in reads {
            match last_reads.get_mut(&read) {
                None => {
                    let mut vec = Vec::with_capacity(4);
                    vec.push(system_index);
                    last_reads.insert(read, vec);
                }
                Some(vec) => {
                    vec.push(system_index);
                }
            }

            match last_write.get(&read).copied() {
                None => {}
                Some(write) => {
                    if write != system_index {
                        systems[system_index].edges.predecessors.insert(write);
                        systems[write].edges.successors.insert(system_index);
                    }
                }
            }
        }
    }
}

/// Type alias for a boxed system trait object
type BoxedSystem = Box<dyn System<In = (), Out = ()> + Send + Sync>;

/// Type alias for a boxed system trait object
type BoxedExclusiveSystem = Box<dyn System<In = (), Out = ()>>;

/// Type alias for the thread safe slot a system is stored in. The type is very verbose to write.
///
/// We need to double box the system to make sizeof(Option<Box<BoxedSystem>>) == 8 so atomics can
/// be used. Otherwise global locks would be used to send the systems and that's bad
type SystemCell = AtomicCell<Option<Box<BoxedSystem>>>;

/// Type alias for the thread safe slot a system is stored in. The type is very verbose to write.
///
/// We need to double box the system to make sizeof(Option<Box<BoxedSystem>>) == 8 so atomics can
/// be used. Otherwise global locks would be used to send the systems and that's bad
type ExclusiveSystemCell = Cell<Option<Box<BoxedExclusiveSystem>>>;

/// Generic wrapper for allowing to schedule different system streams with the same implementations
trait GenericSystemCell {
    fn declare_access(&self, access: &mut SystemAccessDescriptor);
}

impl GenericSystemCell for SystemCell {
    fn declare_access(&self, access: &mut SystemAccessDescriptor) {
        let mut system = self.take().unwrap();
        system.declare_access(access);
        self.store(Some(system))
    }
}

impl GenericSystemCell for ExclusiveSystemCell {
    fn declare_access(&self, access: &mut SystemAccessDescriptor) {
        let mut system = self.take().unwrap();
        system.declare_access(access);
        self.set(Some(system))
    }
}

///
/// Internal container for pairing a boxed system with some metadata used to schedule the system
///
struct SystemBox<T> {
    /// The boxed system, stored in an atomic cell so it can be sent to other threads
    system: T,

    /// The accesses declared by the system
    access: SystemAccessDescriptor,

    /// The edges out of the system's node in the execution graph
    edges: GraphEdges,
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
struct GraphEdges {
    /// A set of indices to the systems that precede the execution of `system`
    predecessors: HashSet<usize>,

    /// A set of indices to the systems that execute after `system`
    successors: HashSet<usize>,
}

///
/// Internal container for storing the sets of resource accesses of a system
///
struct SystemAccessDescriptor {
    /// The label of the system we're collecting access for currently
    label: Box<dyn Label>,

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
    fn new(label: Box<dyn Label>) -> Self {
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
    fn clear(&mut self) {
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
