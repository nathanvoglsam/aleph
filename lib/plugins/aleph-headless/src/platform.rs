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

use crate::frame_timer::FrameTimerImpl;
use crate::provider::ProviderImpl;
use interfaces::any::AnyArc;
use interfaces::platform::IFrameTimerProvider;
use interfaces::plugin::{
    IInitResponse, IPlugin, IPluginRegistrar, IRegistryAccessor, PluginDescription,
};
use interfaces::schedule::{CoreStage, IScheduleProvider};
use std::any::TypeId;

pub struct PluginPlatformHeadless();

impl PluginPlatformHeadless {
    pub fn new() -> Self {
        Self()
    }
}

impl IPlugin for PluginPlatformHeadless {
    fn get_description(&self) -> PluginDescription {
        PluginDescription {
            name: "PluginPlatformHeadless".to_string(),
            description: "A platform abstraction layer for running headless".to_string(),
            major_version: 0,
            minor_version: 1,
            patch_version: 0,
        }
    }

    fn register(&mut self, registrar: &mut dyn IPluginRegistrar) {
        registrar.must_init_after::<dyn IScheduleProvider>();
        registrar.depends_on::<dyn IScheduleProvider>();

        registrar.provides_interface::<dyn IFrameTimerProvider>();
    }

    fn on_init(&mut self, registry: &dyn IRegistryAccessor) -> Box<dyn IInitResponse> {
        let provider = AnyArc::new(ProviderImpl {
            frame_timer: FrameTimerImpl::new(),
        });

        let quit_handle = registry.quit_handle();
        ctrlc::set_handler(move || {
            println!();
            quit_handle.quit()
        })
        .expect("Failed to registr ctrl+c handler");

        let schedule_provider = registry.get_interface::<dyn IScheduleProvider>().unwrap();
        let schedule_cell = schedule_provider.get();
        let mut schedule = schedule_cell.get();

        let send_provider = provider.clone();
        schedule.add_system_to_stage(
            &CoreStage::InputCollection,
            "platform_headless::input_collection",
            move || {
                send_provider.frame_timer.update();
            },
        );

        // Provide our declared implementations to the plugin registry
        let response = vec![(
            TypeId::of::<dyn IFrameTimerProvider>(),
            AnyArc::into_any(provider),
        )];
        Box::new(response)
    }
}

impl Default for PluginPlatformHeadless {
    fn default() -> Self {
        Self::new()
    }
}

interfaces::any::declare_interfaces!(PluginPlatformHeadless, [IPlugin]);
