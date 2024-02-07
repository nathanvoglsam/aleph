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

//
// MIT License
// 
// Copyright (c) 2023 Sebastian Aaltonen
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

type NodeIndex = u32;

const NUM_TOP_BINS: u32 = 32;
const BINS_PER_LEAF: u32 = 8;
const TOP_BINS_INDEX_SHIFT: u32 = 3;
const LEAF_BINS_INDEX_MASK: u32 = 0x7;
const NUM_LEAF_BINS: u32 = NUM_TOP_BINS * BINS_PER_LEAF;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Allocation {
    pub offset: u32,
    metadata: NodeIndex,
}

impl Allocation {
    const NO_SPACE: u32 = 0xFFFFFFFF;
}

impl Default for Allocation {
    fn default() -> Self {
        Self {
            offset: u32::MAX,
            metadata: NodeIndex::MAX,
        }
    }
}

#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct StorageReport {
    pub total_free_space: u32,
    pub largest_free_region: u32,
}

#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct StorageReportRegion {
    pub size: u32,
    pub count: u32,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct StorageReportFull {
    pub free_regions: [StorageReportRegion; NUM_LEAF_BINS as usize],
}

impl Default for StorageReportFull {
    fn default() -> Self {
        Self {
            free_regions: [Default::default(); 256],
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Node {
    data_offset: u32,
    data_size: u32,
    bin_list_prev: NodeIndex,
    bin_list_next: NodeIndex,
    neighbor_prev: NodeIndex,
    neighbor_next: NodeIndex,
    used: bool,
}

impl Node {
    pub const UNUSED: NodeIndex = NodeIndex::MAX;
}

impl Default for Node {
    fn default() -> Self {
        Self {
            data_offset: 0,
            data_size: 0,
            bin_list_prev: Self::UNUSED,
            bin_list_next: Self::UNUSED,
            neighbor_prev: Self::UNUSED,
            neighbor_next: Self::UNUSED,
            used: false,
        }
    }
}

pub struct OffsetAllocator {
    size: u32,
    max_allocs: u32,
    free_storage: u32,
    used_bins_top: u32,
    used_bins: [u8; NUM_TOP_BINS as usize],
    bin_indices: [NodeIndex; NUM_LEAF_BINS as usize],
    nodes: Box<[Node]>,
    free_nodes: Box<[NodeIndex]>,
    free_offset: u32,
}

impl OffsetAllocator {
    pub const DEFAULT_MAX_ALLOCS: u32 = 128 * 1024;

    pub fn new(size: u32, max_allocs: u32) -> Self {
        let free_storage = 0;
        let used_bins_top = 0;
        let free_offset = max_allocs - 1;

        let used_bins = [0; NUM_TOP_BINS as usize];
        let bin_indices = [Node::UNUSED; NUM_LEAF_BINS as usize];

        let nodes = vec![Node::default(); max_allocs as usize].into_boxed_slice();
        let mut free_nodes = vec![0; max_allocs as usize].into_boxed_slice();
        free_nodes
            .iter_mut()
            .enumerate()
            .for_each(|(i, v)| *v = max_allocs - (i as u32) - 1);

        let mut out = Self {
            size,
            max_allocs,
            free_storage,
            used_bins_top,
            used_bins,
            bin_indices,
            nodes,
            free_nodes,
            free_offset,
        };

        out.insert_node_into_bin(out.size, 0);

        out
    }

    pub fn allocate(&mut self, size: u32) -> Allocation {
        // Out of allocations?
        if self.free_offset == 0 {
            return Allocation {
                offset: Allocation::NO_SPACE,
                metadata: Allocation::NO_SPACE,
            };
        }

        // Round up to bin index to ensure that alloc >= bin
        // Gives us min bin index that fits the size
        let min_bin_index = small_float::uint_to_float_round_up(size);

        let min_top_bin_index = min_bin_index >> TOP_BINS_INDEX_SHIFT;
        let min_leaf_bin_index = min_bin_index & LEAF_BINS_INDEX_MASK;

        let mut top_bin_index = min_top_bin_index;
        let mut leaf_bin_index = Allocation::NO_SPACE;

        // If top bin exists, scan its leaf bin. This can fail (NO_SPACE).
        if (self.used_bins_top & (1 << top_bin_index)) != 0 {
            leaf_bin_index = find_lowest_set_bit_after(
                self.used_bins[top_bin_index as usize] as u32,
                min_leaf_bin_index,
            );
        }

        // If we didn't find space in top bin, we search top bin from +1
        if leaf_bin_index == Allocation::NO_SPACE {
            top_bin_index = find_lowest_set_bit_after(self.used_bins_top, min_top_bin_index + 1);

            // Out of space?
            if top_bin_index == Allocation::NO_SPACE {
                return Allocation {
                    offset: Allocation::NO_SPACE,
                    metadata: Allocation::NO_SPACE,
                };
            }

            // All leaf bins here fit the alloc, since the top bin was rounded up. Start leaf search from bit 0.
            // NOTE: This search can't fail since at least one leaf bit was set because the top bit was set.
            leaf_bin_index = (self.used_bins[top_bin_index as usize] as u32).trailing_zeros();
        }

        let bin_index = (top_bin_index << TOP_BINS_INDEX_SHIFT) | leaf_bin_index;

        // Pop the top node of the bin. Bin top = node.next.
        let node_index = self.bin_indices[bin_index as usize];
        let node = &mut self.nodes[node_index as usize];
        let node_total_size = node.data_size;
        node.data_size = size;
        node.used = true;
        self.bin_indices[bin_index as usize] = node.bin_list_next;
        if node.bin_list_next != Node::UNUSED {
            let i = node.bin_list_next as usize;
            self.nodes[i].bin_list_prev = Node::UNUSED;
        }
        self.free_storage -= node_total_size;

        // Bin empty?
        if self.bin_indices[bin_index as usize] == Node::UNUSED {
            // Remove a leaf bin mask bit
            self.used_bins[top_bin_index as usize] &= !(1 << leaf_bin_index);

            // All leaf bins empty?
            if self.used_bins[top_bin_index as usize] == 0 {
                // Remove a top bin mask bit
                self.used_bins_top &= !(1 << top_bin_index);
            }
        }

        // Push back reminder N elements to a lower bin
        let reminder_size = node_total_size - size;
        if reminder_size > 0 {
            let node = &self.nodes[node_index as usize];
            let new_node_index = self.insert_node_into_bin(reminder_size, node.data_offset + size);

            // Link nodes next to each other so that we can merge them later if both are free
            // And update the old next neighbor to point to the new node (in middle)
            let node = &self.nodes[node_index as usize];
            if node.neighbor_next != Node::UNUSED {
                let i = node.neighbor_next as usize;
                self.nodes[i].neighbor_prev = new_node_index;
            }
            self.nodes[new_node_index as usize].neighbor_prev = node_index;

            let v = self.nodes[node_index as usize].neighbor_next;
            self.nodes[new_node_index as usize].neighbor_next = v;

            self.nodes[node_index as usize].neighbor_next = new_node_index;
        }

        let node = &self.nodes[node_index as usize];

        return Allocation {
            offset: node.data_offset,
            metadata: node_index,
        };
    }

    pub fn free(&mut self, allocation: Allocation) {
        assert_ne!(allocation.metadata, Allocation::NO_SPACE);

        let node_index = allocation.metadata;
        let node = &self.nodes[node_index as usize].clone();

        // Double delete check
        assert!(node.used);

        // Merge with neighbors...
        let mut offset = node.data_offset;
        let mut size = node.data_size;

        if (node.neighbor_prev != Node::UNUSED)
            && (self.nodes[node.neighbor_prev as usize].used == false)
        {
            // Previous (contiguous) free node: Change offset to previous node offset. Sum sizes
            let prev_node = self.nodes[node.neighbor_prev as usize].clone();
            offset = prev_node.data_offset;
            size += prev_node.data_size;

            // Remove node from the bin linked list and put it in the freelist
            self.remove_node_from_bin(node.neighbor_prev);

            assert_eq!(prev_node.neighbor_next, node_index);
            self.nodes[node_index as usize].neighbor_prev = prev_node.neighbor_prev;
        }

        let node = &self.nodes[node_index as usize];

        if (node.neighbor_next != Node::UNUSED)
            && (self.nodes[node.neighbor_next as usize].used == false)
        {
            // Next (contiguous) free node: Offset remains the same. Sum sizes.
            let next_node = self.nodes[node.neighbor_next as usize].clone();
            size += next_node.data_size;

            // Remove node from the bin linked list and put it in the freelist
            self.remove_node_from_bin(node.neighbor_next);

            assert_eq!(next_node.neighbor_prev, node_index);
            self.nodes[node_index as usize].neighbor_next = next_node.neighbor_next;
        }

        let node = &self.nodes[node_index as usize];

        let neighbor_next = node.neighbor_next;
        let neighbor_prev = node.neighbor_prev;

        // Insert the removed node to freelist
        self.free_offset += 1;
        self.free_nodes[self.free_offset as usize] = node_index;

        // Insert the (combined) free node to bin
        let combined_node_index = self.insert_node_into_bin(size, offset);

        // Connect neighbors with the new combined node
        if neighbor_next != Node::UNUSED {
            self.nodes[combined_node_index as usize].neighbor_next = neighbor_next;
            self.nodes[neighbor_next as usize].neighbor_prev = combined_node_index;
        }
        if neighbor_prev != Node::UNUSED {
            self.nodes[combined_node_index as usize].neighbor_prev = neighbor_prev;
            self.nodes[neighbor_prev as usize].neighbor_next = combined_node_index;
        }
    }

    pub fn reset(&mut self) {
        self.free_storage = 0;
        self.used_bins_top = 0;
        self.free_offset = self.max_allocs - 1;

        self.used_bins.iter_mut().for_each(|v| *v = 0);
        self.bin_indices.iter_mut().for_each(|v| *v = Node::UNUSED);

        self.nodes = vec![Node::default(); self.max_allocs as usize].into_boxed_slice();
        self.free_nodes = vec![0; self.max_allocs as usize].into_boxed_slice();
        self.free_nodes
            .iter_mut()
            .enumerate()
            .for_each(|(i, v)| *v = self.max_allocs - (i as u32) - 1);

        self.insert_node_into_bin(self.size, 0);
    }

    pub fn allocation_size(&self, allocation: Allocation) -> u32 {
        if allocation.metadata == Allocation::NO_SPACE {
            return 0;
        };

        self.nodes[allocation.metadata as usize].data_size
    }

    pub fn storage_report(&self) -> StorageReport {
        let mut largest_free_region = 0;
        let mut free_storage = 0;

        // Out of allocations? -> Zero free space
        if self.free_offset > 0 {
            free_storage = self.free_storage;
            if self.used_bins_top != 0 {
                let top_bin_index = 31 - self.used_bins_top.leading_zeros();
                let leaf_bin_index = 31 - (self.used_bins[top_bin_index as usize] as u32).leading_zeros();
                largest_free_region = small_float::float_to_uint(
                    (top_bin_index << TOP_BINS_INDEX_SHIFT) | leaf_bin_index,
                );
                assert!(
                    free_storage >= largest_free_region,
                    "{free_storage} < {largest_free_region}"
                );
            }
        }

        StorageReport {
            total_free_space: free_storage,
            largest_free_region,
        }
    }

    pub fn storage_report_full(&self) -> StorageReportFull {
        let mut report = StorageReportFull::default();
        for i in 0..NUM_LEAF_BINS {
            let mut count = 0;
            let mut node_index = self.bin_indices[i as usize];
            while node_index != Node::UNUSED {
                node_index = self.nodes[node_index as usize].bin_list_next;
                count += 1;
            }
            report.free_regions[i as usize] = StorageReportRegion {
                size: small_float::float_to_uint(i),
                count,
            };
        }
        report
    }

    fn insert_node_into_bin(&mut self, size: u32, data_offset: u32) -> u32 {
        // Round down to bin index to ensure that bin >= alloc
        let bin_index = small_float::uint_to_float_round_down(size);

        let top_bin_index = bin_index >> TOP_BINS_INDEX_SHIFT;
        let leaf_bin_index = bin_index & LEAF_BINS_INDEX_MASK;

        // Bin was empty before?
        if self.bin_indices[bin_index as usize] == Node::UNUSED {
            // Set bin mask bits
            self.used_bins[top_bin_index as usize] |= 1 << leaf_bin_index;
            self.used_bins_top |= 1 << top_bin_index;
        }

        // Take a freelist node and insert on top of the bin linked list (next = old top)
        let top_node_index = self.bin_indices[bin_index as usize];
        let node_index = self.free_nodes[self.free_offset as usize];
        self.free_offset -= 1;
        self.nodes[node_index as usize] = Node {
            data_offset: data_offset,
            data_size: size,
            bin_list_next: top_node_index,
            ..Default::default()
        };
        if top_node_index != Node::UNUSED {
            self.nodes[top_node_index as usize].bin_list_prev = node_index;
        }
        self.bin_indices[bin_index as usize] = node_index;

        self.free_storage += size;

        node_index
    }

    fn remove_node_from_bin(&mut self, node_index: u32) {
        let node = &self.nodes[node_index as usize];

        if node.bin_list_prev != Node::UNUSED {
            // Easy case: We have previous node. Just remove this node from the middle of the list.
            self.nodes[node.bin_list_prev as usize].bin_list_next = node.bin_list_next;
            let node = &self.nodes[node_index as usize].clone();
            if node.bin_list_next != Node::UNUSED {
                self.nodes[node.bin_list_next as usize].bin_list_prev = node.bin_list_prev;
            }
        } else {
            // Hard case: We are the first node in a bin. Find the bin.

            // Round down to bin index to ensure that bin >= alloc
            let bin_index = small_float::uint_to_float_round_down(node.data_size);

            let top_bin_index = bin_index >> TOP_BINS_INDEX_SHIFT;
            let leaf_bin_index = bin_index & LEAF_BINS_INDEX_MASK;

            self.bin_indices[bin_index as usize] = node.bin_list_next;
            if node.bin_list_next != Node::UNUSED {
                self.nodes[node.bin_list_next as usize].bin_list_prev = Node::UNUSED;
            }

            // Bin empty?
            if self.bin_indices[bin_index as usize] == Node::UNUSED {
                // Remove a leaf bin mask bit
                self.used_bins[top_bin_index as usize] &= !(1 << leaf_bin_index);

                // All leaf bins empty?
                if self.used_bins[top_bin_index as usize] == 0 {
                    // Remove a top bin mask bit
                    self.used_bins_top &= !(1 << top_bin_index);
                }
            }
        }

        let node = &self.nodes[node_index as usize];

        // Insert the node to freelist
        self.free_offset += 1;
        self.free_nodes[self.free_offset as usize] = node_index;

        self.free_storage -= node.data_size;
    }
}

mod small_float {
    pub const MANTISSA_BITS: u32 = 3;
    pub const MANTISSA_VALUE: u32 = 1 << MANTISSA_BITS;
    pub const MANTISSA_MASK: u32 = MANTISSA_VALUE - 1;

    // Bin sizes follow floating point (exponent + mantissa) distribution (piecewise linear log approx)
    // This ensures that for each size class, the average overhead percentage stays the same
    pub fn uint_to_float_round_up(size: u32) -> u32 {
        let mut exp = 0;
        let mut mantissa;

        if size < MANTISSA_VALUE {
            // Denorm: 0..(MANTISSA_VALUE-1)
            mantissa = size;
        } else {
            // Normalized: Hidden high bit always 1. Not stored. Just like float.
            let leading_zeros: u32 = size.leading_zeros();
            let highest_set_bit = 31 - leading_zeros;

            let mantissa_start_bit = highest_set_bit - MANTISSA_BITS;
            exp = mantissa_start_bit + 1;
            mantissa = (size >> mantissa_start_bit) & MANTISSA_MASK;

            let low_bits_mask = (1 << mantissa_start_bit) - 1;

            // Round up!
            if (size & low_bits_mask) != 0 {
                mantissa += 1;
            }
        }

        (exp << MANTISSA_BITS) + mantissa // + allows mantissa->exp overflow for round up
    }

    pub fn uint_to_float_round_down(size: u32) -> u32 {
        let mut exp = 0;
        let mantissa;

        if size < MANTISSA_VALUE {
            // Denorm: 0..(MANTISSA_VALUE-1)
            mantissa = size;
        } else {
            // Normalized: Hidden high bit always 1. Not stored. Just like float.
            let leading_zeros: u32 = size.leading_zeros();
            let highest_set_bit = 31 - leading_zeros;

            let mantissa_start_bit = highest_set_bit - MANTISSA_BITS;
            exp = mantissa_start_bit + 1;
            mantissa = (size >> mantissa_start_bit) & MANTISSA_MASK;
        }

        (exp << MANTISSA_BITS) | mantissa
    }

    pub fn float_to_uint(float_value: u32) -> u32 {
        let exponent = float_value >> MANTISSA_BITS;
        let mantissa = float_value & MANTISSA_MASK;
        if exponent == 0 {
            // Denorms
            mantissa
        } else {
            (mantissa | MANTISSA_VALUE) << (exponent - 1)
        }
    }
}

fn find_lowest_set_bit_after(bit_mask: u32, start_bit_index: u32) -> u32 {
    let mask_before_start_index = (1 << start_bit_index) - 1;
    let mask_after_start_index = !mask_before_start_index;
    let bits_after = bit_mask & mask_after_start_index;
    if bits_after == 0 {
        Allocation::NO_SPACE
    } else {
        bits_after.trailing_zeros()
    }
}

#[cfg(test)]
mod tests {
    use crate::offset_allocator::small_float;
    use crate::offset_allocator::Allocation;
    use crate::offset_allocator::OffsetAllocator;

    #[test]
    fn numbers_small_float_uint_to_float() {
        // Denorms, exp=1 and exp=2 + mantissa = 0 are all precise.
        // NOTE: Assuming 8 value (3 bit) mantissa.
        // If this test fails, please change this assumption!
        let precise_number_count = 17u32;
        for i in 0..precise_number_count {
            let round_up = small_float::uint_to_float_round_up(i);
            let round_down = small_float::uint_to_float_round_down(i);
            assert_eq!(i, round_up);
            assert_eq!(i, round_down);
        }

        // Test some random picked numbers
        struct NumberFloatUpDown {
            number: u32,
            up: u32,
            down: u32,
        }

        let test_data = [
            NumberFloatUpDown {
                number: 17,
                up: 17,
                down: 16,
            },
            NumberFloatUpDown {
                number: 118,
                up: 39,
                down: 38,
            },
            NumberFloatUpDown {
                number: 1024,
                up: 64,
                down: 64,
            },
            NumberFloatUpDown {
                number: 65536,
                up: 112,
                down: 112,
            },
            NumberFloatUpDown {
                number: 529445,
                up: 137,
                down: 136,
            },
            NumberFloatUpDown {
                number: 1048575,
                up: 144,
                down: 143,
            },
        ];

        for v in test_data {
            let round_up = small_float::uint_to_float_round_up(v.number);
            let round_down = small_float::uint_to_float_round_down(v.number);
            assert_eq!(round_up, v.up);
            assert_eq!(round_down, v.down);
        }
    }

    #[test]
    fn numbers_small_float_float_to_uint() {
        // Denorms, exp=1 and exp=2 + mantissa = 0 are all precise.
        // NOTE: Assuming 8 value (3 bit) mantissa.
        // If this test fails, please change this assumption!
        let precise_number_count = 17u32;
        for i in 0..precise_number_count {
            let v = small_float::float_to_uint(i);
            assert_eq!(i, v);
        }

        // Test that float->uint->float conversion is precise for all numbers
        // NOTE: Test values < 240. 240->4G = overflows 32 bit integer
        for i in 0..240u32 {
            let v = small_float::float_to_uint(i);
            let round_up = small_float::uint_to_float_round_up(v);
            let round_down = small_float::uint_to_float_round_down(v);
            assert_eq!(i, round_up);
            assert_eq!(i, round_down);
        }
    }

    #[test]
    fn basic_offset_allocator() {
        let mut allocator =
            OffsetAllocator::new(1024 * 1024 * 256, OffsetAllocator::DEFAULT_MAX_ALLOCS);
        let a = allocator.allocate(1337);
        let offset = a.offset;
        assert_eq!(offset, 0);
        allocator.free(a);
    }

    #[test]
    fn allocate_offset_allocator_simple() {
        let mut allocator =
            OffsetAllocator::new(1024 * 1024 * 256, OffsetAllocator::DEFAULT_MAX_ALLOCS);
        // Free merges neighbor empty nodes. Next allocation should also have offset = 0
        let a = allocator.allocate(0);
        assert_eq!(a.offset, 0);

        let b = allocator.allocate(1);
        assert_eq!(b.offset, 0);

        let c = allocator.allocate(123);
        assert_eq!(c.offset, 1);

        let d = allocator.allocate(1234);
        assert_eq!(d.offset, 124);

        allocator.free(a);
        allocator.free(b);
        allocator.free(c);
        allocator.free(d);

        // End: Validate that allocator has no fragmentation left. Should be 100% clean.
        let validate_all = allocator.allocate(1024 * 1024 * 256);
        assert!(validate_all.offset == 0);
        allocator.free(validate_all);
    }

    #[test]
    fn allocate_offset_allocator_merge_trivial() {
        let mut allocator =
            OffsetAllocator::new(1024 * 1024 * 256, OffsetAllocator::DEFAULT_MAX_ALLOCS);

        // Free merges neighbor empty nodes. Next allocation should also have offset = 0
        let a = allocator.allocate(1337);
        assert_eq!(a.offset, 0);
        allocator.free(a);

        let b = allocator.allocate(1337);
        assert_eq!(b.offset, 0);
        allocator.free(b);

        // End: Validate that allocator has no fragmentation left. Should be 100% clean.
        let validate_all = allocator.allocate(1024 * 1024 * 256);
        assert_eq!(validate_all.offset, 0);
        allocator.free(validate_all);
    }

    #[test]
    fn allocate_offset_allocator_reuse_trivial() {
        let mut allocator =
            OffsetAllocator::new(1024 * 1024 * 256, OffsetAllocator::DEFAULT_MAX_ALLOCS);

        // Allocator should reuse node freed by A since the allocation C fits in the same bin (using pow2 size to be sure)
        let a = allocator.allocate(1024);
        assert_eq!(a.offset, 0);

        let b = allocator.allocate(3456);
        assert_eq!(b.offset, 1024);

        allocator.free(a);

        let c = allocator.allocate(1024);
        assert_eq!(c.offset, 0);

        allocator.free(c);
        allocator.free(b);

        // End: Validate that allocator has no fragmentation left. Should be 100% clean.
        let validate_all = allocator.allocate(1024 * 1024 * 256);
        assert_eq!(validate_all.offset, 0);
        allocator.free(validate_all);
    }

    #[test]
    fn allocate_offset_allocator_reuse_complex() {
        let mut allocator =
            OffsetAllocator::new(1024 * 1024 * 256, OffsetAllocator::DEFAULT_MAX_ALLOCS);

        // Allocator should not reuse node freed by A since the allocation C doesn't fits in the same bin
        // However node D and E fit there and should reuse node from A
        let a = allocator.allocate(1024);
        assert!(a.offset == 0);

        let b = allocator.allocate(3456);
        assert!(b.offset == 1024);

        allocator.free(a);

        let c = allocator.allocate(2345);
        assert!(c.offset == 1024 + 3456);

        let d = allocator.allocate(456);
        assert!(d.offset == 0);

        let e = allocator.allocate(512);
        assert!(e.offset == 456);

        let report = allocator.storage_report();
        assert!(report.total_free_space == 1024 * 1024 * 256 - 3456 - 2345 - 456 - 512);
        assert!(report.largest_free_region != report.total_free_space);

        allocator.free(c);
        allocator.free(d);
        allocator.free(b);
        allocator.free(e);

        // End: Validate that allocator has no fragmentation left. Should be 100% clean.
        let validate_all = allocator.allocate(1024 * 1024 * 256);
        assert!(validate_all.offset == 0);
        allocator.free(validate_all);
    }

    #[test]
    fn allocate_offset_allocator_zero_fragmentation() {
        let mut allocator =
            OffsetAllocator::new(1024 * 1024 * 256, OffsetAllocator::DEFAULT_MAX_ALLOCS);

        // Allocate 256x 1MB. Should fit. Then free four random slots and reallocate four slots.
        // Plus free four contiguous slots an allocate 4x larger slot. All must be zero fragmentation!
        let mut allocations = [Allocation::default(); 256];
        for i in 0..256 {
            allocations[i] = allocator.allocate(1024 * 1024);
            assert_eq!(allocations[i].offset as usize, i * 1024 * 1024);
        }

        let report = allocator.storage_report();
        assert_eq!(report.total_free_space, 0);
        assert_eq!(report.largest_free_region, 0);

        // Free four random slots
        allocator.free(allocations[243]);
        allocator.free(allocations[5]);
        allocator.free(allocations[123]);
        allocator.free(allocations[95]);

        // Free four contiguous slot (allocator must merge)
        allocator.free(allocations[151]);
        allocator.free(allocations[152]);
        allocator.free(allocations[153]);
        allocator.free(allocations[154]);

        allocations[243] = allocator.allocate(1024 * 1024);
        allocations[5] = allocator.allocate(1024 * 1024);
        allocations[123] = allocator.allocate(1024 * 1024);
        allocations[95] = allocator.allocate(1024 * 1024);
        allocations[151] = allocator.allocate(1024 * 1024 * 4); // 4x larger
        assert_ne!(allocations[243].offset, Allocation::NO_SPACE);
        assert_ne!(allocations[5].offset, Allocation::NO_SPACE);
        assert_ne!(allocations[123].offset, Allocation::NO_SPACE);
        assert_ne!(allocations[95].offset, Allocation::NO_SPACE);
        assert_ne!(allocations[151].offset, Allocation::NO_SPACE);

        for i in 0..256 {
            if i < 152 || i > 154 {
                allocator.free(allocations[i]);
            }
        }

        let report2 = allocator.storage_report();
        assert_eq!(report2.total_free_space, 1024 * 1024 * 256);
        assert_eq!(report2.largest_free_region, 1024 * 1024 * 256);

        // End: Validate that allocator has no fragmentation left. Should be 100% clean.
        let validate_all = allocator.allocate(1024 * 1024 * 256);
        assert_eq!(validate_all.offset, 0);
        allocator.free(validate_all);
    }
}
