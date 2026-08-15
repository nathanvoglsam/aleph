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

use std::sync::Arc;

use api::label::{Label, make_label};
use api::platform::IWindow;
use api::schedule::CoreStage;
use api::scheduler::Schedule;
use crossbeam::channel::Sender;
use mg::renderer::surface_notify::SurfaceNotification;

pub struct SurfaceSenderSystem {
    window: Arc<dyn IWindow>,
    sender: Sender<SurfaceNotification>,
}

impl SurfaceSenderSystem {
    pub const LABEL: Label = make_label!("render::SurfaceSenderSystem");

    pub fn new(window: Arc<dyn IWindow>, sender: Sender<SurfaceNotification>) -> Self {
        Self { window, sender }
    }

    pub fn register(self, schedule: &mut Schedule) {
        let system = move || {
            self.run();
        };
        schedule.add_exclusive_at_end_system_to_stage(
            CoreStage::InputCollection.into(),
            Self::LABEL,
            system,
        );
    }

    pub fn run(&self) {
        if self.window.resized() {
            let size = self.window.drawable_size();
            let size = rhi::Extent2D::new(size.0, size.1);
            self.sender
                .try_send(SurfaceNotification::Resized(size))
                .ok()
                .unwrap()
        }
    }
}
