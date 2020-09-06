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

///
/// Enumeration of all supported build types
///
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum BuildType {
    Release,
    Debug,
    Unknown,
}

impl BuildType {
    pub fn print_host_cargo_cfg(self) {
        match self {
            BuildType::Release => {
                println!("cargo:rustc-cfg=ALEPH_BUILD_PROFILE_HOST_is_release");
            }
            BuildType::Debug => {
                println!("cargo:rustc-cfg=ALEPH_BUILD_PROFILE_HOST_is_debug");
            }
            BuildType::Unknown => {
                println!("cargo:rustc-cfg=ALEPH_BUILD_PROFILE_HOST_is_unknown");
            }
        }
    }

    pub fn print_target_cargo_cfg(self) {
        match self {
            BuildType::Release => {
                println!("cargo:rustc-cfg=ALEPH_BUILD_PROFILE_TARGET_is_release");
            }
            BuildType::Debug => {
                println!("cargo:rustc-cfg=ALEPH_BUILD_PROFILE_TARGET_is_debug");
            }
            BuildType::Unknown => {
                println!("cargo:rustc-cfg=ALEPH_BUILD_PROFILE_TARGET_is_unknown");
            }
        }
    }

    ///
    /// Get the build type name
    ///
    pub const fn name(self) -> &'static str {
        match self {
            BuildType::Release => "release",
            BuildType::Debug => "debug",
            BuildType::Unknown => "unknown",
        }
    }

    pub const fn pretty_name(self) -> &'static str {
        match self {
            BuildType::Release => "Release",
            BuildType::Debug => "Debug",
            BuildType::Unknown => "unknown",
        }
    }

    pub const fn is_release(self) -> bool {
        match self {
            BuildType::Release => true,
            _ => false,
        }
    }

    pub const fn is_debug(self) -> bool {
        match self {
            BuildType::Debug => true,
            _ => false,
        }
    }

    pub const fn is_unknown(self) -> bool {
        match self {
            BuildType::Unknown => true,
            _ => false,
        }
    }
}

#[inline]
pub fn get_build_type_from(profile: &str) -> BuildType {
    if profile == "release" {
        BuildType::Release
    } else if profile == "debug" {
        BuildType::Debug
    } else {
        BuildType::Unknown
    }
}
