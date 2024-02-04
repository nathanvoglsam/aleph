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

/// A virtual ring-buffer allocator. Controls a 'capcity' sized block of memory in some region of
/// memory managed outside of the [RingBuffer] object.
///
/// This object represents the allocation safe allocation logic. Any unsafe memory management must
/// be done by the owning object.
#[derive(Clone, Debug)]
pub struct RingBuffer {
    /// The maximum capacity of the ring buffer. Must be a power of two
    capacity: usize,

    /// The current head 'ptr' within the ring buffer
    head: usize,

    /// The number of bytes currently allocated out of the ring buffer. Also encodes the 'tail' as
    /// wrap(head - size)
    size: usize,
}

impl RingBuffer {
    /// The max capacity is constrained to having two less bits than target usize to prevent
    /// overflows when adding/freeing from the list. 2^62 bytes is probably more than enough.
    ///
    /// # 32-bit targets
    ///
    /// We're only _really_ targeting 64-bit targets, and even then 32-bit targets still get 1GB
    /// ring buffers so I think that's more than enough for any expected use for this data
    /// structure.
    pub const MAX_CAPACITY: usize = 2usize.pow(usize::BITS - 2);

    pub fn new(capacity: usize) -> Option<Self> {
        if !capacity.is_power_of_two() || capacity > Self::MAX_CAPACITY {
            None
        } else {
            Some(Self {
                capacity: capacity,
                head: 0,
                size: 0,
            })
        }
    }

    /// Allocate the given number of bytes from the ring buffer.
    ///
    /// This will allocate _at least_ the given number of bytes, but may have to allocate more to
    /// ensure a contiguous block is returned. If the requested allocation would run over the end
    /// of the ring buffer then this routine will pad the allocation with however many bytes were
    /// left before the edge of the buffer to ensure a contiguous block.
    ///
    /// The output [AllocationResult] will communicate whether this happens via
    /// [AllocationResult::allocated], which will contain the actual number of bytes allocated
    /// including any wasted padding.
    ///
    /// # Important
    ///
    /// The returned allocation will _always_ only be valid for exactly the requested number of
    /// bytes. If [AllocationResult::allocated] > 'size' then this _does not_ mean that the block
    /// is valid for [AllocationResult::allocated] bytes. Any bytes more than what was requested in
    /// 'size' are padding bytes and are not contiguous with the returned block.
    ///
    /// [AllocationResult::allocated] is provided so the caller can keep track of the true number
    /// of bytes allocated from the ring buffer so the correct value can be provided to
    /// [RingBuffer::free]. Otherwise naively assuming 'size' was allocated would lead to memory
    /// being leaked. Other strategies like taking a snapshot of [RingBuffer::size] before and after
    /// some work and freeing the difference can be used, but the information is immediately
    /// available from the algorithm so it's included in the result.
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
    #[inline]
    pub fn allocate(&mut self, size: usize) -> AllocationResult {
        assert!(
            size <= self.capacity,
            "Requested allocation larger than buffer capacity '{}'",
            self.capacity
        );

        // Check we have enough space for the allocation
        assert!(
            size <= self.size_remaining(),
            "(size) {} > (size_remaining) {}: OOM",
            size,
            self.size_remaining()
        );

        let head = self.head;
        let new_head = head + size;

        if new_head <= self.capacity {
            // If we aren't stradling the edge of the end of the ring buffer we can just consume
            // the given number of bytes and exit
            self.size += size;
            self.head = new_head;
            AllocationResult {
                ptr: head,
                allocated: size,
            }
        } else {
            self.allocate_over_buffer_edge(size)
        }
    }

