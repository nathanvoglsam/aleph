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

use dx12::{Heap, Resource};
use std::ffi::c_void;
use std::mem::transmute;
use std::ptr::NonNull;
use std::sync::Arc;

pub(crate) struct AllocationInner(pub(crate) NonNull<c_void>);

impl Drop for AllocationInner {
    fn drop(&mut self) {
        unsafe {
            alloc_raw::D3D12MA_Allocation_Release(self.0.as_ptr());
        }
    }
}

#[repr(transparent)]
pub struct Allocation(pub(crate) Arc<AllocationInner>);

impl Allocation {
    pub fn get_resource(&self) -> Option<Resource> {
        unsafe {
            alloc_raw::D3D12MA_Allocation_GetResource(self.0 .0.as_ptr()).map(|v| transmute(v))
        }
    }

    pub fn get_heap(&self) -> Option<Heap> {
        unsafe { alloc_raw::D3D12MA_Allocation_GetHeap(self.0 .0.as_ptr()).map(|v| transmute(v)) }
    }
}
