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
use crate::internal::profile::{profile_props_from_loaders, PROFILE_NAME, PROFILE_SPEC};
use crate::internal::strcpy::strcpy_str_to_cstr;
use crate::internal::unwrap;
use crate::surface::Surface;
use aleph_gpu_impl_utils::conv::pci_id_to_vendor;
use aleph_vulkan_profiles::*;
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
    fn select_device<T>(
        entry: &erupt::CustomEntryLoader<T>,
        instance: &erupt::InstanceLoader,
        surface: Option<vk::SurfaceKHR>,
        power_class: AdapterPowerClass,
    ) -> Option<(String, AdapterVendor, vk::PhysicalDevice)> {
        let devices = unsafe {
            instance
                .enumerate_physical_devices(None)
                .expect("Failed to enumerate vulkan devices")
        };
        let mut scores: Vec<(&str, AdapterVendor, vk::PhysicalDevice, i32)> = Vec::new();

        for physical_device in devices.iter().copied() {
            // Push the score for the device, if the device meets the minimum requirements
            if let Some(score) =
                Self::score_device(entry, instance, physical_device, surface, power_class)
            {
                let properties =
                    unsafe { instance.get_physical_device_properties(physical_device) };

                let name = unsafe {
                    CStr::from_ptr(properties.device_name.as_ptr())
                        .to_str()
                        .unwrap()
                };

                let vendor = pci_id_to_vendor(properties.vendor_id);

                scores.push((name, vendor, physical_device, score));
            }
        }

        scores
            .iter()
            .max_by_key(|v| &v.3)
            .map(|v| (v.0.to_owned(), v.1, v.2))
    }

    #[allow(clippy::too_many_arguments)]
    fn score_device<T>(
        entry: &erupt::CustomEntryLoader<T>,
        instance: &erupt::InstanceLoader,
        physical_device: vk::PhysicalDevice,
        surface: Option<vk::SurfaceKHR>,
        power_class: AdapterPowerClass,
    ) -> Option<i32> {
        use erupt::extensions::*;

        let (properties, extensions) = unsafe {
            let properties = instance.get_physical_device_properties(physical_device);
            let extensions = instance
                .enumerate_device_extension_properties(physical_device, None, None)
                .result()
                .unwrap_or_default();
            (properties, extensions)
        };

        {
            let vendor = pci_id_to_vendor(properties.vendor_id);

            let name = unsafe {
                CStr::from_ptr(properties.device_name.as_ptr())
                    .to_str()
                    .unwrap()
            };

            log::trace!("=====================");
            log::trace!("Considering Device: ");
            log::trace!("Vendor : {vendor}");
            log::trace!("Name   : {name}");
        }

        let mut score = 0i32;

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
            let (_surface_capabilities, surface_formats, present_modes) =
                Self::get_device_surface_support(instance, physical_device, surface).ok()?;

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

        unsafe {
            let supported = Self::check_device_supports_profile(
                entry,
                instance,
                physical_device,
                PROFILE_NAME,
                PROFILE_SPEC,
            )?;

            if !supported {
                log::trace!(
                    "Device doesn't support required profile '{}:{}'",
                    PROFILE_NAME,
                    PROFILE_SPEC
                );
                return None;
            }
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

    pub(crate) unsafe fn check_device_supports_profile<T>(
        entry: &erupt::CustomEntryLoader<T>,
        instance: &erupt::InstanceLoader,
        physical_device: vk::PhysicalDevice,
        profile_name: &str,
        spec_version: u32,
    ) -> Option<bool> {
        let profile = profile_props_from_loaders(entry, Some(instance), profile_name, spec_version);

        let mut supported = Default::default();
        let result = vpGetPhysicalDeviceProfileSupport(
            instance.handle,
            physical_device,
            &profile,
            &mut supported,
        );

        if result.0.is_negative() {
            log::trace!("Call to vpGetPhysicalDeviceProfileSupport failed");
            None
        } else {
            Some(supported != 0)
        }
    }

    pub(crate) fn get_device_surface_support(
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
        Context::select_device(
            &self.entry_loader,
            &self.instance_loader,
            surface,
            options.power_class,
        )
        .map(|(name, vendor, physical_device)| {
            let adapter = AnyArc::new_cyclic(move |v| Adapter {
                this: v.clone(),
                context: self._this.upgrade().unwrap(),
                name,
                vendor,
                physical_device,
            });
            AnyArc::map::<dyn IAdapter, _>(adapter, |v| v)
        })
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
