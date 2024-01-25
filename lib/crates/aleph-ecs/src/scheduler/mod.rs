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

// Much of the implementation of this code is either copied or heavily based on code from the Bevy
// project. Available here: https://github.com/bevyengine/bevy and https://bevyengine.org/
//
// To respect the license terms I provide the license here.
//
// MIT License
//
// Copyright (c) 2020 bevyengine.org
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

mod stage;
mod system_schedule;

use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::ptr::NonNull;

use aleph_arena_drop_list::DropLink;
use aleph_label::Label;
use bumpalo::Bump;
pub use stage::{AccessDescriptor, Stage};
pub use system_schedule::SystemSchedule;

use crate::system::{IntoSystem, System};
use crate::world::World;

///
/// Provides an interface for dynamically composing a set of executable code blocks into a sequence
/// of stages. The order these stages are executed in is configurable at runtime.
///
/// # Credit
///
/// The API and most of the implementation of this struct is ripped from `bevy_ecs`. I don't think
/// I can beat the API that `bevy_ecs` designed, so rather than trying to I just use it for myself.
///
#[derive(Default)]
pub struct Schedule {
    /// A bump allocated arena used for storing the labels and stages.
    arena: Bump,

    /// Stores the execution stages along with the label they were registered with
    stages: HashMap<LabelKey, NonNull<dyn Stage>>,

    /// The head of a linked list of all objects stored in the schedule's bump arena that need to
    /// be dropped when the Schedule is destroyed. This includes all the labels, stages and systems.
    drop_head: Option<NonNull<DropLink>>,

    /// Stores the execution order of the schedule as an ordered sequence of stage labels
    stage_order: Vec<NonNull<dyn Label>>,

    /// Stores the schedule's run criteria system which checks prior to executing the stages whether
    /// the schedule should actually run.
    ///
    /// Functions as a dynamically slot for a run precondition function.
    run_criteria: RunCriteriaBox,
}

impl Schedule {
    /// Sets the run criteria system for this schedule, replacing the old one if it existed.
    ///
    /// The run criteria system will be called when [`Schedule`] is used as a [`Stage`] inside
    /// another [`Schedule`]. A [`Schedule`]'s run criteria system will be called before any of the
    /// child stages to check if the child stages should actually execute.
    ///
    /// This provides a dynamic and composable system for skipping work based on arbitrary
    /// preconditions.
    #[inline]
    pub fn set_run_criteria<
        Param,
        T: System<In = (), Out = ShouldRun> + Send + Sync,
        S: IntoSystem<(), ShouldRun, Param, System = T>,
    >(
        &mut self,
        system: S,
    ) -> &mut Self {
        let system = self.arena.alloc(system.system());
        let system = NonNull::from(system);
        DropLink::append_drop_list(&self.arena, &mut self.drop_head, system);
        self.run_criteria.system = Some(NonNull::from(system));
        self.run_criteria.initialized = false;
        self
    }

    /// This adds a single execution stage to the [`Schedule`] with the provided [`Label`]. The
    /// stage will be appended to the very end of the execution order, so it will run last.
    #[inline]
    pub fn add_stage<S: Stage>(&mut self, label: impl Label, stage: S) -> &mut Self {
        let label = self.arena.alloc(label);
        let label = NonNull::from(label);
        DropLink::append_drop_list(&self.arena, &mut self.drop_head, label);
        self.stage_order.push(label);

        let stage = self.arena.alloc(stage);
        let stage = NonNull::from(stage);
        DropLink::append_drop_list(&self.arena, &mut self.drop_head, stage);
        let prev = self.stages.insert(LabelKey(label), stage);
        if prev.is_some() {
            let label = unsafe { label.as_ref() };
            panic!("Stage already exists: {label:?}.");
        }
        self
    }

