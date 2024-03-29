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

use aleph_any::{declare_interfaces, AnyArc, AnyWeak};
use aleph_rhi_api::*;
use aleph_rhi_impl_utils::try_clone_value_into_slot;
use ash::vk;

use crate::device::Device;
use crate::pipeline_layout::PipelineLayout;

pub struct GraphicsPipeline {
    pub(crate) _this: AnyWeak<Self>,
    pub(crate) _device: AnyArc<Device>,
    pub(crate) _pipeline_layout: AnyArc<PipelineLayout>,
    pub(crate) pipeline: vk::Pipeline,
}

declare_interfaces!(GraphicsPipeline, [IGraphicsPipeline]);

impl IGetPlatformInterface for GraphicsPipeline {
    unsafe fn __query_platform_interface(&self, target: TypeId, out: *mut ()) -> Option<()> {
        try_clone_value_into_slot(&self.pipeline, out, target)
    }
}

impl IGraphicsPipeline for GraphicsPipeline {
    fn upgrade(&self) -> AnyArc<dyn IGraphicsPipeline> {
        AnyArc::map::<dyn IGraphicsPipeline, _>(self._this.upgrade().unwrap(), |v| v)
    }

    fn strong_count(&self) -> usize {
        self._this.strong_count()
    }

    fn weak_count(&self) -> usize {
        self._this.weak_count()
    }
}

impl Drop for GraphicsPipeline {
    fn drop(&mut self) {
        unsafe {
            self._device.device.destroy_pipeline(self.pipeline, None);
        }
    }
}

pub struct ComputePipeline {
    pub(crate) _this: AnyWeak<Self>,
    pub(crate) _device: AnyArc<Device>,
    pub(crate) _pipeline_layout: AnyArc<PipelineLayout>,
    pub(crate) pipeline: vk::Pipeline,
}

declare_interfaces!(ComputePipeline, [IComputePipeline]);

impl IGetPlatformInterface for ComputePipeline {
    unsafe fn __query_platform_interface(&self, target: TypeId, out: *mut ()) -> Option<()> {
        try_clone_value_into_slot(&self.pipeline, out, target)
    }
}

impl IComputePipeline for ComputePipeline {
    fn upgrade(&self) -> AnyArc<dyn IComputePipeline> {
        AnyArc::map::<dyn IComputePipeline, _>(self._this.upgrade().unwrap(), |v| v)
    }

    fn strong_count(&self) -> usize {
        self._this.strong_count()
    }

    fn weak_count(&self) -> usize {
        self._this.weak_count()
    }
}

impl Drop for ComputePipeline {
    fn drop(&mut self) {
        unsafe {
            self._device.device.destroy_pipeline(self.pipeline, None);
        }
    }
}
