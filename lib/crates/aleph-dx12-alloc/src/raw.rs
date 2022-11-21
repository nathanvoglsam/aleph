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
#![allow(non_camel_case_types)]

use std::ffi::c_void;
use windows::core::{GUID, HRESULT};
use windows::Win32::Foundation::BOOL;
use windows::Win32::Graphics::Direct3D12::*;
use windows::Win32::Graphics::Dxgi::IDXGIAdapter;

macro_rules! flags_bitwise_impl {
    ($t:ident) => {
        impl std::ops::BitOr for $t {
            type Output = Self;

            fn bitor(self, rhs: Self) -> Self::Output {
                Self(self.0 | rhs.0)
            }
        }

        impl std::ops::BitOrAssign for $t {
            fn bitor_assign(&mut self, rhs: Self) {
                self.0 |= rhs.0
            }
        }

        impl std::ops::BitAnd for $t {
            type Output = Self;

            fn bitand(self, rhs: Self) -> Self::Output {
                Self(self.0 & rhs.0)
            }
        }

        impl std::ops::BitAndAssign for $t {
            fn bitand_assign(&mut self, rhs: Self) {
                self.0 &= rhs.0
            }
        }

        impl std::ops::BitXor for $t {
            type Output = Self;

            fn bitxor(self, rhs: Self) -> Self::Output {
                Self(self.0 ^ rhs.0)
            }
        }

        impl std::ops::BitXorAssign for $t {
            fn bitxor_assign(&mut self, rhs: Self) {
                self.0 ^= rhs.0
            }
        }
    };
}

pub type D3D12MA_ALLOCATE_FN = extern "C" fn();
pub type D3D12MA_FREE_FN = extern "C" fn();

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
#[repr(transparent)]
pub struct D3D12MA_ALLOCATION_FLAGS(pub u32);

impl D3D12MA_ALLOCATION_FLAGS {
    pub const NONE: Self = Self(0b0);
    pub const COMMITTED: Self = Self(0b1);
    pub const NEVER_ALLOCATE: Self = Self(0b10);
    pub const WITHIN_BUDGET: Self = Self(0b100);
}

flags_bitwise_impl!(D3D12MA_ALLOCATION_FLAGS);

#[repr(C)]
pub struct D3D12MA_ALLOCATION_CALLBACKS {
    pub pAllocate: D3D12MA_ALLOCATE_FN,
    pub pFree: D3D12MA_FREE_FN,
    pub pUserData: *mut c_void,
}

#[repr(C)]
pub struct D3D12MA_ALLOCATION_DESC {
    pub Flags: D3D12MA_ALLOCATION_FLAGS,
    pub HeapType: D3D12_HEAP_TYPE,
    pub ExtraHeapFlags: D3D12_HEAP_FLAGS,
    pub Pool: *mut c_void,
}

#[repr(C)]
pub struct D3D12MA_POOL_DESC {
    pub HeapType: D3D12_HEAP_TYPE,
    pub HeapFlags: D3D12_HEAP_FLAGS,
    pub BlockSize: u64,
    pub MinBlockCount: u32,
    pub MaxBlockCount: u32,
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
#[repr(transparent)]
pub struct D3D12MA_ALLOCATOR_FLAGS(pub u32);

impl D3D12MA_ALLOCATOR_FLAGS {
    pub const NONE: Self = Self(0b0);
    pub const SINGLE_THREADED: Self = Self(0b1);
    pub const ALWAYS_COMMITTED: Self = Self(0b10);
}

flags_bitwise_impl!(D3D12MA_ALLOCATOR_FLAGS);

#[repr(C)]
pub struct D3D12MA_ALLOCATOR_DESC {
    pub Flags: D3D12MA_ALLOCATOR_FLAGS,

    // AddRef/Release handled by allocator
    pub pDevice: Option<ID3D12Device>,

    pub PreferredBlockSize: u64,
    pub pAllocationCallbacks: *const D3D12MA_ALLOCATION_CALLBACKS,

