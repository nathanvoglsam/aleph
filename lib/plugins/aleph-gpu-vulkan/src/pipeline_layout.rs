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

use crate::device::Device;
use crate::internal::try_clone_value_into_slot;
use erupt::vk;
use interfaces::any::{declare_interfaces, AnyArc, AnyWeak};
use interfaces::gpu::*;
use std::any::TypeId;

pub struct PipelineLayout {
    pub(crate) _this: AnyWeak<Self>,
    pub(crate) _device: AnyArc<Device>,
    pub(crate) pipeline_layout: vk::PipelineLayout,
    pub(crate) push_constant_blocks: Vec<PushConstantBlockInfo>,
}

declare_interfaces!(PipelineLayout, [IPipelineLayout]);

impl IGetPlatformInterface for PipelineLayout {
    unsafe fn __query_platform_interface(&self, target: TypeId, out: *mut ()) -> Option<()> {
        try_clone_value_into_slot::<vk::PipelineLayout>(&self.pipeline_layout, out, target)
    }
}

impl IPipelineLayout for PipelineLayout {
    fn upgrade(&self) -> AnyArc<dyn IPipelineLayout> {
        AnyArc::map::<dyn IPipelineLayout, _>(self._this.upgrade().unwrap(), |v| v)
    }

    fn strong_count(&self) -> usize {
        self._this.strong_count()
    }

    fn weak_count(&self) -> usize {
        self._this.weak_count()
    }
}

impl Drop for PipelineLayout {
    fn drop(&mut self) {
        unsafe {
            self._device
                .device_loader
                .destroy_pipeline_layout(self.pipeline_layout, None);
        }
    }
}

/// Internal struct for caching information necessary for implementing command recording
pub struct PushConstantBlockInfo {
    pub size: u32,
    pub offset: u32,
    pub visibility: vk::ShaderStageFlags,
}
