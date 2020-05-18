//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use crate::gpu::alloc::Pool;
use core::ptr;
use erupt::vk1_0::{DeviceMemory, DeviceSize, MemoryPropertyFlags};
use vma_sys::raw;

///
/// A rusty wrapper around the raw VmaAllocationCreateFlag constants
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct AllocationCreateFlag(u32);

impl AllocationCreateFlag {
    ///
    /// VMA_ALLOCATION_CREATE_DEDICATED_MEMORY_BIT
    ///
    pub const DEDICATED_MEMORY_BIT: AllocationCreateFlag = AllocationCreateFlag(
        raw::VmaAllocationCreateFlagBits_VMA_ALLOCATION_CREATE_DEDICATED_MEMORY_BIT,
    );

    ///
    /// VMA_ALLOCATION_CREATE_NEVER_ALLOCATE_BIT
    ///
    pub const NEVER_ALLOCATE_BIT: AllocationCreateFlag = AllocationCreateFlag(
        raw::VmaAllocationCreateFlagBits_VMA_ALLOCATION_CREATE_NEVER_ALLOCATE_BIT,
    );

    ///
    /// VMA_ALLOCATION_CREATE_MAPPED_BIT
    ///
    pub const MAPPED_BIT: AllocationCreateFlag =
        AllocationCreateFlag(raw::VmaAllocationCreateFlagBits_VMA_ALLOCATION_CREATE_MAPPED_BIT);

    ///
    /// VMA_ALLOCATION_CREATE_CAN_BECOME_LOST_BIT
    ///
    pub const CAN_BECOME_LOST_BIT: AllocationCreateFlag = AllocationCreateFlag(
        raw::VmaAllocationCreateFlagBits_VMA_ALLOCATION_CREATE_CAN_BECOME_LOST_BIT,
    );

    ///
    /// VMA_ALLOCATION_CREATE_CAN_MAKE_OTHER_LOST_BIT
    ///
    pub const CAN_MAKE_OTHER_LOST_BIT: AllocationCreateFlag = AllocationCreateFlag(
        raw::VmaAllocationCreateFlagBits_VMA_ALLOCATION_CREATE_CAN_MAKE_OTHER_LOST_BIT,
    );

    ///
    /// VMA_ALLOCATION_CREATE_USER_DATA_COPY_STRING_BIT
    ///
    pub const USER_DATA_COPY_STRING_BIT: AllocationCreateFlag = AllocationCreateFlag(
        raw::VmaAllocationCreateFlagBits_VMA_ALLOCATION_CREATE_USER_DATA_COPY_STRING_BIT,
    );

    ///
    /// VMA_ALLOCATION_CREATE_UPPER_ADDRESS_BIT
    ///
    pub const UPPER_ADDRESS_BIT: AllocationCreateFlag = AllocationCreateFlag(
        raw::VmaAllocationCreateFlagBits_VMA_ALLOCATION_CREATE_UPPER_ADDRESS_BIT,
    );

    ///
    /// VMA_ALLOCATION_CREATE_STRATEGY_BEST_FIT_BIT
    ///
    pub const STRATEGY_BEST_FIT_BIT: AllocationCreateFlag = AllocationCreateFlag(
        raw::VmaAllocationCreateFlagBits_VMA_ALLOCATION_CREATE_STRATEGY_BEST_FIT_BIT,
    );

    ///
    /// VMA_ALLOCATION_CREATE_STRATEGY_WORST_FIT_BIT
    ///
    pub const STRATEGY_WORST_FIT_BIT: AllocationCreateFlag = AllocationCreateFlag(
        raw::VmaAllocationCreateFlagBits_VMA_ALLOCATION_CREATE_STRATEGY_WORST_FIT_BIT,
    );

    ///
    /// VMA_ALLOCATION_CREATE_STRATEGY_FIRST_FIT_BIT
    ///
    pub const STRATEGY_FIRST_FIT_BIT: AllocationCreateFlag = AllocationCreateFlag(
        raw::VmaAllocationCreateFlagBits_VMA_ALLOCATION_CREATE_STRATEGY_FIRST_FIT_BIT,
    );

    ///
    /// VMA_ALLOCATION_CREATE_STRATEGY_MIN_MEMORY_BIT
    ///
    pub const STRATEGY_MIN_MEMORY_BIT: AllocationCreateFlag = AllocationCreateFlag(
        raw::VmaAllocationCreateFlagBits_VMA_ALLOCATION_CREATE_STRATEGY_MIN_MEMORY_BIT,
    );

    ///
    /// VMA_ALLOCATION_CREATE_STRATEGY_MIN_TIME_BIT
    ///
    pub const STRATEGY_MIN_TIME_BIT: AllocationCreateFlag = AllocationCreateFlag(
        raw::VmaAllocationCreateFlagBits_VMA_ALLOCATION_CREATE_STRATEGY_MIN_TIME_BIT,
    );

    ///
    /// VMA_ALLOCATION_CREATE_STRATEGY_MIN_FRAGMENTATION_BIT
    ///
    pub const STRATEGY_MIN_FRAGMENTATION_BIT: AllocationCreateFlag = AllocationCreateFlag(
        raw::VmaAllocationCreateFlagBits_VMA_ALLOCATION_CREATE_STRATEGY_MIN_FRAGMENTATION_BIT,
    );

    ///
    /// VMA_ALLOCATION_CREATE_STRATEGY_MASK
    ///
    pub const STRATEGY_MASK: AllocationCreateFlag =
        AllocationCreateFlag(raw::VmaAllocationCreateFlagBits_VMA_ALLOCATION_CREATE_STRATEGY_MASK);
}

impl From<u32> for AllocationCreateFlag {
    fn from(input: u32) -> Self {
        AllocationCreateFlag(input)
    }
}

impl Into<u32> for AllocationCreateFlag {
    fn into(self) -> u32 {
        self.0
    }
}

///
/// VmaMemoryUsage
///
#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum MemoryUsage {
    Unknown = raw::VmaMemoryUsage_VMA_MEMORY_USAGE_UNKNOWN,
    GPUOnly = raw::VmaMemoryUsage_VMA_MEMORY_USAGE_GPU_ONLY,
    CPUOnly = raw::VmaMemoryUsage_VMA_MEMORY_USAGE_CPU_ONLY,
    CPUToGPU = raw::VmaMemoryUsage_VMA_MEMORY_USAGE_CPU_TO_GPU,
    GPUToCPU = raw::VmaMemoryUsage_VMA_MEMORY_USAGE_GPU_TO_CPU,
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
            flags: AllocationCreateFlag::from(0u32),
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