    /// An extended form of [RingBuffer::allocate] that also handles aligning the resulting block
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
    #[inline]
    pub fn allocate_aligned(&mut self, size: usize, align: usize) -> AllocationResult {
        debug_assert!(align.is_power_of_two());

        // Check if the allocation is larger than the maximum size we could serve
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

        // Forward align the head pointer to the required alignment, keeping it in place if it's
        // already aligned
        let aligned_head = if self.head & (align - 1) == 0 {
            self.head
        } else {
            (self.head + align) & !(align - 1)
        };
        debug_assert!(aligned_head & (align - 1) == 0);

        let new_head = aligned_head + size;
        if new_head <= self.capacity {
            // Check we have enough space for the allocation
            let total_size = new_head - self.head;
            assert!(
                total_size <= self.size_remaining(),
                "(total_size) {} > (size_remaining) {}: OOM",
                total_size,
                self.size_remaining()
            );

            // If we aren't stradling the edge of the end of the ring buffer we can just consume
            // the total number of bytes and exit
            self.size += total_size;
            self.head = new_head;
            AllocationResult {
                ptr: aligned_head,
                allocated: total_size,
            }
        } else {
            self.allocate_over_buffer_edge(size)
        }
    }

    /// Free the given number of bytes back to the ring buffer.
    #[inline]
    pub fn free(&mut self, size: usize) {
        assert!(
            size <= self.size,
            "Tried to free more memory than the ring buffer has allocated"
        );
        self.size -= size;
    }

    /// Frees all currently allocated bytes, but leaves the head in place.
    pub fn clear(&mut self) {
        self.size = 0;
    }

    /// Returns whether the ring buffer is empty and has no bytes in use.
    pub const fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// Returns the capacity of the ring buffer in bytes.
    pub const fn capacity(&self) -> usize {
        self.capacity
    }

    /// Returns the size of the ring buffer in bytes.
    pub const fn size(&self) -> usize {
        self.size
    }

    /// Returns the size of the ring buffer in bytes.
    pub const fn size_remaining(&self) -> usize {
        self.capacity - self.size
    }

    /// Internal function for getting the capacity wrap mask.
    const fn mask(&self) -> usize {
        self.capacity - 1
    }

