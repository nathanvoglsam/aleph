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

use std::num::NonZero;
use std::ptr::NonNull;

use aleph_alloc::BVec;
use aleph_alloc::instrumentation::system;
use aleph_rhi_impl_utils::RhiSystem;

use crate::device::Device;
use crate::internal::memory_block::MemoryBlock;

const BLOCK_SIZE: usize = 1024 * 32;

/// A simple bump allocator that manages a growable arena.
///
/// This is intended to be used for managing upload memory for push constants and push parameters.
/// The command encoder needs to be able to allocate one-time-use upload memory.
///
/// This API is intended for serving allocations substantially smaller than the block size. The
/// implementation will waste some memory as internal fragmentation if the active block doesn't have
/// enough space to serve an allocation. Because a new block must be made to serve the request some
/// memory will be left unused in most blocks. The smaller the size of your allocations, the less
/// memory you will waste.
///
/// Allocates new blocks as needed to serve allocations. Grows the internal block size
/// geometrically.
pub struct UploadBumpAllocator {
    active_block: MemoryBlock,
    full_blocks: BVec<MemoryBlock, RhiSystem>,
    cursor: usize,
}

impl UploadBumpAllocator {
    /// Constructs a new empty allocator.
    ///
    /// Eagerly allocates the first active block.
    pub fn new(device: &Device) -> Option<Self> {
        let out = Self {
            active_block: MemoryBlock::new(device, BLOCK_SIZE)?,
            full_blocks: BVec::new_in(system()),
            cursor: 0,
        };
        Some(out)
    }

    /// Allocate 'len' bytes out of this allocator, and return a (cpu addr, gpu addr) pair. The
    /// lifetime of the memory is tied to the lifetime of the allocator it comes from.
    ///
    /// # Panics
    ///
    /// - If len > BLOCK_SIZE. Allocations can't straddle blocks, so the largest single allocation
    ///   we can serve is BLOCK_SIZE.
    pub fn allocate(&mut self, device: &Device, len: usize) -> (NonNull<u8>, NonZero<u64>) {
        // round up to u64 granularity. also guarantees 8 byte alignment for all the blocks we
        // allocate.
        let len = len.next_multiple_of(size_of::<u64>());

        assert!(
            len <= BLOCK_SIZE,
            "Can't serve an allocation greater than the size of a block"
        );

        let new_cursor = self.cursor + len;
        if new_cursor >= self.active_block.len {
            // Allocate a new block, swap it into the place of the 'active' block and then place the
            // old 'active' block (which is now full) into the 'full blocks' list.
            //
            // To reduce the number of individual buffers we allocate we grow the block size
            // geometrically.
            let mut block = MemoryBlock::new(device, self.next_block_size()).unwrap();
            std::mem::swap(&mut self.active_block, &mut block);
            self.full_blocks.push(block);

            // We consume the first 'len' bytes in the new block when allocating a new block.
            let cpu_addr = self.active_block.cpu_addr;
            let gpu_addr = self.active_block.gpu_addr;

            // New block assumes an old cursor value of 0.
            //
            // That means the true new_cursor is just len.
            self.cursor = len;

            (cpu_addr, gpu_addr)
        } else {
            // There's enough space in the active block so get the addresses as offsets based on the
            // 'new_cursor'. Increment the cursor and return is all we need to do here.
            let cpu_addr = unsafe { self.active_block.cpu_addr.add(new_cursor) };
            let gpu_addr = self.active_block.gpu_addr.saturating_add(new_cursor as u64);

            self.cursor = new_cursor;

            (cpu_addr, gpu_addr)
        }
    }

    fn next_block_size(&self) -> usize {
        BLOCK_SIZE * self.full_blocks.len().max(1)
    }
}
