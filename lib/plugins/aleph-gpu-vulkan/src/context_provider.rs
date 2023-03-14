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
use crate::internal::{VK_MAJOR_VERSION, VK_MINOR_VERSION};
use erupt::vk;
use erupt::vk::{
    make_api_version, DebugUtilsMessageSeverityFlagsEXT, DebugUtilsMessageTypeFlagsEXT,
    DebugUtilsMessengerCreateInfoEXTBuilder, DebugUtilsMessengerEXT,
};
use interfaces::any::{declare_interfaces, AnyArc};
use interfaces::anyhow::anyhow;
use interfaces::gpu;
use interfaces::gpu::*;
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

declare_interfaces!(ContextProvider, [IContextProvider]);

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
        // If validation is requested we must force debug on as we require debug extensions to log
        // the validation messages
        let debug = if validation { true } else { debug };

        // Select the set of extensions that we want to load
        let wanted_extensions = get_wanted_extensions(debug);

        // Strip all unsupported extensions from the list of wanted extensions
        let supported_extensions =
            strip_unsupported_extensions(entry_loader, wanted_extensions.clone());

        // Log all unsupported extensions and error if we can't continue with the set of extensions
        // available
        check_all_extensions_supported(&wanted_extensions, &supported_extensions)?;

        // Select the set of layers we want to load
        let wanted_layers = get_wanted_layers(validation);

        // Strip all unsupported layers from the list of wanted layers
        let supported_layers = strip_unsupported_layers(entry_loader, wanted_layers.clone());

        // Log all unsupported layers and error if we can't continue with the set of layers
        // available
        check_all_layers_supported(&wanted_layers, &supported_layers)?;

        // We require at least vulkan 1.2. Get the API version the current system supports and assert
        // that the version is at least 1.2
        let api_version =
            assert_version_supported(entry_loader, VK_MAJOR_VERSION, VK_MINOR_VERSION)?;

        // Mandatory description we must give vulkan about our app
        let app_info = app_and_engine_info(api_version);

        // Fill out InstanceCreateInfo for creating a vulkan instance
        let create_info = erupt::vk1_0::InstanceCreateInfoBuilder::new()
            .application_info(&app_info)
            .enabled_extension_names(&supported_extensions)
            .enabled_layer_names(&supported_layers);

        // Construct the vulkan instance
        log::trace!("Creating Vulkan instance");
        let instance_loader = unsafe {
            erupt::InstanceLoader::new(entry_loader, &create_info).map_err(|e| anyhow!(e))?
        };

        Ok((instance_loader, api_version))
    }
}

impl IContextProvider for ContextProvider {
    fn make_context(
        &self,
        options: &ContextOptions,
    ) -> Result<AnyArc<dyn IContext>, ContextCreateError> {
        match self
            .context_made
            .compare_exchange(false, true, Ordering::Relaxed, Ordering::Relaxed)
        {
            Ok(_) => {
                let entry_loader = erupt::EntryLoader::new().map_err(|e| anyhow!(e))?;

                let (instance_loader, _version) =
                    Self::create_instance(&entry_loader, options.debug, options.validation)?;

                let messenger = if options.validation {
                    match install_debug_messenger(&instance_loader) {
                        Ok(v) => Some(v),
                        Err(e) => {
                            log::warn!(
                                "Context validation requested but couldn't create debug messenger"
                            );
                            log::warn!("Reason: {:?}", e);
                            log::warn!("No validation messages will be logged");
                            None
                        }
                    }
                } else {
                    None
                };

                let context = AnyArc::new_cyclic(move |v| Context {
                    this: v.clone(),
                    instance_loader,
                    messenger,
                });
                Ok(AnyArc::map::<dyn IContext, _>(context, |v| v))
            }
            Err(_) => Err(ContextCreateError::ContextAlreadyCreated),
        }
    }
}

fn get_wanted_extensions(debug: bool) -> Vec<*const c_char> {
    use erupt::extensions::*;

    // Get surface extensions
    // Push the base surface extension
    let mut extensions = vec![khr_surface::KHR_SURFACE_EXTENSION_NAME];

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

    extensions
}

