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

use std::collections::HashMap;
use std::num::NonZeroU32;

use aleph_any::AnyArc;
use aleph_object_system::{ArcedObject, unsafe_impl_iobject};
use aleph_rhi_api::*;

use crate::ValidationDevice;

pub struct ValidationDescriptorSetLayout {
    pub(crate) _device: AnyArc<ValidationDevice>,
    pub(crate) inner: DescriptorSetLayoutHandle,
    pub(crate) binding_info: HashMap<u32, DescriptorBindingInfo>,
}

unsafe_impl_iobject!(
    ValidationDescriptorSetLayout,
    "01944ff7-295e-7131-805b-27db8b658346"
);

impl ValidationDescriptorSetLayout {
    /// Internal function
    ///
    /// Queries the binding metadata for the given binding index.
    pub fn get_binding_info(&self, binding: u32) -> Option<DescriptorBindingInfo> {
        self.binding_info.get(&binding).cloned()
    }
}

impl ValidationDescriptorSetLayout {
    pub(crate) fn get_owned(v: &DescriptorSetLayoutHandle) -> std::sync::Arc<ArcedObject<Self>> {
        v.clone()
            .into_inner()
            .downcast::<Self>()
            .expect("Unknown DescriptorSetLayout implementation!")
    }

    pub(crate) fn get(v: &DescriptorSetLayoutHandle) -> &Self {
        v.get()
            .downcast_ref::<Self>()
            .expect("Unknown DescriptorSetLayout implementation!")
    }

    pub(crate) fn get_id(&self, device: &ValidationDevice) -> std::num::NonZeroU64 {
        device.inner.get_descriptor_set_layout_id(&self.inner)
    }
}

/// Internal struct, stores computed cached info about a descriptor binding
#[derive(Clone)]
pub struct DescriptorBindingInfo {
    /// The type of descriptor this binding contains
    pub r#type: DescriptorType,

    /// The number of descriptors in the binding. [None] encodes a non-array binding with 1
    /// descriptor, while [Some(n)] encodes an array binding with 'n' descriptors.
    pub descriptor_count: Option<NonZeroU32>,

    /// Whether this binding is a static sampler.
    pub is_static_sampler: bool,
}
