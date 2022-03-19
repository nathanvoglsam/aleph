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

use crate::{DescriptorHeapFlags, DescriptorHeapType};

#[derive(Clone, Debug)]
pub struct DescriptorHeapDescBuilder {
    heap_type: Option<DescriptorHeapType>,
    num_descriptors: u32,
    flags: DescriptorHeapFlags,
    node_mask: u32,
}

impl DescriptorHeapDescBuilder {
    #[inline]
    pub fn new() -> Self {
        Self {
            heap_type: None,
            num_descriptors: 0,
            flags: DescriptorHeapFlags::NONE,
            node_mask: 0,
        }
    }

    #[inline]
    pub fn heap_type(mut self, heap_type: DescriptorHeapType) -> Self {
        self.heap_type = Some(heap_type);
        self
    }

    #[inline]
    pub fn num_descriptors(mut self, num_descriptors: u32) -> Self {
        self.num_descriptors = num_descriptors;
        self
    }

    #[inline]
    pub fn flags(mut self, flags: DescriptorHeapFlags) -> Self {
        self.flags |= flags;
        self
    }

    #[inline]
    pub fn node_mask(mut self, node_mask: u32) -> Self {
        self.node_mask = node_mask;
        self
    }

    #[inline]
    pub fn build(self) -> DescriptorHeapDesc {
        DescriptorHeapDesc {
            heap_type: self.heap_type.unwrap(),
            num_descriptors: self.num_descriptors,
            flags: self.flags,
            node_mask: self.node_mask,
        }
    }
}

impl Default for DescriptorHeapDescBuilder {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[repr(C)]
#[derive(Clone, Debug, Hash)]
pub struct DescriptorHeapDesc {
    pub heap_type: DescriptorHeapType,
    pub num_descriptors: u32,
    pub flags: DescriptorHeapFlags,
    pub node_mask: u32,
}

impl DescriptorHeapDesc {
    #[inline]
    pub fn builder() -> DescriptorHeapDescBuilder {
        DescriptorHeapDescBuilder::new()
    }
}
