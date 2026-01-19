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

use std::fmt::Display;

use crate::generated;

/// Enumeration of all supported platforms
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum Platform {
    WindowsGNU,
    WindowsMSVC,
    Linux,
    MacOS,
    IOS,
    Unknown,
}

impl Platform {
    /// Returns the platform the host application was compiled for.
    pub const fn host() -> Self {
        generated::PLATFORM
    }

    /// Deduce the platform for a given Rust target triple.
    #[inline]
    pub fn from_triple(triple: &str) -> Platform {
        let target = triple;
        if target.contains("pc-windows") {
            if target.contains("msvc") {
                Platform::WindowsMSVC
            } else if target.contains("gnu") {
                Platform::WindowsGNU
            } else {
                Platform::Unknown
            }
        } else if target.contains("linux") {
            Platform::Linux
        } else if target.contains("apple-darwin") {
            Platform::MacOS
        } else if target.contains("apple-ios") {
            Platform::IOS
        } else {
            Platform::Unknown
        }
    }

    /// Returns the target platform that we're currently building a rust crate for.
    ///
    /// When called inside a `build.rs` script this will yield the target platform for the
    /// current build. This does _not_ return the platform for the build machine itself. This
    /// returns the platform that the compiled output is being built for.
    ///
    /// Under cross compilation [`Platform::host`] and [`Platform::build_target`] will _not_
    /// match inside a build script.
    ///
    /// # Build Script
    ///
    /// This is only sane to use within the `build.rs` script. Use outside of build script is likely
    /// to panic, but may return if someone defines the appropriate env vars to mimic how cargo
    /// invokes build scripts.
    #[inline(always)]
    pub fn build_target() -> Platform {
        let target = std::env::var("TARGET").unwrap();
        Self::from_triple(&target)
    }

    pub const fn name(self) -> &'static str {
        match self {
            Platform::WindowsGNU => "windows-gnu",
            Platform::WindowsMSVC => "windows-msvc",
            Platform::Linux => "linux",
            Platform::MacOS => "macos",
            Platform::IOS => "ios",
            Platform::Unknown => "unknown",
        }
    }

    pub const fn pretty_name(self) -> &'static str {
        match self {
            Platform::WindowsGNU => "Windows GNU",
            Platform::WindowsMSVC => "Windows MSVC",
            Platform::Linux => "Linux",
            Platform::MacOS => "macOS",
            Platform::IOS => "iOS",
            Platform::Unknown => "Unknown",
        }
    }

    /// Is this platform any of the windows or universal windows platforms
    pub const fn is_windows(self) -> bool {
        matches!(self, Platform::WindowsMSVC | Platform::WindowsGNU)
    }

    pub const fn is_linux(self) -> bool {
        matches!(self, Platform::Linux)
    }

    pub const fn is_apple(self) -> bool {
        matches!(self, Platform::MacOS | Platform::IOS)
    }

    pub const fn is_macos(self) -> bool {
        matches!(self, Platform::MacOS)
    }

    pub const fn is_ios(self) -> bool {
        matches!(self, Platform::IOS)
    }

    pub const fn is_msvc(self) -> bool {
        matches!(self, Platform::WindowsMSVC)
    }

    pub const fn is_gnu(self) -> bool {
        matches!(self, Platform::WindowsGNU)
    }

    pub const fn is_unknown(self) -> bool {
        matches!(self, Platform::Unknown)
    }
}

impl Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.pretty_name())
    }
}
