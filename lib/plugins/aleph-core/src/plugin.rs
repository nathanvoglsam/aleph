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

use crate::console_provider::ConsoleProvider;
use crate::schedule_provider::ScheduleProvider;
use crate::world_provider::WorldProvider;
use aleph_label::Label;
use aleph_log::LevelFilter;
use interfaces::any::{AnyArc, IAny};
use interfaces::console::{DebugConsole, IDebugConsoleProvider};
use interfaces::make_plugin_description_for_crate;
use interfaces::plugin::{
    IInitResponse, IPlugin, IPluginRegistrar, IRegistryAccessor, PluginDescription,
};
use interfaces::schedule::{CoreStage, IScheduleProvider, Schedule, Stage, SystemSchedule};
use interfaces::world::IWorldProvider;
use std::any::TypeId;
use std::io::BufReader;
use std::net::TcpStream;

pub struct PluginCore {
    world_provider: AnyArc<WorldProvider>,
    schedule_provider: AnyArc<ScheduleProvider>,
    console_provider: AnyArc<ConsoleProvider>,
}

impl PluginCore {
    pub fn new() -> Self {
        // This will be one of the earliest pieces of code to run in aleph engine so initialize the
        // logger here. By initializing it here then this plugin remains optional (technically)
        let env_logger = env_logger::Builder::from_default_env()
            .filter_level(LevelFilter::Trace)
            .build();
        let command_stream = interfaces::console::Logger::from(env_logger).install();

        // Construct the debug console. If the console feature is disabled then this will just make
        // an empty object so we don't need to be conditional here
        let debug_console = DebugConsole::new();

        // Construct a thread that handles reading messages from the remote console and publishes
        // complete command messages to a channel which can be read by the main thread.
        //
        // This transparently handles when the "remote" feature is disabled as Logger::install will
        // just always return None and so this code will never execute.
        let channel = if let Some(command_stream) = command_stream {
            // Construct our channel
            let (channel, receiver) = std::sync::mpsc::sync_channel(1024);

            // Build the persistent thread, sending the channel and command stream over
            std::thread::spawn(move || receiver_thread(channel, BufReader::new(command_stream)));

            Some(receiver)
        } else {
            None
        };

        let mut core_schedule = SystemSchedule::default();

        // Add a system that runs at the absolute very start of the frame for handling rcon
        // commands. No feature gating is needed here as when "remote" is disabled the channel will
        // never be constructed so this will never execute.
        if let Some(channel) = channel {
            let console = debug_console.clone();
            core_schedule.add_exclusive_at_start_system(
                "aleph_core::internal_core_stage",
                move || {
                    while let Ok(message) = channel.try_recv() {
                        console.eval(&message);
                    }
                },
            );
        }

        let mut schedule = Schedule::default();
        schedule.add_stage(InternalStage::Core, core_schedule);
        schedule.add_stage(CoreStage::InputCollection, SystemSchedule::default());
        schedule.add_stage(CoreStage::PreUpdate, SystemSchedule::default());
        schedule.add_stage(CoreStage::Update, SystemSchedule::default());
        schedule.add_stage(CoreStage::PostUpdate, SystemSchedule::default());
        schedule.add_stage(CoreStage::Render, SystemSchedule::default());

        let world_provider = AnyArc::new(WorldProvider::new());
        let schedule_provider = AnyArc::new(ScheduleProvider::new(schedule));
        let console_provider = AnyArc::new(ConsoleProvider::new(debug_console));
        Self {
            world_provider,
            schedule_provider,
            console_provider,
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
        registrar.provides_interface::<dyn IDebugConsoleProvider>();
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
            (
                TypeId::of::<dyn IDebugConsoleProvider>(),
                AnyArc::map::<dyn IAny, _>(self.console_provider.clone(), |v| v),
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

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
enum InternalStage {
    Core,
}

impl Label for InternalStage {
    fn dyn_clone(&self) -> Box<dyn Label> {
        Box::new(*self)
    }
}

fn receiver_thread(channel: std::sync::mpsc::SyncSender<String>, mut stream: BufReader<TcpStream>) {
    use std::io::BufRead;

    let mut buffer = Vec::new();
    loop {
        // Clear the buffer from last iteration
        buffer.clear();

        // All commands are delimited by null bytes, this will read a single well formed message
        // into buffer.
        stream.read_until(b'\0', &mut buffer).unwrap();
        stream.read_until(b'\0', &mut buffer).unwrap();

        // Buffer will contain the delimiters so we strip them
        let slice = buffer.strip_prefix(&[0]).unwrap();
        let slice = slice.strip_suffix(&[0]).unwrap();

        // Then verify the message is valid UTF8
        let string = std::str::from_utf8(slice).unwrap();

        // Finally we can send the command onto the channel
        channel.send(string.to_string()).unwrap();
    }
}
