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
use windows::Win32::Graphics::Direct3D12::*;
use windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT;

use crate::device::Device;

const HEAP_TYPES: [D3D12_HEAP_TYPE; 3] = [
    D3D12_HEAP_TYPE_DEFAULT,
    D3D12_HEAP_TYPE_UPLOAD,
    D3D12_HEAP_TYPE_READBACK,
];
const RESOURCE_TYPES: [D3D12_HEAP_FLAGS; 3] = [
    D3D12_HEAP_FLAG_ALLOW_ONLY_BUFFERS,
    D3D12_HEAP_FLAG_ALLOW_ONLY_NON_RT_DS_TEXTURES,
    D3D12_HEAP_FLAG_ALLOW_ONLY_RT_DS_TEXTURES,
];

pub struct D3D12AllocatorBridge;

impl<'a> IApiBridge for D3D12AllocatorBridge {
    type BridgeHandle<'b> = Device;
    type BufferHandle = ID3D12Resource;
    type TextureHandle = ID3D12Resource;
    type BufferDesc<'b> = ExtendedResourceDesc<'b>;
    type TextureDesc<'b> = ExtendedResourceDesc<'b>;
    type AllocatorInfo = ();
    type PoolInfo = D3D12PoolInfo;
    type BlockInfo = D3D12BlockInfo;
    type DedicatedBlockInfo = ();
    type AllocationMetadata = ();

    fn get_allocator_info(_bridge: &Self::BridgeHandle<'_>) -> Self::AllocatorInfo {
        ()
    }

    fn get_memory_pools(
        _bridge: &Self::BridgeHandle<'_>,
        _info: &Self::AllocatorInfo,
        _config: &AllocatorConfig,
    ) -> Vec<MemoryPool<Self>> {
        let mut pools = Vec::new();
        for heap_type in HEAP_TYPES {
            for resource_type in RESOURCE_TYPES {
                let config = PoolConfig {
                    is_mappable: heap_type == D3D12_HEAP_TYPE_UPLOAD
                        || heap_type == D3D12_HEAP_TYPE_READBACK,
                    is_device_local: heap_type == D3D12_HEAP_TYPE_DEFAULT,
                };

                let mut heap_flags = resource_type | D3D12_HEAP_FLAG_CREATE_NOT_ZEROED;
                if heap_type == D3D12_HEAP_TYPE_DEFAULT {
                    heap_flags |= D3D12_HEAP_FLAG_ALLOW_SHADER_ATOMICS;
                }

                let info = D3D12PoolInfo {
                    heap_type,
                    heap_flags,
                    resource_type,
                };

                let pool = MemoryPool::<Self>::new(config, info);
                pools.push(pool);
            }
        }
        pools
    }

    unsafe fn create_buffer_object(
        bridge: &Self::BridgeHandle<'_>,
        desc: &AllocationDesc<Self::BufferDesc<'_>>,
        allocation: &GpuAllocation,
        _allocator_info: &Self::AllocatorInfo,
        pool_info: &Self::PoolInfo,
        block_info: &Self::BlockInfo,
    ) -> Result<Self::BufferHandle, ()> {
        debug_assert!(pool_info.resource_type == D3D12_HEAP_FLAG_ALLOW_ONLY_BUFFERS);
        unsafe {
            let mut out_resource: Option<ID3D12Resource> = None;
            bridge
                .device
                .CreatePlacedResource2(
                    block_info.heap.as_ref().unwrap(),
                    allocation.block_offset() as u64,
                    &desc.desc.desc,
                    desc.desc.initial_layout,
                    desc.desc
                        .optimized_clear_value
                        .map(|v| v as *const D3D12_CLEAR_VALUE),
                    desc.desc.pcastableformats,
                    &mut out_resource,
                )
                .map_err(|v| log::error!("Platform Error: {:#?}", v))?;

            let resource = out_resource.ok_or(())?;

            Ok(resource)
        }
    }

    unsafe fn destroy_buffer_object(
        _bridge: &Self::BridgeHandle<'_>,
        _allocator_info: &Self::AllocatorInfo,
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
        unsafe {
            let heap_properties = D3D12_HEAP_PROPERTIES {
                Type: pool_info.heap_type,
                ..Default::default()
            };

            let mut out_resource: Option<ID3D12Resource> = None;
            bridge
                .device
                .CreateCommittedResource3(
                    &heap_properties,
                    Default::default(),
                    &desc.desc.desc,
                    desc.desc.initial_layout,
                    desc.desc
                        .optimized_clear_value
                        .map(|v| v as *const D3D12_CLEAR_VALUE),
                    None,
                    desc.desc.pcastableformats,
                    &mut out_resource,
                )
                .map_err(|v| log::error!("Platform Error: {:#?}", v))?;
            let resource = out_resource.ok_or(())?;

            Ok(((), resource))
        }
    }

    unsafe fn create_texture_object(
        bridge: &Self::BridgeHandle<'_>,
        desc: &AllocationDesc<Self::TextureDesc<'_>>,
        allocation: &GpuAllocation,
        _allocator_info: &Self::AllocatorInfo,
        pool_info: &Self::PoolInfo,
        block_info: &Self::BlockInfo,
    ) -> Result<Self::TextureHandle, ()> {
        debug_assert!(pool_info.resource_type != D3D12_HEAP_FLAG_ALLOW_ONLY_BUFFERS);
        unsafe {
            let mut out_resource: Option<ID3D12Resource> = None;
            bridge
                .device
                .CreatePlacedResource2(
                    block_info.heap.as_ref().unwrap(),
                    allocation.block_offset() as u64,
                    &desc.desc.desc,
                    desc.desc.initial_layout,
                    desc.desc
                        .optimized_clear_value
                        .map(|v| v as *const D3D12_CLEAR_VALUE),
                    desc.desc.pcastableformats,
                    &mut out_resource,
                )
                .map_err(|v| log::error!("Platform Error: {:#?}", v))?;
            let resource = out_resource.ok_or(())?;

            Ok(resource)
        }
    }

    unsafe fn destroy_texture_object(
        _bridge: &Self::BridgeHandle<'_>,
        _allocator_info: &Self::AllocatorInfo,
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
        unsafe {
            let heap_properties = D3D12_HEAP_PROPERTIES {
                Type: pool_info.heap_type,
                ..Default::default()
            };

            let mut out_resource: Option<ID3D12Resource> = None;
            bridge
                .device
                .CreateCommittedResource3(
                    &heap_properties,
                    Default::default(),
                    &desc.desc.desc,
                    desc.desc.initial_layout,
                    desc.desc
                        .optimized_clear_value
                        .map(|v| v as *const D3D12_CLEAR_VALUE),
                    None,
                    desc.desc.pcastableformats,
                    &mut out_resource,
                )
                .map_err(|v| log::error!("Platform Error: {:#?}", v))?;
            let resource = out_resource.ok_or(())?;

            Ok(((), resource))
        }
    }

    unsafe fn create_block(
        bridge: &Self::BridgeHandle<'_>,
        _allocator_info: &Self::AllocatorInfo,
        pool_info: &Self::PoolInfo,
        size: u64,
    ) -> Option<Self::BlockInfo> {
        unsafe {
            // Only need MSAA alignment for heaps that can contain render targets
            let alignment = if pool_info.resource_type == D3D12_HEAP_FLAG_ALLOW_ONLY_RT_DS_TEXTURES
            {
                D3D12_DEFAULT_MSAA_RESOURCE_PLACEMENT_ALIGNMENT as u64
            } else {
                D3D12_DEFAULT_RESOURCE_PLACEMENT_ALIGNMENT as u64
            };

            let heap_desc = D3D12_HEAP_DESC {
                SizeInBytes: size,
                Properties: D3D12_HEAP_PROPERTIES {
                    Type: pool_info.heap_type,
                    ..Default::default()
                },
                Alignment: alignment,
                Flags: pool_info.heap_flags,
            };

            let mut out_heap: Option<ID3D12Heap> = None;
            bridge.device.CreateHeap(&heap_desc, &mut out_heap).ok()?;
            let heap = out_heap?;

            Some(D3D12BlockInfo { heap: Some(heap) })
        }
    }

    unsafe fn destroy_block(
        _bridge: &Self::BridgeHandle<'_>,
        _allocator_info: &Self::AllocatorInfo,
        block: &mut Self::BlockInfo,
    ) {
        let _sink = block.heap.take();
    }

    unsafe fn destroy_dedicated_block(
        _bridge: &Self::BridgeHandle<'_>,
        _allocator_info: &Self::AllocatorInfo,
        _block: &mut Self::DedicatedBlockInfo,
    ) {
        // Intentional no-op
    }

    fn get_requirements_for_buffer(
        bridge: &Self::BridgeHandle<'_>,
        _info: &Self::AllocatorInfo,
        desc: &AllocationDesc<Self::BufferDesc<'_>>,
    ) -> Option<MemoryRequirements> {
        let pool_index = memory_location_and_resource_type_to_pool_index(
            desc.location,
            D3D12_HEAP_FLAG_ALLOW_ONLY_BUFFERS,
        );
        let pool_index: u16 = pool_index.try_into().ok()?;

        let info = unsafe {
            bridge
                .device
                .GetResourceAllocationInfo2(0, 1, &desc.desc.desc, None)
        };
        if info.SizeInBytes == u64::MAX {
            return None;
        }
        Some(MemoryRequirements {
            pool_index,
            layout: GpuLayout::new(info.SizeInBytes, info.Alignment)?,
            dedicated_block_preferred: false,
            dedicated_block_required: false,
        })
    }

    fn get_requirements_for_texture(
        bridge: &Self::BridgeHandle<'_>,
        _info: &Self::AllocatorInfo,
        desc: &AllocationDesc<Self::TextureDesc<'_>>,
    ) -> Option<MemoryRequirements> {
        // Render targets get their own pool.
        let is_rt = desc
            .desc
            .desc
            .Flags
            .contains(D3D12_RESOURCE_FLAG_ALLOW_RENDER_TARGET);
        let is_ds = desc
            .desc
            .desc
            .Flags
            .contains(D3D12_RESOURCE_FLAG_ALLOW_DEPTH_STENCIL);
        let resource_type = if is_rt || is_ds {
            D3D12_HEAP_FLAG_ALLOW_ONLY_RT_DS_TEXTURES
        } else {
            D3D12_HEAP_FLAG_ALLOW_ONLY_NON_RT_DS_TEXTURES
        };

        let pool_index =
            memory_location_and_resource_type_to_pool_index(desc.location, resource_type);
        let pool_index: u16 = pool_index.try_into().ok()?;

        let info = unsafe {
            bridge
                .device
                .GetResourceAllocationInfo2(0, 1, &desc.desc.desc, None)
        };
        if info.SizeInBytes == u64::MAX {
            return None;
        }
        Some(MemoryRequirements {
            pool_index,
            layout: GpuLayout::new(info.SizeInBytes, info.Alignment)?,
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
    resource_type: D3D12_HEAP_FLAGS,
) -> usize {
    let resource_idx = match resource_type {
        D3D12_HEAP_FLAG_ALLOW_ONLY_BUFFERS => 0,
        D3D12_HEAP_FLAG_ALLOW_ONLY_NON_RT_DS_TEXTURES => 1,
        D3D12_HEAP_FLAG_ALLOW_ONLY_RT_DS_TEXTURES => 2,
        _ => unimplemented!(),
    };
    let heap_idx = match memory_location {
        MemoryLocation::GpuLocal => 0,
        MemoryLocation::CpuToGpu => 1,
        MemoryLocation::GpuToCpu => 2,
    };
    (HEAP_TYPES.len() * heap_idx) + resource_idx
}

pub struct ExtendedResourceDesc<'a> {
    pub desc: D3D12_RESOURCE_DESC1,
    pub initial_layout: D3D12_BARRIER_LAYOUT,
    pub optimized_clear_value: Option<&'a D3D12_CLEAR_VALUE>,
    pub pcastableformats: Option<&'a [DXGI_FORMAT]>,
}

pub struct D3D12PoolInfo {
    heap_type: D3D12_HEAP_TYPE,
    heap_flags: D3D12_HEAP_FLAGS,
    resource_type: D3D12_HEAP_FLAGS,
}

pub struct D3D12BlockInfo {
    heap: Option<ID3D12Heap>,
}

// unsafe impl Send for D3D12BlockInfo {}
