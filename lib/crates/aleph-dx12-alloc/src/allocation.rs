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
    D3D12MA_Allocation_GetAlignment, D3D12MA_Allocation_GetHeap, D3D12MA_Allocation_GetName,
    D3D12MA_Allocation_GetOffset, D3D12MA_Allocation_GetPrivateData,
    D3D12MA_Allocation_GetResource, D3D12MA_Allocation_GetSize, D3D12MA_Allocation_Release,
    D3D12MA_Allocation_SetName, D3D12MA_Allocation_SetPrivateData, D3D12MA_Allocation_SetResource,
    D3D12MA_Allocation_WasZeroInitialized,
};
use std::ffi::c_void;
use std::ptr::NonNull;
use windows::utils::WeakRef;
use windows::Win32::Graphics::Direct3D12::*;

#[repr(transparent)]
pub struct Allocation(pub(crate) NonNull<c_void>);

impl Allocation {
    #[inline(always)]
    pub fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr() as *const _
    }

    #[inline(always)]
    pub fn GetOffset(&self) -> u64 {
        unsafe { D3D12MA_Allocation_GetOffset(self.0.as_ptr()) }
    }

    #[inline(always)]
    pub fn GetAlignment(&self) -> u64 {
        unsafe { D3D12MA_Allocation_GetAlignment(self.0.as_ptr()) }
    }

    #[inline(always)]
    pub fn GetSize(&self) -> u64 {
        unsafe { D3D12MA_Allocation_GetSize(self.0.as_ptr()) }
    }

    #[inline(always)]
    pub fn GetResource(&self) -> Option<ID3D12Resource> {
        unsafe { D3D12MA_Allocation_GetResource(self.0.as_ptr()) }
    }

    #[inline(always)]
    pub fn SetResource(&mut self, pResource: WeakRef<ID3D12Resource>) {
        unsafe { D3D12MA_Allocation_SetResource(self.0.as_ptr(), pResource) }
    }

    #[inline(always)]
    pub fn GetHeap(&self) -> Option<ID3D12Heap> {
        unsafe { D3D12MA_Allocation_GetHeap(self.0.as_ptr()) }
    }

    #[inline(always)]
    pub fn SetPrivateData(&mut self, pPrivateData: *mut c_void) {
        unsafe { D3D12MA_Allocation_SetPrivateData(self.0.as_ptr(), pPrivateData) }
    }

    #[inline(always)]
    pub fn GetPrivateData(&self) -> *mut c_void {
        unsafe { D3D12MA_Allocation_GetPrivateData(self.0.as_ptr()) }
    }

    #[inline(always)]
    pub fn SetName(&mut self, Name: *const u16) {
        unsafe { D3D12MA_Allocation_SetName(self.0.as_ptr(), Name) }
    }

    #[inline(always)]
    pub fn GetName(&self) -> *const u16 {
        unsafe { D3D12MA_Allocation_GetName(self.0.as_ptr()) }
    }

    #[inline(always)]
    pub fn WasZeroInitialized(&self) -> bool {
        unsafe { D3D12MA_Allocation_WasZeroInitialized(self.0.as_ptr()).as_bool() }
    }
}

impl Drop for Allocation {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe {
            D3D12MA_Allocation_Release(self.0.as_ptr());
        }
    }
}