    #[track_caller]
    #[inline(always)]
    fn allocate_over_buffer_edge(&mut self, size: usize) -> AllocationResult {
        let new_head = self.capacity + size;
        let total_size = new_head - self.head;

        // Check we have enough space for our larger allocation
        assert!(
            total_size <= self.size_remaining(),
            "(total_size) {} > (size_remaining) {}: OOM",
            total_size,
            self.size_remaining()
        );

        // Perform our allocation with the new inflated size and wrap the head pointer around
        let new_head = new_head & self.mask();
        self.size += total_size;
        self.head = new_head;

        AllocationResult {
            ptr: 0,
            allocated: total_size,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct AllocationResult {
    /// The pointer to the start of the allocation in the ring buffer
    pub ptr: usize,

    /// The number of bytes that were _actually_ allocated, including any wastage spent wrapping
    /// the head ptr around the end of the buffer to provide a contiguous allocation.
    pub allocated: usize,
}

#[cfg(test)]
mod tests {
    use crate::ring_buffer::RingBuffer;

    #[test]
    fn test_ring_buffer_create_success() {
        RingBuffer::new(16).unwrap();
    }

    #[test]
    fn test_ring_buffer_create_success_max_capacity() {
        RingBuffer::new(RingBuffer::MAX_CAPACITY).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_ring_buffer_create_failure_npot() {
        RingBuffer::new(21).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_ring_buffer_create_failure_above_max_capacity() {
        RingBuffer::new(RingBuffer::MAX_CAPACITY + 1).unwrap();
    }

    #[test]
    fn test_ring_buffer_allocate_free() {
        let mut rb = RingBuffer::new(16).unwrap();

        let allocation = rb.allocate(4);
        assert_eq!(allocation.ptr, 0);
        assert_eq!(allocation.allocated, 4);
        assert_eq!(rb.size(), 4);
        assert_eq!(rb.size_remaining(), 12);

        let allocation = rb.allocate(2);
        assert_eq!(allocation.ptr, 4);
        assert_eq!(allocation.allocated, 2);
        assert_eq!(rb.size(), 6);
        assert_eq!(rb.size_remaining(), 10);

        rb.free(2);
        assert_eq!(rb.size(), 4);
        assert_eq!(rb.size_remaining(), 12);
    }

    #[test]
    fn test_ring_buffer_allocate_roll_over() {
        let mut rb = RingBuffer::new(16).unwrap();

        let allocation = rb.allocate(12);
        assert_eq!(allocation.ptr, 0);
        assert_eq!(allocation.allocated, 12);
        assert_eq!(rb.size(), 12);
        assert_eq!(rb.size_remaining(), 4);

        rb.free(12);
        assert_eq!(rb.size(), 0);
        assert_eq!(rb.size_remaining(), 16);

        let allocation = rb.allocate(6);
        assert_eq!(allocation.ptr, 0);
        assert_eq!(allocation.allocated, 10);
        assert_eq!(rb.size(), 10);
        assert_eq!(rb.size_remaining(), 6);

        let allocation = rb.allocate(6);
        assert_eq!(allocation.ptr, 6);
        assert_eq!(allocation.allocated, 6);
        assert_eq!(rb.size(), 16);
        assert_eq!(rb.size_remaining(), 0);
    }

    #[test]
    fn test_ring_buffer_allocate_roll_over_to_full() {
        let mut rb = RingBuffer::new(16).unwrap();

        let allocation = rb.allocate(12);
        assert_eq!(allocation.ptr, 0);
        assert_eq!(allocation.allocated, 12);
        assert_eq!(rb.size(), 12);
        assert_eq!(rb.size_remaining(), 4);

        rb.free(12);
        assert_eq!(rb.size(), 0);
        assert_eq!(rb.size_remaining(), 16);

        let allocation = rb.allocate(12);
        assert_eq!(allocation.ptr, 0);
        assert_eq!(allocation.allocated, 16);
        assert_eq!(rb.size(), 16);
        assert_eq!(rb.size_remaining(), 0);
    }

    #[test]
    fn test_ring_buffer_allocate_max_size() {
        let mut rb = RingBuffer::new(16).unwrap();

        let allocation = rb.allocate(16);
        assert_eq!(allocation.ptr, 0);
        assert_eq!(allocation.allocated, 16);
        assert_eq!(rb.size(), 16);
        assert_eq!(rb.size_remaining(), 0);

        rb.free(16);

        let allocation = rb.allocate(16);
        assert_eq!(allocation.ptr, 0);
        assert_eq!(allocation.allocated, 16);
        assert_eq!(rb.size(), 16);
        assert_eq!(rb.size_remaining(), 0);
    }

    #[test]
    #[should_panic]
    fn test_ring_buffer_allocate_oom() {
        let mut rb = RingBuffer::new(16).unwrap();

        let allocation = rb.allocate(8);
        assert_eq!(allocation.ptr, 0);
        assert_eq!(allocation.allocated, 8);
        assert_eq!(rb.size(), 8);
        assert_eq!(rb.size_remaining(), 8);

        let _allocation = rb.allocate(10);
    }

    #[test]
    fn test_ring_buffer_allocate_aligned() {
        let mut rb = RingBuffer::new(64).unwrap();

        let allocation = rb.allocate(12);
        assert_eq!(allocation.ptr, 0);
        assert_eq!(allocation.allocated, 12);
        assert_eq!(rb.size(), 12);
        assert_eq!(rb.size_remaining(), 52);

        let allocation = rb.allocate_aligned(6, 16);
        assert_eq!(allocation.ptr, 16);
        assert_eq!(allocation.allocated, 10);
        assert_eq!(rb.size(), 22);
        assert_eq!(rb.size_remaining(), 42);
    }

    #[test]
    fn test_ring_buffer_allocate_aligned_over_edge() {
        let mut rb = RingBuffer::new(64).unwrap();

        let allocation = rb.allocate(48);
        assert_eq!(allocation.ptr, 0);
        assert_eq!(allocation.allocated, 48);
        assert_eq!(rb.size(), 48);
        assert_eq!(rb.size_remaining(), 16);

        rb.free(32);

        let allocation = rb.allocate_aligned(8, 32);
        assert_eq!(allocation.ptr, 0);
        assert_eq!(allocation.allocated, 24);
        assert_eq!(rb.size(), 40);
        assert_eq!(rb.size_remaining(), 24);
    }
}
