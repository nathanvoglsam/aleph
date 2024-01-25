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

use std::collections::HashMap;
use std::mem::size_of;

use aleph_target::build::{target_architecture, target_platform};
use aleph_target::{Architecture, Profile};
use anyhow::{anyhow, Context};
use bumpalo::Bump;
use camino::{Utf8Path, Utf8PathBuf};
use cargo_metadata::semver::{Version, VersionReq};
use cargo_metadata::Package;
use once_cell::sync::OnceCell;

use crate::project_schema::ProjectSchema;
use crate::utils::{find_project_file, BuildPlatform, Target};

/// A tuple of [Version] and an unwrapped [cargo_metadata::PackageId]. Unrapping the type to a str
/// instead of a String allows avoiding extra .clone calls when building the table.
pub type VersionedPackageId<'a> = (Version, &'a str);

/// A table of crate versions keyed by the name of the crate.
pub type CrateTable<'a> = HashMap<&'a str, &'a [VersionedPackageId<'a>]>;

/// A hash table that maps a [cargo_metadata::PackageId] to the index in the metadata's packages
/// array.
pub type CrateIdMap<'a> = HashMap<&'a str, usize>;

#[derive(Clone)]
pub struct AlephProject<'a> {
    /// A bump allocated arena for the project to use for w/e purpose is needed
    arena: &'a Bump,

    /// The path to the 'aleph-project.toml' file for this project
    project_file: Utf8PathBuf,

    /// The path to the directory that contains 'aleph-project.toml' for this project. Just a cached
    /// form of `project_file.parent()`
    project_root: Utf8PathBuf,

    /// The path to the '.aleph' folder for this project
    dot_aleph_path: Utf8PathBuf,

    /// The path to the '.aleph/.shaders' folder for this project
    shader_build_path: Utf8PathBuf,

    /// Path to the android project in the '.aleph/proj' directory
    android_proj_path: Utf8PathBuf,

    /// Path to the uwp x86_64 project in the '.aleph/proj' directory
    uwp_x86_64_proj_path: Utf8PathBuf,

    /// Path to the uwp aarch64 project in the '.aleph/proj' directory
    uwp_aarch64_proj_path: Utf8PathBuf,

    /// The path to the '.aleph/sdks/ndk' folder for this project
    ndk_path: Utf8PathBuf,

    /// The path to the '.aleph/sdks/dxc' bin folder for this project
    dxc_path: Utf8PathBuf,

    /// The path to the '.aleph/sdks/slang' bin folder for this project
    slang_path: Utf8PathBuf,

    /// The path to the '.aleph/sdks/ninja' bin folder for this project
    ninja_path: Utf8PathBuf,

    /// The path to the Cargo.toml file adjacent to the aleph-project.toml
    cargo_toml_file: Utf8PathBuf,

    /// The cargo target directory
    cargo_target_dir: Utf8PathBuf,

    /// A cached copy of the collect Cargo metadata
    cargo_metadata: OnceCell<cargo_metadata::Metadata>,

    /// A precomputed lookup acceleration table based on cargo metadata. Maps any crate name to a
    /// list of (version, package_id) pairs that represents all versions of that crate referenced
    /// by the entire workspace.
    crate_table: OnceCell<CrateTable<'a>>,

    /// A hash table that maps a [cargo_metadata::PackageId] to the index in the metadata's packages
    /// array.
    crate_id_map: OnceCell<CrateIdMap<'a>>,

    /// A cache of the (package index, target index) values for the 'game' crate specified by the
    /// project schema
    game_crate_and_target: OnceCell<(usize, Option<usize>)>,

    /// A cached copy of the aleph-project.toml file
    project_schema: OnceCell<ProjectSchema<'static>>,
}

