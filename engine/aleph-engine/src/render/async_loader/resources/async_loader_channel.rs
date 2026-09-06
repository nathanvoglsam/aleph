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
use aleph_vfs::IRouter;
use api::ecs::entity::EntityHandle;
use crossbeam::channel::Sender;

use crate::render::async_loader::internal::buffer_load::{BufferLoadPayload, BufferLoadTask};
use crate::render::async_loader::internal::task::ITaskFactory;
use crate::render::async_loader::internal::worker::WorkerTask;
use crate::render::async_loader::resources::async_loader_requests::{
    AsyncLoaderRequests, ResourceLoadState,
};

pub struct AsyncLoaderChannel {
    pub(crate) loader_sender: Sender<WorkerTask>,
    pub(crate) buffer_loader: Arc<dyn ITaskFactory>,
}

impl AsyncLoaderChannel {
    pub(crate) fn new(loader_sender: Sender<WorkerTask>, vfs: Arc<dyn IRouter>) -> Self {
        Self {
            loader_sender,
            buffer_loader: BufferLoadTask::new(vfs),
        }
    }

    pub fn spawn_vertex_buffer_load(
        &self,
        requests: &mut AsyncLoaderRequests,
        entity: EntityHandle,
    ) {
        let load_handle = requests
            .states
            .alloc(ResourceLoadState::VertexBuffer { entity });
        let _ = self.loader_sender.send(WorkerTask::new(
            self.buffer_loader.clone(),
            BufferLoadPayload {
                cookie: load_handle,
                path: "i".into(),
                offset: 0,
                size: 0,
            },
        ));
    }

    pub fn spawn_index_buffer_load(
        &self,
        requests: &mut AsyncLoaderRequests,
        entity: EntityHandle,
    ) {
        let load_handle = requests
            .states
            .alloc(ResourceLoadState::IndexBuffer { entity });
        let _ = self.loader_sender.send(WorkerTask::new(
            self.buffer_loader.clone(),
            BufferLoadPayload {
                cookie: load_handle,
                path: "i".into(),
                offset: 0,
                size: 0,
            },
        ));
    }
}

unsafe_impl_iobject!(AsyncLoaderChannel, "01a0507a-e696-7c80-8c65-4de589b72364");
