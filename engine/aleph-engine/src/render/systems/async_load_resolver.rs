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

use aleph_gen_arena::{GenArena, Handle, make_handle_id};
use api::components::StaticMesh;
use api::ecs::entity::EntityHandle;
use api::ecs::world::World;
use mg::async_resource_loader::loader_notify::{LoaderMessage, LoaderNotify};
use mg::common::channel::TryRecvError;
use mg::renderer::Renderer;

use crate::internal::EngineSystem;

pub struct AsyncLoadResolverSystem {
    pub load_channel: LoaderNotify<ResourceLoadHandle>,
}

impl AsyncLoadResolverSystem {
    pub fn run(
        &mut self,
        world: &mut World,
        renderer: &mut Renderer,
        load_states: &mut ResourceLoadStates,
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
                        if let Some(state) = load_states.states.get_mut(cookie) {
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
                                    unreachable!("Got a buffer message for a texture load!");
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
                    let _ = load_states.states.free(cookie);
                }
                LoaderMessage::TextureComplete { cookie, .. } => {
                    // Finally we can free the request as it is fully resolved
                    let _ = load_states.states.free(cookie);
                    todo!();
                }
                LoaderMessage::Failed { cookie } => {
                    log::info!("Load failed");

                    // Finally we can free the request as it is fully resolved
                    let _ = load_states.states.free(cookie);
                }
                LoaderMessage::Canceled { cookie } => {
                    log::info!("Load canceled");

                    // Finally we can free the request as it is fully resolved
                    let _ = load_states.states.free(cookie);
                }
            }
        }
    }
}

pub struct ResourceLoad;

make_handle_id!(ResourceLoad);

pub type ResourceLoadHandle = Handle<ResourceLoad>;

pub enum ResourceLoadState {
    VertexBuffer { entity: EntityHandle },
    IndexBuffer { entity: EntityHandle },
    Texture { entity: EntityHandle },
}

pub struct ResourceLoadStates {
    pub states: GenArena<ResourceLoadState, ResourceLoadHandle, EngineSystem>,
}
