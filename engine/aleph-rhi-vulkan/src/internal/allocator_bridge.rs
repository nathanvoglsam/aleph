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

use aleph_gpu_allocator::{
    AllocationDesc, AllocatorConfig, GpuAllocation, GpuLayout, IApiBridge, MemoryLocation,
    MemoryPool, MemoryRequirements, PoolConfig,
};
use ash::vk;

use crate::device::Device;
use crate::internal::allocation_callbacks::GLOBAL_CALLBACKS;

pub struct VulkanAllocatorBridge;

impl<'a> IApiBridge for VulkanAllocatorBridge {
    type BridgeHandle<'b> = Device;
    type BufferHandle = vk::Buffer;
    type TextureHandle = vk::Image;
    type BufferDesc<'b> = vk::BufferCreateInfo<'b>;
    type TextureDesc<'b> = vk::ImageCreateInfo<'b>;
    type AllocatorInfo = VulkanAllocatorInfo;
    type PoolInfo = VulkanPoolInfo;
    type BlockInfo = VulkanBlockInfo;
    type DedicatedBlockInfo = VulkanBlockInfo;
    type AllocationMetadata = VulkanAllocationMetadata;

    fn get_allocator_info(bridge: &Self::BridgeHandle<'_>) -> Self::AllocatorInfo {
        let mut properties = vk::PhysicalDeviceMemoryProperties2::default();
        unsafe {
            bridge
                .context
                .instance
                .get_physical_device_memory_properties2(
                    bridge.adapter.physical_device,
                    &mut properties,
                );
        }

        fn filter_memory_types(
            types: &[vk::MemoryType],
            wanted_mask: vk::MemoryPropertyFlags,
        ) -> u32 {
            let mut bitset = 0;
            for memory_type in types {
                bitset <<= 1; // does nothing on first iteration, post shift would over-shift

                let type_is_superset = (memory_type.property_flags & wanted_mask) == wanted_mask;
                if type_is_superset {
                    bitset |= 1;
                }
            }
            bitset
        }

        // Prefilter our memory types for each 'MemoryLocation' variant. This allows us to cheaply
        // skip irrelevant memory types when looking for where to place an allocation.
        //
        // GPU local requires device local memory as a minimum
        let gpu_local_mask = vk::MemoryPropertyFlags::DEVICE_LOCAL;
        let gpu_local_types = filter_memory_types(
            properties.memory_properties.memory_types_as_slice(),
            gpu_local_mask,
        );

        // First try device local, host visible + uncached memory when going for CPU upload/staging.
        let cpu_to_gpu_t0_mask = vk::MemoryPropertyFlags::HOST_VISIBLE
            | vk::MemoryPropertyFlags::HOST_COHERENT
            | vk::MemoryPropertyFlags::DEVICE_LOCAL;
        let cpu_to_gpu_t0_types = filter_memory_types(
            properties.memory_properties.memory_types_as_slice(),
            cpu_to_gpu_t0_mask,
        );

        // Then try _host_ memory, with visible + uncached.
        let cpu_to_gpu_t1_mask =
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT;
        let cpu_to_gpu_t1_types = filter_memory_types(
            properties.memory_properties.memory_types_as_slice(),
            cpu_to_gpu_t1_mask,
        );

        // First try _cached_ host memory, as we're going to be reading this (readback)
        let gpu_to_cpu_t0_mask = vk::MemoryPropertyFlags::HOST_VISIBLE
            | vk::MemoryPropertyFlags::HOST_COHERENT
            | vk::MemoryPropertyFlags::HOST_CACHED;
        let gpu_to_cpu_t0_types = filter_memory_types(
            properties.memory_properties.memory_types_as_slice(),
            gpu_to_cpu_t0_mask,
        );

        // Otherwise fall back to staging memory. Slow but does work.
        let gpu_to_cpu_t1_mask = vk::MemoryPropertyFlags::HOST_VISIBLE
            | vk::MemoryPropertyFlags::HOST_COHERENT
            | vk::MemoryPropertyFlags::HOST_CACHED;
        let gpu_to_cpu_t1_types = filter_memory_types(
            properties.memory_properties.memory_types_as_slice(),
            gpu_to_cpu_t1_mask,
        );

        VulkanAllocatorInfo {
            alloc_callbacks: GLOBAL_CALLBACKS,
            memory_props: properties.memory_properties,
            gpu_local_types,
            cpu_to_gpu_t0_types,
            cpu_to_gpu_t1_types,
            gpu_to_cpu_t0_types,
            gpu_to_cpu_t1_types,
        }
    }

    fn get_memory_pools(
        _bridge: &Self::BridgeHandle<'_>,
        info: &Self::AllocatorInfo,
        _config: &AllocatorConfig,
    ) -> Vec<MemoryPool<Self>> {
        let mut pools = Vec::new();
        for (i, memory_type) in info.memory_props.memory_types_as_slice().iter().enumerate() {
            let config = PoolConfig {
                is_mappable: memory_type
                    .property_flags
                    .contains(vk::MemoryPropertyFlags::HOST_VISIBLE),
                is_device_local: memory_type
                    .property_flags
                    .contains(vk::MemoryPropertyFlags::DEVICE_LOCAL),
            };

            let info = VulkanPoolInfo {
                memory_type_index: i as u32,
                mappable: config.is_mappable,
                is_buffer_pool: false,
            };
            let pool = MemoryPool::<Self>::new(config.clone(), info);
            pools.push(pool);

            let info = VulkanPoolInfo {
                memory_type_index: i as u32,
                mappable: config.is_mappable,
                is_buffer_pool: true,
            };
            let pool = MemoryPool::<Self>::new(config, info);
            pools.push(pool);
        }
        pools
    }

    unsafe fn create_buffer_object(
        bridge: &Self::BridgeHandle<'_>,
        desc: &AllocationDesc<Self::BufferDesc<'_>>,
        allocation: &GpuAllocation,
        allocator_info: &Self::AllocatorInfo,
        pool_info: &Self::PoolInfo,
        block_info: &Self::BlockInfo,
    ) -> Result<Self::BufferHandle, ()> {
        debug_assert!(pool_info.is_buffer_pool);
        unsafe {
            let memory = block_info.memory;
            let offset = allocation.block_offset();

            let buffer = bridge
                .device
                .create_buffer(&desc.desc, Some(&allocator_info.alloc_callbacks))
                .map_err(|_| ())?;
            bridge
                .device
                .bind_buffer_memory(buffer, memory, offset.into())
                .map_err(|_| ())?;
            Ok(buffer)
        }
    }

    unsafe fn destroy_buffer_object(
        bridge: &Self::BridgeHandle<'_>,
        allocator_info: &Self::AllocatorInfo,
        buffer: Self::BufferHandle,
    ) {
        unsafe {
            bridge
                .device
                .destroy_buffer(buffer, Some(&allocator_info.alloc_callbacks));
        }
    }

    unsafe fn create_dedicated_buffer_object(
        bridge: &Self::BridgeHandle<'_>,
        desc: &AllocationDesc<Self::BufferDesc<'_>>,
        memory_requirements: &MemoryRequirements,
        allocation: &GpuAllocation,
        allocator_info: &Self::AllocatorInfo,
        pool_info: &Self::PoolInfo,
    ) -> Result<(Self::DedicatedBlockInfo, Self::BufferHandle), ()> {
        debug_assert!(pool_info.is_buffer_pool);
        unsafe {
            let buffer = bridge
                .device
                .create_buffer(&desc.desc, Some(&allocator_info.alloc_callbacks))
                .map_err(|_| ())?;

            let mut dedicated = vk::MemoryDedicatedAllocateInfo::default().buffer(buffer);
            let info = vk::MemoryAllocateInfo::default()
                .allocation_size(memory_requirements.layout.size())
                .memory_type_index(pool_info.memory_type_index)
                .push_next(&mut dedicated);
            let memory = bridge
                .device
                .allocate_memory(&info, Some(&allocator_info.alloc_callbacks))
                .map_err(|_| ())?;

            let mapped_address = if pool_info.mappable {
                let result = bridge
                    .device
                    .map_memory(memory, 0, vk::WHOLE_SIZE, vk::MemoryMapFlags::empty())
                    .map_err(|_| ())?;
                NonNull::new(result.cast::<u8>())
            } else {
                None
            };
            let block_info = VulkanBlockInfo {
                memory,
                mapped_address,
            };

            let offset = allocation.block_offset();
            bridge
                .device
                .bind_buffer_memory(buffer, memory, offset.into())
                .map_err(|_| ())?;

            Ok((block_info, buffer))
        }
    }

    unsafe fn create_texture_object(
        bridge: &Self::BridgeHandle<'_>,
        desc: &AllocationDesc<Self::TextureDesc<'_>>,
        allocation: &GpuAllocation,
        allocator_info: &Self::AllocatorInfo,
        pool_info: &Self::PoolInfo,
        block_info: &Self::BlockInfo,
    ) -> Result<Self::TextureHandle, ()> {
        debug_assert!(!pool_info.is_buffer_pool);
        unsafe {
            let memory = block_info.memory;
            let offset = allocation.block_offset();

            let image = bridge
                .device
                .create_image(&desc.desc, Some(&allocator_info.alloc_callbacks))
                .map_err(|_| ())?;
            bridge
                .device
                .bind_image_memory(image, memory, offset.into())
                .map_err(|_| ())?;
            Ok(image)
        }
    }

    unsafe fn destroy_texture_object(
        bridge: &Self::BridgeHandle<'_>,
        allocator_info: &Self::AllocatorInfo,
        texture: Self::TextureHandle,
    ) {
        unsafe {
            bridge
                .device
                .destroy_image(texture, Some(&allocator_info.alloc_callbacks));
        }
    }

    unsafe fn create_dedicated_texture_object(
        bridge: &Self::BridgeHandle<'_>,
        desc: &AllocationDesc<Self::TextureDesc<'_>>,
        memory_requirements: &MemoryRequirements,
        allocation: &GpuAllocation,
        allocator_info: &Self::AllocatorInfo,
        pool_info: &Self::PoolInfo,
    ) -> Result<(Self::DedicatedBlockInfo, Self::TextureHandle), ()> {
        debug_assert!(!pool_info.is_buffer_pool);
        unsafe {
            let image = bridge
                .device
                .create_image(&desc.desc, Some(&allocator_info.alloc_callbacks))
                .map_err(|_| ())?;

            let mut dedicated = vk::MemoryDedicatedAllocateInfo::default().image(image);
            let info = vk::MemoryAllocateInfo::default()
                .allocation_size(memory_requirements.layout.size())
                .memory_type_index(pool_info.memory_type_index)
                .push_next(&mut dedicated);
            let memory = bridge
                .device
                .allocate_memory(&info, Some(&allocator_info.alloc_callbacks))
                .map_err(|_| ())?;

            let mapped_address = if pool_info.mappable {
                let result = bridge
                    .device
                    .map_memory(memory, 0, vk::WHOLE_SIZE, vk::MemoryMapFlags::empty())
                    .map_err(|_| ())?;
                NonNull::new(result.cast::<u8>())
            } else {
                None
            };
            let block_info = VulkanBlockInfo {
                memory,
                mapped_address,
            };

            let offset = allocation.block_offset();
            bridge
                .device
                .bind_image_memory(image, memory, offset.into())
                .map_err(|_| ())?;

            Ok((block_info, image))
        }
    }

    unsafe fn create_block(
        bridge: &Self::BridgeHandle<'_>,
        allocator_info: &Self::AllocatorInfo,
        pool_info: &Self::PoolInfo,
        size: u64,
    ) -> Option<Self::BlockInfo> {
        unsafe {
            let info = vk::MemoryAllocateInfo::default()
                .allocation_size(size)
                .memory_type_index(pool_info.memory_type_index);
            let memory = bridge
                .device
                .allocate_memory(&info, Some(&allocator_info.alloc_callbacks))
                .ok()?;

            let mapped_address = if pool_info.mappable {
                let result = bridge
                    .device
                    .map_memory(memory, 0, vk::WHOLE_SIZE, vk::MemoryMapFlags::empty())
                    .ok()?;
                NonNull::new(result.cast::<u8>())
            } else {
                None
            };
            Some(VulkanBlockInfo {
                memory,
                mapped_address,
            })
        }
    }

    unsafe fn destroy_block(
        bridge: &Self::BridgeHandle<'_>,
        allocator_info: &Self::AllocatorInfo,
        block: &mut Self::BlockInfo,
    ) {
        unsafe {
            bridge
                .device
                .free_memory(block.memory, Some(&allocator_info.alloc_callbacks));
            block.memory = vk::DeviceMemory::null();
        };
    }

    unsafe fn destroy_dedicated_block(
        bridge: &Self::BridgeHandle<'_>,
        allocator_info: &Self::AllocatorInfo,
        block: &mut Self::DedicatedBlockInfo,
    ) {
        unsafe { Self::destroy_block(bridge, allocator_info, block) }
    }

    fn get_requirements_for_buffer(
        bridge: &Self::BridgeHandle<'_>,
        info: &Self::AllocatorInfo,
        desc: &AllocationDesc<Self::BufferDesc<'_>>,
    ) -> Option<MemoryRequirements> {
        unsafe {
            let vk_desc = vk::DeviceBufferMemoryRequirements::default().create_info(&desc.desc);

            let mut dedicated = vk::MemoryDedicatedRequirements::default();
            let mut requirements = vk::MemoryRequirements2::default().push_next(&mut dedicated);
            bridge
                .device
                .get_device_buffer_memory_requirements(&vk_desc, &mut requirements);

            // This is to remove 'dedicated' from the p_next chain and release the mutable borrow
            // so we can pass both requirements and dedicated into the next call
            let requirements = vk::MemoryRequirements2 {
                memory_requirements: requirements.memory_requirements,
                ..Default::default()
            };

            Self::get_requirements_for_resource(info, desc, &requirements, &dedicated, true)
        }
    }

    fn get_requirements_for_texture(
        bridge: &Self::BridgeHandle<'_>,
        info: &Self::AllocatorInfo,
        desc: &AllocationDesc<Self::TextureDesc<'_>>,
    ) -> Option<MemoryRequirements> {
        unsafe {
            let vk_desc = vk::DeviceImageMemoryRequirements::default().create_info(&desc.desc);

            let mut dedicated = vk::MemoryDedicatedRequirements::default();
            let mut requirements = vk::MemoryRequirements2::default().push_next(&mut dedicated);
            bridge
                .device
                .get_device_image_memory_requirements(&vk_desc, &mut requirements);

            // This is to remove 'dedicated' from the p_next chain and release the mutable borrow
            // so we can pass both requirements and dedicated into the next call
            let requirements = vk::MemoryRequirements2 {
                memory_requirements: requirements.memory_requirements,
                ..Default::default()
            };

            Self::get_requirements_for_resource(info, desc, &requirements, &dedicated, false)
        }
    }

    fn get_metadata_for_allocation(
        _bridge: &Self::BridgeHandle<'_>,
        _info: &Self::AllocatorInfo,
        _pool_info: &Self::PoolInfo,
        block_info: &Self::BlockInfo,
        allocation: &GpuAllocation,
    ) -> Self::AllocationMetadata {
        unsafe {
            let base = block_info.mapped_address;
            let base = base.map(|v| v.add(allocation.block_offset() as usize));

            VulkanAllocationMetadata {
                memory: block_info.memory,
                mapped_address: base,
            }
        }
    }

    fn get_metadata_for_dedicated_allocation(
        bridge: &Self::BridgeHandle<'_>,
        info: &Self::AllocatorInfo,
        pool_info: &Self::PoolInfo,
        block_info: &Self::BlockInfo,
        allocation: &GpuAllocation,
    ) -> Self::AllocationMetadata {
        Self::get_metadata_for_allocation(bridge, info, pool_info, block_info, allocation)
    }
}

