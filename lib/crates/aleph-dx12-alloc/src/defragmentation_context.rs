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
    D3D12MA_DefragmentationContext_BeginPass, D3D12MA_DefragmentationContext_EndPass,
    D3D12MA_DefragmentationContext_GetStats, D3D12MA_DefragmentationContext_Release,
};
use crate::{DEFRAGMENTATION_PASS_MOVE_INFO, DEFRAGMENTATION_STATS};
use std::ffi::c_void;
use std::ptr::NonNull;
use windows::core::HRESULT;

#[repr(transparent)]
pub struct DefragmentationContext(pub(crate) NonNull<c_void>);

impl DefragmentationContext {
    #[inline(always)]
    pub fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr() as *const _
    }

    pub unsafe fn BeginPass(&mut self, pPassInfo: &mut DEFRAGMENTATION_PASS_MOVE_INFO) -> HRESULT {
        D3D12MA_DefragmentationContext_BeginPass(self.0.as_ptr(), pPassInfo)
    }

    pub unsafe fn EndPass(&mut self, pPassInfo: &mut DEFRAGMENTATION_PASS_MOVE_INFO) -> HRESULT {
        D3D12MA_DefragmentationContext_EndPass(self.0.as_ptr(), pPassInfo)
    }

    pub unsafe fn GetStats(&mut self, pStats: &mut DEFRAGMENTATION_STATS) {
        D3D12MA_DefragmentationContext_GetStats(self.0.as_ptr(), pStats)
    }
}

impl Drop for DefragmentationContext {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe {
            D3D12MA_DefragmentationContext_Release(self.0.as_ptr());
        }
    }
}
