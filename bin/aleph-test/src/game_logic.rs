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

extern crate aleph_engine as aleph;
extern crate egui_demo_lib;

use aleph::egui::IEguiContextProvider;
use aleph::interfaces::make_plugin_description_for_crate;
use aleph::interfaces::plugin::{
    IInitResponse, IPlugin, IPluginRegistrar, IRegistryAccessor, PluginDescription,
};
use aleph::interfaces::schedule::{CoreStage, IScheduleProvider};
use aleph::Engine;

struct PluginGameLogic();

impl PluginGameLogic {
    pub fn new() -> Self {
        Self()
    }
}

impl IPlugin for PluginGameLogic {
    fn get_description(&self) -> PluginDescription {
        make_plugin_description_for_crate!()
    }

    fn register(&mut self, registrar: &mut dyn IPluginRegistrar) {
        registrar.depends_on::<dyn IScheduleProvider>();
        registrar.must_init_after::<dyn IScheduleProvider>();

        //registrar.depends_on::<dyn IEguiContextProvider>();
        registrar.must_init_after::<dyn IEguiContextProvider>();
    }

    fn on_init(&mut self, registry: &dyn IRegistryAccessor) -> Box<dyn IInitResponse> {
        let mut demo_window = egui_demo_lib::DemoWindows::default();
        let mut colour_test = egui_demo_lib::ColorTest::default();

        let egui_provider = registry.get_interface::<dyn IEguiContextProvider>();

        let schedule_provider = registry.get_interface::<dyn IScheduleProvider>().unwrap();
        let schedule_cell = schedule_provider.get();
        let mut schedule = schedule_cell.get();

        schedule.add_exclusive_at_start_system_to_stage(
            &CoreStage::Render,
            "platform_headless::input_collection",
            move || {
                if let Some(egui) = egui_provider.as_ref() {
                    let egui_ctx = egui.get_context();

                    demo_window.ui(&egui_ctx);

                    aleph::egui::Window::new("Colour Test")
                        .collapsible(true)
                        .hscroll(true)
                        .show(&egui_ctx, |ui| {
                            colour_test.ui(ui);
                        });

                    aleph::egui::Window::new("Settings")
                        .collapsible(true)
                        .hscroll(true)
                        .show(&egui_ctx, |ui| {
                            egui_ctx.settings_ui(ui);
                        });
                }
            },
        );

        Box::new(Vec::new())
    }
}

aleph::any::declare_interfaces!(PluginGameLogic, [IPlugin]);

pub fn engine_runner() {
    let mut engine = Engine::builder();
    engine.default_plugins(false);
    engine.plugin(PluginGameLogic::new());
    engine.build().run()
}
