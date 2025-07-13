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

use std::sync::Arc;

use aleph_any::AnyArc;
use aleph_object_system::{ArcedObject, unsafe_impl_iobject};
use aleph_rhi_api::*;

use crate::{ValidationDevice, ValidationPipelineLayout};

pub struct ValidationGraphicsPipeline {
    pub(crate) _device: AnyArc<ValidationDevice>,
    pub(crate) _pipeline_layout: Arc<ArcedObject<ValidationPipelineLayout>>,
    pub(crate) inner: GraphicsPipelineHandle,
}

unsafe_impl_iobject!(
    ValidationGraphicsPipeline,
    "01945001-1330-7701-967e-67870337ea4b"
);

impl ValidationGraphicsPipeline {
    pub(crate) fn get_owned(v: &GraphicsPipelineHandle) -> std::sync::Arc<ArcedObject<Self>> {
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

pub struct ValidationComputePipeline {
    pub(crate) _device: AnyArc<ValidationDevice>,
    pub(crate) _pipeline_layout: Arc<ArcedObject<ValidationPipelineLayout>>,
    pub(crate) inner: ComputePipelineHandle,
}

unsafe_impl_iobject!(
    ValidationComputePipeline,
    "01945000-ead1-7c43-b472-84105f560419"
);

impl ValidationComputePipeline {
    pub(crate) fn get_owned(v: &ComputePipelineHandle) -> std::sync::Arc<ArcedObject<Self>> {
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
