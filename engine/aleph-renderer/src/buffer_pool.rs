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

use std::num::NonZeroU8;

use aleph_any::AnyArc;
use aleph_rhi_api::*;

use crate::{BufferHandle, ObjectPool};

pub struct BufferPool {
    pool: ObjectPool<BufferObject>,
}

impl BufferPool {
    /// Constructs a new [BufferPool] with the given pool id tag.
    ///
    /// The `id` tag is stored inside any [BufferHandle] objects this pool allocates so that it
    /// can do some cheap (though not bulletproof) verification that the handle came from this
    /// pool.
    ///
    /// It's up to the caller to not reuse a pool id, at the very least at the same time.
    pub fn new(id: NonZeroU8) -> Self {
        Self {
            pool: ObjectPool::new(id),
        }
    }

    /// Creates a new buffer handle with no buffer data stored inside.
    ///
    /// The buffer object inside will be missing until the data is populated using
    /// [BufferPool::update_buffer].
    pub fn reserve_handle(&mut self) -> BufferHandle {
        self.alloc(BufferObject { buffer: None })
    }

    /// Creates a new buffer object from the given buffer and returns a [BufferHandle] that can
    /// be used to retreive the buffer from the pool in the future.
    pub fn create_buffer(&mut self, buffer: Option<AnyArc<dyn IBuffer>>) -> BufferHandle {
        let object = BufferObject { buffer };

        self.alloc(object)
    }

    /// Updates an existing buffer object, identified by the given [BufferHandle] with a new RHI
    /// buffer. Returns the old buffer if one was present for the given handle.
    pub fn update_buffer(
        &mut self,
        handle: BufferHandle,
        buffer: AnyArc<dyn IBuffer>,
    ) -> Option<AnyArc<dyn IBuffer>> {
        if let Some(object) = self.get_mut(handle) {
            if let Some(old_buffer) = &object.buffer {
                let new_desc = buffer.desc_ref();
                let old_desc = old_buffer.desc_ref();

                // It is illegal for any major property of the new buffer to change from the old
                // buffer.
                debug_assert_eq!(new_desc.usage, old_desc.usage);
            }

            // Swap the old buffer for the new, taking the old buffer to send it out to the caller
            let mut buffer = Some(buffer);
            std::mem::swap(&mut buffer, &mut object.buffer);

            // And give the old buffer back out to the caller
            buffer
        } else {
            None
        }
    }

    /// Returns the buffer object associated with the given [BufferHandle].
    ///
    /// May return [None] if either the handle is invalid (dead, wrong pool, etc) or if the pool
    /// doesn't have a buffer for the requested handle yet. It's possible for a handle to have
    /// no buffer, such as if the handle was reserved but hasn't been initialized with
    /// [BufferPool::update_buffer] yet.
    pub fn get_buffer(&self, handle: BufferHandle) -> Option<&dyn IBuffer> {
        if let Some(object) = self.get_ref(handle) {
            object.buffer.as_deref()
        } else {
            None
        }
    }

    /// Removes the given buffer from the pool, returning the [IBuffer] object it was storing if
    /// one exists.
    ///
    /// The nested options represent two levels of optional values. The outer option return directly
    /// by this function signals whether a buffer object was found and removed from the given
    /// handle. The inner option reflects that the referenced buffer may not have an [IBuffer]
    /// to return. Flattening the options would make a missing [IBuffer] look like an invalid
    /// handle.
    pub fn destroy_buffer(&mut self, handle: BufferHandle) -> Option<Option<AnyArc<dyn IBuffer>>> {
        self.free(handle).map(|v| v.buffer)
    }
}

impl BufferPool {
    pub(crate) fn alloc(&mut self, data: BufferObject) -> BufferHandle {
        let handle = self.pool.alloc(data);

        // Safety: uuuh, this is a buffer pool, of course we give out _buffer handles_
        unsafe { BufferHandle::from_handle(handle) }
    }

    pub(crate) fn get_ref(&self, handle: BufferHandle) -> Option<&BufferObject> {
        self.pool.get_ref(handle.to_handle())
    }

    pub(crate) fn get_mut(&mut self, handle: BufferHandle) -> Option<&mut BufferObject> {
        self.pool.get_mut(handle.to_handle())
    }

    pub(crate) fn free(&mut self, handle: BufferHandle) -> Option<BufferObject> {
        self.pool.free(handle.to_handle())
    }
}

pub struct BufferObject {
    /// The buffer object itself
    buffer: Option<AnyArc<dyn IBuffer>>,
}
