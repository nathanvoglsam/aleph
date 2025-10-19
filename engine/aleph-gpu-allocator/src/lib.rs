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

mod gpu_allocation;
mod gpu_allocator;
mod gpu_layout;

pub use gpu_allocation::GpuAllocation;
pub use gpu_allocator::{AllocatorConfig, GpuAllocator, MemoryBlock, MemoryPool};
pub use gpu_layout::GpuLayout;

/// Supported set of allocation memory locations. Used when making device allocations to determine
/// what kind of memory is used to back the allocation.
///
/// Each type has different performance and capabilities, ensure you chose the correct location for
/// your resources and workloads.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MemoryLocation {
    /// Store the allocation in device local, GPU only memory. This is where most resources should
    /// be allocated.
    GpuLocal,

    /// Store the allocation in host local, device addressable memory. Best used for CPU->GPU
    /// uploads.
    CpuToGpu,

    /// Store the allocation in memory ideal for GPU->CPU readback.
    GpuToCpu,
}

/// Bridge to a given GPU API. Each API will get an implementation of this interface.
///
/// This interface bridges the API specific details into our platform-agnostic allocator framework.
/// Anything that would require touching the GPU API goes through this. This includes:
/// - Determining which memory type to use for a resource/allocation.
/// - Enumerating and building our memory type list.
/// - Getting size and alignment for a resource description.
/// - Determine if resources should go in dedicated allocations or be sub-allocated from a block.
pub trait IApiBridge {
    /// Type passed in to all functions that may need to communicate with the platform API.
    type BridgeHandle<'b>;

    /// The buffer object handle type in the GPU API. This is created by the API bridge and returned
    /// by an allocator when allocating a resource.
    type BufferHandle: Sized;

    /// The texture object handle type in the GPU API. This is created by the API bridge and
    /// returned by an allocator when allocating a resource.
    type TextureHandle: Sized;

    /// The GPU API description of a buffer. This will be passed into the resource create function
    /// so the bridge can construct an approprite resource and bind the allocated memory to it.
    type BufferDesc<'a>: Sized + 'a;

    /// The GPU API description of a texture. This will be passed into the resource create function
    /// so the bridge can construct an approprite resource and bind the allocated memory to it.
    type TextureDesc<'a>: Sized + 'a;

    /// Cached information associated with a whole allocator. This would typically contain
    /// information about memory types and heaps queried from the GPU device.
    ///
    /// This will be queried from a bridge instance when constructing a new allocator. The
    /// information will not be requeried after construction.
    type AllocatorInfo: Sized;

    /// Cached information associated with a specific memory pool. A memory pool will typically
    /// map to a distinct type of device/host memory, and so this will typically identify the type
    /// of memory the pool is managing.
    ///
    /// The bridge will construct this in its implementation of [`IApiBridge::get_memory_pools`]
    /// when constructing the pools.
    type PoolInfo: Sized;

    /// Information associated with a specific memory block, inside a pool. This will contain your
    /// API handle to the allocated memory block as well as any other resources and information
    /// needed to bind resources to that memory block.
    type BlockInfo: Sized;

    /// API specific metadata generated for an allocation. This may contain platform specific data
    /// on what type of memory backed the allocation, or pointers to mapped addresses.
    type AllocationMetadata: Sized;

