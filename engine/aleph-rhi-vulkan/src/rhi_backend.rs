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

use std::collections::BTreeSet;
use std::ffi::CStr;
use std::iter;
use std::mem::ManuallyDrop;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use aleph_alloc::instrumentation::IAllocationCategory;
use aleph_rhi_api::{ContextCreateError, ContextOptions, IContext};
use aleph_rhi_impl_utils::{Rhi, abort_on_unwind};
use ash::vk;

use crate::context::{Context, SurfaceLoaders};
use crate::internal::allocation_callbacks::GLOBAL;
use crate::internal::messenger::vulkan_debug_messenger;
use crate::internal::{loader, mvk};

pub static RHI_BACKEND_OBJECT: VulkanLoader = VulkanLoader {
    context_made: AtomicBool::new(false),
};

#[derive(Clone, Hash, PartialEq, Eq, Debug, Default)]
pub struct VulkanConfig {}

pub struct VulkanLoader {
    /// Flags whether a context has already been created
    context_made: AtomicBool,
}

impl VulkanLoader {
    pub fn is_available(&self) -> bool {
        if cfg!(target_os = "ios") {
            // On iOS we statically link to the loader and vulkan is always available via MoltenVK
            true
        } else {
            // Safety: We assume that loading the vulkan dll does not have any unsafe side effects
            loader::VULKAN_LIBRARY.is_some()
        }
    }

    pub fn make_context(
        &self,
        options: &ContextOptions,
        config: &VulkanConfig,
    ) -> Result<Arc<dyn IContext>, ContextCreateError> {
        match self
            .context_made
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
        {
            Ok(_) => abort_on_unwind(|| {
                let entry = unsafe {
                    match loader::load() {
                        None => {
                            log::error!("Failed to load Vulkan library");
                            return Err(ContextCreateError::Platform);
                        }
                        Some(v) => v,
                    }
                };

                let (instance, extensions) = Rhi::with(|| Self::create_instance(&entry, options))?;

                if extensions.debug_loader.is_none() {
                    if options.validation {
                        log::warn!(
                            "ContextOptions::validation = true, but failed to load VK_EXT_debug_utils"
                        );
                    }
                    if options.debug {
                        log::warn!(
                            "ContextOptions::debug = true, but failed to load VK_EXT_debug_utils"
                        );
                    }
                }

                let messenger = if let Some(loader) = extensions.debug_loader.as_ref() {
                    install_debug_messenger(loader)
                        .inspect_err(|e| {
                            log::warn!("Failed to install debug messenger: {e:?}",);
                        })
                        .ok()
                } else {
                    None
                };

                let context: Arc<dyn IContext> = Rhi::with(|| {
                    Arc::new_cyclic(move |v| Context {
                        _this: v.clone(),
                        _options: options.clone(),
                        _config: config.clone(),
                        entry_loader: ManuallyDrop::new(entry),
                        instance: ManuallyDrop::new(instance),
                        surface_loaders: extensions.surface_loaders(),
                        debug_loader: extensions.debug_loader,
                        messenger,
                    })
                });
                Ok(context)
            }),
            Err(_) => Err(ContextCreateError::ContextAlreadyCreated),
        }
    }
}

