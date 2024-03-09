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
use std::io::{Read, Seek};

use aleph_target::build::{target_architecture, target_platform};
use aleph_target::{Architecture, Platform};
use camino::{Utf8Path, Utf8PathBuf};
use zip::ZipArchive;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Target {
    pub arch: Architecture,
    pub platform: BuildPlatform,
}

impl Target {
    pub const fn new(arch: Architecture, platform: BuildPlatform) -> Self {
        Self { arch, platform }
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum BuildPlatform {
    Windows,
    MacOS,
    Linux,
    Uwp,
    Android,
}

impl BuildPlatform {
    /// Creates a [BuildPlatform] variant from the given string. Internally performs a 'to_lower'
    /// on the input string.
    pub fn from_arg(name: &str) -> Option<BuildPlatform> {
        let name = name.to_lowercase();
        match name.as_str() {
            "windows" => Some(BuildPlatform::Windows),
            "macos" => Some(BuildPlatform::MacOS),
            "linux" => Some(BuildPlatform::Linux),
            "uwp" => Some(BuildPlatform::Uwp),
            "android" => Some(BuildPlatform::Android),
            "native" => Some(Self::native()),
            _ => None,
        }
    }

    /// Returns the current platform the aleph tool is running on as a [BuildPlatform] variant.
    pub fn native() -> BuildPlatform {
        target_platform().into()
    }

    pub const fn name(&self) -> &'static str {
        match self {
            BuildPlatform::Windows => "windows",
            BuildPlatform::MacOS => "macos",
            BuildPlatform::Linux => "linux",
            BuildPlatform::Uwp => "uwp",
            BuildPlatform::Android => "android",
        }
    }

    /// Returns whether the given platform is a valid platform for the aleph tool to be running
    /// on.
    pub const fn is_valid_native_platform(&self) -> bool {
        match self {
            BuildPlatform::Windows => true,
            BuildPlatform::MacOS => true,
            BuildPlatform::Linux => true,
            BuildPlatform::Uwp => false,
            BuildPlatform::Android => false,
        }
    }
}

impl From<Platform> for BuildPlatform {
    fn from(value: Platform) -> Self {
        match value {
            Platform::UniversalWindowsGNU => BuildPlatform::Uwp,
            Platform::UniversalWindowsMSVC => BuildPlatform::Uwp,
            Platform::WindowsGNU => BuildPlatform::Windows,
            Platform::WindowsMSVC => BuildPlatform::Windows,
            Platform::Linux => BuildPlatform::Linux,
            Platform::Android => BuildPlatform::Android,
            Platform::MacOS => BuildPlatform::MacOS,
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
        Some(target_architecture())
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

pub fn extract_zip<R: Seek + Read>(
    archive: &mut ZipArchive<R>,
    target_dir: Option<&Utf8Path>,
) -> anyhow::Result<()> {
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = match file.enclosed_name() {
            Some(path) => {
                if let Some(v) = Utf8Path::from_path(path) {
                    v.to_owned()
                } else {
                    continue;
                }
            }
            None => continue,
        };
        let outpath = match target_dir {
            None => outpath,
            Some(v) => v.join(outpath),
        };

        if (*file.name()).ends_with('/') {
            std::fs::create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    std::fs::create_dir_all(p)?;
                }
            }
            let mut outfile = std::fs::File::create(&outpath)?;
            std::io::copy(&mut file, &mut outfile)?;
        }

        // Get and Set permissions
        #[cfg(unix)]
        {
            if let Some(mode) = file.unix_mode() {
                use std::fs;
                use std::os::unix::fs::PermissionsExt;
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }
    Ok(())
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
    pub fn prepare_path_for_build_statement(path: &Utf8Path) -> Cow<str> {
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

    /// A wrapper over [dunce::simplified] that takes a [camino::Utf8Path]
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
