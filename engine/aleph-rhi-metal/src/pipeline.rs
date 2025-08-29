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
use aleph_object_system::{Object, unsafe_impl_iobject};
use aleph_rhi_api::*;
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_metal::*;

use crate::binding_signature::BindingSignature;
use crate::device::Device;

pub struct GraphicsPipeline {
    pub(crate) _device: AnyArc<Device>,
    pub(crate) _binding_signature: AnyArc<BindingSignature>,
    pub(crate) id: NonZeroU64,
    pub(crate) objects: GraphicsPipelineObjects,
    pub(crate) info: CachedGraphicsInfo,
}

unsafe_impl_iobject!(GraphicsPipeline, "01980753-5c4f-7ae3-be3b-9707082d77a7");

impl GraphicsPipeline {
    pub(crate) fn get_owned(v: &GraphicsPipelineHandle) -> std::sync::Arc<Object<Self>> {
        v.clone()
            .into_inner()
            .downcast::<Self>()
            .expect("Unknown GraphicsPipeline implementation!")
    }

    pub(crate) fn get(v: &GraphicsPipelineHandle) -> &Self {
        v.get()
            .downcast_ref::<Self>()
            .expect("Unknown GraphicsPipeline implementation!")
    }
}

impl Drop for GraphicsPipeline {
    fn drop(&mut self) {
        todo!()
    }
}

/// Wrapper type to limit the scope of our 'unsafe impl Send+Sync'
pub struct GraphicsPipelineObjects {
    pub pipeline: Retained<ProtocolObject<dyn MTLRenderPipelineState>>,
    pub depth_stencil_state: Retained<ProtocolObject<dyn MTLDepthStencilState>>,
}

/// Wrapper over all the pipeline data that is _not_ part of the MTLRenderPipelineState that the
/// RHI expects to be a part of the pipeline.
pub struct CachedGraphicsInfo {
    pub primitive_type: MTLPrimitiveType,

    pub cull_mode: MTLCullMode,
    pub front_face: MTLWinding,
    pub polygon_mode: MTLTriangleFillMode,
    pub depth_bias: i32, // If 0, depth bias is disabled
    pub depth_bias_clamp: f32,
    pub depth_bias_slope_factor: f32,
}

// Safety: Needed for 'MTLRenderPipelineState'
unsafe impl Send for GraphicsPipelineObjects {}
unsafe impl Sync for GraphicsPipelineObjects {}

pub struct ComputePipeline {
    pub(crate) _device: AnyArc<Device>,
    pub(crate) _binding_signature: AnyArc<BindingSignature>,
    pub(crate) id: NonZeroU64,
    pub(crate) objects: ComputePipelineObjects,
}

unsafe_impl_iobject!(ComputePipeline, "01980753-5c4f-7ae3-be3b-9719259cfbc3");

impl ComputePipeline {
    pub(crate) fn get_owned(v: &ComputePipelineHandle) -> std::sync::Arc<Object<Self>> {
        v.clone()
            .into_inner()
            .downcast::<Self>()
            .expect("Unknown ComputePipeline implementation!")
    }

    pub(crate) fn get(v: &ComputePipelineHandle) -> &Self {
        v.get()
            .downcast_ref::<Self>()
            .expect("Unknown ComputePipeline implementation!")
    }
}

/// Wrapper type to limit the scope of our 'unsafe impl Send+Sync'
pub struct ComputePipelineObjects {
    pub pipeline: Retained<ProtocolObject<dyn MTLComputePipelineState>>,
}

// Safety: Needed for 'MTLRenderPipelineState
unsafe impl Send for ComputePipelineObjects {}
unsafe impl Sync for ComputePipelineObjects {}
