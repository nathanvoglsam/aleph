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

use std::ffi::c_void;
use std::ptr::NonNull;
use std::sync::Arc;

use windows::core::ComInterface;
use windows::Win32::Graphics::Direct3D12::*;
use windows::Win32::Graphics::Dxgi::DXGI_MEMORY_SEGMENT_GROUP;

use crate::raw::{
    D3D12MA_Allocator_AllocateMemory, D3D12MA_Allocator_BeginDefragmentation,
    D3D12MA_Allocator_CalculateStatistics, D3D12MA_Allocator_CreateAliasingResource,
    D3D12MA_Allocator_CreateAllocator, D3D12MA_Allocator_CreatePool,
    D3D12MA_Allocator_CreateResource, D3D12MA_Allocator_CreateResource2,
    D3D12MA_Allocator_GetBudget, D3D12MA_Allocator_GetD3D12Options,
    D3D12MA_Allocator_GetMemoryCapacity, D3D12MA_Allocator_IsCacheCoherentUMA,
    D3D12MA_Allocator_IsUMA, D3D12MA_Allocator_Release, D3D12MA_Allocator_SetCurrentFrameIndex,
};
use crate::{
    Allocation, Budget, Pool, TotalStatistics, ALLOCATION_DESC, ALLOCATOR_DESC, ALLOCATOR_FLAGS,
    DEFRAGMENTATION_DESC, POOL_DESC,
};

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
pub struct Allocator(pub(crate) Arc<AllocatorInner>);

impl Allocator {
    pub fn new(pAllocatorDesc: &ALLOCATOR_DESC) -> windows::core::Result<Self> {
        if pAllocatorDesc
            .Flags
            .contains(ALLOCATOR_FLAGS::SINGLE_THREADED)
        {
            panic!("AllocatorFlags::SINGLE_THREADED not supported by rust bindings")
        }

        unsafe {
            let mut out = std::ptr::null_mut();
            D3D12MA_Allocator_CreateAllocator(pAllocatorDesc, &mut out)
                .ok()
                .map(|_| {
                    let out = AllocatorInner(NonNull::new(out).unwrap());
                    Allocator(Arc::new(out))
                })
        }
    }

    pub fn GetD3D12Options(&self) -> &D3D12_FEATURE_DATA_D3D12_OPTIONS {
        // Safety: We choose to trust that the foreign library will give us a valid pointer
        unsafe {
            let out = D3D12MA_Allocator_GetD3D12Options(self.0 .0.as_ptr());
            debug_assert!(!out.is_null());
            &*out
        }
    }

    pub fn IsUMA(&self) -> bool {
        unsafe { D3D12MA_Allocator_IsUMA(self.0 .0.as_ptr()).as_bool() }
    }

    pub fn IsCacheCoherentUMA(&self) -> bool {
        unsafe { D3D12MA_Allocator_IsCacheCoherentUMA(self.0 .0.as_ptr()).as_bool() }
    }

    pub fn GetMemoryCapacity(&self, memorySegmentGroup: DXGI_MEMORY_SEGMENT_GROUP) -> u64 {
        unsafe {
            D3D12MA_Allocator_GetMemoryCapacity(self.0 .0.as_ptr(), memorySegmentGroup.0 as u32)
        }
    }

    pub fn CreateResource<T: ComInterface>(
        &self,
        pAllocDesc: &ALLOCATION_DESC,
        pResourceDesc: &D3D12_RESOURCE_DESC,
        InitialResourceState: D3D12_RESOURCE_STATES,
        pOptimizedClearValue: Option<&D3D12_CLEAR_VALUE>,
    ) -> windows::core::Result<(Allocation, T)> {
        unsafe {
            let pOptimizedClearValue = if let Some(v) = pOptimizedClearValue {
                v as *const _
            } else {
                std::ptr::null()
            };

            let mut allocation = std::ptr::null_mut();
            let mut resource = std::ptr::null_mut();
            D3D12MA_Allocator_CreateResource(
                self.0 .0.as_ptr(),
                pAllocDesc,
                pResourceDesc,
                InitialResourceState,
                pOptimizedClearValue,
                &mut allocation,
                &<T as ComInterface>::IID,
                &mut resource,
            )
            .from_abi(resource)
            .map(|v| (Allocation(NonNull::new(allocation).unwrap()), v))
        }
    }

