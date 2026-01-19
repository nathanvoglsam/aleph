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

use std::fmt::{Display, Formatter};

use aleph_alloc::Blink;
use aleph_target::{Architecture, Platform};
use camino::{Utf8Path, Utf8PathBuf};

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Target {
    pub arch: Architecture,
    pub platform: BuildPlatform,
}

impl Target {
    // pub const fn new(arch: Architecture, platform: BuildPlatform) -> Self {
    //     Self { arch, platform }
    // }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum BuildPlatform {
    Windows,
    MacOS,
    Linux,
    IOS,
}

impl BuildPlatform {
    /// Creates a [`BuildPlatform`] variant from the given string. Internally performs a 'to_lower'
    /// on the input string.
    pub fn from_arg(name: &str) -> Option<BuildPlatform> {
        let name = name.to_lowercase();
        match name.as_str() {
            "windows" => Some(BuildPlatform::Windows),
            "macos" => Some(BuildPlatform::MacOS),
            "linux" => Some(BuildPlatform::Linux),
            "ios" => Some(BuildPlatform::IOS),
            "native" => Some(Self::native()),
            _ => None,
        }
    }

    /// Returns the current platform the aleph tool is running on as a [`BuildPlatform`] variant.
    pub fn native() -> BuildPlatform {
        Platform::host().into()
    }

    pub const fn name(&self) -> &'static str {
        match self {
            BuildPlatform::Windows => "windows",
            BuildPlatform::MacOS => "macos",
            BuildPlatform::Linux => "linux",
            BuildPlatform::IOS => "ios",
        }
    }

    /// Returns whether the given platform is a valid platform for the aleph tool to be running
    /// on.
    pub const fn is_valid_native_platform(&self) -> bool {
        match self {
            BuildPlatform::Windows => true,
            BuildPlatform::MacOS => true,
            BuildPlatform::Linux => true,
            BuildPlatform::IOS => false,
        }
    }
}

impl From<Platform> for BuildPlatform {
    fn from(value: Platform) -> Self {
        match value {
            Platform::WindowsGNU => BuildPlatform::Windows,
            Platform::WindowsMSVC => BuildPlatform::Windows,
            Platform::Linux => BuildPlatform::Linux,
            Platform::MacOS => BuildPlatform::MacOS,
            Platform::IOS => BuildPlatform::IOS,
            Platform::Unknown => panic!("Unknown platform"),
        }
    }
}

impl Display for BuildPlatform {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

pub fn architecture_from_arg(arg: &str) -> Option<Architecture> {
    let arg = arg.to_lowercase();
    if arg == "native" {
        Some(Architecture::host())
    } else {
        Architecture::from_name_opt(&arg)
    }
}

pub fn find_project_file<A: AsRef<Utf8Path>>(path: A) -> std::io::Result<Utf8PathBuf> {
    let file = Utf8Path::new("aleph-project.toml");
    find_file_in_parents_of(path, file)
}

pub fn find_file_in_parents_of<A: AsRef<Utf8Path>, B: AsRef<Utf8Path>>(
    path: A,
    file: B,
) -> std::io::Result<Utf8PathBuf> {
    let path = path.as_ref();
    let file = file.as_ref();

    let mut current = path.to_path_buf();
    if !current.is_dir() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Path \"{}\" does not point to a directory", path),
        ));
    }
    assert!(current.is_dir());

    loop {
        let candidate = current.join(file);
        if candidate.exists() {
            return Ok(candidate);
        }
        if !current.pop() {
            break;
        }
    }
    Err(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        format!(
            "Failed to find \"{}\" in any parents of the \"{}\"",
            file, path,
        ),
    ))
}

pub fn resolve_absolute_or_root_relative_path<P: AsRef<Utf8Path>>(
    project_root: &Utf8Path,
    v: P,
) -> Utf8PathBuf {
    let v = v.as_ref();
    if v.is_absolute() {
        v.to_path_buf()
    } else {
        project_root.join(v)
    }
}

pub mod ninja {
    use std::borrow::Cow;

    use camino::Utf8Path;

    use crate::utils::dunce_utf8;

    /// Prepares the given path ready to be used in a ninja build statement
    pub fn prepare_path_for_build_statement(path: &Utf8Path) -> Cow<'_, str> {
        if cfg!(windows) {
            // UNC = sadness for ninja
            let path = dunce_utf8::simplified(path).as_str();

            // The drive letter gets incorrectly parsed as the end of the build output so we have to
            // escape it. /shrug
            let string = path.replace(':', "$:");

            Cow::Owned(string)
        } else {
            Cow::Borrowed(path.as_str())
        }
    }
}

pub mod dunce_utf8 {
    use camino::Utf8Path;

    /// A wrapper over [dunce::simplified] that takes a [Utf8Path]
    pub fn simplified(path: &Utf8Path) -> &Utf8Path {
        let simplified = dunce::simplified(path.as_std_path());
        let path = unsafe {
            // Safety: simplified can strictly only return a subset of the input path which is
            //         guaranteed to be utf8. The subset will respect utf8 so the resulting
            //         simplified path is also guaranteed to be utf8.
            Utf8Path::from_path(simplified).unwrap_unchecked()
        };
        path
    }
}

pub trait BumpExt {
    fn alloc_utf8_path(&self, v: &Utf8Path) -> &Utf8Path;
}

impl BumpExt for Blink {
    fn alloc_utf8_path(&self, v: &Utf8Path) -> &Utf8Path {
        let v = self.copy_str(v.as_str());
        Utf8Path::new(v)
    }
}

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
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "debug" => Some(Profile::Debug),
            "release" => Some(Profile::Release),
            "retail" => Some(Profile::Retail),
            _ => None,
        }
    }
}
