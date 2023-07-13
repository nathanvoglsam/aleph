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

use crate::env::project_root;
use aleph_target::build::{target_architecture, target_platform};
use aleph_target::{Architecture, Platform};
use anyhow::anyhow;
use std::io::{Read, Seek};
use std::path::{Path, PathBuf};
use zip::ZipArchive;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Target {
    pub arch: Architecture,
    pub platform: BuildPlatform,
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum BuildPlatform {
    Windows,
    MacOS,
    Linux,
    UWP,
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
            "uwp" => Some(BuildPlatform::UWP),
            "android" => Some(BuildPlatform::Android),
            "native" => Some(Self::native()),
            _ => None,
        }
    }

    /// Returns the current platform the aleph tool is running on as a [BuildPlatform] variant.
    pub fn native() -> BuildPlatform {
        target_platform().into()
    }

    pub fn name(&self) -> &'static str {
        match self {
            BuildPlatform::Windows => "windows",
            BuildPlatform::MacOS => "macos",
            BuildPlatform::Linux => "linux",
            BuildPlatform::UWP => "uwp",
            BuildPlatform::Android => "android",
        }
    }

    /// Returns whether the given platform is a valid platform for the aleph tool to be running
    /// on.
    pub fn is_valid_native_platform(&self) -> bool {
        match self {
            BuildPlatform::Windows => true,
            BuildPlatform::MacOS => true,
            BuildPlatform::Linux => true,
            BuildPlatform::UWP => false,
            BuildPlatform::Android => false,
        }
    }
}

impl From<Platform> for BuildPlatform {
    fn from(value: Platform) -> Self {
        match value {
            Platform::UniversalWindowsGNU => BuildPlatform::UWP,
            Platform::UniversalWindowsMSVC => BuildPlatform::UWP,
            Platform::WindowsGNU => BuildPlatform::Windows,
            Platform::WindowsMSVC => BuildPlatform::Windows,
            Platform::Linux => BuildPlatform::Linux,
            Platform::Android => BuildPlatform::Android,
            Platform::MacOS => BuildPlatform::MacOS,
            Platform::Unknown => panic!("Unknown platform"),
        }
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

pub fn find_project_file<A: AsRef<Path>>(path: A) -> std::io::Result<PathBuf> {
    let file = Path::new("aleph-project.toml");
    find_file_in_parents_of(path, file)
}

pub fn find_file_in_parents_of<A: AsRef<Path>, B: AsRef<Path>>(
    path: A,
    file: B,
) -> std::io::Result<PathBuf> {
    let path = path.as_ref();
    let file = file.as_ref();

    let mut current = path.to_path_buf();
    if !current.is_dir() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Path \"{:?}\" does not point to a directory", path),
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
            "Failed to find \"{:?}\" in any parents of the \"{:?}\"",
            file, path,
        ),
    ))
}

pub fn extract_zip<R: Seek + Read>(
    archive: &mut ZipArchive<R>,
    target_dir: Option<&Path>,
) -> anyhow::Result<()> {
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
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
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }
    Ok(())
}

pub fn get_cargo_metadata() -> anyhow::Result<cargo_metadata::Metadata> {
    let mut cmd = cargo_metadata::MetadataCommand::new();
    cmd.manifest_path(project_root()?.join("Cargo.toml"));

    let metadata = cmd.exec()?;

    Ok(metadata)
}

pub fn find_crate_and_target<'a>(
    metadata: &'a cargo_metadata::Metadata,
    crate_name: &str,
    target_type: Option<&str>,
) -> anyhow::Result<(
    &'a cargo_metadata::Package,
    Option<&'a cargo_metadata::Target>,
)> {
    let package = metadata
        .packages
        .iter()
        .find(|v| v.name.as_str() == crate_name)
        .ok_or(anyhow!(
            "Failed to find crate \"{crate_name}\" in the cargo workspace"
        ))?;

    if let Some(target_type) = target_type {
        let library_target = package
            .targets
            .iter()
            .find(|v| v.crate_types.iter().any(|v| v.as_str() == "cdylib"))
            .ok_or(anyhow!(
                "Package \"{}\"has no \"{}\" target",
                &package.name,
                target_type
            ))?;
        Ok((package, Some(library_target)))
    } else {
        Ok((package, None))
    }
}