impl VulkanLoader {
    fn create_instance(
        entry: &ash::Entry,
        options: &ContextOptions,
    ) -> Result<(ash::Instance, Extensions), ContextCreateError> {
        // We need to configure MoltenVK before we do much of anything with Vulkan so we can
        // properly observe all the configuration stuff we change.
        let result = Self::configure_mvk(options);
        if result.is_none() && cfg!(any(target_os = "macos", target_os = "ios")) {
            log::warn!("Failed to configure MoltenVK on macOS");
        }

        // We require a Vulkan 1.1+ instance. This ensures the 'api_version' field in
        // ApplicationInfo has the desired behaviour. Vulkan 1.0 instances would fail if you asked
        // for an unsupported version in ApplicationInfo. However you couldn't really find out the
        // supported version until you actually created the instance.
        //
        // 1.1 fixed this, and we require it. We may use a higher version when creating the device
        // later, but the minimum we need to find out what's available is 1.1.
        let instance_version = unsafe {
            entry
                .try_enumerate_instance_version()
                .inspect_err(|v| log::error!("Platform Error: {:#?}", v))
                .map_err(|_| ContextCreateError::Platform)?
        };
        let instance_version = instance_version.ok_or_else(|| {
            log::error!("Vulkan Instance doesn't support Vulkan > 1.0");
            ContextCreateError::Platform
        })?;
        if vk::api_version_major(instance_version) < 1 {
            log::error!("Vulkan Instance doesn't support Vulkan 1.x");
            return Err(ContextCreateError::Platform);
        }
        if vk::api_version_minor(instance_version) < 1 {
            log::error!("Vulkan Instance doesn't support Vulkan 1.1");
            return Err(ContextCreateError::Platform);
        }

        // Select the set of layers we want to load
        let wanted_layers = get_wanted_layers(options.validation);
        let supported_layers = strip_unsupported_layers(entry, wanted_layers.clone());
        check_all_layers_supported(&wanted_layers, &supported_layers)?;
        let layers: Vec<_> = supported_layers
            .iter()
            .copied()
            .map(|v| v.as_ptr())
            .collect();

        // Get the set of all extensions available, including from the layers we are going to use.
        let ext_sets = get_supported_extension_sets(entry, &supported_layers);

        // Get the set of extensions we want to try and load
        let required_extensions = {
            let extensions = get_required_extensions(options);
            let supported = strip_unsupported_extensions(&ext_sets, extensions.clone());
            check_all_extensions_supported(&extensions, &supported)?;
            extensions
        };
        let optional_extensions = {
            let extensions = get_optional_extensions(options);
            strip_unsupported_extensions(&ext_sets, extensions)
        };
        let wsi_extensions = {
            let extensions = get_wsi_extensions();
            let stripped = strip_unsupported_extensions(&ext_sets, extensions);
            if stripped.is_empty() {
                // If no WSI extensions are available we're in a sad place
                log::error!("The Vulkan instance does not support any WSI extensions we know of");
                return Err(ContextCreateError::MissingRequiredFeatures);
            }
            stripped
        };
        let extensions_to_load = {
            let required_iter = required_extensions.iter().copied();
            let wsi_iter = optional_extensions.iter().copied();
            let optional_iter = wsi_extensions.iter().copied();
            let deduplicated_extensions =
                BTreeSet::from_iter(required_iter.chain(wsi_iter).chain(optional_iter));
            Vec::from_iter(deduplicated_extensions.into_iter())
        };
        let extensions_to_load_abi: Vec<_> = extensions_to_load
            .iter()
            .copied()
            .map(CStr::as_ptr)
            .collect();

        // Fill out InstanceCreateInfo for creating a vulkan instance
        let flags = if cfg!(any(target_os = "macos", target_os = "ios")) {
            vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR
        } else {
            vk::InstanceCreateFlags::empty()
        };
        let app_info = app_and_engine_info();
        let create_info = vk::InstanceCreateInfo::default()
            .application_info(&app_info)
            .enabled_extension_names(&extensions_to_load_abi)
            .enabled_layer_names(&layers)
            .flags(flags);

        log::info!("Creating Vulkan instance");
        let instance_loader = unsafe {
            let configurable_validation = options.validation
                && get_supported_in_set(&ext_sets, ash::ext::layer_settings::NAME);
            if !configurable_validation {
                log::warn!(
                    "VK_EXT_layer_settings is not available! Unable to config validation layers."
                );
                entry
                    .create_instance(&create_info, GLOBAL)
                    .inspect_err(|e| log::error!("Platform Error: {:#?}", e))
                    .map_err(|_| ContextCreateError::Platform)?
            } else {
                let settings = vec![
                    vk::LayerSettingEXT::default()
                        .layer_name(VALIDATION_LAYER_NAME)
                        .setting_name(c"validate_sync")
                        .values_bool(&VTRUE),
                ];

                let mut layer_settings =
                    vk::LayerSettingsCreateInfoEXT::default().settings(&settings);

                let create_info = create_info.push_next(&mut layer_settings);
                entry
                    .create_instance(&create_info, GLOBAL)
                    .inspect_err(|e| log::error!("Platform Error: {:#?}", e))
                    .map_err(|_| ContextCreateError::Platform)?
            }
        };

        let extensions =
            Self::load_instance_extensions(entry, &instance_loader, &extensions_to_load);

        Ok((instance_loader, extensions))
    }

