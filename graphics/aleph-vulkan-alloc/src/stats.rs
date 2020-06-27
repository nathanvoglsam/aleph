//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use aleph_vulkan_core::erupt::vk1_0::DeviceSize;

///
/// VmaStatInfo
///
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct StatInfo {
    pub block_count: u32,
    pub allocation_count: u32,
    pub unused_range_count: u32,
    pub used_bytes: DeviceSize,
    pub unused_bytes: DeviceSize,
    pub allocation_size_min: DeviceSize,
    pub allocation_size_avg: DeviceSize,
    pub allocation_size_max: DeviceSize,
    pub unused_range_size_min: DeviceSize,
    pub unused_range_size_avg: DeviceSize,
    pub unused_range_size_max: DeviceSize,
}

impl Default for StatInfo {
    fn default() -> Self {
        StatInfo {
            block_count: 0,
            allocation_count: 0,
            unused_range_count: 0,
            used_bytes: 0,
            unused_bytes: 0,
            allocation_size_min: 0,
            allocation_size_avg: 0,
            allocation_size_max: 0,
            unused_range_size_min: 0,
            unused_range_size_avg: 0,
            unused_range_size_max: 0,
        }
    }
}

///
/// VmaStats
///
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Stats {
    pub memory_type: [StatInfo; 32usize],
    pub memory_heap: [StatInfo; 16usize],
    pub total: StatInfo,
}

impl Default for Stats {
    fn default() -> Self {
        Stats {
            memory_type: [StatInfo::default(); 32],
            memory_heap: [StatInfo::default(); 16],
            total: StatInfo::default(),
        }
    }
}

///
/// VmaPoolStats
///
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PoolStats {
    pub size: DeviceSize,
    pub unused_size: DeviceSize,
    pub allocation_count: usize,
    pub unused_range_count: usize,
    pub unused_range_size_max: DeviceSize,
    pub block_count: usize,
}

impl Default for PoolStats {
    fn default() -> Self {
        PoolStats {
            size: 0,
            unused_size: 0,
            allocation_count: 0,
            unused_range_count: 0,
            unused_range_size_max: 0,
            block_count: 0,
        }
    }
}
