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
    UniversalWindowsGNU,
    UniversalWindowsMSVC,
    WindowsGNU,
    WindowsMSVC,
    Linux,
    Android,
    Unknown,
}

impl Platform {
    pub fn print_host_cargo_cfg(self) {
        match self {
            Platform::UniversalWindowsGNU => {
                println!("cargo:rustc-cfg=ALEPH_BUILD_PLATFORM_TARGET_is_gnu");
                println!("cargo:rustc-cfg=ALEPH_BUILD_PLATFORM_TARGET_is_universal_windows");
                println!("cargo:rustc-cfg=ALEPH_BUILD_PLATFORM_TARGET_is_windows");
            }
            Platform::UniversalWindowsMSVC => {
                println!("cargo:rustc-cfg=ALEPH_BUILD_PLATFORM_TARGET_is_msvc");
                println!("cargo:rustc-cfg=ALEPH_BUILD_PLATFORM_TARGET_is_universal_windows");
                println!("cargo:rustc-cfg=ALEPH_BUILD_PLATFORM_TARGET_is_windows");
            }
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
            Platform::Unknown => {
                println!("cargo:rustc-cfg=ALEPH_BUILD_PLATFORM_TARGET_is_unknown");
            }
        }
    }

    pub fn print_target_cargo_cfg(self) {
        match self {
            Platform::UniversalWindowsGNU => {
                println!("cargo:rustc-cfg=ALEPH_BUILD_PLATFORM_HOST_is_gnu");
                println!("cargo:rustc-cfg=ALEPH_BUILD_PLATFORM_HOST_is_windows");
                println!("cargo:rustc-cfg=ALEPH_BUILD_PLATFORM_HOST_is_universal_windows");
            }
            Platform::UniversalWindowsMSVC => {
                println!("cargo:rustc-cfg=ALEPH_BUILD_PLATFORM_HOST_is_msvc");
                println!("cargo:rustc-cfg=ALEPH_BUILD_PLATFORM_HOST_is_windows");
                println!("cargo:rustc-cfg=ALEPH_BUILD_PLATFORM_HOST_is_universal_windows");
            }
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
            Platform::Unknown => {
                println!("cargo:rustc-cfg=ALEPH_BUILD_PLATFORM_HOST_is_unknown");
            }
        }
    }

    pub const fn name(self) -> &'static str {
        match self {
            Platform::UniversalWindowsGNU => "uwp-gnu",
            Platform::UniversalWindowsMSVC => "uwp-msvc",
            Platform::WindowsGNU => "windows-gnu",
            Platform::WindowsMSVC => "windows-msvc",
            Platform::Linux => "linux",
            Platform::Android => "android",
            Platform::Unknown => "unknown",
        }
    }

    pub const fn pretty_name(self) -> &'static str {
        match self {
            Platform::UniversalWindowsGNU => "Universal Windows GNU",
            Platform::UniversalWindowsMSVC => "Universal Windows MSVC",
            Platform::WindowsGNU => "Windows GNU",
            Platform::WindowsMSVC => "Windows MSVC",
            Platform::Linux => "Linux",
            Platform::Android => "Android",
            Platform::Unknown => "Unknown",
        }
    }

    /// Is this platform any of the win32 (non universal) windows platforms {
    ///
    pub const fn is_win32(self) -> bool {
        match self {
            Platform::WindowsMSVC | Platform::WindowsGNU => true,
            _ => false,
        }
    }

    /// Is this platform any of the windows or universal windows platforms
    pub const fn is_windows(self) -> bool {
        match self {
            Platform::WindowsMSVC
            | Platform::WindowsGNU
            | Platform::UniversalWindowsGNU
            | Platform::UniversalWindowsMSVC => true,
            _ => false,
        }
    }

    /// Is this platform any of the universal windows platforms
    pub const fn is_uwp(self) -> bool {
        match self {
            Platform::UniversalWindowsGNU | Platform::UniversalWindowsMSVC => true,
            _ => false,
        }
    }

    pub const fn is_linux(self) -> bool {
        match self {
            Platform::Linux => true,
            _ => false,
        }
    }

    pub const fn is_msvc(self) -> bool {
        match self {
            Platform::WindowsMSVC => true,
            Platform::UniversalWindowsMSVC => true,
            _ => false,
        }
    }

    pub const fn is_gnu(self) -> bool {
        match self {
            Platform::WindowsGNU => true,
            Platform::UniversalWindowsGNU => true,
            _ => false,
        }
    }

    pub const fn is_android(self) -> bool {
        match self {
            Platform::Android => true,
            _ => false,
        }
    }

    pub const fn is_unknown(self) -> bool {
        match self {
            Platform::Unknown => true,
            _ => false,
        }
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
            Platform::Unknown
        }
    } else if target.contains("uwp-windows") {
        if target.contains("msvc") {
            Platform::UniversalWindowsMSVC
        } else if target.contains("gnu") {
            Platform::UniversalWindowsGNU
        } else {
            Platform::Unknown
        }
    } else if target.contains("android") {
        Platform::Android
    } else if target.contains("linux") {
        Platform::Linux
    } else {
        Platform::Unknown
    }
}
