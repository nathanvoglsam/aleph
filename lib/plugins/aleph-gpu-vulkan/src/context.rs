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

use crate::adapter::Adapter;
use crate::internal::device_info::DeviceInfo;
use crate::internal::unwrap;
use crate::surface::Surface;
use aleph_gpu_impl_utils::conv::pci_id_to_vendor;
use erupt::vk;
use interfaces::any::{declare_interfaces, AnyArc, AnyWeak};
use interfaces::anyhow::anyhow;
use interfaces::gpu::*;
use interfaces::platform::{HasRawWindowHandle, RawWindowHandle};
use std::any::TypeId;
use std::ffi::CStr;
use std::mem::ManuallyDrop;

pub struct Context {
    pub(crate) _this: AnyWeak<Self>,
    pub(crate) entry_loader: ManuallyDrop<erupt::EntryLoader>,
    pub(crate) instance_loader: ManuallyDrop<erupt::InstanceLoader>,
    pub(crate) messenger: Option<vk::DebugUtilsMessengerEXT>,
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
        instance: &erupt::InstanceLoader,
        surface: Option<vk::SurfaceKHR>,
        options: &AdapterRequestOptions,
    ) -> Option<(String, AdapterVendor, vk::PhysicalDevice)> {
        let devices = unsafe {
            instance
                .enumerate_physical_devices(None)
                .expect("Failed to enumerate vulkan devices")
        };
        let mut scores: Vec<(&str, AdapterVendor, vk::PhysicalDevice, i32)> = Vec::new();

        for physical_device in devices.iter().copied() {
            let device_info = DeviceInfo::load(instance, physical_device);

            Self::log_device_info(&device_info);

            if vk::api_version_major(device_info.properties_10.api_version) < 1 {
                log::trace!("Device does not support Vulkan 1.x");
                continue;
            }

            if vk::api_version_minor(device_info.properties_10.api_version) < 2 {
                log::trace!("Device does not support Vulkan 1.2");
            }

            // Check if the physical device supports the requested surface
            if Self::check_surface_support(
                instance,
                physical_device,
                &device_info.extensions,
                surface,
            )
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
            let name = unsafe {
                CStr::from_ptr(device_info.properties_10.device_name.as_ptr())
                    .to_str()
                    .unwrap()
            };
            scores.push((name, vendor, physical_device, score));
        }

        scores
            .iter()
            .max_by_key(|v| &v.3)
            .map(|v| (v.0.to_owned(), v.1, v.2))
    }

    pub fn log_device_info(device_info: &DeviceInfo) {
        unsafe {
            let vendor = pci_id_to_vendor(device_info.properties_10.vendor_id);
            let name = CStr::from_ptr(device_info.properties_10.device_name.as_ptr())
                .to_str()
                .unwrap();

            log::trace!("=====================");
            log::trace!("Considering Device: ");
            log::trace!("Vendor         : {vendor}");
            log::trace!("Name           : {name}");

            // Log additional driver information if available
            let v = device_info.properties_10.api_version;
            if vk::api_version_major(v) >= 1 && vk::api_version_minor(v) >= 2 {
                let driver_name = CStr::from_ptr(device_info.properties_12.driver_name.as_ptr())
                    .to_str()
                    .unwrap();
                let driver_info = CStr::from_ptr(device_info.properties_12.driver_info.as_ptr())
                    .to_str()
                    .unwrap();

                log::trace!("Driver         : {driver_name}");
                log::trace!("Driver ID      : {:?}", device_info.properties_12.driver_id);
                log::trace!("Driver Info    : {driver_info}");
            }

            let dv_major = vk::api_version_major(device_info.properties_10.driver_version);
            let dv_minor = vk::api_version_minor(device_info.properties_10.driver_version);
            let dv_patch = vk::api_version_patch(device_info.properties_10.driver_version);
            log::trace!("Driver Version : {dv_major}.{dv_minor}.{dv_patch}");
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
                log::warn!("Unknown VkPhysicalDeviceType '{}'", v.0);
            }
        }

        Some(score)
    }

    pub fn check_surface_support(
        instance: &erupt::InstanceLoader,
        physical_device: vk::PhysicalDevice,
        extensions: &[vk::ExtensionProperties],
        surface: Option<vk::SurfaceKHR>,
    ) -> Option<()> {
        use erupt::extensions::*;

        unsafe {
            let ext_name = CStr::from_ptr(khr_swapchain::KHR_SWAPCHAIN_EXTENSION_NAME)
                .to_str()
                .unwrap_unchecked();

            let surface_extension_supported = extensions
                .iter()
                .map(|v| {
                    CStr::from_ptr(v.extension_name.as_ptr())
                        .to_str()
                        .unwrap_unchecked()
                })
                .any(|v| v == ext_name);

            // The VK_KHR_surface must be supported if a surface is requested
            if surface.is_some() & !surface_extension_supported {
                log::trace!("Device doesn't support '{ext_name}' extension");
                return None;
            }
        }

        // Check if the device can present to the requested surface, if one was requested
        if let Some(surface) = surface {
            // Load information about the device's support of the requested swap chain. If we can't
            // at least load the surface_capabilities then we assume no support and return None to
            // flag the device as unsuitable
            let (surface_capabilities, surface_formats, present_modes) =
                Self::get_device_surface_support(instance, physical_device, surface).ok()?;

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
        let DeviceInfo {
            extensions,
            properties_10,
            // properties_11,
            // properties_12,
            // portability_properties,
            features_10,
            // features_11,
            features_12,
            dynamic_rendering_features,
            // portability_features,
            ..
        } = device_info;

        unsafe {
            #[allow(unused)]
            macro_rules! check_for_feature {
                ($f:expr) => {
                    let text = stringify!($f);
                    if $f == false {
                        log::error!("Device does not support feature: '{}'", text);
                        return None;
                    }
                };
            }

            #[allow(unused)]
            macro_rules! check_for_feature_vk {
                ($f:expr) => {
                    let text = stringify!($f);
                    if $f != vk::TRUE {
                        log::error!("Device does not support feature: '{}'", text);
                        return None;
                    }
                };
            }

            #[allow(unused)]
            macro_rules! check_for_extension {
                ($name:expr) => {{
                    let name = $name;
                    let has = extensions
                        .iter()
                        .map(|v| {
                            CStr::from_ptr(v.extension_name.as_ptr())
                                .to_str()
                                .unwrap_unchecked()
                        })
                        .any(|v| v == $name);

                    if !has {
                        log::error!("Device does not support extension {}", name);
                        return None;
                    }
                }};
            }

            #[allow(unused)]
            macro_rules! check_for_extension_vk {
                ($name:expr) => {{
                    let name = CStr::from_ptr($name).to_str().unwrap_unchecked();
                    let has = extensions
                        .iter()
                        .map(|v| {
                            CStr::from_ptr(v.extension_name.as_ptr())
                                .to_str()
                                .unwrap_unchecked()
                        })
                        .any(|v| v == name);

                    if !has {
                        log::error!("Device does not support extension {}", name);
                        return None;
                    }
                }};
            }

            #[allow(unused)]
            macro_rules! check_for_limit_min {
                ($limit:expr, $min:expr) => {{
                    let limit_name = stringify!($limit);
                    let limit = $limit;
                    let min = $min;
                    if limit < min {
                        log::error!(
                            "Device limit '{limit_name}' too low. Want: {min}, got {limit}"
                        );
                        return None;
                    }
                }};
            }

            check_for_extension_vk!(vk::KHR_DYNAMIC_RENDERING_EXTENSION_NAME);
            check_for_extension_vk!(vk::KHR_SYNCHRONIZATION_2_EXTENSION_NAME);

            // macOS will always be MoltenVK and portability subset must be available
            if cfg!(target_os = "macos") {
                check_for_extension_vk!(vk::KHR_PORTABILITY_SUBSET_EXTENSION_NAME);
            }

            check_for_limit_min!(properties_10.limits.max_bound_descriptor_sets, 8);

            check_for_feature_vk!(features_10.full_draw_index_uint32);
            check_for_feature_vk!(features_12.descriptor_indexing);
            check_for_feature_vk!(features_12.buffer_device_address);
            check_for_feature_vk!(features_12.timeline_semaphore);
            check_for_feature_vk!(dynamic_rendering_features.dynamic_rendering);

            Some(())
        }
    }

    pub fn get_device_surface_support(
        instance: &erupt::InstanceLoader,
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
            instance
                .get_physical_device_surface_capabilities_khr(physical_device, surface)
                .result()?
        };
        let formats = unsafe {
            instance
                .get_physical_device_surface_formats_khr(physical_device, surface, None)
                .result()
                .unwrap_or_default()
                .to_vec()
        };
        let present_modes = unsafe {
            instance
                .get_physical_device_surface_present_modes_khr(physical_device, surface, None)
                .result()
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
        Context::select_device(&self.instance_loader, surface, options).map(
            |(name, vendor, physical_device)| {
                let adapter = AnyArc::new_cyclic(move |v| Adapter {
                    this: v.clone(),
                    context: self._this.upgrade().unwrap(),
                    name,
                    vendor,
                    physical_device,
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
                    use erupt::extensions::khr_wayland_surface::*;

                    let create_info = WaylandSurfaceCreateInfoKHR {
                        display: handle.display,
                        surface: handle.surface,
                        ..Default::default()
                    };

                    self.instance_loader
                        .create_wayland_surface_khr(&create_info, None)
                }

                #[cfg(any(
                    target_os = "linux",
                    target_os = "dragonfly",
                    target_os = "freebsd",
                    target_os = "netbsd",
                    target_os = "openbsd"
                ))]
                RawWindowHandle::Xlib(handle) => {
                    use erupt::extensions::khr_xlib_surface::*;

                    let create_info = XlibSurfaceCreateInfoKHR {
                        dpy: handle.display as *mut _,
                        window: handle.window as _,
                        ..Default::default()
                    };

                    self.instance_loader
                        .create_xlib_surface_khr(&create_info, None)
                }

                #[cfg(any(
                    target_os = "linux",
                    target_os = "dragonfly",
                    target_os = "freebsd",
                    target_os = "netbsd",
                    target_os = "openbsd"
                ))]
                RawWindowHandle::Xcb(handle) => {
                    use erupt::extensions::khr_xcb_surface::*;

                    let create_info = XcbSurfaceCreateInfoKHR {
                        connection: handle.connection as *mut _,
                        window: handle.window,
                        ..Default::default()
                    };

                    self.instance_loader
                        .create_xcb_surface_khr(&create_info, None)
                }

                #[cfg(any(target_os = "android"))]
                RawWindowHandle::Android(handle) => {
                    use erupt::extensions::khr_android_surface::*;

                    let create_info = AndroidSurfaceCreateInfoKHR {
                        window: handle.a_native_window as _,
                        ..Default::default()
                    };

                    self.instance_loader
                        .create_android_surface_khr(&create_info, None)
                }

                #[cfg(any(target_os = "macos"))]
                RawWindowHandle::AppKit(handle) => {
                    use erupt::extensions::mvk_macos_surface::*;

                    let create_info = MacOSSurfaceCreateInfoMVK {
                        p_view: &*handle.ns_view,
                        ..Default::default()
                    };

                    self.instance_loader
                        .create_mac_os_surface_mvk(&create_info, None)
                }

                #[cfg(any(target_os = "ios"))]
                RawWindowHandle::IOS(handle) => {
                    use erupt::extensions::mvk_ios_surface::*;

                    let create_info = IOSSurfaceCreateInfoMVK {
                        p_view: &*handle.ui_view,
                        ..Default::default()
                    };

                    self.instance_loader
                        .create_ios_surface_mvk(&create_info, None)
                }

                #[cfg(target_os = "windows")]
                RawWindowHandle::Win32(handle) => {
                    use erupt::extensions::khr_win32_surface::*;

                    let create_info = Win32SurfaceCreateInfoKHR {
                        hinstance: handle.hinstance,
                        hwnd: handle.hwnd,
                        ..Default::default()
                    };

                    self.instance_loader
                        .create_win32_surface_khr(&create_info, None)
                }

                _ => panic!("Unsupported WSI type"),
            }
        };

        let surface = result.result().map_err(|e| anyhow!(e))?;

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
            if let Some(messenger) = self.messenger {
                self.instance_loader
                    .destroy_debug_utils_messenger_ext(messenger, None);
            }
            self.instance_loader.destroy_instance(None);
            ManuallyDrop::drop(&mut self.instance_loader);
            ManuallyDrop::drop(&mut self.entry_loader);
        }
    }
}
