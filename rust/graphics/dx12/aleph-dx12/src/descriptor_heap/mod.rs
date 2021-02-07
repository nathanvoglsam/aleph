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

use crate::raw::windows::win32::direct3d12::{
    ID3D12DescriptorHeap, D3D12_DESCRIPTOR_HEAP_DESC, D3D12_DESCRIPTOR_HEAP_FLAGS,
    D3D12_DESCRIPTOR_HEAP_TYPE,
};
use crate::raw::windows::{Abi, Interface};
use crate::{DescriptorHeapType, Device};

pub struct DescriptorHeapBuilder {
    pub heap_type: DescriptorHeapType,
    pub num_descriptors: u32,
    pub flags: D3D12_DESCRIPTOR_HEAP_FLAGS,
}

impl DescriptorHeapBuilder {
    pub fn new() -> Self {
        Self {
            heap_type: Default::default(),
            num_descriptors: 0,
            flags: Default::default(),
        }
    }

    pub fn heap_type(mut self, heap_type: DescriptorHeapType) -> Self {
        self.heap_type = heap_type;
        self
    }

    pub fn num_descriptors(mut self, num_descriptors: u32) -> Self {
        self.num_descriptors = num_descriptors;
        self
    }

    pub fn shader_visible(mut self) -> Self {
        self.flags.0 |= D3D12_DESCRIPTOR_HEAP_FLAGS::D3D12_DESCRIPTOR_HEAP_FLAG_SHADER_VISIBLE.0;
        self
    }

    pub unsafe fn build(self, device: &Device) -> raw::windows::Result<DescriptorHeap> {
        let desc = D3D12_DESCRIPTOR_HEAP_DESC {
            r#type: self.heap_type.into(),
            num_descriptors: self.num_descriptors,
            flags: self.flags,
            node_mask: 0,
        };
        let mut queue: Option<ID3D12DescriptorHeap> = None;
        device
            .raw()
            .CreateDescriptorHeap(&desc, &ID3D12DescriptorHeap::IID, queue.set_abi())
            .and_some(queue)
            .map(|v| DescriptorHeap { queue: v })
    }
}

pub struct DescriptorHeap {
    queue: ID3D12DescriptorHeap,
}

impl DescriptorHeap {
    pub fn builder() -> DescriptorHeapBuilder {
        DescriptorHeapBuilder::new()
    }

    pub fn raw(&self) -> &ID3D12DescriptorHeap {
        &self.queue
    }
}
