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

use crate::vma;
use aleph_vulkan_alloc_sys::raw;
use core::ptr;
use erupt::utils::VulkanResult;
use erupt::vk;
use std::sync::Arc;

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

impl From<PoolCreateFlag> for u32 {
    fn from(v: PoolCreateFlag) -> u32 {
        v.0
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
    pub const fn new() -> Self {
        PoolBuilder {
            create_info: PoolCreateInfo {
                memory_type_index: 0,
                flags: PoolCreateFlag(0u32),
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
    pub const fn memory_type_index(mut self, index: u32) -> Self {
        self.create_info.memory_type_index = index;
        self
    }

    ///
    ///
    ///
    pub const fn flags(mut self, flags: PoolCreateFlag) -> Self {
        self.create_info.flags = flags;
        self
    }

    ///
    ///
    ///
    pub const fn block_size(mut self, size: vk::DeviceSize) -> Self {
        self.create_info.block_size = size;
        self
    }

    ///
    ///
    ///
    pub const fn min_block_count(mut self, count: usize) -> Self {
        self.create_info.min_block_count = count;
        self
    }

    ///
    ///
    ///
    pub const fn max_block_count(mut self, count: usize) -> Self {
        self.create_info.max_block_count = count;
        self
    }

    ///
    ///
    ///
    pub const fn frame_in_use_count(mut self, count: u32) -> Self {
        self.create_info.frame_in_use_count = count;
        self
    }

    ///
    /// vmaCreatePool
    ///
    #[inline]
    pub unsafe fn build(self, allocator: &Arc<vma::Allocator>) -> VulkanResult<Arc<Pool>> {
        let mut pool: raw::VmaPool = ptr::null_mut();

        let create_ptr = &self.create_info as *const PoolCreateInfo;
        let create_ptr = create_ptr as *const raw::VmaPoolCreateInfo;

        let result = raw::vmaCreatePool(allocator.as_raw(), create_ptr, &mut pool as *mut _);

        debug_assert!(pool.is_null(), "Pool should not be null");

        if result as i32 == 0 {
            let pool = Pool {
                pool,
                allocator: allocator.clone(),
            };
            VulkanResult::new_ok(Arc::new(pool))
        } else {
            VulkanResult::new_err(vk::Result(result as i32))
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
    block_size: vk::DeviceSize,
    min_block_count: usize,
    max_block_count: usize,
    frame_in_use_count: u32,
}

///
/// VmaPool
///
pub struct Pool {
    pool: raw::VmaPool,
    allocator: Arc<vma::Allocator>,
}

impl Pool {
    ///
    /// Returns the underlying raw::VmaPool for use with raw function calls
    ///
    pub const fn get_raw(&self) -> raw::VmaPool {
        self.pool
    }

    ///
    /// vmaGetPoolStats
    ///
    #[inline]
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
            self.allocator.as_raw(),
            self.pool,
            &mut stats as *mut raw::VmaPoolStats,
        );

        stats
    }

    ///
    /// vmaMakePoolAllocationsLost
    ///
    #[inline]
    pub unsafe fn make_pool_allocations_lost(&self) -> usize {
        let mut out = 0usize;

        raw::vmaMakePoolAllocationsLost(
            self.allocator.as_raw(),
            self.pool,
            &mut out as *mut usize as *mut _,
        );

        out
    }

    #[inline]
    pub unsafe fn check_pool_corruption(&self) -> VulkanResult<()> {
        let result = raw::vmaCheckPoolCorruption(self.allocator.as_raw(), self.pool);

        if result as i32 == 0 {
            VulkanResult::new_ok(())
        } else {
            VulkanResult::new_err(vk::Result(result as i32))
        }
    }
}

impl Drop for Pool {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            raw::vmaDestroyPool(self.allocator.as_raw(), self.pool);
        }
    }
}
