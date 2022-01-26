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

use crate::{AppInfo, EngineInfo, Entry};
use erupt::extensions::ext_debug_utils::{
    DebugUtilsMessageSeverityFlagsEXT, DebugUtilsMessageTypeFlagsEXT,
    DebugUtilsMessengerCreateInfoEXTBuilder, DebugUtilsMessengerEXT,
};
use erupt::extensions::khr_surface::SurfaceKHR;
use erupt::InstanceLoader;
use raw_window_handle::HasRawWindowHandle;
use std::ffi::CString;
use std::ops::Deref;
use std::sync::Arc;

///
/// A builder wrapper for creating a vulkan instance
///
pub struct InstanceBuilder {
    debug: bool,
    validation: bool,
}

impl InstanceBuilder {
    ///
    /// Construct a new instance builder.
    ///
    /// Defaults to having all debugging and validation off
    ///
    pub fn new() -> Self {
        Self {
            debug: false,
            validation: false,
        }
    }

    ///
    /// Whether to load debug utils extension
    ///
    pub fn debug(mut self, debug: bool) -> Self {
        self.debug = debug;
        self
    }

    ///
    /// Whether to use validation layers
    ///
    pub fn validation(mut self, validation: bool) -> Self {
        self.validation = validation;
        if validation {
            self.debug = true;
        }
        self
    }

    ///
    /// Construct the instance
    ///
    pub fn build(
        self,
        entry_loader: &Entry,
        window_handle: &impl HasRawWindowHandle,
        app_info: &AppInfo,
        engine_info: &EngineInfo,
    ) -> Instance {
        // Create the vulkan instance
        let (instance_loader, version) = Self::create_instance(
            entry_loader,
            app_info,
            engine_info,
            window_handle,
            self.debug,
            self.validation,
        );

        let messenger = if self.validation {
            Some(Self::install_debug_messenger(&instance_loader))
        } else {
            None
        };

        // Create a surface for the window we're making an instance for
        aleph_log::trace!("Creating Vulkan surface");
        let surface = unsafe {
            crate::surface::create_surface(&instance_loader, window_handle, None)
                .expect("Failed to create surface")
        };

        Instance {
            inner: Arc::new(Inner {
                _entry_loader: entry_loader.clone(),
                instance_loader,
                surface,
                version,
                messenger,
            }),
        }
    }

    ///
    /// Creates a vulkan instance and returns the instance handle
    ///
    fn create_instance<T>(
        entry_loader: &erupt::CustomEntryLoader<T>,
        app_info: &AppInfo,
        engine_info: &EngineInfo,
        window_handle: &impl HasRawWindowHandle,
        debug: bool,
        validation: bool,
    ) -> (erupt::InstanceLoader, u32) {
        use erupt::vk1_0::make_api_version;

        // Fill out ApplicationInfo for creating a vulkan instance
        let app_name_cstr = CString::new(app_info.name).unwrap();
        let app_version = make_api_version(
            0,
            app_info.major_version,
            app_info.minor_version,
            app_info.patch_version,
        );
        let engine_name_cstr = CString::new(engine_info.name).unwrap();
        let engine_version = make_api_version(
            0,
            engine_info.major_version,
            engine_info.minor_version,
            engine_info.patch_version,
        );
        let api_version = Self::assert_version_supported(entry_loader, 1, 2);
        let app_info = erupt::vk1_0::ApplicationInfoBuilder::new()
            .application_name(&app_name_cstr)
            .application_version(app_version)
            .engine_name(&engine_name_cstr)
            .engine_version(engine_version)
            .api_version(api_version);

        let mut extensions = erupt::utils::surface::enumerate_required_extensions(window_handle)
            .expect("Failed to get required vulkan surface extensions");
        if debug {
            extensions.push(erupt::extensions::ext_debug_utils::EXT_DEBUG_UTILS_EXTENSION_NAME);
        }

        let mut layers = Vec::new();
        if validation {
            layers.push(crate::cstr_ptr!("VK_LAYER_KHRONOS_validation"));
        }

        // Fill out InstanceCreateInfo for creating a vulkan instance
        let create_info = erupt::vk1_0::InstanceCreateInfoBuilder::new()
            .application_info(&app_info)
            .enabled_extension_names(&extensions)
            .enabled_layer_names(&layers);

        // Construct the vulkan instance
        aleph_log::trace!("Creating Vulkan instance");
        let instance_loader = unsafe {
            erupt::InstanceLoader::new(entry_loader, &create_info, None)
                .expect("Failed to initialize Vulkan instance loader")
        };

        (instance_loader, api_version)
    }

