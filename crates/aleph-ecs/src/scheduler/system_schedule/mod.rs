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

mod access_descriptor;
mod system_box;
mod system_cell;
mod system_channel;

use crate::scheduler::system_schedule::system_box::SystemBox;
use crate::scheduler::system_schedule::system_cell::{ExclusiveSystemCell, SystemCell};
use crate::scheduler::system_schedule::system_channel::SystemChannel;
use crate::scheduler::Stage;
use crate::system::{IntoSystem, System};
use crate::world::World;
use aleph_label::Label;

#[derive(Default)]
pub struct SystemSchedule {
    /// Systems and graph for single threaded systems that runs before the parallel phase
    exclusive_at_start: SystemChannel<ExclusiveSystemCell>,

    /// Systems and graph for the main parallel system phase
    parallel_systems: SystemChannel<SystemCell>,

    /// Systems and graph for the single threaded systems that runs after the parallel phase
    exclusive_at_end: SystemChannel<ExclusiveSystemCell>,

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
        let index = self.exclusive_at_start.systems.len();
        self.exclusive_at_start
            .systems
            .push(SystemBox::<ExclusiveSystemCell>::new_exclusive(
                label.clone(),
                system.system(),
            ));

        // Insert the label into the label->index map, checking if the label has already been
        // registered (triggers a panic)
        if self
            .exclusive_at_start
            .system_label_map
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
        let index = self.parallel_systems.systems.len();
        self.parallel_systems
            .systems
            .push(SystemBox::new(label.clone(), system.system()));

        // Insert the label into the label->index map, checking if the label has already been
        // registered (triggers a panic)
        if self
            .parallel_systems
            .system_label_map
            .insert(label.clone(), index)
            .is_some()
        {
            panic!("System already exists: {:?}.", label);
        }
        self
    }

    pub fn run_once(&mut self, world: &mut World) {
        self.check_dirty();
        self.exclusive_at_start.execute_exclusive(world);
        self.parallel_systems.execute_parallel(world);
        self.exclusive_at_end.execute_exclusive(world);
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
    fn check_dirty(&mut self) {
        if self.dirty {
            self.rebuild_graph();
        }
    }

    /// Handles rebuilding the execution graph
    fn rebuild_graph(&mut self) {
        self.exclusive_at_start.clear_graph_nodes();
        self.parallel_systems.clear_graph_nodes();
        self.exclusive_at_end.clear_graph_nodes();

        self.exclusive_at_start.collect_access_descriptors();
        self.parallel_systems.collect_access_descriptors();
        self.exclusive_at_end.collect_access_descriptors();

        self.exclusive_at_start.build_graph_nodes();
        self.parallel_systems.build_graph_nodes();
        self.exclusive_at_end.build_graph_nodes();

        self.dirty = false;
    }
}
