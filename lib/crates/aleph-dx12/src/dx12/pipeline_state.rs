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

use windows::Win32::Graphics::Direct3D12::ID3D12PipelineState;

#[repr(transparent)]
pub struct PipelineState(pub(crate) ID3D12PipelineState);

crate::object_impl!(PipelineState);
crate::device_child_impl!(PipelineState);
crate::shared_object!(PipelineState);
windows::deref_impl!(PipelineState, ID3D12PipelineState);

#[repr(transparent)]
pub struct GraphicsPipelineState(pub(crate) ID3D12PipelineState);

impl From<GraphicsPipelineState> for PipelineState {
    #[inline]
    fn from(v: GraphicsPipelineState) -> Self {
        PipelineState(v.0)
    }
}

crate::object_impl!(GraphicsPipelineState);
crate::device_child_impl!(GraphicsPipelineState);
crate::shared_object!(GraphicsPipelineState);
windows::deref_impl!(GraphicsPipelineState, ID3D12PipelineState);

#[repr(transparent)]
pub struct ComputePipelineState(pub(crate) ID3D12PipelineState);

impl From<ComputePipelineState> for PipelineState {
    #[inline]
    fn from(v: ComputePipelineState) -> Self {
        PipelineState(v.0)
    }
}

crate::object_impl!(ComputePipelineState);
crate::device_child_impl!(ComputePipelineState);
crate::shared_object!(ComputePipelineState);
windows::deref_impl!(ComputePipelineState, ID3D12PipelineState);

pub unsafe trait AsPipelineState {
    fn as_pipeline_state(&self) -> &ID3D12PipelineState;
}

unsafe impl AsPipelineState for PipelineState {
    #[inline]
    fn as_pipeline_state(&self) -> &ID3D12PipelineState {
        &self.0
    }
}

unsafe impl AsPipelineState for GraphicsPipelineState {
    #[inline]
    fn as_pipeline_state(&self) -> &ID3D12PipelineState {
        &self.0
    }
}

unsafe impl AsPipelineState for ComputePipelineState {
    #[inline]
    fn as_pipeline_state(&self) -> &ID3D12PipelineState {
        &self.0
    }
}

unsafe impl AsPipelineState for () {
    #[inline]
    fn as_pipeline_state(&self) -> &ID3D12PipelineState {
        unreachable!()
    }
}
