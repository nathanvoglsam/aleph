//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

extern crate aleph_engine as aleph;

use aleph::app::{AppInfo, FrameTimer, Window};
use aleph::imgui::{im_str, MenuItem, Ui};
use aleph::imgui::{Condition, ImString};

struct AlephAppLogic {
    frame_timer: bool,
    frame_times: [f32; 128],
    sleep_micro: i32,
}

impl AlephAppLogic {
    pub fn new() -> Self {
        Self {
            frame_timer: false,
            frame_times: [0.0; 128],
            sleep_micro: 5000,
        }
    }
}

impl aleph::app::AppLogic for AlephAppLogic {
    fn on_init(&mut self) {}

    fn on_update(&mut self, ui: &Ui) {
        aleph::imgui::Window::new(im_str!("MainWindow"))
            .size(
                [Window::width() as f32, Window::height() as f32],
                Condition::Always,
            )
            .position([0.0, 0.0], Condition::Always)
            .resizable(false)
            .title_bar(false)
            .menu_bar(true)
            .bring_to_front_on_focus(false)
            .build(ui, || {
                ui.menu_bar(|| self.menu_bar(ui));
                ui.text(im_str!("AlephEngine test"));
                ui.separator();
            });

        std::thread::sleep(std::time::Duration::from_micros(self.sleep_micro as u64));

        for i in 1..self.frame_times.len() {
            self.frame_times[i - 1] = self.frame_times[i];
        }
        *self.frame_times.last_mut().unwrap() = FrameTimer::delta_time() as f32;

        let mut frame_time = 0.0;
        self.frame_times.iter().for_each(|v| {
            frame_time += *v;
        });
        frame_time /= self.frame_times.len() as f32;

        let mut frame_timer_open = self.frame_timer;
        if frame_timer_open {
            aleph::imgui::Window::new(im_str!("Frame Time Graph"))
                .opened(&mut frame_timer_open)
                .size([400.0, 300.0], Condition::Always)
                .collapsible(false)
                .resizable(false)
                .build(ui, || {
                    ui.text(im_str!("Frame Times"));
                    ui.plot_lines(im_str!(""), &self.frame_times)
                        .scale_min(0.0)
                        .scale_max(1.0 / 30.0)
                        .graph_size([ui.window_size()[0], 100.0])
                        .build();
                    ui.separator();
                    ui.label_text(
                        im_str!("Frame Time (ms)"),
                        &ImString::new(format!("{}", frame_time)),
                    );
                    ui.label_text(
                        im_str!("Frame Rate (fps)"),
                        &ImString::new(format!("{}", 1.0 / frame_time)),
                    );
                    ui.drag_int(im_str!("Sleep Time"), &mut self.sleep_micro)
                        .min(1)
                        .max(16000)
                        .speed(25.0)
                        .build();
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
            aleph::log::info!("Exit");
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
    aleph::app::Engine::start(app_info, AlephAppLogic::new());
}
