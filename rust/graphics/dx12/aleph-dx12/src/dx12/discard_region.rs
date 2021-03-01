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

use crate::utils::optional_slice_to_num_ptr_pair;
use crate::Rect;
use raw::windows::win32::direct3d12::D3D12_DISCARD_REGION;

#[derive(Clone, Debug)]
#[repr(C)]
pub struct DiscardRegion<'a> {
    pub rects: Option<&'a [Rect]>,
    pub first_subresource: u32,
    pub num_subresources: u32,
}

impl<'a> Into<D3D12_DISCARD_REGION> for DiscardRegion<'a> {
    fn into(self) -> D3D12_DISCARD_REGION {
        let (num_rects, p_rects) = optional_slice_to_num_ptr_pair(self.rects);
        D3D12_DISCARD_REGION {
            num_rects,
            p_rects: p_rects as *mut _,
            first_subresource: self.first_subresource,
            num_subresources: self.num_subresources,
        }
    }
}
