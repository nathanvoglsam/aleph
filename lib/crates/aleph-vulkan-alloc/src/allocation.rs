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

use crate::Pool;
use aleph_vulkan_alloc_sys::raw;
use aleph_vulkan_core::erupt::vk1_0::{DeviceMemory, DeviceSize, MemoryPropertyFlags};
use core::ptr;
use std::ops::Deref;

///
/// A rusty wrapper around the raw VmaAllocationCreateFlag constants
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct AllocationCreateFlag(raw::VmaAllocationCreateFlagBits);

impl AllocationCreateFlag {
    ///
    /// VMA_ALLOCATION_CREATE_DEDICATED_MEMORY_BIT
    ///
    pub const DEDICATED_MEMORY_BIT: Self =
        Self(raw::VmaAllocationCreateFlagBits_VMA_ALLOCATION_CREATE_DEDICATED_MEMORY_BIT);

    ///
    /// VMA_ALLOCATION_CREATE_NEVER_ALLOCATE_BIT
    ///
    pub const NEVER_ALLOCATE_BIT: Self =
        Self(raw::VmaAllocationCreateFlagBits_VMA_ALLOCATION_CREATE_NEVER_ALLOCATE_BIT);

    ///
    /// VMA_ALLOCATION_CREATE_MAPPED_BIT
    ///
    pub const MAPPED_BIT: Self =
        Self(raw::VmaAllocationCreateFlagBits_VMA_ALLOCATION_CREATE_MAPPED_BIT);

    ///
    /// VMA_ALLOCATION_CREATE_CAN_BECOME_LOST_BIT
    ///
    pub const CAN_BECOME_LOST_BIT: Self =
        Self(raw::VmaAllocationCreateFlagBits_VMA_ALLOCATION_CREATE_CAN_BECOME_LOST_BIT);

    ///
    /// VMA_ALLOCATION_CREATE_CAN_MAKE_OTHER_LOST_BIT
    ///
    pub const CAN_MAKE_OTHER_LOST_BIT: Self =
        Self(raw::VmaAllocationCreateFlagBits_VMA_ALLOCATION_CREATE_CAN_MAKE_OTHER_LOST_BIT);

    ///
    /// VMA_ALLOCATION_CREATE_USER_DATA_COPY_STRING_BIT
    ///
    pub const USER_DATA_COPY_STRING_BIT: Self =
        Self(raw::VmaAllocationCreateFlagBits_VMA_ALLOCATION_CREATE_USER_DATA_COPY_STRING_BIT);

    ///
    /// VMA_ALLOCATION_CREATE_UPPER_ADDRESS_BIT
    ///
    pub const UPPER_ADDRESS_BIT: Self =
        Self(raw::VmaAllocationCreateFlagBits_VMA_ALLOCATION_CREATE_UPPER_ADDRESS_BIT);

    ///
    /// VMA_ALLOCATION_CREATE_STRATEGY_BEST_FIT_BIT
    ///
    pub const STRATEGY_BEST_FIT_BIT: Self =
        Self(raw::VmaAllocationCreateFlagBits_VMA_ALLOCATION_CREATE_STRATEGY_BEST_FIT_BIT);

    ///
    /// VMA_ALLOCATION_CREATE_STRATEGY_WORST_FIT_BIT
    ///
    pub const STRATEGY_WORST_FIT_BIT: Self =
        Self(raw::VmaAllocationCreateFlagBits_VMA_ALLOCATION_CREATE_STRATEGY_WORST_FIT_BIT);

    ///
    /// VMA_ALLOCATION_CREATE_STRATEGY_FIRST_FIT_BIT
    ///
    pub const STRATEGY_FIRST_FIT_BIT: Self =
        Self(raw::VmaAllocationCreateFlagBits_VMA_ALLOCATION_CREATE_STRATEGY_FIRST_FIT_BIT);

    ///
    /// VMA_ALLOCATION_CREATE_STRATEGY_MIN_MEMORY_BIT
    ///
    pub const STRATEGY_MIN_MEMORY_BIT: Self =
        Self(raw::VmaAllocationCreateFlagBits_VMA_ALLOCATION_CREATE_STRATEGY_MIN_MEMORY_BIT);

