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

use aleph_alloc::offset_allocator;
use parking_lot::Mutex;

use crate::{
    AllocationDesc, AllocationStrategy, GpuAllocation, GpuLayout, IApiBridge, MemoryRequirements,
};

pub struct GpuAllocator<T: IApiBridge> {
    memory_pools: Vec<MemoryPool<T>>,
    info: T::AllocatorInfo,
}

impl<T: IApiBridge> GpuAllocator<T> {
    pub fn new(bridge: &T::BridgeHandle<'_>) -> Self {
        let config = AllocatorConfig::default();
        let info = T::get_allocator_info(bridge);
        let mut memory_pools = T::get_memory_pools(bridge, &info, &AllocatorConfig::default());
        for (pool_index, memory_pool) in memory_pools.iter_mut().enumerate() {
            memory_pool.pool_index = pool_index as u16;
            memory_pool.config = config.clone();
        }
        Self { memory_pools, info }
    }

    /// # Safety
    ///
    /// - 'bridge' must be the exact same object that the allocator was constructed with.
    /// - [`IApiBridge::create_buffer_object`] will be called, the requirements of that function
    ///   must also be upheld here.
    /// - [`IApiBridge::create_block`] may be called, the requirements of that function must also be
    ///   upheld here.
    /// - 'self' must not have been destroyed with [`GpuAllocator::destroy`].
    pub unsafe fn allocate_buffer(
        &self,
        bridge: &T::BridgeHandle<'_>,
        desc: &AllocationDesc<T::BufferDesc<'_>>,
    ) -> Option<(GpuAllocation, T::AllocationMetadata, T::BufferHandle)> {
        let requirements = T::get_requirements_for_buffer(bridge, &self.info, desc)?;
        let pool = self.memory_pools.get(requirements.pool_index as usize)?;

        // Safety: unsafe because of the 'create_buffer' call.
        unsafe { pool.allocate_buffer(bridge, &self.info, &requirements, desc) }
    }

    /// # Safety
    ///
    /// - 'bridge' must be the exact same object that the allocator was constructed with.
    /// - [`IApiBridge::create_texture_object`] will be called, the requirements of that function
    ///   must also be upheld here.
    /// - 'self' must not have been destroyed with [`GpuAllocator::destroy`].
    pub unsafe fn allocate_texture(
        &self,
        bridge: &T::BridgeHandle<'_>,
        desc: &AllocationDesc<T::TextureDesc<'_>>,
    ) -> Option<(GpuAllocation, T::AllocationMetadata, T::TextureHandle)> {
        let requirements = T::get_requirements_for_texture(bridge, &self.info, desc)?;
        let pool = self.memory_pools.get(requirements.pool_index as usize)?;

        // Safety: unsafe because of the 'create_texture' call.
        unsafe { pool.allocate_texture(bridge, &self.info, &requirements, desc) }
    }

    /// # Safety
    ///
    /// - 'bridge' must be the exact same object that the allocator was constructed with.
    /// - The caller must have destroyed any buffer/texture objects occupying the allocation before
    ///   calling this function.
    /// - 'allocation' must have been created from the same allocator instance as it is being freed
    ///   from.
    /// - 'self' must not have been destroyed with [`GpuAllocator::destroy`].
    pub unsafe fn free_allocation(
        &self,
        bridge: &T::BridgeHandle<'_>,
        allocation: GpuAllocation,
    ) -> Option<()> {
        let pool = allocation.pool_index as usize;
        let pool = self.memory_pools.get(pool)?;
        pool.free_allocation(bridge, &self.info, allocation);
        Some(())
    }