    // AddRef/Release handled by allocator
    pub pAdapter: Option<IDXGIAdapter>,
}

#[repr(C)]
pub struct D3D12MA_STAT_INFO {
    pub BlockCount: u32,
    pub AllocationCount: u32,
    pub UnusedRangeCount: u32,
    pub UsedBytes: u64,
    pub UnusedBytes: u64,
    pub AllocationSizeMin: u64,
    pub AllocationSizeAvg: u64,
    pub AllocationSizeMax: u64,
    pub UnusedRangeSizeMin: u64,
    pub UnusedRangeSizeAvg: u64,
    pub UnusedRangeSizeMax: u64,
}

pub const HEAP_TYPE_COUNT: usize = 3;

#[repr(C)]
pub struct D3D12MA_STATS {
    pub Total: D3D12MA_STAT_INFO,
    pub HeapType: [D3D12MA_STAT_INFO; HEAP_TYPE_COUNT],
}

#[repr(C)]
pub struct D3D12MA_BUDGET {
    pub BlockBytes: u64,
    pub AllocationBytes: u64,
    pub UsageBytes: u64,
    pub BudgetBytes: u64,
}

#[repr(C)]
pub struct D3D12MA_VIRTUAL_BLOCK_DESC {
    pub Size: u64,
    pub pAllocationCallbacks: *const D3D12MA_ALLOCATION_CALLBACKS,
}

#[repr(C)]
pub struct D3D12MA_VIRTUAL_ALLOCATION_DESC {
    pub Size: u64,
    pub Alignment: u64,
    pub pUserData: *mut c_void,
}

#[repr(C)]
pub struct D3D12MA_VIRTUAL_ALLOCATION_INFO {
    pub Size: u64,
    pub pUserData: *mut c_void,
}

pub type ThisPtr = *mut c_void;
pub type ThisPtrConst = *const c_void;

#[allow(unused)]
extern "C" {

    //
    // ALLOCATION
    //
    pub fn D3D12MA_Allocation_Release(this: ThisPtr);
    pub fn D3D12MA_Allocation_GetOffset(this: ThisPtrConst) -> u64;
    pub fn D3D12MA_Allocation_GetSize(this: ThisPtrConst) -> u64;
    pub fn D3D12MA_Allocation_GetResource(this: ThisPtrConst) -> Option<ID3D12Resource>;
    pub fn D3D12MA_Allocation_GetHeap(this: ThisPtrConst) -> Option<ID3D12Heap>;
    pub fn D3D12MA_Allocation_SetName(this: ThisPtr, Name: *const u16);
    pub fn D3D12MA_Allocation_GetName(this: ThisPtrConst) -> *const u16;
    pub fn D3D12MA_Allocation_WasZeroInitialized(this: ThisPtrConst) -> BOOL;

    //
    // POOL
    //
    pub fn D3D12MA_Pool_Release(this: ThisPtr);
    pub fn D3D12MA_Pool_GetDesc(this: ThisPtrConst) -> D3D12MA_POOL_DESC;
    pub fn D3D12MA_Pool_SetMinBytes(this: ThisPtr, MinBytes: u64) -> HRESULT;
    pub fn D3D12MA_Pool_CalculateStats(this: ThisPtr, pStats: *mut D3D12MA_STAT_INFO);
    pub fn D3D12MA_Pool_SetName(this: ThisPtr, Name: *const u16);
    pub fn D3D12MA_Pool_GetName(this: ThisPtrConst) -> *const u16;

    //
    // ALLOCATOR
    //
    pub fn D3D12MA_Allocator_CreateAllocator(
        pDesc: *const D3D12MA_ALLOCATOR_DESC,
        ppAllocator: *mut *mut c_void,
    ) -> HRESULT;
    pub fn D3D12MA_Allocator_Release(this: ThisPtr);
    pub fn D3D12MA_Allocator_GetD3D12Options(
        this: ThisPtrConst,
    ) -> *const D3D12_FEATURE_DATA_D3D12_OPTIONS;
    pub fn D3D12MA_Allocator_CreateResource(
        this: ThisPtr,
        pAllocDesc: *const D3D12MA_ALLOCATION_DESC,
        pResourceDesc: *const D3D12_RESOURCE_DESC,
        InitialResourceState: D3D12_RESOURCE_STATES,
        pOptimizedClearValue: *const D3D12_CLEAR_VALUE,
        ppAllocation: *mut *mut c_void,
        riidResource: *const GUID,
        ppvResource: *mut *mut c_void,
    ) -> HRESULT;
    pub fn D3D12MA_Allocator_CreateResource1(
        this: ThisPtr,
        pAllocDesc: *const D3D12MA_ALLOCATION_DESC,
        pResourceDesc: *const D3D12_RESOURCE_DESC,
        InitialResourceState: D3D12_RESOURCE_STATES,
        pOptimizedClearValue: *const D3D12_CLEAR_VALUE,
        pProtectedSession: ID3D12ProtectedResourceSession,
        ppAllocation: *mut *mut c_void,
        riidResource: *const GUID,
        ppvResource: *mut *mut c_void,
    ) -> HRESULT;
    pub fn D3D12MA_Allocator_CreateResource2(
        this: ThisPtr,
        pAllocDesc: *const D3D12MA_ALLOCATION_DESC,
        pResourceDesc: *const D3D12_RESOURCE_DESC1,
        InitialResourceState: D3D12_RESOURCE_STATES,
        pOptimizedClearValue: *const D3D12_CLEAR_VALUE,
        pProtectedSession: ID3D12ProtectedResourceSession,
        ppAllocation: *mut *mut c_void,
        riidResource: *const GUID,
        ppvResource: *mut *mut c_void,
    ) -> HRESULT;
    pub fn D3D12MA_Allocator_AllocateMemory(
        this: ThisPtr,
        pAllocDesc: *const D3D12MA_ALLOCATION_DESC,
        pAllocInfo: *const D3D12_RESOURCE_ALLOCATION_INFO,
        ppAllocation: *mut *mut c_void,
    ) -> HRESULT;
    pub fn D3D12MA_Allocator_AllocateMemory1(
        this: ThisPtr,
        pAllocDesc: *const D3D12MA_ALLOCATION_DESC,
        pAllocInfo: *const D3D12_RESOURCE_ALLOCATION_INFO,
        pProtectedSession: ID3D12ProtectedResourceSession,
        ppAllocation: *mut *mut c_void,
    ) -> HRESULT;
    pub fn D3D12MA_Allocator_CreateAliasingResource(
        this: ThisPtr,
        pAllocation: *mut c_void,
        AllocationLocalOffset: u64,
        pResourceDesc: *const D3D12_RESOURCE_DESC,
        InitialResourceState: D3D12_RESOURCE_STATES,
        pOptimizedClearValue: *const D3D12_CLEAR_VALUE,
        riidResource: *const GUID,
        ppvResource: *mut *mut c_void,
    ) -> HRESULT;
    pub fn D3D12MA_Allocator_CreatePool(
        this: ThisPtr,
        pPoolDesc: *const D3D12MA_POOL_DESC,
        ppPool: *mut *mut c_void,
    ) -> HRESULT;
    pub fn D3D12MA_Allocator_SetDefaultHeapMinBytes(
        this: ThisPtr,
        HeapType: D3D12_HEAP_TYPE,
        HeapFlags: D3D12_HEAP_FLAGS,
        MinBytes: u64,
    ) -> HRESULT;
    pub fn D3D12MA_Allocator_SetCurrentFrameIndex(this: ThisPtr, FrameIndex: u32);
    pub fn D3D12MA_Allocator_CalculateStats(this: ThisPtr, pStats: *mut D3D12MA_STATS);
    pub fn D3D12MA_Allocator_GetBudget(
        this: ThisPtr,
        pGpuBudget: *mut D3D12MA_BUDGET,
        pCpuBudget: *mut D3D12MA_BUDGET,
    );
    pub fn D3D12MA_Allocator_BuildStatsString(
        this: ThisPtrConst,
        ppStatsString: *mut *mut u16,
        DetailedMap: BOOL,
    );
    pub fn D3D12MA_Allocator_FreeStatsString(this: ThisPtrConst, pStatsString: *mut u16);

    //
    // VIRTUAL BLOCK
    //
    pub fn D3D12MA_VirtualBlock_CreateAllocator(
        pDesc: *const D3D12MA_VIRTUAL_BLOCK_DESC,
        ppAllocator: *mut *mut c_void,
    ) -> HRESULT;
    pub fn D3D12MA_VirtualBlock_Release(this: ThisPtr);
    pub fn D3D12MA_VirtualBlock_IsEmpty(this: ThisPtrConst) -> BOOL;
    pub fn D3D12MA_VirtualBlock_GetAllocationInfo(
        this: ThisPtrConst,
        Offset: u64,
        pInfo: *mut D3D12MA_VIRTUAL_ALLOCATION_INFO,
    );
    pub fn D3D12MA_VirtualBlock_Allocate(
        this: ThisPtr,
        pDesc: *const D3D12MA_VIRTUAL_ALLOCATION_DESC,
        pOffset: *mut u64,
    ) -> HRESULT;
    pub fn D3D12MA_VirtualBlock_FreeAllocation(this: ThisPtr, Offset: u64);
    pub fn D3D12MA_VirtualBlock_Clear(this: ThisPtr);
    pub fn D3D12MA_VirtualBlock_SetAllocationUserData(
        this: ThisPtr,
        Offset: u64,
        pUserData: *mut c_void,
    );
    pub fn D3D12MA_VirtualBlock_CalculateStats(this: ThisPtrConst, pStats: *mut D3D12MA_STATS);
    pub fn D3D12MA_VirtualBlock_BuildStatsString(this: ThisPtrConst, ppStatsString: *mut *mut u16);
    pub fn D3D12MA_VirtualBlock_FreeStatsString(this: ThisPtrConst, pStatsString: *mut u16);
}
