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

use std::ffi::{CStr, c_char};

use ash::vk;

use crate::internal::features::CheckMeetsMinimum;
use crate::internal::profile::CreateProfile;

#[derive(Clone, Default)]
#[rustfmt::skip]
pub struct DeviceInfo {
    pub extensions: Vec<vk::ExtensionProperties>,
    pub properties_10: vk::PhysicalDeviceProperties,
    pub properties_11: vk::PhysicalDeviceVulkan11Properties<'static>,
    pub properties_12: vk::PhysicalDeviceVulkan12Properties<'static>,
    pub properties_13: vk::PhysicalDeviceVulkan13Properties<'static>,
    pub portability_properties: vk::PhysicalDevicePortabilitySubsetPropertiesKHR<'static>,
    pub features_10: vk::PhysicalDeviceFeatures,
    pub features_11: vk::PhysicalDeviceVulkan11Features<'static>,
    pub features_12: vk::PhysicalDeviceVulkan12Features<'static>,
    pub features_13: vk::PhysicalDeviceVulkan13Features<'static>,
    pub portability_features: vk::PhysicalDevicePortabilitySubsetFeaturesKHR<'static>,
    pub memory_properties: vk::PhysicalDeviceMemoryProperties2<'static>,
}

impl DeviceInfo {
    pub fn minimum() -> Self {
        fn make_ext_prop(v: &'static str) -> vk::ExtensionProperties {
            assert!(
                v.len() < vk::MAX_EXTENSION_NAME_SIZE - 1,
                "Extension name must be shorter than 255"
            );

            // Zero initialized buffer. Max length of 'v' is 255 to leave one space for a null
            // terminator.
            let mut props = vk::ExtensionProperties {
                extension_name: [0; vk::MAX_EXTENSION_NAME_SIZE],
                spec_version: 0,
            };

            // Memcpy the source string into the destination buffer
            unsafe {
                assert_eq!(size_of::<u8>(), size_of::<c_char>());
                let src = v.as_ptr() as *const c_char;
                let dst = props.extension_name.as_mut_ptr() as *mut c_char;
                std::ptr::copy(src, dst, v.len());
            }

            props
        }
        Self {
            extensions: vec![
                make_ext_prop("VK_KHR_push_descriptor\0"),
                make_ext_prop("VK_KHR_swapchain_mutable_format\0"),
                make_ext_prop("VK_KHR_image_format_list\0"),
            ],
            properties_10: CreateProfile::minimum(),
            properties_11: CreateProfile::minimum(),
            properties_12: CreateProfile::minimum(),
            properties_13: CreateProfile::minimum(),
            portability_properties: Default::default(),
            features_10: CreateProfile::minimum(),
            features_11: CreateProfile::minimum(),
            features_12: CreateProfile::minimum(),
            features_13: CreateProfile::minimum(),
            portability_features: Default::default(),
            memory_properties: Default::default(),
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
            properties_12,
            properties_13,
            portability_properties,
            features_10,
            features_11,
            features_12,
            features_13,
            portability_features,
            memory_properties,
        } = &mut out;

        // Unconditionally required properties
        let mut properties = vk::PhysicalDeviceProperties2::default()
            .push_next(properties_11)
            .push_next(properties_12)
            .push_next(properties_13);

        // Safety: we assume all the strings vulkan gives us are valid
        unsafe {
            // We load the portability subset properties if the extension is present
            if Self::list_contains_extension_cstr(extensions, ash::khr::portability_subset::NAME) {
                properties = properties.push_next(portability_properties)
            }
        };

        *properties_10 = unsafe {
            instance.get_physical_device_properties2(physical_device, &mut properties);
            properties.properties
        };

        // Glue all the feature extension structs together into our monster instance
        let mut features = vk::PhysicalDeviceFeatures2::default()
            .push_next(features_11)
            .push_next(features_12)
            .push_next(features_13);

        unsafe {
            if Self::list_contains_extension_cstr(extensions, ash::khr::portability_subset::NAME) {
                features = features.push_next(portability_features)
            }
        };

        *features_10 = unsafe {
            instance.get_physical_device_features2(physical_device, &mut features);
            features.features
        };
        unsafe {
            instance.get_physical_device_memory_properties2(physical_device, memory_properties);
        }

        // Null the p_next chain pointers to avoid leaving the dangling references. They can't be
        // *used* without unsafe but better be careful.
        out.null_p_next_ptrs();

        out
    }

    fn null_p_next_ptrs(&mut self) {
        self.properties_11.p_next = std::ptr::null_mut();
        self.properties_12.p_next = std::ptr::null_mut();
        self.properties_13.p_next = std::ptr::null_mut();
        self.portability_properties.p_next = std::ptr::null_mut();
        self.features_11.p_next = std::ptr::null_mut();
        self.features_12.p_next = std::ptr::null_mut();
        self.features_13.p_next = std::ptr::null_mut();
        self.portability_features.p_next = std::ptr::null_mut();
        self.memory_properties.p_next = std::ptr::null_mut();
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
        let mut s = Some(());

        let DeviceInfo { extensions: minimum_extensions, .. } = Self::minimum();
        for required in minimum_extensions {
            if !self.supports_extension_ptr(required.extension_name.as_ptr()) {
                let v = unsafe {
                    CStr::from_ptr(required.extension_name.as_ptr()).to_str().unwrap()
                };
                log::error!("Device missing required extension: \"{v}\"");
                s = None;
            }
        }

        if self.properties_10.meets_minimum().is_none() {
            s = None;
        }
        if self.properties_11.meets_minimum().is_none() {
            s = None;
        }
        if self.properties_12.meets_minimum().is_none() {
            s = None;
        }
        if self.properties_13.meets_minimum().is_none() {
            s = None;
        }
        if self.portability_properties.meets_minimum().is_none() {
            s = None;
        }
        if self.features_10.meets_minimum().is_none() {
            s = None;
        }
        if self.features_11.meets_minimum().is_none() {
            s = None;
        }
        if self.features_12.meets_minimum().is_none() {
            s = None;
        }
        if self.features_13.meets_minimum().is_none() {
            s = None;
        }
        if self.portability_features.meets_minimum().is_none() {
            s = None;
        }

        s
    }

    unsafe fn list_contains_extension_ptr(
        extensions: &[vk::ExtensionProperties],
        wanted: *const c_char,
    ) -> bool {
        unsafe { Self::list_contains_extension_cstr(extensions, CStr::from_ptr(wanted)) }
    }

    unsafe fn list_contains_extension_cstr(
        extensions: &[vk::ExtensionProperties],
        wanted: &CStr,
    ) -> bool {
        unsafe { Self::list_contains_extension(extensions, wanted.to_str().unwrap_unchecked()) }
    }

    unsafe fn list_contains_extension(
        extensions: &[vk::ExtensionProperties],
        wanted: &str,
    ) -> bool {
        unsafe {
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
}

unsafe impl Send for DeviceInfo {}
unsafe impl Sync for DeviceInfo {}
