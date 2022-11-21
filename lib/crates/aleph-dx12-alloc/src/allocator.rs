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

#![allow(non_snake_case)]

use crate::raw::{
    D3D12MA_Allocator_CreateAllocator, D3D12MA_Allocator_CreatePool,
    D3D12MA_Allocator_CreateResource, D3D12MA_Allocator_Release,
};
use crate::{
    D3D12MAAllocation, D3D12MAPool, D3D12MA_ALLOCATION_DESC, D3D12MA_ALLOCATOR_DESC,
    D3D12MA_ALLOCATOR_FLAGS, D3D12MA_POOL_DESC,
};
use std::ffi::c_void;
use std::ptr::NonNull;
use std::sync::Arc;
use windows::Win32::Graphics::Direct3D12::*;

#[repr(transparent)]
pub(crate) struct AllocatorInner(pub(crate) NonNull<c_void>);

impl Drop for AllocatorInner {
    fn drop(&mut self) {
        unsafe {
            D3D12MA_Allocator_Release(self.0.as_ptr());
        }
    }
}

#[derive(Clone)]
#[repr(transparent)]
pub struct D3D12MAAllocator(pub(crate) Arc<AllocatorInner>);

impl D3D12MAAllocator {
    pub fn new(pAllocatorDesc: &D3D12MA_ALLOCATOR_DESC) -> windows::core::Result<Self> {
        if pAllocatorDesc.Flags & D3D12MA_ALLOCATOR_FLAGS::SINGLE_THREADED
            != D3D12MA_ALLOCATOR_FLAGS::NONE
        {
            panic!("AllocatorFlags::SINGLE_THREADED not supported by rust bindings")
        }

        unsafe {
            let mut out = std::ptr::null_mut();
            D3D12MA_Allocator_CreateAllocator(pAllocatorDesc, &mut out)
                .ok()
                .map(|_| {
                    let out = AllocatorInner(NonNull::new(out).unwrap());
                    D3D12MAAllocator(Arc::new(out))
                })
        }
    }

    pub fn CreateResource(
        &self,
        pAllocDesc: &D3D12MA_ALLOCATION_DESC,
        pResourceDesc: &D3D12_RESOURCE_DESC,
        InitialResourceState: D3D12_RESOURCE_STATES,
        pOptimizedClearValue: *const D3D12_CLEAR_VALUE,
    ) -> windows::core::Result<D3D12MAAllocation> {
        unsafe {
            let mut allocation = std::ptr::null_mut();

            if !pOptimizedClearValue.is_null() {
                D3D12MA_Allocator_CreateResource(
                    self.0 .0.as_ptr(),
                    pAllocDesc,
                    pResourceDesc,
                    InitialResourceState,
                    pOptimizedClearValue,
                    &mut allocation,
                    std::ptr::null(),
                    std::ptr::null_mut(),
                )
                .ok()
                .map(|_| D3D12MAAllocation(NonNull::new(allocation).unwrap()))
            } else {
                D3D12MA_Allocator_CreateResource(
                    self.0 .0.as_ptr(),
                    pAllocDesc,
                    pResourceDesc,
                    InitialResourceState,
                    std::ptr::null(),
                    &mut allocation,
                    std::ptr::null(),
                    std::ptr::null_mut(),
                )
                .ok()
                .map(|_| D3D12MAAllocation(NonNull::new(allocation).unwrap()))
            }
        }
    }

    pub fn CreatePool(&self, pPoolDesc: &D3D12MA_POOL_DESC) -> windows::core::Result<D3D12MAPool> {
        unsafe {
            let mut pool = std::ptr::null_mut();
            D3D12MA_Allocator_CreatePool(self.0 .0.as_ptr(), pPoolDesc, &mut pool)
                .ok()
                .map(|_| D3D12MAPool(NonNull::new(pool).unwrap()))
        }
    }
}
