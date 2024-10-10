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

use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicBool, Ordering};

use aleph_label::Label;
use aleph_object_system::uuid::Uuid;
use aleph_typed_table::TypedTable;
use crossbeam::atomic::AtomicCell;
use crossbeam::sync::WaitGroup;
use rayon::prelude::*;

use crate::system::{IntoSystem, System};
use crate::system_schedule::system_box::SystemBox;
use crate::system_schedule::system_cell::{ExclusiveSystemCell, GenericSystemCell, SystemCell};
use crate::ScheduleArgs;

pub struct SystemChannel<A, T> {
    /// Stores all systems in the schedule
    pub systems: Vec<SystemBox<T>>,

    /// Maps a label to the system it was registered with. Accelerates looking up a system by label
    /// as well as accelerating duplicate label checks.
    pub system_label_map: HashMap<Label, usize>,

    /// This caches the list of root tasks where execution should start from
    pub root_systems: Vec<usize>,

    _phantom: PhantomData<A>,
}

impl<A, T> Default for SystemChannel<A, T> {
    fn default() -> Self {
        Self {
            systems: Default::default(),
            system_label_map: Default::default(),
            root_systems: Default::default(),
            _phantom: Default::default(),
        }
    }
}

impl<A: ScheduleArgs> SystemChannel<A, SystemCell<A>> {
    pub fn add_system<
        Param,
        T: System<In = A, Out = ()> + Send + Sync,
        S: IntoSystem<A, (), Param, System = T>,
    >(
        &mut self,
        label: Label,
        system: S,
    ) {
        // Push the new system into the system list, capturing the index it will be inserted into
        let index = self.systems.len();
        self.systems.push(SystemBox::new(label, system.system()));

        // Insert the label into the label->index map, checking if the label has already been
        // registered (triggers a panic)
        if self.system_label_map.insert(label, index).is_some() {
            panic!("System already exists: {label:?}.");
        }
    }
}

impl<A: ScheduleArgs> SystemChannel<A, ExclusiveSystemCell<A>> {
    pub fn add_system<
        Param,
        T: System<In = A, Out = ()>,
        S: IntoSystem<A, (), Param, System = T>,
    >(
        &mut self,
        label: Label,
        system: S,
    ) {
        // Push the new system into the system list, capturing the index it will be inserted into
        let index = self.systems.len();
        self.systems
            .push(SystemBox::new_exclusive(label, system.system()));

        // Insert the label into the label->index map, checking if the label has already been
        // registered (triggers a panic)
        if self.system_label_map.insert(label, index).is_some() {
            panic!("System already exists: {label:?}.");
        }
    }
}

impl<A: ScheduleArgs, C: GenericSystemCell<A> + Send + Sync> SystemChannel<A, C> {
    pub fn execute_parallel(&mut self, args: &A::Args<'_>, resources: &mut TypedTable) {
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
            .map(|_| AtomicBool::new(false))
            .collect();

        // SoA list of worker payloads, indexed in parallel with self.systems
        let payloads: Vec<PayloadCell> = (0..self.systems.len())
            .map(|_| {
                let payload = WorkerPayload { wg: wg.clone() };
                AtomicCell::new(Some(Box::new(payload)))
            })
            .collect();

        // This handles executing a system, then recursively executing the successive tasks
        fn exec_task<A: ScheduleArgs, T: GenericSystemCell<A> + Send + Sync>(
            args: &A::Args<'_>,
            systems: &[SystemBox<T>],
            done: &[AtomicBool],
            payloads: &[PayloadCell],
            resources: &TypedTable,
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

            // SAFETY: This is unsafe to call in the event of unsafe implementations of System
            //         that do not access world according to their access flags. If a System
            //         does correctly respect its access declarations then the work scheduler
            //         ensures that aliasing requirements will be upheld, making this safe to
            //         call. This is only unsafe in the presence of other unsafe code.
            unsafe {
                let system = &systems[system_index];
                aleph_profile::scope!("aleph::ExecSystem", system.access.label);
                system.system.execute(args, resources);
            }

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
                        exec_task(args, systems, done, payloads, resources, successor);
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
                exec_task(args, &systems, &done, &payloads, resources, system_index);
            });

        self.systems = systems;

        // Wait for all of the systems to complete their execution
        wg.wait();
    }
}

