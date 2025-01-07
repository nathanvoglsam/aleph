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

use crate::{MaterialInstanceHandle, MaterialInstanceObject, ObjectPool};

pub struct MaterialInstancePool {
    pool: ObjectPool<MaterialInstanceObject>,
}

impl MaterialInstancePool {
    /// Constructs a new [`MaterialInstancePool`] with the given pool id tag.
    ///
    /// The `id` tag is stored inside any [`MaterialInstanceHandle`] objects this pool allocates so that it
    /// can do some cheap (though not bulletproof) verification that the handle came from this
    /// pool.
    ///
    /// It's up to the caller to not reuse a pool id, at the very least at the same time.
    pub fn new(id: NonZeroU8) -> Self {
        Self {
            pool: ObjectPool::new(id),
        }
    }

    pub fn alloc(&mut self, data: MaterialInstanceObject) -> MaterialInstanceHandle {
        let handle = self.pool.alloc(data);

        // Safety: uuuh, this is a material pool, of course we give out _material handles_
        unsafe { MaterialInstanceHandle::from_handle(handle) }
    }

    pub fn get_ref(&self, handle: MaterialInstanceHandle) -> Option<&MaterialInstanceObject> {
        self.pool.get_ref(handle.to_handle())
    }

    pub fn get_mut(
        &mut self,
        handle: MaterialInstanceHandle,
    ) -> Option<&mut MaterialInstanceObject> {
        self.pool.get_mut(handle.to_handle())
    }

    pub fn free(&mut self, handle: MaterialInstanceHandle) -> Option<MaterialInstanceObject> {
        self.pool.free(handle.to_handle())
    }
}
