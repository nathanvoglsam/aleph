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

#![cfg(target_os = "windows")]

extern crate aleph_windows as windows;

mod allocation;
mod allocator;
mod pool;
mod raw;
mod virtual_block;

pub use crate::allocation::D3D12MAAllocation;
pub use crate::allocator::D3D12MAAllocator;
pub use crate::pool::D3D12MAPool;
pub use crate::virtual_block::D3D12MAVirtualBlock;

pub use raw::D3D12MA_ALLOCATE_FN;
pub use raw::D3D12MA_ALLOCATION_CALLBACKS;
pub use raw::D3D12MA_ALLOCATION_DESC;
pub use raw::D3D12MA_ALLOCATION_FLAGS;
pub use raw::D3D12MA_ALLOCATOR_DESC;
pub use raw::D3D12MA_ALLOCATOR_FLAGS;
pub use raw::D3D12MA_BUDGET;
pub use raw::D3D12MA_FREE_FN;
pub use raw::D3D12MA_POOL_DESC;
pub use raw::D3D12MA_STATS;
pub use raw::D3D12MA_STAT_INFO;
pub use raw::D3D12MA_VIRTUAL_ALLOCATION_DESC;
pub use raw::D3D12MA_VIRTUAL_ALLOCATION_INFO;
pub use raw::D3D12MA_VIRTUAL_BLOCK_DESC;
pub use raw::HEAP_TYPE_COUNT;
