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

use crate::device::ValidationDevice;
use crate::pipeline_layout::ValidationPipelineLayout;
use interfaces::any::{AnyArc, AnyWeak};
use interfaces::gpu::{IComputePipeline, IGraphicsPipeline, INamedObject};

pub struct ValidationGraphicsPipeline {
    pub(crate) _this: AnyWeak<Self>,
    pub(crate) _device: AnyArc<ValidationDevice>,
    pub(crate) _pipeline_layout: AnyArc<ValidationPipelineLayout>,
    pub(crate) inner: AnyArc<dyn IGraphicsPipeline>,
}

interfaces::any::declare_interfaces!(ValidationGraphicsPipeline, [IGraphicsPipeline]);

crate::impl_platform_interface_passthrough!(ValidationGraphicsPipeline);

impl IGraphicsPipeline for ValidationGraphicsPipeline {
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

impl INamedObject for ValidationGraphicsPipeline {
    fn set_name(&self, name: &str) {
        self.inner.set_name(name)
    }
}

pub struct ValidationComputePipeline {
    pub(crate) _this: AnyWeak<Self>,
    pub(crate) _device: AnyArc<ValidationDevice>,
    pub(crate) _pipeline_layout: AnyArc<ValidationPipelineLayout>,
    pub(crate) inner: AnyArc<dyn IComputePipeline>,
}

interfaces::any::declare_interfaces!(ValidationComputePipeline, [IComputePipeline]);

crate::impl_platform_interface_passthrough!(ValidationComputePipeline);

impl IComputePipeline for ValidationComputePipeline {
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

impl INamedObject for ValidationComputePipeline {
    fn set_name(&self, name: &str) {
        self.inner.set_name(name)
    }
}