    /// This adds a single execution stage to the [`Schedule`] with the provided [`Label`]. The
    /// stage will be inserted into the execution immediately after the `target` stage.
    #[inline]
    pub fn add_stage_after<S: Stage>(
        &mut self,
        target: impl Label,
        label: impl Label,
        stage: S,
    ) -> &mut Self {
        // Box the label for the stage we're adding
        let label = self.arena.alloc(label);
        let label = NonNull::from(label);
        DropLink::append_drop_list(&self.arena, &mut self.drop_head, label);

        // Lookup the index of the stage that the new stage should execute after
        let target = &target as &dyn Label;
        let target_index = self.index_from_label(target);

        // Insert the stage's label into the execution order directly after the prerequisite stage.
        //
        // This fulfills the requirement for ensuring the new stage runs after the requested stage.
        self.stage_order.insert(target_index + 1, label);

        // Insert the new stage into the stage storage. We panic if there was already a stage
        // registered with the provided label as we do not allow overwriting already provided
        // stages.
        let stage = self.arena.alloc(stage);
        let stage = NonNull::from(stage);
        DropLink::append_drop_list(&self.arena, &mut self.drop_head, stage);
        let prev = self.stages.insert(LabelKey(label), stage);
        if prev.is_some() {
            let label = unsafe { label.as_ref() };
            panic!("Stage already exists: {label:?}.");
        }
        self
    }

    /// This adds a single execution stage to the [`Schedule`] with the provided [`Label`]. The
    /// stage will be inserted into the execution immediately before the `target` stage.
    #[inline]
    pub fn add_stage_before<S: Stage>(
        &mut self,
        target: impl Label,
        label: impl Label,
        stage: S,
    ) -> &mut Self {
        // Box the label for the stage we're adding
        let label = self.arena.alloc(label);
        let label = NonNull::from(label);
        DropLink::append_drop_list(&self.arena, &mut self.drop_head, label);

        // Lookup the index of the stage that the new stage should execute before
        let target = &target as &dyn Label;
        let target_index = self.index_from_label(target);

        // Insert the stage's label into the execution order directly before the stage that the new
        // stage needs to run before.
        //
        // This fulfills the requirement for ensuring the new stage runs before the requested stage.
        self.stage_order.insert(target_index, label);

        // Insert the new stage into the stage storage. We panic if there was already a stage
        // registered with the provided label as we do not allow overwriting already provided
        // stages.
        let stage = self.arena.alloc(stage);
        let stage = NonNull::from(stage);
        DropLink::append_drop_list(&self.arena, &mut self.drop_head, stage);
        let prev = self.stages.insert(LabelKey(label), stage);
        if prev.is_some() {
            let label = unsafe { label.as_ref() };
            panic!("Stage already exists: {label:?}.");
        }
        self
    }

    /// Inserts the given system labeled `label` into the stage with the `target` label.
    ///
    /// # Panics
    ///
    /// - Will panic if the stage with [`Label`] of `label` does not exist.
    /// - Will panic if the stage is not of type [`SystemSchedule`].
    #[inline]
    pub fn add_system_to_stage<
        Param,
        T: System<In = (), Out = ()> + Send + Sync,
        S: IntoSystem<(), (), Param, System = T>,
    >(
        &mut self,
        target: &impl Label,
        label: impl Label,
        system: S,
    ) -> &mut Self {
        self.stage(target, move |v: &mut SystemSchedule| {
            v.add_system(label, system)
        })
    }

    /// Inserts the given system labeled `label` into the stage with the `target` label.
    ///
    /// # Panics
    ///
    /// - Will panic if the stage with [`Label`] of `label` does not exist.
    /// - Will panic if the stage is not of type [`SystemSchedule`].
    #[inline]
    pub fn add_exclusive_at_start_system_to_stage<
        Param,
        T: System<In = (), Out = ()>,
        S: IntoSystem<(), (), Param, System = T>,
    >(
        &mut self,
        target: &impl Label,
        label: impl Label,
        system: S,
    ) -> &mut Self {
        self.stage(target, move |v: &mut SystemSchedule| {
            v.add_exclusive_at_start_system(label, system)
        })
    }

    // Inserts the given system labeled `label` into the stage with the `target` label.
    ///
    /// # Panics
    ///
    /// - Will panic if the stage with [`Label`] of `label` does not exist.
    /// - Will panic if the stage is not of type [`SystemSchedule`].
    #[inline]
    pub fn add_exclusive_at_end_system_to_stage<
        Param,
        T: System<In = (), Out = ()>,
        S: IntoSystem<(), (), Param, System = T>,
    >(
        &mut self,
        target: &impl Label,
        label: impl Label,
        system: S,
    ) -> &mut Self {
        self.stage(target, move |v: &mut SystemSchedule| {
            v.add_exclusive_at_end_system(label, system)
        })
    }

