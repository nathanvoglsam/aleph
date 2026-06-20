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

use aleph_alloc::BVec;
use aleph_alloc::allocator_global_handle::AllocatorGlobalHandle;
use aleph_alloc::instrumentation::system;
use aleph_rhi_impl_utils::RhiSystem;
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_foundation::NSError;
use objc2_metal::*;
use parking_lot::Mutex;

/// A sharded pool allocator that handles allocating image views in a pool of [`MTLTextureViewPool`]
/// objects.
///
/// This pool uses 'shards' to reduce cross thread contention. Whenever interacting with the pool
/// the caller is expected to provide a 'shard' index that is then used to map into the internal
/// set of pools. All allocations from a shard must be returned to the same shard, and it is the
/// caller's responsibility to ensure this is upheld.
///
/// Shards are mapped onto a fixed number of internal pools. Each individual pool is guarded by a
/// mutex. If shards are well distributed across calling threads then contention will be reduced as
/// it becomes less likely for two threads to try and lock the same pool.
///
/// Otherwise, the interface is just a standard pool allocator.
pub struct ShardedImageViewPool<const NUM_POOLS: usize> {
    pools: BVec<ImageViewPool, RhiSystem>,
    handles: Handles,
}

impl<const NUM_POOLS: usize> ShardedImageViewPool<NUM_POOLS> {
    /// Constructs a new [`ShardedImageViewPool`].
    ///
    /// Needs to retain a reference to the device for creating new image view pools.
    pub fn new(device: Retained<ProtocolObject<dyn MTLDevice>>) -> Self {
        let mut pools = BVec::new_in(RhiSystem::make_handle());
        for _ in 0..NUM_POOLS {
            let pool = ImageViewPool::new();
            pools.push(pool);
        }

        Self {
            pools,
            handles: Handles { device },
        }
    }

    /// Allocate and create a texture view in the pool for the given texture + descriptor.
    ///
    /// Returns the resource ID + a handle that can be used to free the allocation with
    /// [`ImageViewPool::free`].
    pub fn alloc(
        &self,
        shard: usize,
        texture: &ProtocolObject<dyn MTLTexture>,
        descriptor: &MTLTextureViewDescriptor,
    ) -> (MTLResourceID, ImageViewAllocation) {
        self.pools[shard % NUM_POOLS].alloc(self.handles.device.as_ref(), texture, descriptor)
    }

    /// Returns the allocation to the pool.
    ///
    /// # Safety
    ///
    /// It is the caller's responsibility to ensure the allocation being freed is returned to the
    /// allocator it was retrieved from.
    pub unsafe fn free(&self, shard: usize, allocation: ImageViewAllocation) {
        unsafe { self.pools[shard % NUM_POOLS].free(allocation) }
    }
}

/// Opaque handle that identifies an allocation within one of the shards in a
/// [`ShardedImageViewPool`].
pub struct ImageViewAllocation(pub(crate) usize);

// TODO: is it worth doing geometric growth?
const BLOCK_SIZE: usize = 4096;

struct ImageViewPool {
    inner: Mutex<Inner>,
}

impl ImageViewPool {
    fn new() -> Self {
        Self {
            inner: Mutex::new(Inner {
                count: 0,
                blocks: BVec::new_in(system()),
                free_list: BVec::new_in(system()),
            }),
        }
    }

    fn alloc(
        &self,
        device: &ProtocolObject<dyn MTLDevice>,
        texture: &ProtocolObject<dyn MTLTexture>,
        descriptor: &MTLTextureViewDescriptor,
    ) -> (MTLResourceID, ImageViewAllocation) {
        let mut inner = self.inner.lock();

        if let Some(index) = inner.free_list.pop() {
            // We found an entry in the free list.
            inner.count += 1;

            // Map the object index to the block, and index within that block
            let block_index = index % BLOCK_SIZE;
            let index_in_block = index / BLOCK_SIZE;

            // Get the resource ID for the slot within the block
            let block = &mut inner.blocks[block_index];
            let id_base = block.base.to_raw();
            let id = id_base + index_in_block as u64;
            let id = unsafe { MTLResourceID::from_raw(id) };

            // Write the view into the pool at the correct index
            unsafe {
                block
                    .pool
                    .setTextureView_descriptor_atIndex(texture, descriptor, index_in_block);
            }

            (id, ImageViewAllocation(index))
        } else {
            // Allocate a new index as the free list is exhausted
            //
            // If the free list is empty then 'inner.count' will be both the number of objects
            // allocated in the pool, but also the index of the next object.
            let index = inner.count;
            inner.count += 1;

            // Map the object index to the block, and index within that block
            let block_index = index % BLOCK_SIZE;
            let index_in_block = index / BLOCK_SIZE;

            if let Some(block) = inner.blocks.get_mut(block_index) {
                // If we reach here then we still have space in the block list to immediately make
                // a new view.
                //
                // So get the ID and...
                let id_base = block.base.to_raw();
                let id = id_base + index_in_block as u64;
                let id = unsafe { MTLResourceID::from_raw(id) };

                // Write the view
                unsafe {
                    block.pool.setTextureView_descriptor_atIndex(
                        texture,
                        descriptor,
                        index_in_block,
                    );
                }

                (id, ImageViewAllocation(index))
            } else {
                // If we reach here then 'index' projected into a block that doesn't exist. This
                // means all our blocks are full so we need to make a new one.
                //
                // So we make the new block...
                let block = Self::new_block(device, &mut inner).unwrap();

                // Get the resource ID
                let id_base = block.base.to_raw();
                let id = id_base + index_in_block as u64;
                let id = unsafe { MTLResourceID::from_raw(id) };

                // And write the view
                unsafe {
                    block.pool.setTextureView_descriptor_atIndex(
                        texture,
                        descriptor,
                        index_in_block,
                    );
                }

                (id, ImageViewAllocation(index))
            }
        }
    }

    unsafe fn free(&self, allocation: ImageViewAllocation) {
        // We simply assume the allocation we've been given is:
        //   1. Live (i.e. hasn't already been freed)
        //   2. Was originally allocated from this pool
        //
        // It's the caller's job to make sure this holds true.
        let mut inner = self.inner.lock();
        inner.free_list.push(allocation.0);
        inner.count -= 1;
    }

    fn new_block<'b>(
        device: &ProtocolObject<dyn MTLDevice>,
        inner: &'b mut Inner,
    ) -> Result<&'b mut Block, Retained<NSError>> {
        let descriptor = MTLResourceViewPoolDescriptor::new();
        unsafe {
            descriptor.setResourceViewCount(BLOCK_SIZE);
        }

        let pool = device.newTextureViewPoolWithDescriptor_error(&descriptor)?;
        let base = pool.baseResourceID();

        let i = inner.blocks.len();
        inner.blocks.push(Block { pool, base });

        Ok(&mut inner.blocks[i])
    }
}

struct Inner {
    count: usize,
    blocks: BVec<Block, RhiSystem>,
    free_list: BVec<usize, RhiSystem>,
}

struct Block {
    pool: Retained<ProtocolObject<dyn MTLTextureViewPool>>,
    base: MTLResourceID,
}

struct Handles {
    device: Retained<ProtocolObject<dyn MTLDevice>>,
}
