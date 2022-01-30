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
use crate::surface::Surface;
use erupt::vk;
use interfaces::any::{declare_interfaces, QueryInterface, QueryInterfaceBox};
use interfaces::gpu::{
    AdapterPowerClass, AdapterRequestOptions, IAdapter, IContext, ISurface, SurfaceCreateError,
};
use interfaces::platform::{HasRawWindowHandle, RawWindowHandle};
use std::ffi::CStr;

pub struct Context {
    pub(crate) instance_loader: erupt::InstanceLoader,
    pub(crate) messenger: Option<vk::DebugUtilsMessengerEXT>,
}

impl Context {
    fn select_device(
        instance: &erupt::InstanceLoader,
        // features: &PhysicalDeviceFeatures,
        major_version: u32,
        minor_version: u32,
        surface: Option<vk::SurfaceKHR>,
        power_class: AdapterPowerClass,
    ) -> Option<(String, vk::PhysicalDevice)> {
        let devices = unsafe {
            instance
                .enumerate_physical_devices(None)
                .expect("Failed to enumerate vulkan devices")
        };
        let mut scores: Vec<(&str, vk::PhysicalDevice, i32)> = Vec::new();

        for physical_device in devices.iter().copied() {
            let (properties, features, extensions) = unsafe {
                let properties = instance.get_physical_device_properties(physical_device);
                let features = instance.get_physical_device_features(physical_device);
                let extensions = instance
                    .enumerate_device_extension_properties(physical_device, None, None)
                    .result()
                    .unwrap_or_default();
                (properties, features, extensions)
            };

            // Push the score for the device, if the device meets the minimum requirements
            if let Some(score) = Self::score_device(
                instance,
                physical_device,
                &properties,
                &features,
                &extensions,
                major_version,
                minor_version,
                surface,
                power_class,
            ) {
                // Convert the GPU name to a string
                let name = unsafe {
                    CStr::from_ptr(properties.device_name.as_ptr())
                        .to_str()
                        .unwrap()
                };
                scores.push((name, physical_device, score));
            }
        }

        scores
            .iter()
            .max_by_key(|v| &v.1)
            .map(|v| (v.0.to_owned(), v.1))
    }

    #[allow(clippy::too_many_arguments)]
    fn score_device(
        instance: &erupt::InstanceLoader,
        physical_device: vk::PhysicalDevice,
        properties: &vk::PhysicalDeviceProperties,
        _features: &vk::PhysicalDeviceFeatures,
        extensions: &[vk::ExtensionProperties],
        major_version: u32,
        minor_version: u32,
        surface: Option<vk::SurfaceKHR>,
        power_class: AdapterPowerClass,
    ) -> Option<i32> {
        use erupt::extensions::*;

        let mut score = 0i32;

        unsafe {
            let khr_surface_name = CStr::from_ptr(khr_surface::KHR_SURFACE_EXTENSION_NAME);
            let surface_extension_supported = extensions
                .iter()
                .map(|v| CStr::from_ptr(v.extension_name.as_ptr()))
                .any(|v| v == khr_surface_name);

            // The VK_KHR_surface must be supported if a surface is requested
            if surface.is_some() & !surface_extension_supported {
                return None;
            }
        }

        // Check if the device can present to the requested surface, if one was requested
        if let Some(surface) = surface {
            // Load information about the device's support of the requested swap chain. If we can't
            // at least load the surface_capabilities then we assume no support and return None to
            // flag the device as unsuitable
            let (_surface_capabilities, surface_formats, present_modes) =
                Self::get_device_surface_support(instance, physical_device, surface)?;

            // No present modes means we can't present to the surface. If empty then the surface
            // is unsupported.
            if present_modes.is_empty() {
                return None;
            }

            // We require at least a single format for the surface to be supported.
            if surface_formats.is_empty() {
                return None;
            }
        }

        // Major version breaks API compatibility so the expected version must match exactly
        let device_major_version = erupt::vk1_0::api_version_major(properties.api_version);
        if device_major_version == major_version {
            return None;
        }

        // Minor versions are additive so we only need to check be sure the minor version is
        // at least the requested minor version
        let device_minor_version = erupt::vk1_0::api_version_minor(properties.api_version);
        if device_minor_version < minor_version {
            return None;
        }

        // Whether we prefer an integrated or discrete GPU depends on the requested power class
        let (discrete_score, integrated_score) = match power_class {
            AdapterPowerClass::LowPower => (1_000, 10_000),
            AdapterPowerClass::HighPower => (10_000, 1_000),
        };
        // We only care about choosing between discrete or integrated. The other types are
        // either obscuring the underlying hardware (virtual) or too slow to care (cpu).
        if properties.device_type == vk::PhysicalDeviceType::DISCRETE_GPU {
            score += discrete_score;
        } else if properties.device_type == vk::PhysicalDeviceType::INTEGRATED_GPU {
            score += integrated_score;
        }

        Some(score)
    }

