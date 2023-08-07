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

use crate::internal::features::CheckMeetsMinimum;
use crate::internal::profile::CreateProfile;
use ash::vk;
use std::ffi::{c_char, CStr};

#[derive(Clone, Default)]
#[rustfmt::skip]
pub struct DeviceInfo {
    pub extensions: Vec<vk::ExtensionProperties>,
    pub properties_10: vk::PhysicalDeviceProperties,
    pub properties_11: vk::PhysicalDeviceVulkan11Properties,
    pub descriptor_indexing_properties: vk::PhysicalDeviceDescriptorIndexingProperties,
    pub float_controls_properties: vk::PhysicalDeviceFloatControlsProperties,
    pub depth_stencil_resolve_properties: vk::PhysicalDeviceDepthStencilResolveProperties,
    pub timeline_semaphore_properties: vk::PhysicalDeviceTimelineSemaphoreProperties,
    pub sampler_filter_minmax_properties: vk::PhysicalDeviceSamplerFilterMinmaxProperties,
    pub driver_properties: vk::PhysicalDeviceDriverProperties,
    pub portability_properties: vk::PhysicalDevicePortabilitySubsetPropertiesKHR,
    pub features_10: vk::PhysicalDeviceFeatures,
    pub features_11: vk::PhysicalDeviceVulkan11Features,
    pub descriptor_indexing_features: vk::PhysicalDeviceDescriptorIndexingFeatures,
    pub imageless_framebuffer_features: vk::PhysicalDeviceImagelessFramebufferFeaturesKHR,
    pub scalar_block_layout_features: vk::PhysicalDeviceScalarBlockLayoutFeatures,
    pub timeline_semaphore_features: vk::PhysicalDeviceTimelineSemaphoreFeatures,
    pub buffer_device_address_features: vk::PhysicalDeviceBufferDeviceAddressFeatures,
    pub uniform_buffer_standard_layout_features: vk::PhysicalDeviceUniformBufferStandardLayoutFeatures,
    pub t_8bit_storage_features: vk::PhysicalDevice8BitStorageFeatures,
    pub shader_float16int8features: vk::PhysicalDeviceShaderFloat16Int8Features,
    pub host_query_reset_features: vk::PhysicalDeviceHostQueryResetFeatures,
    pub shader_atomic_int_64_features: vk::PhysicalDeviceShaderAtomicInt64Features,
    pub vulkan_memory_model_features: vk::PhysicalDeviceVulkanMemoryModelFeatures,
    pub dynamic_rendering_features: vk::PhysicalDeviceDynamicRenderingFeaturesKHR,
    pub portability_features: vk::PhysicalDevicePortabilitySubsetFeaturesKHR,
    pub synchronization_2_features: vk::PhysicalDeviceSynchronization2FeaturesKHR,
}

impl DeviceInfo {
    pub fn minimum() -> Self {
        Self {
            extensions: vec![],
            properties_10: CreateProfile::minimum(),
            properties_11: CreateProfile::minimum(),
            descriptor_indexing_properties: CreateProfile::minimum(),
            float_controls_properties: CreateProfile::minimum(),
            depth_stencil_resolve_properties: CreateProfile::minimum(),
            timeline_semaphore_properties: CreateProfile::minimum(),
            sampler_filter_minmax_properties: CreateProfile::minimum(),
            driver_properties: CreateProfile::minimum(),
            portability_properties: Default::default(),
            features_10: CreateProfile::minimum(),
            features_11: CreateProfile::minimum(),
            descriptor_indexing_features: CreateProfile::minimum(),
            imageless_framebuffer_features: CreateProfile::minimum(),
            scalar_block_layout_features: CreateProfile::minimum(),
            timeline_semaphore_features: CreateProfile::minimum(),
            buffer_device_address_features: CreateProfile::minimum(),
            uniform_buffer_standard_layout_features: CreateProfile::minimum(),
            t_8bit_storage_features: CreateProfile::minimum(),
            shader_float16int8features: CreateProfile::minimum(),
            host_query_reset_features: CreateProfile::minimum(),
            shader_atomic_int_64_features: CreateProfile::minimum(),
            vulkan_memory_model_features: CreateProfile::minimum(),
            dynamic_rendering_features: CreateProfile::minimum(),
            portability_features: Default::default(),
            synchronization_2_features: Default::default(),
        }
    }