    /// Destroy all device resources associated with the allocator
    ///
    /// This is logically equivalent to freeing all allocations, but will also purge all allocated
    /// blocks and internal API resources. Once called the allocator is no longer valid to use and
    /// will be unable to serve new allocations.
    ///
    /// This function is guaranteed to free all device and host memory blocks back to the system.
    ///
    /// It is the caller's responsibility to destroy the resources that were placed into the memory
    /// backed by a [`GpuAllocator`].
    ///
    /// # Destruction
    ///
    /// This _must_ be called before dropping the allocator to guarantee that all internal API
    /// resources are freed correctly. Some APIs may not be able to use 'Drop' to do proper clean
    /// up, as they require a handle to a device context in order to destroy the resources.
    ///
    /// You _can_ ignore calling this function, however you _may_ leak API resource handles if you
    /// do so. It may not be sound to leak GPU API handles.
    ///
    /// # Safety
    ///
    /// - No object or allocation created by a given allocator can be in use, on the host or on the
    ///   device, when this is called on the source allocator.
    /// - All [`GpuAllocation`] handles allocated by a given allocator instance become dangling as
    ///   soon as a thread enters this function. It is unsound to call 'free_allocation' using
    ///   allocation handles created prior to calling this function.
    /// - 'bridge' must be the exact same object that the allocator was constructed with.
    /// - 'self' must not have already been destroyed with [`GpuAllocator::destroy`].
    pub unsafe fn destroy(&mut self, bridge: &T::BridgeHandle<'_>) {
        self.memory_pools.iter_mut().for_each(|pool| {
            let blocks = &mut pool.pool_blocks.get_mut().memory_blocks;
            blocks.iter_mut().for_each(|block| unsafe {
                T::destroy_block(bridge, &self.info, &mut block.info);
            });

            let blocks = &mut pool.dedicated_blocks.get_mut().memory_blocks;
            blocks.iter_mut().for_each(|block| unsafe {
                T::destroy_block(bridge, &self.info, block);
            });
        });
    }
}

pub struct MemoryPool<T: IApiBridge + ?Sized> {
    /// The index of this memory pool in the pool list of the owning [`GpuAllocator`].
    pool_index: u16,

    /// Allocator configuration. Given from the owning [`GpuAllocator`].
    config: AllocatorConfig,

    /// The set of memory blocks currently managed by the pool. This is managed separately to the
    /// set of dedicated blocks
    pool_blocks: Mutex<PoolBlocks<T>>,

    /// The set of blocks that back allocations using a dedicated blockl
    dedicated_blocks: Mutex<DedicatedBlocks<T>>,

    /// Backend specific information that should be attached to memory pools.
    info: T::PoolInfo,
}

impl<T: IApiBridge + ?Sized> MemoryPool<T> {
    pub fn new(info: T::PoolInfo) -> Self {
        Self {
            pool_index: 0,              // will be patched by the gpu allocator
            config: Default::default(), // will be patched by the gpu allocator
            pool_blocks: Default::default(),
            dedicated_blocks: Default::default(),
            info,
        }
    }

    pub(crate) unsafe fn allocate_buffer(
        &self,
        bridge: &T::BridgeHandle<'_>,
        info: &T::AllocatorInfo,
        requirements: &MemoryRequirements,
        desc: &AllocationDesc<T::BufferDesc<'_>>,
    ) -> Option<(GpuAllocation, T::AllocationMetadata, T::BufferHandle)> {
        let layout = &requirements.layout;

        // If the requested allocation size is greater than half the default block size then we
        // opt to force a dedicated allocation for this allocation instead. This is a heuristic
        // to avoid long searches for blocks with enough space.
        if layout.size() > self.config.default_block_size as u64 / 2 {
            self.allocate_dedicated_block(
                bridge,
                info,
                requirements,
                desc,
                T::create_buffer_object,
                T::destroy_buffer_object,
            )
        } else {
            let mut pool_blocks = self.pool_blocks.lock();
            let allocation = unsafe {
                self.allocate_block(bridge, info, &mut pool_blocks, desc.strategy, layout)?
            };

            // If we reach here then we have successfully sub-allocated from a block.
            let block = &pool_blocks.memory_blocks[allocation.block_index as usize];
            let buffer = unsafe {
                T::create_buffer_object(bridge, desc, &allocation, info, &self.info, &block.info)
                    .ok()?
            };

            // Query any extra, API specific metadata we should return to the caller describing the
            // allocation.
            let metadata =
                T::get_metadata_for_allocation(bridge, info, &self.info, &block.info, &allocation);

            // Perform a single round of sorting to try and maintain a roughly sorted order of
            pool_blocks.incrementally_sort_blocks_by_free_size();

            Some((allocation, metadata, buffer))
        }
    }

