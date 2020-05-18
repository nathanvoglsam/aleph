//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use crate::gpu::alloc::Allocator;
use core::ptr;
use erupt::utils::VulkanResult;
use erupt::vk1_0::DeviceSize;
use std::sync::Arc;
use vma_sys::raw;

///
/// A rusty wrapper around the raw VmaPoolCreateFlag constants
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct PoolCreateFlag(u32);

impl PoolCreateFlag {
    ///
    /// VMA_POOL_CREATE_IGNORE_BUFFER_IMAGE_GRANULARITY_BIT
    ///
    pub const IGNORE_BUFFER_IMAGE_GRANULARITY_BIT: PoolCreateFlag = PoolCreateFlag(
        raw::VmaPoolCreateFlagBits_VMA_POOL_CREATE_IGNORE_BUFFER_IMAGE_GRANULARITY_BIT as u32,
    );

    ///
    /// VMA_POOL_CREATE_LINEAR_ALGORITHM_BIT
    ///
    pub const LINEAR_ALGORITHM_BIT: PoolCreateFlag =
        PoolCreateFlag(raw::VmaPoolCreateFlagBits_VMA_POOL_CREATE_LINEAR_ALGORITHM_BIT as u32);

    ///
    /// VMA_POOL_CREATE_BUDDY_ALGORITHM_BIT
    ///
    pub const BUDDY_ALGORITHM_BIT: PoolCreateFlag =
        PoolCreateFlag(raw::VmaPoolCreateFlagBits_VMA_POOL_CREATE_BUDDY_ALGORITHM_BIT as u32);

    ///
    /// VMA_POOL_CREATE_ALGORITHM_MASK
    ///
    pub const ALGORITHM_MASK: PoolCreateFlag =
        PoolCreateFlag(raw::VmaPoolCreateFlagBits_VMA_POOL_CREATE_ALGORITHM_MASK as u32);
}

impl From<u32> for PoolCreateFlag {
    fn from(input: u32) -> Self {
        PoolCreateFlag(input)
    }
}

impl Into<u32> for PoolCreateFlag {
    fn into(self) -> u32 {
        self.0
    }
}

///
/// Builder for a VmaPool, wraps VmaPoolCreateInfo, vmaCreatePool and vmaDestroyPool (drop)
///
pub struct PoolBuilder {
    create_info: PoolCreateInfo,
}

impl PoolBuilder {
    ///
    ///
    ///
    pub fn new() -> Self {
        PoolBuilder {
            create_info: PoolCreateInfo {
                memory_type_index: 0,
                flags: PoolCreateFlag::from(0u32),
                block_size: 0,
                min_block_count: 0,
                max_block_count: 0,
                frame_in_use_count: 0,
            },
        }
    }

    ///
    ///
    ///
    pub fn memory_type_index(mut self, index: u32) -> Self {
        self.create_info.memory_type_index = index;
        self
    }

    ///
    ///
    ///
    pub fn flags(mut self, flags: PoolCreateFlag) -> Self {
        self.create_info.flags = flags;
        self
    }

    ///
    ///
    ///
    pub fn block_size(mut self, size: DeviceSize) -> Self {
        self.create_info.block_size = size;
        self
    }

    ///
    ///
    ///
    pub fn min_block_count(mut self, count: usize) -> Self {
        self.create_info.min_block_count = count;
        self
    }

    ///
    ///
    ///
    pub fn max_block_count(mut self, count: usize) -> Self {
        self.create_info.max_block_count = count;
        self
    }

    ///
    ///
    ///
    pub fn frame_in_use_count(mut self, count: u32) -> Self {
        self.create_info.frame_in_use_count = count;
        self
    }

    ///
    /// vmaCreatePool
    ///
    pub unsafe fn build(self, allocator: &Arc<Allocator>) -> VulkanResult<Arc<Pool>> {
        let mut pool: raw::VmaPool = ptr::null_mut();

        let create_ptr = &self.create_info as *const PoolCreateInfo;
        let create_ptr = create_ptr as *const raw::VmaPoolCreateInfo;

        let result = raw::vmaCreatePool(
            allocator.into_raw(),
            create_ptr,
            &mut pool as *mut raw::VmaPool,
        );

        if result as i32 == 0 {
            let pool = Pool {
                pool,
                allocator: allocator.clone(),
            };
            VulkanResult::new_ok(Arc::new(pool))
        } else {
            VulkanResult::new_err(erupt::vk1_0::Result(result as i32))
        }
    }
}

///
/// VmaPoolCreateInfo
///
#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct PoolCreateInfo {
    memory_type_index: u32,
    flags: PoolCreateFlag,
    block_size: DeviceSize,
    min_block_count: usize,
    max_block_count: usize,
    frame_in_use_count: u32,
}

///
/// VmaPool
///
pub struct Pool {
    pool: raw::VmaPool,
    allocator: Arc<Allocator>,
}

impl Pool {
    ///
    /// Returns the underlying raw::VmaPool for use with raw function calls
    ///
    pub fn get_raw(&self) -> raw::VmaPool {
        self.pool
    }

    ///
    /// vmaGetPoolStats
    ///
    pub unsafe fn get_pool_stats(&self) -> raw::VmaPoolStats {
        let mut stats = raw::VmaPoolStats {
            size: 0,
            unusedSize: 0,
            allocationCount: 0,
            unusedRangeCount: 0,
            unusedRangeSizeMax: 0,
            blockCount: 0,
        };

        raw::vmaGetPoolStats(
            self.allocator.into_raw(),
            self.pool,
            &mut stats as *mut raw::VmaPoolStats,
        );

        stats
    }

    ///
    /// vmaMakePoolAllocationsLost
    ///
    pub unsafe fn make_pool_allocations_lost(&self) -> usize {
        let mut out = 0usize;

        raw::vmaMakePoolAllocationsLost(
            self.allocator.into_raw(),
            self.pool,
            &mut out as *mut usize,
        );

        out
    }

    pub unsafe fn check_pool_corruption(&self) -> VulkanResult<()> {
        let result = raw::vmaCheckPoolCorruption(self.allocator.into_raw(), self.pool);

        if result as i32 == 0 {
            VulkanResult::new_ok(())
        } else {
            VulkanResult::new_err(erupt::vk1_0::Result(result as i32))
        }
    }
}

impl Drop for Pool {
    fn drop(&mut self) {
        unsafe {
            raw::vmaDestroyPool(self.allocator.into_raw(), self.pool);
        }
    }
}
