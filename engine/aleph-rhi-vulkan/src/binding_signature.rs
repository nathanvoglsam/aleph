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
use aleph_rhi_api::IBindingSignature;
use aleph_rhi_impl_utils::RhiSystem;
use ash::vk;

use crate::device::Device;
use crate::internal::allocation_callbacks::GLOBAL;
use crate::parameter_block_layout::ParameterBlockLayout;

pub struct BindingSignature {
    pub(crate) _this: Weak<Self>,
    pub(crate) _device: Arc<Device>,
    pub(crate) id: NonZeroU64,
    pub(crate) pipeline_layout: vk::PipelineLayout,
    pub(crate) parameter_block_layouts: BVec<Arc<ParameterBlockLayout>, RhiSystem>,
    pub(crate) push_constant_block: Option<vk::PushConstantRange>,
}

impl IBindingSignature for BindingSignature {
    fn upgrade(&self) -> Arc<dyn IBindingSignature> {
        self._this.upgrade().unwrap()
    }

    fn strong_count(&self) -> usize {
        self._this.strong_count()
    }

    fn weak_count(&self) -> usize {
        self._this.weak_count()
    }

    fn get_id(&self) -> NonZeroU64 {
        self.id
    }
}

impl Drop for BindingSignature {
    fn drop(&mut self) {
        unsafe {
            self._device
                .device
                .destroy_pipeline_layout(self.pipeline_layout, GLOBAL);
        }
    }
}