impl VulkanAllocatorBridge {
    fn get_requirements_for_resource<T>(
        info: &VulkanAllocatorInfo,
        desc: &AllocationDesc<T>,
        requirements: &vk::MemoryRequirements2,
        dedicated: &vk::MemoryDedicatedRequirements,
        is_buffer: bool,
    ) -> Option<MemoryRequirements> {
        let requirements = requirements.memory_requirements;
        let layout = GpuLayout::new(requirements.size, requirements.alignment)?;

        // The first tier of memory types for each 'memory location' we try and use. If we can't
        // use any of the types in these subsets then we fall back to the expanded set in t1.
        //
        // GPU memory doesn't have a t1 type because there must _always_ be a DEVICE_LOCAL type.
        //
        // t0 should be the fastest for the intended usage.
        let t0_memory_types = match desc.location {
            MemoryLocation::GpuLocal => info.gpu_local_types,
            MemoryLocation::CpuToGpu => info.cpu_to_gpu_t0_types,
            MemoryLocation::GpuToCpu => info.gpu_to_cpu_t0_types,
        };

        // The second tier of memory types for each 'memory location'. These may be slower than the
        // t0 types, but still _can_ be used. For devices that don't support the t0 types then we
        // fall back to the t1 types.
        //
        // If neither the t0 nor t1 type is viable for the use case then we just give up as we can't
        // find a memory type valid for the usage being asked for.
        let t1_memory_types = match desc.location {
            MemoryLocation::GpuLocal => info.gpu_local_types,
            MemoryLocation::CpuToGpu => info.cpu_to_gpu_t1_types,
            MemoryLocation::GpuToCpu => info.gpu_to_cpu_t1_types,
        };

        // We perform a two stage search. First we check the t0 memory types, which we expect to be
        // faster. If we can't find a usable t0 type we try a t1 memory type. If we can't find a
        // type for either than we're hosed, just return 'None'.
        let found_type;
        'found: {
            let required_bits = requirements.memory_type_bits;
            if let Some(v) = find_first_memory_type_in_subset(required_bits, t0_memory_types) {
                found_type = v;
                break 'found;
            }
            if let Some(v) = find_first_memory_type_in_subset(required_bits, t1_memory_types) {
                found_type = v;
                break 'found;
            }
            return None;
        }

