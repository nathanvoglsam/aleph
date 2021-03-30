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

use crate::{CPUDescriptorHandle, DescriptorHeapDesc, GPUDescriptorHandle};
use std::convert::TryFrom;
use std::mem::transmute;
use windows_raw::Win32::Direct3D12::ID3D12DescriptorHeap;

#[repr(transparent)]
pub struct DescriptorHeap(pub(crate) ID3D12DescriptorHeap);

impl DescriptorHeap {
    pub fn get_cpu_descriptor_handle_for_heap_start(&self) -> Option<CPUDescriptorHandle> {
        unsafe {
            let out = self.0.GetCPUDescriptorHandleForHeapStart();
            CPUDescriptorHandle::try_from(out).ok()
        }
    }

    pub fn get_gpu_descriptor_handle_for_heap_start(&self) -> Option<GPUDescriptorHandle> {
        unsafe {
            let out = self.0.GetGPUDescriptorHandleForHeapStart();
            GPUDescriptorHandle::try_from(out).ok()
        }
    }

    pub fn get_desc(&self) -> DescriptorHeapDesc {
        unsafe { transmute(self.0.GetDesc()) }
    }
}

crate::object_impl!(DescriptorHeap);
crate::device_child_impl!(DescriptorHeap);
crate::shared_object!(DescriptorHeap);
windows_raw::deref_impl!(DescriptorHeap, ID3D12DescriptorHeap);
