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

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Platform {
    WindowsGNU,
    WindowsMSVC,
    Linux,
    Android,
}

impl Platform {
    pub fn print_host_cargo_cfg(self) {
        match self {
            Platform::WindowsGNU => {
                println!("cargo:rustc-cfg=ALEPH_BUILD_PLATFORM_TARGET_is_gnu");
                println!("cargo:rustc-cfg=ALEPH_BUILD_PLATFORM_TARGET_is_windows");
            }
            Platform::WindowsMSVC => {
                println!("cargo:rustc-cfg=ALEPH_BUILD_PLATFORM_TARGET_is_msvc");
                println!("cargo:rustc-cfg=ALEPH_BUILD_PLATFORM_TARGET_is_windows");
            }
            Platform::Linux => {
                println!("cargo:rustc-cfg=ALEPH_BUILD_PLATFORM_TARGET_is_linux");
            }
            Platform::Android => {
                println!("cargo:rustc-cfg=ALEPH_BUILD_PLATFORM_TARGET_is_android");
            }
        }
    }

    pub fn print_target_cargo_cfg(self) {
        match self {
            Platform::WindowsGNU => {
                println!("cargo:rustc-cfg=ALEPH_BUILD_PLATFORM_HOST_is_gnu");
                println!("cargo:rustc-cfg=ALEPH_BUILD_PLATFORM_HOST_is_windows");
            }
            Platform::WindowsMSVC => {
                println!("cargo:rustc-cfg=ALEPH_BUILD_PLATFORM_HOST_is_msvc");
                println!("cargo:rustc-cfg=ALEPH_BUILD_PLATFORM_HOST_is_windows");
            }
            Platform::Linux => {
                println!("cargo:rustc-cfg=ALEPH_BUILD_PLATFORM_HOST_is_linux");
            }
            Platform::Android => {
                println!("cargo:rustc-cfg=ALEPH_BUILD_PLATFORM_HOST_is_android");
            }
        }
    }

    #[inline]
    pub fn name(self) -> &'static str {
        match self {
            Platform::WindowsGNU => "gnu",
            Platform::WindowsMSVC => "msvc",
            Platform::Linux => "linux",
            Platform::Android => "android",
        }
    }

    #[inline]
    pub fn pretty_name(self) -> &'static str {
        match self {
            Platform::WindowsGNU => "Windows GNU",
            Platform::WindowsMSVC => "Windows MSVC",
            Platform::Linux => "Linux",
            Platform::Android => "Android",
        }
    }

    #[inline]
    pub fn is_windows(self) -> bool {
        self == Platform::WindowsMSVC || self == Platform::WindowsGNU
    }

    #[inline]
    pub fn is_linux(self) -> bool {
        self == Platform::Linux
    }

    #[inline]
    pub fn is_msvc(self) -> bool {
        self == Platform::WindowsMSVC
    }

    #[inline]
    pub fn is_gnu(self) -> bool {
        self == Platform::WindowsGNU
    }

    #[inline]
    pub fn is_android(self) -> bool {
        self == Platform::Android
    }
}

#[inline]
pub fn get_platform_from(triple: &str) -> Platform {
    let target = triple;
    if target.contains("pc-windows") {
        if target.contains("msvc") {
            Platform::WindowsMSVC
        } else if target.contains("gnu") {
            Platform::WindowsGNU
        } else {
            panic!("Unsupported Platform")
        }
    } else if target.contains("android") {
        Platform::Android
    } else if target.contains("linux") {
        Platform::Linux
    } else {
        panic!("Unsupported Platform")
    }
}
