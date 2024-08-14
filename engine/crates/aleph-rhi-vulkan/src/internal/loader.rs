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

pub const fn platform_library_name() -> &'static str {
    #[cfg(windows)]
    const LIB_PATH: &str = "vulkan-1.dll";

    #[cfg(all(
        unix,
        not(any(target_os = "macos", target_os = "ios", target_os = "android"))
    ))]
    const LIB_PATH: &str = "libvulkan.so.1";

    #[cfg(target_os = "android")]
    const LIB_PATH: &str = "libvulkan.so";

    #[cfg(any(target_os = "macos", target_os = "ios"))]
    const LIB_PATH: &str = "libvulkan.dylib";

    LIB_PATH
}

#[cfg(not(target_os = "ios"))]
pub type LibraryType = libloading::Library;

#[cfg(target_os = "ios")]
pub type LibraryType = ();

#[cfg(not(target_os = "ios"))]
pub unsafe fn load() -> Option<(LibraryType, ash::Entry)> {
    unsafe fn load_from(path: impl AsRef<std::ffi::OsStr>) -> Option<(LibraryType, ash::Entry)> {
        let lib = LibraryType::new(path).ok()?;

        let static_fn = ash::StaticFn::load_checked(|name| {
            lib.get(name.to_bytes_with_nul())
                .map(|symbol| *symbol)
                .unwrap_or(core::ptr::null_mut())
        })
        .ok()?;

        Some((lib, ash::Entry::from_static_fn(static_fn)))
    }

    load_from(platform_library_name())
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
