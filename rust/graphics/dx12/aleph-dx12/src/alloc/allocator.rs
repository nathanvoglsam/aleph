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

use crate::{raw, DXGIAdapter, Device};
use std::ffi::c_void;
use std::sync::Arc;

struct AllocatorInner(*mut c_void);

impl Drop for AllocatorInner {
    fn drop(&mut self) {
        unsafe {
            alloc_raw::D3D12MA_Allocator_Release(self.0);
        }
    }
}

pub struct AllocatorBuilder<'a, 'b> {
    pub(crate) device: &'a Device,
    pub(crate) adapter: &'b DXGIAdapter,
    pub(crate) flags: alloc_raw::AllocatorFlags,
    pub(crate) preferred_block_size: u64,
}

impl<'a, 'b> AllocatorBuilder<'a, 'b> {
    pub fn always_committed(mut self) -> Self {
        self.flags.0 |= alloc_raw::AllocatorFlags::ALWAYS_COMMITTED.0;
        self
    }

    pub unsafe fn build(self) -> raw::windows::Result<Allocator> {
        let desc = alloc_raw::AllocatorDesc {
            flags: self.flags,
            p_device: std::mem::transmute_copy(&self.device.0),
            preferred_block_size: self.preferred_block_size,
            p_allocation_callbacks: std::ptr::null(),
            p_adapter: std::mem::transmute_copy(&self.adapter.0),
        };
        let mut out = std::ptr::null_mut();
        alloc_raw::D3D12MA_Allocator_CreateAllocator(&desc, &mut out)
            .ok()
            .map(|_| {
                assert!(!out.is_null());
                let out = AllocatorInner(out);
                let out = Allocator(Arc::new(out));
                out
            })
    }
}

#[derive(Clone)]
#[repr(transparent)]
pub struct Allocator(Arc<AllocatorInner>);

impl Allocator {}
