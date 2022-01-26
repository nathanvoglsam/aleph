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

use aleph_vulkan_core::erupt::vk1_0::DeviceSize;

///
/// VmaStatInfo
///
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct StatInfo {
    pub block_count: u32,
    pub allocation_count: u32,
    pub unused_range_count: u32,
    pub used_bytes: DeviceSize,
    pub unused_bytes: DeviceSize,
    pub allocation_size_min: DeviceSize,
    pub allocation_size_avg: DeviceSize,
    pub allocation_size_max: DeviceSize,
    pub unused_range_size_min: DeviceSize,
    pub unused_range_size_avg: DeviceSize,
    pub unused_range_size_max: DeviceSize,
}

///
/// VmaStats
///
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct Stats {
    pub memory_type: [StatInfo; 32usize],
    pub memory_heap: [StatInfo; 16usize],
    pub total: StatInfo,
}

///
/// VmaPoolStats
///
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct PoolStats {
    pub size: DeviceSize,
    pub unused_size: DeviceSize,
    pub allocation_count: usize,
    pub unused_range_count: usize,
    pub unused_range_size_max: DeviceSize,
    pub block_count: usize,
}
