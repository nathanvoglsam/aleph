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
use std::ffi::CStr;
use std::mem::ManuallyDrop;

use aleph_any::{declare_interfaces, AnyArc, AnyWeak};
use aleph_rhi_api::*;
use aleph_rhi_impl_utils::conv::pci_id_to_vendor;
use aleph_rhi_impl_utils::str_from_ptr;
use aleph_rhi_loader_api::VulkanConfig;
use ash::vk;
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

use crate::adapter::Adapter;
use crate::internal::device_info::DeviceInfo;
use crate::internal::unwrap;
use crate::surface::Surface;

pub struct Context {
    pub _this: AnyWeak<Self>,
    pub config: VulkanConfig,
    pub library: ManuallyDrop<libloading::Library>,
    pub entry_loader: ManuallyDrop<ash::Entry>,
    pub instance: ManuallyDrop<ash::Instance>,
    pub surface_loaders: SurfaceLoaders,
    pub debug_loader: Option<ash::extensions::ext::DebugUtils>,
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
                log::trace!("Device does not support Vulkan 1.x");
                continue;
            }

            if vk::api_version_minor(device_info.properties_10.api_version) < 1 {
                log::trace!("Device does not support Vulkan 1.1");
            }

            // Check if the physical device supports the requested surface
            if Self::check_surface_support(entry, instance, &device_info, physical_device, surface)
                .is_none()
            {
                continue;
            }

            if let None = Self::check_device_supports_minimum_features(&device_info) {
                log::trace!("Device rejected as doesn't support minimum feature requirements");
                continue;
            }

            // Score the physical device based on the device preferences provided by the user
            let score = Self::score_device(&device_info, options);
            let score = match score {
                None => continue,
                Some(v) => v,
            };

            let vendor = pci_id_to_vendor(device_info.properties_10.vendor_id);
            let name =
                unsafe { str_from_ptr(device_info.properties_10.device_name.as_ptr()).to_owned() };
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
            let name = str_from_ptr(device_info.properties_10.device_name.as_ptr());

            log::trace!("=====================");
            log::trace!("Considering Device: ");
            log::trace!("Vendor         : {vendor}");
            log::trace!("Name           : {name}");

            // Log additional driver information if available
            let v = device_info.properties_10.api_version;
            if vk::api_version_major(v) >= 1 && vk::api_version_minor(v) >= 2 {
                let driver_name = str_from_ptr(device_info.driver_properties.driver_name.as_ptr());
                let driver_info = str_from_ptr(device_info.driver_properties.driver_info.as_ptr());
                let driver_id = device_info.driver_properties.driver_id;

                log::trace!("Driver         : {driver_name}");
                log::trace!("Driver ID      : {driver_id:?}");
                log::trace!("Driver Info    : {driver_info}");
            }

            // The VERSION_x functions are deprecated but we're supposed to use them here as this
            // is a driver version not an API version. We don't have any 'variant' shenanigans to
            // care about.
            #[allow(deprecated)]
            {
                let dv_major = vk::version_major(device_info.properties_10.driver_version);
                let dv_minor = vk::version_minor(device_info.properties_10.driver_version);
                let dv_patch = vk::version_patch(device_info.properties_10.driver_version);
                log::trace!("Driver Version : {dv_major}.{dv_minor}.{dv_patch}");
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
            v @ _ => {
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
            let ext_name = vk::KhrSwapchainFn::name().to_str().unwrap_unchecked();
            let surface_extension_supported = device_info.supports_extension(ext_name);

            // The VK_KHR_surface must be supported if a surface is requested
            if surface.is_some() & !surface_extension_supported {
                log::trace!("Device doesn't support '{ext_name}' extension");
                return None;
            }
        }

        // Check if the device can present to the requested surface, if one was requested
        if let Some(surface) = surface {
            let surface_khr = ash::extensions::khr::Surface::new(entry, instance);

            // Load information about the device's support of the requested swap chain. If we can't
            // at least load the surface_capabilities then we assume no support and return None to
            // flag the device as unsuitable
            let (surface_capabilities, surface_formats, present_modes) =
                Self::get_device_surface_support(&surface_khr, physical_device, surface).ok()?;

            // Theoretically you could get no allowed usage flags on the surface, which would
            // mean the device can't actually do anything with the swap images.
            if surface_capabilities.supported_usage_flags.is_empty() {
                log::trace!("Device doesn't allow any usage flags for the surface");
                return None;
            }

            // No present modes means we can't present to the surface. If empty then the surface
            // is unsupported.
            if present_modes.is_empty() {
                log::trace!("Device doesn't expose any present modes for requested surface");
                return None;
            }

            // We require at least a single format for the surface to be supported.
            if surface_formats.is_empty() {
                log::trace!("Device doesn't expose any image formats for requested surface");
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
            if !is_supported(vk::KhrDynamicRenderingFn::name()) {
                log::warn!(
                    "Device does not support extension {:?}. Support will be emulated",
                    vk::KhrDynamicRenderingFn::name()
                );
                if !is_supported(vk::ExtLoadStoreOpNoneFn::name()) {
                    log::warn!(
                        "Device does not support extension {:?}. Falling back to QCOM extension",
                        vk::ExtLoadStoreOpNoneFn::name()
                    );
                    check_for_extension_vk!(vk::QcomRenderPassStoreOpsFn::name())
                }
            }

            // macOS will always be MoltenVK and portability subset must be available
            if cfg!(target_os = "macos") {
                check_for_extension_vk!(vk::KhrPortabilitySubsetFn::name());
            }

            device_info.meets_minimum_requirements()?;

            Some(())
        }
    }

    pub fn get_device_surface_support(
        surface_khr: &ash::extensions::khr::Surface,
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
            surface_khr.get_physical_device_surface_capabilities(physical_device, surface)?
        };
        let formats = unsafe {
            surface_khr
                .get_physical_device_surface_formats(physical_device, surface)
                .unwrap_or_default()
                .to_vec()
        };
        let present_modes = unsafe {
            surface_khr
                .get_physical_device_surface_present_modes(physical_device, surface)
                .unwrap_or_default()
                .to_vec()
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
                let adapter = AnyArc::new_cyclic(move |v| Adapter {
                    this: v.clone(),
                    context: self._this.upgrade().unwrap(),
                    name,
                    vendor,
                    physical_device,
                    device_info: DeviceInfo::load(&self.instance, physical_device),
                });
                AnyArc::map::<dyn IAdapter, _>(adapter, |v| v)
            },
        )
    }

    fn create_surface(
        &self,
        window: &dyn HasRawWindowHandle,
    ) -> Result<AnyArc<dyn ISurface>, SurfaceCreateError> {
        let result = unsafe {
            match window.raw_window_handle() {
                #[cfg(any(
                    target_os = "linux",
                    target_os = "dragonfly",
                    target_os = "freebsd",
                    target_os = "netbsd",
                    target_os = "openbsd"
                ))]
                RawWindowHandle::Wayland(handle) => {
                    let create_info = vk::WaylandSurfaceCreateInfoKHR {
                        display: handle.display,
                        surface: handle.surface,
                        ..Default::default()
                    };

                    self.surface_loaders
                        .wayland
                        .as_ref()
                        .unwrap()
                        .create_wayland_surface(&create_info, None)
                }

                #[cfg(any(
                    target_os = "linux",
                    target_os = "dragonfly",
                    target_os = "freebsd",
                    target_os = "netbsd",
                    target_os = "openbsd"
                ))]
                RawWindowHandle::Xlib(handle) => {
                    let create_info = vk::XlibSurfaceCreateInfoKHR {
                        dpy: handle.display as *mut _,
                        window: handle.window as _,
                        ..Default::default()
                    };

                    self.surface_loaders
                        .xlib
                        .as_ref()
                        .unwrap()
                        .create_xlib_surface(&create_info, None)
                }

                #[cfg(any(
                    target_os = "linux",
                    target_os = "dragonfly",
                    target_os = "freebsd",
                    target_os = "netbsd",
                    target_os = "openbsd"
                ))]
                RawWindowHandle::Xcb(handle) => {
                    let create_info = vk::XcbSurfaceCreateInfoKHR {
                        connection: handle.connection as *mut _,
                        window: handle.window,
                        ..Default::default()
                    };

                    self.surface_loaders
                        .xcb
                        .as_ref()
                        .unwrap()
                        .create_xcb_surface(&create_info, None)
                }

                #[cfg(any(target_os = "android"))]
                RawWindowHandle::AndroidNdk(handle) => {
                    let create_info = vk::AndroidSurfaceCreateInfoKHR {
                        window: handle.a_native_window as _,
                        ..Default::default()
                    };

                    self.surface_loaders
                        .android
                        .as_ref()
                        .unwrap()
                        .create_android_surface(&create_info, None)
                }

                #[cfg(any(target_os = "macos"))]
                RawWindowHandle::AppKit(handle) => {
                    let create_info = vk::MacOSSurfaceCreateInfoMVK {
                        p_view: &*handle.ns_view,
                        ..Default::default()
                    };

                    self.surface_loaders
                        .macos
                        .as_ref()
                        .unwrap()
                        .create_mac_os_surface(&create_info, None)
                }

                #[cfg(any(target_os = "ios"))]
                RawWindowHandle::IOS(handle) => {
                    let create_info = vk::IOSSurfaceCreateInfoMVK {
                        p_view: &*handle.ui_view,
                        ..Default::default()
                    };

                    self.surface_loaders
                        .ios
                        .as_ref()
                        .unwrap()
                        .create_ios_surface(&create_info, None)
                }

                #[cfg(target_os = "windows")]
                RawWindowHandle::Win32(handle) => {
                    let create_info = vk::Win32SurfaceCreateInfoKHR {
                        hinstance: handle.hinstance,
                        hwnd: handle.hwnd,
                        ..Default::default()
                    };

                    self.surface_loaders
                        .win32
                        .as_ref()
                        .unwrap()
                        .create_win32_surface(&create_info, None)
                }

                _ => panic!("Unsupported WSI type"),
            }
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
            match (&self.debug_loader, &self.messenger) {
                (Some(debug_loader), Some(messenger)) => {
                    debug_loader.destroy_debug_utils_messenger(*messenger, None);
                }
                _ => {}
            }
            self.instance.destroy_instance(None);
            ManuallyDrop::drop(&mut self.instance);
            ManuallyDrop::drop(&mut self.entry_loader);
            ManuallyDrop::drop(&mut self.library);
        }
    }
}

/// Internal wrapper struct to make it easier to pass the surface extensions around
pub struct SurfaceLoaders {
    pub base: Option<ash::extensions::khr::Surface>,
    pub win32: Option<ash::extensions::khr::Win32Surface>,
    pub xlib: Option<ash::extensions::khr::XlibSurface>,
    pub xcb: Option<ash::extensions::khr::XcbSurface>,
    pub wayland: Option<ash::extensions::khr::WaylandSurface>,
    pub android: Option<ash::extensions::khr::AndroidSurface>,
    pub macos: Option<ash::extensions::mvk::MacOSSurface>,
    pub ios: Option<ash::extensions::mvk::IOSSurface>,
}
