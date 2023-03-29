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

use crate::internal::strcpy::strcpy_str_to_cstr;
use aleph_vulkan_profiles::*;

#[allow(unused)]
pub const PROFILE_NAME: &str = VP_KHR_ROADMAP_2022_NAME;

#[allow(unused)]
pub const PROFILE_SPEC: u32 = VP_KHR_ROADMAP_2022_SPEC_VERSION;

#[allow(unused)]
pub const PROFILE_MIN_VERSION: u32 = VP_KHR_ROADMAP_2022_MIN_API_VERSION;

/// Utility function for constructing a [VpProfileProperties] instance with the
pub fn profile_props_from_loaders<T>(
    entry_loader: &erupt::CustomEntryLoader<T>,
    instance_loader: Option<&erupt::InstanceLoader>,
    profile_name: &str,
    spec_version: u32,
) -> VpProfileProperties {
    assert!(
        profile_name.len() < VP_MAX_PROFILE_NAME_SIZE,
        "name must be < {} to leave space for the null terminator",
        VP_MAX_PROFILE_NAME_SIZE
    );

    let get_physical_device_properties =
        instance_loader.map(|v| v.get_physical_device_properties.unwrap());
    let enumerate_device_extension_properties =
        instance_loader.map(|v| v.enumerate_device_extension_properties.unwrap());
    let create_device = instance_loader.map(|v| v.create_device.unwrap());

    let mut profile = VpProfileProperties {
        profileName: [0; VP_MAX_PROFILE_NAME_SIZE],
        specVersion: 0,
        vkGetInstanceProcAddr: entry_loader.get_instance_proc_addr,
        vkEnumerateInstanceExtensionProperties: entry_loader
            .enumerate_instance_extension_properties
            .unwrap(),
        vkCreateInstance: entry_loader.create_instance.unwrap(),
        vkGetPhysicalDeviceProperties: get_physical_device_properties,
        vkEnumerateDeviceExtensionProperties: enumerate_device_extension_properties,
        vkCreateDevice: create_device,
    };
    strcpy_str_to_cstr(profile_name, &mut profile.profileName);
    profile.specVersion = spec_version;

    profile
}