    pub(crate) unsafe fn allocate_texture(
        &self,
        bridge: &T::BridgeHandle<'_>,
        info: &T::AllocatorInfo,
        requirements: &MemoryRequirements,
        desc: &AllocationDesc<T::TextureDesc<'_>>,
    ) -> Option<(GpuAllocation, T::AllocationMetadata, T::TextureHandle)> {
        let layout = &requirements.layout;

        // If the requested allocation size is greater than half the default block size then we
        // opt to force a dedicated allocation for this allocation instead. This is a heuristic
        // to avoid long searches for blocks with enough space.
        if layout.size() > self.config.default_block_size as u64 / 2 {
            self.allocate_dedicated_block(
                bridge,
                info,
                requirements,
                desc,
                T::create_texture_object,
                T::destroy_texture_object,
            )
        } else {
            let mut pool_blocks = self.pool_blocks.lock();
            let allocation = unsafe {
                self.allocate_block(bridge, info, &mut pool_blocks, desc.strategy, layout)?
            };

            // If we reach here then we have successfully sub-allocated from a block.
            let block = &pool_blocks.memory_blocks[allocation.block_index as usize];
            let texture = unsafe {
                T::create_texture_object(bridge, desc, &allocation, info, &self.info, &block.info)
                    .ok()?
            };

            // Query any extra, API specific metadata we should return to the caller describing the
            // allocation.
            let metadata =
                T::get_metadata_for_allocation(bridge, info, &self.info, &block.info, &allocation);

            // Perform a single round of sorting to try and maintain a roughly sorted order of
            pool_blocks.incrementally_sort_blocks_by_free_size();

            Some((allocation, metadata, texture))
        }
    }

    unsafe fn allocate_block(
        &self,
        bridge: &T::BridgeHandle<'_>,
        info: &T::AllocatorInfo,
        pool_blocks: &mut PoolBlocks<T>,
        strategy: AllocationStrategy,
        layout: &GpuLayout,
    ) -> Option<GpuAllocation> {
        fn try_allocate_from_blocks<T: IApiBridge + ?Sized>(
            memory_blocks: &mut [MemoryBlock<T>],
            layout: &GpuLayout,
            iter: impl Iterator<Item = usize>,
        ) -> Option<GpuAllocation> {
            'search: for i in iter {
                let block = &mut memory_blocks[i];

                // Reject blocks that don't have enough free bytes immediately
                if layout.size() > block.free_bytes() as u64 {
                    continue 'search;
                }

                // Otherwise, try and allocate from the block. This may still fail because of memory
                // fragmentation. If we fail here move to the next block.
                match block.allocate(&layout) {
                    None => {
                        // No luck allocating? Try the next block...
                        continue 'search;
                    }
                    Some(v) => {
                        return Some(v);
                    }
                }
            }

            // No block exists with enough space for our allocation...
            None
        }

        // forward iter = best fit
        // reverse iter = first fit
        let allocation = match strategy {
            AllocationStrategy::BestFit => try_allocate_from_blocks::<T>(
                &mut pool_blocks.memory_blocks,
                layout,
                pool_blocks.sorted_blocks.iter().copied(),
            ),
            AllocationStrategy::FirstFit => try_allocate_from_blocks::<T>(
                &mut pool_blocks.memory_blocks,
                layout,
                pool_blocks.sorted_blocks.iter().rev().copied(),
            ),
        };

        // We got an allocation from an existing block? We're done! Otherwise, we must create a new
        // block.
        if let Some(allocation) = allocation {
            return Some(allocation);
        }