    fn load_instance_extensions(
        entry: &ash::Entry,
        instance: &ash::Instance,
        supported_extensions: &[&CStr],
    ) -> Extensions {
        let debug_loader = if supported_extensions.contains(&ash::ext::debug_utils::NAME) {
            Some(ash::ext::debug_utils::Instance::new(entry, instance))
        } else {
            None
        };
        let surface_loader = if supported_extensions.contains(&ash::khr::surface::NAME) {
            Some(ash::khr::surface::Instance::new(entry, instance))
        } else {
            None
        };
        let xlib_loader = if supported_extensions.contains(&ash::khr::xlib_surface::NAME) {
            Some(ash::khr::xlib_surface::Instance::new(entry, instance))
        } else {
            None
        };
        let xcb_loader = if supported_extensions.contains(&ash::khr::xcb_surface::NAME) {
            Some(ash::khr::xcb_surface::Instance::new(entry, instance))
        } else {
            None
        };
        let wayland_loader = if supported_extensions.contains(&ash::khr::wayland_surface::NAME) {
            Some(ash::khr::wayland_surface::Instance::new(entry, instance))
        } else {
            None
        };
        let win32_loader = if supported_extensions.contains(&ash::khr::win32_surface::NAME) {
            Some(ash::khr::win32_surface::Instance::new(entry, instance))
        } else {
            None
        };
        let metal_surface_loader = if supported_extensions.contains(&ash::ext::metal_surface::NAME)
        {
            Some(ash::ext::metal_surface::Instance::new(entry, instance))
        } else {
            None
        };
        let macos_loader = if supported_extensions.contains(&ash::mvk::macos_surface::NAME) {
            Some(ash::mvk::macos_surface::Instance::new(entry, instance))
        } else {
            None
        };
        let ios_loader = if supported_extensions.contains(&ash::mvk::ios_surface::NAME) {
            Some(ash::mvk::ios_surface::Instance::new(entry, instance))
        } else {
            None
        };

        Extensions {
            debug_loader,
            surface_loader,
            xlib_loader,
            xcb_loader,
            wayland_loader,
            win32_loader,
            metal_surface_loader,
            macos_loader,
            ios_loader,
        }
    }

    fn configure_mvk(options: &ContextOptions) -> Option<()> {
        Self::load_mvk_fns(move |get_fn, set_fn| unsafe {
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

            if options.validation {
                config.log_level = mvk::ConfigLogLevel::DEBUG;
            } else {
                config.log_level = mvk::ConfigLogLevel::NONE;
            }
            config.use_metal_argument_buffers = mvk::UseMetalArgumentBuffers::DESCRIPTOR_INDEXING;
            config.specialized_queue_families = 0;
            config.synchronous_queue_submits = 1;

            // Safety: If the cstr inside isn't valid even though we just got it from the runtime
            //         we're hosed so assume it's good.
            log::info!("== MOLTEN-VK SETTINGS ==");
            config.log();

            let mut size = std::mem::size_of_val(&config);
            let result = set_fn(vk::Instance::null(), &config, &mut size);
            if result.as_raw() < 0 {
                log::warn!("'vkSetMoltenVKConfigurationMVK' failed with error '{result}'");
                return None;
            };
            Some(())
        })
    }

