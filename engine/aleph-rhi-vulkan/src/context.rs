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

use std::any::TypeId;
use std::ffi::{CStr, c_void};
use std::mem::ManuallyDrop;
use std::ptr::NonNull;

use aleph_any::{AnyArc, AnyWeak, declare_interfaces};
use aleph_rhi_api::*;
use aleph_rhi_impl_utils::arc::new_rhi_object;
use aleph_rhi_impl_utils::conv::pci_id_to_vendor;
use aleph_rhi_impl_utils::str_from_ptr;
use ash::vk;
use raw_window_handle::{HasDisplayHandle, HasWindowHandle, RawDisplayHandle, RawWindowHandle};

use crate::VulkanConfig;
use crate::adapter::Adapter;
use crate::internal::allocation_callbacks::GLOBAL;
use crate::internal::device_info::DeviceInfo;
use crate::internal::loader::LibraryType;
use crate::internal::unwrap;
use crate::surface::Surface;

pub struct Context {
    pub _this: AnyWeak<Self>,
    pub config: VulkanConfig,
    pub library: ManuallyDrop<LibraryType>,
    pub entry_loader: ManuallyDrop<ash::Entry>,
    pub instance: ManuallyDrop<ash::Instance>,
    pub surface_loaders: SurfaceLoaders,
    pub debug_loader: Option<ash::ext::debug_utils::Instance>,
    pub messenger: Option<vk::DebugUtilsMessengerEXT>,
}

declare_interfaces!(Context, [IContext]);

impl IGetPlatformInterface for Context {
    unsafe fn __query_platform_interface(&self, _target: TypeId, _out: *mut ()) -> Option<()> {
        // TODO: expose the instance loader via an arc or something
        None
    }
}

impl Context {
    pub fn select_device(
        entry: &ash::Entry,
        instance: &ash::Instance,
        surface: Option<vk::SurfaceKHR>,
        options: &AdapterRequestOptions,
    ) -> Option<(String, AdapterVendor, vk::PhysicalDevice)> {
        let devices = unsafe {
            instance
                .enumerate_physical_devices()
                .expect("Failed to enumerate vulkan devices")
        };
        let mut scores: Vec<(String, AdapterVendor, vk::PhysicalDevice, i32)> = Vec::new();

        for physical_device in devices.iter().copied() {
            let device_info = DeviceInfo::load(instance, physical_device);

            Self::log_device_info(&device_info);

            if vk::api_version_major(device_info.properties_10.api_version) < 1 {
                log::debug!("Device does not support Vulkan 1.x");
                continue;
            }

            if vk::api_version_minor(device_info.properties_10.api_version) < 1 {
                log::debug!("Device does not support Vulkan 1.1");
            }

            // Check if the physical device supports the requested surface
            if Self::check_surface_support(entry, instance, &device_info, physical_device, surface)
                .is_none()
            {
                log::debug!("Device does not support the target surface");
                continue;
            }

            if Self::check_device_supports_minimum_features(&device_info).is_none() {
                log::debug!("Device rejected as doesn't support minimum feature requirements");
                continue;
            }

            // Score the physical device based on the device preferences provided by the user
            let score = Self::score_device(&device_info, options);
            let score = match score {
                None => continue,
                Some(v) => v,
            };

            let vendor = pci_id_to_vendor(device_info.properties_10.vendor_id);
            let name = unsafe {
                (*str_from_ptr(device_info.properties_10.device_name.as_ptr())).to_owned()
            };
            scores.push((name, vendor, physical_device, score));
        }

        let selected_index = scores
            .iter()
            .enumerate()
            .max_by_key(|(_i, v)| &v.3)
            .map(|(i, _)| i);
        if let Some(i) = selected_index {
            let out = scores.swap_remove(i);
            Some((out.0, out.1, out.2))
        } else {
            None
        }
    }

