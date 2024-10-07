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

use aleph_engine::any::AnyArc;
use aleph_engine::interfaces::components::{StaticMesh, Transform};
use aleph_engine::interfaces::ecs::{EntityId, World};
use aleph_engine::interfaces::math::{DVec3, Rotor3};
use aleph_engine::interfaces::platform::IFrameTimer;

pub struct ThrobberLogic {
    frame_timer: AnyArc<dyn IFrameTimer>,
    throbber: EntityId,
}

impl ThrobberLogic {
    pub fn new(frame_timer: AnyArc<dyn IFrameTimer>, throbber: EntityId) -> Self {
        Self {
            frame_timer,
            throbber,
        }
    }

    pub fn tick(&self, world: &mut World) {
        let elapsed = self.frame_timer.elapsed_time();

        let y = (elapsed * 0.5).sin() * 10.0;
        let (transform, _) = world
            .query_one_mut::<(&mut Transform, &StaticMesh)>(self.throbber)
            .unwrap();
        transform.position = DVec3::new(0.0, y, -5.0);
        transform.rotation = Rotor3::from_rotation_xz(elapsed as f32);
    }
}
