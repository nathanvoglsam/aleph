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

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum Platform {
    UniversalWindowsGNU,
    UniversalWindowsMSVC,
    WindowsGNU,
    WindowsMSVC,
    Linux,
    Android,
    MacOS,
    Unknown,
}

impl Platform {
    pub const fn name(self) -> &'static str {
        match self {
            Platform::UniversalWindowsGNU => "uwp-gnu",
            Platform::UniversalWindowsMSVC => "uwp-msvc",
            Platform::WindowsGNU => "windows-gnu",
            Platform::WindowsMSVC => "windows-msvc",
            Platform::Linux => "linux",
            Platform::Android => "android",
            Platform::MacOS => "mac-os",
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
            Platform::MacOS => "macOS",
            Platform::Unknown => "Unknown",
        }
    }

    /// Is this platform any of the win32 (non universal) windows platforms {
    ///
    pub const fn is_win32(self) -> bool {
        matches!(self, Platform::WindowsMSVC | Platform::WindowsGNU)
    }

    /// Is this platform any of the windows or universal windows platforms
    pub const fn is_windows(self) -> bool {
        matches!(
            self,
            Platform::WindowsMSVC
                | Platform::WindowsGNU
                | Platform::UniversalWindowsGNU
                | Platform::UniversalWindowsMSVC
        )
    }

    /// Is this platform any of the universal windows platforms
    pub const fn is_uwp(self) -> bool {
        matches!(
            self,
            Platform::UniversalWindowsGNU | Platform::UniversalWindowsMSVC
        )
    }

    pub const fn is_linux(self) -> bool {
        matches!(self, Platform::Linux)
    }

    pub const fn is_macos(self) -> bool {
        matches!(self, Platform::MacOS)
    }

    pub const fn is_msvc(self) -> bool {
        matches!(self, Platform::WindowsMSVC | Platform::UniversalWindowsMSVC)
    }

    pub const fn is_gnu(self) -> bool {
        matches!(self, Platform::WindowsGNU | Platform::UniversalWindowsGNU)
    }

    pub const fn is_android(self) -> bool {
        matches!(self, Platform::Android)
    }

    pub const fn is_unknown(self) -> bool {
        matches!(self, Platform::Unknown)
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
    } else if target.contains("apple-darwin") {
        Platform::MacOS
    } else {
        Platform::Unknown
    }
}
