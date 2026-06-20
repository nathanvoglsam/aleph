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

use aleph_gpu_allocator::{
    AllocationDesc, AllocatorConfig, GpuAllocation, GpuLayout, IApiBridge, MemoryLocation,
    MemoryPool, MemoryRequirements, PoolConfig,
};
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_metal::*;
use parking_lot::Mutex;

use crate::device::Device;
use crate::internal::residency_set::ResidencySet;

pub struct MetalAllocatorBridge;

impl<'a> IApiBridge for MetalAllocatorBridge {
    type BridgeHandle<'b> = Device;
    type BufferHandle = Retained<ProtocolObject<dyn MTLBuffer>>;
    type TextureHandle = Retained<ProtocolObject<dyn MTLTexture>>;
    type BufferDesc<'b> = usize;
    type TextureDesc<'b> = Retained<MTLTextureDescriptor>;
    type AllocatorInfo = ();
    type PoolInfo = MetalPoolInfo;
    type BlockInfo = MetalBlockInfo;
    type DedicatedBlockInfo = Option<Retained<ProtocolObject<dyn MTLResource>>>;
    type AllocationMetadata = ();

    fn get_allocator_info(_bridge: &Self::BridgeHandle<'_>) -> Self::AllocatorInfo {
        ()
    }

    fn get_memory_pools(
        bridge: &Self::BridgeHandle<'_>,
        _info: &Self::AllocatorInfo,
        _config: &AllocatorConfig,
    ) -> Vec<MemoryPool<Self>> {
        let mut pools = Vec::new();
        for location in [
            MemoryLocation::GpuLocal,
            MemoryLocation::CpuToGpu,
            MemoryLocation::GpuToCpu,
        ] {
            let is_mappable = matches!(
                location,
                MemoryLocation::CpuToGpu | MemoryLocation::GpuToCpu
            );
            let is_device_local = matches!(location, MemoryLocation::GpuLocal);
            let config = PoolConfig {
                is_mappable,
                is_device_local,
            };

            let (cache_mode, storage_mode, resource_options) = location_to_info(location);

            let info = MetalPoolInfo {
                cache_mode,
                storage_mode,
                resource_options,
                is_buffer_pool: false,
                heap_residency: Mutex::new(ResidencySet::new(bridge).unwrap()),
                dedicated_residency_set: Mutex::new(ResidencySet::new(bridge).unwrap()),
            };
            let pool = MemoryPool::<Self>::new(config.clone(), info);
            pools.push(pool);

            let info = MetalPoolInfo {
                cache_mode,
                storage_mode,
                resource_options,
                is_buffer_pool: true,
                heap_residency: Mutex::new(ResidencySet::new(bridge).unwrap()),
                dedicated_residency_set: Mutex::new(ResidencySet::new(bridge).unwrap()),
            };
            let pool = MemoryPool::<Self>::new(config.clone(), info);
            pools.push(pool);
        }
        pools
    }

    unsafe fn create_buffer_object(
        _bridge: &Self::BridgeHandle<'_>,
        desc: &AllocationDesc<Self::BufferDesc<'_>>,
        allocation: &GpuAllocation,
        _allocator_info: &Self::AllocatorInfo,
        pool_info: &Self::PoolInfo,
        block_info: &Self::BlockInfo,
    ) -> Result<Self::BufferHandle, ()> {
        debug_assert!(pool_info.is_buffer_pool);
        unsafe {
            let memory = &block_info.memory.as_ref().unwrap();
            let offset = allocation.block_offset();

            // All options that can be provided for a resource are for controlling what kind of
            // memory it ends up in. This is inherited from the pool/heap so we don't ask the user
            // to provide their own options.
            let options = pool_info.resource_options;

            let buffer = memory
                .newBufferWithLength_options_offset(desc.desc, options, offset as usize)
                .ok_or(())?;
            Ok(buffer)
        }
    }

    unsafe fn destroy_buffer_object(
        _bridge: &Self::BridgeHandle<'_>,
        _allocator_info: &Self::AllocatorInfo,
        _pool_info: &Self::PoolInfo,
        buffer: Self::BufferHandle,
    ) {
        drop(buffer);
    }

    unsafe fn create_dedicated_buffer_object(
        bridge: &Self::BridgeHandle<'_>,
        desc: &AllocationDesc<Self::BufferDesc<'_>>,
        _memory_requirements: &MemoryRequirements,
        _allocation: &GpuAllocation,
        _allocator_info: &Self::AllocatorInfo,
        pool_info: &Self::PoolInfo,
    ) -> Result<(Self::DedicatedBlockInfo, Self::BufferHandle), ()> {
        debug_assert!(pool_info.is_buffer_pool);

        // All options that can be provided for a resource are for controlling what kind of
        // memory it ends up in. This is inherited from the pool/heap so we don't ask the user
        // to provide their own options.
        let options = pool_info.resource_options;

        let buffer = bridge
            .device
            .newBufferWithLength_options(desc.desc, options)
            .ok_or(())?;

        let allocation = ProtocolObject::from_retained(buffer.clone());
        pool_info
            .dedicated_residency_set
            .lock()
            .add_allocation(allocation.as_ref());

        Ok((Some(allocation), buffer))
    }

    unsafe fn create_texture_object(
        _bridge: &Self::BridgeHandle<'_>,
        desc: &AllocationDesc<Self::TextureDesc<'_>>,
        allocation: &GpuAllocation,
        _allocator_info: &Self::AllocatorInfo,
        pool_info: &Self::PoolInfo,
        block_info: &Self::BlockInfo,
    ) -> Result<Self::TextureHandle, ()> {
        debug_assert!(!pool_info.is_buffer_pool);
        unsafe {
            let memory = block_info.memory.as_ref().unwrap();
            let offset = allocation.block_offset();

            let image = memory
                .newTextureWithDescriptor_offset(&desc.desc, offset as usize)
                .ok_or(())?;
            Ok(image)
        }
    }

    unsafe fn destroy_texture_object(
        _bridge: &Self::BridgeHandle<'_>,
        _allocator_info: &Self::AllocatorInfo,
        _pool_info: &Self::PoolInfo,
        texture: Self::TextureHandle,
    ) {
        drop(texture);
    }

    unsafe fn create_dedicated_texture_object(
        bridge: &Self::BridgeHandle<'_>,
        desc: &AllocationDesc<Self::TextureDesc<'_>>,
        _memory_requirements: &MemoryRequirements,
        _allocation: &GpuAllocation,
        _allocator_info: &Self::AllocatorInfo,
        pool_info: &Self::PoolInfo,
    ) -> Result<(Self::DedicatedBlockInfo, Self::TextureHandle), ()> {
        debug_assert!(!pool_info.is_buffer_pool);
        let image = bridge
            .device
            .newTextureWithDescriptor(&desc.desc)
            .ok_or(())?;

        let allocation = ProtocolObject::from_retained(image.clone());
        pool_info
            .dedicated_residency_set
            .lock()
            .add_allocation(allocation.as_ref());

        Ok((Some(allocation), image))
    }

    unsafe fn create_block(
        bridge: &Self::BridgeHandle<'_>,
        _allocator_info: &Self::AllocatorInfo,
        pool_info: &Self::PoolInfo,
        size: u64,
    ) -> Option<Self::BlockInfo> {
        let descriptor = MTLHeapDescriptor::new();
        descriptor.setSize(size as usize);
        descriptor.setCpuCacheMode(pool_info.cache_mode);
        descriptor.setStorageMode(pool_info.storage_mode);
        descriptor.setResourceOptions(pool_info.resource_options);
        descriptor.setType(MTLHeapType::Placement);
        descriptor.setHazardTrackingMode(HAZARD_TRACKING_MODE);

        let memory = bridge.device.newHeapWithDescriptor(&descriptor)?;

        pool_info
            .heap_residency
            .lock()
            .add_allocation(memory.as_ref());

        Some(MetalBlockInfo {
            memory: Some(memory),
        })
    }

    unsafe fn destroy_block(
        _bridge: &Self::BridgeHandle<'_>,
        _allocator_info: &Self::AllocatorInfo,
        pool_info: &Self::PoolInfo,
        block: &mut Self::BlockInfo,
    ) {
        if let Some(memory) = block.memory.as_mut() {
            pool_info
                .heap_residency
                .lock()
                .remove_allocation(memory.as_ref());
        }
        block.memory = None;
    }

    unsafe fn destroy_dedicated_block(
        _bridge: &Self::BridgeHandle<'_>,
        _allocator_info: &Self::AllocatorInfo,
        pool_info: &Self::PoolInfo,
        block: &mut Self::DedicatedBlockInfo,
    ) {
        if let Some(memory) = block.as_mut() {
            pool_info
                .dedicated_residency_set
                .lock()
                .remove_allocation(memory.as_ref());
        }
        *block = None;
    }

    fn get_requirements_for_buffer(
        bridge: &Self::BridgeHandle<'_>,
        _info: &Self::AllocatorInfo,
        desc: &AllocationDesc<Self::BufferDesc<'_>>,
    ) -> Option<MemoryRequirements> {
        let pool_index =
            memory_location_and_resource_type_to_pool_index(desc.location, true) as u16;
        let options = location_to_info(desc.location).2;
        let size_align = bridge
            .device
            .heapBufferSizeAndAlignWithLength_options(desc.desc, options);

        let layout = GpuLayout::new(size_align.size as u64, size_align.align as u64).unwrap();

        Some(MemoryRequirements {
            pool_index,
            layout,
            dedicated_block_preferred: false,
            dedicated_block_required: false,
        })
    }

    fn get_requirements_for_texture(
        bridge: &Self::BridgeHandle<'_>,
        _info: &Self::AllocatorInfo,
        desc: &AllocationDesc<Self::TextureDesc<'_>>,
    ) -> Option<MemoryRequirements> {
        let pool_index =
            memory_location_and_resource_type_to_pool_index(desc.location, false) as u16;
        let (cache_mode, storage_mode, resource_options) = location_to_info(desc.location);

        // This _must_ be run to ensure the cache and storage behavior is correct.
        desc.desc.setCpuCacheMode(cache_mode);
        desc.desc.setStorageMode(storage_mode);
        desc.desc.setResourceOptions(resource_options);
        desc.desc.setHazardTrackingMode(HAZARD_TRACKING_MODE);

        let size_align = bridge
            .device
            .heapTextureSizeAndAlignWithDescriptor(&desc.desc);

        let layout = GpuLayout::new(size_align.size as u64, size_align.align as u64).unwrap();

        Some(MemoryRequirements {
            pool_index,
            layout,
            dedicated_block_preferred: false,
            dedicated_block_required: false,
        })
    }

    fn get_metadata_for_allocation(
        _bridge: &Self::BridgeHandle<'_>,
        _info: &Self::AllocatorInfo,
        _pool_info: &Self::PoolInfo,
        _block_info: &Self::BlockInfo,
        _allocation: &GpuAllocation,
    ) -> Self::AllocationMetadata {
        ()
    }

    fn get_metadata_for_dedicated_allocation(
        _bridge: &Self::BridgeHandle<'_>,
        _info: &Self::AllocatorInfo,
        _pool_info: &Self::PoolInfo,
        _block_info: &Self::DedicatedBlockInfo,
        _allocation: &GpuAllocation,
    ) -> Self::AllocationMetadata {
        ()
    }
}

