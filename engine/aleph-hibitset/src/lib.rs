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

#[cfg(test)]
mod tests;

/// A heirarchical bitset type that allows contructing a large bitset. This bitset implementation
/// allows accelerating iteration over the indices of the set bits by maintaining a hierarchy of
/// 'overlay' bitsets which summarize the level below them.
///
/// # Info
///
/// All levels except the first are boxed and dynamically sized.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct HiBitSet {
    /// The bottom level of the bitset. This contains the actual bitset itself, all other levels
    /// are metadata that flag whether any bits in their covered region in the next lower level are
    /// set.
    pub(crate) level_0: Vec<u64>,

    /// The next level above `level_0`. Each bit in this set maps to a single [`u64`] in `level_0`.
    pub(crate) level_1: Vec<u64>,

    /// The next level above `level_1`. Each bit in this set maps to a single [`u64`] in `level_1`.
    pub(crate) level_2: Vec<u64>,

    /// The next level above `level_2`. Each bit in this set maps to a single [`u64`] in `level_2`.
    pub(crate) level_3: u64,

    /// The number of bits we guarantee there to be space in the bitset currently. This may be
    /// smaller than the actual number of bits that are allocated.
    pub(crate) len: usize,
}

impl HiBitSet {
    pub const MAX_BITS: usize = 16_777_216;

    pub const fn new() -> Self {
        Self {
            level_0: Vec::new(),
            level_1: Vec::new(),
            level_2: Vec::new(),
            level_3: 0,
            len: 0,
        }
    }

