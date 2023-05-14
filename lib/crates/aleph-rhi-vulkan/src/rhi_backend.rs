use crate::context::{Context, SurfaceLoaders};
use crate::internal::messenger::vulkan_debug_messenger;
use crate::internal::mvk;
use aleph_any::AnyArc;
use aleph_rhi_api::{BackendAPI, IContext};
use aleph_rhi_impl_utils::cstr;
use aleph_rhi_loader_api::{ContextCreateError, ContextOptions, IRhiBackend};
use anyhow::anyhow;
use ash::extensions::ext::DebugUtils;
use ash::extensions::khr::{
    AndroidSurface, Surface, WaylandSurface, Win32Surface, XcbSurface, XlibSurface,
};
use ash::extensions::mvk::{IOSSurface, MacOSSurface};
use ash::vk;
use std::ffi::CStr;
use std::mem::ManuallyDrop;
use std::sync::atomic::{AtomicBool, Ordering};

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
                let entry = unsafe { ash::Entry::load().map_err(|e| anyhow!(e))? };

                let (instance, extensions) =
                    Self::create_instance(&entry, options.debug, options.validation)?;

                let messenger = match (extensions.debug_loader.as_ref(), options.validation) {
                    (Some(loader), true) => match install_debug_messenger(loader) {
                        Ok(v) => Some(v),
                        Err(e) => {
                            log::warn!(
                                "Context validation requested but couldn't create debug messenger"
                            );
                            log::warn!("Reason: {:?}", e);
                            log::warn!("No validation messages will be logged");
                            None
                        }
                    },
                    (None, true) => {
                        log::warn!(
                            "Context validation requested but couldn't create debug messenger"
                        );
                        log::warn!("Reason: Failed to load VK_EXT_debug_utils");
                        log::warn!("No validation messages will be logged");
                        None
                    }
                    (_, false) => None,
                };

                let context = AnyArc::new_cyclic(move |v| Context {
                    _this: v.clone(),
                    entry_loader: ManuallyDrop::new(entry),
                    instance: ManuallyDrop::new(instance),
                    surface_loaders: extensions.surface_loaders(),
                    debug_loader: extensions.debug_loader,
                    messenger,
                });
                Ok(AnyArc::map::<dyn IContext, _>(context, |v| v))
            }
            Err(_) => Err(ContextCreateError::ContextAlreadyCreated),
        }
    }
}

impl RhiBackend {
    fn create_instance(
        entry: &ash::Entry,
        debug: bool,
        validation: bool,
    ) -> Result<(ash::Instance, Extensions), ContextCreateError> {
        // If validation is requested we must force debug on as we require debug extensions to log
        // the validation messages
        let debug = if validation { true } else { debug };

        // We need to configure MoltenVK before we do much of anything with Vulkan so we can
        // properly observe all the configuration stuff we change.
        let result = Self::configure_mvk(debug);
        if result.is_none() && cfg!(target_os = "macos") {
            log::warn!("Failed to configure MoltenVK on macOS");
        }

        let instance_version = entry
            .try_enumerate_instance_version()
            .map_err(|v| anyhow!(v))?;
        let instance_version = instance_version.unwrap();
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
        let supported_extensions = strip_unsupported_extensions(entry, wanted_extensions.clone());
        check_all_extensions_supported(&wanted_extensions, &supported_extensions)?;
        let extensions: Vec<_> = supported_extensions
            .iter()
            .copied()
            .map(|v| v.as_ptr())
            .collect();

        // Select the set of layers we want to load
        let wanted_layers = get_wanted_layers(validation);
        let supported_layers = strip_unsupported_layers(entry, wanted_layers.clone());
        check_all_layers_supported(&wanted_layers, &supported_layers)?;
        let layers: Vec<_> = supported_layers
            .iter()
            .copied()
            .map(|v| v.as_ptr())
            .collect();

        // Fill out InstanceCreateInfo for creating a vulkan instance
        let flags = if cfg!(target_os = "macos") {
            vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR
        } else {
            vk::InstanceCreateFlags::empty()
        };
        let app_info = app_and_engine_info();
        let create_info = vk::InstanceCreateInfo::builder()
            .application_info(&app_info)
            .enabled_extension_names(&extensions)
            .enabled_layer_names(&layers)
            .flags(flags);

        // Construct the vulkan instance
        log::trace!("Creating Vulkan instance");
        let instance_loader = unsafe {
            entry
                .create_instance(&create_info, None)
                .map_err(|e| anyhow!(e))?
        };

        let extensions =
            Self::load_instance_extensions(entry, &instance_loader, &supported_extensions);

        Ok((instance_loader, extensions))
    }

