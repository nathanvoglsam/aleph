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
    ID3D12DescriptorHeap, ID3D12DeviceChild, ID3D12Object, D3D12_DESCRIPTOR_HEAP_DESC,
    D3D12_DESCRIPTOR_HEAP_FLAGS, D3D12_DESCRIPTOR_HEAP_TYPE,
};
use crate::raw::windows::{Abi, Interface};
use crate::Device;

/// Wrapper for `D3D12_COMMAND_LIST_TYPE`
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub enum DescriptorHeapType {
    CbvSrvUav,
    Sampler,
    RenderTargetView,
    DepthStencilView,
}

impl Into<D3D12_DESCRIPTOR_HEAP_TYPE> for DescriptorHeapType {
    fn into(self) -> D3D12_DESCRIPTOR_HEAP_TYPE {
        match self {
            DescriptorHeapType::CbvSrvUav => {
                D3D12_DESCRIPTOR_HEAP_TYPE::D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV
            }
            DescriptorHeapType::Sampler => {
                D3D12_DESCRIPTOR_HEAP_TYPE::D3D12_DESCRIPTOR_HEAP_TYPE_SAMPLER
            }
            DescriptorHeapType::RenderTargetView => {
                D3D12_DESCRIPTOR_HEAP_TYPE::D3D12_DESCRIPTOR_HEAP_TYPE_RTV
            }
            DescriptorHeapType::DepthStencilView => {
                D3D12_DESCRIPTOR_HEAP_TYPE::D3D12_DESCRIPTOR_HEAP_TYPE_DSV
            }
        }
    }
}

pub struct DescriptorHeapBuilder {
    pub heap_type: Option<DescriptorHeapType>,
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
        self.heap_type = Some(heap_type);
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
        let heap_type = self.heap_type.unwrap();
        let desc = D3D12_DESCRIPTOR_HEAP_DESC {
            r#type: heap_type.into(),
            num_descriptors: self.num_descriptors,
            flags: self.flags,
            node_mask: 0,
        };
        let mut heap: Option<ID3D12DescriptorHeap> = None;
        device
            .0
            .CreateDescriptorHeap(&desc, &ID3D12DescriptorHeap::IID, heap.set_abi())
            .and_some(heap)
            .map(|v| DescriptorHeap(v))
    }
}

#[derive(Clone)]
#[repr(transparent)]
pub struct DescriptorHeap(pub(crate) ID3D12DescriptorHeap);

impl DescriptorHeap {
    pub fn builder() -> DescriptorHeapBuilder {
        DescriptorHeapBuilder::new()
    }
}

impl Into<ID3D12Object> for DescriptorHeap {
    fn into(self) -> ID3D12Object {
        self.0.into()
    }
}

impl Into<ID3D12DeviceChild> for DescriptorHeap {
    fn into(self) -> ID3D12DeviceChild {
        self.0.into()
    }
}
