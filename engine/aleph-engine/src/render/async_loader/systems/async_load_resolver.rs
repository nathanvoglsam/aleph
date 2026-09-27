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

use aleph_object_system::unsafe_impl_iobject;
use api::ecs::world::World;
use api::label::{Label, make_label};
use api::schedule::{CoreStage, WorldResource};
use api::scheduler::{ExplicitDependencies, IntoSystem, ResMut, Schedule};
use crossbeam::queue::SegQueue;
use mg::renderer::Renderer;

use crate::render::core::systems::publish_render_scene::PublishRenderSceneSystem;

/// Handle to the 'async load resolver' queue that can be shared across threads and systems.
///
/// Can be used to enqueue closures onto the resolver queue. Enqueued jobs will be run on the next
/// available simulation frame.
///
/// The intended use case is for async io tasks to publish steps of execution onto the simulation
/// thread so they can interact with the world and renderer.
#[derive(Clone, Default)]
pub struct AsyncLoadResolverQueue(Arc<SegQueue<Box<ResolverFn>>>);

unsafe_impl_iobject!(
    AsyncLoadResolverQueue,
    "01a0e240-ae10-7753-89ff-f561ecfc2945"
);

impl AsyncLoadResolverQueue {
    /// Push the given closure onto the queue to be executed in the future.
    pub fn push<F>(&self, resolver_fn: F)
    where
        F: FnOnce(&mut World, &mut Renderer) + Send + 'static,
    {
        self.0.push(Box::new(resolver_fn));
    }
}

pub struct AsyncLoadResolverSystem {
    queue: AsyncLoadResolverQueue,
}

impl AsyncLoadResolverSystem {
    pub const LABEL: Label = make_label!("render::AsyncLoadResolverSystem");

    pub fn new(queue: AsyncLoadResolverQueue) -> Self {
        Self { queue }
    }

    pub fn queue(&self) -> AsyncLoadResolverQueue {
        self.queue.clone()
    }

    pub fn register(mut self, schedule: &mut Schedule) {
        let system = move |mut world: ResMut<WorldResource>, mut renderer: ResMut<Renderer>| {
            self.run(&mut world.0, &mut renderer);
        };
        let system = system.system().runs_before(PublishRenderSceneSystem::LABEL);
        schedule.add_system_to_stage(CoreStage::Render.into(), Self::LABEL, system);
    }

    pub fn run(&mut self, world: &mut World, renderer: &mut Renderer) {
        // The async loader will publish messages onto this channel once the resources are loaded
        // and available.
        //
        // We poll the channel and drain all the messages.
        loop {
            let msg = match self.queue.0.pop() {
                Some(msg) => msg,
                None => break,
            };

            msg(world, renderer);
        }
    }
}

type ResolverFn = dyn FnOnce(&mut World, &mut Renderer) + Send + 'static;
