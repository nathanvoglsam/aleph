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

use aleph_any::{declare_interfaces, AnyArc};
use aleph_rhi_api::*;
use aleph_rhi_impl_utils::try_clone_value_into_slot;
use ash::vk;
use ash::vk::Handle;

use crate::descriptor_pool::DescriptorPool;
use crate::device::Device;
use crate::internal::unwrap;

pub struct DescriptorArena {
    pub(crate) _device: AnyArc<Device>,
    pub(crate) descriptor_pool: vk::DescriptorPool,
}

declare_interfaces!(DescriptorArena, [IDescriptorArena]);

impl IGetPlatformInterface for DescriptorArena {
    unsafe fn __query_platform_interface(&self, target: TypeId, out: *mut ()) -> Option<()> {
        try_clone_value_into_slot(&self.descriptor_pool, out, target)
    }
}

impl IDescriptorArena for DescriptorArena {
    fn allocate_set(
        &self,
        layout: &dyn IDescriptorSetLayout,
    ) -> Result<DescriptorSetHandle, DescriptorPoolAllocateError> {
        let layout = unwrap::descriptor_set_layout(layout);
        let set_layouts = [layout.descriptor_set_layout];

        let allocate_info = vk::DescriptorSetAllocateInfo::builder()
            .descriptor_pool(self.descriptor_pool)
            .set_layouts(&set_layouts);
        let descriptor_sets = unsafe {
            let result = self._device.device.allocate_descriptor_sets(&allocate_info);

            DescriptorPool::handle_allocate_result(result)?
        };
        let descriptor_set = descriptor_sets[0];

        unsafe { Ok(DescriptorSetHandle::from_raw_int(descriptor_set.as_raw()).unwrap()) }
    }

    fn allocate_sets(
        &self,
        layout: &dyn IDescriptorSetLayout,
        num_sets: usize,
    ) -> Result<Vec<DescriptorSetHandle>, DescriptorPoolAllocateError> {
        let layout = unwrap::descriptor_set_layout(layout);
        let mut set_layouts = Vec::with_capacity(num_sets);
        for _ in 0..num_sets {
            set_layouts.push(layout.descriptor_set_layout);
        }

        let allocate_info = vk::DescriptorSetAllocateInfo::builder()
            .descriptor_pool(self.descriptor_pool)
            .set_layouts(&set_layouts);
        let descriptor_sets = unsafe {
            let result = self._device.device.allocate_descriptor_sets(&allocate_info);

            DescriptorPool::handle_allocate_result(result)?
        };

        unsafe { Ok(core::mem::transmute(descriptor_sets.to_vec())) }
    }

    unsafe fn free(&self, sets: &[DescriptorSetHandle]) {
        let descriptor_sets =
            core::slice::from_raw_parts(sets.as_ptr() as *const vk::DescriptorSet, sets.len());
        self._device
            .device
            .free_descriptor_sets(self.descriptor_pool, descriptor_sets)
            .unwrap()
    }

    unsafe fn reset(&self) {
        self._device
            .device
            .reset_descriptor_pool(self.descriptor_pool, Default::default())
            .unwrap();
    }
}

impl Drop for DescriptorArena {
    fn drop(&mut self) {
        unsafe {
            self._device
                .device
                .destroy_descriptor_pool(self.descriptor_pool, None);
        }
    }
}
