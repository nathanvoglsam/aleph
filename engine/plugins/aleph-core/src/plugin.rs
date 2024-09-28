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

use std::any::TypeId;

use aleph_label::{make_label, Label};
use interfaces::any::{AnyArc, IAny};
use interfaces::make_plugin_description_for_crate;
use interfaces::plugin::{
    IInitResponse, IPlugin, IPluginRegistrar, IRegistryAccessor, PluginDescription,
};
use interfaces::schedule::{CoreStage, IScheduleProvider};
use interfaces::scheduler::{Resources, Schedule, Stage, SystemSchedule};
use interfaces::world::IWorldProvider;

use crate::schedule_provider::ScheduleProvider;
use crate::world_provider::WorldProvider;

pub struct PluginCore {
    world_provider: AnyArc<WorldProvider>,
    schedule_provider: AnyArc<ScheduleProvider>,
}

impl PluginCore {
    pub fn new() -> Self {
        let core_schedule = SystemSchedule::default();

        let mut schedule = Schedule::default();
        schedule.add_stage(InternalStage::Core.into(), core_schedule);
        schedule.add_stage(CoreStage::InputCollection.into(), SystemSchedule::default());
        schedule.add_stage(CoreStage::PreUpdate.into(), SystemSchedule::default());
        schedule.add_stage(CoreStage::Update.into(), SystemSchedule::default());
        schedule.add_stage(CoreStage::PostUpdate.into(), SystemSchedule::default());
        schedule.add_stage(CoreStage::Render.into(), SystemSchedule::default());

        let world_provider = AnyArc::new(WorldProvider::new());
        let schedule_provider = AnyArc::new(ScheduleProvider::new(schedule));
        Self {
            world_provider,
            schedule_provider,
        }
    }
}

impl Default for PluginCore {
    fn default() -> Self {
        Self::new()
    }
}

impl IPlugin for PluginCore {
    fn get_description(&self) -> PluginDescription {
        make_plugin_description_for_crate!()
    }

    fn register(&mut self, registrar: &mut dyn IPluginRegistrar) {
        // We want to update in the pre update stage and post update stage
        registrar.should_update();

        // We export two interfaces
        registrar.provides_interface::<dyn IWorldProvider>();
        registrar.provides_interface::<dyn IScheduleProvider>();
    }

    fn on_init(&mut self, _registry: &dyn IRegistryAccessor) -> Box<dyn IInitResponse> {
        let response = vec![
            (
                TypeId::of::<dyn IWorldProvider>(),
                AnyArc::map::<dyn IAny, _>(self.world_provider.clone(), |v| v),
            ),
            (
                TypeId::of::<dyn IScheduleProvider>(),
                AnyArc::map::<dyn IAny, _>(self.schedule_provider.clone(), |v| v),
            ),
        ];
        Box::new(response)
    }

    fn on_update(&mut self, _: &dyn IRegistryAccessor) {
        // let world_cell = self.world_provider.get();
        let schedule_cell = self.schedule_provider.get();
        // let mut world = world_cell.get();
        let mut schedule = schedule_cell.get();

        let mut resources = Resources::new();
        schedule.run(&(), &mut resources);
    }
}

interfaces::any::declare_interfaces!(PluginCore, [IPlugin]);

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
enum InternalStage {
    Core,
}

impl InternalStage {
    pub const fn to_label(self) -> Label {
        match self {
            InternalStage::Core => make_label!("aleph-core::InternalStage::Core"),
        }
    }
}

impl Into<Label> for InternalStage {
    #[inline(always)]
    fn into(self) -> Label {
        self.to_label()
    }
}
