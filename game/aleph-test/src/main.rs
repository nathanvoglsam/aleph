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

#![cfg_attr(target_vendor = "uwp", windows_subsystem = "windows")]

extern crate aleph_engine as aleph;
extern crate egui_demo_lib;

use aleph::egui::IEguiContextProvider;
use aleph::plugin_registry::interfaces::any::AnyArc;
use aleph::plugin_registry::interfaces::plugin::{
    IInitResponse, IPlugin, IPluginRegistrar, IRegistryAccessor, PluginDescription, UpdateStage,
};
use aleph::Engine;

struct AlephAppLogic {
    demo_window: egui_demo_lib::DemoWindows,
    colour_test: egui_demo_lib::ColorTest,
    egui_provider: Option<AnyArc<dyn IEguiContextProvider>>,
}

impl AlephAppLogic {
    pub fn new() -> Self {
        Self {
            demo_window: Default::default(),
            colour_test: Default::default(),
            egui_provider: None,
        }
    }
}

impl IPlugin for AlephAppLogic {
    fn get_description(&self) -> PluginDescription {
        PluginDescription {
            name: "test-game-app-logic".to_string(),
            description: "The game logic implementation for test-game".to_string(),
            major_version: 0,
            minor_version: 1,
            patch_version: 0,
        }
    }

    fn register(&mut self, registrar: &mut dyn IPluginRegistrar) {
        registrar.update_stage(UpdateStage::Update);

        registrar.depends_on::<dyn IEguiContextProvider>();
        registrar.must_init_after::<dyn IEguiContextProvider>();
    }

    fn on_init(&mut self, registry: &dyn IRegistryAccessor) -> Box<dyn IInitResponse> {
        let egui_provider = registry
            .get_interface::<dyn IEguiContextProvider>()
            .unwrap();
        self.egui_provider = Some(egui_provider);

        Box::new(Vec::new())
    }

    fn on_update(&mut self, _registry: &dyn IRegistryAccessor) {
        let egui_ctx = self.egui_provider.as_ref().unwrap().get_context();

        self.demo_window.ui(&egui_ctx);

        aleph::egui::Window::new("Colour Test")
            .collapsible(false)
            .scroll(true)
            .show(&egui_ctx, |ui| {
                let mut tex_allocator = None;
                self.colour_test.ui(ui, &mut tex_allocator);
            });

        aleph::egui::Window::new("Settings")
            .collapsible(false)
            .scroll(true)
            .show(&egui_ctx, |ui| {
                egui_ctx.settings_ui(ui);
            });
    }
}

aleph::plugin_registry::interfaces::any::declare_interfaces!(AlephAppLogic, [IPlugin]);

fn main() {
    let mut engine = Engine::builder();
    engine.default_plugins();
    engine.plugin(AlephAppLogic::new());
    engine.build().run()
}