    ///
    /// VMA_ALLOCATION_CREATE_STRATEGY_MIN_TIME_BIT
    ///
    pub const STRATEGY_MIN_TIME_BIT: Self =
        Self(raw::VmaAllocationCreateFlagBits_VMA_ALLOCATION_CREATE_STRATEGY_MIN_TIME_BIT);

    ///
    /// VMA_ALLOCATION_CREATE_STRATEGY_MIN_FRAGMENTATION_BIT
    ///
    pub const STRATEGY_MIN_FRAGMENTATION_BIT: Self =
        Self(raw::VmaAllocationCreateFlagBits_VMA_ALLOCATION_CREATE_STRATEGY_MIN_FRAGMENTATION_BIT);

    ///
    /// VMA_ALLOCATION_CREATE_STRATEGY_MASK
    ///
    pub const STRATEGY_MASK: Self =
        Self(raw::VmaAllocationCreateFlagBits_VMA_ALLOCATION_CREATE_STRATEGY_MASK);
}

impl From<raw::VmaAllocationCreateFlagBits> for AllocationCreateFlag {
    fn from(input: raw::VmaAllocationCreateFlagBits) -> Self {
        AllocationCreateFlag(input)
    }
}

impl Into<raw::VmaAllocationCreateFlagBits> for AllocationCreateFlag {
    fn into(self) -> raw::VmaAllocationCreateFlagBits {
        self.0
    }
}

///
/// VmaMemoryUsage
///
#[derive(Debug, Copy, Clone)]
pub enum MemoryUsage {
    Unknown,
    GPUOnly,
    CPUOnly,
    CPUToGPU,
    GPUToCPU,
}

impl MemoryUsage {
    ///
    /// Convert a raw VmaMemoryUsage enum into our nice rust wrapper
    ///
    pub fn from_raw(memory_usage: raw::VmaMemoryUsage) -> Self {
        match memory_usage {
            raw::VmaMemoryUsage_VMA_MEMORY_USAGE_UNKNOWN => MemoryUsage::Unknown,
            raw::VmaMemoryUsage_VMA_MEMORY_USAGE_GPU_ONLY => MemoryUsage::GPUOnly,
            raw::VmaMemoryUsage_VMA_MEMORY_USAGE_CPU_ONLY => MemoryUsage::CPUOnly,
            raw::VmaMemoryUsage_VMA_MEMORY_USAGE_CPU_TO_GPU => MemoryUsage::CPUToGPU,
            raw::VmaMemoryUsage_VMA_MEMORY_USAGE_GPU_TO_CPU => MemoryUsage::GPUToCPU,
            _ => unreachable!("Invalid VmaMemoryUsage variant"),
        }
    }

    ///
    /// Convert our enum back into the raw VmaMemoryUsage value
    ///
    pub fn into_raw(self) -> raw::VmaMemoryUsage {
        match self {
            MemoryUsage::Unknown => raw::VmaMemoryUsage_VMA_MEMORY_USAGE_UNKNOWN,
            MemoryUsage::GPUOnly => raw::VmaMemoryUsage_VMA_MEMORY_USAGE_GPU_ONLY,
            MemoryUsage::CPUOnly => raw::VmaMemoryUsage_VMA_MEMORY_USAGE_CPU_ONLY,
            MemoryUsage::CPUToGPU => raw::VmaMemoryUsage_VMA_MEMORY_USAGE_CPU_TO_GPU,
            MemoryUsage::GPUToCPU => raw::VmaMemoryUsage_VMA_MEMORY_USAGE_GPU_TO_CPU,
        }
    }
}

///
/// Wrapper around a VmaAllocation
///
#[derive(Copy, Clone)]
pub struct Allocation {
    allocation: raw::VmaAllocation,
}