    fn assert_version_supported<T>(
        entry_loader: &erupt::CustomEntryLoader<T>,
        major_version: u32,
        minor_version: u32,
    ) -> u32 {
        // Get the latest supported API version
        let max_version = unsafe {
            entry_loader
                .enumerate_instance_version()
                .expect("Failed to get the latest supported instance version")
        };
        let max_version_major = erupt::vk1_0::api_version_major(max_version);
        let max_version_minor = erupt::vk1_0::api_version_minor(max_version);

        // Check if the major version is supported
        if max_version_major < major_version {
            panic!(
                "Current driver or GPU doesn't support Vulkan {}.x",
                major_version
            );
        }

        // Check if the minor version is supported
        if max_version_minor < minor_version {
            panic!(
                "Current driver or GPU doesn't support Vulkan {}.{}",
                major_version, minor_version
            );
        }

        // Return the packed version
        erupt::vk1_0::make_api_version(0, major_version, minor_version, 0)
    }

    ///
    ///
    ///
    fn install_debug_messenger(instance_loader: &erupt::InstanceLoader) -> DebugUtilsMessengerEXT {
        aleph_log::trace!("Installing VK_EXT_debug_utils messenger");
        let create_info = DebugUtilsMessengerCreateInfoEXTBuilder::new()
            .message_severity(
                DebugUtilsMessageSeverityFlagsEXT::ERROR_EXT
                    | DebugUtilsMessageSeverityFlagsEXT::INFO_EXT
                    | DebugUtilsMessageSeverityFlagsEXT::VERBOSE_EXT
                    | DebugUtilsMessageSeverityFlagsEXT::WARNING_EXT,
            )
            .message_type(
                DebugUtilsMessageTypeFlagsEXT::VALIDATION_EXT
                    | DebugUtilsMessageTypeFlagsEXT::PERFORMANCE_EXT,
            )
            .pfn_user_callback(Some(crate::debug::vulkan_debug_messenger));

        unsafe {
            instance_loader
                .create_debug_utils_messenger_ext(&create_info, None)
                .expect("Failed to install VK_EXT_debug_utils messenger")
        }
    }
}

impl Default for InstanceBuilder {
    fn default() -> Self {
        Self::new()
    }
}

///
/// A wrapper for representing a vulkan instance and it's dynamically loaded functions
///
#[derive(Clone)]
pub struct Instance {
    inner: Arc<Inner>,
}

impl Instance {
    ///
    /// Get a builder for constructing an instance. Just a wrapper for `InstanceBuilder::new`
    ///
    pub fn builder() -> InstanceBuilder {
        InstanceBuilder::new()
    }

    ///
    /// Gets the SurfaceKHR we made when creating the instance
    ///
    pub fn surface(&self) -> SurfaceKHR {
        self.inner.surface
    }

    /// Returns the major version of the vulkan instance
    pub fn major_version(&self) -> u32 {
        erupt::vk1_0::api_version_major(self.inner.version)
    }

    /// Returns the minor version of the vulkan instance
    pub fn minor_version(&self) -> u32 {
        erupt::vk1_0::api_version_minor(self.inner.version)
    }

    /// Returns the patch version of the vulkan instance
    pub fn patch_version(&self) -> u32 {
        erupt::vk1_0::api_version_patch(self.inner.version)
    }
}

impl Deref for Instance {
    type Target = InstanceLoader;

    fn deref(&self) -> &Self::Target {
        &self.inner.instance_loader
    }
}

struct Inner {
    _entry_loader: Entry,
    instance_loader: InstanceLoader,
    surface: SurfaceKHR,
    version: u32,
    messenger: Option<DebugUtilsMessengerEXT>,
}

impl Drop for Inner {
    fn drop(&mut self) {
        unsafe {
            aleph_log::trace!("Destroying Vulkan surface");
            self.instance_loader
                .destroy_surface_khr(Some(self.surface), None);
            if let Some(messenger) = self.messenger {
                aleph_log::trace!("Destroying debug messenger");
                self.instance_loader
                    .destroy_debug_utils_messenger_ext(Some(messenger), None);
            }
            aleph_log::trace!("Destroying Vulkan instance");
            self.instance_loader.destroy_instance(None);
        }
    }
}
