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
    D3D12MA_Pool_BeginDefragmentation, D3D12MA_Pool_CalculateStatistics, D3D12MA_Pool_GetDesc,
    D3D12MA_Pool_GetName, D3D12MA_Pool_GetStatistics, D3D12MA_Pool_Release, D3D12MA_Pool_SetName,
};
use crate::{DetailedStatistics, Statistics, DEFRAGMENTATION_DESC, POOL_DESC};
use std::ffi::c_void;
use std::ptr::NonNull;

#[repr(transparent)]
pub struct Pool(pub(crate) NonNull<c_void>);

impl Pool {
    #[inline(always)]
    pub fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr() as *const _
    }

    #[inline(always)]
    pub fn GetDesc(&self) -> POOL_DESC {
        unsafe { D3D12MA_Pool_GetDesc(self.0.as_ptr()) }
    }

    #[inline(always)]
    pub fn GetStatistics(&self) -> Statistics {
        let mut out = Statistics::default();
        unsafe {
            D3D12MA_Pool_GetStatistics(self.0.as_ptr(), &mut out);
        }
        out
    }

    #[inline(always)]
    pub fn CalculateStatistics(&self) -> DetailedStatistics {
        let mut out = DetailedStatistics::default();
        unsafe {
            D3D12MA_Pool_CalculateStatistics(self.0.as_ptr(), &mut out);
        }
        out
    }

    #[inline(always)]
    pub fn SetName(&mut self, Name: *const u16) {
        unsafe { D3D12MA_Pool_SetName(self.0.as_ptr(), Name) }
    }

    #[inline(always)]
    pub fn GetName(&self) -> *const u16 {
        unsafe { D3D12MA_Pool_GetName(self.0.as_ptr()) }
    }

    #[inline(always)]
    pub fn BeginDefragmentation(
        &mut self,
        pDesc: &DEFRAGMENTATION_DESC,
        ppContext: *mut *mut c_void, // TODO
    ) {
        unsafe { D3D12MA_Pool_BeginDefragmentation(self.0.as_ptr(), pDesc, ppContext) }
    }
}

impl Drop for Pool {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe { D3D12MA_Pool_Release(self.0.as_ptr()) }
    }
}
