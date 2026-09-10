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

use std::io;
use std::num::NonZero;
use std::ptr::NonNull;
use std::sync::Arc;

use aleph_gen_arena::{GenArena, Handle, HandleType, RawHandle, make_handle_id};
use aleph_io_queue::channel::{ChannelError, LoadChannel, OpenChannel, ReadChannel};
use aleph_vfs::async_io::AsyncIoMessage;
use aleph_vfs::file::{IAsyncVFile, IAsyncVFileExt};
use aleph_vfs::path::VPath;
use aleph_vfs::{IRouter, IRouterExt};
use api::ecs::world::World;
use api::label::{Label, make_label};
use api::schedule::{CoreStage, WorldResource};
use api::scheduler::{ExplicitDependencies, IntoSystem, ResMut, Schedule};
use crossbeam::queue::SegQueue;
use mg::renderer::Renderer;

use crate::core::alloc::EngineSystem;
use crate::gltf::resources::gltf_scene_loader_requests::GltfSceneLoaderRequests;

pub struct GltfSceneLoaderSystem {
    vfs: Arc<dyn IRouter>,
    queue: Arc<SegQueue<AsyncIoMessage>>,
    states: GenArena<GltfLoadState, GltfLoadHandle, EngineSystem>,
}

impl GltfSceneLoaderSystem {
    pub const LABEL: Label = make_label!("gltf::GltfSceneLoaderSystem");

    pub fn new(vfs: Arc<dyn IRouter>) -> Self {
        Self {
            vfs,
            queue: Arc::new(Default::default()),
            states: GenArena::new_in(),
        }
    }

    pub fn register(mut self, schedule: &mut Schedule) {
        let system = move |mut world: ResMut<WorldResource>,
                           mut renderer: ResMut<Renderer>,
                           mut loader: ResMut<GltfSceneLoaderRequests>| {
            self.run(&mut world.0, &mut renderer, &mut loader);
        };
        let system = system.system().runs_before(Self::LABEL);
        schedule.add_system_to_stage(CoreStage::Render.into(), Self::LABEL, system);
    }

    pub fn run(
        &mut self,
        world: &mut World,
        renderer: &mut Renderer,
        loader: &mut GltfSceneLoaderRequests,
    ) {
        let mut to_retire = Vec::new();

        let (states, handles) = self.states.objects_and_handles_mut();
        for (state, &handle) in states.iter_mut().zip(handles) {
            match state {
                GltfLoadState::PendingOpen { path } => {
                    let sender = AsyncIoSender(self.queue.clone());
                    let path = path.clone();
                    let opaque = handle.to_bare_handle().into_int().get();
                    if let Err(_) = self.vfs.open_async(sender, path, opaque) {
                        to_retire.push(handle);
                    }
                    *state = GltfLoadState::WaitingOpen;
                }
                GltfLoadState::WaitingOpen => {} // Do nothing here, handled later
                GltfLoadState::WaitingLoad { .. } => {} // Do nothing here, handled later
            }
        }

        'outer: while let Some(msg) = self.queue.pop() {
            let handle = msg.opaque();
            let handle = match NonZero::new(handle) {
                None => continue,
                Some(v) => v,
            };
            let handle = RawHandle::from_int(handle);
            let handle = GltfLoadHandle::from_bare_handle(handle);

            let state = match self.states.get_mut(handle) {
                None => continue,
                Some(v) => v,
            };

            match msg {
                AsyncIoMessage::ReadSuccess { path, .. } => {
                    log::error!("Unexpected 'ReadSuccess' while loading GLTF file '{path}'");
                }
                AsyncIoMessage::ReadFail { path, err, .. } => {
                    log::error!("Failed to load GLTF file '{path}' with error: '{err}'");
                }
                AsyncIoMessage::LoadSuccess { path, data, .. } => match state {
                    GltfLoadState::PendingOpen { .. } | GltfLoadState::WaitingOpen => {
                        log::error!("Unexpected 'LoadSuccess' while loading GLTF file '{path}'");
                    }
                    GltfLoadState::WaitingLoad { .. } => {
                        self.spawn_gltf_scene(world, renderer, loader, path, data);
                    }
                },
                AsyncIoMessage::LoadFail { path, err, .. } => {
                    log::error!("Failed to load GLTF file '{path}' with error: '{err}'");
                }
                AsyncIoMessage::OpenSuccess { file, opaque } => match state {
                    GltfLoadState::PendingOpen { .. } | GltfLoadState::WaitingLoad { .. } => {
                        let path = file.path();
                        log::error!("Unexpected 'OpenSuccess' while loading GLTF file '{path}'");
                    }
                    GltfLoadState::WaitingOpen => {
                        match file.load(AsyncIoSender(self.queue.clone()), opaque) {
                            Ok(_) => {
                                *state = GltfLoadState::WaitingLoad { file };
                                continue 'outer;
                            }
                            Err(_) => {
                                log::error!("Failed to load GLTF file '{}'", file.path());
                            }
                        }
                    }
                },
                AsyncIoMessage::OpenFail { err, .. } => {
                    log::error!("Failed to open GLTF file with error: '{err}'");
                }
            }

            self.states.free(handle);
        }
    }

    pub fn spawn_gltf_scene(
        &mut self,
        world: &mut World,
        renderer: &mut Renderer,
        loader: &mut GltfSceneLoaderRequests,
        path: Arc<VPath>,
        data: Vec<u8>,
    ) {
    }
}