const fn memory_location_and_resource_type_to_pool_index(
    memory_location: MemoryLocation,
    is_buffer: bool,
) -> usize {
    let resource_idx = if is_buffer { 1 } else { 0 };
    let heap_idx = match memory_location {
        MemoryLocation::GpuLocal => 0,
        MemoryLocation::CpuToGpu => 1,
        MemoryLocation::GpuToCpu => 2,
    };
    (2 * heap_idx) + resource_idx
}

const fn location_to_info(
    location: MemoryLocation,
) -> (MTLCPUCacheMode, MTLStorageMode, MTLResourceOptions) {
    let cache_mode = match location {
        MemoryLocation::GpuLocal => MTLCPUCacheMode::DefaultCache,
        MemoryLocation::CpuToGpu => MTLCPUCacheMode::WriteCombined,
        MemoryLocation::GpuToCpu => MTLCPUCacheMode::DefaultCache,
    };

    let storage_mode = match location {
        MemoryLocation::GpuLocal => MTLStorageMode::Private,
        MemoryLocation::CpuToGpu => MTLStorageMode::Shared,
        MemoryLocation::GpuToCpu => MTLStorageMode::Shared,
    };

    let resource_options = match location {
        MemoryLocation::GpuLocal => MTLResourceOptions::StorageModePrivate
            .union(MTLResourceOptions::CPUCacheModeDefaultCache)
            .union(HAZARD_TRACKING_BITS),
        MemoryLocation::CpuToGpu => MTLResourceOptions::StorageModeShared
            .union(MTLResourceOptions::CPUCacheModeWriteCombined)
            .union(HAZARD_TRACKING_BITS),
        MemoryLocation::GpuToCpu => MTLResourceOptions::StorageModeShared
            .union(MTLResourceOptions::CPUCacheModeDefaultCache)
            .union(HAZARD_TRACKING_BITS),
    };

    (cache_mode, storage_mode, resource_options)
}

const HAZARD_TRACKING_MODE: MTLHazardTrackingMode = MTLHazardTrackingMode::Untracked;
const HAZARD_TRACKING_BITS: MTLResourceOptions = MTLResourceOptions::HazardTrackingModeUntracked;

pub struct MetalPoolInfo {
    pub cache_mode: MTLCPUCacheMode,
    pub storage_mode: MTLStorageMode,
    pub resource_options: MTLResourceOptions,
    pub is_buffer_pool: bool,
    pub heap_residency: Mutex<ResidencySet>,
    pub dedicated_residency_set: Mutex<ResidencySet>,
}

pub struct MetalBlockInfo {
    memory: Option<Retained<ProtocolObject<dyn MTLHeap>>>,
}

// unsafe impl Send for MetalBlockInfo {}
