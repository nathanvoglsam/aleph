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
use interfaces::any::{declare_interfaces, AnyArc, AnyWeak};
use interfaces::gpu::{IComputePipeline, IGraphicsPipeline, INamedObject};
use windows::core::PCWSTR;
use windows::Win32::Graphics::Direct3D::*;
use windows::Win32::Graphics::Direct3D12::*;

pub struct GraphicsPipeline {
    pub(crate) this: AnyWeak<Self>,
    pub(crate) pipeline_layout: AnyArc<PipelineLayout>,
    pub(crate) pipeline: ID3D12PipelineState,

    /// Vulkan bakes this into the pipeline, d3d12 doesn't. We have to behave like vulkan as vulkan
    /// can't do the reverse.
    pub(crate) primitive_topology: D3D_PRIMITIVE_TOPOLOGY,

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
    fn get_raw_handle(&self) -> ID3D12PipelineState;
}

impl IGraphicsPipelineExt for GraphicsPipeline {
    fn get_raw_handle(&self) -> ID3D12PipelineState {
        self.pipeline.clone()
    }
}

impl INamedObject for GraphicsPipeline {
    fn set_name(&self, name: &str) {
        unsafe {
            let utf16: Vec<u16> = name.encode_utf16().chain(std::iter::once(0)).collect();
            let name = PCWSTR::from_raw(utf16.as_ptr());
            self.pipeline.SetName(name).unwrap();
        }
    }
}

pub struct ComputePipeline {
    pub(crate) this: AnyWeak<Self>,
    pub(crate) _pipeline_layout: AnyArc<PipelineLayout>,
    pub(crate) pipeline: ID3D12PipelineState,
}

declare_interfaces!(ComputePipeline, [IComputePipeline, IComputePipelineExt]);

impl IComputePipeline for ComputePipeline {
    fn upgrade(&self) -> AnyArc<dyn IComputePipeline> {
        AnyArc::map::<dyn IComputePipeline, _>(self.this.upgrade().unwrap(), |v| v)
    }

    fn strong_count(&self) -> usize {
        self.this.strong_count()
    }

    fn weak_count(&self) -> usize {
        self.this.weak_count()
    }
}

pub trait IComputePipelineExt: IComputePipeline {
    fn get_raw_handle(&self) -> ID3D12PipelineState;
}

impl IComputePipelineExt for ComputePipeline {
    fn get_raw_handle(&self) -> ID3D12PipelineState {
        self.pipeline.clone()
    }
}

impl INamedObject for ComputePipeline {
    fn set_name(&self, name: &str) {
        unsafe {
            let utf16: Vec<u16> = name.encode_utf16().chain(std::iter::once(0)).collect();
            let name = PCWSTR::from_raw(utf16.as_ptr());
            self.pipeline.SetName(name).unwrap();
        }
    }
}
