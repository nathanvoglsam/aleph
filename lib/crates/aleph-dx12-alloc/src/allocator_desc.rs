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

use crate::AllocatorFlags;
use dx12::{dxgi, Device};

pub struct AllocatorDescBuilder {
    inner: AllocatorDesc,
}

impl AllocatorDescBuilder {
    pub fn new() -> Self {
        Self {
            inner: AllocatorDesc::default(),
        }
    }

    pub fn device(mut self, device: Device) -> Self {
        self.inner.device = Some(device);
        self
    }

    pub fn adapter(mut self, adapter: dxgi::Adapter) -> Self {
        self.inner.adapter = Some(adapter);
        self
    }

    pub fn preferred_block_size(mut self, preferred_block_size: u64) -> Self {
        self.inner.preferred_block_size = preferred_block_size;
        self
    }

    pub fn flags(mut self, flags: AllocatorFlags) -> Self {
        self.inner.flags |= flags;
        self
    }

    pub fn build(self) -> AllocatorDesc {
        self.inner
    }
}

impl Default for AllocatorDescBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[repr(C)]
pub struct AllocatorDesc {
    pub flags: AllocatorFlags,
    pub device: Option<Device>,
    pub preferred_block_size: u64,
    p_allocation_callbacks: *const u8, // D3D12MA_ALLOCATION_CALLBACKS
    pub adapter: Option<dxgi::Adapter>,
}

impl AllocatorDesc {
    pub fn builder() -> AllocatorDescBuilder {
        AllocatorDescBuilder::new()
    }
}

impl Default for AllocatorDesc {
    fn default() -> Self {
        Self {
            device: None,
            adapter: None,
            preferred_block_size: 0,
            p_allocation_callbacks: std::ptr::null(),
            flags: Default::default(),
        }
    }
}