    fn load_instance_extensions(
        entry: &ash::Entry,
        instance: &ash::Instance,
        supported_extensions: &[&CStr],
    ) -> Extensions {
        let debug_loader = if supported_extensions.contains(&DebugUtils::name()) {
            Some(DebugUtils::new(entry, instance))
        } else {
            None
        };
        let surface_loader = if supported_extensions.contains(&Surface::name()) {
            Some(Surface::new(entry, instance))
        } else {
            None
        };
        let xlib_loader = if supported_extensions.contains(&XlibSurface::name()) {
            Some(XlibSurface::new(entry, instance))
        } else {
            None
        };
        let xcb_loader = if supported_extensions.contains(&XcbSurface::name()) {
            Some(XcbSurface::new(entry, instance))
        } else {
            None
        };
        let wayland_loader = if supported_extensions.contains(&WaylandSurface::name()) {
            Some(WaylandSurface::new(entry, instance))
        } else {
            None
        };
        let android_loader = if supported_extensions.contains(&AndroidSurface::name()) {
            Some(AndroidSurface::new(entry, instance))
        } else {
            None
        };
        let win32_loader = if supported_extensions.contains(&Win32Surface::name()) {
            Some(Win32Surface::new(entry, instance))
        } else {
            None
        };
        let macos_loader = if supported_extensions.contains(&MacOSSurface::name()) {
            Some(MacOSSurface::new(entry, instance))
        } else {
            None
        };
        let ios_loader = if supported_extensions.contains(&IOSSurface::name()) {
            Some(IOSSurface::new(entry, instance))
        } else {
            None
        };

        Extensions {
            debug_loader,
            surface_loader,
            xlib_loader,
            xcb_loader,
            wayland_loader,
            android_loader,
            win32_loader,
            macos_loader,
            ios_loader,
        }
    }

    fn configure_mvk(debug: bool) -> Option<()> {
        unsafe {
            let library = libloading::Library::new("libMoltenVK.dylib").ok()?;

            let name = "vkGetMoltenVKConfigurationMVK\0".as_bytes();
            let get_fn = library
                .get::<mvk::PFN_vkGetMoltenVKConfigurationMVK>(name)
                .ok()?;

            let name = "vkSetMoltenVKConfigurationMVK\0".as_bytes();
            let set_fn = library
                .get::<mvk::PFN_vkSetMoltenVKConfigurationMVK>(name)
                .ok()?;

            let mut config = mvk::Configuration::default();
            let mut size = std::mem::size_of_val(&config);

            let result = get_fn(vk::Instance::null(), &mut config, &mut size);
            if result.as_raw() < 0 {
                log::warn!("'vkGetMoltenVKConfigurationMVK' failed with error '{result}'");
                return None;
            }
            if size < std::mem::size_of_val(&config) {
                log::warn!(
                    "Size from 'vkGetMoltenVKConfigurationMVK' too small, can't give mvk required \
                    configuration settings"
                );
            }

            if debug {
                config.log_level = mvk::ConfigLogLevel::DEBUG;
            } else {
                config.log_level = mvk::ConfigLogLevel::NONE;
            }
            config.use_metal_argument_buffers = mvk::UseMetalArgumentBuffers::DESCRIPTOR_INDEXING;

            let mut size = std::mem::size_of_val(&config);
            let result = set_fn(vk::Instance::null(), &config, &mut size);
            if result.as_raw() < 0 {
                log::warn!("'vkSetMoltenVKConfigurationMVK' failed with error '{result}'");
                return None;
            };
        }
        Some(())
    }
}

fn get_wanted_extensions(debug: bool) -> Vec<&'static CStr> {
    // Get surface extensions
    // Push the base surface extension
    let mut extensions = vec![vk::KhrSurfaceFn::name()];

    // Push all possible WSI extensions for the underlying platform
    if cfg!(all(
        unix,
        not(target_os = "android"),
        not(target_os = "macos")
    )) {
        // This is the branch for linux. Linux has a bunch of WSI extensions so add them all,
        // any unsupported extensions will be stripped later.
        extensions.push(vk::KhrXlibSurfaceFn::name());
        extensions.push(vk::KhrXcbSurfaceFn::name());
        extensions.push(vk::KhrWaylandSurfaceFn::name());
    }
    if cfg!(target_os = "android") {
        // Android, only one. A sane platform
        extensions.push(vk::KhrAndroidSurfaceFn::name());
    }
    if cfg!(target_os = "windows") {
        // Windows, again a single WSI extension.
        extensions.push(vk::KhrWin32SurfaceFn::name());
    }
    if cfg!(target_os = "macos") {
        // We need the molten vk surface extension as well as VK_KHR_portability_enumeration in
        // order for the loader to give us our mvk device.
        extensions.push(vk::MvkMacosSurfaceFn::name());
        extensions.push(vk::KhrPortabilityEnumerationFn::name());
    }
    if cfg!(target_os = "ios") {
        // We need the molten vk surface extension as well as VK_KHR_portability_enumeration in
        // order for the loader to give us our mvk device.
        extensions.push(vk::MvkIosSurfaceFn::name());
        extensions.push(vk::KhrPortabilityEnumerationFn::name());
    }

    // Add the debug extension if requested
    if debug {
        extensions.push(vk::ExtDebugUtilsFn::name());
    }

    extensions
}