        // We split buffers and textures into separate pools to avoid buffer/image granularity
        // issues. We interleave the pools so every even pool is for textures and every odd pool is
        // for buffers.
        //
        // Can't overflow u32 as 'find_first_memory_type_in_subset' can't return a number > 32.
        let buffer_offset = if is_buffer { 1 } else { 0 };
        let found_type = (found_type * 2) + buffer_offset;

        // Shouldn't ever fail, but I really want to make this code panic proof.
        let pool_index = u16::try_from(found_type).ok()?;

        Some(MemoryRequirements {
            pool_index,
            layout,
            dedicated_block_preferred: dedicated.prefers_dedicated_allocation != 0,
            dedicated_block_required: dedicated.requires_dedicated_allocation != 0,
        })
    }
}

fn find_first_memory_type_in_subset(required_types: u32, search_set: u32) -> Option<u32> {
    // Filter out all types not in the search set.
    let search_types = required_types & search_set;

    // Early exit if there are no common types in the required types and search set
    if search_types == 0 {
        return None;
    }

    // Constrain our search to just the region where bits are set in 'search types'. This will
    // reduce the number of iterations we have to do by trimming all leading and trailing zeroes
    // from the search.
    let first_type = search_types.trailing_zeros();
    let last_type = 32u32.saturating_sub(search_types.leading_zeros());
    for i in first_type..last_type {
        let type_bit = 1 << i;
        let is_allowed = (search_types & type_bit) != 0;
        if is_allowed {
            return Some(i);
        }
    }
    None
}

pub struct VulkanAllocatorInfo {
    alloc_callbacks: vk::AllocationCallbacks<'static>,
    memory_props: vk::PhysicalDeviceMemoryProperties,
    gpu_local_types: u32,
    cpu_to_gpu_t0_types: u32,
    cpu_to_gpu_t1_types: u32,
    gpu_to_cpu_t0_types: u32,
    gpu_to_cpu_t1_types: u32,
}

pub struct VulkanPoolInfo {
    memory_type_index: u32,
    mappable: bool,
    is_buffer_pool: bool,
}

pub struct VulkanBlockInfo {
    memory: vk::DeviceMemory,
    mapped_address: Option<NonNull<u8>>,
}

unsafe impl Send for VulkanBlockInfo {}

pub struct VulkanAllocationMetadata {
    pub memory: vk::DeviceMemory,
    pub mapped_address: Option<NonNull<u8>>,
}
