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

use aleph_alloc::offset_allocator;

use crate::GpuLayout;

/// Allocation description given out by a [`GpuAllocator`] that identifies the allocation
pub struct GpuAllocation {
    /// Allocation object that contains the state needed by the source pool's allocator to free the
    /// allocation.
    pub(crate) allocation: offset_allocator::Allocation,

    /// The size/alignment that this allocation was requested to fulfill.
    pub(crate) layout: GpuLayout,

    /// Offset within the block the allocation is sub-allocated from.
    pub(crate) block_offset: u32,

    /// Which pool the allocation was allocated from.
    pub(crate) pool_index: u16,

    /// Index into the set of memory blocks in the given pool that the allocation belongs to.
    pub(crate) block_index: u16,
}

impl GpuAllocation {
    /// The offset within the source memory block the allocated object begins at. This is where the
    /// object should be placed when constructing an API image/buffer object.
    pub const fn block_offset(&self) -> u32 {
        self.block_offset
    }

    /// Checks whether this is a dedicated allocation, rather than a sub-allocation of a larger
    /// memory block.
    pub(crate) const fn is_dedicated(&self) -> bool {
        // We can use the 'fail' niche on 'allocation' as a flag for whether the allocation is
        // backed by a dedicated block or not. Sub allocated blocks will never have a 'fail'
        // allocation, and dedicated blocks will never have a valid allocation handle. Perfect.
        self.allocation.is_fail()
    }
}
