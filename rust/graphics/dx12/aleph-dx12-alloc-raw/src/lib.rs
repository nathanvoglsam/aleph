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

extern crate aleph_dx12_raw as dx12_raw;

use dx12_raw::windows::win32::direct3d12::{
    ID3D12ProtectedResourceSession, D3D12_CLEAR_VALUE, D3D12_FEATURE_DATA_D3D12_OPTIONS,
    D3D12_HEAP_FLAGS, D3D12_HEAP_TYPE, D3D12_RESOURCE_ALLOCATION_INFO, D3D12_RESOURCE_DESC,
    D3D12_RESOURCE_DESC1, D3D12_RESOURCE_STATES,
};
use dx12_raw::windows::ErrorCode;
use dx12_raw::windows::Guid;
use dx12_raw::windows::BOOL;
use std::ffi::c_void;

pub type AllocateFn = extern "C" fn();
pub type FreeFn = extern "C" fn();

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
#[repr(transparent)]
pub struct AllocationFlags(pub u32);

impl AllocationFlags {
    pub const NONE: Self = Self(0b0);
    pub const COMMITTED: Self = Self(0b1);
    pub const NEVER_ALLOCATE: Self = Self(0b10);
    pub const WITHIN_BUDGET: Self = Self(0b100);
}

#[repr(C)]
pub struct AllocationCallbacks {
    pub p_allocate: AllocateFn,
    pub p_free: FreeFn,
    pub p_user_data: *mut c_void,
}

#[repr(C)]
pub struct AllocationDesc {
    pub flags: AllocationFlags,
    pub heap_type: D3D12_HEAP_TYPE,
    pub extra_heap_flags: D3D12_HEAP_FLAGS,
    pub pool: *mut c_void,
}

#[repr(C)]
pub struct PoolDesc {
    pub heap_type: D3D12_HEAP_TYPE,
    pub heap_flags: D3D12_HEAP_FLAGS,
    pub block_size: u64,
    pub min_block_count: u32,
    pub max_block_count: u32,
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
#[repr(transparent)]
pub struct AllocatorFlags(pub u32);

impl AllocatorFlags {
    pub const NONE: Self = Self(0b0);
    pub const SINGLE_THREADED: Self = Self(0b1);
    pub const ALWAYS_COMMITTED: Self = Self(0b10);
}

#[repr(C)]
pub struct AllocatorDesc {
    pub flags: AllocatorFlags,

    // AddRef/Release handled by allocator
    pub p_device: *mut c_void,

    pub preferred_block_size: u64,
    pub p_allocation_callbacks: *const AllocationCallbacks,