        // If we've made too many blocks that we can't index with a u16 anymore then we're in a
        // bad spot. We'd need 16TB of blocks with a 256MB default block size to hit this.
        // Unlikely on consumer GPUs for a long time.
        //
        // Treat it like OOM and return an error.
        let block_index = u16::try_from(pool_blocks.memory_blocks.len()).ok()?;

        // Create a new block to satisfy the allocation, as we've failed to find space in an
        // existing memory block.
        let block_size = self.config.default_block_size;
        let block = unsafe { T::create_block(bridge, info, &self.info, block_size as u64)? };

        // Insert a new block object into the memory block set.
        let block = MemoryBlock::<T> {
            sub_allocator: offset_allocator::OffsetAllocator::new(block_size, 65535),
            pool_index: self.pool_index,
            block_index,
            block_size,
            used_bytes: 0,
            info: block,
        };
        pool_blocks.memory_blocks.push(block);
        pool_blocks.sorted_blocks.push(block_index as usize);

        // Query the block we just inserted back and serve the allocation from that new block.
        //
        // By inserting and re-querying the block this way it means we're left in a valid state
        // if the 'allocate' call below fails. If the call fails we leave an empty block in the
        // set which will quickly get used by other allocations.
        //
        // If we didn't do it this way we'd have to either free the block, or ensure we insert
        // the block into the set in both the fail and success code paths.
        let block = &mut pool_blocks.memory_blocks[block_index as usize];
        block.allocate(layout)
    }

    fn allocate_dedicated_block<DT, HT>(
        &self,
        bridge: &T::BridgeHandle<'_>,
        info: &T::AllocatorInfo,
        requirements: &MemoryRequirements,
        desc: &DT,
        init_r: unsafe fn(
            &T::BridgeHandle<'_>,
            &DT,
            &GpuAllocation,
            &T::AllocatorInfo,
            &T::PoolInfo,
            &T::BlockInfo,
        ) -> Result<HT, ()>,
        free_r: unsafe fn(&T::BridgeHandle<'_>, &T::AllocatorInfo, HT),
    ) -> Option<(GpuAllocation, T::AllocationMetadata, HT)> {
        let layout = &requirements.layout;

        // We do the create_block -> create_resource dance without taking the lock so we don't
        // block other threads while negotiating with the GPU API. This should mean the time we
        // spent locked is tiny.
        //
        // Create a fresh block to serve the allocation. May fail!
        let mut block = unsafe { T::create_block(bridge, info, &self.info, layout.size())? };

        // 'allocation.is_fail()' is our niche for a dedicated block, hence Default::default()
        // for 'allocation'.
        //
        // Partially init the allocation so we can create the resource. We patch the block index
        // after we take the lock on 'dedicated_blocks' and get the final block index.
        let mut allocation = GpuAllocation {
            allocation: Default::default(),
            layout: layout.clone(),
            block_offset: 0,
            pool_index: self.pool_index,
            block_index: 0,
        };

        // Try to create the resource before taking 'dedicated_blocks' lock. Make sure we destroy
        // the block if this fails. Otherwise, we'd leak the block.
        let resource = unsafe {
            let result = init_r(bridge, desc, &allocation, info, &self.info, &block).ok();
            match result {
                None => {
                    T::destroy_block(bridge, info, &mut block);
                    return None;
                }
                Some(v) => v,
            }
        };

        // Query any extra, API specific metadata we should return to the caller describing the
        // allocation.
        let metadata =
            T::get_metadata_for_allocation(bridge, info, &self.info, &block, &allocation);

        // Finally we insert the new block into the 'dedicated_blocks' set. Again handling the
        // case we fail here by freeing both the resource _and_ the block.
        let mut dedicated_blocks = self.dedicated_blocks.lock();
        match dedicated_blocks.insert_new_block(block) {
            Ok(index) => {
                allocation.block_index = index;
            }
            Err(mut block) => {
                // Unlock the mutex as we don't need to hold it while we do cleanup on allocation
                // failure
                drop(dedicated_blocks);
                unsafe { free_r(bridge, info, resource) };
                unsafe { T::destroy_block(bridge, info, &mut block) };
                return None;
            }
        }

        Some((allocation, metadata, resource))
    }

    pub(crate) fn free_allocation(
        &self,
        bridge: &T::BridgeHandle<'_>,
        info: &T::AllocatorInfo,
        allocation: GpuAllocation,
    ) {
        debug_assert_eq!(self.pool_index, allocation.pool_index);

        if allocation.is_dedicated() {
            let mut dedicated_blocks = self.dedicated_blocks.lock();
            dedicated_blocks.free_blocks.push(allocation.block_index);

            let block = &mut dedicated_blocks.memory_blocks[allocation.block_index as usize];
            unsafe {
                T::destroy_block(bridge, info, block);
            }
        } else {
            let pool_blocks = &mut self.pool_blocks.lock();
            let block = &mut pool_blocks.memory_blocks[allocation.block_index as usize];
            block.deallocate(allocation);

            pool_blocks.incrementally_sort_blocks_by_free_size();
        }
    }
}

