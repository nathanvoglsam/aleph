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

use std::collections::{VecDeque, vec_deque};

use aleph_alloc::instrumentation::{
    AllocationCategoryIter, IAllocationCategory, get_allocated_bytes,
};
use egui::{CollapsingHeader, Context, Grid, Window};
use egui_plot::{Bar, BarChart, Legend, Plot};

use crate::Egui;

pub struct FrameTimeHistory {
    data: VecDeque<f64>,
}

impl FrameTimeHistory {
    pub fn new() -> Self {
        Self {
            data: VecDeque::from([0.16; 128]),
        }
    }

    pub fn next_frame(&mut self, v: f64) {
        self.data.pop_front();
        self.data.push_back(v);
    }

    pub fn latest(&self) -> f64 {
        self.data.back().copied().unwrap()
    }

    pub fn iter(&self) -> vec_deque::Iter<'_, f64> {
        self.data.iter()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn average_over_window(&self, window: usize) -> f64 {
        // Clamp the mean window to the size of our history window
        let window = window.max(self.len());

        let sum: f64 = self.data.iter().rev().take(window).sum();
        let mean = sum / window as f64;
        mean
    }
}

pub fn frame_stats(ctx: &Context, frame_time_history: &FrameTimeHistory) {
    let f = || {
        Window::new("FrameStats")
            .id("frame-time-window".into())
            .collapsible(true)
            .resizable([false, false])
            .show(&ctx, |ui| {
                ui.heading("Frame Time");
                ui.separator();

                // Take the average framerate over the last 8 frames so we get a smoother value
                let mean_dt = frame_time_history.average_over_window(8);
                let mean_fr = 1.0 / mean_dt;
                let mean_dt_ms = mean_dt * 1000.0;

                Grid::new("frame-time-grid")
                    .num_columns(2)
                    .min_col_width(75.0)
                    .show(ui, |ui| {
                        ui.label(format!("FPS {mean_fr:.3}"));
                        ui.label(format!("{mean_dt_ms:.4}ms"));
                    });
                CollapsingHeader::new("Frame Time History")
                    .default_open(true)
                    .show(ui, |ui| {
                        let _ = Plot::new("Frame Time Plot")
                            .legend(Legend::default())
                            .include_y(0.0) // Anchor 0 on the y axis
                            .include_y(40.0) // Anchor 40ms on the y axis
                            .y_axis_label("Frame Time")
                            .show_axes([false, true])
                            .show_grid(true)
                            .allow_zoom(false)
                            .allow_drag(false)
                            .allow_boxed_zoom(false)
                            .allow_scroll(false)
                            .allow_double_click_reset(false)
                            .height(200.0)
                            .show(ui, |ui| {
                                let bars: Vec<Bar> = frame_time_history
                                    .iter()
                                    .enumerate()
                                    .map(|(i, v)| Bar::new(i as _, v * 1000.0))
                                    .collect();
                                let chart = BarChart::new("Frame Time", bars).name("Frame Time");
                                ui.bar_chart(chart);
                            });
                    });
            });
    };
    Egui::with(f);
}

pub struct MemoryHistory {
    data: VecDeque<usize>,
}

impl MemoryHistory {
    pub fn new() -> Self {
        Self {
            data: VecDeque::from([0; 128]),
        }
    }

    pub fn next_frame(&mut self, v: usize) {
        self.data.pop_front();
        self.data.push_back(v);
    }

    pub fn latest(&self) -> usize {
        self.data.back().copied().unwrap()
    }

    pub fn iter(&self) -> vec_deque::Iter<'_, usize> {
        self.data.iter()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

pub fn memory_stats(ctx: &Context, history: &MemoryHistory) {
    let f = || {
        Window::new("Memory Stats")
            .id("memory-stats-window".into())
            .collapsible(true)
            .resizable([false, false])
            .show(&ctx, |ui| {
                ui.heading("Memory Stats");
                ui.separator();

                Grid::new("memory-stats-grid")
                    .num_columns(2)
                    .min_col_width(75.0)
                    .show(ui, |ui| {
                        let b = get_allocated_bytes();
                        let kb = b / 1_000;
                        ui.label("System:");
                        ui.label(format!("{kb}KB"));
                        ui.end_row();

                        for cat in AllocationCategoryIter::new() {
                            let b = cat.allocated();
                            let kb = b / 1_000;
                            ui.label(format!("Category: {}", cat.name()));
                            ui.label(format!("{kb}KB"));
                            ui.end_row();
                        }
                    });
                CollapsingHeader::new("Memory History")
                    .default_open(true)
                    .show(ui, |ui| {
                        let _ = Plot::new("Memory Usage Plot")
                            .legend(Legend::default())
                            .y_axis_label("Memory Usage")
                            .show_axes([false, true])
                            .show_grid(true)
                            .allow_zoom(false)
                            .allow_drag(false)
                            .allow_boxed_zoom(false)
                            .allow_scroll(false)
                            .allow_double_click_reset(false)
                            .height(200.0)
                            .show(ui, |ui| {
                                let bars: Vec<Bar> = history
                                    .iter()
                                    .enumerate()
                                    .map(|(i, &v)| Bar::new(i as _, v as f64))
                                    .collect();
                                let chart = BarChart::new("Frame Time", bars).name("Frame Time");
                                ui.bar_chart(chart);
                            });
                    });
            });
    };
    Egui::with(f);
}
