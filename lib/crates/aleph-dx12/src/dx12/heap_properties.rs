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

use crate::{CpuPageProperty, HeapType, MemoryPool};
use windows::Win32::Graphics::Direct3D12::D3D12_HEAP_PROPERTIES;

// D3D12_CPU_PAGE_PROPERTY CPUPageProperty;
// D3D12_MEMORY_POOL MemoryPoolPreference;

#[repr(C)]
#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Debug, Hash)]
pub struct HeapProperties {
    pub r#type: HeapType,
    pub cpu_page_property: CpuPageProperty,
    pub memory_pool_preference: MemoryPool,
    pub creation_node_mask: u32,
    pub visible_node_mask: u32,
}

impl From<HeapProperties> for D3D12_HEAP_PROPERTIES {
    fn from(v: HeapProperties) -> Self {
        Self {
            Type: v.r#type.into(),
            CPUPageProperty: v.cpu_page_property.into(),
            MemoryPoolPreference: v.memory_pool_preference.into(),
            CreationNodeMask: v.creation_node_mask,
            VisibleNodeMask: v.visible_node_mask,
        }
    }
}
