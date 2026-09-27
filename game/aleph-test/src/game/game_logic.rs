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

use aleph_egui::AEguiContextProvider;
use aleph_egui::widgets::{FrameTimeHistory, MemoryStats, frame_stats, memory_stats};
use aleph_engine::api::components::{Camera, StaticMesh, Transform, TransformHistory};
use aleph_engine::api::label::make_label;
use aleph_engine::api::make_plugin_description_for_crate;
use aleph_engine::api::math::{DVec3, Rotor3, Vec3};
use aleph_engine::api::mg::renderer::Renderer;
use aleph_engine::api::platform::{AFrameTimer, AGamepads};
use aleph_engine::api::plugin::{
    CoreRefs, IPlugin, IPluginRegistrar, IRegistryAccessor, InitOrder, PluginDescription,
};
use aleph_engine::api::schedule::{CoreStage, WorldResource};
use aleph_engine::api::scheduler::ResMut;
use aleph_engine::engine::Engine;
use aleph_engine::render::PluginRender;
use aleph_engine::render::default_resources::DefaultResources;

use crate::game::config::Config;
use crate::game::cube_mesh::upload_cube_buffers;
use crate::game::free_camera::FreeCamera;
use crate::game::throbber_logic::ThrobberLogic;

pub fn engine_runner() {
    let mut engine = Engine::builder();
    engine.plugin(aleph_egui::PluginEgui::new());
    engine.plugin(PluginRender::new());
    engine.plugin(PluginGameLogic::new());
    engine.build().run();
}

struct PluginGameLogic();

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
        registrar.requires::<AGamepads>(InitOrder::After);
        registrar.requires::<AFrameTimer>(InitOrder::After);
        registrar.uses::<AEguiContextProvider>(InitOrder::After);
    }

    fn on_init(&mut self, registry: &mut dyn IRegistryAccessor) {
        let config = registry.config("aleph-test").unwrap();
        let config: Config = serde_json::from_value(config.clone()).unwrap();
        config.log();

        let egui_provider = registry
            .get_interface::<AEguiContextProvider>()
            .map(|v| v.get());
        let frame_timer = registry.get_interface::<AFrameTimer>().unwrap().get();
        let gamepads = registry.get_interface::<AGamepads>().unwrap().get();

        let CoreRefs {
            resources,
            schedule,
            world,
        } = registry.core();

        let e_frame_timer = frame_timer.clone();
        let mut frame_time_history = FrameTimeHistory::new();
        let mut memory_stats_state = MemoryStats::new();
        schedule.add_exclusive_at_end_system_to_stage(
            CoreStage::Update.into(),
            make_label!("aleph_test::ui"),
            move || {
                if let Some(egui) = egui_provider.as_ref() {
                    let egui_ctx = egui.get_context();

                    let dt = e_frame_timer.delta_time();
                    frame_time_history.next_frame(dt);
                    frame_stats(&egui_ctx, &frame_time_history);

                    memory_stats_state.next_frame();
                    memory_stats(&egui_ctx, &mut memory_stats_state);
                }
            },
        );

        let camera = world.insert((
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

        let default_resources = resources.get_ref::<DefaultResources>().unwrap().clone();
        let renderer = resources.get_mut::<Renderer>().unwrap();

        let (idx, vtx) = upload_cube_buffers(renderer);

        let transform = Transform {
            position: DVec3::zero(),
            rotation: Rotor3::identity(),
            scale: Vec3::one() * 2.0,
        };
        let throbber = world.insert((
            transform.clone(),
            TransformHistory {
                previous: transform,
            },
            StaticMesh {
                vtx,
                idx,
                material_instance: default_resources.default_material,
            },
        ));

        let mut free_camera = FreeCamera::new(frame_timer.clone(), gamepads.get_accessor(), camera);
        let throbber_logic = ThrobberLogic::new(frame_timer.clone(), throbber);
        schedule.add_system_to_stage(
            CoreStage::Update.into(),
            make_label!("aleph_test::logic"),
            move |mut world: ResMut<WorldResource>| {
                free_camera.tick(&mut world.0);
                throbber_logic.tick(&mut world.0);
            },
        );
    }
}
