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

use crate::raw::windows::win32::direct3d12::{ID3D12Resource, D3D12_RANGE};
use std::ops::Range;
use std::ptr::NonNull;

#[derive(Clone)]
#[repr(transparent)]
pub struct Resource(pub(crate) ID3D12Resource);

impl Resource {
    pub fn get_gpu_virtual_address(&self) -> u64 {
        unsafe { self.0.GetGPUVirtualAddress() }
    }

    pub fn map(
        &self,
        subresource: u32,
        read_range: Option<Range<usize>>,
    ) -> raw::windows::Result<Option<NonNull<u8>>> {
        unsafe {
            let mut out = std::ptr::null_mut();
            if let Some(read_range) = read_range {
                let read_range = D3D12_RANGE {
                    begin: read_range.start,
                    end: read_range.end,
                };
                self.0
                    .Map(subresource, &read_range, &mut out)
                    .ok()
                    .map(|_| NonNull::new(out as *mut u8))
            } else {
                self.0
                    .Map(subresource, std::ptr::null(), &mut out)
                    .ok()
                    .map(|_| NonNull::new(out as *mut u8))
            }
        }
    }

    pub fn unmap(&self, subresource: u32, written_range: Option<Range<usize>>) {
        unsafe {
            if let Some(written_range) = written_range {
                let written_range = D3D12_RANGE {
                    begin: written_range.start,
                    end: written_range.end,
                };
                self.0.Unmap(subresource, &written_range)
            } else {
                self.0.Unmap(subresource, std::ptr::null())
            }
        }
    }
}

crate::object_impl!(Resource);
crate::device_child_impl!(Resource);
