//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

pub extern crate aleph_vulkan_alloc_sys as vulkan_alloc_sys;

mod allocation;
mod allocator;
mod defer;
mod pool;
mod stats;
mod utils;
mod vulkan_functions;

pub use self::allocation::{
    Allocation, AllocationCreateFlag, AllocationCreateInfo, AllocationCreateInfoBuilder,
    AllocationInfo, MemoryUsage,
};
pub use self::allocator::{
    Allocator, AllocatorBuilder, AllocatorBuilderError, AllocatorCreateFlag,
};
pub use self::pool::{Pool, PoolBuilder, PoolCreateFlag};
pub use self::stats::{PoolStats, StatInfo, Stats};
