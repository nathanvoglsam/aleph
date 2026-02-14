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

use aleph_gen_arena::{Handle, make_handle_id};

use crate::internal::buffer::{BufferObject, BufferObjectStore};

pub struct Buffer;
make_handle_id!(Buffer);
pub type BufferHandle = Handle<Buffer>;

#[derive(Copy, Clone)]
pub struct BufferPoolAccessor<'a>(pub(crate) &'a BufferObjectStore);

impl<'a> BufferPoolAccessor<'a> {
    pub fn get_ref(&self, handle: BufferHandle) -> Option<BufferReader<'_>> {
        self.0.pool.get_ref(handle).map(BufferReader)
    }
}

#[derive(Copy, Clone)]
pub struct BufferReader<'a>(pub(crate) &'a BufferObject);

impl<'a> BufferReader<'a> {
    pub const fn handle(&self) -> Option<&rhi::BufferHandle> {
        self.0.object.as_ref()
    }
}
