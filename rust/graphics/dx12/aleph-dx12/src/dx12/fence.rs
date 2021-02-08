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

use crate::raw::windows::win32::direct3d12::{ID3D12Device4, ID3D12Fence, D3D12_FENCE_FLAGS};
use crate::raw::windows::{Abi, Interface};
use crate::{D3D12DeviceChild, D3D12Object, Device, Event};

pub struct FenceBuilder<'a> {
    pub(crate) device: &'a Device,
    pub(crate) initial_value: u64,
    pub(crate) flags: D3D12_FENCE_FLAGS,
}

impl<'a> FenceBuilder<'a> {
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

    pub unsafe fn build(self) -> raw::windows::Result<Fence> {
        let mut fence: Option<ID3D12Fence> = None;
        self.device
            .0
            .CreateFence(
                self.initial_value,
                self.flags,
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
    pub unsafe fn signal(&self, value: u64) -> raw::windows::Result<()> {
        self.0.Signal(value).ok()
    }

    pub unsafe fn set_event_on_completion(
        &self,
        value: u64,
        event: &Event,
    ) -> raw::windows::Result<()> {
        self.0.SetEventOnCompletion(value, event.0).ok()
    }
}

impl D3D12Object for Fence {
    unsafe fn set_name_raw(&self, name: &[u16]) -> raw::windows::Result<()> {
        self.0.SetName(name.as_ptr()).ok()
    }
}

impl D3D12DeviceChild for Fence {
    unsafe fn get_device(&self) -> raw::windows::Result<Device> {
        let mut device: Option<ID3D12Device4> = None;
        self.0
            .GetDevice(&ID3D12Device4::IID, device.set_abi())
            .and_some(device)
            .map(|v| Device(v))
    }
}
