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
use interfaces::any::{AnyArc, AnyWeak};
use interfaces::gpu::{DescriptorType, IDescriptorSetLayout, INamedObject};
use std::collections::HashMap;
use std::num::NonZeroU32;

pub struct ValidationDescriptorSetLayout {
    pub(crate) _this: AnyWeak<Self>,
    pub(crate) _device: AnyArc<ValidationDevice>,
    pub(crate) inner: AnyArc<dyn IDescriptorSetLayout>,
    pub(crate) binding_info: HashMap<u32, DescriptorBindingInfo>,
}

crate::validation_declare_interfaces!(ValidationDescriptorSetLayout, [IDescriptorSetLayout]);

impl ValidationDescriptorSetLayout {
    /// Internal function
    ///
    /// Queries the binding metadata for the given binding index.
    pub fn get_binding_info(&self, binding: u32) -> Option<DescriptorBindingInfo> {
        self.binding_info.get(&binding).cloned()
    }
}

impl IDescriptorSetLayout for ValidationDescriptorSetLayout {
    fn upgrade(&self) -> AnyArc<dyn IDescriptorSetLayout> {
        AnyArc::map::<dyn IDescriptorSetLayout, _>(self._this.upgrade().unwrap(), |v| v)
    }

    fn strong_count(&self) -> usize {
        self._this.strong_count()
    }

    fn weak_count(&self) -> usize {
        self._this.weak_count()
    }
}

impl INamedObject for ValidationDescriptorSetLayout {
    fn set_name(&self, name: &str) {
        self.inner.set_name(name)
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

    /// Declares whether the descriptor's underlying resource can be accessed with write access.
    pub allow_writes: bool,
}