    pub fn load(instance: &ash::Instance, physical_device: vk::PhysicalDevice) -> DeviceInfo {
        let extensions = unsafe {
            instance
                .enumerate_device_extension_properties(physical_device)
                .unwrap_or_default()
                .to_vec()
        };

        let mut out = DeviceInfo {
            extensions,
            ..Default::default()
        };
        let DeviceInfo {
            extensions,
            properties_10,
            properties_11,
            descriptor_indexing_properties,
            float_controls_properties,
            depth_stencil_resolve_properties,
            timeline_semaphore_properties,
            sampler_filter_minmax_properties,
            driver_properties,
            portability_properties,
            features_10,
            features_11,
            descriptor_indexing_features,
            imageless_framebuffer_features,
            scalar_block_layout_features,
            timeline_semaphore_features,
            buffer_device_address_features,
            uniform_buffer_standard_layout_features,
            t_8bit_storage_features,
            shader_float16int8features,
            host_query_reset_features,
            shader_atomic_int_64_features,
            vulkan_memory_model_features,
            dynamic_rendering_features,
            portability_features,
            synchronization_2_features,
        } = &mut out;

        // Unconditionally required properties
        let mut properties = vk::PhysicalDeviceProperties2::builder()
            .push_next(properties_11)
            .push_next(descriptor_indexing_properties)
            .push_next(float_controls_properties)
            .push_next(depth_stencil_resolve_properties)
            .push_next(timeline_semaphore_properties)
            .push_next(sampler_filter_minmax_properties)
            .push_next(driver_properties);

        // Safety: we assume all the strings vulkan gives us are valid
        unsafe {
            // We load the portability subset properties if the extension is present
            if Self::list_contains_extension_cstr(&extensions, vk::KhrPortabilitySubsetFn::name()) {
                properties = properties.push_next(portability_properties)
            }
        };

        *properties_10 = unsafe {
            instance.get_physical_device_properties2(physical_device, &mut properties);
            properties.properties
        };

        // Glue all the feature extension structs together into our monster instance
        let mut features = vk::PhysicalDeviceFeatures2::builder()
            .push_next(features_11)
            .push_next(descriptor_indexing_features)
            .push_next(imageless_framebuffer_features)
            .push_next(scalar_block_layout_features)
            .push_next(timeline_semaphore_features)
            .push_next(buffer_device_address_features)
            .push_next(uniform_buffer_standard_layout_features)
            .push_next(host_query_reset_features);

        unsafe {
            if Self::list_contains_extension_cstr(&extensions, vk::KhrVulkanMemoryModelFn::name()) {
                features = features.push_next(vulkan_memory_model_features)
            }
        };
        unsafe {
            if Self::list_contains_extension_cstr(&extensions, vk::Khr8bitStorageFn::name()) {
                features = features.push_next(t_8bit_storage_features)
            }
        };
        unsafe {
            if Self::list_contains_extension_cstr(&extensions, vk::KhrShaderFloat16Int8Fn::name()) {
                features = features.push_next(shader_float16int8features)
            }
        };
        unsafe {
            if Self::list_contains_extension_cstr(&extensions, vk::KhrShaderAtomicInt64Fn::name()) {
                features = features.push_next(shader_atomic_int_64_features)
            }
        };
        unsafe {
            if Self::list_contains_extension_cstr(&extensions, vk::KhrDynamicRenderingFn::name()) {
                features = features.push_next(dynamic_rendering_features)
            }
        };
        unsafe {
            if Self::list_contains_extension_cstr(&extensions, vk::KhrPortabilitySubsetFn::name()) {
                features = features.push_next(portability_features)
            }
        };
        unsafe {
            if Self::list_contains_extension_cstr(&extensions, vk::KhrSynchronization2Fn::name()) {
                features = features.push_next(synchronization_2_features)
            }
        };

        *features_10 = unsafe {
            instance.get_physical_device_features2(physical_device, &mut features);
            features.features
        };

        // Null the p_next chain pointers to avoid leaving the dangling references. They can't be
        // *used* without unsafe but better be careful.
        out.null_p_next_ptrs();

        out
    }

