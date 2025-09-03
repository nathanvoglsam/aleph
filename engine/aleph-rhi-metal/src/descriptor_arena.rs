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

use std::any::TypeId;

use aleph_any::{AnyArc, declare_interfaces};
use aleph_rhi_api::*;
use blink_alloc::Blink;

use crate::device::Device;
use crate::internal::unwrap;

pub struct DescriptorArena {
    pub(crate) _device: AnyArc<Device>,
    pub(crate) arena: Blink,
}

declare_interfaces!(DescriptorArena, [IDescriptorArena]);

impl IGetPlatformInterface for DescriptorArena {
    unsafe fn __query_platform_interface(&self, _target: TypeId, _out: *mut ()) -> Option<()> {
        todo!()
    }
}

impl IDescriptorArena for DescriptorArena {
    fn allocate_block(
        &self,
        layout: &dyn IParameterBlockLayout,
    ) -> Result<ParameterBlockHandle, DescriptorAllocateError> {
        let layout = unwrap::parameter_block_layout(layout);
        todo!()
    }

    fn allocate_blocks(
        &self,
        layout: &dyn IParameterBlockLayout,
        num_blocks: usize,
    ) -> Result<Box<[ParameterBlockHandle]>, DescriptorAllocateError> {
        let layout = unwrap::parameter_block_layout(layout);
        todo!()
    }

    unsafe fn free(&self, blocks: &[ParameterBlockHandle]) {
        todo!()
    }

    unsafe fn reset(&self) {
        todo!()
    }
}