    /// Query information needed globally by the allocator from the GPU API and return an object
    /// payload that contains that information. This will be stored by a [`GpuAllocator`] and given
    /// to functions on [`IApiBridge`] to allow retreiving that information.
    fn get_allocator_info(bridge: &Self::BridgeHandle<'_>) -> Self::AllocatorInfo;

    /// Initialize the set of memory pools based on the memory types made available by the
    /// underlying API and GPU device.
    ///
    /// An index into the returned array will be stable, and always uniquely identify a pool within
    /// the context of a single allocator instance. It is expected that each distinct memory type
    /// exposed by the device/API be mapped into a memory pool object in this list. The API bridge
    /// can then map a memory request to a specific pool index based on the requested memory
    /// properties and resource type.
    ///
    /// However, the meaning of each pool in the platform API is opaque to the generic
    /// [`GpuAllocator`] object. That meaning is only important to the API bridge.
    fn get_memory_pools(
        bridge: &Self::BridgeHandle<'_>,
        info: &Self::AllocatorInfo,
        config: &AllocatorConfig,
    ) -> Vec<MemoryPool<Self>>;

    unsafe fn create_buffer_object(
        bridge: &Self::BridgeHandle<'_>,
        desc: &AllocationDesc<Self::BufferDesc<'_>>,
        allocation: &GpuAllocation,
        allocator_info: &Self::AllocatorInfo,
        pool_info: &Self::PoolInfo,
        block_info: &Self::BlockInfo,
    ) -> Result<Self::BufferHandle, ()>;
    unsafe fn destroy_buffer_object(
        bridge: &Self::BridgeHandle<'_>,
        allocator_info: &Self::AllocatorInfo,
        buffer: Self::BufferHandle,
    );

    unsafe fn create_texture_object(
        bridge: &Self::BridgeHandle<'_>,
        desc: &AllocationDesc<Self::TextureDesc<'_>>,
        allocation: &GpuAllocation,
        allocator_info: &Self::AllocatorInfo,
        pool_info: &Self::PoolInfo,
        block_info: &Self::BlockInfo,
    ) -> Result<Self::TextureHandle, ()>;
    unsafe fn destroy_texture_object(
        bridge: &Self::BridgeHandle<'_>,
        allocator_info: &Self::AllocatorInfo,
        texture: Self::TextureHandle,
    );

    unsafe fn create_block(
        bridge: &Self::BridgeHandle<'_>,
        allocator_info: &Self::AllocatorInfo,
        pool_info: &Self::PoolInfo,
        size: u64,
    ) -> Option<Self::BlockInfo>;
    unsafe fn destroy_block(
        bridge: &Self::BridgeHandle<'_>,
        allocator_info: &Self::AllocatorInfo,
        block: &mut Self::BlockInfo,
    );

    fn get_requirements_for_buffer(
        bridge: &Self::BridgeHandle<'_>,
        info: &Self::AllocatorInfo,
        desc: &AllocationDesc<Self::BufferDesc<'_>>,
    ) -> Option<MemoryRequirements>;
    fn get_requirements_for_texture(
        bridge: &Self::BridgeHandle<'_>,
        info: &Self::AllocatorInfo,
        desc: &AllocationDesc<Self::TextureDesc<'_>>,
    ) -> Option<MemoryRequirements>;

    fn get_metadata_for_allocation(
        bridge: &Self::BridgeHandle<'_>,
        info: &Self::AllocatorInfo,
        pool_info: &Self::PoolInfo,
        block_info: &Self::BlockInfo,
        allocation: &GpuAllocation,
    ) -> Self::AllocationMetadata;
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum AllocationStrategy {
    /// Search for a better fitting block that will fit the requested allocation. Will take more
    /// time but will cause less fragmentation as this strategy prefers to use blocks that have
    /// less free space.
    BestFit,

    /// Quickly search for the first block that will fit the requested allocation. Will quickly find
    /// a block at the cost of additional fragmentation. This strategy will pick the first block
    /// with enough space it finds.
    FirstFit,
}

impl Default for AllocationStrategy {
    #[inline(always)]
    fn default() -> Self {
        // We chose 'best fit' as the default to minimize fragmentation at the expense of
        // throughput.
        Self::BestFit
    }
}

pub struct AllocationDesc<T: Sized> {
    /// The type of memory that should be used to back the allocation. This is categorized by how
    /// the memory will be used. See [`MemoryLocation`] for more.
    pub location: MemoryLocation,

    /// Hint to the allocator how you would prefer the allocation to be filled. This affects how
    /// blocks are searched, leading to longer or shorter search times at the expense of additional
    /// memory fragmentation. See [`AllocationStrategy`] for more.
    pub strategy: AllocationStrategy,

    /// Platform API's description of the resource. Will be used by the allocator to create
    /// the resource through the [`IApiBridge`] interface.
    pub desc: T,
}

/// Internal struct sent between an [`IApiBridge`] and [`GpuAllocator`] to negotiate the target
/// pool, and size/align requirements for an object allocation request.
pub struct MemoryRequirements {
    /// The index of the pool the resource should be placed in.
    pub pool_index: u16,

    /// The size/align requirements for the resource. For images this is opaque so only the API
    /// runtime can tell as the values for this. For buffers size is obvious, but alignment is often
    /// platform dependent.
    pub layout: GpuLayout,
}
