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

use std::path::Path;

use aleph::interfaces::make_plugin_description_for_crate;
use aleph::interfaces::plugin::{IPlugin, IPluginRegistrar, IRegistryAccessor, PluginDescription};
use aleph::interfaces::renderer::Renderer;
use aleph::interfaces::schedule::CoreStage;
use aleph::Engine;
use aleph_egui::widgets::{frame_stats, FrameTimeHistory};
use aleph_egui::IEguiContextProvider;
use aleph_engine::interfaces::components::{Camera, StaticMesh, Transform, TransformHistory};
use aleph_engine::interfaces::label::make_label;
use aleph_engine::interfaces::math::{DVec3, Rotor3, Vec3};
use aleph_engine::interfaces::platform::{IFrameTimer, IGamepads};
use aleph_engine::interfaces::plugin::{CoreRefs, InitOrder};
use aleph_engine::interfaces::schedule::WorldResource;
use aleph_engine::interfaces::scheduler::ResMut;

use crate::game::async_texture_loader::AsyncTextureLoader;
use crate::game::config::Config;
use crate::game::cube_mesh::upload_cube_buffers;
use crate::game::free_camera::FreeCamera;
use crate::game::gltf_loader::{load_scene, BumpThingy, PollResult};
use crate::game::throbber_logic::ThrobberLogic;

pub fn engine_runner() {
    let mut engine = Engine::builder();
    engine.plugin(aleph_egui::PluginEgui::new());
    engine.plugin(aleph_render::PluginRender::new());
    engine.plugin(PluginGameLogic::new());
    engine.build(|engine| engine.run())
}

struct PluginGameLogic();

aleph::any::declare_interfaces!(PluginGameLogic, [IPlugin]);

impl PluginGameLogic {
    pub fn new() -> Self {
        Self()
    }
}

impl IPlugin for PluginGameLogic {
    fn get_description(&self) -> PluginDescription {
        make_plugin_description_for_crate!()
    }

    fn register(&mut self, registrar: &mut dyn IPluginRegistrar) {
        registrar.requires::<dyn IGamepads>(InitOrder::After);
        registrar.requires::<dyn IFrameTimer>(InitOrder::After);
        registrar.uses::<dyn IEguiContextProvider>(InitOrder::After);
    }

    fn on_init(&mut self, registry: &mut dyn IRegistryAccessor) {
        let config = registry.config("aleph-test").unwrap();
        let config: Config = serde_json::from_value(config.clone()).unwrap();
        config.log();

        let egui_provider = registry.get_interface::<dyn IEguiContextProvider>();
        let frame_timer = registry.get_interface::<dyn IFrameTimer>().unwrap();
        let gamepads = registry.get_interface::<dyn IGamepads>().unwrap();

        let CoreRefs {
            resources,
            schedule,
            world,
        } = registry.core();

        let e_frame_timer = frame_timer.clone();
        let mut frame_time_history = FrameTimeHistory::new();
        schedule.add_exclusive_at_end_system_to_stage(
            CoreStage::Update.into(),
            make_label!("aleph_test::ui"),
            move || {
                if let Some(egui) = egui_provider.as_ref() {
                    let egui_ctx = egui.get_context();

                    let dt = e_frame_timer.delta_time();
                    frame_time_history.next_frame(dt);
                    frame_stats(&egui_ctx, &frame_time_history);
                }
            },
        );

        let camera = world.extend_one((
            Transform {
                position: DVec3::zero(),
                rotation: Rotor3::identity(),
                scale: Vec3::one(),
            },
            Camera {
                vertical_fov: 90.0,
                z_near: 0.1,
            },
        ));

        let renderer = resources.get_mut::<Renderer>().unwrap();

        let async_texture_loader = AsyncTextureLoader::new(
            renderer.device().upgrade(),
            renderer.get_texture_loader_handle(),
        );

        let mut arena = BumpThingy::new(renderer.device());

        let mut thinkers1 = load_scene(
            world,
            renderer,
            &mut arena,
            &async_texture_loader,
            Path::new(&config.scene),
        );

        let (idx, vtx) = upload_cube_buffers(renderer);

        let transform = Transform {
            position: DVec3::zero(),
            rotation: Rotor3::identity(),
            scale: Vec3::one() * 2.0,
        };
        let throbber = world.extend_one((
            transform.clone(),
            TransformHistory {
                previous: transform,
            },
            StaticMesh {
                vtx,
                idx,
                colour_tex: renderer.default_resources().white_texture_rgba8(),
                colour: [0.5, 1.0, 0.5, 1.0],
                metalness: 0.0,
                roughness: 0.5,
                metal_roughness_tex: renderer.default_resources().white_texture_rgba8(),
                normal_map_tex: renderer.default_resources().normal_texture_rgba8(),
            },
        ));

        resources.insert(async_texture_loader);

        let mut free_camera = FreeCamera::new(frame_timer.clone(), gamepads.get_accessor(), camera);
        let throbber_logic = ThrobberLogic::new(frame_timer.clone(), throbber);
        schedule.add_system_to_stage(
            CoreStage::Update.into(),
            make_label!("aleph_test::logic"),
            move |mut world: ResMut<WorldResource>| {
                free_camera.tick(&mut world.0);
                throbber_logic.tick(&mut world.0);
                thinkers1.retain_mut(|t| match t.poll_and_resolve(&mut world.0) {
                    PollResult::Success => false,
                    PollResult::Waiting => true,
                    PollResult::Fail => {
                        log::error!("Thinker Failed!");
                        false
                    }
                });
            },
        );
    }
}
