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
use std::sync::{Arc, Weak};

use aleph_alloc::BVec;
use aleph_rhi_api::*;
use aleph_rhi_impl_utils::RhiSystem;
use aleph_rhi_impl_utils::owned_desc::OwnedParameterBlockDesc;
use ash::vk;

use crate::device::Device;
use crate::internal::allocation_callbacks::GLOBAL;
use crate::internal::unwrap;

pub struct ParameterBlockLayout {
    pub(crate) _this: Weak<Self>,
    pub(crate) _device: Arc<Device>,
    pub(crate) id: NonZeroU64,
    pub(crate) descriptor_set_layout: vk::DescriptorSetLayout,
    pub(crate) pool_sizes: BVec<vk::DescriptorPoolSize, RhiSystem>,
    pub(crate) desc: OwnedParameterBlockDesc,
}

impl IParameterBlockLayout for ParameterBlockLayout {
    fn upgrade(&self) -> Arc<dyn IParameterBlockLayout> {
        self._this.upgrade().unwrap()
    }

    fn strong_count(&self) -> usize {
        self._this.strong_count()
    }

    fn weak_count(&self) -> usize {
        self._this.weak_count()
    }

    fn desc(&self) -> &ParameterBlockDesc<'_> {
        self.desc.get()
    }

    fn get_id(&self) -> NonZeroU64 {
        self.id
    }

    fn is_compatible(&self, other: &dyn IParameterBlockLayout) -> bool {
        let other = unwrap::parameter_block_layout(other);
        self.desc.get().is_compatible(other.desc.get())
    }
}

impl Drop for ParameterBlockLayout {
    fn drop(&mut self) {
        unsafe {
            self._device
                .device
                .destroy_descriptor_set_layout(self.descriptor_set_layout, GLOBAL);
        }
    }
}
