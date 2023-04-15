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
use erupt::vk;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;

pub use raw::AllocationCreateFlags;

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
    pub const fn from_raw(memory_usage: raw::VmaMemoryUsage) -> Self {
        match memory_usage {
            raw::VMA_MEMORY_USAGE_UNKNOWN => MemoryUsage::Unknown,
            raw::VMA_MEMORY_USAGE_GPU_ONLY => MemoryUsage::GPUOnly,
            raw::VMA_MEMORY_USAGE_CPU_ONLY => MemoryUsage::CPUOnly,
            raw::VMA_MEMORY_USAGE_CPU_TO_GPU => MemoryUsage::CPUToGPU,
            raw::VMA_MEMORY_USAGE_GPU_TO_CPU => MemoryUsage::GPUToCPU,
            _ => panic!("Invalid VmaMemoryUsage variant"),
        }
    }

    ///
    /// Convert our enum back into the raw VmaMemoryUsage value
    ///
    pub const fn into_raw(self) -> raw::VmaMemoryUsage {
        match self {
            MemoryUsage::Unknown => raw::VMA_MEMORY_USAGE_UNKNOWN,
            MemoryUsage::GPUOnly => raw::VMA_MEMORY_USAGE_GPU_ONLY,
            MemoryUsage::CPUOnly => raw::VMA_MEMORY_USAGE_CPU_ONLY,
            MemoryUsage::CPUToGPU => raw::VMA_MEMORY_USAGE_CPU_TO_GPU,
            MemoryUsage::GPUToCPU => raw::VMA_MEMORY_USAGE_GPU_TO_CPU,
        }
    }
}

///
/// Wrapper around a VmaAllocation
///
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct Allocation {
    allocation: raw::VmaAllocation,
}

impl Allocation {
    ///
    /// Create from a raw handle.
    ///
    /// Must have a lifetime shorter than it's parent allocator
    ///
    pub const fn from_raw(allocation: raw::VmaAllocation) -> Self {
        Allocation { allocation }
    }

    ///
    /// Gets a "null" allocation
    ///
    #[inline]
    pub const fn null() -> Self {
        Self {
            allocation: ptr::null_mut(),
        }
    }

    ///
    /// Returns the underlying raw::VmaAllocation for use with raw function calls
    ///
    pub const fn into_raw(self) -> raw::VmaAllocation {
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
    flags: AllocationCreateFlags,
    usage: MemoryUsage,
    required_flags: vk::MemoryPropertyFlags,
    preferred_flags: vk::MemoryPropertyFlags,
    memory_type_bits: u32,
    pool: raw::VmaPool,
    p_user_data: *mut ::std::os::raw::c_void,
}

impl AllocationCreateInfo {
    pub const fn builder() -> AllocationCreateInfoBuilder {
        AllocationCreateInfoBuilder::new()
    }

    pub(crate) const fn into_raw(self) -> raw::VmaAllocationCreateInfo {
        raw::VmaAllocationCreateInfo {
            flags: self.flags,
            usage: self.usage.into_raw(),
            required_flags: self.required_flags,
            preferred_flags: self.preferred_flags,
            memory_type_bits: self.memory_type_bits,
            pool: self.pool,
            p_user_data: self.p_user_data,
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
    pub const fn new() -> Self {
        let info = AllocationCreateInfo {
            flags: AllocationCreateFlags::empty(),
            usage: MemoryUsage::Unknown,
            required_flags: vk::MemoryPropertyFlags::empty(),
            preferred_flags: vk::MemoryPropertyFlags::empty(),
            memory_type_bits: 0,
            pool: ptr::null_mut(),
            p_user_data: ptr::null_mut(),
        };
        AllocationCreateInfoBuilder { info }
    }

    ///
    ///
    ///
    pub const fn flags(mut self, flags: AllocationCreateFlags) -> Self {
        self.info.flags = flags;
        self
    }

    ///
    ///
    ///
    pub const fn required_flags(mut self, flags: vk::MemoryPropertyFlags) -> Self {
        self.info.required_flags = flags;
        self
    }

    ///
    ///
    ///
    pub const fn preferred_flags(mut self, flags: vk::MemoryPropertyFlags) -> Self {
        self.info.preferred_flags = flags;
        self
    }

    ///
    ///
    ///
    pub const fn usage(mut self, usage: MemoryUsage) -> Self {
        self.info.usage = usage;
        self
    }

    ///
    ///
    ///
    pub const fn pool(mut self, pool: &vma::Pool) -> Self {
        self.info.pool = pool.get_raw();
        self
    }

    ///
    ///
    ///
    pub const fn build(self) -> AllocationCreateInfo {
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

impl Debug for AllocationCreateInfoBuilder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.info.fmt(f)
    }
}

pub type AllocationInfo = raw::VmaAllocationInfo;
