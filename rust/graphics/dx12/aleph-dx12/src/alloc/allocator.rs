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

use crate::alloc::{Allocation, AllocationDesc, AllocatorFlags};
use crate::dx12::clear_value::D3D12_CLEAR_VALUE;
use crate::{dxgi, raw, ClearValue, Device, Resource, ResourceDesc, ResourceStates};
use alloc_raw::D3D12MA_ALLOCATOR_DESC;
use raw::windows::win32::direct3d12::ID3D12Resource;
use raw::windows::{Abi, Interface};
use std::ffi::c_void;
use std::mem::transmute_copy;
use std::ptr::NonNull;

pub struct AllocatorDescBuilder<'a> {
    inner: AllocatorDesc<'a>,
}

impl<'a> AllocatorDescBuilder<'a> {
    pub fn new() -> Self {
        Self {
            inner: AllocatorDesc::default(),
        }
    }

    pub fn device(mut self, device: &'a Device) -> Self {
        self.inner.device = Some(device);
        self
    }

    pub fn adapter(mut self, adapter: &'a dxgi::Adapter) -> Self {
        self.inner.adapter = Some(adapter);
        self
    }

    pub fn preferred_block_size(mut self, preferred_block_size: u64) -> Self {
        self.inner.preferred_block_size = preferred_block_size;
        self
    }

    pub fn flag(mut self, flag: AllocatorFlags) -> Self {
        self.inner.flags |= flag;
        self
    }

    pub fn build(self) -> AllocatorDesc<'a> {
        self.inner
    }
}

pub struct AllocatorDesc<'a> {
    pub device: Option<&'a Device>,
    pub adapter: Option<&'a dxgi::Adapter>,
    pub preferred_block_size: u64,
    pub flags: AllocatorFlags,
    // pub p_allocation_callbacks: *const D3D12MA_ALLOCATION_CALLBACKS,
}

impl<'a> AllocatorDesc<'a> {
    pub fn builder() -> AllocatorDescBuilder<'a> {
        AllocatorDescBuilder::new()
    }
}

impl<'a> Default for AllocatorDesc<'a> {
    fn default() -> Self {
        Self {
            device: None,
            adapter: None,
            preferred_block_size: 0,
            flags: Default::default(),
        }
    }
}

impl<'a> Into<D3D12MA_ALLOCATOR_DESC> for &AllocatorDesc<'a> {
    fn into(self) -> D3D12MA_ALLOCATOR_DESC {
        unsafe {
            D3D12MA_ALLOCATOR_DESC {
                flags: self.flags,
                p_device: transmute_copy(&self.device.unwrap().0),
                preferred_block_size: self.preferred_block_size,
                p_allocation_callbacks: std::ptr::null(),
                p_adapter: transmute_copy(&self.adapter.unwrap().0),
            }
        }
    }
}

#[repr(transparent)]
pub struct Allocator(NonNull<c_void>);

impl Allocator {
    pub fn new(allocator_desc: &AllocatorDesc) -> raw::windows::Result<Self> {
        if allocator_desc.flags & AllocatorFlags::SINGLE_THREADED != AllocatorFlags::NONE {
            panic!("AllocatorFlags::SINGLE_THREADED not supported by rust bindings")
        }

        unsafe {
            let desc = allocator_desc.into();
            let mut out = std::ptr::null_mut();
            alloc_raw::D3D12MA_Allocator_CreateAllocator(&desc, &mut out)
                .ok()
                .map(|_| {
                    let out = Allocator(NonNull::new(out).unwrap());
                    out
                })
        }
    }

    pub fn create_resource(
        &self,
        alloc_desc: &AllocationDesc,
        resource_desc: &ResourceDesc,
        initial_resource_state: ResourceStates,
        optimized_clear_value: Option<&ClearValue>,
    ) -> raw::windows::Result<(Resource, Allocation)> {
        unsafe {
            let alloc_desc = alloc_desc.into();
            let resource_desc = resource_desc.clone().into();

            let mut allocation = std::ptr::null_mut();
            let mut resource: Option<ID3D12Resource> = None;

            if let Some(optimized_clear_value) = optimized_clear_value {
                let optimized_clear_value: D3D12_CLEAR_VALUE = optimized_clear_value.clone().into();
                alloc_raw::D3D12MA_Allocator_CreateResource(
                    self.0.as_ptr(),
                    &alloc_desc,
                    &resource_desc,
                    initial_resource_state.into(),
                    &optimized_clear_value as *const D3D12_CLEAR_VALUE as *const _,
                    &mut allocation,
                    &ID3D12Resource::IID,
                    resource.set_abi(),
                )
                .ok()
                .map(|_| {
                    let resource = Resource(resource.unwrap());
                    let allocation = Allocation(NonNull::new(allocation).unwrap());
                    (resource, allocation)
                })
            } else {
                alloc_raw::D3D12MA_Allocator_CreateResource(
                    self.0.as_ptr(),
                    &alloc_desc,
                    &resource_desc,
                    initial_resource_state.into(),
                    std::ptr::null(),
                    &mut allocation,
                    &ID3D12Resource::IID,
                    resource.set_abi(),
                )
                .ok()
                .map(|_| {
                    let resource = Resource(resource.unwrap());
                    let allocation = Allocation(NonNull::new(allocation).unwrap());
                    (resource, allocation)
                })
            }
        }
    }
}

impl Drop for Allocator {
    fn drop(&mut self) {
        unsafe {
            alloc_raw::D3D12MA_Allocator_Release(self.0.as_ptr());
        }
    }
}
