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

use crate::alloc::AllocatorBuilder;
use crate::raw::windows::win32::direct3d12::{
    ID3D12CommandAllocator, ID3D12CommandList, ID3D12Device4, ID3D12GraphicsCommandList,
    ID3D12Object, PFN_D3D12_CREATE_DEVICE,
};
use crate::raw::windows::{Abi, Interface};
use crate::utils::DynamicLoadCell;
use crate::{
    CommandAllocator, CommandList, CommandListType, CommandQueueBuilder, DXGIAdapter, FeatureLevel,
    FenceBuilder, GraphicsCommandList,
};
use utf16_lit::utf16_null;

pub static CREATE_FN: DynamicLoadCell<PFN_D3D12_CREATE_DEVICE> =
    DynamicLoadCell::new(&utf16_null!("d3d12.dll"), "D3D12CreateDevice\0");

#[derive(Clone)]
#[repr(transparent)]
pub struct Device(pub(crate) ID3D12Device4);

impl Device {
    pub fn new(
        adapter: Option<&DXGIAdapter>,
        minimum_feature_level: FeatureLevel,
    ) -> raw::windows::Result<Device> {
        unsafe {
            let create_fn = *CREATE_FN.get().expect("Failed to load d3d12.dll");
            let mut device: Option<ID3D12Device4> = None;
            create_fn(
                adapter.map(|v| (&v.0).into()),
                minimum_feature_level.into(),
                &ID3D12Device4::IID,
                device.set_abi(),
            )
            .and_some(device)
            .map(|v| Self(v))
        }
    }

    pub fn command_queue_builder<'a>(
        &'a self,
        queue_type: CommandListType,
    ) -> CommandQueueBuilder<'a> {
        CommandQueueBuilder::<'a> {
            device: self,
            priority: 0,
            queue_type,
            flags: Default::default(),
        }
    }

    pub fn fence_builder<'a>(&'a self) -> FenceBuilder<'a> {
        FenceBuilder::<'a> {
            device: self,
            initial_value: 0,
            flags: Default::default(),
        }
    }

    pub fn create_command_allocator(
        &self,
        list_type: CommandListType,
    ) -> raw::windows::Result<CommandAllocator> {
        unsafe {
            let mut out: Option<ID3D12CommandAllocator> = None;
            self.0
                .CreateCommandAllocator(
                    list_type.into(),
                    &ID3D12CommandAllocator::IID,
                    out.set_abi(),
                )
                .and_some(out)
                .map(|v| CommandAllocator(v))
        }
    }

    pub fn create_command_list(
        &self,
        list_type: CommandListType,
    ) -> raw::windows::Result<CommandList> {
        unsafe {
            let mut out: Option<ID3D12CommandList> = None;
            self.0
                .CreateCommandList1(
                    0,
                    list_type.into(),
                    Default::default(),
                    &ID3D12CommandList::IID,
                    out.set_abi(),
                )
                .and_some(out)
                .map(|v| CommandList(v))
        }
    }

    pub fn create_graphics_command_list(
        &self,
        list_type: CommandListType,
    ) -> raw::windows::Result<GraphicsCommandList> {
        unsafe {
            let mut out: Option<ID3D12GraphicsCommandList> = None;
            self.0
                .CreateCommandList1(
                    0,
                    list_type.into(),
                    Default::default(),
                    &ID3D12GraphicsCommandList::IID,
                    out.set_abi(),
                )
                .and_some(out)
                .map(|v| GraphicsCommandList(v))
        }
    }

    pub fn create_allocator_builder<'a, 'b>(
        &'a self,
        adapter: &'b DXGIAdapter,
        preferred_block_size: u64,
    ) -> AllocatorBuilder<'a, 'b> {
        AllocatorBuilder {
            device: &self,
            adapter,
            flags: alloc_raw::AllocatorFlags::NONE,
            preferred_block_size,
        }
    }
}

impl Into<ID3D12Object> for Device {
    fn into(self) -> ID3D12Object {
        self.0.into()
    }
}
