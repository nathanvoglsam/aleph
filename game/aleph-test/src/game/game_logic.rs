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
extern crate egui_demo_lib;

use aleph::egui::IEguiContextProvider;
use aleph::interfaces::make_plugin_description_for_crate;
use aleph::interfaces::plugin::{
    IInitResponse, IPlugin, IPluginRegistrar, IRegistryAccessor, PluginDescription,
};
use aleph::interfaces::renderer::Renderer;
use aleph::interfaces::schedule::CoreStage;
use aleph::Engine;
use aleph_engine::interfaces::components::{Camera, StaticMesh, Transform};
use aleph_engine::interfaces::label::make_label;
use aleph_engine::interfaces::math::{DVec3, Rotor3, Vec3};
use aleph_engine::interfaces::platform::{IFrameTimerProvider, IGamepadsProvider};
use aleph_engine::interfaces::schedule::WorldResource;
use aleph_engine::interfaces::scheduler::ResMut;

use crate::game::cube_mesh::upload_cube_buffers;
use crate::game::free_camera::FreeCamera;
use crate::game::gltf_loader::load_scene;
use crate::game::throbber_logic::ThrobberLogic;

pub fn engine_runner() {
    let mut engine = Engine::builder();
    engine.default_plugins();
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
        registrar.depends_on::<dyn IGamepadsProvider>();
        registrar.must_init_after::<dyn IGamepadsProvider>();

        registrar.depends_on::<dyn IFrameTimerProvider>();
        registrar.must_init_after::<dyn IFrameTimerProvider>();

        //registrar.depends_on::<dyn IEguiContextProvider>();
        registrar.must_init_after::<dyn IEguiContextProvider>();
    }

    fn on_init(&mut self, registry: &mut dyn IRegistryAccessor) -> Box<dyn IInitResponse> {
        let world = registry.world();

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

        let mut demo_window = egui_demo_lib::DemoWindows::default();
        let mut colour_test = egui_demo_lib::ColorTest::default();
        let egui_provider = registry.get_interface::<dyn IEguiContextProvider>();
        registry.schedule().add_exclusive_at_end_system_to_stage(
            CoreStage::Update.into(),
            make_label!("aleph_test::ui"),
            move || {
                if let Some(egui) = egui_provider.as_ref() {
                    let egui_ctx = egui.get_context();

                    demo_window.ui(&egui_ctx);

                    aleph::egui::Window::new("Colour Test")
                        .collapsible(true)
                        .hscroll(true)
                        .show(&egui_ctx, |ui| {
                            colour_test.ui(ui);
                        });

                    aleph::egui::Window::new("Settings")
                        .collapsible(true)
                        .hscroll(true)
                        .show(&egui_ctx, |ui| {
                            egui_ctx.settings_ui(ui);
                        });
                }
            },
        );

        let frame_timer = registry
            .get_interface::<dyn IFrameTimerProvider>()
            .unwrap()
            .get_frame_timer()
            .unwrap();

        let gamepads = registry
            .get_interface::<dyn IGamepadsProvider>()
            .unwrap()
            .get_gamepads()
            .unwrap();

        let mut free_camera = FreeCamera::new(frame_timer.clone(), gamepads.get_accessor(), camera);
        let mut throbber_logic = ThrobberLogic::new(frame_timer.clone());
        let mut should_init = true;
        registry.schedule().add_system_to_stage(
            CoreStage::Update.into(),
            make_label!("aleph_test::logic"),
            move |mut world: ResMut<WorldResource>, mut renderer: ResMut<Renderer>| {
                if should_init {
                    should_init = false;
                    load_scene(
                        &mut world.0,
                        &mut renderer,
                        "E:\\Files\\ORCA\\IntelSponza\\Main\\NewSponza_Main_Blender_glTF.gltf",
                    );
                    load_scene(&mut world.0, &mut renderer, "OrientationTest.gltf");

                    let (idx, vtx) = upload_cube_buffers(&mut renderer);

                    throbber_logic.throbber = Some(world.0.extend_one((
                        Transform {
                            position: DVec3::zero(),
                            rotation: Rotor3::identity(),
                            scale: Vec3::one() * 2.0,
                        },
                        StaticMesh {
                            vtx,
                            idx,
                            colour_tex: renderer.default_resources().white_texture_rgba8(),
                            colour: [0.5, 1.0, 0.5, 1.0],
                            metalness: 0.0,
                            roughness: 0.5,
                            metal_roughness_tex: renderer.default_resources().white_texture_rgba8(),
                        },
                    )));
                }
                free_camera.tick(&mut world.0);
                throbber_logic.tick(&mut world.0);
            },
        );

        Box::new(Vec::new())
    }
}
