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

use crate::utils::{find_project_file, BuildPlatform, Target};
use aleph_target::{Architecture, Profile};
use anyhow::{anyhow, Context};
use once_cell::sync::OnceCell;
use std::path::{Path, PathBuf};

#[derive(Clone)]
pub struct AlephProject {
    /// The path to the 'aleph-project.toml' file for this project
    project_file: PathBuf,

    /// The path to the directory that contains 'aleph-project.toml' for this project. Just a cached
    /// form of `project_file.parent()`
    project_root: PathBuf,

    /// The path to the '.aleph' folder for this project
    dot_aleph_path: PathBuf,

    /// Path to the android project in the '.aleph/proj' directory
    android_proj_path: PathBuf,

    /// Path to the uwp x86_64 project in the '.aleph/proj' directory
    uwp_x86_64_proj_path: PathBuf,

    /// Path to the uwp aarch64 project in the '.aleph/proj' directory
    uwp_aarch64_proj_path: PathBuf,

    /// The path to the Cargo.toml file adjacent to the aleph-project.toml
    cargo_toml_file: PathBuf,

    /// The cargo target directory
    cargo_target_dir: PathBuf,

    /// A cached copy of the collect Cargo metadata
    cargo_metadata: OnceCell<cargo_metadata::Metadata>,
}

impl AlephProject {
    pub fn new() -> anyhow::Result<Self> {
        let project_file = find_project_file(std::env::current_dir()?)
            .context("Finding aleph-project.toml in current working directory")?
            .canonicalize()
            .context("Canonicalizing aleph-project.toml path")?;
        let project_root = project_file.parent().unwrap().to_path_buf();
        let dot_aleph_path = project_root.join(".aleph");
        let cargo_toml_file = project_root.join("Cargo.toml");
        let cargo_target_dir = project_root.join("target");

        let target = Target::new(Architecture::Unknown, BuildPlatform::Android);
        let android_proj_path = Self::compute_target_project_root(&dot_aleph_path, &target)?;
        let target = Target::new(Architecture::X8664, BuildPlatform::UWP);
        let uwp_x86_64_proj_path = Self::compute_target_project_root(&dot_aleph_path, &target)?;
        let target = Target::new(Architecture::AARCH64, BuildPlatform::UWP);
        let uwp_aarch64_proj_path = Self::compute_target_project_root(&dot_aleph_path, &target)?;

        let out = Self {
            project_file,
            project_root,
            dot_aleph_path,
            android_proj_path,
            uwp_x86_64_proj_path,
            uwp_aarch64_proj_path,
            cargo_toml_file,
            cargo_target_dir,
            cargo_metadata: Default::default(),
        };

        out.ensure_core_files_and_directories()?;

        Ok(out)
    }

    /// Returns the path to the `aleph-project.toml` file for the project we're working with
    pub fn project_file(&self) -> &Path {
        &self.project_file
    }

    /// Returns the path to the folder that contains the `aleph-project.toml` file for the project
    /// we're working with
    pub fn project_root(&self) -> &Path {
        &self.project_root
    }

    // /// The path to the '.aleph' directory for the current project. The .aleph directory will be
    // /// in the same directory as the 'aleph-project.toml' file.
    // pub fn dot_aleph_path(&self) -> &Path {
    //     &self.dot_aleph_path
    // }

    /// A utility around [Self::target_project_root] that returns also ensures that the project
    /// directory exists and is a directory.
    pub fn ensure_target_project_root(&self, target: &Target) -> anyhow::Result<&Path> {
        let path = self.target_project_root(target)?;
        std::fs::create_dir_all(path)?;
        Ok(path)
    }

