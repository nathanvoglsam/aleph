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

use aleph_gen_arena::GenArena;

use crate::internal::Magnesium;
use crate::resource::buffer::{BufferHandle, BufferPoolAccessor};

/// Manager that owns the buffer object pool, and any other resources directly associated with
/// our pooled buffer resources.
pub struct BufferObjectStore {
    pub pool: GenArena<BufferObject, BufferHandle, MgBufSystem>,
}

impl BufferObjectStore {
    /// Constructs a new, empty object store
    pub fn new() -> Self {
        Self {
            pool: GenArena::new_in(),
        }
    }

    /// Create a read-only [`BufferPoolAccessor`]. See the accessor type for more information.
    pub const fn accessor(&self) -> BufferPoolAccessor<'_> {
        BufferPoolAccessor(self)
    }

    /// Cleanup code invoked when the renderer object is dropped.
    pub fn clean_up(&mut self) {
        self.pool.clear();
    }
}

/// The object we store _inside_ the buffer pool managed by a [`BufferObjectStore`]
pub struct BufferObject {
    /// The rhi object this pooled object owns.
    pub object: Option<rhi::BufferHandle>,
}

pub const fn make_standard_buffer_desc(size: u64) -> rhi::BufferDesc<'static> {
    // There is no reason not to just enable everything for buffers.
    const USAGE: rhi::ResourceUsageFlags = rhi::ResourceUsageFlags::COPY_SOURCE
        .union(rhi::ResourceUsageFlags::COPY_DEST)
        .union(rhi::ResourceUsageFlags::VERTEX_BUFFER)
        .union(rhi::ResourceUsageFlags::INDEX_BUFFER)
        .union(rhi::ResourceUsageFlags::CONSTANT_BUFFER)
        .union(rhi::ResourceUsageFlags::INDIRECT_DRAW_ARGS)
        // .union(rhi::ResourceUsageFlags::ACCELERATION_STRUCTURE_BUILD_INPUT)
        // .union(rhi::ResourceUsageFlags::ACCELERATION_STRUCTURE_STORAGE)
        .union(rhi::ResourceUsageFlags::SHADER_RESOURCE)
        .union(rhi::ResourceUsageFlags::UNORDERED_ACCESS);
    rhi::BufferDesc {
        size,
        cpu_access: rhi::CpuAccessMode::None,
        usage: USAGE,
        name: None,
    }
}

/// Allocator category for the buffer object store
pub struct Buffer;
aleph_alloc::new_child_alloc_category!(Magnesium, Buffer, "019b2ef7-63cc-70f0-8369-da19cf0a80cc");

pub type MgBufSystem = aleph_alloc::instrumentation::Instrumented<Buffer>;
