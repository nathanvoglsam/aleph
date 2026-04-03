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

use api::any::AnyArc;
use api::ecs::world::query::Read;
use api::scheduler::ResMut;
use mg::renderer::Renderer;
use mg::renderer::draw_options::DrawOptions;
use mg::scene::components::{PerspectiveCamera, RenderTransform};

use crate::render::config::Config;
use crate::render::resources::render_scene::RenderSceneResource;

pub struct RenderSystem {
    pub device: AnyArc<dyn rhi::IDevice>,
    pub render_config: Config,
}

impl RenderSystem {
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