impl Allocation {
    ///
    /// Create from a raw handle.
    ///
    /// Must have a lifetime shorter than it's parent allocator
    ///
    pub fn from_raw(allocation: raw::VmaAllocation) -> Self {
        Allocation { allocation }
    }

    ///
    /// Gets a "null" allocation
    ///
    pub fn null() -> Self {
        Self::default()
    }

    ///
    /// Returns the underlying raw::VmaAllocation for use with raw function calls
    ///
    pub fn into_raw(self) -> raw::VmaAllocation {
        self.allocation
    }

    ///
    /// Returns if this value can not be holding a valid allocation
    ///
    pub fn is_invalid(&self) -> bool {
        self.allocation.is_null()
    }
}

//
// Implementing these is safe because this is simply a handle and doesn't own any data
//
unsafe impl Send for Allocation {}
unsafe impl Sync for Allocation {}

impl Default for Allocation {
    fn default() -> Self {
        Self {
            allocation: ptr::null_mut(),
        }
    }
}

///
/// VmaAllocationCreateInfo
///
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AllocationCreateInfo {
    flags: AllocationCreateFlag,
    usage: MemoryUsage,
    required_flags: MemoryPropertyFlags,
    preferred_flags: MemoryPropertyFlags,
    memory_type_bits: u32,
    pool: raw::VmaPool,
    p_user_data: *mut ::std::os::raw::c_void,
}

impl AllocationCreateInfo {
    pub fn builder() -> AllocationCreateInfoBuilder {
        AllocationCreateInfoBuilder::new()
    }

    pub(crate) fn into_raw(self) -> raw::VmaAllocationCreateInfo {
        raw::VmaAllocationCreateInfo {
            flags: self.flags.0 as u32,
            usage: self.usage.into_raw(),
            requiredFlags: self.required_flags.bits(),
            preferredFlags: self.preferred_flags.bits(),
            memoryTypeBits: self.memory_type_bits,
            pool: self.pool,
            pUserData: self.p_user_data,
        }
    }
}

///
///
///
pub struct AllocationCreateInfoBuilder {
    info: AllocationCreateInfo,
}

impl AllocationCreateInfoBuilder {
    ///
    ///
    ///
    pub fn new() -> Self {
        let info = AllocationCreateInfo {
            flags: AllocationCreateFlag::from(0),
            usage: MemoryUsage::Unknown,
            required_flags: MemoryPropertyFlags::empty(),
            preferred_flags: MemoryPropertyFlags::empty(),
            memory_type_bits: 0,
            pool: ptr::null_mut(),
            p_user_data: ptr::null_mut(),
        };
        AllocationCreateInfoBuilder { info }
    }

    ///
    ///
    ///
    pub fn flags(mut self, flags: AllocationCreateFlag) -> Self {
        self.info.flags = flags;
        self
    }

    ///
    ///
    ///
    pub fn required_flags(mut self, flags: MemoryPropertyFlags) -> Self {
        self.info.required_flags = flags;
        self
    }

    ///
    ///
    ///
    pub fn preferred_flags(mut self, flags: MemoryPropertyFlags) -> Self {
        self.info.preferred_flags = flags;
        self
    }

    ///
    ///
    ///
    pub fn usage(mut self, usage: MemoryUsage) -> Self {
        self.info.usage = usage;
        self
    }

    ///
    ///
    ///
    pub fn pool(mut self, pool: &Pool) -> Self {
        self.info.pool = pool.get_raw();
        self
    }

    ///
    ///
    ///
    pub fn build(self) -> AllocationCreateInfo {
        self.info
    }
}

impl Default for AllocationCreateInfoBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for AllocationCreateInfoBuilder {
    type Target = AllocationCreateInfo;

    fn deref(&self) -> &Self::Target {
        &self.info
    }
}

///
/// VmaAllocationInfo
///
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AllocationInfo {
    pub memory_type: u32,
    pub device_memory: DeviceMemory,
    pub offset: DeviceSize,
    pub size: DeviceSize,
    pub p_mapped_data: *mut ::std::os::raw::c_void,
    pub p_user_data: *mut ::std::os::raw::c_void,
}
