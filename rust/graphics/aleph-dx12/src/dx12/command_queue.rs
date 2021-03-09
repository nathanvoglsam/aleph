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

use crate::{D3D12DeviceChild, D3D12Object, Device, Fence, SubmissionBuilder};
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, RwLock};
use windows_raw::win32::direct3d12::ID3D12CommandQueue;
use windows_raw::win32::system_services::PWSTR;

pub struct CommandQueueRecorder<'a>(pub(crate) std::sync::RwLockWriteGuard<'a, ID3D12CommandQueue>);

impl<'a> CommandQueueRecorder<'a> {
    pub unsafe fn signal(&mut self, fence: &Fence, value: u64) -> crate::Result<()> {
        self.0.Signal(&fence.0, value).ok()
    }

    pub unsafe fn execute_command_lists(&mut self, command_lists: &SubmissionBuilder) {
        let lists = command_lists.lists();
        self.0.ExecuteCommandLists(lists.0, lists.1);
    }

    pub fn as_raw(&self) -> &ID3D12CommandQueue {
        self.0.deref()
    }

    pub fn as_raw_mut(&mut self) -> &mut ID3D12CommandQueue {
        self.0.deref_mut()
    }
}

#[derive(Clone)]
#[repr(transparent)]
pub struct CommandQueue(pub(crate) Arc<RwLock<ID3D12CommandQueue>>);

impl CommandQueue {
    pub fn record(&self) -> CommandQueueRecorder {
        CommandQueueRecorder(self.get_raw_exclusive())
    }

    pub fn get_raw_shared(&self) -> std::sync::RwLockReadGuard<ID3D12CommandQueue> {
        self.0.read().unwrap()
    }

    pub fn get_raw_exclusive(&self) -> std::sync::RwLockWriteGuard<ID3D12CommandQueue> {
        self.0.write().unwrap()
    }
}

impl D3D12Object for CommandQueue {
    unsafe fn set_name_raw(&self, name: &[u16]) -> crate::Result<()> {
        self.get_raw_shared()
            .SetName(PWSTR(name.as_ptr() as *mut u16))
            .ok()
    }
}

impl D3D12DeviceChild for CommandQueue {
    unsafe fn get_device(&self) -> crate::Result<Device> {
        use windows_raw::{Abi, Interface};
        type D = windows_raw::win32::direct3d12::ID3D12Device4;
        let mut device: Option<D> = None;
        self.get_raw_shared()
            .GetDevice(&D::IID, device.set_abi())
            .and_some(device)
            .map(|v| Device(v))
    }
}