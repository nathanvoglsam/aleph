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

use crate::raw;
use crate::vma;
use core::ptr;
use erupt::utils::VulkanResult;
use erupt::vk;
use std::sync::Arc;

pub use raw::PoolCreateFlags;

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
                flags: PoolCreateFlags::empty(),
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
    pub const fn flags(mut self, flags: PoolCreateFlags) -> Self {
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

        let result = raw::vmaCreatePool(allocator.as_raw(), create_ptr, &mut pool);

        debug_assert!(pool.is_null(), "Pool should not be null");

        VulkanResult::new(
            result,
            Arc::new(Pool {
                pool,
                allocator: allocator.clone(),
            }),
        )
    }
}

pub type PoolCreateInfo = raw::VmaPoolCreateInfo;

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
            unused_size: 0,
            allocation_count: 0,
            unused_range_count: 0,
            unused_range_size_max: 0,
            block_count: 0,
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

        VulkanResult::new(result, ())
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