struct PoolBlocks<T: IApiBridge + ?Sized> {
    /// Set of all sub-allocated memory blocks in this pool.
    memory_blocks: Vec<MemoryBlock<T>>,

    /// Associated table that provides a sorted view of 'memory_blocks'. This table stores indices
    /// into the 'memory_blocks' list and will be incrementally sorted to provide an index of blocks
    /// from least to most free space.
    sorted_blocks: Vec<usize>,
}

impl<T: IApiBridge + ?Sized> Default for PoolBlocks<T> {
    fn default() -> Self {
        Self {
            memory_blocks: Default::default(),
            sorted_blocks: Default::default(),
        }
    }
}

impl<T: IApiBridge + ?Sized> PoolBlocks<T> {
    // fn sort_blocks_by_free_size(&mut self) {
    //     self.sorted_blocks
    //         .sort_by_key(|&i| self.memory_blocks[i].free_bytes());
    // }

    fn incrementally_sort_blocks_by_free_size(&mut self) {
        if self.memory_blocks.is_empty() {
            return;
        }

        // Bubble sort only until first swap.
        for i in 1..self.sorted_blocks.len() {
            let block_prev_i = self.sorted_blocks[i - 1];
            let prev_block = &self.memory_blocks[block_prev_i];

            let block_i = self.sorted_blocks[i];
            let block = &self.memory_blocks[block_i];

            if prev_block.free_bytes() > block.free_bytes() {
                self.sorted_blocks.swap(i - 1, i);
                return;
            }
        }
    }
}

struct DedicatedBlocks<T: IApiBridge + ?Sized> {
    /// Backing storage for each dedicated memory block. This will contain a mix of live and dead
    /// memory blocks, which is required to keep indices into this array stable.
    memory_blocks: Vec<T::BlockInfo>,

    /// A list of indices into 'memory_blocks' that are free to reuse.
    ///
    /// This _could_ use a union based linked list, stored in the space of free block entries. It
    /// would save memory. However, the memory overhead is small compared to the average size of a
    /// dedicated block. I think the simplicity of two arrays is worth the memory cost.
    free_blocks: Vec<u16>,
}

impl<T: IApiBridge + ?Sized> Default for DedicatedBlocks<T> {
    fn default() -> Self {
        Self {
            memory_blocks: Default::default(),
            free_blocks: Default::default(),
        }
    }
}

impl<T: IApiBridge + ?Sized> DedicatedBlocks<T> {
    fn insert_new_block(&mut self, block: T::BlockInfo) -> Result<u16, T::BlockInfo> {
        if let Some(index) = self.free_blocks.pop() {
            self.memory_blocks[index as usize] = block;
            Ok(index)
        } else {
            // If we run out of indices (too many blocks) then we error as-if we hit OOM. We should
            // exhaust the available memory before we hit this error so this shouldn't be a problem
            // in practice.
            //
            // In the (distant) future we may need to increase the bits we use for block indices,
            // but only once GPUs are shipping with 10+TB of VRAM.
            match u16::try_from(self.memory_blocks.len()) {
                Ok(index) => {
                    self.memory_blocks.push(block);
                    Ok(index)
                }
                Err(_) => Err(block),
            }
        }
    }
}

