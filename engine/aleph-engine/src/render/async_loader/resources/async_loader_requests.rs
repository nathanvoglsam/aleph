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

use aleph_gen_arena::{GenArena, Handle, make_handle_id};
use aleph_object_system::unsafe_impl_iobject;
use aleph_vfs::IRouter;
use api::ecs::entity::EntityHandle;

use crate::core::alloc::EngineSystem;
use crate::core::async_io::task::ITaskFactory;
use crate::core::async_io::worker::AsyncLoaderQueue;
use crate::render::async_loader::internal::buffer_load::{BufferLoadPayload, BufferLoadTask};

pub struct AsyncLoaderRequests {
    pub(crate) states: GenArena<ResourceLoadState, ResourceLoadHandle, EngineSystem>,
    pub(crate) buffer_loader: Arc<dyn ITaskFactory>,
}

impl AsyncLoaderRequests {
    pub(crate) fn new(vfs: Arc<dyn IRouter>) -> Self {
        Self {
            states: GenArena::new_in(),
            buffer_loader: BufferLoadTask::new(vfs),
        }
    }

    pub fn spawn_vertex_buffer_load(&mut self, queue: &AsyncLoaderQueue, entity: EntityHandle) {
        let load_handle = self
            .states
            .alloc(ResourceLoadState::VertexBuffer { entity });
        let _ = queue.spawn(
            self.buffer_loader.clone(),
            BufferLoadPayload {
                cookie: load_handle,
                path: "i".into(),
                offset: 0,
                size: 0,
            },
        );
    }

    pub fn spawn_index_buffer_load(&mut self, queue: &AsyncLoaderQueue, entity: EntityHandle) {
        let load_handle = self.states.alloc(ResourceLoadState::IndexBuffer { entity });
        let _ = queue.spawn(
            self.buffer_loader.clone(),
            BufferLoadPayload {
                cookie: load_handle,
                path: "i".into(),
                offset: 0,
                size: 0,
            },
        );
    }
}

unsafe_impl_iobject!(AsyncLoaderRequests, "019d6084-d914-7a01-b9f6-d60d3b3e39d7");

pub struct ResourceLoad;

make_handle_id!(ResourceLoad);

pub type ResourceLoadHandle = Handle<ResourceLoad>;

pub enum ResourceLoadState {
    VertexBuffer { entity: EntityHandle },
    IndexBuffer { entity: EntityHandle },
    Texture { entity: EntityHandle },
}
