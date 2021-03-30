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

use std::ffi::c_void;
use std::mem::ManuallyDrop;
use windows_raw::Win32::Direct3D12::D3D12_PIPELINE_STATE_SUBOBJECT_TYPE;

#[repr(transparent)]
pub(crate) struct PackedPipelineStateStreamObject<T>(AlignmentWrapper<Packed<T>>);

impl<T> PackedPipelineStateStreamObject<T> {
    #[inline]
    pub fn new(object_type: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE, object: T) -> Self {
        let out = AlignmentWrapper {
            wrapped: ManuallyDrop::new(Packed {
                object_type,
                object,
            }),
        };
        Self(out)
    }
}

#[repr(C)]
struct Packed<T> {
    pub object_type: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE,
    pub object: T,
}

#[repr(C)]
union AlignmentWrapper<T> {
    wrapped: ManuallyDrop<T>,
    alignment: *const c_void,
}

impl<T> Drop for AlignmentWrapper<T> {
    #[inline]
    fn drop(&mut self) {
        let _drop = unsafe { ManuallyDrop::take(&mut self.wrapped) };
    }
}
