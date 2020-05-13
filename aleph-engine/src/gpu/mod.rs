//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use crate::app::{AppInfo, ENGINE_VERSION_VK};
use erupt::vk1_0::Vk10CoreLoaderExt;
use raw_window_handle::HasRawWindowHandle;
use std::ffi::{CStr, CString};

///
/// Loads the core vulkan functions required for creating a VkInstance
///
pub fn load_vulkan_core() -> erupt::CoreLoader<libloading::Library> {
    // Load core vulkan functions
    log::trace!("Initializing Vulkan Core Loader");
    let mut core_loader = erupt::CoreLoader::new().expect("Failed to create Vulkan core loader");

    // Load vulkan 1.0 core functions
    log::trace!("Loading Core Functions for Vulkan 1.0");
    core_loader.load_vk1_0().expect("Failed to load Vulkan 1.0");
    core_loader
}

pub fn create_instance<T>(
    core_loader: &erupt::CoreLoader<T>,
    app_info: &AppInfo,
    window: &impl HasRawWindowHandle,
) -> erupt::vk1_0::Instance {
    // Fill out ApplicationInfo for creating a vulkan instance
    let app_name_cstr = CString::new(app_info.name.as_str()).unwrap();
    let app_version = erupt::make_version(app_info.major, app_info.minor, app_info.patch);
    let engine_name = unsafe { CStr::from_ptr(erupt::cstr!("AlephEngine")) };
    let api_version = erupt::make_version(1, 0, 0);
    let app_info = erupt::vk1_0::ApplicationInfoBuilder::new()
        .application_name(&app_name_cstr)
        .application_version(app_version)
        .engine_name(engine_name)
        .engine_version(ENGINE_VERSION_VK)
        .api_version(api_version);

    let mut extensions = erupt::utils::surface::enumerate_required_extensions(window)
        .expect("Failed to get required vulkan surface extensions");
    extensions.push(erupt::extensions::ext_debug_utils::EXT_DEBUG_UTILS_EXTENSION_NAME);

    let mut layers = Vec::new();
    layers.push(erupt::cstr!("VK_LAYER_LUNARG_standard_validation"));

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

pub fn load_vulkan_instance<T>(
    core_loader: &erupt::CoreLoader<T>,
    instance: erupt::vk1_0::Instance,
) -> erupt::InstanceLoader {
    // Load the vulkan instance function pointers
    log::info!("Loading Vulkan Instance functions");
    let mut instance_loader = erupt::InstanceLoader::new(core_loader, instance)
        .expect("Failed to initialize Vulkan instance loader");
    instance_loader
        .load_vk1_0()
        .expect("Failed to load vulkan functions");
    instance_loader
}