    /// Returns the path to the platform project for the requested target.
    ///
    /// # Warning
    ///
    /// This will only return a result for platforms that have a bundling project. Those platforms
    /// are:
    /// - Android (all architectures in one project)
    /// - UWP x86_64
    /// - UWP aarch64
    ///
    /// All other targets will return Err()
    pub fn target_project_root(&self, target: &Target) -> anyhow::Result<&Path> {
        match target.platform {
            BuildPlatform::UWP => {
                assert_ne!(target.arch, Architecture::Unknown);
                match target.arch {
                    Architecture::X8664 => Ok(&self.uwp_x86_64_proj_path),
                    Architecture::AARCH64 => Ok(&self.uwp_aarch64_proj_path),
                    Architecture::Unknown => unreachable!(),
                }
            }
            BuildPlatform::Android => Ok(&self.android_proj_path),
            _ => Err(anyhow!(
                "Platform \"{}\" does not have a target specific sub-project.",
                target.platform.name()
            )),
        }
    }

    /// Returns the path to the Cargo.toml file that the project is using, adjacent to the
    /// 'aleph-project.toml'
    pub fn cargo_toml_file(&self) -> &Path {
        &self.cargo_toml_file
    }

    /// Returns the path to the cargo target directory root, which will be adjacent to the
    /// 'aleph-project.toml' and 'Cargo.toml'
    pub fn cargo_target_dir(&self) -> &Path {
        &self.cargo_target_dir
    }

    /// Returns the './target/{target-triple}/{profile}' path for the request target + profile set.
    pub fn cargo_build_dir_for_target(
        &self,
        target: &Target,
        profile: Profile,
    ) -> anyhow::Result<PathBuf> {
        assert_ne!(target.arch, Architecture::Unknown);

        match target.platform {
            BuildPlatform::UWP => {
                let mut target_dir = self.cargo_target_dir().to_path_buf();
                target_dir.push(format!("{}-uwp-windows-msvc", target.arch.name()));
                target_dir.push(profile.name());
                Ok(target_dir)
            }
            BuildPlatform::Android => {
                let mut target_dir = self.cargo_target_dir().to_path_buf();
                target_dir.push(format!("{}-linux-android", target.arch.name()));
                target_dir.push(profile.name());
                Ok(target_dir)
            }
            _ => Err(anyhow!(
                "Platform \"{}\" does not support cargo_build_dir_for_target",
                target.platform.name()
            )),
        }
    }

    /// Attempts to load cargo metadata for the cargo workspace/project based on an assumed
    /// 'Cargo.toml' adjacent to 'aleph-project.toml'. Caches the result after the first load.
    ///
    /// # Warning
    ///
    /// Will _not_ re-query after the first call. Create a new [AlephProject] to re-query.
    pub fn get_cargo_metadata(&self) -> anyhow::Result<&cargo_metadata::Metadata> {
        self.cargo_metadata.get_or_try_init(|| {
            let mut cmd = cargo_metadata::MetadataCommand::new();
            cmd.manifest_path(self.cargo_toml_file());

            let metadata = cmd.exec()?;

            Ok(metadata)
        })
    }
}

impl AlephProject {
    fn ensure_core_files_and_directories(&self) -> anyhow::Result<()> {
        // Ensure that 'aleph-project.toml' exists and that it is a file
        let project_file_meta = self
            .project_file
            .metadata()
            .context("Checking if 'aleph-project.toml' exists")?;
        if !project_file_meta.is_file() {
            return Err(anyhow!("{:?} is not a file", self.project_file));
        }

        // Create the .aleph folder if it doesn't already exist
        std::fs::create_dir_all(&self.dot_aleph_path)
            .context("Creating .aleph directory if missing")?;

        Ok(())
    }

    fn compute_target_project_root(root: &Path, target: &Target) -> anyhow::Result<PathBuf> {
        match target.platform {
            BuildPlatform::UWP => {
                assert_ne!(target.arch, Architecture::Unknown);
                let mut root = root.to_path_buf();
                root.push("proj");
                root.push("uwp");
                root.push(target.arch.name());
                Ok(root)
            }
            BuildPlatform::Android => {
                let mut root = root.to_path_buf();
                root.push("proj");
                root.push("android");
                Ok(root)
            }
            _ => Err(anyhow!(
                "Platform \"{}\" does not have a target specific sub-project.",
                target.platform.name()
            )),
        }
    }
}
