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

use std::num::NonZeroU64;
use std::sync::Arc;

use aleph_any::AnyArc;
use aleph_object_system::{ArcedObject, unsafe_impl_iobject};
use aleph_rhi_api::*;
use ash::vk;

use crate::device::Device;
use crate::sampler::Sampler;

pub struct DescriptorSetLayout {
    pub(crate) _device: AnyArc<Device>,
    pub(crate) _samplers: Vec<Arc<ArcedObject<Sampler>>>,
    pub(crate) id: NonZeroU64,
    pub(crate) descriptor_set_layout: vk::DescriptorSetLayout,
    pub(crate) pool_sizes: Vec<vk::DescriptorPoolSize>,
}

unsafe_impl_iobject!(DescriptorSetLayout, "01944fe5-3abc-7a62-8c14-19d4d31f92b6");

impl DescriptorSetLayout {
    pub(crate) fn get_owned(v: &DescriptorSetLayoutHandle) -> std::sync::Arc<ArcedObject<Self>> {
        v.clone()
            .into_inner()
            .downcast::<Self>()
            .expect("Unknown DescriptorSetLayout implementation!")
    }

    pub(crate) fn get(v: &DescriptorSetLayoutHandle) -> &Self {
        v.get()
            .downcast_ref::<Self>()
            .expect("Unknown DescriptorSetLayout implementation!")
    }
}

impl Drop for DescriptorSetLayout {
    fn drop(&mut self) {
        unsafe {
            self._device
                .device
                .destroy_descriptor_set_layout(self.descriptor_set_layout, None);
        }
    }
}
