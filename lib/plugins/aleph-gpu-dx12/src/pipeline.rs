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

use crate::pipeline_layout::PipelineLayout;
use dx12::D3D12Object;
use interfaces::any::{declare_interfaces, AnyArc, AnyWeak};
use interfaces::gpu::{IGraphicsPipeline, INamedObject};

pub struct GraphicsPipeline {
    pub(crate) this: AnyWeak<Self>,
    pub(crate) pipeline: dx12::GraphicsPipelineState,
    pub(crate) pipeline_layout: AnyArc<PipelineLayout>,

    /// Vulkan bakes this into the pipeline, d3d12 doesn't. We have to behave like vulkan as vulkan
    /// can't do the reverse.
    pub(crate) primitive_topology: dx12::PrimitiveTopology,

    /// Vulkan specifies these values in the pipeline, d3d12 doesn't. Again we need to behave like
    /// vulkan. D3D12 passes these values when the vertex buffers are bound, so we need to hold on
    /// to these so the command encoder can get them and transparently pass them in.
    pub(crate) input_binding_strides: [u32; 16],
}

declare_interfaces!(GraphicsPipeline, [IGraphicsPipeline, IGraphicsPipelineExt]);

impl IGraphicsPipeline for GraphicsPipeline {
    fn upgrade(&self) -> AnyArc<dyn IGraphicsPipeline> {
        AnyArc::map::<dyn IGraphicsPipeline, _>(self.this.upgrade().unwrap(), |v| v)
    }

    fn strong_count(&self) -> usize {
        self.this.strong_count()
    }

    fn weak_count(&self) -> usize {
        self.this.weak_count()
    }
}

pub trait IGraphicsPipelineExt: IGraphicsPipeline {
    fn get_raw_handle(&self) -> dx12::GraphicsPipelineState;
}

impl IGraphicsPipelineExt for GraphicsPipeline {
    fn get_raw_handle(&self) -> dx12::GraphicsPipelineState {
        self.pipeline.clone()
    }
}

impl INamedObject for GraphicsPipeline {
    fn set_name(&self, name: &str) {
        self.pipeline.set_name(name).unwrap()
    }
}
