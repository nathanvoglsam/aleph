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
use api::scheduler::ResMut;
use mg::renderer::Renderer;

use crate::render::egui::egui_pass::EguiPassContext;
use crate::render::egui::font_texture::EguiFontTexture;
use crate::render::resources::render_scene::RenderSceneResource;

pub struct PublishEguiSceneSystem {
    pub font_texture: EguiFontTexture,
    pub render_data: AnyArc<dyn egui::IEguiRenderData>,
}

impl PublishEguiSceneSystem {
    pub fn run(
        &mut self,
        mut renderer: ResMut<Renderer>,
        mut render_scene: ResMut<RenderSceneResource>,
    ) {
        let render_scene = &mut render_scene.scene;

        let render_data = self.render_data.take();

        // Filter the deltas to only those that affect the font texture and upload
        // a new font texture immediately.
        let font_updates = render_data
            .textures_delta
            .set
            .iter()
            .filter(|(id, _)| *id == egui::TextureId::Managed(0))
            .map(|(_, delta)| delta);
        self.font_texture
            .update_font_texture(&mut renderer, font_updates);

        // Pass the egui commands and font texture that so our egui render pass
        // can do its thing.
        let _ = render_scene.insert_singleton(EguiPassContext {
            font_handle: self.font_texture.font_handle.unwrap(),
            render_data,
        });
    }
}
