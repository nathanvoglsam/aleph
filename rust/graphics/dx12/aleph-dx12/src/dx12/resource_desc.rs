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

use crate::{dxgi, ResourceDimension, ResourceFlags, TextureLayout};
use raw::windows::win32::direct3d12::D3D12_RESOURCE_DESC;

pub struct ResourceDescBuilder {
    inner: ResourceDesc,
}

impl ResourceDescBuilder {
    pub fn new() -> Self {
        Self {
            inner: ResourceDesc::default(),
        }
    }

    pub fn dimension(mut self, dimension: ResourceDimension) -> Self {
        self.inner.dimension = dimension;
        self
    }

    pub fn alignment(mut self, alignment: u64) -> Self {
        self.inner.alignment = alignment;
        self
    }

    pub fn width(mut self, width: u64) -> Self {
        self.inner.width = width;
        self
    }

    pub fn height(mut self, height: u32) -> Self {
        self.inner.height = height;
        self
    }

    pub fn depth_or_array_size(mut self, depth_or_array_size: u16) -> Self {
        self.inner.depth_or_array_size = depth_or_array_size;
        self
    }

    pub fn mip_levels(mut self, mip_levels: u16) -> Self {
        self.inner.mip_levels = mip_levels;
        self
    }

    pub fn format(mut self, format: dxgi::Format) -> Self {
        self.inner.format = format;
        self
    }

    pub fn sample_desc(mut self, sample_desc: dxgi::SampleDesc) -> Self {
        self.inner.sample_desc = sample_desc;
        self
    }

    pub fn layout(mut self, layout: TextureLayout) -> Self {
        self.inner.layout = layout;
        self
    }

    pub fn flags(mut self, flags: ResourceFlags) -> Self {
        self.inner.flags |= flags;
        self
    }

    pub fn build(self) -> ResourceDesc {
        self.inner
    }
}

#[derive(Clone, Debug)]
pub struct ResourceDesc {
    pub dimension: ResourceDimension,
    pub alignment: u64,
    pub width: u64,
    pub height: u32,
    pub depth_or_array_size: u16,
    pub mip_levels: u16,
    pub format: dxgi::Format,
    pub sample_desc: dxgi::SampleDesc,
    pub layout: TextureLayout,
    pub flags: ResourceFlags,
}

impl ResourceDesc {
    pub fn builder() -> ResourceDescBuilder {
        ResourceDescBuilder::new()
    }
}

impl Default for ResourceDesc {
    fn default() -> Self {
        Self {
            dimension: ResourceDimension::Unknown,
            alignment: 0,
            width: 0,
            height: 1,
            depth_or_array_size: 1,
            mip_levels: 1,
            format: Default::default(),
            sample_desc: Default::default(),
            layout: TextureLayout::RowMajor,
            flags: ResourceFlags::NONE,
        }
    }
}

impl Into<D3D12_RESOURCE_DESC> for ResourceDesc {
    fn into(self) -> D3D12_RESOURCE_DESC {
        D3D12_RESOURCE_DESC {
            dimension: self.dimension.into(),
            alignment: self.alignment,
            width: self.width,
            height: self.height,
            depth_or_array_size: self.depth_or_array_size,
            mip_levels: self.mip_levels,
            format: self.format.into(),
            sample_desc: self.sample_desc.into(),
            layout: self.layout.into(),
            flags: self.flags.into(),
        }
    }
}
