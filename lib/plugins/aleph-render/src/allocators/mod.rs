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

mod bump_allocator;
mod ring_buffer;
mod uniforms_bump_allocator;
mod uniforms_ring_buffer;

pub use bump_allocator::BumpAllocator;
pub use ring_buffer::RingBuffer;
pub use uniforms_bump_allocator::UniformsBumpAllocator;
pub use uniforms_ring_buffer::UniformsRingBuffer;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct AllocationResult {
    /// The pointer to the start of the allocation in the ring buffer
    pub offset: usize,

    /// The number of bytes that were _actually_ allocated, including any wastage spent wrapping
    /// the head ptr around the end of the buffer to provide a contiguous allocation.
    pub allocated: usize,
}

pub struct UniformsAllocationResult {
    /// The offset from the start of the buffer that the allocated block starts at in the device's
    /// address space.
    pub device_offset: usize,

    /// Pointer to the start of the block in the host's address space. There is no alignment
    /// guarantees on this pointer.
    pub host_address: std::ptr::NonNull<u8>,

    /// The actual number of bytes allocated for the block, including any padding bytes needed to
    /// wrap over the end of the ring buffer.
    pub allocated: usize,
}

pub struct SubAllocatorResult<T> {
    /// The suballocator object we allocated from the buffer.
    pub allocator: T,

    /// The actual number of bytes consumed to meet the allocation request.
    pub allocated: usize,
}

pub(crate) fn forward_align_offset(v: usize, align: usize) -> usize {
    // Forward align the head pointer to the required alignment, keeping it in place if it's
    // already aligned
    let aligned = if v & (align - 1) == 0 {
        v
    } else {
        (v + align) & !(align - 1)
    };
    debug_assert!(aligned & (align - 1) == 0);
    aligned
}
