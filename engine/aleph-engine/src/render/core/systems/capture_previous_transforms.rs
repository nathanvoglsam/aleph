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

use api::components::{Transform, TransformHistory};
use api::ecs::world::query::{Read, Write};
use api::label::{Label, make_label};
use api::schedule::{CoreStage, WorldResource};
use api::scheduler::{ResMut, Schedule};

pub struct CapturePreviousTransformsSystem;

impl CapturePreviousTransformsSystem {
    pub const LABEL: Label = make_label!("render::CapturePreviousTransforms");

    pub fn register(mut self, schedule: &mut Schedule) {
        let system = move |world: ResMut<WorldResource>| {
            self.run(world);
        };
        schedule.add_exclusive_at_end_system_to_stage(
            CoreStage::Render.into(),
            Self::LABEL,
            system,
        );
    }

    pub fn run(&mut self, mut world: ResMut<WorldResource>) {
        let world = &mut world.0;
        for (_id, (t, h)) in world.query_mut::<(Read<Transform>, Write<TransformHistory>)>() {
            h.previous = t.clone();
        }
    }
}
