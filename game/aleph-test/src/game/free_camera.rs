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
use aleph_engine::api::platform::{GamepadAxis, IFrameTimer, IGamepadsAccessor};

pub struct FreeCamera {
    frame_timer: Arc<dyn IFrameTimer>,
    gamepad: Arc<dyn IGamepadsAccessor>,
    l_dir: f32,
    u_dir: f32,
    camera: EntityHandle,
}

impl FreeCamera {
    pub fn new(
        frame_timer: Arc<dyn IFrameTimer>,
        gamepad: Arc<dyn IGamepadsAccessor>,
        camera: EntityHandle,
    ) -> Self {
        Self {
            frame_timer,
            gamepad,
            l_dir: 0.0,
            u_dir: 0.0,
            camera,
        }
    }

    pub fn tick(&mut self, world: &mut World) {
        let delta = self.frame_timer.delta_time();

        if let Some(state) = self.gamepad.get_active_controller_state() {
            let (transform, _camera) = world
                .query_one_mut::<(Write<Transform>, Read<Camera>)>(self.camera)
                .unwrap();

            let l = state.axis(GamepadAxis::RightX);
            let u = state.axis(GamepadAxis::RightY);
            let l = deadzone(unorm_i16_to_f32(l), 0.1);
            let u = deadzone(unorm_i16_to_f32(u), 0.1);
            self.l_dir += l * (90.0f32 * delta as f32);
            self.u_dir -= u * (90.0f32 * delta as f32);
            let yaw = Rotor3::from_rotation_xz(self.l_dir.to_radians());
            let pitch = Rotor3::from_rotation_yz(self.u_dir.to_radians());
            transform.rotation = yaw * pitch;

            let x = state.axis(GamepadAxis::LeftX);
            let z = state.axis(GamepadAxis::LeftY);
            let x = deadzone(unorm_i16_to_f32(x), 0.1);
            let z = deadzone(unorm_i16_to_f32(z), 0.1);

            // Get the camera's direction vectors
            let cam_forward =
                transform.rotation * (-Vec3::unit_z()) * Vec3::broadcast(z * (4.0 * delta as f32));
            let cam_right =
                transform.rotation * (-Vec3::unit_x()) * Vec3::broadcast(x * (4.0 * delta as f32));

            transform.position -= cam_forward.to_double();
            transform.position -= cam_right.to_double();

            let yd = state.axis(GamepadAxis::TriggerLeft);
            let yu = state.axis(GamepadAxis::TriggerRight);
            let yd = deadzone(unorm_i16_to_f32(yd), 0.1);
            let yu = deadzone(unorm_i16_to_f32(yu), 0.1);
            let y = yu - yd;
            transform.position.y += (y as f64) * (4.0 * delta);
        }
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
