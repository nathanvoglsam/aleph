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

use crate::raw::windows::win32::direct3d12::D3D12_DESCRIPTOR_HEAP_DESC;
use crate::{DescriptorHeapFlags, DescriptorHeapType};

pub struct DescriptorHeapDescBuilder {
    heap_type: Option<DescriptorHeapType>,
    num_descriptors: u32,
    flags: DescriptorHeapFlags,
}

impl DescriptorHeapDescBuilder {
    pub fn new() -> Self {
        Self {
            heap_type: None,
            num_descriptors: 0,
            flags: DescriptorHeapFlags::NONE,
        }
    }

    pub fn heap_type(mut self, heap_type: DescriptorHeapType) -> Self {
        self.heap_type = Some(heap_type);
        self
    }

    pub fn num_descriptors(mut self, num_descriptors: u32) -> Self {
        self.num_descriptors = num_descriptors;
        self
    }

    pub fn shader_visible(mut self) -> Self {
        self.flags.0 |= DescriptorHeapFlags::SHADER_VISIBLE.0;
        self
    }

    pub fn build(self) -> DescriptorHeapDesc {
        DescriptorHeapDesc {
            heap_type: self.heap_type.unwrap(),
            num_descriptors: self.num_descriptors,
            flags: self.flags,
        }
    }
}

#[derive(Clone, Debug, Hash)]
pub struct DescriptorHeapDesc {
    pub heap_type: DescriptorHeapType,
    pub num_descriptors: u32,
    pub flags: DescriptorHeapFlags,
}

impl DescriptorHeapDesc {
    pub fn builder() -> DescriptorHeapDescBuilder {
        DescriptorHeapDescBuilder::new()
    }
}

impl Into<D3D12_DESCRIPTOR_HEAP_DESC> for DescriptorHeapDesc {
    fn into(self) -> D3D12_DESCRIPTOR_HEAP_DESC {
        D3D12_DESCRIPTOR_HEAP_DESC {
            r#type: self.heap_type.into(),
            num_descriptors: self.num_descriptors,
            flags: self.flags.into(),
            node_mask: 0,
        }
    }
}