    /// Looks up the [`Stage`] that was registered with the [`Label`] provided in `label` and passes
    /// a downcasted reference into the closure provided in `func`.
    ///
    /// # Panics
    ///
    /// Will panic if the stage is not present or does not match the type `T`
    #[inline]
    pub fn stage<T: Stage, F: FnOnce(&mut T) -> &mut T>(
        &mut self,
        label: &impl Label,
        func: F,
    ) -> &mut Self {
        let stage = self.get_stage_mut(label).unwrap_or_else(move || {
            panic!("stage '{label:?}' does not exist or is the wrong type")
        });
        func(stage);
        self
    }

    /// Get's a down-casted reference to the stage registered with the [`Label`] provided in `label`
    #[inline]
    pub fn get_stage<T: Stage>(&self, label: &dyn Label) -> Option<&T> {
        self.stages
            .get(&LabelKey(NonNull::from(label)))
            .and_then(|stage| unsafe { stage.as_ref().downcast_ref::<T>() })
    }

    /// Get's a down-casted reference to the stage registered with the [`Label`] provided in `label`
    #[inline]
    pub fn get_stage_mut<T: Stage>(&mut self, label: &dyn Label) -> Option<&mut T> {
        self.stages
            .get_mut(&LabelKey(NonNull::from(label)))
            .and_then(|stage| unsafe { stage.as_mut().downcast_mut::<T>() })
    }

    /// Unconditionally (i.e, the run criteria system is **not** called) performs a single execution
    /// run of the [`Schedule`]
    #[inline]
    pub fn run_once(&mut self, world: &mut World) {
        for label in self.stage_order.iter() {
            let stage = self.stages.get_mut(&LabelKey(*label)).unwrap();
            unsafe {
                stage.as_mut().run(world);
            }
        }
    }

    /// Iterates over all of schedule's stages and their labels, in execution order.
    #[inline]
    pub fn iter_stages(&self) -> impl Iterator<Item = (&dyn Label, &dyn Stage)> {
        self.stage_order
            .iter()
            .map(move |label| unsafe { (label.as_ref(), self.stages[&LabelKey(*label)].as_ref()) })
    }
}

/// Internal utility functions
impl Schedule {
    fn index_from_label(&self, target: &dyn Label) -> usize {
        self.stage_order
            .iter()
            .enumerate()
            .find(|(_i, stage_label)| unsafe { stage_label.as_ref() == target })
            .map(|(i, _)| i)
            .unwrap_or_else(|| panic!("Target stage does not exist: {target:?}."))
    }
}

impl Stage for Schedule {
    fn run(&mut self, world: &mut World) {
        // First we need to check if the schedule's run criteria is met
        if let Some(system) = self.run_criteria.system.as_mut() {
            // Initialize the criteria system if it hasn't already been
            if !self.run_criteria.initialized {
                //system.build(world);
                self.run_criteria.initialized = true;
            }

            // Execute the system and bail if it decides we should not run the schedule
            let system = unsafe { system.as_mut() };
            if system.execute_safe((), world) == ShouldRun::No {
                return;
            }
        }

        // If we pass the above check then we can continue on and execute the schedule
        self.run_once(world);
    }
}

impl Drop for Schedule {
    fn drop(&mut self) {
        // Safety: Drops all the objects that were allocated from the bump arena by walking the
        //         linked list we build while adding them to the schedule.
        unsafe {
            DropLink::drop_and_null(&mut self.drop_head);
        }
    }
}

///
/// A simple enum that specifies the options a `run criteria` system can return.
///
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum ShouldRun {
    Yes,
    No,
}

#[derive(Default)]
struct RunCriteriaBox {
    system: Option<NonNull<dyn System<In = (), Out = ShouldRun>>>,
    initialized: bool,
}

#[repr(transparent)]
struct LabelKey(NonNull<dyn Label>);

impl PartialEq for LabelKey {
    fn eq(&self, other: &Self) -> bool {
        unsafe { self.0.as_ref().eq(other.0.as_ref()) }
    }
}

impl Eq for LabelKey {}

impl Hash for LabelKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe {
            self.0.as_ref().dyn_hash(state);
        }
    }
}