impl<'a> AlephProject<'a> {
    pub fn new(arena: &'a Bump) -> anyhow::Result<Self> {
        let current_dir = std::env::current_dir()?;
        let current_dir = Utf8PathBuf::try_from(current_dir)?;
        let project_file = find_project_file(&current_dir)
            .context("Finding aleph-project.toml in current working directory")?
            .canonicalize()
            .context("Canonicalizing aleph-project.toml path")?;
        let project_file = Utf8PathBuf::try_from(project_file)?;
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

        let shader_build_path = dot_aleph_path.join("shaders");

        let mut ndk_path = dot_aleph_path.clone();
        ndk_path.push("sdks");
        ndk_path.push("ndk");

        let mut dxc_path = dot_aleph_path.clone();
        dxc_path.push("sdks");
        dxc_path.push("dxc");
        dxc_path.push("bin");
        if target_platform().is_windows() {
            let arch = match target_architecture() {
                Architecture::X8664 => "x64",
                Architecture::AARCH64 => "arm64",
                Architecture::Unknown => unreachable!(),
            };
            dxc_path.push(arch);
            dxc_path.push("dxc.exe");
        } else {
            dxc_path.push("dxc");
        }

        let mut slang_path = dot_aleph_path.clone();
        slang_path.push("sdks");
        slang_path.push("slang");
        slang_path.push("bin");
        push_platform_slang_path(&mut slang_path)?;

        let mut ninja_path = dot_aleph_path.clone();
        ninja_path.push("sdks");
        ninja_path.push("ninja");
        if target_platform().is_windows() {
            ninja_path.push("ninja.exe");
        } else {
            ninja_path.push("ninja");
        }

        let out = Self {
            arena,
            project_file,
            project_root,
            dot_aleph_path,
            shader_build_path,
            android_proj_path,
            uwp_x86_64_proj_path,
            uwp_aarch64_proj_path,
            ndk_path,
            dxc_path,
            slang_path,
            ninja_path,
            cargo_toml_file,
            cargo_target_dir,
            cargo_metadata: Default::default(),
            crate_table: Default::default(),
            crate_id_map: Default::default(),
            game_crate_and_target: Default::default(),
            project_schema: Default::default(),
        };

        out.ensure_core_files_and_directories()?;

        Ok(out)
    }

    /// Returns the path to the `aleph-project.toml` file for the project we're working with
    pub fn project_file(&self) -> &Utf8Path {
        &self.project_file
    }

    /// Returns the path to the folder that contains the `aleph-project.toml` file for the project
    /// we're working with
    pub fn project_root(&self) -> &Utf8Path {
        &self.project_root
    }

    // /// The path to the '.aleph' directory for the current project. The .aleph directory will be
    // /// in the same directory as the 'aleph-project.toml' file.
    // pub fn dot_aleph_path(&self) -> &Path {
    //     &self.dot_aleph_path
    // }

    /// Returns the path to the folder that contains the `shaders` directory that will be used as
    /// the output directory for our shader builds
    pub fn shader_build_path(&self) -> &Utf8Path {
        &self.shader_build_path
    }

