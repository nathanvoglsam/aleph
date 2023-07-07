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

/// An enumeration of all the build profiles we support for aleph engine.
///
/// # Profiles
///
/// Currently we support 3 profiles
///
/// - [Profile::Debug]
///     - Just the default rust 'dev' profile. No optimizations, debug symbols on and all the debug
///       checks enabled.
/// - [Profile::Release]
///     - Just the default rust 'release' profile. Full optimizations, no debug symbols and debug
///       assertions off.
/// - [Profile::Retail]
///     - A custom profile designed for creating the most optimal and compact build we can. It's
///       based on release but enables full LTO, codegen-units = 1, no incremental build,
///       panic=abort and will strip the binary. Builds are slow, but should be the fastest running
///       build. Not intended for regular development, use release.
///
/// # Warning
///
/// This will not be made available as a build time utility, like [crate::Architecture], as there's
/// no rust native way to detect the profile. Trying to depend the specific profile violates rust
/// convention to not couple to the profiles with `#if RELEASE` or `#if DEBUG` style checks, but to
/// instead depend on feature flags like `cfg!(debug_assertions)`. This prevents code from being
/// coupled to its build environment (hello C++). Downside is that sometimes we need more
/// granularity than 'debug' and 'release', we need some stuff in between like 'fully optimized but
/// with debug checks' or 'super duper slow very optimized build with LTO turned up to max'.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum Profile {
    /// The default 'dev' cargo build profile.
    Debug,

    /// The default 'release' cargo build profile.
    Release,

    /// Aleph's custom 'retail' cargo build profile with aggressive LTO and optimization settings.
    Retail,
}

impl Profile {
    #[inline]
    pub fn name(&self) -> &'static str {
        match self {
            Profile::Debug => "debug",
            Profile::Release => "release",
            Profile::Retail => "retail",
        }
    }

    #[inline]
    pub fn pretty_name(&self) -> &'static str {
        match self {
            Profile::Debug => "Debug",
            Profile::Release => "Release",
            Profile::Retail => "Retail",
        }
    }
}
