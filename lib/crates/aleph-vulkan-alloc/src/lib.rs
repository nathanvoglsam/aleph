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

#![allow(clippy::missing_safety_doc)]

mod allocation;
mod allocator;
mod pool;
mod raw;
mod utils;
mod vulkan_functions;

pub mod vma {
    pub use crate::allocation::{Allocation, AllocationCreateInfoBuilder};
    pub use crate::allocator::{Allocator, AllocatorBuilder, AllocatorBuilderError};
    pub use crate::pool::{Pool, PoolBuilder};
    pub use crate::raw::{
        AllocationCreateFlags, AllocationCreateInfo, AllocationInfo, AllocatorCreateFlags,
        AllocatorCreateInfo, AllocatorInfo, Budget, DefragmentationFlags, DefragmentationInfo,
        DefragmentationMove, DefragmentationMoveOperation, DefragmentationPassMoveInfo,
        DefragmentationStats, DetailedStatistics, DeviceMemoryCallbacks, MemoryUsage,
        PFN_vmaAllocateDeviceMemoryFunction, PFN_vmaFreeDeviceMemoryFunction, PoolCreateFlags,
        PoolCreateInfo, Statistics, TotalStatistics, VirtualAllocationCreateFlags,
        VirtualAllocationCreateInfo, VirtualAllocationInfo, VirtualBlockCreateFlags,
        VirtualBlockCreateInfo, VulkanFunctions,
    };
}
