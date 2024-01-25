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

use std::num::{NonZeroU64, NonZeroUsize};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use parking_lot::Mutex;
use windows::core::CanInto;
use windows::utils::{CPUDescriptorHandle, GPUDescriptorHandle};
use windows::Win32::Graphics::Direct3D12::*;

pub struct DescriptorHeap {
    /// The device the heap belongs to
    device: ID3D12Device,

    /// The descriptor heap we're allocating into
    heap: ID3D12DescriptorHeap,

    /// The descriptor type of the descriptor heap
    r#type: D3D12_DESCRIPTOR_HEAP_TYPE,

    /// The increment size for the descriptor type this heap stores
    descriptor_increment: u32,

    /// The descriptor handle for the start of the heap on the CPU
    start_cpu_handle: D3D12_CPU_DESCRIPTOR_HANDLE,

    /// The descriptor handle for the start of the heap on the GPU
    start_gpu_handle: D3D12_GPU_DESCRIPTOR_HANDLE,

    /// The mutable state wrapped in a mutex for safe multi-threaded access
    state: Mutex<DescriptorHeapState>,
}

impl DescriptorHeap {
    /// Creates a new [DescriptorHeap] based on the provided settings
    pub fn new(
        device: &impl CanInto<ID3D12Device>,
        r#type: D3D12_DESCRIPTOR_HEAP_TYPE,
        num_descriptors: u32,
        gpu_visible: bool,
    ) -> windows::core::Result<Self> {
        let device = device.can_into();

        if num_descriptors == 0 {
            // A descriptor heap with 0 size makes no sense
            panic!("Creating a DescriptorHeap with num_descriptors = 0 is an error");
        }

        // Round 'num_descriptors' up to the next multiple of '32' so that it neatly maps and fits
        // with our bitset in 'flags'
        let num_descriptors = match num_descriptors % 32 {
            0 => num_descriptors,
            r => num_descriptors + (32 - r),
        };

        if num_descriptors > 1_073_741_824 {
            // This prevents numerical error in the allocate code which converts a u32 to an i32.
            // This value 'num_descriptors' sets the upper bound of what will undergo the u32->i32
            // conversion. By keeping the upper bound below i32::MAX then the conversion will
            // always be safe.
            //
            // No D3D12 implementation supports this many descriptors in a heap anyway
            panic!("Creating a DescriptorHeap with num_descriptors > 2^30 is an error");
        }

        // Create the actual descriptor heap based on the requested settings
        let heap = unsafe {
            let mut flags = D3D12_DESCRIPTOR_HEAP_FLAGS::default();

            if gpu_visible {
                flags |= D3D12_DESCRIPTOR_HEAP_FLAG_SHADER_VISIBLE
            }

            let desc = D3D12_DESCRIPTOR_HEAP_DESC {
                Type: r#type,
                NumDescriptors: num_descriptors,
                Flags: flags,
                NodeMask: 0,
            };
            device.CreateDescriptorHeap::<ID3D12DescriptorHeap>(&desc)?
        };

        // Query (to cache) the descriptor increment size
        let descriptor_size = unsafe { device.GetDescriptorHandleIncrementSize(r#type) };

        // Query (to cache) the handle for the start of the heap on the CPU
        let start_cpu_handle = unsafe { heap.GetCPUDescriptorHandleForHeapStart() };

        // Query (to cache) the handle for the start of the heap on the GPU
        let start_gpu_handle = unsafe {
            if gpu_visible {
                heap.GetGPUDescriptorHandleForHeapStart()
            } else {
                D3D12_GPU_DESCRIPTOR_HANDLE::default()
            }
        };

        // Allocate our bit flag buffer
        let flags = {
            let flags: Vec<u32> = vec![0; (num_descriptors / 32) as usize];
            flags.into_boxed_slice()
        };

        let out = Self {
            device: device.clone(),
            heap,
            r#type,
            descriptor_increment: descriptor_size,
            start_cpu_handle,
            start_gpu_handle,
            state: Mutex::new(DescriptorHeapState {
                capacity: num_descriptors,
                len: 0,
                flags,
            }),
        };
        Ok(out)
    }

    /// Release all descriptors in the heap, resetting it to the empty state
    #[allow(unused)]
    pub fn reset(&self) {
        self.state.lock().reset();
    }

    /// Allocates a contiguous range of 'num_descriptors'
    #[allow(unused)]
    pub fn allocate(&self, num_descriptors: u32) -> Option<DescriptorID> {
        if num_descriptors == 0 {
            return None;
        }

        self.state.lock().allocate(num_descriptors)
    }

    /// Release 'num_descriptors' descriptors, starting from 'id'
    #[allow(unused)]
    pub fn release(&self, id: DescriptorID, num_descriptors: u32) {
        self.state.lock().release(id, num_descriptors)
    }

    /// Converts the given 'id' into the [CPUDescriptorHandle] it represents
    #[allow(unused)]
    pub fn id_to_cpu_handle(&self, id: DescriptorID) -> Option<CPUDescriptorHandle> {
        let size = self.descriptor_increment as usize;
        let id = id.0 as usize;
        let base = self.start_cpu_handle.ptr;
        let ptr = base + (id * size);
        NonZeroUsize::new(ptr).map(CPUDescriptorHandle::from)
    }

    /// Converts the given 'id' into the [GPUDescriptorHandle] it represents
    #[allow(unused)]
    pub fn id_to_gpu_handle(&self, id: DescriptorID) -> Option<GPUDescriptorHandle> {
        let size = self.descriptor_increment as u64;
        let id = id.0 as u64;
        let base = self.start_gpu_handle.ptr;
        let ptr = base + (id * size);
        NonZeroU64::new(ptr).map(GPUDescriptorHandle::from)
    }

    /// Returns the descriptor increment of the descriptor type this heap allocates
    #[allow(unused)]
    pub const fn descriptor_increment(&self) -> u32 {
        self.descriptor_increment
    }

    /// Returns the [ID3D12DescriptorHeap] that this object encapsulates
    #[allow(unused)]
    pub const fn heap(&self) -> &ID3D12DescriptorHeap {
        &self.heap
    }

    /// Returns the [D3D12_DESCRIPTOR_HEAP_TYPE] that this heap stores descriptors for
    #[allow(unused)]
    pub const fn heap_type(&self) -> D3D12_DESCRIPTOR_HEAP_TYPE {
        self.r#type
    }

    /// Returns the [ID3D12Device] that this object wraps the heap for
    #[allow(unused)]
    pub const fn device(&self) -> &ID3D12Device {
        &self.device
    }
}

/// The internal state of the [DescriptorHeap]. This is wrapped in a mutex by [DescriptorHeap] for
/// parallel access.
struct DescriptorHeapState {
    /// The number of descriptors that this heap has capacity for.
    capacity: u32,

    /// The number of descriptors that are currently allocated into the heap
    len: u32,

    /// A set of bit flags that marks whether a descriptor in the heap is used
    flags: Box<[u32]>,
}

impl DescriptorHeapState {
    /// Release all descriptors in the heap, resetting it to the empty state
    fn reset(&mut self) {
        // Zero out the 'flags' array, which marks all descriptors as free.
        self.flags.iter_mut().for_each(|v| *v = 0);
        self.len = 0;
    }

    /// Allocates a contiguous range of 'num_descriptors'
    fn allocate(&mut self, num_descriptors: u32) -> Option<DescriptorID> {
        // Early exit if the caller asked for 0 descriptors
        if num_descriptors == 0 {
            return None;
        }

        let mut result = DescriptorID::NULL;
        let mut first_result = DescriptorID::NULL;
        let mut found_count = 0u32;
        let num_flag_slots = self.capacity / 32;

        for i in 0..num_flag_slots {
            // Look up the slot in the bit set
            let flag = self.flags[i as usize];

            // If the slot is full then we try the next slot.
            //
            // This will either skip the slot when we're searching for a non-full slot or cancel
            // a currently in progress segment if we've continued one from the previous slot.
            if flag == u32::MAX {
                // This will transparently do nothing when 'first_result' is null or 'found_count'
                // is 0. This will handle both when we haven't started consuming a segment of
                // descriptors and when we're part way through consuming a segment.
                //
                // When no segment is in progress this does nothing and just skips the slot
                //
                // When a segment is in progress this will cancel the segment and free the slots we
                // speculatively consumed.
                self.release(first_result, found_count);
                found_count = 0;
                result = DescriptorID::NULL;
                first_result = DescriptorID::NULL;
                continue;
            }

            // Iteration over the 32 bits in the current slot
            //
            // This loop drives either consuming a region of descriptors, or looking for the start
            // of a region of free descriptors
            for j in 0..32 {
                let mask = 0b1 << j;

                // If we've found an unused descriptor
                if (flag & mask) == 0 {
                    // Mark the descriptor as used (sets bit j in flags[i])
                    self.flags[i as usize] |= mask;

                    // Update result with the ID that points to the current slot in the 'flags'
                    // bit set.
                    result = DescriptorID::from_slot_and_bit_offset(i, j);

                    if result.is_null() {
                        panic!("Out of descriptors");
                    }

                    // If first_result is null then we're at the start of a new attempt to find
                    // 'count' contiguous descriptors. As such 'result' is our 'first_result'
                    if first_result.is_null() {
                        first_result = result;
                    }

                    // We've found another descriptor to use so we increment the count
                    found_count += 1;

                    // We also increment the 'len' field. While we may not have found all the
                    // descriptors we need yet, we increment this anyway. When we fail to find a
                    // contiguous segment we use 'release' to free the segment we just speculatively
                    // allocated, which will undo the increments we did.
                    self.len += 1;

                    // If we've found enough descriptors we can exit. The allocator is left in a
                    // correct state during the allocation process
                    if found_count == num_descriptors {
                        return Some(first_result);
                    }
                } else if found_count > 0 {
                    // This does the same thing as above in the flag == u32::MAX check. This will
                    // cancel the segment and release any speculatively consumed descriptors.
                    //
                    // This block is reached when we reach an already allocated descriptor while
                    // still consuming a section of descriptors. This means we haven't found a
                    // contiguous run of 'count' descriptors so must cancel the segment and
                    // continue until we find an available segment
                    self.release(first_result, found_count);
                    found_count = 0;
                    result = DescriptorID::NULL;
                    first_result = DescriptorID::NULL;
                }
            }
        }

        // If we reach this point and 'result' is a null ID it means we've searched the entire
        // heap and not found a free segment of descriptors of 'count' length. This is an OOM
        // condition.
        if result.is_null() {
            panic!("Out of descriptors");
        }

        Some(first_result)
    }

    /// Release 'num_descriptors' descriptors, starting from 'id'
    fn release(&mut self, id: DescriptorID, num_descriptors: u32) {
        if id.is_null() || num_descriptors == 0 {
            return;
        }

        let start = id.0 as u32;
        let end = start + num_descriptors;
        for i in start..end {
            let flag_i = i / 32;
            let mask = !(1u32 << (i % 32));
            self.flags[flag_i as usize] &= mask;
        }

        self.len -= num_descriptors;
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
#[repr(transparent)]
pub struct DescriptorID(pub i32);

impl DescriptorID {
    /// Represents a 'null' [DescriptorID]
    #[allow(unused)]
    pub const NULL: Self = Self(-1);

    /// Constructs a [DescriptorID] based on a slot in the 'flags' array and a bit offset inside the
    /// flag slot.
    ///
    /// This is primarily used inside the [DescriptorHeap] utility, which uses a bitset to encoded
    /// whether a descriptor is used or not. `slot` is the index into the bitset's [u32] array, and
    /// `bit` is which bit index in `slot` is being referred to.
    const fn from_slot_and_bit_offset(slot: u32, bit: i32) -> Self {
        let base_id = (slot * 32) as i32;
        Self(base_id + bit)
    }

    /// Checks if self encodes a null ID
    #[allow(unused)]
    pub const fn is_null(&self) -> bool {
        self.0 < 0
    }

    /// Checks if self encodes a non-null ID
    #[allow(unused)]
    pub const fn is_valid(&self) -> bool {
        !self.is_null()
    }
}

impl Add<i32> for DescriptorID {
    type Output = Self;

    fn add(self, rhs: i32) -> Self::Output {
        Self(self.0.add(rhs))
    }
}

impl AddAssign<i32> for DescriptorID {
    fn add_assign(&mut self, rhs: i32) {
        self.0.add_assign(rhs)
    }
}

impl Sub<i32> for DescriptorID {
    type Output = Self;

    fn sub(self, rhs: i32) -> Self::Output {
        Self(self.0.sub(rhs))
    }
}

impl SubAssign<i32> for DescriptorID {
    fn sub_assign(&mut self, rhs: i32) {
        self.0.sub_assign(rhs)
    }
}

impl Mul<i32> for DescriptorID {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self(self.0.mul(rhs))
    }
}

impl MulAssign<i32> for DescriptorID {
    fn mul_assign(&mut self, rhs: i32) {
        self.0.mul_assign(rhs)
    }
}

impl Div<i32> for DescriptorID {
    type Output = Self;

    fn div(self, rhs: i32) -> Self::Output {
        Self(self.0.sub(rhs))
    }
}

impl DivAssign<i32> for DescriptorID {
    fn div_assign(&mut self, rhs: i32) {
        self.0.div_assign(rhs)
    }
}
