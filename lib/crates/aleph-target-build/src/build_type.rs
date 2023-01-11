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
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum BuildType {
    Release,
    Debug,
    Unknown,
}

impl BuildType {
    /// Utility function that will output the build-script configuration to stdout that is used for
    /// detecting the build type in the 'crate' side of the library.
    pub fn print_target_cargo_cfg(self) {
        println!("cargo:rustc-cfg=target_profile=\"{}\"", self.name());
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
            BuildType::Unknown => "Unknown",
        }
    }

    pub const fn is_optimized(self) -> bool {
        match self {
            BuildType::Release | BuildType::Unknown => {
                true
            }
            BuildType::Debug => false,
        }
    }

    pub const fn has_debug_symbols(self) -> bool {
        match self {
            BuildType::Debug => true,
            BuildType::Release | BuildType::Unknown => false,
        }
    }
}

#[inline]
pub fn get_build_type_from(profile: &str) -> BuildType {
    match profile {
        "release" => BuildType::Release,
        "debug" => BuildType::Debug,
        _ => BuildType::Unknown,
    }
}
