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

/// Enumeration of all supported architectures
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum Architecture {
    X8664,
    AARCH64,
    Unknown,
}

impl Architecture {
    /// Returns the architecture the host application was compiled with.
    pub const fn host() -> Self {
        generated::ARCH
    }

    /// Deduce the architecture for a given Rust target triple.
    #[inline]
    pub fn from_triple(triple: &str) -> Self {
        let target = triple;

        if target.contains("x86_64") {
            Architecture::X8664
        } else if target.contains("aarch64") {
            Architecture::AARCH64
        } else {
            Architecture::Unknown
        }
    }

    /// Returns the target architecture that we're currently building a rust crate for.
    ///
    /// When called inside a `build.rs` script this will yield the target architecture for the
    /// current build. This does _not_ return the architecture for the build machine itself. This
    /// returns the architecture that the compiled output is being built for.
    ///
    /// Under cross compilation [`Architecture::host`] and [`Architecture::build_target`] will _not_
    /// match inside a build script.
    ///
    /// # Build Script
    ///
    /// This is only sane to use within the `build.rs` script. Use outside of build script is likely
    /// to panic, but may return if someone defines the appropriate env vars to mimic how cargo
    /// invokes build scripts.
    #[inline(always)]
    pub fn build_target() -> Self {
        let target = std::env::var("TARGET").unwrap();
        Self::from_triple(&target)
    }

    #[inline(always)]
    pub fn from_name(name: &str) -> Self {
        match name {
            "x86_64" => Architecture::X8664,
            "aarch64" => Architecture::AARCH64,
            _ => Architecture::Unknown,
        }
    }

    #[inline(always)]
    pub fn from_name_opt(name: &str) -> Option<Self> {
        match Self::from_name(name) {
            Architecture::X8664 => Some(Architecture::X8664),
            Architecture::AARCH64 => Some(Architecture::AARCH64),
            Architecture::Unknown => None,
        }
    }

    pub const fn name(self) -> &'static str {
        match self {
            Architecture::X8664 => "x86_64",
            Architecture::AARCH64 => "aarch64",
            Architecture::Unknown => "unknown",
        }
    }

    pub const fn is_x86_64(self) -> bool {
        matches!(self, Architecture::X8664)
    }

    pub const fn is_aarch64(self) -> bool {
        matches!(self, Architecture::AARCH64)
    }

    pub const fn is_unknown(self) -> bool {
        matches!(self, Architecture::Unknown)
    }
}

impl Display for Architecture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
