//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use erupt::extensions::khr_surface::SurfaceKHR;
use erupt::utils::VulkanResult;
use erupt::vk1_0::AllocationCallbacks;
use erupt::InstanceLoader;
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

///
/// Loads the functions for the extension required to use a surface with the WSI determined from the
/// window handle provided
///
pub unsafe fn load_surface_functions(
    instance_loader: &mut InstanceLoader,
    window: &impl HasRawWindowHandle,
) {
    instance_loader
        .load_khr_surface()
        .expect("Failed to load VK_KHR_surface");
    match window.raw_window_handle() {
        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd"
        ))]
        RawWindowHandle::Wayland(_) => {
            instance_loader
                .load_khr_wayland_surface()
                .expect("Failed to load Wayland WSI functions");
        }

        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd"
        ))]
        RawWindowHandle::Xlib(_) => {
            instance_loader
                .load_khr_xlib_surface()
                .expect("Failed to load Xlib WSI functions");
        }

        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd"
        ))]
        RawWindowHandle::Xcb(_) => {
            instance_loader
                .load_khr_xcb_surface()
                .expect("Failed to load XCB WSI functions");
        }

        #[cfg(any(target_os = "android"))]
        RawWindowHandle::Android(_) => {
            instance_loader
                .load_khr_android_surface()
                .expect("Failed to load Android WSI functions");
        }

        #[cfg(any(target_os = "macos"))]
        RawWindowHandle::MacOS(_) => {
            instance_loader
                .load_mvk_macos_surface()
                .expect("Failed to load MacOS WSI functions");
        }

        #[cfg(any(target_os = "ios"))]
        RawWindowHandle::IOS(_) => {
            instance
                .load_mvk_ios_surface()
                .expect("Failed to load IOS WSI functions");
        }

        #[cfg(target_os = "windows")]
        RawWindowHandle::Windows(_) => {
            instance_loader
                .load_khr_win32_surface()
                .expect("Failed to load Win32 WSI functions");
        }

        _ => panic!("Unsupported WSI type"),
    }
}

///
/// Creates the surface for the given window handle using the required WSI functions
///
pub unsafe fn create_surface(
    instance_loader: &InstanceLoader,
    window: &impl HasRawWindowHandle,
    allocation_callbacks: Option<&AllocationCallbacks>,
) -> VulkanResult<SurfaceKHR> {
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

            instance_loader.create_wayland_surface_khr(&create_info, allocation_callbacks, None)
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

            instance_loader.create_xlib_surface_khr(&create_info, allocation_callbacks, None)
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

            instance_loader.create_xcb_surface_khr(&create_info, allocation_callbacks, None)
        }

        #[cfg(any(target_os = "android"))]
        RawWindowHandle::Android(handle) => {
            use erupt::extensions::khr_android_surface::*;

            let create_info = AndroidSurfaceCreateInfoKHR {
                window: handle.a_native_window as _,
                ..Default::default()
            };

            instance_loader.create_android_surface_khr(&create_info, allocation_callbacks, None)
        }

        #[cfg(any(target_os = "macos"))]
        RawWindowHandle::MacOS(handle) => {
            use erupt::extensions::mvk_macos_surface::*;

            let create_info = MacOSSurfaceCreateInfoMVK {
                p_view: &*handle.ns_view,
                ..Default::default()
            };

            instance_loader.create_mac_os_surface_mvk(&create_info, allocation_callbacks, None)
        }

        #[cfg(any(target_os = "ios"))]
        RawWindowHandle::IOS(handle) => {
            use erupt::extensions::mvk_ios_surface::*;

            let create_info = IOSSurfaceCreateInfoMVK {
                p_view: &*handle.ui_view,
                ..Default::default()
            };

            instance_loader.create_ios_surface_mvk(&create_info, allocation_callbacks, None)
        }

        #[cfg(target_os = "windows")]
        RawWindowHandle::Windows(handle) => {
            use erupt::extensions::khr_win32_surface::*;

            let create_info = Win32SurfaceCreateInfoKHR {
                hinstance: handle.hinstance,
                hwnd: handle.hwnd,
                ..Default::default()
            };

            instance_loader.create_win32_surface_khr(&create_info, allocation_callbacks, None)
        }

        _ => panic!("Unsupported WSI type"),
    }
}
