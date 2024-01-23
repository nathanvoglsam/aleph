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

///
/// Enumeration of all supported build profiles
///
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum BuildType {
    Development,
    Retail,
}

impl BuildType {
    pub const fn get_as_target() -> BuildType {
        if cfg!(aleph_target_build_type = "retail") {
            BuildType::Retail
        } else {
            BuildType::Development
        }
    }

    /// Utility function that will output the build-script configuration to stdout that is used for
    /// detecting the build type in the 'crate' side of the library.
    pub fn print_target_cargo_cfg(self) {
        println!(
            "cargo:rustc-cfg=aleph_target_build_type=\"{}\"",
            self.name()
        );
    }

    ///
    /// Get the build type name
    ///
    pub const fn name(self) -> &'static str {
        match self {
            BuildType::Development => "development",
            BuildType::Retail => "retail",
        }
    }

    pub const fn pretty_name(self) -> &'static str {
        match self {
            BuildType::Development => "Development",
            BuildType::Retail => "Retail",
        }
    }
}

impl Display for BuildType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.pretty_name())
    }
}
