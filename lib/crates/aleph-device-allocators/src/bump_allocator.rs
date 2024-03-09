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

use std::cell::Cell;
use std::num::NonZeroUsize;

use crate::{forward_align_offset, AllocationResult};

/// A virtual bump allocator. Controls a 'capcity' sized block of memory in some region of
/// memory managed outside of the [BumpAllocator] object.
///
/// This object represents the allocation safe allocation logic. Any unsafe memory management must
/// be done by the owning object.
#[derive(Clone, Debug)]
pub struct BumpAllocator {
    /// Head of the linked list. Bumps towards higher addresses.
    head: Cell<usize>,
    capacity: NonZeroUsize,
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

    /// Constructs a new [BumpAllocator] with the given capacity.
    ///
    /// # Info
    ///
    /// Will return [None] if `capacity` is > [BumpAllocator::MAX_CAPACITY].
    pub fn new(capacity: usize) -> Option<Self> {
        if capacity <= Self::MAX_CAPACITY {
            NonZeroUsize::new(capacity).map(|capacity| Self {
                head: Cell::new(0),
                capacity,
            })
        } else {
            None
        }
    }

    /// Allocate the given number of bytes from the bump allocator.
    ///
    /// # Alignment
    ///
    /// This routine makes no guarantees about the alignment of the returned pointer. It is the
    /// caller's responsibility to make arrangements to allow aligning the block.
    ///
    /// This could be done by requesting 'size' + 'alignment' bytes and forward aligning with some
    /// memory wasted. You could also coordinate your 'size' requests to always be a multiple of
    /// some minimum alignment. Because [RingBuffer] does not own any memory it is not _unsafe_ for
    /// this data structure to provide unaligned 'pointers'.
    ///
    /// There are also a suite of utilities available for aligned allocation.
    ///
    /// It is also important to note that the allocator can't provide alignment higher than the
    /// alignment of the block you're allocating from (and creating the pointers from) as there's
    /// no way for this utility to know that alignment.
    pub fn allocate(&self, size: usize) -> AllocationResult {
        assert!(size <= self.size_remaining(), "OOM");

        let head = self.head.get();
        self.head.set(head + size);

        AllocationResult {
            offset: head,
            allocated: size,
        }
    }

    /// An extended form of [BumpAllocator::allocate] that also handles aligning the resulting block
    /// to the requested alignment. This may allocate more memory than 'size' to satisfy the
    /// requested alignment. The allocator may forward align the block and consume additional memory
    /// to do so via padding.
    ///
    /// # Warning
    ///
    /// 'align' must be a power of two. Otherwise the algorithm implodes. This function isn't unsafe
    /// because an incorrect alignment can't do anything memory unsafe, only the caller can. It's
    /// the caller's responsibility to ensure 'align' is a power of two and it's the caller's
    /// responsibility to not do anything unsafe with the offsets this allocator yields.
    pub fn allocate_aligned(&self, size: usize, align: usize) -> AllocationResult {
        debug_assert!(align.is_power_of_two());

        assert!(
            size <= self.capacity.get(),
            "Requested allocation larger than buffer capacity '{}'",
            self.capacity
        );
        assert!(
            align <= self.capacity.get(),
            "Requested alignment larger than buffer capacity '{}'",
            self.capacity
        );

        let head = self.head.get();
        let aligned_head = forward_align_offset(head, align);
        let new_head = aligned_head + size;
        let total_size = new_head - head;

        assert!(
            total_size <= self.size_remaining(),
            "(total_size) {} > (size_remaining) {}: OOM",
            total_size,
            self.size_remaining()
        );

        self.head.set(new_head);

        AllocationResult {
            offset: aligned_head,
            allocated: total_size,
        }
    }

    /// Clear the bump allocator, resetting it to the empty state
    pub fn clear(&self) {
        self.head.set(0);
    }

    /// The total capacity the bump allocator can allocate for
    pub const fn capacity(&self) -> usize {
        self.capacity.get()
    }

    /// The current number of bytes allocated from the allocator
    #[inline]
    pub fn size(&self) -> usize {
        self.head.get()
    }

    /// The number of bytes remaining that can still be allocated from the allocator
    #[inline]
    pub fn size_remaining(&self) -> usize {
        self.capacity() - self.size()
    }
}

#[cfg(test)]
mod tests {
    use crate::BumpAllocator;

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
        let ba = BumpAllocator::new(16).unwrap();

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
        let ba = BumpAllocator::new(16).unwrap();

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
        let ba = BumpAllocator::new(16).unwrap();

        let allocation = ba.allocate(8);
        assert_eq!(allocation.offset, 0);
        assert_eq!(allocation.allocated, 8);
        assert_eq!(ba.size(), 8);
        assert_eq!(ba.size_remaining(), 8);

        let _allocation = ba.allocate(10);
    }

    #[test]
    fn test_bump_allocator_allocate_aligned() {
        let ba = BumpAllocator::new(64).unwrap();

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
