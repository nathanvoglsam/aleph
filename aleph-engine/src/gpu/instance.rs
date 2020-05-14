//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use crate::app::{AppInfo, ENGINE_NAME_CSTR, ENGINE_VERSION_VK};
use crate::gpu;
use erupt::extensions::ext_debug_utils::{
    DebugUtilsMessageSeverityFlagsEXT, DebugUtilsMessageTypeFlagsEXT,
    DebugUtilsMessengerCreateInfoEXTBuilder, DebugUtilsMessengerEXT,
    ExtDebugUtilsInstanceLoaderExt,
};
use erupt::vk1_0::{Vk10CoreLoaderExt, Vk10InstanceLoaderExt};
use erupt::{CoreLoader, InstanceLoader};
use libloading::Library;
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use std::ffi::CString;
use std::sync::Arc;

///
/// An abusrd internal wrapper because of erupt's interface needing a concrete type
///
struct AbsurdWindowWrapper<'a> {
    window: &'a dyn HasRawWindowHandle,
}

unsafe impl<'a> HasRawWindowHandle for AbsurdWindowWrapper<'a> {
    fn raw_window_handle(&self) -> RawWindowHandle {
        self.window.raw_window_handle()
    }
}

///
/// A builder wrapper for creating a vulkan instance
///
pub struct InstanceBuilder<'a> {
    debug: bool,
    validation: bool,
    window: Option<AbsurdWindowWrapper<'a>>,
}

impl<'a> InstanceBuilder<'a> {
    ///
    /// Construct a new instance builder.
    ///
    /// Defaults to having all debugging and validation off
    ///
    pub fn new() -> Self {
        Self {
            debug: false,
            validation: false,
            window: None,
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
        self
    }

    ///
    /// Create the instance so that it supports presenting to the given surface
    ///
    pub fn surface(mut self, window: &'a dyn HasRawWindowHandle) -> Self {
        self.window = Some(AbsurdWindowWrapper { window });
        self
    }

    ///
    /// Construct the instance
    ///
    pub fn build(self, app_info: &AppInfo) -> Arc<Instance> {
        // Load core vulkan functions for creating an instance
        let core_loader = Self::load_vulkan_core();

        // Create the vulkan instance
        let instance = Self::create_instance(
            &core_loader,
            app_info,
            self.window.as_ref().unwrap(),
            self.debug,
            self.validation,
        );

        // Load the vulkan instance functions
        let instance_loader = Self::load_vulkan_instance(
            &core_loader,
            instance,
            self.window.as_ref().unwrap(),
            self.debug,
        );

        let messenger = if self.validation {
            Some(Self::install_debug_messenger(&instance_loader))
        } else {
            None
        };

        let instance = Instance {
            _core_loader: Arc::new(core_loader),
            instance_loader: Arc::new(instance_loader),
            messenger,
        };
        Arc::new(instance)
    }

    ///
    /// Loads the core vulkan functions required for creating a VkInstance
    ///
    fn load_vulkan_core() -> erupt::CoreLoader<libloading::Library> {
        // Load core vulkan functions
        log::trace!("Initializing Vulkan Core Loader");
        let mut core_loader =
            erupt::CoreLoader::new().expect("Failed to create Vulkan core loader");

        // Load vulkan 1.0 core functions
        log::trace!("Loading Core Functions for Vulkan 1.0");
        core_loader.load_vk1_0().expect("Failed to load Vulkan 1.0");
        core_loader
    }

    ///
    /// Creates a vulkan instance and returns the instance handle
    ///
    fn create_instance<T>(
        core_loader: &erupt::CoreLoader<T>,
        app_info: &AppInfo,
        window: &impl HasRawWindowHandle,
        debug: bool,
        validation: bool,
    ) -> erupt::vk1_0::Instance {
        // Fill out ApplicationInfo for creating a vulkan instance
        let app_name_cstr = CString::new(app_info.name.as_str()).unwrap();
        let app_version = erupt::make_version(app_info.major, app_info.minor, app_info.patch);
        let engine_name: &CString = &ENGINE_NAME_CSTR;
        let engine_version = ENGINE_VERSION_VK;
        let api_version = erupt::make_version(1, 0, 0);
        let app_info = erupt::vk1_0::ApplicationInfoBuilder::new()
            .application_name(&app_name_cstr)
            .application_version(app_version)
            .engine_name(engine_name)
            .engine_version(engine_version)
            .api_version(api_version);

        let mut extensions = erupt::utils::surface::enumerate_required_extensions(window)
            .expect("Failed to get required vulkan surface extensions");
        if debug {
            extensions.push(erupt::extensions::ext_debug_utils::EXT_DEBUG_UTILS_EXTENSION_NAME);
        }

        let mut layers = Vec::new();
        if validation {
            layers.push(erupt::cstr!("VK_LAYER_LUNARG_standard_validation"));
        }

        // Fill out InstanceCreateInfo for creating a vulkan instance
        let create_info = erupt::vk1_0::InstanceCreateInfoBuilder::new()
            .application_info(&app_info)
            .enabled_extension_names(&extensions)
            .enabled_layer_names(&layers);

        // Construct the vulkan instance
        log::info!("Creating Vulkan instance");
        unsafe {
            let instance = core_loader.create_instance(&create_info, None, None);
            instance.expect("Failed to create Vulkan instance")
        }
    }

    ///
    /// Loads the vulkan functions that require an instance before they can be loaded
    ///
    fn load_vulkan_instance<T>(
        core_loader: &erupt::CoreLoader<T>,
        instance: erupt::vk1_0::Instance,
        window: &impl HasRawWindowHandle,
        debug: bool,
    ) -> erupt::InstanceLoader {
        // Load the vulkan instance function pointers
        log::info!("Loading Vulkan Instance functions");
        let mut instance_loader = erupt::InstanceLoader::new(core_loader, instance)
            .expect("Failed to initialize Vulkan instance loader");
        instance_loader
            .load_vk1_0()
            .expect("Failed to load vulkan functions");
        if debug {
            instance_loader
                .load_ext_debug_utils()
                .expect("Failed to load VK_EXT_debug_utils functions");
        }

        unsafe { gpu::surface::load_surface_functions(&mut instance_loader, window) }

        instance_loader
    }

    ///
    ///
    ///
    fn install_debug_messenger(instance_loader: &erupt::InstanceLoader) -> DebugUtilsMessengerEXT {
        log::info!("Installing VK_EXT_debug_utils messenger");
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
            .pfn_user_callback(Some(gpu::debug::vulkan_debug_messenger));

        unsafe {
            instance_loader
                .create_debug_utils_messenger_ext(&create_info, None, None)
                .expect("Failed to install VK_EXT_debug_utils messenger")
        }
    }
}

///
/// A wrapper for representing a vulkan instance and it's dynamically loaded functions
///
pub struct Instance {
    _core_loader: Arc<CoreLoader<Library>>,
    instance_loader: Arc<InstanceLoader>,
    messenger: Option<DebugUtilsMessengerEXT>,
}

impl Instance {
    ///
    /// Get a builder for constructing an instance. Just a wrapper for `InstanceBuilder::new`
    ///
    pub fn builder<'a>() -> InstanceBuilder<'a> {
        InstanceBuilder::new()
    }

    ///
    /// Get a reference to the instance loader
    ///
    pub fn loader(&self) -> &Arc<InstanceLoader> {
        &self.instance_loader
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        unsafe {
            if let Some(messenger) = self.messenger {
                self.instance_loader
                    .destroy_debug_utils_messenger_ext(messenger, None);
            }
            self.instance_loader.destroy_instance(None);
        }
    }
}
