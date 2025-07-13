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
use std::sync::Arc;

use aleph_any::AnyArc;
use aleph_object_system::{ArcedObject, unsafe_impl_iobject};
use aleph_rhi_api::*;

use crate::{NullDevice, NullPipelineLayout};

pub struct NullGraphicsPipeline {
    pub(crate) _device: AnyArc<NullDevice>,
    pub(crate) _pipeline_layout: Arc<ArcedObject<NullPipelineLayout>>,
    pub(crate) id: NonZeroU64,
}

unsafe_impl_iobject!(NullGraphicsPipeline, "01944fbe-c91c-7cd2-8fde-0ba32aa2c01f");

impl NullGraphicsPipeline {
    pub(crate) fn get(v: &GraphicsPipelineHandle) -> &Self {
        v.get()
            .downcast_ref::<Self>()
            .expect("Unknown GraphicsPipeline implementation!")
    }
}

pub struct NullComputePipeline {
    pub(crate) _device: AnyArc<NullDevice>,
    pub(crate) _pipeline_layout: Arc<ArcedObject<NullPipelineLayout>>,
    pub(crate) id: NonZeroU64,
}

unsafe_impl_iobject!(NullComputePipeline, "01944fbe-c91c-7cd2-8fde-0bba2e16937b");

impl NullComputePipeline {
    pub(crate) fn get(v: &ComputePipelineHandle) -> &Self {
        v.get()
            .downcast_ref::<Self>()
            .expect("Unknown ComputePipeline implementation!")
    }
}
