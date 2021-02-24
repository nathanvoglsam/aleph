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

use crate::{HeapFlags, HeapType};
use alloc_raw::D3D12MA_POOL_DESC;

pub struct PoolDescBuilder {
    inner: PoolDesc,
}

impl PoolDescBuilder {
    pub fn new() -> Self {
        Self {
            inner: PoolDesc::default(),
        }
    }

    pub fn heap_type(mut self, heap_type: HeapType) -> Self {
        self.inner.heap_type = heap_type;
        self
    }

    pub fn heap_flags(mut self, heap_flags: HeapFlags) -> Self {
        self.inner.heap_flags |= heap_flags;
        self
    }

    pub fn block_size(mut self, block_size: u64) -> Self {
        self.inner.block_size = block_size;
        self
    }

    pub fn min_block_count(mut self, min_block_count: u32) -> Self {
        self.inner.min_block_count = min_block_count;
        self
    }

    pub fn max_block_count(mut self, max_block_count: u32) -> Self {
        self.inner.max_block_count = max_block_count;
        self
    }

    pub fn build(self) -> PoolDesc {
        self.inner
    }
}

#[derive(Clone, Debug)]
pub struct PoolDesc {
    pub heap_type: HeapType,
    pub heap_flags: HeapFlags,
    pub block_size: u64,
    pub min_block_count: u32,
    pub max_block_count: u32,
}

impl PoolDesc {
    pub fn builder() -> PoolDescBuilder {
        PoolDescBuilder::new()
    }
}

impl Default for PoolDesc {
    fn default() -> Self {
        Self {
            heap_type: HeapType::Default,
            heap_flags: HeapFlags::NONE,
            block_size: 0,
            min_block_count: 0,
            max_block_count: 0,
        }
    }
}

impl Into<D3D12MA_POOL_DESC> for PoolDesc {
    fn into(self) -> D3D12MA_POOL_DESC {
        D3D12MA_POOL_DESC {
            heap_type: self.heap_type.into(),
            heap_flags: self.heap_flags.into(),
            block_size: self.block_size,
            min_block_count: self.min_block_count,
            max_block_count: self.max_block_count,
        }
    }
}
