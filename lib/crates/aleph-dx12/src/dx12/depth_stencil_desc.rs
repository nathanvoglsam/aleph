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

use crate::{Bool, ComparisonFunc, DepthStencilOpDesc, DepthWriteMask};

pub struct DepthStencilDescBuilder {
    inner: DepthStencilDesc,
}

impl DepthStencilDescBuilder {
    #[inline]
    pub fn new() -> Self {
        Self {
            inner: DepthStencilDesc::default(),
        }
    }

    #[inline]
    pub fn depth_enable(mut self, depth_enable: bool) -> Self {
        self.inner.depth_enable = depth_enable.into();
        self
    }

    #[inline]
    pub fn depth_write_mask(mut self, depth_write_mask: DepthWriteMask) -> Self {
        self.inner.depth_write_mask = depth_write_mask;
        self
    }

    #[inline]
    pub fn depth_func(mut self, depth_func: ComparisonFunc) -> Self {
        self.inner.depth_func = depth_func;
        self
    }

    #[inline]
    pub fn stencil_enable(mut self, stencil_enable: bool) -> Self {
        self.inner.stencil_enable = stencil_enable.into();
        self
    }

    #[inline]
    pub fn stencil_read_mask(mut self, stencil_read_mask: u8) -> Self {
        self.inner.stencil_read_mask = stencil_read_mask;
        self
    }

    #[inline]
    pub fn stencil_write_mask(mut self, stencil_write_mask: u8) -> Self {
        self.inner.stencil_write_mask = stencil_write_mask;
        self
    }

    #[inline]
    pub fn front_face(mut self, front_face: DepthStencilOpDesc) -> Self {
        self.inner.front_face = front_face;
        self
    }

    #[inline]
    pub fn back_face(mut self, back_face: DepthStencilOpDesc) -> Self {
        self.inner.back_face = back_face;
        self
    }

    #[inline]
    pub fn build(self) -> DepthStencilDesc {
        self.inner
    }
}

impl Default for DepthStencilDescBuilder {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[repr(C)]
#[derive(Clone, Debug, Hash)]
pub struct DepthStencilDesc {
    pub depth_enable: Bool,
    pub depth_write_mask: DepthWriteMask,
    pub depth_func: ComparisonFunc,
    pub stencil_enable: Bool,
    pub stencil_read_mask: u8,
    pub stencil_write_mask: u8,
    pub front_face: DepthStencilOpDesc,
    pub back_face: DepthStencilOpDesc,
}

impl DepthStencilDesc {
    #[inline]
    pub fn builder() -> DepthStencilDescBuilder {
        DepthStencilDescBuilder::new()
    }
}

impl Default for DepthStencilDesc {
    #[inline]
    fn default() -> Self {
        Self {
            depth_enable: true.into(),
            depth_write_mask: DepthWriteMask::All,
            depth_func: ComparisonFunc::Less,
            stencil_enable: false.into(),
            stencil_read_mask: 0xFF,
            stencil_write_mask: 0xFF,
            front_face: Default::default(),
            back_face: Default::default(),
        }
    }
}
