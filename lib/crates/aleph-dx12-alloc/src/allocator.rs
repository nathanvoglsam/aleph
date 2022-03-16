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

use crate::allocation::AllocationInner;
use crate::pool::PoolInner;
use crate::{Allocation, AllocationDesc, AllocatorDesc, AllocatorFlags, Pool, PoolDesc};
use dx12::{ClearValue, ResourceDesc, ResourceStates};
use std::ffi::c_void;
use std::mem::{align_of, size_of};
use std::ptr::NonNull;
use std::sync::Arc;
use windows::Win32::Graphics::Direct3D12::{D3D12_CLEAR_VALUE, D3D12_RESOURCE_DESC};

#[repr(transparent)]
pub(crate) struct AllocatorInner(pub(crate) NonNull<c_void>);

impl Drop for AllocatorInner {
    fn drop(&mut self) {
        unsafe {
            alloc_raw::D3D12MA_Allocator_Release(self.0.as_ptr());
        }
    }
}

#[derive(Clone)]
#[repr(transparent)]
pub struct Allocator(pub(crate) Arc<AllocatorInner>);

impl Allocator {
    pub fn new(allocator_desc: &AllocatorDesc) -> dx12::Result<Self> {
        assert_eq!(
            size_of::<AllocatorDesc>(),
            size_of::<aleph_dx12_alloc_raw::D3D12MA_ALLOCATOR_DESC>()
        );
        assert_eq!(
            align_of::<AllocatorDesc>(),
            align_of::<aleph_dx12_alloc_raw::D3D12MA_ALLOCATOR_DESC>()
        );

        if allocator_desc.flags & AllocatorFlags::SINGLE_THREADED != AllocatorFlags::NONE {
            panic!("AllocatorFlags::SINGLE_THREADED not supported by rust bindings")
        }

        unsafe {
            let mut out = std::ptr::null_mut();
            alloc_raw::D3D12MA_Allocator_CreateAllocator(
                allocator_desc as *const AllocatorDesc as *const _,
                &mut out,
            )
            .ok()
            .map(|_| {
                let out = AllocatorInner(NonNull::new(out).unwrap());
                Allocator(Arc::new(out))
            })
        }
    }

    pub fn create_resource(
        &self,
        alloc_desc: &AllocationDesc,
        resource_desc: &ResourceDesc,
        initial_resource_state: ResourceStates,
        optimized_clear_value: Option<&ClearValue>,
    ) -> dx12::Result<Allocation> {
        unsafe {
            let alloc_desc = alloc_desc.into();

            let mut allocation = std::ptr::null_mut();

            if let Some(optimized_clear_value) = optimized_clear_value {
                let optimized_clear_value: D3D12_CLEAR_VALUE = optimized_clear_value.clone().into();
                alloc_raw::D3D12MA_Allocator_CreateResource(
                    self.0 .0.as_ptr(),
                    &alloc_desc,
                    resource_desc as *const _ as *const D3D12_RESOURCE_DESC,
                    initial_resource_state.into(),
                    &optimized_clear_value as *const D3D12_CLEAR_VALUE as *const _,
                    &mut allocation,
                    std::ptr::null(),
                    std::ptr::null_mut(),
                )
                .ok()
                .map(|_| {
                    let allocation = AllocationInner(NonNull::new(allocation).unwrap());
                    Allocation(Arc::new(allocation))
                })
            } else {
                alloc_raw::D3D12MA_Allocator_CreateResource(
                    self.0 .0.as_ptr(),
                    &alloc_desc,
                    resource_desc as *const _ as *const D3D12_RESOURCE_DESC,
                    initial_resource_state.into(),
                    std::ptr::null(),
                    &mut allocation,
                    std::ptr::null(),
                    std::ptr::null_mut(),
                )
                .ok()
                .map(|_| {
                    let allocation = AllocationInner(NonNull::new(allocation).unwrap());
                    Allocation(Arc::new(allocation))
                })
            }
        }
    }

    pub fn create_pool(&self, pool_desc: &PoolDesc) -> dx12::Result<Pool> {
        unsafe {
            let pool_desc = pool_desc.clone().into();
            let mut pool = std::ptr::null_mut();
            alloc_raw::D3D12MA_Allocator_CreatePool(self.0 .0.as_ptr(), &pool_desc, &mut pool)
                .ok()
                .map(|_| {
                    let pool = PoolInner(NonNull::new(pool).unwrap());
                    Pool(Arc::new(pool))
                })
        }
    }
}