    fn load_mvk_fns(
        cont: impl FnOnce(
            mvk::PFN_vkGetMoltenVKConfigurationMVK,
            mvk::PFN_vkSetMoltenVKConfigurationMVK,
        ) -> Option<()>,
    ) -> Option<()> {
        #[cfg(target_os = "ios")]
        {
            let get_fn = mvk::vkGetMoltenVKConfigurationMVK;
            let set_fn = mvk::vkSetMoltenVKConfigurationMVK;
            cont(get_fn, set_fn)
        }

        #[cfg(not(target_os = "ios"))]
        {
            unsafe {
                let library = loader::MVK_LIBRARY.as_ref();
                let library = library?;

                let get_fn = sdl3_sys::loadso::SDL_LoadFunction(
                    library.0.as_ptr(),
                    c"vkGetMoltenVKConfigurationMVK".as_ptr(),
                )?;
                let get_fn =
                    std::mem::transmute::<_, mvk::PFN_vkGetMoltenVKConfigurationMVK>(get_fn);

                let set_fn = sdl3_sys::loadso::SDL_LoadFunction(
                    library.0.as_ptr(),
                    c"vkSetMoltenVKConfigurationMVK".as_ptr(),
                )?;
                let set_fn =
                    std::mem::transmute::<_, mvk::PFN_vkSetMoltenVKConfigurationMVK>(set_fn);

                cont(get_fn, set_fn)
            }
        }
    }
}

fn get_required_extensions(options: &ContextOptions) -> Vec<&'static CStr> {
    // Get surface extensions
    // Push the base surface extension
    let mut extensions = vec![ash::khr::surface::NAME];

    // The debug extension is _required_ if validation is requested, but optional otherwise
    if options.validation {
        extensions.push(ash::ext::debug_utils::NAME);
    }

    extensions
}

fn get_optional_extensions(options: &ContextOptions) -> Vec<&'static CStr> {
    let mut extensions = vec![];

    // Only add this if the validation option hasn't added it to the 'required' set.
    if options.debug && !options.validation {
        extensions.push(ash::ext::debug_utils::NAME);
    }

    extensions
}

fn get_wsi_extensions() -> Vec<&'static CStr> {
    // Get surface extensions
    // Push the base surface extension
    let mut extensions = vec![];

    // Push all possible WSI extensions for the underlying platform
    if cfg!(all(unix, not(target_os = "macos"), not(target_os = "ios"))) {
        // This is the branch for linux. Linux has a bunch of WSI extensions so add them all,
        // any unsupported extensions will be stripped later.
        extensions.push(ash::khr::xlib_surface::NAME);
        extensions.push(ash::khr::xcb_surface::NAME);
        extensions.push(ash::khr::wayland_surface::NAME);
    }
    if cfg!(target_os = "windows") {
        // Windows, again a single WSI extension.
        extensions.push(ash::khr::win32_surface::NAME);
    }
    if cfg!(target_os = "macos") {
        // We need the molten vk surface extension as well as VK_KHR_portability_enumeration in
        // order for the loader to give us our mvk device.
        extensions.push(ash::mvk::macos_surface::NAME);
        extensions.push(ash::ext::metal_surface::NAME);
        extensions.push(ash::khr::portability_enumeration::NAME);
    }
    if cfg!(target_os = "ios") {
        // We need the molten vk surface extension as well as VK_KHR_portability_enumeration in
        // order for the loader to give us our mvk device.
        extensions.push(ash::mvk::ios_surface::NAME);
        extensions.push(ash::ext::metal_surface::NAME);
        extensions.push(ash::khr::portability_enumeration::NAME);
    }

    extensions
}