impl<A: ScheduleArgs, C: GenericSystemCell<A>> SystemChannel<A, C> {
    pub fn execute_exclusive(&mut self, args: &A::Args<'_>, resources: &mut TypedTable) {
        // SoA list of flags that denote whether the matching task has completed, indexed in
        // parallel with self.systems
        let mut done: Vec<bool> = (0..self.systems.len()).map(|_| false).collect();

        // Stores the number of systems that have not yet been executed. Once this reaches 0 all
        // systems are done and we exit the exec loop
        let mut live_count = self.systems.len();

        while live_count != 0 {
            for system_index in 0..self.systems.len() {
                // Skip this system if we've already executed it
                if done[system_index] {
                    continue;
                }

                // Skip this system if we haven't executed all the predecessors yet
                let all_predecessors_done = self.systems[system_index]
                    .edges
                    .predecessors
                    .iter()
                    .copied()
                    .all(|predecessor| done[predecessor]);
                if !all_predecessors_done {
                    continue;
                }

                // Execute the system
                {
                    let system = &self.systems[system_index];
                    aleph_profile::scope!("aleph::ExecSystem", system.access.label);
                    system.system.execute_safe(args, resources);
                }

                // Update the "done" flag now that the system has executed
                done[system_index] = true;

                // Decrement the live system count. Once this reaches 0 all systems are done and we
                // exit the exec loop
                live_count -= 1;
            }
        }
    }

    /// Used for clearing all the edges from all the nodes prior to a graph rebuild
    pub fn clear_graph_nodes(&mut self) {
        self.systems.iter_mut().for_each(|v| {
            v.edges.predecessors.clear();
            v.edges.successors.clear();
        });
        self.root_systems.clear();
    }

    pub fn collect_access_descriptors(&mut self) {
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

    pub fn build_graph_nodes(&mut self) {
        let mut last_resource_write: HashMap<Uuid, usize> = HashMap::new();
        let mut last_resource_reads: HashMap<Uuid, Vec<usize>> = HashMap::new();

        for system_index in 0..self.systems.len() {
            self.handle_writes(
                &mut last_resource_write,
                &mut last_resource_reads,
                system_index,
            );

            self.handle_reads(
                &mut last_resource_write,
                &mut last_resource_reads,
                system_index,
            );
        }

        for (i, system) in self.systems.iter().enumerate() {
            if system.edges.predecessors.is_empty() {
                self.root_systems.push(i);
            }
        }
    }

    pub fn handle_writes(
        &mut self,
        last_resource_write: &mut HashMap<Uuid, usize>,
        last_resource_reads: &mut HashMap<Uuid, Vec<usize>>,
        system_index: usize,
    ) {
        let writes = std::mem::take(&mut self.systems[system_index].access.resource_writes);
        self.handle_writes_generic(
            writes.iter(),
            last_resource_write,
            last_resource_reads,
            system_index,
        );
        self.systems[system_index].access.resource_writes = writes;
    }

    pub fn handle_writes_generic<'a, T, TB, I>(
        &mut self,
        writes: I,
        last_write: &mut HashMap<T, usize>,
        last_reads: &mut HashMap<T, Vec<usize>>,
        system_index: usize,
    ) where
        TB: ToOwned<Owned = T> + ?Sized + Eq + Hash + 'a,
        T: Eq + Hash + Borrow<TB>,
        I: Iterator<Item = &'a TB>,
    {
        for write in writes {
            last_write.insert(write.to_owned(), system_index);

            match last_reads.get_mut(write) {
                None => {}
                Some(reads) => {
                    for read in reads.iter().copied() {
                        if read != system_index {
                            self.systems[system_index].edges.predecessors.insert(read);
                            self.systems[read].edges.successors.insert(system_index);
                        }
                    }
                    reads.clear();
                }
            }
        }
    }

    pub fn handle_reads(
        &mut self,
        last_resource_write: &mut HashMap<Uuid, usize>,
        last_resource_reads: &mut HashMap<Uuid, Vec<usize>>,
        system_index: usize,
    ) {
        let reads = std::mem::take(&mut self.systems[system_index].access.resource_reads);
        self.handle_reads_generic(
            reads.iter(),
            last_resource_write,
            last_resource_reads,
            system_index,
        );
        self.systems[system_index].access.resource_reads = reads;
    }

    pub fn handle_reads_generic<'a, T, TB, I>(
        &mut self,
        reads: I,
        last_write: &mut HashMap<T, usize>,
        last_reads: &mut HashMap<T, Vec<usize>>,
        system_index: usize,
    ) where
        TB: ToOwned<Owned = T> + ?Sized + Hash + Eq + 'a,
        T: Eq + Hash + Borrow<TB>,
        I: Iterator<Item = &'a TB>,
    {
        for read in reads {
            match last_reads.get_mut(read) {
                None => {
                    let mut vec = Vec::with_capacity(4);
                    vec.push(system_index);
                    last_reads.insert(read.to_owned(), vec);
                }
                Some(vec) => {
                    vec.push(system_index);
                }
            }

            match last_write.get(read).copied() {
                None => {}
                Some(write) => {
                    if write != system_index {
                        self.systems[system_index].edges.predecessors.insert(write);
                        self.systems[write].edges.successors.insert(system_index);
                    }
                }
            }
        }
    }
}