struct AsyncIoSender(Arc<SegQueue<AsyncIoMessage>>);

impl OpenChannel<Arc<dyn IAsyncVFile>> for AsyncIoSender {
    fn send_success(&self, opaque: u64, file: Arc<dyn IAsyncVFile>) -> Result<(), ChannelError> {
        let _ = self.0.push(AsyncIoMessage::OpenSuccess { file, opaque });
        Ok(())
    }

    fn send_fail(
        &self,
        opaque: u64,
        _file: Arc<dyn IAsyncVFile>,
        err: io::Error,
    ) -> Result<(), ChannelError> {
        let _ = self.0.push(AsyncIoMessage::OpenFail { err, opaque });
        Ok(())
    }
}

impl ReadChannel<Arc<VPath>> for AsyncIoSender {
    fn send_success(
        &self,
        opaque: u64,
        file: Arc<VPath>,
        buf: NonNull<[u8]>,
        offset: u64,
        bytes_transferred: usize,
    ) -> Result<(), ChannelError> {
        let _ = self.0.push(AsyncIoMessage::ReadSuccess {
            path: file,
            buf,
            offset,
            bytes_transferred,
            opaque,
        });
        Ok(())
    }

    fn send_fail(
        &self,
        opaque: u64,
        file: Arc<VPath>,
        buf: NonNull<[u8]>,
        offset: u64,
        err: io::Error,
    ) -> Result<(), ChannelError> {
        let _ = self.0.push(AsyncIoMessage::ReadFail {
            path: file,
            buf,
            offset,
            err,
            opaque,
        });
        Ok(())
    }
}

impl LoadChannel<Arc<VPath>> for AsyncIoSender {
    fn send_success(
        &self,
        opaque: u64,
        file: Arc<VPath>,
        data: Vec<u8>,
    ) -> Result<(), ChannelError> {
        let _ = self.0.push(AsyncIoMessage::LoadSuccess {
            path: file,
            data,
            opaque,
        });
        Ok(())
    }

    fn send_fail(&self, opaque: u64, file: Arc<VPath>, err: io::Error) -> Result<(), ChannelError> {
        let _ = self.0.push(AsyncIoMessage::LoadFail {
            path: file,
            err,
            opaque,
        });
        Ok(())
    }
}

pub struct GltfLoad;
make_handle_id!(GltfLoad);
pub type GltfLoadHandle = Handle<GltfLoad>;

pub enum GltfLoadState {
    PendingOpen {
        /// The path to the GLTF file to open
        path: Arc<VPath>,
    },
    WaitingOpen,
    WaitingLoad {
        file: Arc<dyn IAsyncVFile>,
    },
}
