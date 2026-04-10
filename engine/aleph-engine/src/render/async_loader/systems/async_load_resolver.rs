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

use api::components::StaticMesh;
use api::ecs::world::World;
use api::label::{Label, make_label};
use api::schedule::{CoreStage, WorldResource};
use api::scheduler::{ExplicitDependencies, IntoSystem, ResMut, Schedule};
use mg::async_resource_loader::loader_notify::{LoaderMessage, LoaderNotify};
use mg::common::channel::TryRecvError;
use mg::renderer::Renderer;

use crate::render::async_loader::resources::async_loader_requests::{
    AsyncLoaderRequests, ResourceLoadHandle, ResourceLoadState,
};
use crate::render::core::systems::publish_render_scene::PublishRenderSceneSystem;

pub struct AsyncLoadResolverSystem {
    load_channel: LoaderNotify<ResourceLoadHandle>,
}

impl AsyncLoadResolverSystem {
    pub const LABEL: Label = make_label!("render::AsyncLoadResolverSystem");

    pub fn new(load_channel: LoaderNotify<ResourceLoadHandle>) -> Self {
        Self { load_channel }
    }

    pub fn register(mut self, schedule: &mut Schedule) {
        let system = move |mut world: ResMut<WorldResource>,
                           mut renderer: ResMut<Renderer>,
                           mut loader: ResMut<AsyncLoaderRequests>| {
            self.run(&mut world.0, &mut renderer, &mut loader);
        };
        let system = system.system().runs_before(PublishRenderSceneSystem::LABEL);
        schedule.add_system_to_stage(CoreStage::Render.into(), Self::LABEL, system);
    }

    pub fn run(
        &mut self,
        world: &mut World,
        renderer: &mut Renderer,
        loader: &mut AsyncLoaderRequests,
    ) {
        // The async loader will publish messages onto this channel once the resources are loaded
        // and available.
        //
        // We poll the channel and drain all the messages.
        loop {
            let msg = match self.load_channel.try_recv() {
                Ok(msg) => msg,
                Err(TryRecvError::Empty) => break,

                // There's not much we can do locally if the loader disconnects. This should only
                // ever happen during renderer shutdown, however. We can just ignore this with
                // no real consequence.
                Err(TryRecvError::Disconnected) => break,
            };

            match msg {
                LoaderMessage::BufferComplete { cookie, resource } => {
                    // When a buffer load is complete we need to update the attached entity with
                    // the loaded buffer.
                    //
                    // Right now we only use this for vertex/index buffers and identify which of
                    // the two we've just loaded with the 'vertex' flag.
                    //
                    // TODO: In the future we're going to need to do this in a not stupid way, but
                    //       for now this will do for basic GLTF loading.
                    'success: {
                        if let Some(state) = loader.states.get_mut(cookie) {
                            match state {
                                ResourceLoadState::VertexBuffer { entity } => {
                                    let comp = world.get_component_mut::<StaticMesh>(*entity);
                                    if let Some(comp) = comp {
                                        comp.vtx = resource;
                                        break 'success;
                                    }
                                }
                                ResourceLoadState::IndexBuffer { entity } => {
                                    let comp = world.get_component_mut::<StaticMesh>(*entity);
                                    if let Some(comp) = comp {
                                        comp.idx = resource;
                                        break 'success;
                                    }
                                }
                                ResourceLoadState::Texture { .. } => {
                                    log::warn!("Got a buffer message for a texture load!");
                                }
                            }
                        }

                        // If we reach this code then we have _failed_ to store the loaded resource
                        // handle somewhere, or otherwise handle the loaded resource. We should
                        // destroy the resource, as otherwise we will leak the handle.
                        log::warn!("Load resolved onto an entity unable to receive it!");
                        renderer.destroy_buffer(resource);
                    }

                    // Finally we can free the request as it is fully resolved
                    let _ = loader.states.free(cookie);
                }
                LoaderMessage::TextureComplete { cookie, resource } => {
                    'success: {
                        if let Some(state) = loader.states.get_mut(cookie) {
                            match state {
                                ResourceLoadState::VertexBuffer { .. } => {
                                    log::warn!("Got a texture message for a buffer load!");
                                }
                                ResourceLoadState::IndexBuffer { .. } => {
                                    log::warn!("Got a texture message for a buffer load!");
                                }
                                ResourceLoadState::Texture { entity } => {
                                    let comp = world.get_component_mut::<StaticMesh>(*entity);
                                    if let Some(comp) = comp {
                                        log::warn!("Unimplemented texture load!");
                                        break 'success;
                                    }
                                }
                            }
                        }

                        // If we reach this code then we have _failed_ to store the loaded resource
                        // handle somewhere, or otherwise handle the loaded resource. We should
                        // destroy the resource, as otherwise we will leak the handle.
                        log::warn!("Load resolved onto an entity unable to receive it!");
                        renderer.destroy_texture(resource);
                    }

                    // Finally we can free the request as it is fully resolved
                    let _ = loader.states.free(cookie);
                }
                LoaderMessage::Failed { cookie } => {
                    log::info!("Load failed");

                    // Finally we can free the request as it is fully resolved
                    let _ = loader.states.free(cookie);
                }
                LoaderMessage::Canceled { cookie } => {
                    log::info!("Load canceled");

                    // Finally we can free the request as it is fully resolved
                    let _ = loader.states.free(cookie);
                }
            }
        }
    }
}