    /// A utility around [Self::target_project_root] that returns also ensures that the project
    /// directory exists and is a directory.
    pub fn ensure_target_project_root(&self, target: &Target) -> anyhow::Result<&Utf8Path> {
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
    pub fn target_project_root(&self, target: &Target) -> anyhow::Result<&Utf8Path> {
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
    pub fn cargo_toml_file(&self) -> &Utf8Path {
        &self.cargo_toml_file
    }

    /// Returns the path to the cargo target directory root, which will be adjacent to the
    /// 'aleph-project.toml' and 'Cargo.toml'
    pub fn cargo_target_dir(&self) -> &Utf8Path {
        &self.cargo_target_dir
    }

    /// Returns the path to the project's bundled NDK, in '.aleph/sdks/ndk'. This path may not exist
    /// so check before using!
    pub fn ndk_path(&self) -> &Utf8Path {
        &self.ndk_path
    }

    /// Returns the path to the project's bundled dxc, in '.aleph/sdks/dxc'. This path may not exist
    /// so check before using!
    pub fn dxc_path(&self) -> &Utf8Path {
        &self.dxc_path
    }

    /// Returns the path to the project's bundled dxc, in '.aleph/sdks/slang'. This path may not
    /// exist so check before using!
    pub fn slang_path(&self) -> &Utf8Path {
        &self.slang_path
    }

    /// Returns the path to the project's bundled ninja, in '.aleph/sdks/ninja'. This path may not
    /// exist so check before using!
    pub fn ninja_path(&self) -> &Utf8Path {
        &self.ninja_path
    }

    /// Returns the './target/{target-triple}/{profile}' path for the request target + profile set.
    pub fn cargo_build_dir_for_target(
        &self,
        target: &Target,
        profile: Profile,
    ) -> anyhow::Result<Utf8PathBuf> {
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

    /// Attempts to load and parse the aleph-project.toml file. Caches the result after the first
    /// load.
    ///
    /// # Warning
    ///
    /// Will _not_ re-query after the first call. Create a new [AlephProject] to re-query.
    pub fn get_project_schema(&self) -> anyhow::Result<&ProjectSchema> {
        self.project_schema.get_or_try_init(|| {
            let toml = std::fs::read_to_string(self.project_file())?;
            let project_schema: ProjectSchema = toml::from_str(&toml)?;

            Ok(project_schema)
        })
    }

    /// Computes and returns the 'crate table' structure.
    ///
    /// A precomputed lookup acceleration table based on cargo metadata. Maps any crate name to a
    /// list of (version, package_id) pairs that represents all versions of that crate referenced
    /// by the entire workspace.
    ///
    /// This can be used to efficiently look up a crate by name using a hash lookup instead of a
    /// linear search through an array. It is also logically a multimap, containing N elements at
    /// any table entry. Those N elements will be the different versions of the crate present in
    /// the workspace as it's possible for multiple versions of a crate to be present.
    ///
    /// This calls [Self::get_cargo_metadata] internally so can error in the same way as that
    /// function.
    pub fn get_crate_table(&self) -> anyhow::Result<&CrateTable> {
        self.crate_table.get_or_try_init(|| {
            let cargo_metadata = self.get_cargo_metadata()?;

            // Temporary arena that makes it cheap to build the list of crates as a linked list
            // before we ossify it to flat arrays allocated out of the [AlephProject]'s arena.
            //
            // Size hint to avoid allocating new pages in the loop. We should have exactly
            // num_packages links.
            let size_hint = size_of::<CrateLink>() * cargo_metadata.packages.len();
            let temp_arena = Bump::with_capacity(size_hint);

            // Link in the linked list we build while accumulating all the present versions of a
            // given crate
            struct CrateLink<'a, 'x> {
                pub item: VersionedPackageId<'a>,
                pub next: Option<&'x CrateLink<'a, 'x>>,
            }

            // First stage, iterate over all packages and bundle up all the versions of each crate
            // by building a temporary linked list for every crate name key in the map.
            //
            // Using the linked list avoids reallocs, and allocation out of the temporary arena
            // is cheap.
            let mut crate_table = HashMap::new();
            for package in cargo_metadata.packages.iter() {
                let name = &*self.arena.alloc_str(&package.name);
                let id = &*self.arena.alloc_str(&package.id.repr);
                crate_table
                    .entry(name)
                    .and_modify(|old| {
                        let next = *old;
                        let link = temp_arena.alloc(CrateLink {
                            item: (package.version.clone(), id),
                            next: Some(next),
                        });
                        *old = link;
                    })
                    .or_insert_with(|| {
                        temp_arena.alloc(CrateLink {
                            item: (package.version.clone(), id),
                            next: None,
                        })
                    });
            }

            let mut final_crate_table = HashMap::with_capacity(crate_table.capacity());
            for (name, list) in crate_table.drain() {
                // Count the number of entries in the crate list
                let mut num_links = 1;
                let mut next = list.next;
                while let Some(v) = next {
                    next = v.next;
                    num_links += 1;
                }

                // 'Arrayify' our linked list
                let mut link = Some(list);
                let list = self.arena.alloc_slice_fill_with(num_links, |_| {
                    let l = link.unwrap();
                    link = list.next;
                    l.item.clone()
                });
                final_crate_table.insert(name, &*list);
            }

            Ok(final_crate_table)
        })
    }

    /// Computes and returns a hash table that maps a [cargo_metadata::PackageId] to the index in
    /// the [cargo_metadata::Metadata::packages] array the package is from.
    pub fn get_crate_id_map(&self) -> anyhow::Result<&CrateIdMap> {
        self.crate_id_map.get_or_try_init(|| {
            let cargo_metadata = self.get_cargo_metadata()?;

            let mut crate_id_table = HashMap::with_capacity(cargo_metadata.packages.len());
            for (i, package) in cargo_metadata.packages.iter().enumerate() {
                if !crate_id_table.contains_key(package.id.repr.as_str()) {
                    let key = &*self.arena.alloc_str(&package.id.repr);
                    crate_id_table.insert(key, i);
                }
            }

            Ok(crate_id_table)
        })
    }

    // /// Utility function that will return a package reference for the given criteria.
    // ///
    // /// The function searches for a crate with the given name and the highest version number that
    // /// matches the provided version spec.
    // ///
    // /// This is useful for looking up the concrete package for a crate's dependency spec.
    // pub fn find_matching_crate(
    //     &self,
    //     name: &str,
    //     version_spec: &VersionReq,
    // ) -> anyhow::Result<Option<&Package>> {
    //     let cargo_metadata = self.get_cargo_metadata()?;
    //     self.find_matching_crate_index(name, version_spec)
    //         .map(|v| v.map(|v| &cargo_metadata.packages[v]))
    // }

    /// Utility function that will return a package index for the given criteria.
    ///
    /// The function searches for a crate with the given name and the highest version number that
    /// matches the provided version spec.
    ///
    /// This is useful for looking up the concrete package for a crate's dependency spec.
    pub fn find_matching_crate_index(
        &self,
        name: &str,
        version_spec: &VersionReq,
    ) -> anyhow::Result<Option<usize>> {
        let crate_table = self.get_crate_table()?;

        if let Some(&versions) = crate_table.get(name) {
            let best_match = versions.iter().fold(&versions[0], |acc, v| {
                if version_spec.matches(&acc.0) && acc.0 > v.0 {
                    acc
                } else {
                    v
                }
            });

            let crate_id_map = self.get_crate_id_map()?;
            if let Some(&package_index) = crate_id_map.get(best_match.1) {
                Ok(Some(package_index))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    pub fn get_game_crate_and_target(
        &self,
    ) -> anyhow::Result<(&Package, Option<&cargo_metadata::Target>)> {
        let cargo_metadata = self.get_cargo_metadata()?;
        self.game_crate_and_target
            .get_or_try_init(|| {
                let project = self.get_project_schema()?;
                let crate_id_map = self.get_crate_id_map()?;

                let name = project.game.crate_name.as_ref();

                let package_index = cargo_metadata
                    .workspace_members
                    .iter()
                    .map(|v| *crate_id_map.get(v.repr.as_str()).unwrap())
                    .find(|&v| cargo_metadata.packages[v].name.as_str() == name)
                    .ok_or(anyhow!(
                        "Failed to find crate \"{name}\" in the cargo workspace"
                    ))?;

                let package = &cargo_metadata.packages[package_index];
                let target_index = package
                    .targets
                    .iter()
                    .enumerate()
                    .find(|(_i, v)| v.kind.iter().any(|v| v.as_str() == "lib"))
                    .map(|(i, _v)| i);

                Ok((package_index, target_index))
            })
            .map(|(package_index, target_index)| {
                let package = &cargo_metadata.packages[*package_index];
                let target = target_index.map(|v| &package.targets[v]);
                (package, target)
            })
    }
}

impl<'a> AlephProject<'a> {
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

    fn compute_target_project_root(
        root: &Utf8Path,
        target: &Target,
    ) -> anyhow::Result<Utf8PathBuf> {
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

fn push_platform_slang_path(slang_path: &mut Utf8PathBuf) -> anyhow::Result<()> {
    match target_platform() {
        p @ aleph_target::Platform::WindowsGNU | p @ aleph_target::Platform::WindowsMSVC => {
            match target_architecture() {
                Architecture::X8664 => slang_path.push("windows-x64"),
                Architecture::AARCH64 => slang_path.push("windows-arm64"),
                v @ Architecture::Unknown => {
                    return Err(anyhow!(
                        "Unsupported build host arch '{}' for platform '{}'",
                        v,
                        p
                    ));
                }
            };
            slang_path.push("release");
            slang_path.push("slangc.exe");
        }
        p @ aleph_target::Platform::Linux => {
            match target_architecture() {
                Architecture::X8664 => slang_path.push("linux-x64"),
                Architecture::AARCH64 => slang_path.push("linux-arm64"),
                v @ Architecture::Unknown => {
                    return Err(anyhow!(
                        "Unsupported build host arch '{}' for platform '{}'",
                        v,
                        p
                    ));
                }
            };
            slang_path.push("release");
            slang_path.push("slangc");
        }
        p @ aleph_target::Platform::MacOS => {
            match target_architecture() {
                Architecture::X8664 => slang_path.push("macosx-x64"),
                Architecture::AARCH64 => slang_path.push("macosx-aarch64"),
                v @ Architecture::Unknown => {
                    return Err(anyhow!(
                        "Unsupported build host arch '{}' for platform '{}'",
                        v,
                        p
                    ));
                }
            };
            slang_path.push("release");
            slang_path.push("slangc");
        }
        v @ aleph_target::Platform::UniversalWindowsGNU
        | v @ aleph_target::Platform::UniversalWindowsMSVC
        | v @ aleph_target::Platform::Android
        | v @ aleph_target::Platform::Unknown => {
            return Err(anyhow!("Unsupported build host platform '{}'", v));
        }
    }
    Ok(())
}
