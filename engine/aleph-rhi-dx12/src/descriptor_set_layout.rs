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

use std::collections::HashMap;
use std::num::NonZeroU64;

use aleph_any::AnyArc;
use aleph_object_system::{unsafe_impl_iobject, ArcedObject};
use aleph_rhi_api::*;
use windows::Win32::Graphics::Direct3D12::*;

use crate::device::Device;

pub struct DescriptorSetLayout {
    pub(crate) _device: AnyArc<Device>,
    pub(crate) id: NonZeroU64,
    pub(crate) binding_info: HashMap<u32, DescriptorBindingInfo>,
    pub(crate) visibility: D3D12_SHADER_VISIBILITY,
    pub(crate) dynamic_constant_buffers: Vec<D3D12_ROOT_DESCRIPTOR1>,
    pub(crate) resource_table: Vec<D3D12_DESCRIPTOR_RANGE1>,
    pub(crate) resource_num: u32,
    pub(crate) sampler_tables: Vec<D3D12_DESCRIPTOR_RANGE1>,
    pub(crate) static_samplers: Vec<D3D12_STATIC_SAMPLER_DESC>,
}

unsafe_impl_iobject!(DescriptorSetLayout, "01944fed-ed20-7631-ab3e-c6683ac06428");

impl DescriptorSetLayout {
    /// Internal function
    ///
    /// Queries the binding metadata for the given binding index.
    pub fn get_binding_info(&self, binding: u32) -> Option<DescriptorBindingInfo> {
        self.binding_info.get(&binding).cloned()
    }
}

impl DescriptorSetLayout {
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
}

/// Internal struct, stores computed cached info about a descriptor binding
#[derive(Clone)]
pub struct DescriptorBindingInfo {
    /// The type of descriptor this binding contains
    pub _type: DescriptorType,

    /// Whether this binding is a static sampler.
    pub _is_static_sampler: bool,

    /// The layout of the binding in descriptor memory
    pub layout: DescriptorBindingLayout,
}

/// Internal struct, represents the in-descriptor-heap layout of some descriptor set binding
#[derive(Clone)]
pub struct DescriptorBindingLayout {
    /// The base address of the binding layout, in descriptors
    pub base: u32,

    /// The number of descriptors in this binding (dictates the size)
    pub _num_descriptors: u32,
}
