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

use aleph::app_info::AppInfo;
use aleph::{Engine, FrameRate};

struct AlephAppLogic {
    _frame_timer: bool,
    frame_times: FrameRate,
    demo_window: egui_demo_lib::DemoWindows,
    colour_test: egui_demo_lib::ColorTest,
}

impl AlephAppLogic {
    pub fn new() -> Self {
        Self {
            _frame_timer: true,
            frame_times: FrameRate::new(),
            demo_window: Default::default(),
            colour_test: Default::default(),
        }
    }
}

impl aleph::AppLogic for AlephAppLogic {
    fn on_init(&mut self) {}

    fn on_update(&mut self, egui_ctx: &aleph::egui::CtxRef) {
        self.frame_times.update();

        //self.demo_window.ui(egui_ctx);

        aleph::egui::Window::new("Colour Test")
            .collapsible(false)
            .scroll(true)
            .show(egui_ctx, |ui| {
                let mut tex_allocator = None;
                self.colour_test.ui(ui, &mut tex_allocator);
            });

        aleph::egui::Window::new("Settings")
            .collapsible(false)
            .scroll(true)
            .show(egui_ctx, |ui| {
                egui_ctx.settings_ui(ui);
            });
    }

    fn on_exit(&mut self) {}
}

fn main() {
    let app_info = AppInfo {
        name: "AlephTest".to_string(),
        author: "MindSpunk".to_string(),
        major: 0,
        minor: 1,
        patch: 0,
    };
    Engine::start(app_info, AlephAppLogic::new());
}
