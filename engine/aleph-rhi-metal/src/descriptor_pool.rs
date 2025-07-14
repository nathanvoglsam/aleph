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
use std::sync::Arc;

use aleph_any::{AnyArc, declare_interfaces};
use aleph_object_system::ArcedObject;
use aleph_rhi_api::*;

use crate::descriptor_set_layout::DescriptorSetLayout;
use crate::device::Device;

pub struct DescriptorPool {
    pub(crate) _device: AnyArc<Device>,
    pub(crate) _layout: Arc<ArcedObject<DescriptorSetLayout>>,
}

declare_interfaces!(DescriptorPool, [IDescriptorPool]);

impl IGetPlatformInterface for DescriptorPool {
    unsafe fn __query_platform_interface(&self, target: TypeId, out: *mut ()) -> Option<()> {
        todo!()
    }
}

impl IDescriptorPool for DescriptorPool {
    fn allocate_set(&mut self) -> Result<DescriptorSetHandle, DescriptorPoolAllocateError> {
        todo!()
    }

    fn allocate_sets(
        &mut self,
        num_sets: usize,
    ) -> Result<Box<[DescriptorSetHandle]>, DescriptorPoolAllocateError> {
        todo!()
    }

    unsafe fn free(&mut self, sets: &[DescriptorSetHandle]) {
        todo!()
    }

    unsafe fn reset(&mut self) {
        todo!()
    }
}

impl Drop for DescriptorPool {
    fn drop(&mut self) {
        todo!()
    }
}
