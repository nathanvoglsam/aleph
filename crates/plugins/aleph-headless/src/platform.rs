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
    IInitResponse, IPlugin, IPluginRegistrar, IRegistryAccessor, PluginDescription, UpdateStage,
};
use std::any::TypeId;

pub struct PluginPlatformHeadless {
    provider: AnyArc<ProviderImpl>,
}

impl PluginPlatformHeadless {
    pub fn new() -> Self {
        Self {
            provider: AnyArc::new(ProviderImpl {
                frame_timer: FrameTimerImpl::new(),
            }),
        }
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
        registrar.provides_interface::<dyn IFrameTimerProvider>();
        registrar.update_stage(UpdateStage::InputCollection);
    }

    fn on_init(&mut self, registry: &dyn IRegistryAccessor) -> Box<dyn IInitResponse> {
        let quit_handle = registry.quit_handle();
        ctrlc::set_handler(move || {
            println!();
            quit_handle.quit()
        })
        .expect("Failed to registr ctrl+c handler");

        // Provide our declared implementations to the plugin registry
        let response = vec![(
            TypeId::of::<dyn IFrameTimerProvider>(),
            AnyArc::into_any(self.provider.clone()),
        )];
        Box::new(response)
    }

    fn on_input_collection(&mut self, _registry: &dyn IRegistryAccessor) {
        self.provider.frame_timer.update();
    }
}

interfaces::any::declare_interfaces!(PluginPlatformHeadless, [IPlugin]);
