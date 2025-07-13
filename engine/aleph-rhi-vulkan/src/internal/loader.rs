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

pub const fn platform_library_search_stack() -> &'static [&'static str] {
    if cfg!(windows) {
        // Windows
        &["vulkan-1.dll"]
    } else if cfg!(target_os = "macos") {
        // MacOS via MoltenVK, has two paths to search incase 'libvulkan.dylib' isn't in the rpath.
        // This is a workaround as the game can't find 'libvulkan.dylib' when running in some dev
        // tools (debuggers) otherwise. We use the absolute path if the first fails.
        &["libvulkan.dylib", "/usr/local/lib/libvulkan.dylib"]
    } else if cfg!(target_os = "ios") {
        // IOS doesn't fall victim to the same flaw because we're in an app bundle, and there's no
        // global Vulkan loader to find anyway.
        &["libvulkan.dylib"]
    } else if cfg!(target_os = "android") {
        // Android matches cfg!(unix) but has a different name than linux typically uses
        &["libvulkan.so"]
    } else if cfg!(unix) {
        // This covers linux and friends with a spread of possible loader library names
        &["libvulkan.so.1", "libvulkan.so"]
    } else {
        panic!("Unsupported platform!");
    }
}

#[cfg(not(target_os = "ios"))]
pub type LibraryType = libloading::Library;

#[cfg(target_os = "ios")]
pub type LibraryType = ();

#[cfg(not(target_os = "ios"))]
pub unsafe fn load() -> Option<(LibraryType, ash::Entry)> {
    unsafe {
        unsafe fn load_from(
            path: impl AsRef<std::ffi::OsStr>,
        ) -> Option<(LibraryType, ash::Entry)> {
            unsafe {
                let lib = LibraryType::new(path).ok()?;

                let static_fn = ash::StaticFn::load_checked(|name| {
                    lib.get(name.to_bytes_with_nul())
                        .map(|symbol| *symbol)
                        .unwrap_or(core::ptr::null_mut())
                })
                .ok()?;

                Some((lib, ash::Entry::from_static_fn(static_fn)))
            }
        }

        for lib in platform_library_search_stack() {
            let result = load_from(lib);
            if result.is_some() {
                return result;
            }
        }
        None
    }
}

#[cfg(target_os = "ios")]
#[link(kind = "framework", name = "vulkan")]
extern "system" {
    fn vkGetInstanceProcAddr(
        instance: ash::vk::Instance,
        name: *const core::ffi::c_char,
    ) -> ash::vk::PFN_vkVoidFunction;
}

#[cfg(target_os = "ios")]
pub unsafe fn load() -> Option<(LibraryType, ash::Entry)> {
    let entry = ash::Entry::from_static_fn(ash::StaticFn {
        get_instance_proc_addr: vkGetInstanceProcAddr,
    });

    Some(((), entry))
}