fn get_supported_extension_sets(
    entry: &ash::Entry,
    layers: &[&CStr],
) -> Vec<Vec<vk::ExtensionProperties>> {
    // We can source extensions from the loader/driver directly, *or* from layers. We have to ask
    // for extensions from the driver and extensions from the layer in separate calls as separate
    // 'sets' of supported extensions. This is what we do here. 'layers' contains a list of layers
    // that *will* be loaded, so we can include those layers each as a source of extensions.
    //
    // This is primarily needed on platforms where some extensions are provided by layers like
    // android where VK_EXT_debug_utils is provided by the validation layers. Other examples would
    // be feature emulation layers.
    let extension_sources = iter::once(None).chain(layers.iter().map(|v| Some(*v)));
    extension_sources
        .map(|v| unsafe {
            entry
                .enumerate_instance_extension_properties(v)
                .unwrap_or_default()
        })
        .collect()
}

fn get_supported_in_set(sets: &[Vec<vk::ExtensionProperties>], ext: &CStr) -> bool {
    // SAFETY: Everything is guaranteed to be a C string here
    sets.iter().any(|set| {
        set.iter()
            .map(|s| unsafe { CStr::from_ptr(s.extension_name.as_ptr()) })
            .any(|s| ext == s)
    })
}

fn strip_unsupported_extensions<'a>(
    sets: &[Vec<vk::ExtensionProperties>],
    mut extensions: Vec<&'a CStr>,
) -> Vec<&'a CStr> {
    // Strip all unsupported extensions
    extensions.retain(|&v| get_supported_in_set(sets, v));
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
        log::error!("Unsupported layer is required by runtime");
        return Err(ContextCreateError::Platform);
    }
    Ok(())
}

fn get_wanted_layers(validation: bool) -> Vec<&'static CStr> {
    let mut layers = Vec::new();
    if validation {
        layers.push(VALIDATION_LAYER_NAME);
    }
    layers
}

fn strip_unsupported_layers<'a>(entry: &ash::Entry, mut layers: Vec<&'a CStr>) -> Vec<&'a CStr> {
    let supported_instance_layers = unsafe {
        entry
            .enumerate_instance_layer_properties()
            .unwrap_or_default()
    };

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
        log::error!("Unsupported extension is required by runtime");
        return Err(ContextCreateError::Platform);
    }
    Ok(())
}

fn diff_lists<'a>(
    list_a: &'a [&'a CStr],
    list_b: &'a [&'a CStr],
) -> impl Iterator<Item = &'a CStr> {
    list_a.iter().copied().filter(|&a| {
        let in_both = list_b.contains(&a);
        !in_both
    })
}

fn app_and_engine_info<'a>() -> vk::ApplicationInfo<'a> {
    // Unconditionally ask for api_version 1.3 as that's the version we require for our _device_.
    vk::ApplicationInfo::default()
        .application_name(c"aleph-gpu")
        .application_version(vk::make_api_version(
            0,
            aleph_rhi_api::API_VERSION_MAJOR.parse().unwrap(),
            aleph_rhi_api::API_VERSION_MINOR.parse().unwrap(),
            aleph_rhi_api::API_VERSION_PATCH.parse().unwrap(),
        ))
        .engine_name(c"aleph-gpu-vulkan")
        .engine_version(vk::make_api_version(
            0,
            env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap(),
            env!("CARGO_PKG_VERSION_MINOR").parse().unwrap(),
            env!("CARGO_PKG_VERSION_PATCH").parse().unwrap(),
        ))
        .api_version(vk::API_VERSION_1_3)
}

fn install_debug_messenger(
    loader: &ash::ext::debug_utils::Instance,
) -> Result<vk::DebugUtilsMessengerEXT, ContextCreateError> {
    let create_info = vk::DebugUtilsMessengerCreateInfoEXT::default()
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

    let messenger = unsafe {
        loader
            .create_debug_utils_messenger(&create_info, GLOBAL)
            .inspect_err(|e| log::error!("Platform Error: {:#?}", e))
            .map_err(|_| ContextCreateError::Platform)?
    };
    Ok(messenger)
}