    pub const fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn iter(&self) -> HiBitSetIter<'_> {
        if self.level_3 == 0 {
            HiBitSetIter {
                set: self,
                level_sets: [0, 0, 0, 0],
                state: IterState::Terminal,
            }
        } else {
            HiBitSetIter {
                set: self,
                level_sets: [0, 0, 0, self.level_3 as i64],
                state: IterState::Level3(0),
            }
        }
    }

    /// Returns the number of bytes allocated by this bitset across the three heap allocated levels
    /// of the bitset.
    ///
    /// This is just the sum of all capacities for the 3 Vec's, returned as size in bytes.
    #[inline(always)]
    pub fn bytes_allocated(&self) -> usize {
        let level_0 = self.level_0.capacity();
        let level_1 = self.level_1.capacity();
        let level_2 = self.level_2.capacity();
        (level_0 + level_1 + level_2) * size_of::<u64>()
    }

    /// Clears all bits in the bitset back to zero. The number of bits the set contains is
    /// unchanged, and the size of the allocations remain unchanged
    #[inline]
    pub fn clear(&mut self) {
        self.level_0.iter_mut().for_each(|v| *v = 0);
        self.level_1.iter_mut().for_each(|v| *v = 0);
        self.level_2.iter_mut().for_each(|v| *v = 0);
        self.level_3 = 0;
    }

    /// Resize the bitset to contain `new_size` bits. Will expand or shrink the allocations as
    /// needed. Truncated bits will become unset (implicitly, no work is done). Newly created bits
    /// will be set to 0.
    ///
    /// This _will_ grow or shrink the backing allocations.
    ///
    /// # Warning
    ///
    /// Currently shrinking the set (except to size 0) is unimplemented. It's possible but the
    /// implementation is complex and I don't have a use case currently. It's also relatively
    /// expensive as we invalidate a lot of the acceleration hierarchy and we have to resummarize
    /// a chunk of the tree.
    pub fn resize(&mut self, new_size: usize) {
        assert!(new_size <= Self::MAX_BITS);

        if new_size == 0 {
            // Special case for 0 as it can blindly reset everything to zero size
            self.level_0 = Vec::new();
            self.level_1 = Vec::new();
            self.level_2 = Vec::new();
            self.level_3 = 0;
            self.len = 0;
        } else {
            // Calculate the minimum number of slots we need to allocate in each level
            let new_capacity_0 = new_size.div_ceil(64);
            let new_capacity_1 = new_capacity_0.div_ceil(64);
            let new_capacity_2 = new_capacity_1.div_ceil(64);

            match self.len.cmp(&new_size) {
                std::cmp::Ordering::Less => {
                    // We can simply expand the existing space as logically we don't actually
                    // change the state of any bits. We don't need to update the heirarchy as a
                    // result!
                    self.level_0.resize(new_capacity_0, 0);
                    self.level_1.resize(new_capacity_1, 0);
                    self.level_2.resize(new_capacity_2, 0);

                    self.len = new_size;
                }
                std::cmp::Ordering::Equal => {
                    // Do nothing, already the same size!
                    return;
                }
                std::cmp::Ordering::Greater => {
                    // I honestly don't have a use case for this yet so I'm just going to leave it
                    // unimplemented for now.
                    unimplemented!();
                    // // This will shrink the levels to the smallest possible size to fit the
                    // // requested number of bits
                    // let old_cap_0 = self.level_0.len();
                    // self.level_0.truncate(new_capacity_0);
                    //
                    // let old_cap_1 = self.level_1.len();
                    // self.level_1.truncate(new_capacity_1);
                    //
                    // let old_cap_2 = self.level_2.len();
                    // self.level_2.truncate(new_capacity_2);
                    //
                    // // Reallocate the buffers
                    // self.level_0.shrink_to_fit();
                    // self.level_1.shrink_to_fit();
                    // self.level_2.shrink_to_fit();
                    //
                    // // The number of bits after 'new_size' that have space allocated and that
                    // // we need to zero out
                    // //
                    // // This effectively calculates (new_size.next_multiple_of(64) - new_size) which
                    // // gives us the number of bits after 'len' we have allocated space for
                    // let danging_bits = (new_capacity_0 * 64) - new_size;
                    //
                    // // This makes a mask that when 'and'ed with the block it will clear all the
                    // // dangling bits
                    // let clear_mask = !((!0) << danging_bits);
                    //
                    // // Safety: We already bounds check and guarantee there is a block because we
                    // //         just checked if new_size > 0 to be here.
                    // let block = unsafe {
                    //     self.level_0.last_mut().unwrap_unchecked()
                    // };
                    // *block &= clear_mask; // Apply the mask
                    //
                    //
                    //
                    //
                    // // Now is the fun part. There is a 'dirty' region at the end of the bitset where
                    // // we've shrunk some entires off the end of the set. This modifies the bitset's
                    // // logical contents and as such we have to patch up the hierarchy to correctly
                    // // represent this new state.
                }
            }
        }
    }

    /// Checks if the specific bit in the bitset is set.
    ///
    /// Will return 'false' for out-of-bounds bits.
    ///
    /// # Info
    ///
    /// This skips reading the hierarchy and just jumps directly to the bottom level.
    #[inline]
    pub fn has_bit(&self, i: usize) -> bool {
        if i >= self.len {
            // Early exit with bounds check
            false
        } else {
            let index = Self::index_to_level_0_index(i);
            let shift = Self::index_to_level_0_shift(i);

            // Safety: We bounds check ourselves once, rather than for every array access
            unsafe { Self::check_bit_in_level(&self.level_0, index, shift) }
        }
    }

    /// Marks the given bit as set. This will also update the bit hierarchy.
    #[inline]
    pub fn set_bit(&mut self, i: usize) {
        if i < self.len {
            let index = Self::index_to_level_0_index(i);
            let shift = Self::index_to_level_0_shift(i);

            // Safety: We bounds check ourselves once, rather than for every array access
            unsafe {
                Self::set_bit_in_level(&mut self.level_0, index, shift);
            }

            let index = Self::index_to_level_1_index(i);
            let shift = Self::index_to_level_1_shift(i);

            // Safety: We bounds check ourselves once, rather than for every array access
            unsafe {
                Self::set_bit_in_level(&mut self.level_1, index, shift);
            }

            let index = Self::index_to_level_2_index(i);
            let shift = Self::index_to_level_2_shift(i);

            // Safety: We bounds check ourselves once, rather than for every array access
            unsafe {
                Self::set_bit_in_level(&mut self.level_2, index, shift);
            }

            let index = Self::index_to_level_3_index(i);
            let shift = Self::index_to_level_3_shift(i);

            // Safety: We bounds check ourselves once, rather than for every array access
            unsafe {
                Self::set_bit_in_level(std::slice::from_mut(&mut self.level_3), index, shift);
            }
        }
    }

    /// Marks the given bit as unset. This will also update the bit hierarchy.
    #[inline]
    pub fn unset_bit(&mut self, i: usize) {
        if i < self.len {
            let index = Self::index_to_level_0_index(i);
            let shift = Self::index_to_level_0_shift(i);

            // Safety: We bounds check ourselves once, rather than for every array access
            let v = unsafe { Self::unset_bit_in_level(&mut self.level_0, index, shift) };

            // If there are still bits in the block we exit, there's nothing to do
            if v != 0 {
                return;
            }

            // Otherwise we update the level above
            let index = Self::index_to_level_1_index(i);
            let shift = Self::index_to_level_1_shift(i);

            // Safety: We bounds check ourselves once, rather than for every array access
            let v = unsafe { Self::unset_bit_in_level(&mut self.level_1, index, shift) };

            // If there are still bits in the block we exit, there's nothing to do
            if v != 0 {
                return;
            }

            // Otherwise we update the level above
            let index = Self::index_to_level_2_index(i);
            let shift = Self::index_to_level_2_shift(i);

            // Safety: We bounds check ourselves once, rather than for every array access
            let v = unsafe { Self::unset_bit_in_level(&mut self.level_2, index, shift) };

            // If there are still bits in the block we exit, there's nothing to do
            if v != 0 {
                return;
            }

            // Otherwise we update the level above
            let index = Self::index_to_level_3_index(i);
            let shift = Self::index_to_level_3_shift(i);

            // Safety: We bounds check ourselves once, rather than for every array access
            let _v = unsafe {
                Self::unset_bit_in_level(std::slice::from_mut(&mut self.level_3), index, shift)
            };

            // Now there really is nothing to do
        }
    }

    #[inline]
    unsafe fn check_bit_in_level(level: &[u64], index: usize, shift: usize) -> bool {
        let slot = unsafe { *level.get_unchecked(index) };
        let bit = slot >> shift;
        let masked = bit & 1;

        masked != 0
    }

    #[inline]
    unsafe fn set_bit_in_level(level: &mut [u64], index: usize, shift: usize) {
        unsafe {
            let bit = 1 << shift;
            *level.get_unchecked_mut(index) |= bit;
        }
    }

    #[inline]
    unsafe fn unset_bit_in_level(level: &mut [u64], index: usize, shift: usize) -> u64 {
        unsafe {
            let bit = 1 << shift;
            let v = *level.get_unchecked_mut(index);
            let v = v & !bit;
            *level.get_unchecked_mut(index) = v;

            // We return 'v' so callers can check if all the bits in a block are unset so we can update
            // the acceleration hierarchy
            v
        }
    }

    const fn index_to_level_0_index(i: usize) -> usize {
        i / 64
    }

    const fn index_to_level_0_shift(i: usize) -> usize {
        i & (64 - 1) // remainder (i / 64)
    }

    const fn index_to_level_1_index(i: usize) -> usize {
        Self::index_to_level_0_index(i) / 64
    }

    const fn index_to_level_1_shift(i: usize) -> usize {
        Self::index_to_level_0_index(i) & (64 - 1) // remainder (i / 64)
    }

    const fn index_to_level_2_index(i: usize) -> usize {
        Self::index_to_level_1_index(i) / 64
    }

    const fn index_to_level_2_shift(i: usize) -> usize {
        Self::index_to_level_1_index(i) & (64 - 1) // remainder (i / 64)
    }

    const fn index_to_level_3_index(i: usize) -> usize {
        Self::index_to_level_2_index(i) / 64
    }

    const fn index_to_level_3_shift(i: usize) -> usize {
        Self::index_to_level_2_index(i) & (64 - 1) // remainder (i / 64)
    }
}

