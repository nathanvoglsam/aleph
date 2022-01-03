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

use crate::context::Context;
use crate::internal::messenger::vulkan_debug_messenger;
use erupt::vk;
use erupt::vk::{
    make_api_version, DebugUtilsMessageSeverityFlagsEXT, DebugUtilsMessageTypeFlagsEXT,
    DebugUtilsMessengerCreateInfoEXTBuilder, DebugUtilsMessengerEXT,
};
use interfaces::any::declare_interfaces;
use interfaces::gpu;
use interfaces::gpu::{ContextCreateError, ContextOptions, IGpuContext, IGpuContextProvider};
use std::ffi::CStr;
use std::marker::PhantomData;
use std::os::raw::c_char;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct ContextProvider {
    /// Flags whether a context has already been created
    context_made: AtomicBool,

    /// Hack to ensure ContextProvider is not Send/Sync
    no_send_sync: PhantomData<*const ()>,
}

impl ContextProvider {
    pub fn new() -> Self {
        Self {
            context_made: AtomicBool::new(false),
            no_send_sync: Default::default(),
        }
    }

    fn create_instance<T>(
        entry_loader: &erupt::CustomEntryLoader<T>,
        debug: bool,
        validation: bool,
    ) -> Result<(erupt::InstanceLoader, u32), ContextCreateError> {
        let extensions = instance_extensions(entry_loader, debug)?;
        let layers = instance_layers(entry_loader, validation)?;
        let app_info = app_and_engine_info(entry_loader)?;

        let api_version = app_info.api_version;

        // Fill out InstanceCreateInfo for creating a vulkan instance
        let create_info = erupt::vk1_0::InstanceCreateInfoBuilder::new()
            .application_info(&app_info)
            .enabled_extension_names(&extensions)
            .enabled_layer_names(&layers);

        // Construct the vulkan instance
        aleph_log::trace!("Creating Vulkan instance");
        let instance_loader = unsafe {
            erupt::InstanceLoader::new(entry_loader, &create_info, None).map_err(|e| {
                let e = Box::new(e);
                ContextCreateError::Platform(e)
            })?
        };

        Ok((instance_loader, api_version))
    }
}

impl IGpuContextProvider for ContextProvider {
    fn make_context(
        &self,
        options: &ContextOptions,
    ) -> Result<Box<dyn IGpuContext>, ContextCreateError> {
        match self
            .context_made
            .compare_exchange(false, true, Ordering::Relaxed, Ordering::Relaxed)
        {
            Ok(_) => {
                let entry_loader = erupt::EntryLoader::new().map_err(|e| {
                    let e = Box::new(e);
                    ContextCreateError::Platform(e)
                })?;

                let (instance_loader, _version) =
                    Self::create_instance(&entry_loader, options.validation, options.validation)?;

                let messenger = if options.validation {
                    match install_debug_messenger(&instance_loader) {
                        Ok(v) => Some(v),
                        Err(e) => {
                            aleph_log::warn!(
                                "Context validation requested but couldn't create debug messenger"
                            );
                            aleph_log::warn!("Reason: {:?}", e);
                            aleph_log::warn!("No validation messages will be logged");
                            None
                        }
                    }
                } else {
                    None
                };

                let context = Context {
                    instance_loader,
                    messenger,
                };
                let context = Box::new(context);

                Ok(context)
            }
            Err(_) => Err(ContextCreateError::ContextAlreadyCreated),
        }
    }
}

declare_interfaces!(ContextProvider, [IGpuContextProvider]);

fn instance_extensions<T>(
    entry_loader: &erupt::CustomEntryLoader<T>,
    debug: bool,
) -> Result<Vec<*const c_char>, ContextCreateError> {
    use erupt::extensions::*;

    let supported_instance_extensions = unsafe {
        entry_loader
            .enumerate_instance_extension_properties(None, None)
            .map_err(|e| {
                let e = Box::new(e);
                ContextCreateError::Platform(e)
            })?
    };

    // Get surface extensions
    let mut extensions = Vec::new();

    // Push the base surface extension
    extensions.push(khr_surface::KHR_SURFACE_EXTENSION_NAME);

    // Push all possible WSI extensions for the underlying platform
    if cfg!(all(
        unix,
        not(target_os = "android"),
        not(target_os = "macos")
    )) {
        // This is the branch for linux. Linux has a bunch of WSI extensions so add them all,
        // any unsupported extensions will be stripped later.
        extensions.push(khr_xlib_surface::KHR_XLIB_SURFACE_EXTENSION_NAME);
        extensions.push(khr_xcb_surface::KHR_XCB_SURFACE_EXTENSION_NAME);
        extensions.push(khr_wayland_surface::KHR_WAYLAND_SURFACE_EXTENSION_NAME);
    }
    if cfg!(target_os = "android") {
        // Android, only one. A sane platform
        extensions.push(khr_android_surface::KHR_ANDROID_SURFACE_EXTENSION_NAME);
    }
    if cfg!(target_os = "windows") {
        // Windows, again a single WSI extension.
        extensions.push(khr_win32_surface::KHR_WIN32_SURFACE_EXTENSION_NAME);
    }

    // Add the debug extension if requested
    if debug {
        extensions.push(ext_debug_utils::EXT_DEBUG_UTILS_EXTENSION_NAME);
    }

    // Strip all unsupported extensions
    extensions.retain(|v| {
        // SAFETY: Everything is guaranteed to be a C string here
        unsafe {
            let v = CStr::from_ptr(*v);

            // Strip any unsupported extensions
            supported_instance_extensions
                .iter()
                .map(|s| CStr::from_ptr(s.extension_name.as_ptr()))
                .any(|s| v == s)
        }
    });

    unsafe {
        let debug_extensions = CStr::from_ptr(ext_debug_utils::EXT_DEBUG_UTILS_EXTENSION_NAME);
        let debug_loaded = supported_instance_extensions
            .iter()
            .map(|s| CStr::from_ptr(s.extension_name.as_ptr()))
            .any(|s| debug_extensions == s);
        if debug && !debug_loaded {
            aleph_log::warn!("Debug context requested but debug extensions failed to load");
        }
    }

    Ok(extensions)
}

