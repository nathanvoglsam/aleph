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

use ash::vk;
use std::ffi::{c_char, CStr};

pub struct DeviceInfo {
    pub extensions: Vec<vk::ExtensionProperties>,

    pub properties_10: vk::PhysicalDeviceProperties,
    pub properties_11: vk::PhysicalDeviceVulkan11Properties,
    pub properties_12: vk::PhysicalDeviceVulkan12Properties,
    pub portability_properties: vk::PhysicalDevicePortabilitySubsetPropertiesKHR,

    pub features_10: vk::PhysicalDeviceFeatures,
    pub features_11: vk::PhysicalDeviceVulkan11Features,
    pub features_12: vk::PhysicalDeviceVulkan12Features,
    pub dynamic_rendering_features: vk::PhysicalDeviceDynamicRenderingFeaturesKHR,
    pub portability_features: vk::PhysicalDevicePortabilitySubsetFeaturesKHR,
    pub synchronization_2_features: vk::PhysicalDeviceSynchronization2FeaturesKHR,
}

impl DeviceInfo {
    pub fn load(instance: &ash::Instance, physical_device: vk::PhysicalDevice) -> DeviceInfo {
        let extensions = unsafe {
            instance
                .enumerate_device_extension_properties(physical_device)
                .unwrap_or_default()
                .to_vec()
        };

        let mut properties_11 = vk::PhysicalDeviceVulkan11Properties::default();
        let mut properties_12 = vk::PhysicalDeviceVulkan12Properties::default();
        let mut portability_properties =
            vk::PhysicalDevicePortabilitySubsetPropertiesKHR::default();

        // Unconditionally required properties
        let mut properties = vk::PhysicalDeviceProperties2::builder()
            .push_next(&mut properties_11)
            .push_next(&mut properties_12);

        // Safety: we assume all the strings vulkan gives us are valid
        unsafe {
            // We load the portability subset properties if the extension is present
            if Self::list_contains_extension_cstr(&extensions, vk::KhrPortabilitySubsetFn::name()) {
                properties = properties.push_next(&mut portability_properties)
            }
        };

        let properties_10 = unsafe {
            instance.get_physical_device_properties2(physical_device, &mut properties);
            properties.properties
        };

        let mut features_11 = vk::PhysicalDeviceVulkan11Features::default();
        let mut features_12 = vk::PhysicalDeviceVulkan12Features::default();
        let mut dynamic_rendering_features =
            vk::PhysicalDeviceDynamicRenderingFeaturesKHR::default();
        let mut portability_features = vk::PhysicalDevicePortabilitySubsetFeaturesKHR::default();
        let mut synchronization_2_features =
            vk::PhysicalDeviceSynchronization2FeaturesKHR::default();

        // Glue all the feature extension structs together into our monster instance
        let mut features = vk::PhysicalDeviceFeatures2::builder()
            .push_next(&mut features_11)
            .push_next(&mut features_12);

        unsafe {
            if Self::list_contains_extension_cstr(&extensions, vk::KhrDynamicRenderingFn::name()) {
                features = features.push_next(&mut dynamic_rendering_features)
            }
        };
        unsafe {
            if Self::list_contains_extension_cstr(&extensions, vk::KhrPortabilitySubsetFn::name()) {
                features = features.push_next(&mut portability_features)
            }
        };
        unsafe {
            if Self::list_contains_extension_cstr(&extensions, vk::KhrSynchronization2Fn::name()) {
                features = features.push_next(&mut synchronization_2_features)
            }
        };

        let features_10 = unsafe {
            instance.get_physical_device_features2(physical_device, &mut features);
            features.features
        };

        // Null the p_next chain pointers to avoid leaving the dangling references. They can't be
        // *used* without unsafe but better be careful.
        properties_11.p_next = std::ptr::null_mut();
        properties_12.p_next = std::ptr::null_mut();
        portability_properties.p_next = std::ptr::null_mut();
        features_11.p_next = std::ptr::null_mut();
        features_12.p_next = std::ptr::null_mut();
        dynamic_rendering_features.p_next = std::ptr::null_mut();
        portability_features.p_next = std::ptr::null_mut();
        synchronization_2_features.p_next = std::ptr::null_mut();

        Self {
            extensions,
            properties_10,
            properties_11,
            properties_12,
            portability_properties,
            features_10,
            features_11,
            features_12,
            dynamic_rendering_features,
            portability_features,
            synchronization_2_features,
        }
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
