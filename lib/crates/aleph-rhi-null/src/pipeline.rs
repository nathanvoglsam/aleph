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

use aleph_any::{declare_interfaces, AnyArc, AnyWeak};
use aleph_rhi_api::*;

use crate::{NullDevice, NullPipelineLayout};

pub struct NullGraphicsPipeline {
    pub(crate) _this: AnyWeak<Self>,
    pub(crate) _device: AnyArc<NullDevice>,
    pub(crate) _pipeline_layout: AnyArc<NullPipelineLayout>,
}

declare_interfaces!(NullGraphicsPipeline, [IGraphicsPipeline]);

crate::impl_platform_interface_passthrough!(NullGraphicsPipeline);

impl IGraphicsPipeline for NullGraphicsPipeline {
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

pub struct NullComputePipeline {
    pub(crate) _this: AnyWeak<Self>,
    pub(crate) _device: AnyArc<NullDevice>,
    pub(crate) _pipeline_layout: AnyArc<NullPipelineLayout>,
}

declare_interfaces!(NullComputePipeline, [IComputePipeline]);

crate::impl_platform_interface_passthrough!(NullComputePipeline);

impl IComputePipeline for NullComputePipeline {
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
