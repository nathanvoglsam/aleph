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
use aleph_object_system::unsafe_impl_iobject;
use api::ecs::entity::EntityHandle;

use crate::internal::EngineSystem;

pub struct AsyncLoaderRequests {
    pub(crate) states: GenArena<ResourceLoadState, ResourceLoadHandle, EngineSystem>,
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
