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

use crate::generated;

/// Description of how the host application was compiled.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct BuildConfig {
    pub(crate) debug: bool,
    pub(crate) optimized: bool,
}

impl BuildConfig {
    /// Returns the build configuration the host application was compiled with.
    pub const fn host() -> Self {
        generated::CONFIG
    }

    /// Constructs a new [`BuildConfig`] from the given parameters.
    pub const fn new(debug: bool, optimized: bool) -> BuildConfig {
        BuildConfig { debug, optimized }
    }

    /// Returns the target build config that we're currently building a rust crate for.
    ///
    /// When called inside a `build.rs` script this will yield the target build config for the
    /// current build. This does _not_ return the build config for the build machine itself. This
    /// returns the platform that the compiled output is being built for.
    ///
    /// # Build Script
    ///
    /// This is only sane to use within the `build.rs` script. Use outside of build script is likely
    /// to panic, but may return if someone defines the appropriate env vars to mimic how cargo
    /// invokes build scripts.
    #[inline(always)]
    pub fn build_target() -> Self {
        let debug = std::env::var("DEBUG").unwrap() == "true";
        let optimized = std::env::var("OPT_LEVEL").unwrap() != "0";
        Self::new(debug, optimized)
    }

    /// Returns 'true' if the host application was compiled with debug symbols.
    pub const fn is_debug(self) -> bool {
        self.debug
    }

    /// Returns 'true' if the host application was compiled with optimizations enabled.
    pub const fn is_optimized(self) -> bool {
        self.optimized
    }
}
