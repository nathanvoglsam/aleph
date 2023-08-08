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

use ash::vk;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VmaAllocator_T([u8; 0]);
pub type VmaAllocator = *mut VmaAllocator_T;

pub type PFN_vmaAllocateDeviceMemoryFunction = unsafe extern "C" fn(
    allocator: VmaAllocator,
    memory_type: u32,
    memory: vk::DeviceMemory,
    size: vk::DeviceSize,
);
pub type PFN_vmaFreeDeviceMemoryFunction = unsafe extern "C" fn(
    allocator: VmaAllocator,
    memory_type: u32,
    memory: vk::DeviceMemory,
    size: vk::DeviceSize,
);

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VmaDeviceMemoryCallbacks {
    pub pfn_allocate: Option<PFN_vmaAllocateDeviceMemoryFunction>,
    pub pfn_free: Option<PFN_vmaFreeDeviceMemoryFunction>,
}

bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
    pub struct AllocatorCreateFlags: u32 {
        /// VMA_ALLOCATOR_CREATE_EXTERNALLY_SYNCHRONIZED_BIT
        const EXTERNALLY_SYNCHRONIZED_BIT = 1;

        /// VMA_ALLOCATOR_CREATE_KHR_DEDICATED_ALLOCATION_BIT
        const KHR_DEDICATED_ALLOCATION_BIT = 2;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
#[allow(non_snake_case)]
pub struct VmaVulkanFunctions {
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
}

bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
    pub struct RecordFlags: u32 {
        /// VMA_RECORD_FLUSH_AFTER_CALL_BIT
        const FLUSH_AFTER_CALL_BIT = 1;
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VmaRecordSettings {
    pub flags: RecordFlags,
    pub p_file_path: *const std::os::raw::c_char,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VmaAllocatorCreateInfo {
    pub flags: AllocatorCreateFlags,
    pub physical_device: vk::PhysicalDevice,
    pub device: vk::Device,
    pub preferred_large_heap_block_size: vk::DeviceSize,
    pub p_allocation_callbacks: *const vk::AllocationCallbacks,
    pub p_device_memory_callbacks: *const VmaDeviceMemoryCallbacks,
    pub frame_in_use_count: u32,
    pub p_heap_size_limit: *const vk::DeviceSize,
    pub p_vulkan_functions: *const VmaVulkanFunctions,
    pub p_record_settings: *const VmaRecordSettings,
    pub vulkan_api_version: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VmaStatInfo {
    pub block_count: u32,
    pub allocation_count: u32,
    pub unused_range_count: u32,
    pub used_bytes: vk::DeviceSize,
    pub unused_bytes: vk::DeviceSize,
    pub allocation_size_min: vk::DeviceSize,
    pub allocation_size_avg: vk::DeviceSize,
    pub allocation_size_max: vk::DeviceSize,
    pub unused_range_size_min: vk::DeviceSize,
    pub unused_range_size_avg: vk::DeviceSize,
    pub unused_range_size_max: vk::DeviceSize,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VmaStats {
    pub memory_type: [VmaStatInfo; 32usize],
    pub memory_heap: [VmaStatInfo; 16usize],
    pub total: VmaStatInfo,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VmaPool_T([u8; 0]);
pub type VmaPool = *mut VmaPool_T;

pub const VMA_MEMORY_USAGE_UNKNOWN: VmaMemoryUsage = 0;
pub const VMA_MEMORY_USAGE_GPU_ONLY: VmaMemoryUsage = 1;
pub const VMA_MEMORY_USAGE_CPU_ONLY: VmaMemoryUsage = 2;
pub const VMA_MEMORY_USAGE_CPU_TO_GPU: VmaMemoryUsage = 3;
pub const VMA_MEMORY_USAGE_GPU_TO_CPU: VmaMemoryUsage = 4;
pub type VmaMemoryUsage = i32;

bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
    pub struct AllocationCreateFlags: u32 {
        /// VMA_ALLOCATION_CREATE_DEDICATED_MEMORY_BIT
        const DEDICATED_MEMORY_BIT = 1;

        /// VMA_ALLOCATION_CREATE_NEVER_ALLOCATE_BIT
        const NEVER_ALLOCATE_BIT = 2;

        /// VMA_ALLOCATION_CREATE_MAPPED_BIT
        const MAPPED_BIT = 4;

        /// VMA_ALLOCATION_CREATE_CAN_BECOME_LOST_BIT
        const CAN_BECOME_LOST_BIT = 8;

        /// VMA_ALLOCATION_CREATE_CAN_MAKE_OTHER_LOST_BIT
        const CAN_MAKE_OTHER_LOST_BIT = 16;

        /// VMA_ALLOCATION_CREATE_USER_DATA_COPY_STRING_BIT
        const USER_DATA_COPY_STRING_BIT = 32;

        /// VMA_ALLOCATION_CREATE_UPPER_ADDRESS_BIT
        const UPPER_ADDRESS_BIT = 64;

        /// VMA_ALLOCATION_CREATE_STRATEGY_BEST_FIT_BIT
        const STRATEGY_BEST_FIT_BIT = 65536;

        /// VMA_ALLOCATION_CREATE_STRATEGY_WORST_FIT_BIT
        const STRATEGY_WORST_FIT_BIT = 131072;

        /// VMA_ALLOCATION_CREATE_STRATEGY_FIRST_FIT_BIT
        const STRATEGY_FIRST_FIT_BIT = 262144;

        /// VMA_ALLOCATION_CREATE_STRATEGY_MIN_MEMORY_BIT
        const STRATEGY_MIN_MEMORY_BIT = 65536;

        /// VMA_ALLOCATION_CREATE_STRATEGY_MIN_TIME_BIT
        const STRATEGY_MIN_TIME_BIT = 262144;

        /// VMA_ALLOCATION_CREATE_STRATEGY_MIN_FRAGMENTATION_BIT
        const STRATEGY_MIN_FRAGMENTATION_BIT = 131072;

        /// VMA_ALLOCATION_CREATE_STRATEGY_MASK
        const STRATEGY_MASK = 458752;
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VmaAllocationCreateInfo {
    pub flags: AllocationCreateFlags,
    pub usage: VmaMemoryUsage,
    pub required_flags: vk::MemoryPropertyFlags,
    pub preferred_flags: vk::MemoryPropertyFlags,
    pub memory_type_bits: u32,
    pub pool: VmaPool,
    pub p_user_data: *mut std::os::raw::c_void,
}

bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
    pub struct PoolCreateFlags: u32 {
        /// VMA_POOL_CREATE_IGNORE_BUFFER_IMAGE_GRANULARITY_BIT
        const IGNORE_BUFFER_IMAGE_GRANULARITY_BIT = 2;

        /// VMA_POOL_CREATE_LINEAR_ALGORITHM_BIT
        const LINEAR_ALGORITHM_BIT = 4;

        /// VMA_POOL_CREATE_BUDDY_ALGORITHM_BIT
        const BUDDY_ALGORITHM_BIT = 8;

        /// VMA_POOL_CREATE_ALGORITHM_MASK
        const ALGORITHM_MASK = 12;
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VmaPoolCreateInfo {
    pub memory_type_index: u32,
    pub flags: PoolCreateFlags,
    pub block_size: vk::DeviceSize,
    pub min_block_count: usize,
    pub max_block_count: usize,
    pub frame_in_use_count: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct VmaPoolStats {
    pub size: vk::DeviceSize,
    pub unused_size: vk::DeviceSize,
    pub allocation_count: usize,
    pub unused_range_count: usize,
    pub unused_range_size_max: vk::DeviceSize,
    pub block_count: usize,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VmaAllocation_T([u8; 0]);
pub type VmaAllocation = *mut VmaAllocation_T;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VmaAllocationInfo {
    pub memory_type: u32,
    pub device_memory: vk::DeviceMemory,
    pub offset: vk::DeviceSize,
    pub size: vk::DeviceSize,
    pub p_mapped_data: *mut std::os::raw::c_void,
    pub p_user_data: *mut std::os::raw::c_void,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VmaDefragmentationContext_T([u8; 0]);
pub type VmaDefragmentationContext = *mut VmaDefragmentationContext_T;

bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
    pub struct DefragmentationFlags: u32 {}
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VmaDefragmentationInfo2 {
    pub flags: DefragmentationFlags,
    pub allocation_count: u32,
    pub p_allocations: *mut VmaAllocation,
    pub p_allocations_changed: *mut vk::Bool32,
    pub pool_count: u32,
    pub p_pools: *mut VmaPool,
    pub max_cpu_bytes_to_move: vk::DeviceSize,
    pub max_cpu_allocations_to_move: u32,
    pub max_gpu_bytes_to_move: vk::DeviceSize,
    pub max_gpu_allocations_to_move: u32,
    pub command_buffer: vk::CommandBuffer,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VmaDefragmentationInfo {
    pub max_bytes_to_move: vk::DeviceSize,
    pub max_allocations_to_move: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VmaDefragmentationStats {
    pub bytes_moved: vk::DeviceSize,
    pub bytes_freed: vk::DeviceSize,
    pub allocations_moved: u32,
    pub device_memory_blocks_freed: u32,
}

#[link(name = "vma", kind = "static")]
extern "C" {
    pub fn vmaCreateAllocator(
        p_create_info: *const VmaAllocatorCreateInfo,
        p_allocator: *mut VmaAllocator,
    ) -> vk::Result;

    pub fn vmaDestroyAllocator(allocator: VmaAllocator);

    pub fn vmaGetPhysicalDeviceProperties(
        allocator: VmaAllocator,
        pp_physical_device_properties: *mut *const vk::PhysicalDeviceProperties,
    );

    pub fn vmaGetMemoryProperties(
        allocator: VmaAllocator,
        pp_physical_device_memory_properties: *mut *const vk::PhysicalDeviceMemoryProperties,
    );

    pub fn vmaGetMemoryTypeProperties(
        allocator: VmaAllocator,
        memory_type_index: u32,
        p_flags: *mut vk::MemoryPropertyFlags,
    );

    pub fn vmaSetCurrentFrameIndex(allocator: VmaAllocator, frame_index: u32);

    pub fn vmaCalculateStats(allocator: VmaAllocator, p_stats: *mut VmaStats);

    pub fn vmaBuildStatsString(
        allocator: VmaAllocator,
        pp_stats_string: *mut *mut std::os::raw::c_char,
        detailed_map: vk::Bool32,
    );

    pub fn vmaFreeStatsString(allocator: VmaAllocator, p_stats_string: *mut std::os::raw::c_char);

    pub fn vmaFindMemoryTypeIndex(
        allocator: VmaAllocator,
        memory_type_bits: u32,
        p_allocation_create_info: *const VmaAllocationCreateInfo,
        p_memory_type_index: *mut u32,
    ) -> vk::Result;

    pub fn vmaFindMemoryTypeIndexForBufferInfo(
        allocator: VmaAllocator,
        p_buffer_create_info: *const vk::BufferCreateInfo,
        p_allocation_create_info: *const VmaAllocationCreateInfo,
        p_memory_type_index: *mut u32,
    ) -> vk::Result;

    pub fn vmaFindMemoryTypeIndexForImageInfo(
        allocator: VmaAllocator,
        p_image_create_info: *const vk::ImageCreateInfo,
        p_allocation_create_info: *const VmaAllocationCreateInfo,
        p_memory_type_index: *mut u32,
    ) -> vk::Result;

    pub fn vmaCreatePool(
        allocator: VmaAllocator,
        p_create_info: *const VmaPoolCreateInfo,
        p_pool: *mut VmaPool,
    ) -> vk::Result;

    pub fn vmaDestroyPool(allocator: VmaAllocator, pool: VmaPool);

    pub fn vmaGetPoolStats(allocator: VmaAllocator, pool: VmaPool, p_pool_stats: *mut VmaPoolStats);

    pub fn vmaMakePoolAllocationsLost(
        allocator: VmaAllocator,
        pool: VmaPool,
        p_lost_allocation_count: *mut usize,
    );

    pub fn vmaCheckPoolCorruption(allocator: VmaAllocator, pool: VmaPool) -> vk::Result;

    pub fn vmaAllocateMemory(
        allocator: VmaAllocator,
        p_vk_memory_requirements: *const vk::MemoryRequirements,
        p_create_info: *const VmaAllocationCreateInfo,
        p_allocation: *mut VmaAllocation,
        p_allocation_info: *mut VmaAllocationInfo,
    ) -> vk::Result;

    pub fn vmaAllocateMemoryPages(
        allocator: VmaAllocator,
        p_vk_memory_requirements: *const vk::MemoryRequirements,
        p_create_info: *const VmaAllocationCreateInfo,
        allocation_count: usize,
        p_allocations: *mut VmaAllocation,
        p_allocation_info: *mut VmaAllocationInfo,
    ) -> vk::Result;

    pub fn vmaAllocateMemoryForBuffer(
        allocator: VmaAllocator,
        buffer: vk::Buffer,
        p_create_info: *const VmaAllocationCreateInfo,
        p_allocation: *mut VmaAllocation,
        p_allocation_info: *mut VmaAllocationInfo,
    ) -> vk::Result;

    pub fn vmaAllocateMemoryForImage(
        allocator: VmaAllocator,
        image: vk::Image,
        p_create_info: *const VmaAllocationCreateInfo,
        p_allocation: *mut VmaAllocation,
        p_allocation_info: *mut VmaAllocationInfo,
    ) -> vk::Result;

    pub fn vmaFreeMemory(allocator: VmaAllocator, allocation: VmaAllocation);

    pub fn vmaFreeMemoryPages(
        allocator: VmaAllocator,
        allocation_count: usize,
        p_allocations: *mut VmaAllocation,
    );

    pub fn vmaResizeAllocation(
        allocator: VmaAllocator,
        allocation: VmaAllocation,
        new_size: vk::DeviceSize,
    ) -> vk::Result;

    pub fn vmaGetAllocationInfo(
        allocator: VmaAllocator,
        allocation: VmaAllocation,
        p_allocation_info: *mut VmaAllocationInfo,
    );

    pub fn vmaTouchAllocation(allocator: VmaAllocator, allocation: VmaAllocation) -> vk::Bool32;

    pub fn vmaSetAllocationUserData(
        allocator: VmaAllocator,
        allocation: VmaAllocation,
        p_user_data: *mut std::os::raw::c_void,
    );

    pub fn vmaCreateLostAllocation(allocator: VmaAllocator, p_allocation: *mut VmaAllocation);

    pub fn vmaMapMemory(
        allocator: VmaAllocator,
        allocation: VmaAllocation,
        pp_data: *mut *mut std::os::raw::c_void,
    ) -> vk::Result;

    pub fn vmaUnmapMemory(allocator: VmaAllocator, allocation: VmaAllocation);

    pub fn vmaFlushAllocation(
        allocator: VmaAllocator,
        allocation: VmaAllocation,
        offset: vk::DeviceSize,
        size: vk::DeviceSize,
    );

    pub fn vmaInvalidateAllocation(
        allocator: VmaAllocator,
        allocation: VmaAllocation,
        offset: vk::DeviceSize,
        size: vk::DeviceSize,
    );

    pub fn vmaCheckCorruption(allocator: VmaAllocator, memory_type_bits: u32) -> vk::Result;

    pub fn vmaDefragmentationBegin(
        allocator: VmaAllocator,
        p_info: *const VmaDefragmentationInfo2,
        p_stats: *mut VmaDefragmentationStats,
        p_context: *mut VmaDefragmentationContext,
    ) -> vk::Result;

    pub fn vmaDefragmentationEnd(
        allocator: VmaAllocator,
        context: VmaDefragmentationContext,
    ) -> vk::Result;

    pub fn vmaDefragment(
        allocator: VmaAllocator,
        p_allocations: *mut VmaAllocation,
        allocation_count: usize,
        p_allocations_changed: *mut vk::Bool32,
        p_defragmentation_info: *const VmaDefragmentationInfo,
        p_defragmentation_stats: *mut VmaDefragmentationStats,
    ) -> vk::Result;

    pub fn vmaBindBufferMemory(
        allocator: VmaAllocator,
        allocation: VmaAllocation,
        buffer: vk::Buffer,
    ) -> vk::Result;

    pub fn vmaBindImageMemory(
        allocator: VmaAllocator,
        allocation: VmaAllocation,
        image: vk::Image,
    ) -> vk::Result;

    pub fn vmaCreateBuffer(
        allocator: VmaAllocator,
        p_buffer_create_info: *const vk::BufferCreateInfo,
        p_allocation_create_info: *const VmaAllocationCreateInfo,
        p_buffer: *mut vk::Buffer,
        p_allocation: *mut VmaAllocation,
        p_allocation_info: *mut VmaAllocationInfo,
    ) -> vk::Result;

    pub fn vmaDestroyBuffer(allocator: VmaAllocator, buffer: vk::Buffer, allocation: VmaAllocation);

    pub fn vmaCreateImage(
        allocator: VmaAllocator,
        p_image_create_info: *const vk::ImageCreateInfo,
        p_allocation_create_info: *const VmaAllocationCreateInfo,
        p_image: *mut vk::Image,
        p_allocation: *mut VmaAllocation,
        p_allocation_info: *mut VmaAllocationInfo,
    ) -> vk::Result;

    pub fn vmaDestroyImage(allocator: VmaAllocator, image: vk::Image, allocation: VmaAllocation);
}