fn strip_unsupported_extensions<'a>(
    entry: &ash::Entry,
    mut extensions: Vec<&'a CStr>,
) -> Vec<&'a CStr> {
    let supported_instance_extensions: Vec<vk::ExtensionProperties> = entry
        .enumerate_instance_extension_properties(None)
        .unwrap_or_default();

    // Strip all unsupported extensions
    extensions.retain(|&v| {
        // SAFETY: Everything is guaranteed to be a C string here
        unsafe {
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
    wanted_layers: &[&CStr],
    supported_layers: &[&CStr],
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

fn get_wanted_layers(validation: bool) -> Vec<&'static CStr> {
    let mut layers = Vec::new();
    if validation {
        layers.push(cstr!("VK_LAYER_KHRONOS_validation"));
    }
    layers
}

fn strip_unsupported_layers<'a>(entry: &ash::Entry, mut layers: Vec<&'a CStr>) -> Vec<&'a CStr> {
    let supported_instance_layers = entry
        .enumerate_instance_layer_properties()
        .unwrap_or_default();

    // Strip all unsupported layers
    layers.retain(|&v| {
        // SAFETY: Everything is guaranteed to be a C string here
        unsafe {
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
    wanted_extensions: &[&CStr],
    supported_extensions: &[&CStr],
) -> Result<(), ContextCreateError> {
    let mut missing_layers = diff_lists(wanted_extensions, supported_extensions).peekable();
    if missing_layers.peek().is_some() {
        for missing in missing_layers {
            log::error!("Runtime requested unsupported extension '{:#?}'.", missing);
        }
        return Err(ContextCreateError::Platform(anyhow!(
            "Unsupported extension is required by runtime"
        )));
    }
    Ok(())
}

fn diff_lists<'a>(
    list_a: &'a [&'a CStr],
    list_b: &'a [&'a CStr],
) -> impl Iterator<Item = &'a CStr> {
    list_a.iter().copied().filter(|&a| {
        let in_both = list_b.iter().any(|&b| a == b);
        !in_both
    })
}

fn app_and_engine_info<'a>() -> vk::ApplicationInfoBuilder<'a> {
    vk::ApplicationInfo::builder()
        .application_name(cstr!("aleph-gpu"))
        .application_version(vk::make_api_version(
            0,
            aleph_rhi_api::API_VERSION_MAJOR.parse().unwrap(),
            aleph_rhi_api::API_VERSION_MINOR.parse().unwrap(),
            aleph_rhi_api::API_VERSION_PATCH.parse().unwrap(),
        ))
        .engine_name(cstr!("aleph-gpu-vulkan"))
        .engine_version(vk::make_api_version(
            0,
            env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap(),
            env!("CARGO_PKG_VERSION_MINOR").parse().unwrap(),
            env!("CARGO_PKG_VERSION_PATCH").parse().unwrap(),
        ))
        .api_version(vk::API_VERSION_1_2)
}

fn install_debug_messenger(
    loader: &ash::extensions::ext::DebugUtils,
) -> Result<vk::DebugUtilsMessengerEXT, ContextCreateError> {
    let create_info = vk::DebugUtilsMessengerCreateInfoEXT::builder()
        .message_severity(
            vk::DebugUtilsMessageSeverityFlagsEXT::ERROR
                | vk::DebugUtilsMessageSeverityFlagsEXT::INFO
                | vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE
                | vk::DebugUtilsMessageSeverityFlagsEXT::WARNING,
        )
        .message_type(
            vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
                | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
        )
        .pfn_user_callback(Some(vulkan_debug_messenger));

    unsafe {
        loader
            .create_debug_utils_messenger(&create_info, None)
            .map_err(|e| ContextCreateError::Platform(anyhow!(e)))
    }
}

struct Extensions {
    debug_loader: Option<DebugUtils>,
    surface_loader: Option<Surface>,
    xlib_loader: Option<XlibSurface>,
    xcb_loader: Option<XcbSurface>,
    wayland_loader: Option<WaylandSurface>,
    android_loader: Option<AndroidSurface>,
    win32_loader: Option<Win32Surface>,
    macos_loader: Option<MacOSSurface>,
    ios_loader: Option<IOSSurface>,
}

impl Extensions {
    pub fn surface_loaders(&self) -> SurfaceLoaders {
        SurfaceLoaders {
            base: self.surface_loader.clone(),
            win32: self.win32_loader.clone(),
            xlib: self.xlib_loader.clone(),
            xcb: self.xcb_loader.clone(),
            wayland: self.wayland_loader.clone(),
            android: self.android_loader.clone(),
            macos: self.macos_loader.clone(),
            ios: self.ios_loader.clone(),
        }
    }
}