    // AddRef/Release handled by allocator
    pub p_adapter: *mut c_void,
}

#[repr(C)]
pub struct StatInfo {
    pub block_count: u32,
    pub allocation_count: u32,
    pub unused_range_count: u32,
    pub used_bytes: u64,
    pub unused_bytes: u64,
    pub allocation_size_min: u64,
    pub allocation_size_avg: u64,
    pub allocation_size_max: u64,
    pub unused_range_size_min: u64,
    pub unused_range_size_avg: u64,
    pub unused_range_size_max: u64,
}

pub const HEAP_TYPE_COUNT: usize = 3;

#[repr(C)]
pub struct Stats {
    pub total: StatInfo,
    pub heap_type: [StatInfo; HEAP_TYPE_COUNT],
}

#[repr(C)]
pub struct Budget {
    pub block_bytes: u64,
    pub allocation_bytes: u64,
    pub usage_bytes: u64,
    pub budget_bytes: u64,
}

#[repr(C)]
pub struct VirtualBlockDesc {
    pub size: u64,
    pub p_allocation_callbacks: *const AllocationCallbacks,
}

#[repr(C)]
pub struct VirtualAllocationDesc {
    pub size: u64,
    pub alignment: u64,
    pub p_user_data: *mut c_void,
}

#[repr(C)]
pub struct VirtualAllocationInfo {
    pub size: u64,
    pub p_user_data: *mut c_void,
}

pub type ThisPtr = *mut c_void;
pub type ThisPtrConst = *const c_void;

extern "C" {

    //
    // ALLOCATION
    //
    pub fn D3D12MA_Allocation_Release(this: ThisPtr);
    pub fn D3D12MA_Allocation_GetOffset(this: ThisPtrConst) -> u64;
    pub fn D3D12MA_Allocation_GetSize(this: ThisPtrConst) -> u64;
    pub fn D3D12MA_Allocation_GetResource(this: ThisPtrConst) -> *mut ();
    pub fn D3D12MA_Allocation_GetHeap(this: ThisPtrConst) -> *mut ();
    pub fn D3D12MA_Allocation_SetName(this: ThisPtr, name: *const u16);
    pub fn D3D12MA_Allocation_GetName(this: ThisPtrConst) -> *const u16;
    pub fn D3D12MA_Allocation_WasZeroInitialized(this: ThisPtrConst) -> BOOL;

    //
    // POOL
    //
    pub fn D3D12MA_Pool_Release(this: ThisPtr);
    pub fn D3D12MA_Pool_GetDesc(this: ThisPtrConst) -> PoolDesc;
    pub fn D3D12MA_Pool_SetMinBytes(this: ThisPtr, min_bytes: u64) -> ErrorCode;
    pub fn D3D12MA_Pool_CalculateStats(this: ThisPtr, p_stats: *mut StatInfo);
    pub fn D3D12MA_Pool_SetName(this: ThisPtr, name: *const u16);
    pub fn D3D12MA_Pool_GetName(this: ThisPtrConst) -> *const u16;

    //
    // ALLOCATOR
    //
    pub fn D3D12MA_Allocator_CreateAllocator(
        p_desc: *const AllocatorDesc,
        pp_allocator: *mut *mut c_void,
    ) -> ErrorCode;
    pub fn D3D12MA_Allocator_Release(this: ThisPtr);
    pub fn D3D12MA_Allocator_GetD3D12Options(
        this: ThisPtrConst,
    ) -> *const D3D12_FEATURE_DATA_D3D12_OPTIONS;
    pub fn D3D12MA_Allocator_CreateResource(
        this: ThisPtr,
        p_alloc_desc: *const AllocationDesc,
        p_resource_desc: *const D3D12_RESOURCE_DESC,
        initial_resource_state: D3D12_RESOURCE_STATES,
        p_optimized_clear_value: *const D3D12_CLEAR_VALUE,
        pp_allocation: *mut *mut c_void,
        riid_resource: Guid,
        ppv_resource: *mut *mut c_void,
    ) -> ErrorCode;
    pub fn D3D12MA_Allocator_CreateResource1(
        this: ThisPtr,
        p_alloc_desc: *const AllocationDesc,
        p_resource_desc: *const D3D12_RESOURCE_DESC,
        initial_resource_state: D3D12_RESOURCE_STATES,
        p_optimized_clear_value: *const D3D12_CLEAR_VALUE,
        p_protected_session: ID3D12ProtectedResourceSession,
        pp_allocation: *mut *mut c_void,
        riid_resource: Guid,
        ppv_resource: *mut *mut c_void,
    ) -> ErrorCode;
    pub fn D3D12MA_Allocator_CreateResource2(
        this: ThisPtr,
        p_alloc_desc: *const AllocationDesc,
        p_resource_desc: *const D3D12_RESOURCE_DESC1,
        initial_resource_state: D3D12_RESOURCE_STATES,
        p_optimized_clear_value: *const D3D12_CLEAR_VALUE,
        p_protected_session: ID3D12ProtectedResourceSession,
        pp_allocation: *mut *mut c_void,
        riid_resource: Guid,
        ppv_resource: *mut *mut c_void,
    ) -> ErrorCode;
    pub fn D3D12MA_Allocator_AllocateMemory(
        this: ThisPtr,
        p_alloc_desc: *const AllocationDesc,
        p_alloc_info: *const D3D12_RESOURCE_ALLOCATION_INFO,
        pp_allocation: *mut *mut c_void,
    ) -> ErrorCode;
    pub fn D3D12MA_Allocator_AllocateMemory1(
        this: ThisPtr,
        p_alloc_desc: *const AllocationDesc,
        p_alloc_info: *const D3D12_RESOURCE_ALLOCATION_INFO,
        p_protected_session: ID3D12ProtectedResourceSession,
        pp_allocation: *mut *mut c_void,
    ) -> ErrorCode;
    pub fn D3D12MA_Allocator_CreateAliasingResource(
        this: ThisPtr,
        p_allocation: *mut c_void,
        allocation_local_offset: u64,
        p_resource_desc: *const D3D12_RESOURCE_DESC,
        initial_resource_state: D3D12_RESOURCE_STATES,
        p_optimized_clear_value: *const D3D12_CLEAR_VALUE,
        riid_resource: Guid,
        ppv_resource: *mut *mut c_void,
    ) -> ErrorCode;
    pub fn D3D12MA_Allocator_CreatePool(
        this: ThisPtr,
        p_pool_desc: *const PoolDesc,
        pp_pool: *mut *mut c_void,
    ) -> ErrorCode;
    pub fn D3D12MA_Allocator_SetDefaultHeapMinBytes(
        this: ThisPtr,
        heap_type: D3D12_HEAP_TYPE,
        heap_flags: D3D12_HEAP_FLAGS,
        min_bytes: u64,
    ) -> ErrorCode;
    pub fn D3D12MA_Allocator_SetCurrentFrameIndex(this: ThisPtr, frame_index: u32);
    pub fn D3D12MA_Allocator_CalculateStats(this: ThisPtr, p_stats: *mut Stats);
    pub fn D3D12MA_Allocator_GetBudget(
        this: ThisPtr,
        p_gpu_budget: *mut Budget,
        p_cpu_budget: *mut Budget,
    );
    pub fn D3D12MA_Allocator_BuildStatsString(
        this: ThisPtrConst,
        pp_stats_string: *mut *mut u16,
        detailed_map: BOOL,
    );
    pub fn D3D12MA_Allocator_FreeStatsString(this: ThisPtrConst, p_stats_string: *mut u16);

    //
    // VIRTUAL BLOCK
    //
    pub fn D3D12MA_VirtualBlock_CreateAllocator(
        p_desc: *const VirtualBlockDesc,
        pp_allocator: *mut *mut c_void,
    ) -> ErrorCode;
    pub fn D3D12MA_VirtualBlock_Release(this: ThisPtr);
    pub fn D3D12MA_VirtualBlock_IsEmpty(this: ThisPtrConst) -> BOOL;
    pub fn D3D12MA_VirtualBlock_GetAllocationInfo(
        this: ThisPtrConst,
        offset: u64,
        p_info: *mut VirtualAllocationInfo,
    );
    pub fn D3D12MA_VirtualBlock_Allocate(
        this: ThisPtr,
        p_desc: *const VirtualAllocationDesc,
        p_offset: *mut u64,
    ) -> ErrorCode;
    pub fn D3D12MA_VirtualBlock_FreeAllocation(this: ThisPtr, offset: u64);
    pub fn D3D12MA_VirtualBlock_Clear(this: ThisPtr);
    pub fn D3D12MA_VirtualBlock_SetAllocationUserData(
        this: ThisPtr,
        offset: u64,
        p_user_data: *mut c_void,
    );
    pub fn D3D12MA_VirtualBlock_CalculateStats(this: ThisPtrConst, p_stats: *mut Stats);
    pub fn D3D12MA_VirtualBlock_BuildStatsString(
        this: ThisPtrConst,
        pp_stats_string: *mut *mut u16,
    );
    pub fn D3D12MA_VirtualBlock_FreeStatsString(this: ThisPtrConst, p_stats_string: *mut u16);
}