pub struct MemoryBlock<T: IApiBridge + ?Sized> {
    /// The actual allocator implementation.
    sub_allocator: offset_allocator::OffsetAllocator,

    /// The index of the owning memory pool in the pool list of the owning [`GpuAllocator`].
    pool_index: u16,

    /// The index of this block in the block list of the owning memory pool.
    block_index: u16,

    /// The size of the block we're closing over, in bytes. Is u32 to guarantee the size is less
    /// than isize::MAX.
    block_size: u32,

    /// The number of bytes that have not been consumed from the block by allocations. This is not
    /// necessarily the largest allocation we could serve from the block. Heap fragmentation may
    /// cause the largest allocation we can serve to be smaller than this value.
    ///
    /// Can be used as a heuristic to how full the block is.
    used_bytes: u32,

    /// Any memory block state that the client API needs. This will likely contain the API handle
    /// to the memory page we pull from the API.
    info: T::BlockInfo,
}

impl<T: IApiBridge + ?Sized> MemoryBlock<T> {
    fn allocate(&mut self, layout: &GpuLayout) -> Option<GpuAllocation> {
        // Guard against overflows
        let alloc_size = layout.size().saturating_add(layout.alignment());
        let alloc_size = u32::try_from(alloc_size).ok()?;

        // If the allocation is bigger than the owned block then obviously we can't continue.
        if alloc_size > self.block_size {
            return None;
        }

        // Allocte a block, padded out so we can offset within the block to satisfy the requested
        // alignment.
        //
        // We don't deal with buffer/image granularity here, we make the choice to not place images
        // and buffers in the same heaps.
        let allocation = self.sub_allocator.allocate(alloc_size);
        if allocation.is_fail() {
            return None;
        }

        // Add 'alignment' to the allocation's offset, then chop off the least significant bits so
        // we get an offset starting at the requested alignment. The allocation is guaranteed to be
        // big enough because pad for the worst case.
        let block_start = allocation.offset as u64 + layout.alignment();
        let block_start = block_start & !(layout.alignment() - 1);
        match u32::try_from(block_start).ok() {
            None => {
                self.sub_allocator.free(allocation);
                None
            }
            Some(block_start) => {
                debug_assert!(block_start as u64 + layout.size() < self.block_size as u64);

                // Update the heuristic for number of bytes used from the block
                self.used_bytes += alloc_size;
                debug_assert!(self.used_bytes <= self.block_size);

                Some(GpuAllocation {
                    allocation,
                    layout: layout.clone(),
                    block_offset: block_start,
                    pool_index: self.pool_index,
                    block_index: self.block_index,
                })
            }
        }
    }

    fn deallocate(&mut self, allocation: GpuAllocation) {
        debug_assert_eq!(self.pool_index, allocation.pool_index);
        debug_assert_eq!(self.block_index, allocation.block_index);

        let GpuAllocation {
            allocation, layout, ..
        } = allocation;

        if allocation.is_fail() {
            return;
        }

        // The implementation should make it impossible to construct a GpuAllocation where this can
        // overflow a u32.
        let alloc_size = layout.size() + layout.alignment();
        let alloc_size = alloc_size as u32;

        self.sub_allocator.free(allocation);

        // Update the heuristic for number of bytes used from the block
        self.used_bytes -= alloc_size;
    }

    const fn free_bytes(&self) -> u32 {
        self.block_size - self.used_bytes
    }
}

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct AllocatorConfig {
    pub default_block_size: u32,
    pub default_host_block_size: u32,
}

impl Default for AllocatorConfig {
    fn default() -> Self {
        const MB: u32 = 1024 * 1024;
        Self {
            default_block_size: 256 * MB,
            default_host_block_size: 64 * MB,
        }
    }
}
