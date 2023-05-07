use crate::context::Context;
use crate::internal::messenger::vulkan_debug_messenger;
use aleph_any::AnyArc;
use aleph_rhi_api::{BackendAPI, IContext};
use aleph_rhi_loader_api::{ContextCreateError, ContextOptions, IRhiBackend};
use anyhow::anyhow;
use erupt::vk;
use std::ffi::{c_char, CStr};
use std::mem::ManuallyDrop;
use std::sync::atomic::{AtomicBool, Ordering};

#[cfg(all(
    unix,
    not(any(target_os = "macos", target_os = "ios", target_os = "android"))
))]
const LIB_PATH: &str = "libvulkan.so.1";

#[cfg(target_os = "android")]
const LIB_PATH: &str = "libvulkan.so";

#[cfg(any(target_os = "macos", target_os = "ios"))]
const LIB_PATH: &str = "libvulkan.dylib";

#[cfg(windows)]
const LIB_PATH: &str = "vulkan-1.dll";

pub static RHI_BACKEND: &'static dyn IRhiBackend = &RHI_BACKEND_OBJECT;

static RHI_BACKEND_OBJECT: RhiBackend = RhiBackend {
    context_made: AtomicBool::new(false),
};

struct RhiBackend {
    /// Flags whether a context has already been created
    context_made: AtomicBool,
}

impl IRhiBackend for RhiBackend {
    fn backend(&self) -> BackendAPI {
        BackendAPI::Vulkan
    }

    fn is_available(&self) -> bool {
        // Safety: We assume that loading the vulkan dll does not have any unsafe side effects
        unsafe { libloading::Library::new(LIB_PATH).is_ok() }
    }

    fn make_context(
        &self,
        options: &ContextOptions,
    ) -> Result<AnyArc<dyn IContext>, ContextCreateError> {
        match self
            .context_made
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
        {
            Ok(_) => {
                let entry_loader = erupt::EntryLoader::new().map_err(|e| anyhow!(e))?;

                let instance_loader =
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
                    _this: v.clone(),
                    entry_loader: ManuallyDrop::new(entry_loader),
                    instance_loader: ManuallyDrop::new(instance_loader),
                    messenger,
                });
                Ok(AnyArc::map::<dyn IContext, _>(context, |v| v))
            }
            Err(_) => Err(ContextCreateError::ContextAlreadyCreated),
        }
    }
}

impl RhiBackend {
    fn create_instance<T>(
        entry_loader: &erupt::CustomEntryLoader<T>,
        debug: bool,
        validation: bool,
    ) -> Result<erupt::InstanceLoader, ContextCreateError> {
        // If validation is requested we must force debug on as we require debug extensions to log
        // the validation messages
        let debug = if validation { true } else { debug };

        let instance_version = entry_loader.instance_version();
        if vk::api_version_major(instance_version) < 1 {
            return Err(ContextCreateError::Platform(anyhow!(
                "Vulkan Instance doesn't support Vulkan 1.x"
            )));
        }
        if vk::api_version_minor(instance_version) < 2 {
            return Err(ContextCreateError::Platform(anyhow!(
                "Vulkan Instance doesn't support Vulkan 1.2"
            )));
        }

        // Select the set of extensions that we want to load
        let wanted_extensions = get_wanted_extensions(debug);
        let supported_extensions =
            strip_unsupported_extensions(entry_loader, wanted_extensions.clone());
        check_all_extensions_supported(&wanted_extensions, &supported_extensions)?;

        // Select the set of layers we want to load
        let wanted_layers = get_wanted_layers(validation);
        let supported_layers = strip_unsupported_layers(entry_loader, wanted_layers.clone());
        check_all_layers_supported(&wanted_layers, &supported_layers)?;

        // Fill out InstanceCreateInfo for creating a vulkan instance
        let flags = if cfg!(target_os = "macos") {
            // Add the VK_INSTANCE_CREATE_ENUMERATE_PORTABILITY_BIT_KHR flag manually as erupt
            // doesn't have it yet.
            unsafe { vk::InstanceCreateFlags::from_bits_unchecked(0b1) }
        } else {
            vk::InstanceCreateFlags::empty()
        };
        let app_info = app_and_engine_info();
        let create_info = erupt::vk1_0::InstanceCreateInfoBuilder::new()
            .application_info(&app_info)
            .enabled_extension_names(&supported_extensions)
            .enabled_layer_names(&supported_layers)
            .flags(flags);

        // Construct the vulkan instance
        log::trace!("Creating Vulkan instance");
        let instance_loader = unsafe {
            erupt::InstanceLoaderBuilder::new()
                .build(&entry_loader, &create_info)
                .map_err(|e| anyhow!(e))?
        };

        Ok(instance_loader)
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
    if cfg!(target_os = "macos") {
        // We need the molten vk surface extension as well as VK_KHR_portability_enumeration in
        // order for the loader to give us our mvk device.
        extensions.push(mvk_macos_surface::MVK_MACOS_SURFACE_EXTENSION_NAME);
        extensions.push("VK_KHR_portability_enumeration\0".as_ptr() as *const _);
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
    wanted_layers: &[*const c_char],
    supported_layers: &[*const c_char],
) -> Result<(), ContextCreateError> {
    let mut missing_extensions = diff_lists(wanted_layers, supported_layers).peekable();
    if missing_extensions.peek().is_some() {
        for missing in missing_extensions {
            log::error!("Runtime requested unsupported layer '{:#?}'.", missing);
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
    wanted_extensions: &[*const c_char],
    supported_extensions: &[*const c_char],
) -> Result<(), ContextCreateError> {
    let mut missing_layers = diff_lists(wanted_extensions, supported_extensions).peekable();
    if missing_layers.peek().is_some() {
        for missing in missing_layers {
            log::error!("Runtime requested unsupported extension '{:#?}'.", missing);
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

fn app_and_engine_info<'a>() -> vk::ApplicationInfoBuilder<'a> {
    vk::ApplicationInfoBuilder::new()
        .application_name(crate::cstr!("aleph-gpu"))
        .application_version(vk::make_api_version(
            0,
            aleph_rhi_api::API_VERSION_MAJOR.parse().unwrap(),
            aleph_rhi_api::API_VERSION_MINOR.parse().unwrap(),
            aleph_rhi_api::API_VERSION_PATCH.parse().unwrap(),
        ))
        .engine_name(crate::cstr!("aleph-gpu-vulkan"))
        .engine_version(vk::make_api_version(
            0,
            env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap(),
            env!("CARGO_PKG_VERSION_MINOR").parse().unwrap(),
            env!("CARGO_PKG_VERSION_PATCH").parse().unwrap(),
        ))
        .api_version(vk::API_VERSION_1_2)
}

fn install_debug_messenger(
    instance_loader: &erupt::InstanceLoader,
) -> Result<vk::DebugUtilsMessengerEXT, ContextCreateError> {
    let create_info = vk::DebugUtilsMessengerCreateInfoEXTBuilder::new()
        .message_severity(
            vk::DebugUtilsMessageSeverityFlagsEXT::ERROR_EXT
                | vk::DebugUtilsMessageSeverityFlagsEXT::INFO_EXT
                | vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE_EXT
                | vk::DebugUtilsMessageSeverityFlagsEXT::WARNING_EXT,
        )
        .message_type(
            vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION_EXT
                | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE_EXT,
        )
        .pfn_user_callback(Some(vulkan_debug_messenger));

    unsafe {
        instance_loader
            .create_debug_utils_messenger_ext(&create_info, None)
            .map_err(|e| ContextCreateError::Platform(anyhow!(e)))
    }
}
