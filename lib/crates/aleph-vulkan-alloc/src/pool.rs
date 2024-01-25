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

use std::ptr::NonNull;
use std::sync::Arc;

use ash::prelude::VkResult;
use ash::vk;
use raw::PoolCreateFlags;

use crate::raw::{PoolCreateInfo, PoolH};
use crate::{raw, vma};

///
/// Builder for a VmaPool, wraps VmaPoolCreateInfo, vmaCreatePool and vmaDestroyPool (drop)
///
pub struct PoolBuilder {
    create_info: PoolCreateInfo,
}

impl PoolBuilder {
    pub const fn new() -> Self {
        PoolBuilder {
            create_info: PoolCreateInfo {
                memory_type_index: 0,
                flags: PoolCreateFlags::empty(),
                block_size: 0,
                min_block_count: 0,
                max_block_count: 0,
                priority: 0.0,
                min_allocation_alignment: 0,
                p_memory_allocate_next: None,
            },
        }
    }

    pub const fn memory_type_index(mut self, v: u32) -> Self {
        self.create_info.memory_type_index = v;
        self
    }

    pub const fn flags(mut self, v: PoolCreateFlags) -> Self {
        self.create_info.flags = v;
        self
    }

    pub const fn block_size(mut self, v: vk::DeviceSize) -> Self {
        self.create_info.block_size = v;
        self
    }

    pub const fn min_block_count(mut self, v: usize) -> Self {
        self.create_info.min_block_count = v;
        self
    }

    pub const fn max_block_count(mut self, v: usize) -> Self {
        self.create_info.max_block_count = v;
        self
    }

    pub const fn priority(mut self, v: f32) -> Self {
        self.create_info.priority = v;
        self
    }

    pub const fn min_allocation_alignment(mut self, v: vk::DeviceSize) -> Self {
        self.create_info.min_allocation_alignment = v;
        self
    }

    ///
    /// vmaCreatePool
    ///
    #[inline]
    pub unsafe fn build(self, allocator: &Arc<vma::Allocator>) -> VkResult<Arc<Pool>> {
        let mut pool: Option<PoolH> = None;
        let result = raw::vmaCreatePool(
            allocator.as_raw(),
            NonNull::from(&self.create_info),
            NonNull::from(&mut pool),
        );

        result.result_with_success(Arc::new(Pool {
            pool: pool.unwrap(),
            allocator: allocator.clone(),
        }))
    }
}

///
/// VmaPool
///
pub struct Pool {
    pool: PoolH,
    allocator: Arc<vma::Allocator>,
}

impl Pool {
    ///
    /// Returns the underlying raw::VmaPool for use with raw function calls
    ///
    pub const fn get_raw(&self) -> PoolH {
        self.pool
    }

    #[inline]
    pub unsafe fn check_pool_corruption(&self) -> VkResult<()> {
        let result = raw::vmaCheckPoolCorruption(self.allocator.as_raw(), self.pool);
        result.result()
    }
}

impl Drop for Pool {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            raw::vmaDestroyPool(self.allocator.as_raw(), Some(self.pool));
        }
    }
}
