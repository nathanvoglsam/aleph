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
use ash::vk;

use crate::binding_signature::BindingSignature;
use crate::device::Device;
use crate::internal::allocation_callbacks::GLOBAL;

pub struct GraphicsPipeline {
    pub(crate) _device: AnyArc<Device>,
    pub(crate) _binding_signature: AnyArc<BindingSignature>,
    pub(crate) id: NonZeroU64,
    pub(crate) pipeline: vk::Pipeline,
}

unsafe_impl_iobject!(GraphicsPipeline, "01944fe3-393c-7893-aa9a-0b2905168fa4");

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
        unsafe {
            self._device.device.destroy_pipeline(self.pipeline, GLOBAL);
        }
    }
}

pub struct ComputePipeline {
    pub(crate) _device: AnyArc<Device>,
    pub(crate) _binding_signature: AnyArc<BindingSignature>,
    pub(crate) id: NonZeroU64,
    pub(crate) pipeline: vk::Pipeline,
}

unsafe_impl_iobject!(ComputePipeline, "01944fe3-5207-7dd2-8f84-5d37f5623f02");

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

impl Drop for ComputePipeline {
    fn drop(&mut self) {
        unsafe {
            self._device.device.destroy_pipeline(self.pipeline, GLOBAL);
        }
    }
}
