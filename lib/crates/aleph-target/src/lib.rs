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

extern crate aleph_target_build as aleph_target;

pub use aleph_target::recreate_triple;

pub use aleph_target::Architecture;
pub use aleph_target::BuildType;
pub use aleph_target::Platform;

pub use aleph_target::get_architecture_from;
pub use aleph_target::get_build_type_from;
pub use aleph_target::get_platform_from;

pub mod build {

    ///
    /// Returns the host platform. Always returns [aleph_target::Platform::Unknown]
    ///
    /// # Warning
    ///
    /// Doesn't actually work. Simply present so the API matches the 'build-script' version of the
    /// crate.
    ///
    #[inline]
    pub const fn host_platform() -> aleph_target::Platform {
        aleph_target::Platform::Unknown
    }

    ///
    /// Returns the host architecture. Always returns [aleph_target::Architecture::Unknown]
    ///
    /// # Warning
    ///
    /// Doesn't actually work. Simply present so the API matches the 'build-script' version of the
    /// crate.
    ///
    pub const fn host_architecture() -> aleph_target::Architecture {
        aleph_target::Architecture::Unknown
    }

    ///
    /// Returns the host build type.  Always returns [aleph_target::BuildType::Unknown]
    ///
    /// # Warning
    ///
    /// Doesn't actually work. Simply present so the API matches the 'build-script' version of the
    /// crate.
    ///
    pub const fn host_build_type() -> aleph_target::BuildType {
        aleph_target::BuildType::Unknown
    }

    ///
    /// Returns the target platform
    ///
    pub const fn target_platform() -> aleph_target::Platform {
        if cfg!(target_os = "windows") {
            if cfg!(target_vendor = "uwp") {
                if cfg!(target_env = "msvc") {
                    aleph_target::Platform::UniversalWindowsMSVC
                } else if cfg!(target_env = "gnu") {
                    aleph_target::Platform::UniversalWindowsGNU
                } else {
                    aleph_target::Platform::Unknown
                }
            } else if cfg!(target_env = "msvc") {
                aleph_target::Platform::WindowsMSVC
            } else if cfg!(target_env = "gnu") {
                aleph_target::Platform::WindowsGNU
            } else {
                aleph_target::Platform::Unknown
            }
        } else if cfg!(target_os = "android") {
            aleph_target::Platform::Android
        } else if cfg!(target_os = "linux") {
            aleph_target::Platform::Linux
        } else {
            aleph_target::Platform::Unknown
        }
    }

    ///
    /// Returns the target architecture
    ///
    pub const fn target_architecture() -> aleph_target::Architecture {
        if cfg!(target_arch = "x86_64") {
            aleph_target::Architecture::X8664
        } else if cfg!(target_arch = "aarch64") {
            aleph_target::Architecture::AARCH64
        } else {
            aleph_target::Architecture::Unknown
        }
    }

    ///
    /// Returns the target build type
    ///
    pub const fn target_build_type() -> aleph_target::BuildType {
        if cfg!(target_profile = "release") {
            aleph_target::BuildType::Release
        } else if cfg!(target_profile = "debug") {
            aleph_target::BuildType::Debug
        } else {
            aleph_target::BuildType::Unknown
        }
    }
}
