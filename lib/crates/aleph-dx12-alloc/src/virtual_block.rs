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

use crate::raw::{
    D3D12MA_VirtualBlock_Allocate, D3D12MA_VirtualBlock_CalculateStatistics,
    D3D12MA_VirtualBlock_Clear, D3D12MA_VirtualBlock_CreateVirtualBlock,
    D3D12MA_VirtualBlock_FreeAllocation, D3D12MA_VirtualBlock_GetAllocationInfo,
    D3D12MA_VirtualBlock_GetStatistics, D3D12MA_VirtualBlock_IsEmpty, D3D12MA_VirtualBlock_Release,
    D3D12MA_VirtualBlock_SetAllocationPrivateData,
};
use crate::{
    DetailedStatistics, Statistics, VirtualAllocation, VIRTUAL_ALLOCATION_DESC,
    VIRTUAL_ALLOCATION_INFO, VIRTUAL_BLOCK_DESC,
};

#[repr(transparent)]
pub struct VirtualBlock(pub(crate) NonNull<c_void>);

impl VirtualBlock {
    #[inline(always)]
    pub fn new(pBlockDesc: &VIRTUAL_BLOCK_DESC) -> windows::core::Result<Self> {
        unsafe {
            let mut out = std::ptr::null_mut();
            D3D12MA_VirtualBlock_CreateVirtualBlock(pBlockDesc, &mut out)
                .ok()
                .map(|_| VirtualBlock(NonNull::new(out).unwrap()))
        }
    }

    #[inline(always)]
    pub fn IsEmpty(&self) -> bool {
        unsafe { D3D12MA_VirtualBlock_IsEmpty(self.0.as_ptr()).as_bool() }
    }

    /// # Safety
    ///
    /// It is the caller's responsibility to ensure that the given [VirtualAllocation] handle is
    /// both alive (hasn't been freed) and was allocated from this allocator.
    #[inline(always)]
    pub unsafe fn GetAllocationInfo(
        &self,
        allocation: VirtualAllocation,
    ) -> VIRTUAL_ALLOCATION_INFO {
        let mut info = VIRTUAL_ALLOCATION_INFO {
            Offset: 0,
            Size: 0,
            pPrivateData: std::ptr::null_mut(),
        };

        D3D12MA_VirtualBlock_GetAllocationInfo(self.0.as_ptr(), allocation, &mut info);

        info
    }

    #[inline(always)]
    pub fn Allocate(
        &mut self,
        pDesc: &VIRTUAL_ALLOCATION_DESC,
    ) -> windows::core::Result<(VirtualAllocation, u64)> {
        unsafe {
            let mut offset = 0;
            let mut out = None;
            D3D12MA_VirtualBlock_Allocate(self.0.as_ptr(), pDesc, &mut out, &mut offset)
                .ok()
                .map(|_| (out.unwrap(), offset))
        }
    }

    /// # Safety
    ///
    /// It is the caller's responsibility to ensure that the given [VirtualAllocation] handle is
    /// both alive (hasn't been freed) and was allocated from this allocator.
    #[inline(always)]
    pub unsafe fn FreeAllocation(&mut self, allocation: VirtualAllocation) {
        D3D12MA_VirtualBlock_FreeAllocation(self.0.as_ptr(), allocation)
    }

    #[inline(always)]
    pub fn Clear(&mut self) {
        unsafe { D3D12MA_VirtualBlock_Clear(self.0.as_ptr()) }
    }

    /// # Safety
    ///
    /// It is the caller's responsibility to ensure that the given [VirtualAllocation] handle is
    /// both alive (hasn't been freed) and was allocated from this allocator.
    #[inline(always)]
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub unsafe fn SetAllocationPrivateData(
        &mut self,
        allocation: VirtualAllocation,
        pPrivateData: *mut c_void,
    ) {
        D3D12MA_VirtualBlock_SetAllocationPrivateData(self.0.as_ptr(), allocation, pPrivateData);
    }

    #[inline(always)]
    pub fn GetStatistics(&self) -> Statistics {
        let mut stats = Statistics::default();
        unsafe { D3D12MA_VirtualBlock_GetStatistics(self.0.as_ptr(), &mut stats) }
        stats
    }

    #[inline(always)]
    pub fn CalculateStatistics(&self) -> DetailedStatistics {
        let mut stats = DetailedStatistics::default();
        unsafe { D3D12MA_VirtualBlock_CalculateStatistics(self.0.as_ptr(), &mut stats) }
        stats
    }

    // #[inline(always)]
    // pub fn BuildStatsString(this: ThisPtrConst, ppStatsString: *mut *mut u16);
    // #[inline(always)]
    // pub fn FreeStatsString(this: ThisPtrConst, pStatsString: *mut u16);
}

impl Drop for VirtualBlock {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe {
            D3D12MA_VirtualBlock_Release(self.0.as_ptr());
        }
    }
}
