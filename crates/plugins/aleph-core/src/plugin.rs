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

use crate::schedule_provider::ScheduleProvider;
use crate::world_provider::WorldProvider;
use interfaces::any::AnyArc;
use interfaces::plugin::{
    IInitResponse, IPlugin, IPluginRegistrar, IRegistryAccessor, PluginDescription,
};
use interfaces::schedule::{IScheduleProvider, Schedule, Stage, SystemSchedule, CoreStage};
use interfaces::world::IWorldProvider;
use std::any::TypeId;

pub struct PluginCore {
    world_provider: AnyArc<WorldProvider>,
    schedule_provider: AnyArc<ScheduleProvider>,
}

impl PluginCore {
    pub fn new() -> Self {
        let mut schedule = Schedule::default();
        schedule.add_stage(CoreStage::InputCollection, SystemSchedule::default());
        schedule.add_stage(CoreStage::PreUpdate, SystemSchedule::default());
        schedule.add_stage(CoreStage::Update, SystemSchedule::default());
        schedule.add_stage(CoreStage::PostUpdate, SystemSchedule::default());
        schedule.add_stage(CoreStage::Render, SystemSchedule::default());

        let world_provider = AnyArc::new(WorldProvider::new());
        let schedule_provider = AnyArc::new(ScheduleProvider::new(schedule));
        Self {
            world_provider,
            schedule_provider,
        }
    }
}

impl IPlugin for PluginCore {
    fn get_description(&self) -> PluginDescription {
        PluginDescription {
            name: "PluginCore".to_string(),
            description: "Foundational plugin that provides core level interfaces".to_string(),
            major_version: 0,
            minor_version: 1,
            patch_version: 0,
        }
    }

    fn register(&mut self, registrar: &mut dyn IPluginRegistrar) {
        // We want to update in the pre update stage and post update stage
        registrar.should_update();
        // registrar.update_stage(UpdateStage::PostUpdate);

        // We export two interfaces
        registrar.provides_interface::<dyn IWorldProvider>();
        registrar.provides_interface::<dyn IScheduleProvider>();
    }

    fn on_init(&mut self, _registry: &dyn IRegistryAccessor) -> Box<dyn IInitResponse> {
        let response = vec![
            (
                TypeId::of::<dyn IWorldProvider>(),
                AnyArc::into_any(self.world_provider.clone()),
            ),
            (
                TypeId::of::<dyn IScheduleProvider>(),
                AnyArc::into_any(self.schedule_provider.clone()),
            ),
        ];
        Box::new(response)
    }

    fn on_update(&mut self, _: &dyn IRegistryAccessor) {
        let world_cell = self.world_provider.get();
        let schedule_cell = self.schedule_provider.get();
        let mut world = world_cell.get();
        let mut schedule = schedule_cell.get();

        schedule.run(&mut world);
    }
}

interfaces::any::declare_interfaces!(PluginCore, [IPlugin]);
