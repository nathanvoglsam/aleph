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

use crate::raw::windows::win32::direct3d12::{ID3D12CommandAllocator, ID3D12Device4};
use crate::raw::windows::{Abi, Interface};
use crate::{D3D12DeviceChild, D3D12Object, Device};

#[derive(Clone)]
#[repr(transparent)]
pub struct CommandAllocator(pub(crate) ID3D12CommandAllocator);

impl CommandAllocator {}

impl D3D12Object for CommandAllocator {
    unsafe fn set_name_raw(&self, name: &[u16]) -> raw::windows::Result<()> {
        self.0.SetName(name.as_ptr()).ok()
    }
}

impl D3D12DeviceChild for CommandAllocator {
    unsafe fn get_device(&self) -> raw::windows::Result<Device> {
        let mut device: Option<ID3D12Device4> = None;
        self.0
            .GetDevice(&ID3D12Device4::IID, device.set_abi())
            .and_some(device)
            .map(|v| Device(v))
    }
}