    fn get_device_surface_support(
        instance: &erupt::InstanceLoader,
        physical_device: vk::PhysicalDevice,
        surface: vk::SurfaceKHR,
    ) -> Option<(
        vk::SurfaceCapabilitiesKHR,
        Vec<vk::SurfaceFormatKHR>,
        Vec<vk::PresentModeKHR>,
    )> {
        let capabilities = unsafe {
            instance
                .get_physical_device_surface_capabilities_khr(physical_device, surface)
                .ok()?
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

        Some((capabilities, formats, present_modes))
    }
}

impl IContext for Context {
    fn request_adapter(&self, options: &AdapterRequestOptions) -> Option<Box<dyn IAdapter>> {
        let surface = options
            .surface
            .and_then(|v| v.query_interface::<Surface>())
            .map(|v| v.surface);
        Context::select_device(
            &self.instance_loader,
            todo!(),
            todo!(),
            surface,
            options.power_class,
        )
        .map(|(name, physical_device)| {
            Box::new(Adapter {
                name,
                physical_device,
                context: todo!(),
            })
            .query_interface()
            .ok()
            .unwrap()
        })
    }

    fn create_surface(
        &self,
        window: &dyn HasRawWindowHandle,
    ) -> Result<Box<dyn ISurface>, SurfaceCreateError> {
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

                    instance_loader.create_wayland_surface_khr(&create_info, None)
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
                        window: handle.window,
                        ..Default::default()
                    };

                    instance_loader.create_xlib_surface_khr(&create_info, None)
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

                    instance_loader.create_xcb_surface_khr(&create_info, None)
                }

                #[cfg(any(target_os = "android"))]
                RawWindowHandle::Android(handle) => {
                    use erupt::extensions::khr_android_surface::*;

                    let create_info = AndroidSurfaceCreateInfoKHR {
                        window: handle.a_native_window as _,
                        ..Default::default()
                    };

                    instance_loader.create_android_surface_khr(&create_info, None)
                }

                #[cfg(any(target_os = "macos"))]
                RawWindowHandle::MacOS(handle) => {
                    use erupt::extensions::mvk_macos_surface::*;

                    let create_info = MacOSSurfaceCreateInfoMVK {
                        p_view: &*handle.ns_view,
                        ..Default::default()
                    };

                    instance_loader.create_mac_os_surface_mvk(&create_info, None)
                }

                #[cfg(any(target_os = "ios"))]
                RawWindowHandle::IOS(handle) => {
                    use erupt::extensions::mvk_ios_surface::*;

                    let create_info = IOSSurfaceCreateInfoMVK {
                        p_view: &*handle.ui_view,
                        ..Default::default()
                    };

                    instance_loader.create_ios_surface_mvk(&create_info, None)
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

        let surface = result
            .result()
            .map_err(|v| SurfaceCreateError::Platform(Box::new(v)))?;
        let surface = Surface { surface };

        Ok(Box::new(surface))
    }
}

pub trait IContextExt: IContext {}

impl IContextExt for Context {}

declare_interfaces!(Context, [IContext, IContextExt]);
