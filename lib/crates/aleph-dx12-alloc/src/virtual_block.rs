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
    D3D12MA_VirtualBlock_Allocate, D3D12MA_VirtualBlock_CalculateStats, D3D12MA_VirtualBlock_Clear,
    D3D12MA_VirtualBlock_CreateAllocator, D3D12MA_VirtualBlock_FreeAllocation,
    D3D12MA_VirtualBlock_GetAllocationInfo, D3D12MA_VirtualBlock_IsEmpty,
    D3D12MA_VirtualBlock_Release, D3D12MA_VirtualBlock_SetAllocationUserData,
};
use crate::{
    D3D12MA_STATS, D3D12MA_VIRTUAL_ALLOCATION_DESC, D3D12MA_VIRTUAL_ALLOCATION_INFO,
    D3D12MA_VIRTUAL_BLOCK_DESC,
};
use std::ffi::c_void;
use std::ptr::NonNull;

#[derive(Clone)]
#[repr(transparent)]
pub struct D3D12MAVirtualBlock(pub(crate) NonNull<c_void>);

impl D3D12MAVirtualBlock {
    #[inline(always)]
    pub fn new(pBlockDesc: &D3D12MA_VIRTUAL_BLOCK_DESC) -> windows::core::Result<Self> {
        unsafe {
            let mut out = std::ptr::null_mut();
            D3D12MA_VirtualBlock_CreateAllocator(pBlockDesc, &mut out)
                .ok()
                .map(|_| D3D12MAVirtualBlock(NonNull::new(out).unwrap()))
        }
    }

    #[inline(always)]
    pub fn IsEmpty(&self) -> bool {
        unsafe { D3D12MA_VirtualBlock_IsEmpty(self.0.as_ptr()).as_bool() }
    }

    #[inline(always)]
    pub fn GetAllocationInfo(&self, Offset: u64, pInfo: &mut D3D12MA_VIRTUAL_ALLOCATION_INFO) {
        unsafe { D3D12MA_VirtualBlock_GetAllocationInfo(self.0.as_ptr(), Offset, pInfo) }
    }

    #[inline(always)]
    pub fn Allocate(
        &mut self,
        pDesc: &D3D12MA_VIRTUAL_ALLOCATION_DESC,
    ) -> windows::core::Result<u64> {
        unsafe {
            let mut out = 0;
            D3D12MA_VirtualBlock_Allocate(self.0.as_ptr(), pDesc, &mut out)
                .ok()
                .map(|_| out)
        }
    }

    #[inline(always)]
    pub fn FreeAllocation(&mut self, Offset: u64) {
        unsafe { D3D12MA_VirtualBlock_FreeAllocation(self.0.as_ptr(), Offset) }
    }

    #[inline(always)]
    pub fn Clear(&mut self) {
        unsafe { D3D12MA_VirtualBlock_Clear(self.0.as_ptr()) }
    }

    #[inline(always)]
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn SetAllocationUserData(&mut self, Offset: u64, pUserData: *mut c_void) {
        unsafe {
            D3D12MA_VirtualBlock_SetAllocationUserData(self.0.as_ptr(), Offset, pUserData);
        }
    }

    #[inline(always)]
    pub fn CalculateStats(&self, pStats: &mut D3D12MA_STATS) {
        unsafe { D3D12MA_VirtualBlock_CalculateStats(self.0.as_ptr(), pStats) }
    }

    // #[inline(always)]
    // pub fn BuildStatsString(this: ThisPtrConst, ppStatsString: *mut *mut u16);
    // #[inline(always)]
    // pub fn FreeStatsString(this: ThisPtrConst, pStatsString: *mut u16);
}

impl Drop for D3D12MAVirtualBlock {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe {
            D3D12MA_VirtualBlock_Release(self.0.as_ptr());
        }
    }
}
