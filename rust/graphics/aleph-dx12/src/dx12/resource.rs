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

use crate::utils::optional_ref_to_ptr;
use crate::windows_raw::win32::direct3d12::{ID3D12Resource, D3D12_RANGE};
use crate::GPUDescriptorHandle;
use std::num::NonZeroU64;
use std::ops::Range;
use std::ptr::NonNull;

#[derive(Clone)]
#[repr(transparent)]
pub struct Resource(pub(crate) ID3D12Resource);

impl Resource {
    pub fn get_gpu_virtual_address(&self) -> Option<GPUDescriptorHandle> {
        unsafe {
            let ptr = self.0.GetGPUVirtualAddress();
            NonZeroU64::new(ptr).map(|v| GPUDescriptorHandle(v))
        }
    }

    pub fn write_to_subresource(
        &self,
        dst_subresource: u32,
        dst_box: Option<&crate::Box>,
        src_data: &[u8],
        src_row_pitch: u32,
        src_depth_pitch: u32,
    ) -> crate::Result<()> {
        // TODO: input size validation on src_row_pitch and src_depth_pitch
        unsafe {
            let dst_box = optional_ref_to_ptr(dst_box);
            self.0
                .WriteToSubresource(
                    dst_subresource,
                    dst_box,
                    src_data.as_ptr() as *const _,
                    src_row_pitch,
                    src_depth_pitch,
                )
                .ok()
        }
    }

    pub fn read_from_subresource(
        &self,
        dst_data: &mut [u8],
        dst_row_pitch: u32,
        dst_depth_pitch: u32,
        src_subresource: u32,
        src_box: Option<&crate::Box>,
    ) -> crate::Result<()> {
        // TODO: input size validation on dst_row_pitch and dst_depth_pitch
        unsafe {
            let src_box = optional_ref_to_ptr(src_box);
            self.0
                .ReadFromSubresource(
                    dst_data.as_mut_ptr() as *mut _,
                    dst_row_pitch,
                    dst_depth_pitch,
                    src_subresource,
                    src_box,
                )
                .ok()
        }
    }

    pub fn map(
        &self,
        subresource: u32,
        read_range: Option<Range<usize>>,
    ) -> crate::Result<Option<NonNull<u8>>> {
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
