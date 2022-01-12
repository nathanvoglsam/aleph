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

use erupt::extensions::khr_surface::SurfaceKHR;
use erupt::utils::VulkanResult;
use erupt::vk1_0::AllocationCallbacks;
use erupt::InstanceLoader;
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

///
/// Creates the surface for the given window handle using the required WSI functions
///
pub unsafe fn create_surface(
    instance_loader: &InstanceLoader,
    window_handle: &impl HasRawWindowHandle,
    allocation_callbacks: Option<&AllocationCallbacks>,
) -> VulkanResult<SurfaceKHR> {
    match window_handle.raw_window_handle() {
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

            instance_loader.create_wayland_surface_khr(&create_info, allocation_callbacks)
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

            instance_loader.create_xlib_surface_khr(&create_info, allocation_callbacks)
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

            instance_loader.create_xcb_surface_khr(&create_info, allocation_callbacks)
        }

        #[cfg(any(target_os = "android"))]
        RawWindowHandle::Android(handle) => {
            use erupt::extensions::khr_android_surface::*;

            let create_info = AndroidSurfaceCreateInfoKHR {
                window: handle.a_native_window as _,
                ..Default::default()
            };

            instance_loader.create_android_surface_khr(&create_info, allocation_callbacks)
        }

        #[cfg(any(target_os = "macos"))]
        RawWindowHandle::MacOS(handle) => {
            use erupt::extensions::mvk_macos_surface::*;

            let create_info = MacOSSurfaceCreateInfoMVK {
                p_view: &*handle.ns_view,
                ..Default::default()
            };

            instance_loader.create_mac_os_surface_mvk(&create_info, allocation_callbacks)
        }

        #[cfg(any(target_os = "ios"))]
        RawWindowHandle::IOS(handle) => {
            use erupt::extensions::mvk_ios_surface::*;

            let create_info = IOSSurfaceCreateInfoMVK {
                p_view: &*handle.ui_view,
                ..Default::default()
            };

            instance_loader.create_ios_surface_mvk(&create_info, allocation_callbacks)
        }

        #[cfg(target_os = "windows")]
        RawWindowHandle::Windows(handle) => {
            use erupt::extensions::khr_win32_surface::*;

            let create_info = Win32SurfaceCreateInfoKHR {
                hinstance: handle.hinstance,
                hwnd: handle.hwnd,
                ..Default::default()
            };

            instance_loader.create_win32_surface_khr(&create_info, allocation_callbacks)
        }

        _ => panic!("Unsupported WSI type"),
    }
}
