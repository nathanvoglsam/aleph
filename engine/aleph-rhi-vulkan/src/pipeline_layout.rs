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

use aleph_any::AnyArc;
use aleph_object_system::{ArcedObject, unsafe_impl_iobject};
use aleph_rhi_api::*;
use ash::vk;

use crate::device::Device;

pub struct PipelineLayout {
    pub(crate) _device: AnyArc<Device>,
    pub(crate) id: NonZeroU64,
    pub(crate) pipeline_layout: vk::PipelineLayout,
    pub(crate) push_constant_blocks: Vec<vk::PushConstantRange>,
}

unsafe_impl_iobject!(PipelineLayout, "01944fe3-98db-7ba1-96d4-b40e2d69f172");

impl PipelineLayout {
    pub(crate) fn get_owned(v: &PipelineLayoutHandle) -> std::sync::Arc<ArcedObject<Self>> {
        v.clone()
            .into_inner()
            .downcast::<Self>()
            .expect("Unknown PipelineLayout implementation!")
    }

    pub(crate) fn get(v: &PipelineLayoutHandle) -> &Self {
        v.get()
            .downcast_ref::<Self>()
            .expect("Unknown PipelineLayout implementation!")
    }
}

impl Drop for PipelineLayout {
    fn drop(&mut self) {
        unsafe {
            self._device
                .device
                .destroy_pipeline_layout(self.pipeline_layout, None);
        }
    }
}
