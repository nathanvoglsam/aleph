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

use aleph_frame_graph::FrameGraphBuilder;
use api::platform::IWindow;
use mg::renderer::frame_graph::GraphArgs;
use mg::renderer::immediate_resource_builder::ImmediateResourceBuilder;
use mg::renderer::render_plane::{IRenderPlane, RenderPlaneOutput};
use mg::renderer::state_cache::StateCache;

use crate::render::egui::egui_pass;

pub struct EguiRenderPlane {
    window: Arc<dyn IWindow>,
}

impl EguiRenderPlane {
    pub fn new(window: Arc<dyn IWindow>) -> Self {
        Self { window }
    }
}

impl IRenderPlane for EguiRenderPlane {
    fn init_resources(&mut self, _resource_builder: &mut ImmediateResourceBuilder) {
        // nothing
    }

    fn register_passes(
        &self,
        frame_graph: &mut FrameGraphBuilder<GraphArgs>,
        device: &dyn rhi::IDevice,
        pin_board: &aleph_pin_board::PinBoard,
        state_cache: &mut StateCache,
    ) -> RenderPlaneOutput {
        let pixels_per_point = self.window.current_display_scale();
        egui_pass::pass(
            frame_graph,
            device,
            pin_board,
            state_cache,
            pixels_per_point,
        )
    }
}
