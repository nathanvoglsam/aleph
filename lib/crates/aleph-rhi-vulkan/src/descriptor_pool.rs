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

use crate::descriptor_set_layout::DescriptorSetLayout;
use crate::device::Device;
use aleph_any::{declare_interfaces, AnyArc};
use aleph_rhi_api::*;
use aleph_rhi_impl_utils::try_clone_value_into_slot;
use ash::prelude::VkResult;
use ash::vk;
use ash::vk::Handle;
use std::any::TypeId;

pub struct DescriptorPool {
    pub(crate) _device: AnyArc<Device>,
    pub(crate) _layout: AnyArc<DescriptorSetLayout>,
    pub(crate) descriptor_pool: vk::DescriptorPool,
}

declare_interfaces!(DescriptorPool, [IDescriptorPool]);

impl IGetPlatformInterface for DescriptorPool {
    unsafe fn __query_platform_interface(&self, target: TypeId, out: *mut ()) -> Option<()> {
        try_clone_value_into_slot(&self.descriptor_pool, out, target)
    }
}

impl DescriptorPool {
    pub fn handle_allocate_result(
        v: VkResult<Vec<vk::DescriptorSet>>,
    ) -> Result<Vec<vk::DescriptorSet>, DescriptorPoolAllocateError> {
        match v {
            Ok(v) => Ok(v),
            Err(e) => match e {
                vk::Result::ERROR_OUT_OF_POOL_MEMORY => {
                    Err(DescriptorPoolAllocateError::OutOfPoolMemory)
                }
                vk::Result::ERROR_OUT_OF_DEVICE_MEMORY | vk::Result::ERROR_OUT_OF_HOST_MEMORY => {
                    Err(DescriptorPoolAllocateError::OutOfMemory)
                }
                vk::Result::ERROR_FRAGMENTED_POOL => {
                    Err(DescriptorPoolAllocateError::FragmentedPool)
                }
                _ => {
                    log::error!("Platform Error: {:#?}", e);
                    Err(DescriptorPoolAllocateError::Platform)
                }
            },
        }
    }
}

impl IDescriptorPool for DescriptorPool {
    fn allocate_set(&mut self) -> Result<DescriptorSetHandle, DescriptorPoolAllocateError> {
        let set_layouts = [self._layout.descriptor_set_layout];

        let allocate_info = vk::DescriptorSetAllocateInfo::builder()
            .descriptor_pool(self.descriptor_pool)
            .set_layouts(&set_layouts);
        let descriptor_sets = unsafe {
            let result = self._device.device.allocate_descriptor_sets(&allocate_info);

            Self::handle_allocate_result(result)?
        };
        let descriptor_set = descriptor_sets[0];

        unsafe { Ok(DescriptorSetHandle::from_raw_int(descriptor_set.as_raw()).unwrap()) }
    }

    fn allocate_sets(
        &mut self,
        num_sets: usize,
    ) -> Result<Vec<DescriptorSetHandle>, DescriptorPoolAllocateError> {
        let mut set_layouts = Vec::with_capacity(num_sets);
        for _ in 0..num_sets {
            set_layouts.push(self._layout.descriptor_set_layout);
        }

        let allocate_info = vk::DescriptorSetAllocateInfo::builder()
            .descriptor_pool(self.descriptor_pool)
            .set_layouts(&set_layouts);
        let descriptor_sets = unsafe {
            let result = self._device.device.allocate_descriptor_sets(&allocate_info);

            Self::handle_allocate_result(result)?
        };

        unsafe { Ok(core::mem::transmute(descriptor_sets.to_vec())) }
    }

    unsafe fn free(&mut self, sets: &[DescriptorSetHandle]) {
        let descriptor_sets =
            core::slice::from_raw_parts(sets.as_ptr() as *const vk::DescriptorSet, sets.len());
        self._device
            .device
            .free_descriptor_sets(self.descriptor_pool, descriptor_sets)
            .unwrap()
    }

    unsafe fn reset(&mut self) {
        self._device
            .device
            .reset_descriptor_pool(self.descriptor_pool, Default::default())
            .unwrap();
    }
}

impl Drop for DescriptorPool {
    fn drop(&mut self) {
        unsafe {
            self._device
                .device
                .destroy_descriptor_pool(self.descriptor_pool, None);
        }
    }
}
