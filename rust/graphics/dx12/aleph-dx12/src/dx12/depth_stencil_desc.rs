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

use crate::{ComparisonFunc, DepthStencilOpDesc, DepthWriteMask};
use raw::windows::win32::direct3d12::D3D12_DEPTH_STENCIL_DESC;

#[derive(Clone, Debug, Hash)]
pub struct DepthStencilDesc {
    pub depth_enable: bool,
    pub depth_write_mask: DepthWriteMask,
    pub depth_func: ComparisonFunc,
    pub stencil_enable: bool,
    pub stencil_read_mask: u8,
    pub stencil_write_mask: u8,
    pub front_face: DepthStencilOpDesc,
    pub back_face: DepthStencilOpDesc,
}

impl Default for DepthStencilDesc {
    fn default() -> Self {
        Self {
            depth_enable: true,
            depth_write_mask: DepthWriteMask::All,
            depth_func: ComparisonFunc::Less,
            stencil_enable: false,
            stencil_read_mask: 0xFF,
            stencil_write_mask: 0xFF,
            front_face: Default::default(),
            back_face: Default::default(),
        }
    }
}

impl Into<D3D12_DEPTH_STENCIL_DESC> for DepthStencilDesc {
    fn into(self) -> D3D12_DEPTH_STENCIL_DESC {
        D3D12_DEPTH_STENCIL_DESC {
            depth_enable: self.depth_enable.into(),
            depth_write_mask: self.depth_write_mask.into(),
            depth_func: self.depth_func.into(),
            stencil_enable: self.stencil_enable.into(),
            stencil_read_mask: self.stencil_read_mask.into(),
            stencil_write_mask: self.stencil_write_mask.into(),
            front_face: self.front_face.into(),
            back_face: self.back_face.into(),
        }
    }
}
