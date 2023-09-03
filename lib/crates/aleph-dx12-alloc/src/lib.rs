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

pub use crate::allocation::Allocation;
pub use crate::allocator::Allocator;
pub use crate::defragmentation_context::DefragmentationContext;
pub use crate::pool::Pool;
pub use crate::virtual_block::VirtualBlock;

pub use raw::ALLOCATE_FN;
pub use raw::ALLOCATION_CALLBACKS;
pub use raw::ALLOCATOR_DESC;
pub use raw::ALLOCATOR_FLAGS;
pub use raw::FREE_FN;

pub use raw::AllocHandle;
pub use raw::ALLOCATION_DESC;
pub use raw::ALLOCATION_FLAGS;

pub use raw::POOL_DESC;
pub use raw::POOL_FLAGS;

pub use raw::DEFRAGMENTATION_DESC;
pub use raw::DEFRAGMENTATION_FLAGS;
pub use raw::DEFRAGMENTATION_MOVE;
pub use raw::DEFRAGMENTATION_MOVE_OPERATION;
pub use raw::DEFRAGMENTATION_PASS_MOVE_INFO;
pub use raw::DEFRAGMENTATION_STATS;

pub use raw::Budget;
pub use raw::DetailedStatistics;
pub use raw::Statistics;
pub use raw::TotalStatistics;

pub use raw::VirtualAllocation;
pub use raw::VIRTUAL_ALLOCATION_DESC;
pub use raw::VIRTUAL_ALLOCATION_FLAGS;
pub use raw::VIRTUAL_ALLOCATION_INFO;
pub use raw::VIRTUAL_BLOCK_DESC;
pub use raw::VIRTUAL_BLOCK_FLAGS;
