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

use crate::raw::windows::win32::direct3d12::{ID3D12Fence, D3D12_FENCE_FLAGS};
use crate::raw::windows::{Abi, Interface};
use crate::{D3D12Object, Device};

pub struct FenceBuilder {
    initial_value: u64,
    flags: D3D12_FENCE_FLAGS,
}

impl FenceBuilder {
    pub fn new() -> Self {
        Self {
            initial_value: 0,
            flags: D3D12_FENCE_FLAGS::D3D12_FENCE_FLAG_NONE,
        }
    }

    pub fn shared(mut self) -> Self {
        self.flags.0 |= D3D12_FENCE_FLAGS::D3D12_FENCE_FLAG_SHARED.0;
        self
    }

    pub fn shared_cross_adapter(mut self) -> Self {
        self.flags.0 |= D3D12_FENCE_FLAGS::D3D12_FENCE_FLAG_SHARED_CROSS_ADAPTER.0;
        self
    }

    pub fn non_monitored(mut self) -> Self {
        self.flags.0 |= D3D12_FENCE_FLAGS::D3D12_FENCE_FLAG_NON_MONITORED.0;
        self
    }

    pub fn initial_value(mut self, initial_value: u64) -> Self {
        self.initial_value = initial_value;
        self
    }

    pub unsafe fn build(self, device: &Device) -> raw::windows::Result<Fence> {
        let mut fence: Option<ID3D12Fence> = None;
        device
            .raw()
            .CreateFence(
                self.initial_value,
                D3D12_FENCE_FLAGS::D3D12_FENCE_FLAG_NONE,
                &ID3D12Fence::IID,
                fence.set_abi(),
            )
            .and_some(fence)
            .map(|v| Fence(v))
    }
}

#[derive(Clone)]
#[repr(transparent)]
pub struct Fence(pub(crate) ID3D12Fence);

impl Fence {
    pub fn builder() -> FenceBuilder {
        FenceBuilder::new()
    }

    pub fn raw(&self) -> &ID3D12Fence {
        &self.0
    }
}

impl D3D12Object for Fence {
    unsafe fn set_name_raw(&self, name: &[u16]) -> raw::windows::Result<()> {
        self.0.SetName(name.as_ptr()).ok()
    }
}