    pub fn CreateResource2<T: ComInterface>(
        &self,
        pAllocDesc: &ALLOCATION_DESC,
        pResourceDesc: &D3D12_RESOURCE_DESC1,
        InitialResourceState: D3D12_RESOURCE_STATES,
        pOptimizedClearValue: Option<&D3D12_CLEAR_VALUE>,
    ) -> windows::core::Result<(Allocation, T)> {
        unsafe {
            let pOptimizedClearValue = if let Some(v) = pOptimizedClearValue {
                v as *const _
            } else {
                std::ptr::null()
            };

            let mut allocation = std::ptr::null_mut();
            let mut resource = std::ptr::null_mut();
            D3D12MA_Allocator_CreateResource2(
                self.0 .0.as_ptr(),
                pAllocDesc,
                pResourceDesc,
                InitialResourceState,
                pOptimizedClearValue,
                &mut allocation,
                &<T as ComInterface>::IID,
                &mut resource,
            )
            .from_abi(resource)
            .map(|v| (Allocation(NonNull::new(allocation).unwrap()), v))
        }
    }

    pub fn AllocateMemory(
        &self,
        pAllocDesc: &ALLOCATION_DESC,
        pAllocInfo: &D3D12_RESOURCE_ALLOCATION_INFO,
    ) -> windows::core::Result<Allocation> {
        unsafe {
            let mut allocation = std::ptr::null_mut();
            D3D12MA_Allocator_AllocateMemory(
                self.0 .0.as_ptr(),
                pAllocDesc,
                pAllocInfo,
                &mut allocation,
            )
            .ok()
            .map(|_| Allocation(NonNull::new(allocation).unwrap()))
        }
    }

    pub fn CreateAliasingResource<T: ComInterface>(
        &self,
        pAllocation: Allocation,
        AllocationLocalOffset: u64,
        pResourceDesc: &D3D12_RESOURCE_DESC,
        InitialResourceState: D3D12_RESOURCE_STATES,
        pOptimizedClearValue: Option<&D3D12_CLEAR_VALUE>,
    ) -> windows::core::Result<ID3D12Resource> {
        unsafe {
            let pOptimizedClearValue = if let Some(v) = pOptimizedClearValue {
                v as *const _
            } else {
                std::ptr::null()
            };

            let mut resource = std::ptr::null_mut();
            D3D12MA_Allocator_CreateAliasingResource(
                self.0 .0.as_ptr(),
                pAllocation.0.as_ptr(),
                AllocationLocalOffset,
                pResourceDesc,
                InitialResourceState,
                pOptimizedClearValue,
                &<T as ComInterface>::IID,
                &mut resource,
            )
            .from_abi(resource)
        }
    }

    pub fn CreatePool(&self, pPoolDesc: &POOL_DESC) -> windows::core::Result<Pool> {
        unsafe {
            let mut pool = std::ptr::null_mut();
            D3D12MA_Allocator_CreatePool(self.0 .0.as_ptr(), pPoolDesc, &mut pool)
                .ok()
                .map(|_| Pool(NonNull::new(pool).unwrap()))
        }
    }

    pub fn SetCurrentFrameIndex(&mut self, FrameIndex: u32) {
        unsafe { D3D12MA_Allocator_SetCurrentFrameIndex(self.0 .0.as_ptr(), FrameIndex) }
    }

    /// Returns (localBudget, nonLocalBudget)
    pub fn GetBudget(&mut self) -> (Budget, Budget) {
        let mut localBudget = Budget::default();
        let mut nonLocalBudget = Budget::default();
        unsafe {
            D3D12MA_Allocator_GetBudget(self.0 .0.as_ptr(), &mut localBudget, &mut nonLocalBudget);
        }
        (localBudget, nonLocalBudget)
    }

    pub fn CalculateStatistics(&mut self) -> TotalStatistics {
        let mut stats = TotalStatistics::default();
        unsafe { D3D12MA_Allocator_CalculateStatistics(self.0 .0.as_ptr(), &mut stats) }
        stats
    }

    // pub fn D3D12MA_Allocator_BuildStatsString(
    //     &self,
    //     ppStatsString: *mut *mut u16,
    //     DetailedMap: BOOL,
    // );
    // pub fn D3D12MA_Allocator_FreeStatsString(&self, pStatsString: *mut u16);

    pub fn D3D12MA_Allocator_BeginDefragmentation(
        &mut self,
        pDesc: &DEFRAGMENTATION_DESC,
        ppContext: *mut *mut c_void,
    ) {
        unsafe {
            D3D12MA_Allocator_BeginDefragmentation(self.0 .0.as_ptr(), pDesc, ppContext);
        }
    }
}
