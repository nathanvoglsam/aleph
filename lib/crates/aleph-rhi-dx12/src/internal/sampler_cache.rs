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

use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};

use aleph_rhi_api::SamplerDesc;
use parking_lot::RwLock;
use windows::core::CanInto;
use windows::utils::{CPUDescriptorHandle, GPUDescriptorHandle};
use windows::Win32::Graphics::Direct3D12::*;

use crate::internal::conv::{
    border_color_to_dx12, compare_op_to_dx12, sampler_address_mode_to_dx12, sampler_filters_to_dx12,
};
use crate::internal::sampler_cache_key::SamplerCacheKey;

/// This data structure owns and manages the GPU visible sampler heap. Samplers need special
/// treatment because of the extremely limited gpu visible heap size of 2048. We can't allocate them
/// like resource samplers and we can't create tables in the same way we do for resources.
///
/// Our solution for samplers is to disallow sampler arrays and map each sampler in a descriptor set
/// to a separate descriptor table in the root signature. That means each descriptor set will use
/// a root parameter for the resource table, and then N additional tables for each sampler inside
/// the descriptor set.
///
/// We trade update-after-bind support on samplers with this scheme (update_descriptor_sets doesn't
/// write any GPU visible memory) but gain the ability to use samplers like Vulkan without rapidly
/// exhausting sampler heap space. A worthwhile trade, as there's no other way to reconcile this
/// without full bindless.
///
pub struct SamplerCache {
    /// Device handle so we can create new blocks on demand
    device: ID3D12Device,

    /// The descriptor heap we're allocating into
    heap: ID3D12DescriptorHeap,

    /// The increment size for the descriptor type this heap stores
    descriptor_increment: u32,

    /// The descriptor handle for the start of the heap on the CPU
    start_cpu_handle: CPUDescriptorHandle,

    /// The descriptor handle for the start of the heap on the GPU
    start_gpu_handle: GPUDescriptorHandle,

    /// A cache that maps a sampler description to an index in the heap
    cache: RwLock<HashMap<SamplerCacheKey<'static>, u32>>,

    /// The mutable state wrapped in a mutex for safe multi-threaded access
    state: AtomicU32,
}

impl SamplerCache {
    const NUM_DESCRIPTORS: u32 = 2048;

    #[allow(unused)]
    pub fn new(device: &impl CanInto<ID3D12Device>) -> windows::core::Result<Self> {
        let device: ID3D12Device = device.can_clone_into();

        let heap = unsafe {
            let desc = D3D12_DESCRIPTOR_HEAP_DESC {
                Type: D3D12_DESCRIPTOR_HEAP_TYPE_SAMPLER,
                NumDescriptors: Self::NUM_DESCRIPTORS,
                Flags: D3D12_DESCRIPTOR_HEAP_FLAG_SHADER_VISIBLE,
                NodeMask: 0,
            };
            device.CreateDescriptorHeap::<ID3D12DescriptorHeap>(&desc)?
        };

        let descriptor_increment =
            unsafe { device.GetDescriptorHandleIncrementSize(D3D12_DESCRIPTOR_HEAP_TYPE_SAMPLER) };

        let start_cpu_handle = unsafe { heap.GetCPUDescriptorHandleForHeapStart() };
        let start_cpu_handle = CPUDescriptorHandle::try_from(start_cpu_handle).unwrap();
        let start_gpu_handle = unsafe { heap.GetGPUDescriptorHandleForHeapStart() };
        let start_gpu_handle = GPUDescriptorHandle::try_from(start_gpu_handle).unwrap();

        let out = Self {
            device,
            heap,
            descriptor_increment,
            start_cpu_handle,
            start_gpu_handle,
            cache: Default::default(),
            state: Default::default(),
        };

        Ok(out)
    }

    pub fn get(&self, desc: &SamplerDesc) -> Option<GPUDescriptorHandle> {
        assert!(desc.lod_bias.is_finite());
        assert!(desc.min_lod.is_finite());
        assert!(desc.max_lod.is_finite());

        let key = unsafe { SamplerCacheKey::from_desc(desc) };
        // Try and grab from the cache first. Speculate a hit is likely so we only grab the reader
        // lock. Cache hits are quite likely once an app has warmed the cache
        let read_only = self.cache.read();
        if let Some(i) = read_only.get(key).copied() {
            return Some(self.index_to_handles(i).1);
        }
        drop(read_only);

        // If we get a cache miss we need to insert a new entry into the cache. First thing we do
        // is allocate the handle. Everything up until this point is mostly lock free (except for
        // potentially waiting on another writer for the above read lock). This way we can early
        // exit if we've run out of heap space without locking the cache
        let next = self.next()?;

        // Insert the index into the cache keyed by the sampler description. We have a debug check
        // to ensure we don't clobber an old entry.
        let mut writeable = self.cache.write();
        let key = SamplerCacheKey::new(desc.clone().strip_name());
        let result = writeable.insert(key, next);
        debug_assert!(result.is_none());

        // Convert the index to descriptor handles
        let (cpu, gpu) = self.index_to_handles(next);

        // Initialize the descriptor from the given description
        unsafe {
            let desc = D3D12_SAMPLER_DESC {
                Filter: sampler_filters_to_dx12(
                    desc.min_filter,
                    desc.mag_filter,
                    desc.mip_filter,
                    desc.compare_op.is_some(),
                    desc.enable_anisotropy,
                ),
                AddressU: sampler_address_mode_to_dx12(desc.address_mode_u),
                AddressV: sampler_address_mode_to_dx12(desc.address_mode_v),
                AddressW: sampler_address_mode_to_dx12(desc.address_mode_w),
                MipLODBias: desc.lod_bias,
                MaxAnisotropy: desc.max_anisotropy,
                ComparisonFunc: desc
                    .compare_op
                    .map(compare_op_to_dx12)
                    .unwrap_or(D3D12_COMPARISON_FUNC(0)),
                BorderColor: border_color_to_dx12(desc.border_color),
                MinLOD: desc.min_lod,
                MaxLOD: desc.max_lod,
            };
            self.device.CreateSampler(&desc, cpu.into());
        }

        // Guarantee the exclusive lock is held while we write the sampler with 'CreateSampler'.
        // This prevents multiple threads from racing on each other writing to the sampler.
        //
        // In practice they can't race as no thread can ever be given the same index from 'next' due
        // to the atomic bump used, but just to be sure.
        drop(writeable);

        Some(gpu)
    }

    /// Returns the [ID3D12DescriptorHeap] that this object encapsulates
    #[allow(unused)]
    pub const fn heap(&self) -> &ID3D12DescriptorHeap {
        &self.heap
    }

    /// Get the next set of descriptor handles from the bump allocator. Returns `None` when the heap
    /// is exhausted.
    fn next(&self) -> Option<u32> {
        let allocated = self.state.fetch_add(1, Ordering::Release);
        if allocated > Self::NUM_DESCRIPTORS {
            None
        } else {
            Some(allocated)
        }
    }

    /// Converts an index into the sampler heap into cpu and gpu handles based on the stored
    /// descriptor increment.
    fn index_to_handles(&self, index: u32) -> (CPUDescriptorHandle, GPUDescriptorHandle) {
        debug_assert!(index < Self::NUM_DESCRIPTORS);

        let cpu = self
            .start_cpu_handle
            .add_increments(index as usize, self.descriptor_increment as usize);
        let gpu = self
            .start_gpu_handle
            .add_increments(index as u64, self.descriptor_increment as u64);
        (cpu, gpu)
    }
}
