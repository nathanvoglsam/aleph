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

/// Enumeration of all supported build types
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum BuildType {
    Development,
    Retail,
}

impl BuildType {
    /// Returns the build type the host application was compiled with.
    pub const fn host() -> Self {
        if cfg!(feature = "development-build") {
            BuildType::Development
        } else {
            BuildType::Retail
        }
    }

    /// Get the build type name. This is a stable, file-system friendly name that could be used in
    /// a path segment.
    pub const fn name(self) -> &'static str {
        match self {
            BuildType::Development => "development",
            BuildType::Retail => "retail",
        }
    }

    /// Get the build type name as a 'pretty' string intended for display to a user in an interface.
    pub const fn pretty_name(self) -> &'static str {
        match self {
            BuildType::Development => "Development",
            BuildType::Retail => "Retail",
        }
    }

    pub const fn is_development(&self) -> bool {
        matches!(self, BuildType::Development)
    }

    pub const fn is_retail(&self) -> bool {
        matches!(self, BuildType::Retail)
    }
}

impl Display for BuildType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.pretty_name())
    }
}