fn strip_unsupported_extensions<T>(
    entry_loader: &erupt::CustomEntryLoader<T>,
    mut extensions: Vec<*const c_char>,
) -> Vec<*const c_char> {
    let supported_instance_extensions = unsafe {
        entry_loader
            .enumerate_instance_extension_properties(None, None)
            .result()
            .unwrap_or_default()
    };

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
    extensions
}

fn check_all_layers_supported(
    wanted_extensions: &[*const c_char],
    supported_extensions: &[*const c_char],
) -> Result<(), ContextCreateError> {
    let mut missing_extensions = diff_lists(wanted_extensions, supported_extensions).peekable();
    if missing_extensions.peek().is_some() {
        for missing in missing_extensions {
            log::error!("Runtime requested unsupported extension '{:#?}'.", missing);
        }
        return Err(ContextCreateError::Platform(anyhow!(
            "Unsupported extension is required by runtime"
        )));
    }
    Ok(())
}

fn get_wanted_layers(validation: bool) -> Vec<*const c_char> {
    let mut layers = Vec::new();
    if validation {
        layers.push(crate::cstr_ptr!("VK_LAYER_KHRONOS_validation"));
    }
    layers
}

fn strip_unsupported_layers<T>(
    entry_loader: &erupt::CustomEntryLoader<T>,
    mut layers: Vec<*const c_char>,
) -> Vec<*const c_char> {
    let supported_instance_layers = unsafe {
        entry_loader
            .enumerate_instance_layer_properties(None)
            .result()
            .unwrap_or_default()
    };

    // Strip all unsupported layers
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
    layers
}

fn check_all_extensions_supported(
    wanted_layers: &[*const c_char],
    supported_layers: &[*const c_char],
) -> Result<(), ContextCreateError> {
    let mut missing_layers = diff_lists(wanted_layers, supported_layers).peekable();
    if missing_layers.peek().is_some() {
        for missing in missing_layers {
            log::error!("Runtime requested unsupported layer '{:#?}'.", missing);
        }
        return Err(ContextCreateError::Platform(anyhow!(
            "Unsupported layer is required by runtime"
        )));
    }
    Ok(())
}

fn diff_lists<'a>(
    list_a: &'a [*const c_char],
    list_b: &'a [*const c_char],
) -> impl Iterator<Item = &'a CStr> {
    unsafe {
        list_a
            .iter()
            .copied()
            .map(|v| CStr::from_ptr(v))
            .filter(|a| {
                let in_both = list_b
                    .iter()
                    .copied()
                    .map(|v| CStr::from_ptr(v))
                    .any(|b| *a == b);
                !in_both
            })
    }
}

fn app_and_engine_info<'a>(api_version: u32) -> vk::ApplicationInfoBuilder<'a> {
    vk::ApplicationInfoBuilder::new()
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
        .api_version(api_version)
}

fn assert_version_supported<T>(
    entry_loader: &erupt::CustomEntryLoader<T>,
    major_version: u32,
    minor_version: u32,
) -> Result<u32, ContextCreateError> {
    // Get the latest supported API version
    let max_version = unsafe {
        entry_loader
            .enumerate_instance_version()
            .map_err(|e| anyhow!(e))?
    };
    let max_version_major = erupt::vk1_0::api_version_major(max_version);
    let max_version_minor = erupt::vk1_0::api_version_minor(max_version);

    // Check if the major version is supported
    if max_version_major < major_version {
        let e = format!("Current driver or GPU doesn't support Vulkan {major_version}.x",);
        return Err(ContextCreateError::Platform(anyhow!(e)));
    }

    // Check if the minor version is supported
    if max_version_minor < minor_version {
        let e = format!(
            "Current driver or GPU doesn't support Vulkan {major_version}.{minor_version}",
        );
        return Err(ContextCreateError::Platform(anyhow!(e)));
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
            .map_err(|e| ContextCreateError::Platform(anyhow!(e)))
    }
}
