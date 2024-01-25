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

#![cfg(windows)]

extern crate aleph_windows as windows;

mod allocation;
mod allocator;
mod defragmentation_context;
mod pool;
mod raw;
mod virtual_block;

pub use raw::{
    AllocHandle, Budget, DetailedStatistics, Statistics, TotalStatistics, VirtualAllocation,
    ALLOCATE_FN, ALLOCATION_CALLBACKS, ALLOCATION_DESC, ALLOCATION_FLAGS, ALLOCATOR_DESC,
    ALLOCATOR_FLAGS, DEFRAGMENTATION_DESC, DEFRAGMENTATION_FLAGS, DEFRAGMENTATION_MOVE,
    DEFRAGMENTATION_MOVE_OPERATION, DEFRAGMENTATION_PASS_MOVE_INFO, DEFRAGMENTATION_STATS, FREE_FN,
    POOL_DESC, POOL_FLAGS, VIRTUAL_ALLOCATION_DESC, VIRTUAL_ALLOCATION_FLAGS,
    VIRTUAL_ALLOCATION_INFO, VIRTUAL_BLOCK_DESC, VIRTUAL_BLOCK_FLAGS,
};

pub use crate::allocation::Allocation;
pub use crate::allocator::Allocator;
pub use crate::defragmentation_context::DefragmentationContext;
pub use crate::pool::Pool;
pub use crate::virtual_block::VirtualBlock;