    pub fn log_device_info(device_info: &DeviceInfo) {
        unsafe {
            let vendor = pci_id_to_vendor(device_info.properties_10.vendor_id);
            let name = &*str_from_ptr(device_info.properties_10.device_name.as_ptr());

            log::info!("=====================");
            log::info!("Considering Device: ");
            log::info!("Vendor         : {vendor}");
            log::info!("Name           : {name}");

            // Log additional driver information if available
            let v = device_info.properties_10.api_version;
            if vk::api_version_major(v) >= 1 && vk::api_version_minor(v) >= 2 {
                let driver_name =
                    &*str_from_ptr(device_info.driver_properties.driver_name.as_ptr());
                let driver_info =
                    &*str_from_ptr(device_info.driver_properties.driver_info.as_ptr());
                let driver_id = device_info.driver_properties.driver_id;

                log::info!("Driver         : {driver_name}");
                log::info!("Driver ID      : {driver_id:?}");
                log::info!("Driver Info    : {driver_info}");
            }

            // The VERSION_x functions are deprecated but we're supposed to use them here as this
            // is a driver version not an API version. We don't have any 'variant' shenanigans to
            // care about.
            #[allow(deprecated)]
            {
                let dv_major = vk::version_major(device_info.properties_10.driver_version);
                let dv_minor = vk::version_minor(device_info.properties_10.driver_version);
                let dv_patch = vk::version_patch(device_info.properties_10.driver_version);
                log::info!("Driver Version : {dv_major}.{dv_minor}.{dv_patch}");
            }
        }
    }

    pub fn score_device(device_info: &DeviceInfo, options: &AdapterRequestOptions) -> Option<i32> {
        let mut score = 0i32;

        // Whether we prefer an integrated or discrete GPU depends on the requested power class
        let (discrete_score, integrated_score) = match options.power_class {
            AdapterPowerClass::LowPower => (1_000, 10_000),
            AdapterPowerClass::HighPower => (10_000, 1_000),
        };

        // Whether we prefer a hardware or software device depends on the requested type preference
        let (hardware_score, software_score) = match options.type_preference {
            AdapterTypePreference::Hardware => (10_000, 1_000),
            AdapterTypePreference::Software => (1_000, 10_000),
        };

        match device_info.properties_10.device_type {
            vk::PhysicalDeviceType::INTEGRATED_GPU => {
                if !options.deny_hardware_adapters {
                    score += integrated_score;
                    score += hardware_score;
                } else {
                    log::error!("Device is a GPU and deny_hardware_adapters = true");
                    return None;
                }
            }
            vk::PhysicalDeviceType::DISCRETE_GPU => {
                if !options.deny_hardware_adapters {
                    score += discrete_score;
                    score += hardware_score;
                } else {
                    log::error!("Device is a GPU and deny_hardware_adapters = true");
                    return None;
                }
            }
            vk::PhysicalDeviceType::CPU => {
                if options.allow_software_adapters {
                    // CPU devices will perform very slowly compared to a GPU, we should warn in the
                    // logs that we've got one.
                    log::warn!("Device is a CPU");
                    score += software_score;
                } else {
                    log::error!("Device is a CPU and allow_software_adapters = false");
                    return None;
                }
            }
            vk::PhysicalDeviceType::VIRTUAL_GPU => {
                // We make no determination on virtual GPUs, but warn in the logs when we have one
                // as they're likely to be less reliable implementations.
                log::warn!("Device is a 'Virtual GPU'");
            }
            v => {
                log::warn!("Unknown VkPhysicalDeviceType '{}'", v.as_raw());
            }
        }

        Some(score)
    }

