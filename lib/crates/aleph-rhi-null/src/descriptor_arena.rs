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

use std::cell::Cell;

use aleph_any::{declare_interfaces, AnyArc};
use aleph_rhi_api::*;

use crate::NullDevice;

pub struct NullDescriptorArena {
    pub(crate) _device: AnyArc<NullDevice>,
    pub(crate) counter: Cell<u64>,
}

declare_interfaces!(NullDescriptorArena, [IDescriptorArena]);

crate::impl_platform_interface_passthrough!(NullDescriptorArena);

impl IDescriptorArena for NullDescriptorArena {
    fn allocate_set(
        &self,
        _layout: &dyn IDescriptorSetLayout,
    ) -> Result<DescriptorSetHandle, DescriptorPoolAllocateError> {
        let handle = self.counter.get();
        self.counter.set(self.counter.get() + 1);
        Ok(unsafe { DescriptorSetHandle::from_raw_int(handle).unwrap() })
    }

    unsafe fn free(&self, _sets: &[DescriptorSetHandle]) {
        // Intentionally do nothing
    }

    unsafe fn reset(&self) {
        self.counter.set(1);
    }
}
