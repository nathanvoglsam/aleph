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

use aleph_engine::api::components::{Camera, Transform};
use aleph_engine::api::ecs::entity::EntityHandle;
use aleph_engine::api::ecs::world::World;
use aleph_engine::api::ecs::world::query::{Read, Write};
use aleph_engine::api::math::{Rotor3, ToDouble, Vec3};
use aleph_engine::api::platform::{
    GamepadAxis, IFrameTimer, IGamepadsAccessor, IKeyboard, IMouse, MouseEvent, ScanCode,
};

pub struct FreeCamera {
    frame_timer: Arc<dyn IFrameTimer>,
    keyboard: Arc<dyn IKeyboard>,
    mouse: Arc<dyn IMouse>,
    gamepad: Arc<dyn IGamepadsAccessor>,
    l_dir: f32,
    u_dir: f32,
    camera: EntityHandle,
}

impl FreeCamera {
    pub fn new(
        frame_timer: Arc<dyn IFrameTimer>,
        keyboard: Arc<dyn IKeyboard>,
        mouse: Arc<dyn IMouse>,
        gamepad: Arc<dyn IGamepadsAccessor>,
        camera: EntityHandle,
    ) -> Self {
        Self {
            frame_timer,
            keyboard,
            mouse,
            gamepad,
            l_dir: 0.0,
            u_dir: 0.0,
            camera,
        }
    }

    pub fn tick(&mut self, world: &mut World) {
        let delta = self.frame_timer.delta_time() as f32;

        let (transform, _camera) =
            if let Some(v) = world.query_one_mut::<(Write<Transform>, Read<Camera>)>(self.camera) {
                v
            } else {
                return;
            };

        // Decode keyboard state to motion commands
        //
        // +x = right
        // +y = up
        // -z = forward
        let keyboard = self.keyboard.get_state();
        let key = |key| keyboard.scan_code_down(key);
        let mut move_x = f32::from(key(ScanCode::D)) - f32::from(key(ScanCode::A));
        let mut move_z = f32::from(key(ScanCode::S)) - f32::from(key(ScanCode::W));
        let mut move_y = f32::from(key(ScanCode::Space)) - f32::from(key(ScanCode::LCtrl));
        drop(keyboard);

        // Accumulate mouse motion into camera rotation
        for event in self.mouse.events().events() {
            if let MouseEvent::MouseMotion(motion) = event {
                if motion.mouse_state.right() {
                    self.l_dir += motion.x_rel * 0.1;
                    self.u_dir -= motion.y_rel * 0.1;
                }
            }
        }

        // If there's an active controller, apply the controller commands on top of the keyboard and
        // mouse inputs. Can you go faster if you use the controller and keyboard together? Yes. Do
        // I care? No.
        if let Some(state) = self.gamepad.get_active_controller_state() {
            let look_x = deadzone(unorm_i16_to_f32(state.axis(GamepadAxis::RightX)), 0.1);
            let look_y = deadzone(unorm_i16_to_f32(state.axis(GamepadAxis::RightY)), 0.1);
            self.l_dir += look_x * (90.0 * delta);
            self.u_dir -= look_y * (90.0 * delta);

            move_x += deadzone(unorm_i16_to_f32(state.axis(GamepadAxis::LeftX)), 0.1);
            move_z += deadzone(unorm_i16_to_f32(state.axis(GamepadAxis::LeftY)), 0.1);

            let down = deadzone(unorm_i16_to_f32(state.axis(GamepadAxis::TriggerLeft)), 0.1);
            let up = deadzone(unorm_i16_to_f32(state.axis(GamepadAxis::TriggerRight)), 0.1);
            move_y += up - down;
        }

        // Clamp the vertical rotation to prevent flipping over. Necks can't bend that far
        // typically.
        self.u_dir = self.u_dir.clamp(-89.0, 89.0);

        // We maintain a shadow euler angle representation to ensure that the camera up stays
        // locked to +Y.
        let yaw = Rotor3::from_rotation_xz(self.l_dir.to_radians());
        let pitch = Rotor3::from_rotation_yz(self.u_dir.to_radians());
        transform.rotation = yaw * pitch;

        let lin_vel = 4.0 * delta;
        let vel = transform.rotation * Vec3::new(move_x, 0.0, move_z); // Rotate into frame
        let vel = vel * Vec3::broadcast(lin_vel); // Apply horizontal velocity
        let vel = vel + Vec3::new(0.0, move_y * lin_vel, 0.0); // Add vertical velocity
        transform.position += vel.to_double();
    }
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
