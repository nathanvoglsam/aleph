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

use crate::scheduler::system_schedule::system_box::SystemBox;
use crate::scheduler::system_schedule::system_cell::GenericSystemCell;
use crate::world::{ComponentTypeId, ResourceId, World};
use aleph_label::Label;
use crossbeam::atomic::AtomicCell;
use crossbeam::sync::WaitGroup;
use rayon::prelude::*;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Default)]
pub struct SystemChannel<T: GenericSystemCell> {
    /// Stores all systems in the schedule
    pub systems: Vec<SystemBox<T>>,

    /// Maps a label to the system it was registered with. Accelerates looking up a system by label
    /// as well as accelerating duplicate label checks.
    pub system_label_map: HashMap<Box<dyn Label>, usize>,

    /// This caches the list of root tasks where execution should start from
    pub root_systems: Vec<usize>,
}

impl<C: GenericSystemCell + Send + Sync> SystemChannel<C> {
    pub fn execute_parallel(&mut self, world: &mut World) {
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
        fn exec_task<T: GenericSystemCell + Send + Sync>(
            systems: &[SystemBox<T>],
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

            unsafe {
                // SAFETY: This is unsafe to call in the event of unsafe implementations of System
                //         that do not access world according to their access flags. If a System
                //         does correctly respect its access declarations then the work scheduler
                //         ensures that aliasing requirements will be upheld, making this safe to
                //         call. This is only unsafe in the presence of other unsafe code.
                systems[system_index].system.execute(world);
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
}

impl<C: GenericSystemCell> SystemChannel<C> {
    pub fn execute_exclusive(&mut self, world: &mut World) {
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
        fn exec_task<T: GenericSystemCell>(
            systems: &[SystemBox<T>],
            done: &[AtomicBool],
            payloads: &[PayloadCell],
            world: &mut World,
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

            // Execute the system
            systems[system_index].system.execute_safe(world);

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

        let systems = std::mem::take(&mut self.systems);

        // Kick off parallel tasks for each of the root systems
        self.root_systems.iter().copied().for_each(|system_index| {
            exec_task(&systems, &done, &payloads, world, system_index);
        });

        self.systems = systems;

        // Wait for all of the systems to complete their execution
        wg.wait();
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
        let mut last_component_write: HashMap<ComponentTypeId, usize> = HashMap::new();
        let mut last_component_reads: HashMap<ComponentTypeId, Vec<usize>> = HashMap::new();
        let mut last_resource_write: HashMap<ResourceId, usize> = HashMap::new();
        let mut last_resource_reads: HashMap<ResourceId, Vec<usize>> = HashMap::new();

        for system_index in 0..self.systems.len() {
            self.handle_writes(
                &mut last_component_write,
                &mut last_component_reads,
                &mut last_resource_write,
                &mut last_resource_reads,
                system_index,
            );

            self.handle_reads(
                &mut last_component_write,
                &mut last_component_reads,
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
        last_component_write: &mut HashMap<ComponentTypeId, usize>,
        last_component_reads: &mut HashMap<ComponentTypeId, Vec<usize>>,
        last_resource_write: &mut HashMap<ResourceId, usize>,
        last_resource_reads: &mut HashMap<ResourceId, Vec<usize>>,
        system_index: usize,
    ) {
        let writes = std::mem::take(&mut self.systems[system_index].access.component_writes);
        self.handle_writes_generic(
            writes.iter().copied(),
            last_component_write,
            last_component_reads,
            system_index,
        );
        self.systems[system_index].access.component_writes = writes;

        let writes = std::mem::take(&mut self.systems[system_index].access.resource_writes);
        self.handle_writes_generic(
            writes.iter().copied(),
            last_resource_write,
            last_resource_reads,
            system_index,
        );
        self.systems[system_index].access.resource_writes = writes;
    }

    pub fn handle_writes_generic<T: Copy + Eq + Hash>(
        &mut self,
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
        last_component_write: &mut HashMap<ComponentTypeId, usize>,
        last_component_reads: &mut HashMap<ComponentTypeId, Vec<usize>>,
        last_resource_write: &mut HashMap<ResourceId, usize>,
        last_resource_reads: &mut HashMap<ResourceId, Vec<usize>>,
        system_index: usize,
    ) {
        let reads = std::mem::take(&mut self.systems[system_index].access.component_reads);
        self.handle_reads_generic(
            reads.iter().copied(),
            last_component_write,
            last_component_reads,
            system_index,
        );
        self.systems[system_index].access.component_reads = reads;

        let reads = std::mem::take(&mut self.systems[system_index].access.resource_reads);
        self.handle_reads_generic(
            reads.iter().copied(),
            last_resource_write,
            last_resource_reads,
            system_index,
        );
        self.systems[system_index].access.resource_reads = reads;
    }

    pub fn handle_reads_generic<T: Copy + Eq + Hash>(
        &mut self,
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
                        self.systems[system_index].edges.predecessors.insert(write);
                        self.systems[write].edges.successors.insert(system_index);
                    }
                }
            }
        }
    }
}
