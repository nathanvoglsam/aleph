/*
 *
 * This file is a part of NovaEngine
 * https:    ///gitlab.com/MindSpunk/NovaEngine
 *
 *
 * MIT License
 *
 * Copyright (c) 2020 Nathan Voglsam
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(clippy::all)]

use erupt::vk;
use std::ffi::{c_char, c_void};

pub const VP_ANDROID_BASELINE_2021_NAME: &str = "VP_ANDROID_baseline_2021";
pub const VP_ANDROID_BASELINE_2021_SPEC_VERSION: u32 = 2;
pub const VP_ANDROID_BASELINE_2021_MIN_API_VERSION: u32 = vk::make_api_version(0, 1, 0, 68);

pub const VP_ANDROID_BASELINE_2022_NAME: &str = "VP_ANDROID_baseline_2022";
pub const VP_ANDROID_BASELINE_2022_SPEC_VERSION: u32 = 1;
pub const VP_ANDROID_BASELINE_2022_MIN_API_VERSION: u32 = vk::make_api_version(0, 1, 1, 106);

pub const VP_KHR_ROADMAP_2022_NAME: &str = "VP_KHR_roadmap_2022";
pub const VP_KHR_ROADMAP_2022_SPEC_VERSION: u32 = 1;
pub const VP_KHR_ROADMAP_2022_MIN_API_VERSION: u32 = vk::make_api_version(0, 1, 3, 204);

pub const VP_LUNARG_DESKTOP_BASELINE_2022_NAME: &str = "VP_LUNARG_desktop_baseline_2022";
pub const VP_LUNARG_DESKTOP_BASELINE_2022_SPEC_VERSION: u32 = 1;
pub const VP_LUNARG_DESKTOP_BASELINE_2022_MIN_API_VERSION: u32 = vk::make_api_version(0, 1, 1, 139);

pub const VP_LUNARG_DESKTOP_PORTABILITY_2022_NAME: &str = "VP_LUNARG_desktop_portability_2022";
pub const VP_LUNARG_DESKTOP_PORTABILITY_2022_SPEC_VERSION: u32 = 1;
pub const VP_LUNARG_DESKTOP_PORTABILITY_2022_MIN_API_VERSION: u32 =
    vk::make_api_version(0, 1, 1, 208);

pub const VP_MAX_PROFILE_NAME_SIZE: usize = 256;

#[repr(C)]
pub struct VpProfileProperties {
    pub profileName: [c_char; VP_MAX_PROFILE_NAME_SIZE],
    pub specVersion: u32,
    pub vkGetInstanceProcAddr: vk::PFN_vkGetInstanceProcAddr,
    pub vkEnumerateInstanceExtensionProperties: vk::PFN_vkEnumerateInstanceExtensionProperties,
    pub vkCreateInstance: vk::PFN_vkCreateInstance,
    pub vkGetPhysicalDeviceProperties: Option<vk::PFN_vkGetPhysicalDeviceProperties>,
    pub vkEnumerateDeviceExtensionProperties: Option<vk::PFN_vkEnumerateDeviceExtensionProperties>,
    pub vkCreateDevice: Option<vk::PFN_vkCreateDevice>,
}

bitflags::bitflags! {
    /// Default behavior:
    /// - profile extensions are used (application must not specify extensions)
    #[repr(transparent)]
    pub struct VpInstanceCreateFlags: u32 {
        /// Merge application provided extension list and profile extension list
        const MERGE_EXTENSIONS_BIT = 0x00000001;

        /// Use application provided extension list
        const OVERRIDE_EXTENSIONS_BIT = 0x00000002;
    }
}

#[repr(C)]
pub struct VpInstanceCreateInfo {
    pub pCreateInfo: *const vk::InstanceCreateInfo,
    pub pProfile: *const VpProfileProperties,
    pub flags: VpInstanceCreateFlags,
}

bitflags::bitflags! {
    /// Default behavior:
    /// - profile extensions are used (application must not specify extensions)
    /// - profile feature structures are used (application must not specify any of them) extended
    ///   with any other application provided struct that isn't defined by the profile
    #[repr(transparent)]
    pub struct VpDeviceCreateFlags: u32 {
        /// Merge application provided extension list and profile extension list
        const MERGE_EXTENSIONS_BIT = 0x00000001;

        /// Use application provided extension list
        const OVERRIDE_EXTENSIONS_BIT = 0x00000002;

        // Merge application provided versions of feature structures with the profile features
        // Currently unsupported, but is considered for future inclusion in which case the
        // default behavior could potentially be changed to merging as the currently defined
        // default behavior is forward-compatible with that
        // const MERGE_FEATURES_BIT = 0x00000004;

        /// Use application provided versions of feature structures but use the profile feature
        /// structures when the application doesn't provide one (robust access disable flags are
        /// ignored if the application overrides the corresponding feature structures)
        const OVERRIDE_FEATURES_BIT = 0x00000008;

        // Only use application provided feature structures, don't add any profile specific
        // feature structures (robust access disable flags are ignored in this case and only the
        // application provided structures are used)
        const OVERRIDE_ALL_FEATURES_BIT = 0x00000010;

        const DISABLE_ROBUST_BUFFER_ACCESS_BIT = 0x00000020;
        const DISABLE_ROBUST_IMAGE_ACCESS_BIT = 0x00000040;
        const DISABLE_ROBUST_ACCESS = Self::DISABLE_ROBUST_BUFFER_ACCESS_BIT.bits | Self::DISABLE_ROBUST_IMAGE_ACCESS_BIT.bits;
    }
}

#[repr(C)]
pub struct VpDeviceCreateInfo {
    pub pCreateInfo: *const vk::DeviceCreateInfo,
    pub pProfile: *const VpProfileProperties,
    pub flags: VpDeviceCreateFlags,
}

pub type VpDebugMessageCallbackFn = extern "C" fn(*const c_char);

#[link(name = "vulkan_profiles", kind = "static")]
extern "C" {
    /// Query the list of available profiles in the library
    pub fn vpGetProfiles(
        pPropertyCount: *mut u32,
        pProperties: *mut VpProfileProperties,
    ) -> vk::Result;

    /// List the recommended fallback profiles of a profile
    pub fn vpGetProfileFallbacks(
        pProfile: *const VpProfileProperties,
        pPropertyCount: *mut u32,
        pProperties: *mut VpProfileProperties,
    ) -> vk::Result;

    /// Check whether a profile is supported at the instance level
    pub fn vpGetInstanceProfileSupport(
        pLayerName: *const c_char,
        pProfile: *const VpProfileProperties,
        pSupported: *mut vk::Bool32,
    ) -> vk::Result;

    /// Create a VkInstance with the profile instance extensions enabled
    pub fn vpCreateInstance(
        pCreateInfo: *const VpInstanceCreateInfo,
        pAllocator: *const vk::AllocationCallbacks,
        pInstance: *mut vk::Instance,
    ) -> vk::Result;

    /// Check whether a profile is supported by the physical device
    pub fn vpGetPhysicalDeviceProfileSupport(
        instance: vk::Instance,
        physicalDevice: vk::PhysicalDevice,
        pProfile: *const VpProfileProperties,
        pSupported: *mut vk::Bool32,
    ) -> vk::Result;

    /// Create a VkDevice with the profile features and device extensions enabled
    pub fn vpCreateDevice(
        physicalDevice: vk::PhysicalDevice,
        pCreateInfo: *const VpDeviceCreateInfo,
        pAllocator: *const vk::AllocationCallbacks,
        pDevice: *mut vk::Device,
    ) -> vk::Result;

    /// Query the list of instance extensions of a profile
    pub fn vpGetProfileInstanceExtensionProperties(
        pProfile: *const VpProfileProperties,
        pPropertyCount: *mut u32,
        pProperties: *mut vk::ExtensionProperties,
    ) -> vk::Result;

    /// Query the list of device extensions of a profile
    pub fn vpGetProfileDeviceExtensionProperties(
        pProfile: *const VpProfileProperties,
        pPropertyCount: *mut u32,
        pProperties: *mut vk::ExtensionProperties,
    ) -> vk::Result;

    /// Fill the feature structures with the requirements of a profile
    pub fn vpGetProfileFeatures(pProfile: *const VpProfileProperties, pNext: *mut c_void);

    /// Query the list of feature structure types specified by the profile
    pub fn vpGetProfileFeatureStructureTypes(
        pProfile: *const VpProfileProperties,
        pStructureTypeCount: *mut u32,
        pStructureTypes: *mut vk::StructureType,
    ) -> vk::Result;

    /// Fill the property structures with the requirements of a profile
    pub fn vpGetProfileProperties(pProfile: *const VpProfileProperties, pNext: *mut c_void);

    /// Query the list of property structure types specified by the profile
    pub fn vpGetProfilePropertyStructureTypes(
        pProfile: *const VpProfileProperties,
        pStructureTypeCount: *mut u32,
        pStructureTypes: *mut vk::StructureType,
    ) -> vk::Result;

    /// Query the requirements of queue families by a profile
    pub fn vpGetProfileQueueFamilyProperties(
        pProfile: *const VpProfileProperties,
        pPropertyCount: *mut u32,
        pProperties: *mut vk::QueueFamilyProperties2KHR,
    ) -> vk::Result;

    /// Query the list of query family structure types specified by the profile
    pub fn vpGetProfileQueueFamilyStructureTypes(
        pProfile: *const VpProfileProperties,
        pStructureTypeCount: *mut u32,
        pStructureTypes: *mut vk::StructureType,
    ) -> vk::Result;

    /// Query the list of formats with specified requirements by a profile
    pub fn vpGetProfileFormats(
        pProfile: *const VpProfileProperties,
        pFormatCount: *mut u32,
        pFormats: *mut vk::Format,
    ) -> vk::Result;

    /// Query the requirements of a format for a profile
    pub fn vpGetProfileFormatProperties(
        pProfile: *const VpProfileProperties,
        format: vk::Format,
        pNext: *mut c_void,
    );

    /// Query the list of format structure types specified by the profile
    pub fn vpGetProfileFormatStructureTypes(
        pProfile: *const VpProfileProperties,
        pStructureTypeCount: *mut u32,
        pStructureTypes: *mut vk::StructureType,
    ) -> vk::Result;

    /// Sets the debug callback to the given function pointer
    pub fn vpAlephSetCallback(pCallback: VpDebugMessageCallbackFn);
}
