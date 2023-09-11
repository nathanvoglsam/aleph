/*
 *
 * This file is a part of NovaEngine
 * https://gitlab.com/MindSpunk/NovaEngine
 *
 *
 * MIT License
 *
 * Copyright (c) 2020 Nathan Voglsam
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused)]
#![allow(clippy::all)]

use crate::vma::AllocationCreateInfoBuilder;
use ash::vk;
use std::ffi::{c_char, c_float, c_void};
use std::ptr::NonNull;

bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy, Default)]
    pub struct AllocatorCreateFlags: u32 {
        /// VMA_ALLOCATOR_CREATE_EXTERNALLY_SYNCHRONIZED_BIT
        const EXTERNALLY_SYNCHRONIZED_BIT = 0x1;

        /// VMA_ALLOCATOR_CREATE_KHR_DEDICATED_ALLOCATION_BIT
        const KHR_DEDICATED_ALLOCATION_BIT = 0x2;

        /// VMA_ALLOCATOR_CREATE_KHR_BIND_MEMORY2_BIT
        const KHR_BIND_MEMORY2_BIT = 0x4;

        /// VMA_ALLOCATOR_CREATE_EXT_MEMORY_BUDGET_BIT
        const EXT_MEMORY_BUDGET_BIT = 0x8;

        /// VMA_ALLOCATOR_CREATE_AMD_DEVICE_COHERENT_MEMORY_BIT
        const AMD_DEVICE_COHERENT_MEMORY_BIT = 0x10;

        /// VMA_ALLOCATOR_CREATE_BUFFER_DEVICE_ADDRESS_BIT
        const BUFFER_DEVICE_ADDRESS_BIT = 0x20;

        /// VMA_ALLOCATOR_CREATE_EXT_MEMORY_PRIORITY_BIT
        const EXT_MEMORY_PRIORITY_BIT = 0x40;
    }
}

#[repr(i32)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
pub enum MemoryUsage {
    Unknown,
    GpuOnly,
    CpuOnly,
    CpuToGpu,
    GpuToCpu,
    CpuCopy,
    GpuLazilyAllocation,
    Auto,
    AutoPreferDevice,
    AutoPreferHost,
}

impl Default for MemoryUsage {
    #[inline(always)]
    fn default() -> Self {
        Self::Unknown
    }
}

bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy, Default)]
    pub struct AllocationCreateFlags: u32 {
        /// VMA_ALLOCATION_CREATE_DEDICATED_MEMORY_BIT
        const DEDICATED_MEMORY_BIT = 0x1;

        /// VMA_ALLOCATION_CREATE_NEVER_ALLOCATE_BIT
        const NEVER_ALLOCATE_BIT = 0x2;

        /// VMA_ALLOCATION_CREATE_MAPPED_BIT
        const MAPPED_BIT = 0x4;

        // /// VMA_ALLOCATION_CREATE_CAN_BECOME_LOST_BIT
        // #[deprecated]
        // const CAN_BECOME_LOST_BIT = 0x8;
        //
        // /// VMA_ALLOCATION_CREATE_CAN_MAKE_OTHER_LOST_BIT
        // #[deprecated]
        // const CAN_MAKE_OTHER_LOST_BIT = 0x10;

        /// VMA_ALLOCATION_CREATE_USER_DATA_COPY_STRING_BIT
        #[deprecated]
        const USER_DATA_COPY_STRING_BIT = 0x20;

        /// VMA_ALLOCATION_CREATE_UPPER_ADDRESS_BIT
        const UPPER_ADDRESS_BIT = 0x40;

        /// VMA_ALLOCATION_CREATE_DONT_BIND_BIT
        const DONT_BIND_BIT = 0x80;

        /// VMA_ALLOCATION_CREATE_WITHIN_BUDGET_BIT
        const WITHIN_BUDGET_BIT = 0x100;

        /// VMA_ALLOCATION_CREATE_CAN_ALIAS_BIT
        const CAN_ALIAS_BIT = 0x200;

        /// VMA_ALLOCATION_CREATE_HOST_ACCESS_SEQUENTIAL_WRITE_BIT
        const HOST_ACCESS_SEQUENTIAL_WRITE_BIT = 0x400;

        /// VMA_ALLOCATION_CREATE_HOST_ACCESS_RANDOM_BIT
        const HOST_ACCESS_RANDOM_BIT = 0x800;

        /// VMA_ALLOCATION_CREATE_HOST_ACCESS_ALLOW_TRANSFER_INSTEAD_BIT
        const HOST_ACCESS_ALLOW_TRANSFER_INSTEAD_BIT = 0x1000;

        /// VMA_ALLOCATION_CREATE_STRATEGY_MIN_MEMORY_BIT
        const STRATEGY_MIN_MEMORY_BIT = 0x10000;

        /// VMA_ALLOCATION_CREATE_STRATEGY_MIN_TIME_BIT
        const STRATEGY_MIN_TIME_BIT = 0x20000;

        /// VMA_ALLOCATION_CREATE_STRATEGY_MIN_OFFSET_BIT
        const STRATEGY_MIN_OFFSET_BIT = 0x40000;

        /// VMA_ALLOCATION_CREATE_STRATEGY_BEST_FIT_BIT
        const STRATEGY_BEST_FIT_BIT = Self::STRATEGY_MIN_MEMORY_BIT.bits();

        /// VMA_ALLOCATION_CREATE_STRATEGY_FIRST_FIT_BIT
        const STRATEGY_FIRST_FIT_BIT = Self::STRATEGY_MIN_TIME_BIT.bits();

        /// VMA_ALLOCATION_CREATE_STRATEGY_MASK
        const STRATEGY_MASK = Self::STRATEGY_MIN_MEMORY_BIT.bits()
            | Self::STRATEGY_MIN_TIME_BIT.bits()
            | Self::STRATEGY_MIN_OFFSET_BIT.bits();
    }
}

bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy, Default)]
    pub struct PoolCreateFlags: u32 {
        /// VMA_POOL_CREATE_IGNORE_BUFFER_IMAGE_GRANULARITY_BIT
        const IGNORE_BUFFER_IMAGE_GRANULARITY_BIT = 0x2;

        /// VMA_POOL_CREATE_LINEAR_ALGORITHM_BIT
        const LINEAR_ALGORITHM_BIT = 0x4;

        /// VMA_POOL_CREATE_ALGORITHM_MASK
        const ALGORITHM_MASK = Self::LINEAR_ALGORITHM_BIT.bits();
    }
}

bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy, Default)]
    pub struct DefragmentationFlags: u32 {
        /// VMA_DEFRAGMENTATION_FLAG_ALGORITHM_FAST_BIT
        const ALGORITHM_FAST_BIT = 0x1;

        /// VMA_DEFRAGMENTATION_FLAG_ALGORITHM_BALANCE
        const ALGORITHM_BALANCED_BIT = 0x2;

        /// VMA_DEFRAGMENTATION_FLAG_ALGORITHM_FULL_BIT
        const ALGORITHM_FULL_BIT = 0x4;

        /// VMA_DEFRAGMENTATION_FLAG_ALGORITHM_EXTENSIVE_BIT
        const ALGORITHM_EXTENSIVE_BIT = 0x8;

        /// VMA_DEFRAGMENTATION_FLAG_ALGORITHM_MASK
        const ALGORITHM_MASK = Self::ALGORITHM_FAST_BIT.bits()
            | Self::ALGORITHM_BALANCED_BIT.bits()
            | Self::ALGORITHM_FULL_BIT.bits()
            | Self::ALGORITHM_EXTENSIVE_BIT.bits();
    }
}

#[repr(i32)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
pub enum DefragmentationMoveOperation {
    Copy,
    Ignore,
    Destroy,
}

impl Default for DefragmentationMoveOperation {
    #[inline(always)]
    fn default() -> Self {
        Self::Copy
    }
}

bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy, Default)]
    pub struct VirtualBlockCreateFlags: u32 {
        /// VMA_VIRTUAL_BLOCK_CREATE_LINEAR_ALGORITHM_BIT
        const LINEAR_ALGORITHM_BIT = 0x1;

        /// VMA_VIRTUAL_BLOCK_CREATE_ALGORITHM_MASK
        const ALGORITHM_MASK = Self::LINEAR_ALGORITHM_BIT.bits();
    }
}

bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy, Default)]
    pub struct VirtualAllocationCreateFlags: u32 {
        /// VMA_VIRTUAL_ALLOCATION_CREATE_UPPER_ADDRESS_BIT
        const UPPER_ADDRESS_BIT = AllocationCreateFlags::UPPER_ADDRESS_BIT.bits();

        /// VMA_VIRTUAL_ALLOCATION_CREATE_STRATEGY_MIN_MEMORY_BIT
        const STRATEGY_MIN_MEMORY_BIT = AllocationCreateFlags::STRATEGY_MIN_MEMORY_BIT.bits();

        /// VMA_VIRTUAL_ALLOCATION_CREATE_STRATEGY_MIN_TIME_BIT
        const STRATEGY_MIN_TIME_BIT = AllocationCreateFlags::STRATEGY_MIN_TIME_BIT.bits();

        /// VMA_VIRTUAL_ALLOCATION_CREATE_STRATEGY_MIN_OFFSET_BIT
        const STRATEGY_MIN_OFFSET_BIT = AllocationCreateFlags::STRATEGY_MIN_OFFSET_BIT.bits();

        /// VMA_VIRTUAL_ALLOCATION_CREATE_STRATEGY_MASK
        const STRATEGY_MASK = Self::STRATEGY_MIN_MEMORY_BIT.bits()
            | Self::STRATEGY_MIN_TIME_BIT.bits()
            | Self::STRATEGY_MIN_OFFSET_BIT.bits();
    }
}

/// Opaque type for Allocator pointers to point to. Just a type-safety utility
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AllocatorT([u8; 0]);
pub type AllocatorH = NonNull<AllocatorT>;

/// Opaque type for Pool pointers to point to. Just a type-safety utility
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PoolT([u8; 0]);
pub type PoolH = NonNull<PoolT>;

/// Opaque type for Allocation pointers to point to. Just a type-safety utility
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AllocationT([u8; 0]);
pub type AllocationH = NonNull<AllocationT>;

/// Opaque type for DefragmentationContext pointers to point to. Just a type-safety utility
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DefragmentationContextT([u8; 0]);
pub type DefragmentationContextH = NonNull<DefragmentationContextT>;

/// Opaque type for VirtualAllocation pointers to point to. Just a type-safety utility
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VirtualAllocationT([u8; 0]);
pub type VirtualAllocationH = NonNull<VirtualAllocationT>;

/// Opaque type for VirtualBlock pointers to point to. Just a type-safety utility
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VirtualBlockT([u8; 0]);
pub type VirtualBlockH = NonNull<VirtualBlockT>;

pub type PFN_vmaAllocateDeviceMemoryFunction = unsafe extern "C" fn(
    allocator: AllocatorH,
    memory_type: u32,
    memory: vk::DeviceMemory,
    size: vk::DeviceSize,
    p_user_data: Option<NonNull<c_void>>,
);
pub type PFN_vmaFreeDeviceMemoryFunction = unsafe extern "C" fn(
    allocator: AllocatorH,
    memory_type: u32,
    memory: vk::DeviceMemory,
    size: vk::DeviceSize,
    p_user_data: Option<NonNull<c_void>>,
);

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct DeviceMemoryCallbacks {
    pub pfn_allocate: Option<PFN_vmaAllocateDeviceMemoryFunction>,
    pub pfn_free: Option<PFN_vmaFreeDeviceMemoryFunction>,
    pub p_user_data: Option<NonNull<c_void>>,
}

#[repr(C)]
#[derive(Copy, Clone)]
#[allow(non_snake_case)]
pub struct VulkanFunctions {
    pub vkGetInstanceProcAddr: Option<vk::PFN_vkGetInstanceProcAddr>,
    pub vkGetDeviceProcAddr: Option<vk::PFN_vkGetDeviceProcAddr>,
    pub vkGetPhysicalDeviceProperties: Option<vk::PFN_vkGetPhysicalDeviceProperties>,
    pub vkGetPhysicalDeviceMemoryProperties: Option<vk::PFN_vkGetPhysicalDeviceMemoryProperties>,
    pub vkAllocateMemory: Option<vk::PFN_vkAllocateMemory>,
    pub vkFreeMemory: Option<vk::PFN_vkFreeMemory>,
    pub vkMapMemory: Option<vk::PFN_vkMapMemory>,
    pub vkUnmapMemory: Option<vk::PFN_vkUnmapMemory>,
    pub vkFlushMappedMemoryRanges: Option<vk::PFN_vkFlushMappedMemoryRanges>,
    pub vkInvalidateMappedMemoryRanges: Option<vk::PFN_vkInvalidateMappedMemoryRanges>,
    pub vkBindBufferMemory: Option<vk::PFN_vkBindBufferMemory>,
    pub vkBindImageMemory: Option<vk::PFN_vkBindImageMemory>,
    pub vkGetBufferMemoryRequirements: Option<vk::PFN_vkGetBufferMemoryRequirements>,
    pub vkGetImageMemoryRequirements: Option<vk::PFN_vkGetImageMemoryRequirements>,
    pub vkCreateBuffer: Option<vk::PFN_vkCreateBuffer>,
    pub vkDestroyBuffer: Option<vk::PFN_vkDestroyBuffer>,
    pub vkCreateImage: Option<vk::PFN_vkCreateImage>,
    pub vkDestroyImage: Option<vk::PFN_vkDestroyImage>,
    pub vkCmdCopyBuffer: Option<vk::PFN_vkCmdCopyBuffer>,
    pub vkGetBufferMemoryRequirements2KHR: Option<vk::PFN_vkGetBufferMemoryRequirements2>,
    pub vkGetImageMemoryRequirements2KHR: Option<vk::PFN_vkGetImageMemoryRequirements2>,
    pub vkBindBufferMemory2KHR: Option<vk::PFN_vkBindBufferMemory2>,
    pub vkBindImageMemory2KHR: Option<vk::PFN_vkBindImageMemory2>,
    pub vkGetPhysicalDeviceMemoryProperties2KHR:
        Option<vk::PFN_vkGetPhysicalDeviceMemoryProperties2>,
    pub vkGetDeviceBufferMemoryRequirements: Option<vk::PFN_vkGetDeviceBufferMemoryRequirements>,
    pub vkGetDeviceImageMemoryRequirements: Option<vk::PFN_vkGetDeviceImageMemoryRequirements>,
}

#[repr(C)]
#[derive(Copy, Clone)]
#[allow(non_snake_case)]
pub struct AllocatorInfo {
    pub instance: vk::Instance,
    pub physicalDevice: vk::PhysicalDevice,
    pub device: vk::Device,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AllocatorCreateInfo {
    pub flags: AllocatorCreateFlags,
    pub physical_device: vk::PhysicalDevice,
    pub device: vk::Device,
    pub preferred_large_heap_block_size: vk::DeviceSize,
    pub p_allocation_callbacks: *const vk::AllocationCallbacks,
    pub p_device_memory_callbacks: *const DeviceMemoryCallbacks,
    pub p_heap_size_limit: *const vk::DeviceSize,
    pub p_vulkan_functions: *const VulkanFunctions,
    pub instance: vk::Instance,
    pub vulkan_api_version: u32,
    pub p_type_external_memory_handle_types: *const vk::ExternalMemoryHandleTypeFlagsKHR,
}

#[repr(C)]
#[derive(Clone, PartialEq, Eq, Hash, Default, Debug)]
pub struct Statistics {
    pub block_count: u32,
    pub allocation_count: u32,
    pub block_bytes: vk::DeviceSize,
    pub allocation_bytes: vk::DeviceSize,
}

#[repr(C)]
#[derive(Clone, PartialEq, Eq, Hash, Default, Debug)]
pub struct DetailedStatistics {
    pub statistics: Statistics,
    pub unused_range_count: u32,
    pub allocation_size_min: vk::DeviceSize,
    pub allocation_size_max: vk::DeviceSize,
    pub unused_range_size_min: vk::DeviceSize,
    pub unused_range_size_max: vk::DeviceSize,
}

#[repr(C)]
#[derive(Clone, PartialEq, Eq, Hash, Default, Debug)]
pub struct TotalStatistics {
    pub memory_type: [DetailedStatistics; vk::MAX_MEMORY_TYPES],
    pub memory_heap: [DetailedStatistics; vk::MAX_MEMORY_HEAPS],
    pub total: DetailedStatistics,
}

#[repr(C)]
#[derive(Clone, PartialEq, Eq, Hash, Default, Debug)]
pub struct Budget {
    pub statistics: Statistics,
    pub usage: vk::DeviceSize,
    pub budget: vk::DeviceSize,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct AllocationCreateInfo {
    pub flags: AllocationCreateFlags,
    pub usage: MemoryUsage,
    pub required_flags: vk::MemoryPropertyFlags,
    pub preferred_flags: vk::MemoryPropertyFlags,
    pub memory_type_bits: u32,
    pub pool: Option<PoolH>,
    pub p_user_data: Option<NonNull<c_void>>,
    pub priority: c_float,
}

impl AllocationCreateInfo {
    pub const fn builder<'a>() -> AllocationCreateInfoBuilder<'a> {
        AllocationCreateInfoBuilder::new()
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct PoolCreateInfo {
    pub memory_type_index: u32,
    pub flags: PoolCreateFlags,
    pub block_size: vk::DeviceSize,
    pub min_block_count: usize,
    pub max_block_count: usize,
    pub priority: c_float,
    pub min_allocation_alignment: vk::DeviceSize,
    pub p_memory_allocate_next: Option<NonNull<c_void>>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct AllocationInfo {
    pub memory_type: u32,
    pub device_memory: vk::DeviceMemory,
    pub offset: vk::DeviceSize,
    pub size: vk::DeviceSize,
    pub p_mapped_data: Option<NonNull<c_void>>,
    pub p_user_data: Option<NonNull<c_void>>,
    pub p_name: Option<NonNull<c_char>>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct DefragmentationInfo {
    pub flags: DefragmentationFlags,
    pub pool: Option<PoolH>,
    pub max_bytes_per_pass: vk::DeviceSize,
    pub max_allocations_per_pass: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DefragmentationMove {
    pub operation: DefragmentationMoveOperation,
    pub src_allocation: AllocationH,
    pub dst_tmp_allocation: AllocationH,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DefragmentationPassMoveInfo {
    pub move_count: u32,
    pub p_moves: *const DefragmentationMove,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct DefragmentationStats {
    pub bytes_moved: vk::DeviceSize,
    pub bytes_freed: vk::DeviceSize,
    pub allocations_moved: u32,
    pub device_memory_blocks_freed: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VirtualBlockCreateInfo {
    pub size: vk::DeviceSize,
    pub flags: VirtualBlockCreateFlags,
    pub p_allocation_callbacks: Option<NonNull<vk::AllocationCallbacks>>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VirtualAllocationCreateInfo {
    pub size: vk::DeviceSize,
    pub alignment: vk::DeviceSize,
    pub flags: VirtualAllocationCreateFlags,
    pub p_user_data: Option<NonNull<c_void>>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VirtualAllocationInfo {
    pub offset: vk::DeviceSize,
    pub size: vk::DeviceSize,
    pub p_user_data: Option<NonNull<c_void>>,
}

#[link(name = "vma", kind = "static")]
#[allow(non_snake_case)]
extern "C" {
    pub fn vmaCreateAllocator(
        p_create_info: NonNull<AllocatorCreateInfo>,
        p_allocator: NonNull<Option<AllocatorH>>,
    ) -> vk::Result;
    pub fn vmaDestroyAllocator(allocator: Option<AllocatorH>);
    pub fn vmaGetAllocatorInfo(allocator: AllocatorH, pAllocatorInfo: NonNull<AllocatorInfo>);
    pub fn vmaGetPhysicalDeviceProperties(
        allocator: AllocatorH,
        ppPhysicalDeviceProperties: NonNull<Option<NonNull<vk::PhysicalDeviceProperties>>>,
    );
    pub fn vmaGetMemoryProperties(
        allocator: AllocatorH,
        ppPhysicalDeviceMemoryProperties: NonNull<
            Option<NonNull<vk::PhysicalDeviceMemoryProperties>>,
        >,
    );
    pub fn vmaGetMemoryTypeProperties(
        allocator: AllocatorH,
        memoryTypeIndex: u32,
        pFlags: NonNull<vk::MemoryPropertyFlags>,
    );
    pub fn vmaSetCurrentFrameIndex(allocator: AllocatorH, frameIndex: u32);
    pub fn vmaCalculateStatistics(allocator: AllocatorH, pStats: NonNull<TotalStatistics>);
    pub fn vmaGetHeapBudgets(allocator: AllocatorH, pBudgets: Option<NonNull<Budget>>);
    pub fn vmaFindMemoryTypeIndex(
        allocator: AllocatorH,
        memoryTypeBits: u32,
        pAllocationCreateInfo: NonNull<AllocationCreateInfo>,
        pMemoryTypeIndex: NonNull<u32>,
    ) -> vk::Result;
    pub fn vmaFindMemoryTypeIndexForBufferInfo(
        allocator: AllocatorH,
        pBufferCreateInfo: NonNull<vk::BufferCreateInfo>,
        pAllocationCreateInfo: NonNull<AllocationCreateInfo>,
        pMemoryTypeIndex: NonNull<u32>,
    ) -> vk::Result;
    pub fn vmaFindMemoryTypeIndexForImageInfo(
        allocator: AllocatorH,
        pImageCreateInfo: NonNull<vk::ImageCreateInfo>,
        pAllocationCreateInfo: NonNull<AllocationCreateInfo>,
        pMemoryTypeIndex: NonNull<u32>,
    ) -> vk::Result;
    pub fn vmaCreatePool(
        allocator: AllocatorH,
        pCreateInfo: NonNull<PoolCreateInfo>,
        pPool: NonNull<Option<PoolH>>,
    ) -> vk::Result;
    pub fn vmaDestroyPool(allocator: AllocatorH, pool: Option<PoolH>);
    pub fn vmaGetPoolStatistics(
        allocator: AllocatorH,
        pool: PoolH,
        pPoolStats: NonNull<Statistics>,
    );
    pub fn vmaCalculatePoolStatistics(
        allocator: AllocatorH,
        pool: PoolH,
        pPoolStats: NonNull<DetailedStatistics>,
    );
    pub fn vmaCheckPoolCorruption(allocator: AllocatorH, pool: PoolH) -> vk::Result;
    pub fn vmaGetPoolName(
        allocator: AllocatorH,
        pool: PoolH,
        ppName: NonNull<Option<NonNull<c_char>>>,
    );
    pub fn vmaSetPoolName(allocator: AllocatorH, pool: PoolH, pName: Option<NonNull<c_char>>);
    pub fn vmaAllocateMemory(
        allocator: AllocatorH,
        pVkMemoryRequirements: NonNull<vk::MemoryRequirements>,
        pCreateInfo: NonNull<AllocationCreateInfo>,
        pAllocation: NonNull<Option<AllocationH>>,
        pAllocationInfo: Option<NonNull<AllocationInfo>>,
    ) -> vk::Result;
    pub fn vmaAllocateMemoryPages(
        allocator: AllocatorH,
        pVkMemoryRequirements: Option<NonNull<vk::MemoryRequirements>>,
        pCreateInfo: Option<NonNull<AllocationCreateInfo>>,
        allocationCount: usize,
        pAllocations: NonNull<Option<AllocationH>>,
        pAllocationInfo: Option<NonNull<AllocationInfo>>,
    ) -> vk::Result;
    pub fn vmaAllocateMemoryForBuffer(
        allocator: AllocatorH,
        buffer: vk::Buffer,
        pCreateInfo: NonNull<AllocationCreateInfo>,
        pAllocation: NonNull<Option<AllocationH>>,
        pAllocationInfo: Option<NonNull<AllocationInfo>>,
    ) -> vk::Result;
    pub fn vmaAllocateMemoryForImage(
        allocator: AllocatorH,
        image: vk::Image,
        pCreateInfo: NonNull<AllocationCreateInfo>,
        pAllocation: NonNull<Option<AllocationH>>,
        pAllocationInfo: Option<NonNull<AllocationInfo>>,
    ) -> vk::Result;
    pub fn vmaFreeMemory(allocator: AllocatorH, allocation: Option<AllocationH>);
    pub fn vmaFreeMemoryPages(
        allocator: AllocatorH,
        allocationCount: usize,
        pAllocations: Option<NonNull<AllocationH>>,
    );
    pub fn vmaGetAllocationInfo(
        allocator: AllocatorH,
        allocation: AllocationH,
        pAllocationInfo: NonNull<AllocationInfo>,
    );
    pub fn vmaSetAllocationUserData(
        allocator: AllocatorH,
        allocation: AllocationH,
        pUserData: Option<NonNull<c_void>>,
    );
    pub fn vmaSetAllocationName(
        allocator: AllocatorH,
        allocation: AllocationH,
        pName: Option<NonNull<c_char>>,
    );
    pub fn vmaGetAllocationMemoryProperties(
        allocator: AllocatorH,
        allocation: AllocationH,
        pFlags: NonNull<vk::MemoryPropertyFlags>,
    );
    pub fn vmaMapMemory(
        allocator: AllocatorH,
        allocation: AllocationH,
        ppData: NonNull<Option<NonNull<c_void>>>,
    ) -> vk::Result;
    pub fn vmaUnmapMemory(allocator: AllocatorH, allocation: AllocationH);
    pub fn vmaFlushAllocation(
        allocator: AllocatorH,
        allocation: AllocationH,
        offset: vk::DeviceSize,
        size: vk::DeviceSize,
    ) -> vk::Result;
    pub fn vmaInvalidateAllocation(
        allocator: AllocatorH,
        allocation: AllocationH,
        offset: vk::DeviceSize,
        size: vk::DeviceSize,
    ) -> vk::Result;
    pub fn vmaFlushAllocations(
        allocator: AllocatorH,
        allocationCount: u32,
        allocations: Option<NonNull<AllocationH>>,
        offsets: Option<NonNull<vk::DeviceSize>>,
        sizes: Option<NonNull<vk::DeviceSize>>,
    ) -> vk::Result;
    pub fn vmaInvalidateAllocations(
        allocator: AllocatorH,
        allocationCount: u32,
        allocations: Option<NonNull<AllocationH>>,
        offsets: Option<NonNull<vk::DeviceSize>>,
        sizes: Option<NonNull<vk::DeviceSize>>,
    ) -> vk::Result;
    pub fn vmaCheckCorruption(allocator: AllocatorH, memoryTypeBits: u32) -> vk::Result;
    pub fn vmaBeginDefragmentation(
        allocator: AllocatorH,
        pInfo: NonNull<DefragmentationInfo>,
        pContext: NonNull<Option<DefragmentationContextH>>,
    ) -> vk::Result;
    pub fn vmaEndDefragmentation(
        allocator: AllocatorH,
        context: DefragmentationContextH,
        pStats: Option<NonNull<DefragmentationStats>>,
    );
    pub fn vmaBeginDefragmentationPass(
        allocator: AllocatorH,
        context: DefragmentationContextH,
        pPassInfo: NonNull<DefragmentationPassMoveInfo>,
    ) -> vk::Result;
    pub fn vmaEndDefragmentationPass(
        allocator: AllocatorH,
        context: DefragmentationContextH,
        pPassInfo: NonNull<DefragmentationPassMoveInfo>,
    ) -> vk::Result;
    pub fn vmaBindBufferMemory(
        allocator: AllocatorH,
        allocation: AllocationH,
        buffer: vk::Buffer,
    ) -> vk::Result;
    pub fn vmaBindBufferMemory2(
        allocator: AllocatorH,
        allocation: AllocationH,
        allocationLocalOffset: vk::DeviceSize,
        buffer: vk::Buffer,
        pNext: Option<NonNull<c_void>>,
    ) -> vk::Result;
    pub fn vmaBindImageMemory(
        allocator: AllocatorH,
        allocation: AllocationH,
        image: vk::Image,
    ) -> vk::Result;
    pub fn vmaBindImageMemory2(
        allocator: AllocatorH,
        allocation: AllocationH,
        allocationLocalOffset: vk::DeviceSize,
        image: vk::Image,
        pNext: Option<NonNull<c_void>>,
    ) -> vk::Result;
    pub fn vmaCreateBuffer(
        allocator: AllocatorH,
        pBufferCreateInfo: NonNull<vk::BufferCreateInfo>,
        pAllocationCreateInfo: NonNull<AllocationCreateInfo>,
        pBuffer: NonNull<vk::Buffer>,
        pAllocation: NonNull<Option<AllocationH>>,
        pAllocationInfo: Option<NonNull<AllocationInfo>>,
    ) -> vk::Result;
    pub fn vmaCreateBufferWithAlignment(
        allocator: AllocatorH,
        pBufferCreateInfo: NonNull<vk::BufferCreateInfo>,
        pAllocationCreateInfo: NonNull<AllocationCreateInfo>,
        minAlignment: vk::DeviceSize,
        pBuffer: NonNull<vk::Buffer>,
        pAllocation: NonNull<Option<AllocationH>>,
        pAllocationInfo: Option<NonNull<AllocationInfo>>,
    ) -> vk::Result;
    pub fn vmaCreateAliasingBuffer(
        allocator: AllocatorH,
        allocation: AllocationH,
        pBufferCreateInfo: NonNull<vk::BufferCreateInfo>,
        pBuffer: NonNull<vk::Buffer>,
    ) -> vk::Result;
    pub fn vmaDestroyBuffer(
        allocator: AllocatorH,
        buffer: vk::Buffer,
        allocation: Option<AllocationH>,
    );
    pub fn vmaCreateImage(
        allocator: AllocatorH,
        pImageCreateInfo: NonNull<vk::ImageCreateInfo>,
        pAllocationCreateInfo: NonNull<AllocationCreateInfo>,
        pImage: NonNull<vk::Image>,
        pAllocation: NonNull<Option<AllocationH>>,
        pAllocationInfo: Option<NonNull<AllocationInfo>>,
    ) -> vk::Result;
    pub fn vmaCreateAliasingImage(
        allocator: AllocatorH,
        allocation: AllocationH,
        pImageCreateInfo: NonNull<vk::ImageCreateInfo>,
        pImage: NonNull<vk::Image>,
    ) -> vk::Result;
    pub fn vmaDestroyImage(
        allocator: AllocatorH,
        image: vk::Image,
        allocation: Option<AllocationH>,
    );
    pub fn vmaCreateVirtualBlock(
        pCreateInfo: NonNull<VirtualBlockCreateInfo>,
        pVirtualBlock: NonNull<Option<PoolH>>,
    ) -> vk::Result;
    pub fn vmaDestroyVirtualBlock(virtualBlock: Option<VirtualBlockH>);
    pub fn vmaIsVirtualBlockEmpty(virtualBlock: VirtualBlockH) -> vk::Bool32;
    pub fn vmaGetVirtualAllocationInfo(
        virtualBlock: VirtualBlockH,
        allocation: VirtualAllocationH,
        pVirtualAllocInfo: NonNull<VirtualAllocationInfo>,
    );
    pub fn vmaVirtualAllocate(
        virtualBlock: VirtualBlockH,
        pCreateInfo: NonNull<VirtualAllocationCreateInfo>,
        pAllocation: NonNull<Option<VirtualAllocationH>>,
        pOffset: NonNull<vk::DeviceSize>,
    ) -> vk::Result;
    pub fn vmaVirtualFree(virtualBlock: VirtualBlockH, allocation: VirtualAllocationH);
    pub fn vmaClearVirtualBlock(virtualBlock: VirtualBlockH);
    pub fn vmaSetVirtualAllocationUserData(
        virtualBlock: VirtualBlockH,
        allocation: VirtualAllocationH,
        pUserData: Option<NonNull<c_void>>,
    );
    pub fn vmaGetVirtualBlockStatistics(virtualBlock: VirtualBlockH, pStats: NonNull<Statistics>);
    pub fn vmaCalculateVirtualBlockStatistics(
        virtualBlock: VirtualBlockH,
        pStats: NonNull<DetailedStatistics>,
    );

    // TODO: When time permits we need to update to master version to fix compile issue on gcc
    //       around sprintf and a missing cstdio include
    // // #if VMA_STATS_STRING_ENABLED
    //
    // pub fn vmaBuildVirtualBlockStatsString(
    //     virtualBlock: VirtualBlockH,
    //     ppStatsString: NonNull<Option<NonNull<c_void>>>,
    //     detailedMap: vk::Bool32,
    // );
    // pub fn vmaFreeVirtualBlockStatsString(
    //     virtualBlock: VirtualBlockH,
    //     pStatsString: Option<NonNull<c_char>>,
    // );
    // pub fn vmaBuildStatsString(
    //     allocator: AllocatorH,
    //     ppStatsString: NonNull<Option<NonNull<c_char>>>,
    //     detailedMap: vk::Bool32,
    // );
    // pub fn vmaFreeStatsString(allocator: AllocatorH, pStatsString: Option<NonNull<c_char>>);
}