    pub fn check_surface_support(
        entry: &ash::Entry,
        instance: &ash::Instance,
        device_info: &DeviceInfo,
        physical_device: vk::PhysicalDevice,
        surface: Option<vk::SurfaceKHR>,
    ) -> Option<()> {
        unsafe {
            let ext_name = ash::khr::swapchain::NAME;
            let swapchain_extension_supported = device_info.supports_extension_cstr(ext_name);

            // The VK_KHR_swapchain must be supported if a surface is requested
            if surface.is_some() & !swapchain_extension_supported {
                let ext_name = ext_name.to_str().unwrap_unchecked();
                log::debug!("Device doesn't support '{ext_name}' extension");
                return None;
            }
        }

        // Check if the device can present to the requested surface, if one was requested
        if let Some(surface) = surface {
            let surface_khr = ash::khr::surface::Instance::new(entry, instance);

            // Load information about the device's support of the requested swap chain. If we can't
            // at least load the surface_capabilities then we assume no support and return None to
            // flag the device as unsuitable
            let (surface_capabilities, surface_formats, present_modes) =
                Self::get_device_surface_support(&surface_khr, physical_device, surface).ok()?;

            // Theoretically you could get no allowed usage flags on the surface, which would
            // mean the device can't actually do anything with the swap images.
            if surface_capabilities.supported_usage_flags.is_empty() {
                log::debug!("Device doesn't allow any usage flags for the surface");
                return None;
            }

            // No present modes means we can't present to the surface. If empty then the surface
            // is unsupported.
            if present_modes.is_empty() {
                log::debug!("Device doesn't expose any present modes for requested surface");
                return None;
            }

            // We require at least a single format for the surface to be supported.
            if surface_formats.is_empty() {
                log::debug!("Device doesn't expose any image formats for requested surface");
                return None;
            }
        }

        Some(())
    }

    pub fn check_device_supports_minimum_features(device_info: &DeviceInfo) -> Option<()> {
        unsafe {
            let is_supported = |v: &CStr| device_info.supports_extension_cstr(v);

            #[allow(unused)]
            macro_rules! check_for_extension {
                ($name:expr) => {{
                    let name = $name;
                    if !device_info.supports_extension(name) {
                        log::error!("Device does not support extension {}", name);
                        return None;
                    }
                }};
            }

            #[allow(unused)]
            macro_rules! check_for_extension_vk {
                ($name:expr) => {{
                    let name = $name.to_str().unwrap_unchecked();
                    if !device_info.supports_extension(name) {
                        log::error!("Device does not support extension {}", name);
                        return None;
                    }
                }};
            }

            // Check we meet requirements for store op none. Check for the three extensions that
            // provide it, failing only if none of them are available.
            if !is_supported(ash::khr::dynamic_rendering::NAME) {
                log::warn!(
                    "Device does not support extension {:?}. Support will be emulated",
                    ash::khr::dynamic_rendering::NAME
                );
                if !is_supported(ash::ext::load_store_op_none::NAME) {
                    log::warn!(
                        "Device does not support extension {:?}. Falling back to QCOM extension",
                        ash::ext::load_store_op_none::NAME
                    );
                    check_for_extension_vk!(ash::qcom::render_pass_store_ops::NAME)
                }
            }

            // macOS will always be MoltenVK and portability subset must be available
            if cfg!(any(target_os = "macos", target_os = "ios")) {
                check_for_extension_vk!(ash::khr::portability_subset::NAME);
            }

            device_info.meets_minimum_requirements()?;

            Some(())
        }
    }

    pub fn get_device_surface_support(
        surface_khr: &ash::khr::surface::Instance,
        physical_device: vk::PhysicalDevice,
        surface: vk::SurfaceKHR,
    ) -> Result<
        (
            vk::SurfaceCapabilitiesKHR,
            Vec<vk::SurfaceFormatKHR>,
            Vec<vk::PresentModeKHR>,
        ),
        vk::Result,
    > {
        let capabilities = unsafe {
            surface_khr
                .get_physical_device_surface_capabilities(physical_device, surface)
                .inspect_err(|v| {
                    log::debug!("Failed to get surface capabilities for surface. Reason {v:?}");
                })?
        };
        let formats = unsafe {
            surface_khr
                .get_physical_device_surface_formats(physical_device, surface)
                .unwrap_or_default()
        };
        let present_modes = unsafe {
            surface_khr
                .get_physical_device_surface_present_modes(physical_device, surface)
                .unwrap_or_default()
        };

        Ok((capabilities, formats, present_modes))
    }
}

