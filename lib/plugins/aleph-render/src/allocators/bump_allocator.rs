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

use crate::allocators::{forward_align_offset, AllocationResult};

pub struct BumpAllocator {
    pub head: usize,
    pub capacity: usize,
}

impl BumpAllocator {
    /// The max capacity is constrained to having two less bits than target usize to prevent
    /// overflows when adding/freeing from the list. 2^62 bytes is probably more than enough.
    ///
    /// # 32-bit targets
    ///
    /// We're only _really_ targeting 64-bit targets, and even then 32-bit targets still get 1GB
    /// buffers so I think that's more than enough for any expected use for this data structure.
    pub const MAX_CAPACITY: usize = 2usize.pow(usize::BITS - 2);

    pub fn new(capacity: usize) -> Option<Self> {
        if capacity <= Self::MAX_CAPACITY {
            Some(Self { head: 0, capacity })
        } else {
            None
        }
    }

    pub fn allocate(&mut self, size: usize) -> AllocationResult {
        assert!(size <= self.size_remaining(), "OOM");

        let head = self.head;
        self.head += size;
        AllocationResult {
            offset: head,
            allocated: size,
        }
    }

    pub fn allocate_aligned(&mut self, size: usize, align: usize) -> AllocationResult {
        debug_assert!(align.is_power_of_two());

        assert!(
            size <= self.capacity,
            "Requested allocation larger than buffer capacity '{}'",
            self.capacity
        );
        assert!(
            align <= self.capacity,
            "Requested alignment larger than buffer capacity '{}'",
            self.capacity
        );

        let aligned_head = forward_align_offset(self.head, align);
        let new_head = aligned_head + size;
        let total_size = new_head - self.head;

        assert!(
            total_size <= self.size_remaining(),
            "(total_size) {} > (size_remaining) {}: OOM",
            total_size,
            self.size_remaining()
        );

        self.head = new_head;
        AllocationResult {
            offset: aligned_head,
            allocated: total_size,
        }
    }

    pub fn clear(&mut self) {
        self.head = 0;
    }

    pub const fn capacity(&self) -> usize {
        self.capacity
    }

    pub const fn size(&self) -> usize {
        self.head
    }

    pub const fn size_remaining(&self) -> usize {
        self.capacity - self.head
    }
}

#[cfg(test)]
mod tests {
    use crate::allocators::BumpAllocator;

    #[test]
    fn test_bump_allocator_create_success() {
        BumpAllocator::new(16).unwrap();
    }

    #[test]
    fn test_bump_allocator_create_success_max_capacity() {
        BumpAllocator::new(BumpAllocator::MAX_CAPACITY).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_bump_allocator_create_failure_above_max_capacity() {
        BumpAllocator::new(BumpAllocator::MAX_CAPACITY + 1).unwrap();
    }

    #[test]
    fn test_bump_allocator_allocate() {
        let mut ba = BumpAllocator::new(16).unwrap();

        let allocation = ba.allocate(4);
        assert_eq!(allocation.offset, 0);
        assert_eq!(allocation.allocated, 4);
        assert_eq!(ba.size(), 4);
        assert_eq!(ba.size_remaining(), 12);

        let allocation = ba.allocate(2);
        assert_eq!(allocation.offset, 4);
        assert_eq!(allocation.allocated, 2);
        assert_eq!(ba.size(), 6);
        assert_eq!(ba.size_remaining(), 10);
    }

    #[test]
    fn test_bump_allocator_allocate_max_size() {
        let mut ba = BumpAllocator::new(16).unwrap();

        let allocation = ba.allocate(16);
        assert_eq!(allocation.offset, 0);
        assert_eq!(allocation.allocated, 16);
        assert_eq!(ba.size(), 16);
        assert_eq!(ba.size_remaining(), 0);

        ba.clear();

        let allocation = ba.allocate(16);
        assert_eq!(allocation.offset, 0);
        assert_eq!(allocation.allocated, 16);
        assert_eq!(ba.size(), 16);
        assert_eq!(ba.size_remaining(), 0);
    }

    #[test]
    #[should_panic]
    fn test_bump_allocator_allocate_oom() {
        let mut ba = BumpAllocator::new(16).unwrap();

        let allocation = ba.allocate(8);
        assert_eq!(allocation.offset, 0);
        assert_eq!(allocation.allocated, 8);
        assert_eq!(ba.size(), 8);
        assert_eq!(ba.size_remaining(), 8);

        let _allocation = ba.allocate(10);
    }

    #[test]
    fn test_bump_allocator_allocate_aligned() {
        let mut ba = BumpAllocator::new(64).unwrap();

        let allocation = ba.allocate(12);
        assert_eq!(allocation.offset, 0);
        assert_eq!(allocation.allocated, 12);
        assert_eq!(ba.size(), 12);
        assert_eq!(ba.size_remaining(), 52);

        let allocation = ba.allocate_aligned(6, 16);
        assert_eq!(allocation.offset, 16);
        assert_eq!(allocation.allocated, 10);
        assert_eq!(ba.size(), 22);
        assert_eq!(ba.size_remaining(), 42);
    }
}