struct Extensions {
    debug_loader: Option<ash::ext::debug_utils::Instance>,
    surface_loader: Option<ash::khr::surface::Instance>,
    xlib_loader: Option<ash::khr::xlib_surface::Instance>,
    xcb_loader: Option<ash::khr::xcb_surface::Instance>,
    wayland_loader: Option<ash::khr::wayland_surface::Instance>,
    win32_loader: Option<ash::khr::win32_surface::Instance>,
    metal_surface_loader: Option<ash::ext::metal_surface::Instance>,
    macos_loader: Option<ash::mvk::macos_surface::Instance>,
    ios_loader: Option<ash::mvk::ios_surface::Instance>,
}

impl Extensions {
    pub fn surface_loaders(&self) -> SurfaceLoaders {
        SurfaceLoaders {
            base: self.surface_loader.clone(),
            win32: self.win32_loader.clone(),
            xlib: self.xlib_loader.clone(),
            xcb: self.xcb_loader.clone(),
            wayland: self.wayland_loader.clone(),
            metal: self.metal_surface_loader.clone(),
            macos: self.macos_loader.clone(),
            ios: self.ios_loader.clone(),
        }
    }
}

const VALIDATION_LAYER_NAME: &CStr = c"VK_LAYER_KHRONOS_validation";

#[allow(unused)]
static VTRUE: [vk::Bool32; 1] = [vk::TRUE];

#[allow(unused)]
static VFALSE: [vk::Bool32; 1] = [0];

#[allow(unused)]
trait LayerSettingsExt<'a> {
    fn values_bool(self, values: &'a [vk::Bool32]) -> Self;
    fn values_i32(self, values: &'a [i32]) -> Self;
    fn values_i64(self, values: &'a [i64]) -> Self;
    fn values_u32(self, values: &'a [u32]) -> Self;
    fn values_u64(self, values: &'a [u64]) -> Self;
    fn values_f32(self, values: &'a [f32]) -> Self;
    fn values_f64(self, values: &'a [f64]) -> Self;
    fn values_cstr(self, values: &'a [&'a CStr]) -> Self;
}

impl<'a> LayerSettingsExt<'a> for vk::LayerSettingEXT<'a> {
    fn values_bool(self, values: &'a [vk::Bool32]) -> Self {
        layer_settings_generic_values(self, vk::LayerSettingTypeEXT::BOOL32, values)
    }

    fn values_i32(self, values: &'a [i32]) -> Self {
        layer_settings_generic_values(self, vk::LayerSettingTypeEXT::INT32, values)
    }

    fn values_i64(self, values: &'a [i64]) -> Self {
        layer_settings_generic_values(self, vk::LayerSettingTypeEXT::INT64, values)
    }

    fn values_u32(self, values: &'a [u32]) -> Self {
        layer_settings_generic_values(self, vk::LayerSettingTypeEXT::UINT32, values)
    }

    fn values_u64(self, values: &'a [u64]) -> Self {
        layer_settings_generic_values(self, vk::LayerSettingTypeEXT::UINT64, values)
    }

    fn values_f32(self, values: &'a [f32]) -> Self {
        layer_settings_generic_values(self, vk::LayerSettingTypeEXT::FLOAT32, values)
    }

    fn values_f64(self, values: &'a [f64]) -> Self {
        layer_settings_generic_values(self, vk::LayerSettingTypeEXT::FLOAT64, values)
    }

    fn values_cstr(self, values: &'a [&'a CStr]) -> Self {
        layer_settings_generic_values(self, vk::LayerSettingTypeEXT::STRING, values)
    }
}

fn layer_settings_generic_values<'a, 'b: 'a, T: Sized>(
    mut settings: vk::LayerSettingEXT<'b>,
    ty: vk::LayerSettingTypeEXT,
    values: &'a [T],
) -> vk::LayerSettingEXT<'a> {
    settings.value_count = values.len() as _;
    settings.p_values = values.as_ptr().cast();
    settings.ty(ty)
}