    fn null_p_next_ptrs(&mut self) {
        self.properties_11.p_next = std::ptr::null_mut();
        self.descriptor_indexing_properties.p_next = std::ptr::null_mut();
        self.float_controls_properties.p_next = std::ptr::null_mut();
        self.depth_stencil_resolve_properties.p_next = std::ptr::null_mut();
        self.timeline_semaphore_properties.p_next = std::ptr::null_mut();
        self.sampler_filter_minmax_properties.p_next = std::ptr::null_mut();
        self.driver_properties.p_next = std::ptr::null_mut();
        self.portability_properties.p_next = std::ptr::null_mut();
        self.features_11.p_next = std::ptr::null_mut();
        self.descriptor_indexing_features.p_next = std::ptr::null_mut();
        self.imageless_framebuffer_features.p_next = std::ptr::null_mut();
        self.scalar_block_layout_features.p_next = std::ptr::null_mut();
        self.timeline_semaphore_features.p_next = std::ptr::null_mut();
        self.buffer_device_address_features.p_next = std::ptr::null_mut();
        self.uniform_buffer_standard_layout_features.p_next = std::ptr::null_mut();
        self.t_8bit_storage_features.p_next = std::ptr::null_mut();
        self.shader_float16int8features.p_next = std::ptr::null_mut();
        self.host_query_reset_features.p_next = std::ptr::null_mut();
        self.shader_atomic_int_64_features.p_next = std::ptr::null_mut();
        self.vulkan_memory_model_features.p_next = std::ptr::null_mut();
        self.dynamic_rendering_features.p_next = std::ptr::null_mut();
        self.portability_features.p_next = std::ptr::null_mut();
        self.synchronization_2_features.p_next = std::ptr::null_mut();
    }
}

impl DeviceInfo {
    #[allow(unused)]
    pub fn supports_extension_ptr(&self, wanted: *const c_char) -> bool {
        unsafe { Self::list_contains_extension_ptr(&self.extensions, wanted) }
    }

    #[allow(unused)]
    pub fn supports_extension_cstr(&self, wanted: &CStr) -> bool {
        unsafe { Self::list_contains_extension_cstr(&self.extensions, wanted) }
    }

    #[allow(unused)]
    pub fn supports_extension(&self, wanted: &str) -> bool {
        unsafe { Self::list_contains_extension(&self.extensions, wanted) }
    }

    #[rustfmt::skip]
    pub fn meets_minimum_requirements(&self) -> Option<()> {
        self.properties_10.meets_minimum()?;
        self.properties_11.meets_minimum()?;
        self.descriptor_indexing_properties.meets_minimum()?;
        self.float_controls_properties.meets_minimum()?;
        self.depth_stencil_resolve_properties.meets_minimum()?;
        self.timeline_semaphore_properties.meets_minimum()?;
        self.sampler_filter_minmax_properties.meets_minimum()?;
        self.driver_properties.meets_minimum()?;
        self.portability_properties.meets_minimum()?;
        self.features_10.meets_minimum()?;
        self.features_11.meets_minimum()?;
        self.descriptor_indexing_features.meets_minimum()?;
        self.imageless_framebuffer_features.meets_minimum()?;
        self.scalar_block_layout_features.meets_minimum()?;
        self.timeline_semaphore_features.meets_minimum()?;
        self.buffer_device_address_features.meets_minimum()?;
        self.uniform_buffer_standard_layout_features.meets_minimum()?;
        self.t_8bit_storage_features.meets_minimum()?;
        self.shader_float16int8features.meets_minimum()?;
        self.host_query_reset_features.meets_minimum()?;
        self.shader_atomic_int_64_features.meets_minimum()?;
        self.vulkan_memory_model_features.meets_minimum()?;
        self.dynamic_rendering_features.meets_minimum()?;
        self.portability_features.meets_minimum()?;
        self.synchronization_2_features.meets_minimum()?;
        Some(())
    }

    unsafe fn list_contains_extension_ptr(
        extensions: &[vk::ExtensionProperties],
        wanted: *const c_char,
    ) -> bool {
        Self::list_contains_extension_cstr(extensions, CStr::from_ptr(wanted))
    }

    unsafe fn list_contains_extension_cstr(
        extensions: &[vk::ExtensionProperties],
        wanted: &CStr,
    ) -> bool {
        Self::list_contains_extension(extensions, wanted.to_str().unwrap_unchecked())
    }

    unsafe fn list_contains_extension(
        extensions: &[vk::ExtensionProperties],
        wanted: &str,
    ) -> bool {
        extensions
            .iter()
            .map(|v| {
                CStr::from_ptr(v.extension_name.as_ptr())
                    .to_str()
                    .unwrap_unchecked()
            })
            .any(|v| v == wanted)
    }
}

unsafe impl Send for DeviceInfo {}
unsafe impl Sync for DeviceInfo {}
