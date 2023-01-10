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

//! D3D12 has no "descriptor set layout" like object, it only has the root signature which is
//! similar to the VkPipelineLayout object.
//!
//! We fake a 'VkDescriptorSetLayout' like object by just copying the input to the
//! create_descriptor_set_layout call so we can collect and use it when we create the root
//! signature.

use crate::device::Device;
use interfaces::any::{declare_interfaces, AnyArc, AnyWeak};
use interfaces::gpu::{DescriptorType, IDescriptorSetLayout, INamedObject};
use std::collections::HashMap;
use windows::Win32::Graphics::Direct3D12::*;

pub struct DescriptorSetLayout {
    pub(crate) this: AnyWeak<Self>,
    pub(crate) _device: AnyArc<Device>,
    pub(crate) binding_info: HashMap<u32, DescriptorBindingInfo>,
    pub(crate) visibility: D3D12_SHADER_VISIBILITY,
    pub(crate) resource_table: Vec<D3D12_DESCRIPTOR_RANGE1>,
    pub(crate) resource_num: u32,
    pub(crate) sampler_table: Option<Vec<D3D12_DESCRIPTOR_RANGE1>>,
    pub(crate) sampler_num: u32,
    pub(crate) static_samplers: Vec<D3D12_STATIC_SAMPLER_DESC>,
}

declare_interfaces!(DescriptorSetLayout, [IDescriptorSetLayout]);

impl DescriptorSetLayout {
    /// Internal function
    ///
    /// Queries whether the given descriptor binding is filled by a static sampler.
    ///
    /// This would be used by validation to check if users try to write into static sampler
    /// descriptors.
    pub fn is_binding_static_sampler(&self, binding: u32) -> Option<bool> {
        self.get_binding_info(binding).map(|v| v.is_static_sampler)
    }

    /// Internal function
    ///
    /// Queries the type of descriptor contained in the given binding.
    ///
    /// This would would be used for deducing the descriptor increment size when writing
    /// descriptors. It may also be used for validation, to check if the user is writing the correct
    /// type of descriptors.
    pub fn get_binding_type(&self, binding: u32) -> Option<DescriptorType> {
        self.get_binding_info(binding).map(|v| v.r#type)
    }

    /// Internal function
    ///
    /// Queries the layout for the given descriptor set binding, in this descriptor set layout.
    ///
    /// This would typically be combined with a [crate::descriptor_pool::DescriptorSet] to compute
    /// the GPU and CPU addresses for writing descriptors. It may also be used by validation to
    /// check if the user is writing within the binding's bounds (i.e. not writing outside of a
    /// descriptor array's bounds).
    ///
    pub fn get_binding_layout(&self, binding: u32) -> Option<DescriptorBindingLayout> {
        self.get_binding_info(binding).map(|v| v.layout.clone())
    }

    /// Internal function
    ///
    /// Queries the binding metadata for the given binding index.
    pub fn get_binding_info(&self, binding: u32) -> Option<DescriptorBindingInfo> {
        self.binding_info.get(&binding).cloned()
    }
}

impl IDescriptorSetLayout for DescriptorSetLayout {
    fn upgrade(&self) -> AnyArc<dyn IDescriptorSetLayout> {
        AnyArc::map::<dyn IDescriptorSetLayout, _>(self.this.upgrade().unwrap(), |v| v)
    }

    fn strong_count(&self) -> usize {
        self.this.strong_count()
    }

    fn weak_count(&self) -> usize {
        self.this.weak_count()
    }
}

impl INamedObject for DescriptorSetLayout {
    fn set_name(&self, _name: &str) {
        // Nothing to do, no API object for this exists in d3d12
    }
}

/// Internal struct, stores computed cached info about a descriptor binding
#[derive(Clone)]
pub struct DescriptorBindingInfo {
    /// The type of descriptor this binding contains
    pub r#type: DescriptorType,

    /// Whether this binding is a static sampler.
    pub is_static_sampler: bool,

    /// The layout of the binding in descriptor memory
    pub layout: DescriptorBindingLayout,
}

/// Internal struct, represents the in-descriptor-heap layout of some descriptor set binding
#[derive(Clone)]
pub struct DescriptorBindingLayout {
    /// The base address of the binding layout, in descriptors
    pub base: u32,

    /// The number of descriptors in this binding (dictates the size)
    pub num_descriptors: u32,
}