impl IContext for Context {
    fn upgrade(&self) -> AnyArc<dyn IContext> {
        AnyArc::map::<dyn IContext, _>(self._this.upgrade().unwrap(), |v| v)
    }

    fn strong_count(&self) -> usize {
        self._this.strong_count()
    }

    fn weak_count(&self) -> usize {
        self._this.weak_count()
    }

    fn request_adapter(&self, options: &AdapterRequestOptions) -> Option<AnyArc<dyn IAdapter>> {
        let surface = options.surface.map(unwrap::surface).map(|v| v.surface);
        Context::select_device(&self.entry_loader, &self.instance, surface, options).map(
            |(name, vendor, physical_device)| {
                new_rhi_object(move |v| Adapter {
                    this: v.clone(),
                    context: self._this.upgrade().unwrap(),
                    name,
                    vendor,
                    physical_device,
                    device_info: DeviceInfo::load(&self.instance, physical_device),
                })
            },
        )
    }

    fn create_surface(
        &self,
        display: &dyn HasDisplayHandle,
        window: &dyn HasWindowHandle,
    ) -> Result<AnyArc<dyn ISurface>, SurfaceCreateError> {
        let display = display.display_handle().unwrap().as_raw();
        let window = window.window_handle().unwrap().as_raw();
        let result = unsafe {
            match (display, window) {
                #[cfg(any(
                    target_os = "linux",
                    target_os = "dragonfly",
                    target_os = "freebsd",
                    target_os = "netbsd",
                    target_os = "openbsd"
                ))]
                (RawDisplayHandle::Wayland(display), RawWindowHandle::Wayland(window)) => {
                    let create_info = vk::WaylandSurfaceCreateInfoKHR {
                        display: display.display.as_ptr(),
                        surface: window.surface.as_ptr(),
                        ..Default::default()
                    };

                    self.surface_loaders
                        .wayland
                        .as_ref()
                        .unwrap()
                        .create_wayland_surface(&create_info, GLOBAL)
                }

