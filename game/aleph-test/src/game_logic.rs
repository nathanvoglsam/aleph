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
use aleph::interfaces::schedule::CoreStage;
use aleph::Engine;
use aleph_engine::interfaces::components::{Camera, StaticMesh, Transform};
use aleph_engine::interfaces::label::make_label;
use aleph_engine::interfaces::math::{DVec3, Rotor3, ToDouble, Vec3};
use aleph_engine::interfaces::platform::{GamepadAxis, IFrameTimerProvider, IGamepadsProvider};
use aleph_engine::interfaces::schedule::WorldResource;
use aleph_engine::interfaces::scheduler::ResMut;

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
        registrar.depends_on::<dyn IGamepadsProvider>();
        registrar.must_init_after::<dyn IGamepadsProvider>();

        registrar.depends_on::<dyn IFrameTimerProvider>();
        registrar.must_init_after::<dyn IFrameTimerProvider>();

        //registrar.depends_on::<dyn IEguiContextProvider>();
        registrar.must_init_after::<dyn IEguiContextProvider>();
    }

    fn on_init(&mut self, registry: &mut dyn IRegistryAccessor) -> Box<dyn IInitResponse> {
        let mut demo_window = egui_demo_lib::DemoWindows::default();
        let mut colour_test = egui_demo_lib::ColorTest::default();

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

        let gamepad = gamepads.get_accessor();

        let egui_provider = registry.get_interface::<dyn IEguiContextProvider>();

        let world = registry.world();
        world.extend_discard((
            [
                Transform {
                    position: DVec3::new(0.0, 0.0, -3.0),
                    rotation: Rotor3::identity(),
                    scale: Vec3::one(),
                },
                Transform {
                    position: DVec3::new(-3.0, 0.0, -3.0),
                    rotation: Rotor3::identity(),
                    scale: Vec3::one(),
                },
                Transform {
                    position: DVec3::new(3.0, 0.0, -3.0),
                    rotation: Rotor3::identity(),
                    scale: Vec3::one(),
                },
                Transform {
                    position: DVec3::new(0.0, 3.0, -3.0),
                    rotation: Rotor3::identity(),
                    scale: Vec3::one(),
                },
                Transform {
                    position: DVec3::new(0.0, -3.0, -3.0),
                    rotation: Rotor3::identity(),
                    scale: Vec3::one(),
                },
            ],
            [
                StaticMesh(0),
                StaticMesh(0),
                StaticMesh(0),
                StaticMesh(0),
                StaticMesh(0),
            ],
        ));

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

        let throbber = world.extend_one((
            Transform {
                position: DVec3::zero(),
                rotation: Rotor3::identity(),
                scale: Vec3::one(),
            },
            StaticMesh(0),
        ));

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

        let mut l_dir = 0.0f32;
        let mut u_dir = 0.0f32;
        registry.schedule().add_system_to_stage(
            CoreStage::Update.into(),
            make_label!("aleph_test::logic"),
            move |mut world: ResMut<WorldResource>| {
                let delta = frame_timer.delta_time();
                let elapsed = frame_timer.elapsed_time();

                if let Some(state) = gamepad.get_active_controller_state() {
                    let (transform, _camera) = world
                        .0
                        .query_one_mut::<(&mut Transform, &Camera)>(camera)
                        .unwrap();

                    let l = state.axis(GamepadAxis::RightX);
                    let u = state.axis(GamepadAxis::RightY);
                    let l = deadzone(unorm_i16_to_f32(l), 0.1);
                    let u = deadzone(unorm_i16_to_f32(u), 0.1);
                    l_dir += l * (90.0f32 * delta as f32);
                    u_dir -= u * (90.0f32 * delta as f32);
                    let yaw = Rotor3::from_rotation_xz(l_dir.to_radians());
                    let pitch = Rotor3::from_rotation_yz(u_dir.to_radians());
                    transform.rotation = yaw * pitch;

                    let x = state.axis(GamepadAxis::LeftX);
                    let z = state.axis(GamepadAxis::LeftY);
                    let x = deadzone(unorm_i16_to_f32(x), 0.1);
                    let z = deadzone(unorm_i16_to_f32(z), 0.1);

                    // Get the camera's direction vectors
                    let cam_forward = transform.rotation
                        * (-Vec3::unit_z())
                        * Vec3::broadcast(z * (4.0 * delta as f32));
                    let cam_right = transform.rotation
                        * (-Vec3::unit_x())
                        * Vec3::broadcast(x * (4.0 * delta as f32));

                    transform.position -= cam_forward.to_double();
                    transform.position -= cam_right.to_double();

                    let yd = state.axis(GamepadAxis::TriggerLeft);
                    let yu = state.axis(GamepadAxis::TriggerRight);
                    let yd = deadzone(unorm_i16_to_f32(yd), 0.1);
                    let yu = deadzone(unorm_i16_to_f32(yu), 0.1);
                    let y = yu - yd;
                    transform.position.y += (y as f64) * (4.0 * delta);
                }

                let y = (elapsed * 0.5).sin() * 10.0;
                let (transform, _camera) = world
                    .0
                    .query_one_mut::<(&mut Transform, &StaticMesh)>(throbber)
                    .unwrap();
                transform.position = DVec3::new(0.0, y, -5.0);
                transform.rotation = Rotor3::from_rotation_xz(elapsed as f32);
            },
        );

        Box::new(Vec::new())
    }
}

aleph::any::declare_interfaces!(PluginGameLogic, [IPlugin]);

pub fn engine_runner() {
    let mut engine = Engine::builder();
    engine.default_plugins();
    engine.plugin(PluginGameLogic::new());
    engine.build(|engine| engine.run())
}

fn unorm_i16_to_f32(v: i16) -> f32 {
    ((v as f32) / (i16::MAX as f32)).clamp(-1.0, 1.0)
}
fn deadzone(v: f32, dead: f32) -> f32 {
    let norm_factor = 1.0 / (1.0 - dead);
    if v.is_sign_negative() {
        (v.min(-dead) + dead) * norm_factor
    } else {
        (v.max(dead) - dead) * norm_factor
    }
}