fn instance_layers<T>(
    entry_loader: &erupt::CustomEntryLoader<T>,
    validation: bool,
) -> Result<Vec<*const c_char>, ContextCreateError> {
    let supported_instance_layers = unsafe {
        entry_loader
            .enumerate_instance_layer_properties(None)
            .map_err(|e| {
                let e = Box::new(e);
                ContextCreateError::Platform(e)
            })?
    };

    let mut layers = Vec::new();
    if validation {
        layers.push(crate::cstr_ptr!("VK_LAYER_KHRONOS_validation"));
    }

    layers.retain(|v| {
        // SAFETY: Everything is guaranteed to be a C string here
        unsafe {
            let v = CStr::from_ptr(*v);

            // Strip any unsupported extensions
            supported_instance_layers
                .iter()
                .map(|s| CStr::from_ptr(s.layer_name.as_ptr()))
                .any(|s| v == s)
        }
    });

    unsafe {
        let validation_layer = crate::cstr!("VK_LAYER_KHRONOS_validation");
        let validation_loaded = supported_instance_layers
            .iter()
            .map(|s| CStr::from_ptr(s.layer_name.as_ptr()))
            .any(|s| validation_layer == s);
        if validation && !validation_loaded {
            aleph_log::warn!("Validation context requested but no validation layers are available");
        }
    }

    Ok(layers)
}

fn app_and_engine_info<T>(
    entry_loader: &erupt::CustomEntryLoader<T>,
) -> Result<vk::ApplicationInfoBuilder, ContextCreateError> {
    let api_version = assert_version_supported(entry_loader, 1, 2)?;
    let info = vk::ApplicationInfoBuilder::new()
        .application_name(crate::cstr!("aleph-gpu"))
        .application_version(make_api_version(
            0,
            gpu::API_VERSION_MAJOR.parse().unwrap(),
            gpu::API_VERSION_MINOR.parse().unwrap(),
            gpu::API_VERSION_PATCH.parse().unwrap(),
        ))
        .engine_name(crate::cstr!("aleph-gpu-vulkan"))
        .engine_version(make_api_version(
            0,
            env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap(),
            env!("CARGO_PKG_VERSION_MINOR").parse().unwrap(),
            env!("CARGO_PKG_VERSION_PATCH").parse().unwrap(),
        ))
        .api_version(api_version);
    Ok(info)
}

fn assert_version_supported<T>(
    entry_loader: &erupt::CustomEntryLoader<T>,
    major_version: u32,
    minor_version: u32,
) -> Result<u32, ContextCreateError> {
    // Get the latest supported API version
    let max_version = unsafe {
        entry_loader.enumerate_instance_version().map_err(|e| {
            let e = Box::new(e);
            ContextCreateError::Platform(e)
        })?
    };
    let max_version_major = erupt::vk1_0::api_version_major(max_version);
    let max_version_minor = erupt::vk1_0::api_version_minor(max_version);

    // Check if the major version is supported
    if max_version_major < major_version {
        let e = format!(
            "Current driver or GPU doesn't support Vulkan {}.x",
            major_version
        );
        return Err(ContextCreateError::Platform(Box::new(e)));
    }

    // Check if the minor version is supported
    if max_version_minor < minor_version {
        let e = format!(
            "Current driver or GPU doesn't support Vulkan {}.{}",
            major_version, minor_version
        );
        return Err(ContextCreateError::Platform(Box::new(e)));
    }

    // Return the packed version
    let version = erupt::vk1_0::make_api_version(0, major_version, minor_version, 0);
    Ok(version)
}

fn install_debug_messenger(
    instance_loader: &erupt::InstanceLoader,
) -> Result<DebugUtilsMessengerEXT, ContextCreateError> {
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
        .pfn_user_callback(Some(vulkan_debug_messenger));

    unsafe {
        instance_loader
            .create_debug_utils_messenger_ext(&create_info, None)
            .map_err(|e| {
                let e = Box::new(e);
                ContextCreateError::Platform(e)
            })
    }
}
