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

use std::ffi::CStr;
use std::ptr::NonNull;
use std::sync::LazyLock;

use ash::vk;

const fn platform_library_search_stack() -> &'static [&'static CStr] {
    if cfg!(windows) {
        // Windows
        &[c"vulkan-1.dll"]
    } else if cfg!(target_os = "macos") {
        // MacOS via MoltenVK, has two paths to search incase 'libvulkan.dylib' isn't in the rpath.
        // This is a workaround as the game can't find 'libvulkan.dylib' when running in some dev
        // tools (debuggers) otherwise. We use the absolute path if the first fails.
        &[c"libvulkan.dylib", c"/usr/local/lib/libvulkan.dylib"]
    } else if cfg!(target_os = "ios") {
        // IOS doesn't fall victim to the same flaw because we're in an app bundle, and there's no
        // global Vulkan loader to find anyway.
        &[c"libvulkan.dylib"]
    } else if cfg!(unix) {
        // This covers linux and friends with a spread of possible loader library names
        &[c"libvulkan.so.1", c"libvulkan.so"]
    } else {
        panic!("Unsupported platform!");
    }
}

#[repr(transparent)]
pub struct LibraryType(pub NonNull<sdl3_sys::loadso::SDL_SharedObject>);

unsafe impl Send for LibraryType {}
unsafe impl Sync for LibraryType {}

pub static VULKAN_LIBRARY: LazyLock<Option<LibraryType>> = LazyLock::new(|| unsafe {
    for lib in platform_library_search_stack() {
        let object = sdl3_sys::loadso::SDL_LoadObject(lib.as_ptr());
        let object = NonNull::new(object);
        if let Some(object) = object {
            return Some(LibraryType(object));
        }
    }
    None
});

pub static MVK_LIBRARY: LazyLock<Option<LibraryType>> = LazyLock::new(|| unsafe {
    let object = sdl3_sys::loadso::SDL_LoadObject(c"libMoltenVK.dylib".as_ptr());
    let object = NonNull::new(object);
    if let Some(object) = object {
        return Some(LibraryType(object));
    }
    None
});

pub unsafe fn load() -> Option<ash::Entry> {
    if cfg!(target_os = "ios") {
        let static_fn = static_instance_proc_addr_fn();
        unsafe {
            Some(ash::Entry::from_static_fn(ash::StaticFn {
                get_instance_proc_addr: static_fn,
            }))
        }
    } else {
        if let Some(lib) = &*VULKAN_LIBRARY {
            let static_fn = ash::StaticFn::load_checked(|name| unsafe {
                let v = sdl3_sys::loadso::SDL_LoadFunction(lib.0.as_ptr(), name.as_ptr());
                let v = v.map(|v| std::mem::transmute::<_, *const std::ffi::c_void>(v));
                v.unwrap_or(core::ptr::null_mut())
            });
            let static_fn = static_fn.ok()?;

            unsafe { Some(ash::Entry::from_static_fn(static_fn)) }
        } else {
            None
        }
    }
}

fn static_instance_proc_addr_fn() -> vk::PFN_vkGetInstanceProcAddr {
    #[cfg(target_os = "ios")]
    {
        #[link(kind = "framework", name = "vulkan")]
        unsafe extern "system" {
            fn vkGetInstanceProcAddr(
                instance: vk::Instance,
                name: *const core::ffi::c_char,
            ) -> vk::PFN_vkVoidFunction;
        }
        vkGetInstanceProcAddr
    }

    #[cfg(not(target_os = "ios"))]
    {
        #[allow(non_snake_case)]
        extern "system" fn DUMMY_vkGetInstanceProcAddr(
            _instance: vk::Instance,
            _name: *const core::ffi::c_char,
        ) -> vk::PFN_vkVoidFunction {
            None
        }
        DUMMY_vkGetInstanceProcAddr
    }
}
