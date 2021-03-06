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

use crate::alloc::{AllocationFlags, Pool};
use alloc_raw::D3D12MA_ALLOCATION_DESC;

pub struct AllocationDescBuilder<'a> {
    inner: AllocationDesc<'a>,
}

impl<'a> AllocationDescBuilder<'a> {
    pub fn new() -> Self {
        Self {
            inner: AllocationDesc::default(),
        }
    }

    pub fn flags(mut self, flags: AllocationFlags) -> Self {
        self.inner.flags |= flags;
        self
    }

    pub fn heap_type(mut self, heap_type: crate::HeapType) -> Self {
        self.inner.heap_type = heap_type;
        self
    }

    pub fn extra_heap_flags(mut self, extra_heap_flags: crate::HeapFlags) -> Self {
        self.inner.extra_heap_flags |= extra_heap_flags;
        self
    }

    pub fn pool(mut self, pool: &'a Pool) -> Self {
        self.inner.pool = Some(pool);
        self
    }

    pub fn build(self) -> AllocationDesc<'a> {
        self.inner
    }
}

pub struct AllocationDesc<'a> {
    pub flags: AllocationFlags,
    pub heap_type: crate::HeapType,
    pub extra_heap_flags: crate::HeapFlags,
    pub pool: Option<&'a Pool>,
}

impl<'a> AllocationDesc<'a> {
    pub fn builder() -> AllocationDescBuilder<'a> {
        AllocationDescBuilder::new()
    }
}

impl<'a> Default for AllocationDesc<'a> {
    fn default() -> Self {
        Self {
            flags: AllocationFlags::NONE,
            heap_type: crate::HeapType::Default,
            extra_heap_flags: crate::HeapFlags::NONE,
            pool: None,
        }
    }
}

impl<'a> Into<D3D12MA_ALLOCATION_DESC> for &AllocationDesc<'a> {
    fn into(self) -> D3D12MA_ALLOCATION_DESC {
        D3D12MA_ALLOCATION_DESC {
            flags: self.flags,
            heap_type: self.heap_type.into(),
            extra_heap_flags: self.extra_heap_flags.into(),
            pool: std::ptr::null_mut(),
        }
    }
}