                #[cfg(any(
                    target_os = "linux",
                    target_os = "dragonfly",
                    target_os = "freebsd",
                    target_os = "netbsd",
                    target_os = "openbsd"
                ))]
                (RawDisplayHandle::Xlib(display), RawWindowHandle::Xlib(window)) => {
                    let create_info = vk::XlibSurfaceCreateInfoKHR {
                        dpy: display.display.unwrap().as_ptr(),
                        window: window.window as _,
                        ..Default::default()
                    };

                    self.surface_loaders
                        .xlib
                        .as_ref()
                        .unwrap()
                        .create_xlib_surface(&create_info, GLOBAL)
                }

                #[cfg(any(
                    target_os = "linux",
                    target_os = "dragonfly",
                    target_os = "freebsd",
                    target_os = "netbsd",
                    target_os = "openbsd"
                ))]
                (RawDisplayHandle::Xcb(display), RawWindowHandle::Xcb(window)) => {
                    let create_info = vk::XcbSurfaceCreateInfoKHR {
                        connection: display.connection.unwrap().as_ptr(),
                        window: window.window.get(),
                        ..Default::default()
                    };

                    self.surface_loaders
                        .xcb
                        .as_ref()
                        .unwrap()
                        .create_xcb_surface(&create_info, GLOBAL)
                }

                #[cfg(target_os = "macos")]
                (RawDisplayHandle::AppKit(_), RawWindowHandle::AppKit(window)) => {
                    let create_info = vk::MacOSSurfaceCreateInfoMVK {
                        p_view: window.ns_view.as_ptr(),
                        ..Default::default()
                    };

                    self.surface_loaders
                        .macos
                        .as_ref()
                        .unwrap()
                        .create_mac_os_surface(&create_info, GLOBAL)
                }

                #[cfg(target_os = "ios")]
                (RawDisplayHandle::UiKit(_), RawWindowHandle::UiKit(window)) => {
                    let create_info = vk::IOSSurfaceCreateInfoMVK {
                        p_view: window.ui_view.as_ptr(),
                        ..Default::default()
                    };

                    self.surface_loaders
                        .ios
                        .as_ref()
                        .unwrap()
                        .create_ios_surface(&create_info, GLOBAL)
                }

                #[cfg(target_os = "windows")]
                (RawDisplayHandle::Windows(_), RawWindowHandle::Win32(window)) => {
                    let create_info = vk::Win32SurfaceCreateInfoKHR {
                        hinstance: window.hinstance.unwrap().get(),
                        hwnd: window.hwnd.get(),
                        ..Default::default()
                    };

                    self.surface_loaders
                        .win32
                        .as_ref()
                        .unwrap()
                        .create_win32_surface(&create_info, GLOBAL)
                }

                _ => {
                    log::error!(
                        "Requested Surface for unsupported WSI handle: display {:?} + window {:?}",
                        display,
                        window
                    );
                    return Err(SurfaceCreateError::UnsupportedWSI);
                }
            }
        };

        let surface = result.map_err(|e| log::error!("Platform Error: {:#?}", e))?;

        Ok(new_rhi_object(move |v| Surface {
            this: v.clone(),
            surface,
            context: self._this.upgrade().unwrap(),
        }))
    }

    fn create_surface_for_metal_layer(
        &self,
        layer: NonNull<c_void>,
    ) -> Result<AnyArc<dyn ISurface>, SurfaceCreateError> {
        if !cfg!(any(target_os = "macos", target_os = "ios")) {
            log::warn!("Called 'IContext::create_surface_for_metal_layer' on non apple platform!");
            return Err(SurfaceCreateError::UnsupportedWSI);
        }

        let result = unsafe {
            let create_info = vk::MetalSurfaceCreateInfoEXT {
                p_layer: layer.as_ptr(),
                ..Default::default()
            };
            self.surface_loaders
                .metal
                .as_ref()
                .unwrap()
                .create_metal_surface(&create_info, GLOBAL)
        };

        let surface = result.map_err(|e| log::error!("Platform Error: {:#?}", e))?;

        let surface = AnyArc::new_cyclic(move |v| Surface {
            this: v.clone(),
            surface,
            context: self._this.upgrade().unwrap(),
        });
        Ok(AnyArc::map::<dyn ISurface, _>(surface, |v| v))
    }

    fn get_backend_api(&self) -> BackendAPI {
        BackendAPI::Vulkan
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            if let (Some(debug_loader), Some(messenger)) = (&self.debug_loader, &self.messenger) {
                debug_loader.destroy_debug_utils_messenger(*messenger, GLOBAL);
            }
            self.instance.destroy_instance(GLOBAL);
            ManuallyDrop::drop(&mut self.instance);
            ManuallyDrop::drop(&mut self.entry_loader);
            ManuallyDrop::drop(&mut self.library);
        }
    }
}

/// Internal wrapper struct to make it easier to pass the surface extensions around
#[allow(dead_code)] // Some of these get disabled on different platforms
pub struct SurfaceLoaders {
    pub base: Option<ash::khr::surface::Instance>,
    pub win32: Option<ash::khr::win32_surface::Instance>,
    pub xlib: Option<ash::khr::xlib_surface::Instance>,
    pub xcb: Option<ash::khr::xcb_surface::Instance>,
    pub wayland: Option<ash::khr::wayland_surface::Instance>,
    pub metal: Option<ash::ext::metal_surface::Instance>,
    pub macos: Option<ash::mvk::macos_surface::Instance>,
    pub ios: Option<ash::mvk::ios_surface::Instance>,
}
