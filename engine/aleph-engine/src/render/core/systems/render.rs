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

use api::ecs::world::query::Read;
use api::label::{Label, make_label};
use api::schedule::CoreStage;
use api::scheduler::{ResMut, Schedule};
use mg::renderer::Renderer;
use mg::renderer::draw_options::DrawOptions;
use mg::scene::components::{PerspectiveCamera, RenderTransform};

use crate::render::config::Config;
use crate::render::core::resources::render_scene::RenderSceneResource;

pub struct RenderSystem {
    device: Arc<dyn rhi::IDevice>,
    render_config: Config,
}

impl RenderSystem {
    pub const LABEL: Label = make_label!("render::RenderSystem");

    pub fn new(device: Arc<dyn rhi::IDevice>, render_config: &Config) -> Self {
        Self {
            device,
            render_config: render_config.clone(),
        }
    }

    pub fn register(mut self, schedule: &mut Schedule) {
        let system = move |renderer: ResMut<Renderer>,
                           render_scene: ResMut<RenderSceneResource>| {
            self.run(renderer, render_scene);
        };
        schedule.add_system_to_stage(CoreStage::Render.into(), Self::LABEL, system);
    }

    pub fn run(
        &mut self,
        mut renderer: ResMut<Renderer>,
        mut render_scene: ResMut<RenderSceneResource>,
    ) {
        self.device.garbage_collect().unwrap();

        // Find the first camera object in the scene and make that the active camera.
        let mut query = render_scene
            .scene
            .query::<(Read<RenderTransform>, Read<PerspectiveCamera>)>();
        let (camera_entity, _) = query.next().unwrap();

        let options = DrawOptions {
            force_rebuild_frame_graph: self.render_config.force_graph_rebuild,
        };
        renderer.draw_frame(&options, &mut render_scene.scene, camera_entity);
    }
}
