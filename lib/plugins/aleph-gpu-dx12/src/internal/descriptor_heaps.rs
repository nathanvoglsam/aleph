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

use crate::internal::descriptor_heap::DescriptorHeap;
use aleph_windows::Win32::Graphics::Direct3D12::*;

/// Internal struct that caches the descriptor increment sizes needed for allocating space in
/// descriptor heaps.
pub struct DescriptorHeaps {
    cpu_heaps: [DescriptorHeap; 4],
    view_heap: DescriptorHeap,
    sampler_heap: DescriptorHeap,
}

impl DescriptorHeaps {
    pub fn new(device: &dx12::Device) -> aleph_windows::core::Result<Self> {
        // Construct the CPU side heaps for all 4 descriptor types
        let cpu_heaps = [
            DescriptorHeap::new(
                device.as_raw().into(),
                D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV,
                1024 * 256,
                false,
            )?,
            DescriptorHeap::new(
                device.as_raw().into(),
                D3D12_DESCRIPTOR_HEAP_TYPE_SAMPLER,
                2048,
                false,
            )?,
            DescriptorHeap::new(
                device.as_raw().into(),
                D3D12_DESCRIPTOR_HEAP_TYPE_RTV,
                512,
                false,
            )?,
            DescriptorHeap::new(
                device.as_raw().into(),
                D3D12_DESCRIPTOR_HEAP_TYPE_DSV,
                512,
                false,
            )?,
        ];

        // Construct the two GPU visible heaps
        let view_heap = DescriptorHeap::new(
            device.as_raw().into(),
            D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV,
            D3D12_MAX_SHADER_VISIBLE_DESCRIPTOR_HEAP_SIZE_TIER_1,
            true,
        )?;
        let sampler_heap = DescriptorHeap::new(
            device.as_raw().into(),
            D3D12_DESCRIPTOR_HEAP_TYPE_SAMPLER,
            D3D12_MAX_SHADER_VISIBLE_SAMPLER_HEAP_SIZE,
            true,
        )?;

        let out = Self {
            cpu_heaps,
            view_heap,
            sampler_heap,
        };
        Ok(out)
    }

    /// Returns a reference to the cpu side [DescriptorHeap] list. The array is indexed by
    /// [D3D12_DESCRIPTOR_HEAP_TYPE], where the value of the type enum is the index into the array
    /// for the heap for that type.
    pub const fn cpu_heaps(&self) -> &[DescriptorHeap; 4] {
        &self.cpu_heaps
    }

    /// Returns a reference to the cpu side [DescriptorHeap] for descriptors of type
    /// [D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV].
    pub const fn cpu_view_heap(&self) -> &DescriptorHeap {
        &self.cpu_heaps[D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV.0 as usize]
    }

    /// Returns a reference to the cpu side [DescriptorHeap] for descriptors of type
    /// [D3D12_DESCRIPTOR_HEAP_TYPE_SAMPLER].
    pub const fn cpu_sampler_heap(&self) -> &DescriptorHeap {
        &self.cpu_heaps[D3D12_DESCRIPTOR_HEAP_TYPE_SAMPLER.0 as usize]
    }

    /// Returns a reference to the cpu side [DescriptorHeap] for descriptors of type
    /// [D3D12_DESCRIPTOR_HEAP_TYPE_RTV].
    pub const fn cpu_rtv_heap(&self) -> &DescriptorHeap {
        &self.cpu_heaps[D3D12_DESCRIPTOR_HEAP_TYPE_RTV.0 as usize]
    }

    /// Returns a reference to the cpu side [DescriptorHeap] for descriptors of type
    /// [D3D12_DESCRIPTOR_HEAP_TYPE_DSV].
    pub const fn cpu_dsv_heap(&self) -> &DescriptorHeap {
        &self.cpu_heaps[D3D12_DESCRIPTOR_HEAP_TYPE_DSV.0 as usize]
    }

    /// Returns a reference to the gpu side [DescriptorHeap] for descriptors of type
    /// [D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV].
    pub const fn gpu_view_heap(&self) -> &DescriptorHeap {
        &self.view_heap
    }

    /// Returns a reference to the gpu side [DescriptorHeap] for descriptors of type
    /// [D3D12_DESCRIPTOR_HEAP_TYPE_SAMPLER].
    pub const fn gpu_sampler_heap(&self) -> &DescriptorHeap {
        &self.sampler_heap
    }
}
