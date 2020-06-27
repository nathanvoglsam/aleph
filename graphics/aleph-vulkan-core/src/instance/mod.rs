//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use app_info::{
    engine_name_cstr, engine_version_major, engine_version_minor, engine_version_patch, AppInfo,
};
use erupt::extensions::ext_debug_utils::{
    DebugUtilsMessageSeverityFlagsEXT, DebugUtilsMessageTypeFlagsEXT,
    DebugUtilsMessengerCreateInfoEXTBuilder, DebugUtilsMessengerEXT,
    ExtDebugUtilsInstanceLoaderExt,
};
use erupt::extensions::khr_surface::{KhrSurfaceInstanceLoaderExt, SurfaceKHR};
use erupt::vk1_0::{Vk10CoreLoaderExt, Vk10InstanceLoaderExt};
use erupt::InstanceLoader;
use raw_window_handle::HasRawWindowHandle;
use std::ffi::CString;
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
        self
    }

    ///
    /// Construct the instance
    ///
    pub fn build(
        self,
        window_handle: &impl HasRawWindowHandle,
        app_info: &AppInfo,
    ) -> Arc<Instance> {
        // Load core vulkan functions for creating an instance
        let core_loader = Self::load_vulkan_core();

        // Create the vulkan instance
        let instance = Self::create_instance(
            &core_loader,
            app_info,
            window_handle,
            self.debug,
            self.validation,
        );

        // Load the vulkan instance functions
        let instance_loader =
            Self::load_vulkan_instance(&core_loader, instance, window_handle, self.debug);

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

        let instance = Instance {
            _core_loader: Arc::new(core_loader),
            instance_loader: Arc::new(instance_loader),
            surface,
            messenger,
        };
        Arc::new(instance)
    }

    ///
    /// Loads the core vulkan functions required for creating a VkInstance
    ///
    fn load_vulkan_core() -> erupt::utils::loading::DefaultCoreLoader {
        // Load core vulkan functions
        aleph_log::trace!("Initializing Vulkan Core Loader");
        let mut core_loader =
            erupt::CoreLoader::new().expect("Failed to create Vulkan core loader");

        // Load vulkan 1.0 core functions
        aleph_log::trace!("Loading Core Functions for Vulkan 1.0");
        core_loader.load_vk1_0().expect("Failed to load Vulkan 1.0");
        core_loader
    }

    ///
    /// Creates a vulkan instance and returns the instance handle
    ///
    fn create_instance<T>(
        core_loader: &erupt::CoreLoader<T>,
        app_info: &AppInfo,
        window_handle: &impl HasRawWindowHandle,
        debug: bool,
        validation: bool,
    ) -> erupt::vk1_0::Instance {
        // Fill out ApplicationInfo for creating a vulkan instance
        let app_name_cstr = CString::new(app_info.name.as_str()).unwrap();
        let app_version = erupt::make_version(app_info.major, app_info.minor, app_info.patch);
        let engine_version = erupt::make_version(
            engine_version_major(),
            engine_version_minor(),
            engine_version_patch(),
        );
        let api_version = erupt::make_version(1, 0, 0);
        let app_info = erupt::vk1_0::ApplicationInfoBuilder::new()
            .application_name(&app_name_cstr)
            .application_version(app_version)
            .engine_name(engine_name_cstr())
            .engine_version(engine_version)
            .api_version(api_version);

        let mut extensions = erupt::utils::surface::enumerate_required_extensions(window_handle)
            .expect("Failed to get required vulkan surface extensions");
        if debug {
            extensions.push(erupt::extensions::ext_debug_utils::EXT_DEBUG_UTILS_EXTENSION_NAME);
        }

        let mut layers = Vec::new();
        if validation {
            layers.push(erupt::cstr!("VK_LAYER_KHRONOS_validation"));
        }

        // Fill out InstanceCreateInfo for creating a vulkan instance
        let create_info = erupt::vk1_0::InstanceCreateInfoBuilder::new()
            .application_info(&app_info)
            .enabled_extension_names(&extensions)
            .enabled_layer_names(&layers);

        // Construct the vulkan instance
        aleph_log::trace!("Creating Vulkan instance");
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
        window_handle: &impl HasRawWindowHandle,
        debug: bool,
    ) -> erupt::InstanceLoader {
        // Load the vulkan instance function pointers
        aleph_log::trace!("Loading Vulkan Instance functions");
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

        unsafe { crate::surface::load_surface_functions(&mut instance_loader, window_handle) }

        instance_loader
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
                .create_debug_utils_messenger_ext(&create_info, None, None)
                .expect("Failed to install VK_EXT_debug_utils messenger")
        }
    }
}

///
/// A wrapper for representing a vulkan instance and it's dynamically loaded functions
///
pub struct Instance {
    _core_loader: Arc<erupt::utils::loading::DefaultCoreLoader>,
    instance_loader: Arc<InstanceLoader>,
    surface: SurfaceKHR,
    messenger: Option<DebugUtilsMessengerEXT>,
}

impl Instance {
    ///
    /// Get a builder for constructing an instance. Just a wrapper for `InstanceBuilder::new`
    ///
    pub fn builder() -> InstanceBuilder {
        InstanceBuilder::new()
    }

    ///
    /// Get a reference to the instance loader
    ///
    pub fn loader(&self) -> &Arc<InstanceLoader> {
        &self.instance_loader
    }

    ///
    /// Gets the SurfaceKHR we made when creating the instance
    ///
    pub fn surface(&self) -> SurfaceKHR {
        self.surface
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        unsafe {
            aleph_log::trace!("Destroying Vulkan surface");
            self.instance_loader.destroy_surface_khr(self.surface, None);
            if let Some(messenger) = self.messenger {
                aleph_log::trace!("Destroying debug messenger");
                self.instance_loader
                    .destroy_debug_utils_messenger_ext(messenger, None);
            }
            aleph_log::trace!("Destroying Vulkan instance");
            self.instance_loader.destroy_instance(None);
        }
    }
}