/// Iterator that will yield all the bits set in the bitset efficiently.
///
/// This is the core operation and reason to use the [`HiBitSet`] in the first place. Using the
/// bitset hierarchy we can very efficiently skip over unset bits. If you were, for example,
/// tracking a per-object dirty flag in a large array of objects and want to efficiently iterate the
/// dirty objects then this is a great tool for the job.
///
/// Much more memory efficient than a change list,
pub struct HiBitSetIter<'a> {
    set: &'a HiBitSet,
    level_sets: [i64; 4],
    state: IterState,
}

impl<'a> HiBitSetIter<'a> {
    const fn handle_level(level_set: &mut i64) -> usize {
        let bits = *level_set;
        let lsb_mask = bits & bits.wrapping_neg();
        let r = lsb_mask.trailing_zeros();
        let new_bits = bits ^ lsb_mask;
        *level_set = new_bits;
        r as usize
    }
}

impl<'a> Iterator for HiBitSetIter<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.state {
                IterState::Level3(index) => {
                    if self.level_sets[3] == 0 {
                        self.state = IterState::Terminal;
                    } else {
                        let bit = Self::handle_level(&mut self.level_sets[3]);
                        let slot_index = index * 64 + bit;
                        // Safety: Implicitly bounds checked by iterator implementation
                        let slot = unsafe { *self.set.level_2.get_unchecked(slot_index) };
                        self.level_sets[2] = slot as i64;
                        self.state = IterState::Level2(slot_index);
                    }
                }
                IterState::Level2(index) => {
                    if self.level_sets[2] == 0 {
                        let new_index = index / 64;
                        self.state = IterState::Level3(new_index);
                    } else {
                        let bit = Self::handle_level(&mut self.level_sets[2]);
                        let slot_index = index * 64 + bit;
                        // Safety: Implicitly bounds checked by iterator implementation
                        let slot = unsafe { *self.set.level_1.get_unchecked(slot_index) };
                        self.level_sets[1] = slot as i64;
                        self.state = IterState::Level1(slot_index);
                    }
                }
                IterState::Level1(index) => {
                    if self.level_sets[1] == 0 {
                        let new_index = index / 64;
                        self.state = IterState::Level2(new_index);
                    } else {
                        let bit = Self::handle_level(&mut self.level_sets[1]);
                        let slot_index = index * 64 + bit;
                        // Safety: Implicitly bounds checked by iterator implementation
                        let slot = unsafe { *self.set.level_0.get_unchecked(slot_index) };
                        self.level_sets[0] = slot as i64;
                        self.state = IterState::Level0(slot_index);
                    }
                }
                IterState::Level0(index) => {
                    if self.level_sets[0] == 0 {
                        let new_index = index / 64;
                        self.state = IterState::Level1(new_index);
                    } else {
                        let bit = Self::handle_level(&mut self.level_sets[0]);
                        let new_index = bit + index * 64;
                        return Some(new_index);
                    }
                }
                IterState::Terminal => {
                    return None;
                }
            }
        }
    }
}

#[derive(Copy, Clone)]
enum IterState {
    Level3(usize),
    Level2(usize),
    Level1(usize),
    Level0(usize),
    Terminal,
}
