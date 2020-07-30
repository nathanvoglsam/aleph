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

use aleph::app_info::AppInfo;
use aleph::imgui::{im_str, Condition, MenuItem, Ui};
use aleph::platform::window::Window;
use aleph::{Engine, FrameRate};

struct AlephAppLogic {
    frame_timer: bool,
    frame_times: FrameRate,
}

impl AlephAppLogic {
    pub fn new() -> Self {
        Self {
            frame_timer: true,
            frame_times: FrameRate::new(),
        }
    }
}

impl aleph::AppLogic for AlephAppLogic {
    fn on_init(&mut self) {}

    fn on_update(&mut self, ui: &Ui) {
        self.frame_times.update();

        //aleph::imgui::Window::new(im_str!("MainWindow"))
        //    .size(
        //        [Window::width() as f32, Window::height() as f32],
        //        Condition::Always,
        //    )
        //    .position([0.0, 0.0], Condition::Always)
        //    .resizable(false)
        //    .title_bar(false)
        //    .menu_bar(true)
        //    .bring_to_front_on_focus(false)
        //    .build(ui, || {
        //        ui.menu_bar(|| self.menu_bar(ui));
        //        let token = ui.push_font(ui.fonts().fonts()[3]);
        //        ui.text(im_str!("AlephEngine test"));
        //        token.pop(ui);
        //        ui.separator();
        //    });

        let mut frame_timer_open = self.frame_timer;
        if frame_timer_open {
            aleph::imgui::Window::new(im_str!("Frame Time Graph"))
                .opened(&mut frame_timer_open)
                .size([430.0, 250.0], Condition::Always)
                .collapsible(false)
                .resizable(false)
                .build(ui, || {
                    let token = ui.push_font(ui.fonts().fonts()[1]);
                    ui.text(im_str!("Frame Times"));
                    token.pop(ui);
                    ui.plot_lines(im_str!(""), self.frame_times.frame_time_history())
                        .scale_min(0.0)
                        .scale_max(1.0 / 30.0)
                        .graph_size([ui.window_size()[0], 100.0])
                        .build();
                    ui.separator();

                    ui.text("Frame Time (ms): ");
                    ui.same_line(0.0);
                    ui.text(format!("{}", self.frame_times.frame_time()));

                    ui.text("Frame Rate (ms): ");
                    ui.same_line(0.0);
                    ui.text(format!("{}", self.frame_times.frame_rate()));
                });
        }
        self.frame_timer = frame_timer_open;
    }

    fn on_exit(&mut self) {}
}

impl AlephAppLogic {
    fn menu_bar(&mut self, ui: &Ui) {
        ui.menu(im_str!("File"), true, || self.file_menu(ui));
        ui.menu(im_str!("Edit"), true, || self.edit_menu(ui));
        ui.menu(im_str!("View"), true, || self.view_menu(ui));
    }

    fn file_menu(&mut self, ui: &Ui) {
        let item = MenuItem::new(im_str!("Open"));
        if item.build(ui) {
            aleph::log::info!("Open");
        }
        ui.separator();

        let item = MenuItem::new(im_str!("Exit"));
        if item.build(ui) {
            Engine::exit();
        }
    }

    fn edit_menu(&mut self, _ui: &Ui) {}

    fn view_menu(&mut self, ui: &Ui) {
        let item = MenuItem::new(im_str!("Frame Timer"));
        if item.build(ui) {
            self.frame_timer = !self.frame_timer;
        }
    }
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
