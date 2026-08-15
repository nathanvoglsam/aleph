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

use api::components;
use api::components::{Camera, Transform};
use api::ecs::world::query::{Has, Read};
use api::label::{Label, make_label};
use api::schedule::{CoreStage, WorldResource};
use api::scheduler::{ExplicitDependencies, IntoSystem, Res, ResMut, Schedule};
use mg::scene::components::{DynamicObject, PerspectiveCamera, RenderTransform, StaticMesh};

use crate::render::core::resources::render_scene::RenderSceneResource;
use crate::render::core::systems::render::RenderSystem;

pub struct PublishRenderSceneSystem;

impl PublishRenderSceneSystem {
    pub const LABEL: Label = make_label!("render::PublishRenderSceneSystem");

    pub fn register(mut self, schedule: &mut Schedule) {
        let system = move |world: Res<WorldResource>, render_scene: ResMut<RenderSceneResource>| {
            self.run(world, render_scene);
        };
        let system = system.system().runs_before(RenderSystem::LABEL);

        schedule.add_system_to_stage(CoreStage::Render.into(), Self::LABEL, system);
    }

    pub fn run(
        &mut self,
        world: Res<WorldResource>,
        mut render_scene: ResMut<RenderSceneResource>,
    ) {
        let render_scene = &mut render_scene.scene;

        render_scene.remove_matching::<Has<DynamicObject>>();

        let query = world
            .0
            .query::<(Read<Transform>, Read<components::StaticMesh>)>();
        for (_id, (t, m)) in query {
            render_scene.insert((
                RenderTransform {
                    position: t.position,
                    rotation: t.rotation,
                    scale: t.scale,
                },
                StaticMesh {
                    vtx: Some(m.vtx),
                    idx: Some(m.idx),
                    material_instance: Some(m.material_instance),
                },
                DynamicObject,
            ));
        }

        // Find the first camera object in the scene and make that the active camera.
        let mut query = world.0.query::<(Read<Transform>, Read<Camera>)>();
        let (_, (t, c)) = query.next().unwrap();

        let _ = render_scene.insert((
            RenderTransform {
                position: t.position,
                rotation: t.rotation,
                scale: t.scale,
            },
            PerspectiveCamera {
                vertical_fov: c.vertical_fov,
                z_near: c.z_near,
            },
            DynamicObject,
        ));
    }
}
