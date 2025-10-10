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

use std::collections::{HashMap, HashSet};

use aleph_alloc::{BVec, Blink};
use aleph_target::build::{target_architecture, target_platform};
use aleph_target::{Architecture, Platform, Profile};
use anyhow::{Context, anyhow};
use camino::{Utf8Path, Utf8PathBuf};
use cargo_metadata::semver::{Version, VersionReq};
use cargo_metadata::{DependencyKind, Package, TargetKind};
use once_cell::sync::OnceCell;

use crate::project_schema::ProjectSchema;
use crate::utils::{Target, find_project_file};

/// A tuple of [`Version`] and an unwrapped [cargo_metadata::PackageId]. Unrapping the type to a str
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
    _arena: &'a Blink,

    /// The path to the 'aleph-project.toml' file for this project
    project_file: Utf8PathBuf,

    /// The path to the directory that contains 'aleph-project.toml' for this project. Just a cached
    /// form of `project_file.parent()`
    project_root: Utf8PathBuf,

    /// The path to the '.aleph' folder for this project
    dot_aleph_path: Utf8PathBuf,

    /// The path to the '.aleph/shaders' folder for this project
    shader_build_path: Utf8PathBuf,

    /// The path to the '.aleph/assets' folder for this project
    _assets_build_path: Utf8PathBuf,

    /// The path to the '.aleph/configs' folder for this project
    configs_build_path: Utf8PathBuf,

    /// The path to the '.aleph/sdk/{platform}/{arch}' folder appropriate for the host system,
    _sdk_path: Utf8PathBuf,

    /// The path to the '.aleph/sdk/**/dxc/bin/{platform}/dxc' executable for this project
    dxc_path: Utf8PathBuf,

    /// The path to the '.aleph/sdk/**/slang/bin/{platform}/release/slangc' executable for this
    /// project
    slang_path: Utf8PathBuf,

    /// The path to the '.aleph/sdk/**/ninja/ninja' executable for this project
    ninja_path: Utf8PathBuf,

    /// The path to the Cargo.toml file adjacent to the aleph-project.toml
    cargo_toml_file: Utf8PathBuf,

    /// The path to the aleph.code-workspace file in the root of the workspace
    vscode_workspace_file: Utf8PathBuf,

    /// The cargo target directory
    _cargo_target_dir: Utf8PathBuf,

    /// A cached copy of the collect Cargo metadata
    cargo_metadata: OnceCell<cargo_metadata::Metadata>,

    /// A precomputed lookup acceleration table based on cargo metadata. Maps any crate name to a
    /// list of (version, package_id) pairs that represents all versions of that crate referenced
    /// by the entire workspace.
    _crate_table: OnceCell<CrateTable<'a>>,

    /// A hash table that maps a [cargo_metadata::PackageId] to the index in the metadata's packages
    /// array.
    _crate_id_map: OnceCell<CrateIdMap<'a>>,

    /// A cache of the (package index, target index) values for the 'game' crate specified by the
    /// project schema
    _game_crate_and_target: OnceCell<(usize, Option<usize>)>,

    /// A cache of all direct and transient dependencies of the game crate.
    _game_crate_dependency_set: OnceCell<HashSet<usize>>,

    /// A cached copy of the aleph-project.toml file
    project_schema: OnceCell<ProjectSchema<'static>>,
}

impl<'a> AlephProject<'a> {
    pub fn new(arena: &'a Blink) -> anyhow::Result<Self> {
        let current_dir = std::env::current_dir()?;
        let current_dir = Utf8PathBuf::try_from(current_dir)?;
        let project_file = find_project_file(current_dir)
            .context("Finding aleph-project.toml in current working directory")?
            .canonicalize()
            .context("Canonicalizing aleph-project.toml path")?;
        let project_file = Utf8PathBuf::try_from(project_file)?;
        let project_root = project_file.parent().unwrap().to_path_buf();
        let dot_aleph_path = project_root.join(".aleph");
        let cargo_toml_file = project_root.join("Cargo.toml");
        let vscode_workspace_file = project_root.join("aleph.code-workspace");
        let cargo_target_dir = project_root.join("target");

        let shader_build_path = dot_aleph_path.join("shaders");
        let assets_build_path = dot_aleph_path.join("data").join("assets");
        let configs_build_path = dot_aleph_path.join("configs");

        let mut sdk_path = dot_aleph_path.clone();
        sdk_path.push("sdk");
        sdk_path.push(sdk_platform_name());
        sdk_path.push(target_architecture().name());

        let mut dxc_path = sdk_path.clone();
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

        let mut slang_path = sdk_path.clone();
        slang_path.push("slang");
        slang_path.push("bin");
        if target_platform().is_windows() {
            slang_path.push("slangc.exe");
        } else {
            slang_path.push("slangc");
        }

        let mut ninja_path = sdk_path.clone();
        ninja_path.push("ninja");
        if target_platform().is_windows() {
            ninja_path.push("ninja.exe");
        } else {
            ninja_path.push("ninja");
        }

        let out = Self {
            _arena: arena,
            project_file,
            project_root,
            dot_aleph_path,
            shader_build_path,
            _assets_build_path: assets_build_path,
            configs_build_path,
            _sdk_path: sdk_path,
            dxc_path,
            slang_path,
            ninja_path,
            cargo_toml_file,
            vscode_workspace_file,
            _cargo_target_dir: cargo_target_dir,
            cargo_metadata: Default::default(),
            _crate_table: Default::default(),
            _crate_id_map: Default::default(),
            _game_crate_and_target: Default::default(),
            _game_crate_dependency_set: Default::default(),
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

    /// The path to the '.aleph' directory for the current project. The .aleph directory will be
    /// in the same directory as the 'aleph-project.toml' file.
    pub fn _dot_aleph_path(&self) -> &Utf8Path {
        &self.dot_aleph_path
    }

    /// Returns the path to the folder that contains the `shaders` directory that will be used as
    /// the output directory for our shader builds
    pub fn shader_build_path(&self) -> &Utf8Path {
        &self.shader_build_path
    }

    /// Returns the path to the folder that contains the `assets` directory that will be used as
    /// the output directory for our cooked assets
    pub fn _assets_build_path(&self) -> &Utf8Path {
        &self._assets_build_path
    }

    /// Returns the path to the folder that contains the `configs` directory that will be used as
    /// the output directory for our config files
    pub fn configs_build_path(&self) -> &Utf8Path {
        &self.configs_build_path
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
    ///
    /// All other targets will return Err()
    pub fn target_project_root(&self, target: &Target) -> anyhow::Result<&Utf8Path> {
        match target.platform {
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

    /// Returns the path to the code-workspace file that the project is using, adjacent to the
    /// 'aleph-project.toml'
    pub fn vscode_workspace_file(&self) -> &Utf8Path {
        &self.vscode_workspace_file
    }

    /// Returns the path to the cargo target directory root, which will be adjacent to the
    /// 'aleph-project.toml' and 'Cargo.toml'
    pub fn _cargo_target_dir(&self) -> &Utf8Path {
        &self._cargo_target_dir
    }

    /// Returns the path to the project's bundled NDK, in '.aleph/sdk/**/ndk'. This path may not
    /// exist so check before using!
    pub fn _sdk_path(&self) -> &Utf8Path {
        &self._sdk_path
    }

    /// Returns the path to the project's bundled dxc, in '.aleph/sdk/**/dxc/bin/{platform}/dxc'.
    /// This path may not exist so check before using!
    pub fn dxc_path(&self) -> &Utf8Path {
        &self.dxc_path
    }

    /// Returns the path to the project's bundled dxc, in
    /// '.aleph/sdk/**/slang/bin/{platform}/release/slang'. This path may not exist so check before
    /// using!
    pub fn slang_path(&self) -> &Utf8Path {
        &self.slang_path
    }

    /// Returns the path to the project's bundled ninja, in '.aleph/sdk/**/ninja/ninja'. This path
    /// may not exist so check before using!
    pub fn ninja_path(&self) -> &Utf8Path {
        &self.ninja_path
    }

    /// Returns the './target/{target-triple}/{profile}' path for the request target + profile set.
    pub fn _cargo_build_dir_for_target(
        &self,
        target: &Target,
        _profile: Profile,
    ) -> anyhow::Result<Utf8PathBuf> {
        assert_ne!(target.arch, Architecture::Unknown);

        match target.platform {
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
    /// Will _not_ re-query after the first call. Create a new [`AlephProject`] to re-query.
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
    /// Will _not_ re-query after the first call. Create a new [`AlephProject`] to re-query.
    pub fn get_project_schema(&self) -> anyhow::Result<&ProjectSchema<'_>> {
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
    pub fn _get_crate_table(&self) -> anyhow::Result<&CrateTable<'_>> {
        self._crate_table.get_or_try_init(|| {
            let cargo_metadata = self.get_cargo_metadata()?;

            // Temporary arena that makes it cheap to build the list of crates as a linked list
            // before we ossify it to flat arrays allocated out of the [`AlephProject`]'s arena.
            //
            // Size hint to avoid allocating new pages in the loop. We should have exactly
            // num_packages links.
            let temp_arena = Blink::new();

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
                let name = &*self._arena.copy_str(&package.name);
                let id = &*self._arena.copy_str(&package.id.repr);
                crate_table
                    .entry(name)
                    .and_modify(|old| {
                        let next = *old;
                        // This will leak package.version. We accept this limitation.
                        let link = temp_arena.put_no_drop(CrateLink {
                            item: (package.version.clone(), id),
                            next: Some(next),
                        });
                        *old = link;
                    })
                    .or_insert_with(|| {
                        // This will leak package.version. We accept this limitation.
                        temp_arena.put_no_drop(CrateLink {
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
                let mut out_list =
                    BVec::with_capacity_in(num_links as usize, self._arena.allocator());
                for _ in 0..num_links {
                    let l = link.unwrap();
                    link = list.next;
                    out_list.push(l.item.clone());
                }
                let out_list = out_list.leak();
                final_crate_table.insert(name, &*out_list);
            }

            Ok(final_crate_table)
        })
    }

    /// Computes and returns a hash table that maps a [cargo_metadata::PackageId] to the index in
    /// the [cargo_metadata::Metadata::packages] array the package is from.
    pub fn _get_crate_id_map(&self) -> anyhow::Result<&CrateIdMap<'_>> {
        self._crate_id_map.get_or_try_init(|| {
            let cargo_metadata = self.get_cargo_metadata()?;

            let mut crate_id_table = HashMap::with_capacity(cargo_metadata.packages.len());
            for (i, package) in cargo_metadata.packages.iter().enumerate() {
                if !crate_id_table.contains_key(package.id.repr.as_str()) {
                    let key = &*self._arena.copy_str(&package.id.repr);
                    crate_id_table.insert(key, i);
                }
            }

            Ok(crate_id_table)
        })
    }

    /// Utility function that will return a package reference for the given criteria.
    ///
    /// The function searches for a crate with the given name and the highest version number that
    /// matches the provided version spec.
    ///
    /// This is useful for looking up the concrete package for a crate's dependency spec.
    pub fn _find_matching_crate(
        &self,
        name: &str,
        version_spec: &VersionReq,
    ) -> anyhow::Result<Option<&Package>> {
        let cargo_metadata = self.get_cargo_metadata()?;
        self._find_matching_crate_index(name, version_spec)
            .map(|v| v.map(|v| &cargo_metadata.packages[v]))
    }

    /// Utility function that will return a package index for the given criteria.
    ///
    /// The function searches for a crate with the given name and the highest version number that
    /// matches the provided version spec.
    ///
    /// This is useful for looking up the concrete package for a crate's dependency spec.
    pub fn _find_matching_crate_index(
        &self,
        name: &str,
        version_spec: &VersionReq,
    ) -> anyhow::Result<Option<usize>> {
        let crate_table = self._get_crate_table()?;

        if let Some(&versions) = crate_table.get(name) {
            let best_match = versions.iter().fold(&versions[0], |acc, v| {
                if version_spec.matches(&acc.0) && acc.0 > v.0 {
                    acc
                } else {
                    v
                }
            });

            let crate_id_map = self._get_crate_id_map()?;
            if let Some(&package_index) = crate_id_map.get(best_match.1) {
                Ok(Some(package_index))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    pub fn _get_game_crate_and_target(
        &self,
    ) -> anyhow::Result<(&Package, Option<&cargo_metadata::Target>)> {
        let cargo_metadata = self.get_cargo_metadata()?;
        self._game_crate_and_target
            .get_or_try_init(|| {
                let project = self.get_project_schema()?;
                let crate_id_map = self._get_crate_id_map()?;

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
                    .find(|(_i, v)| v.kind.iter().any(|v| *v == TargetKind::CDyLib))
                    .map(|(i, _v)| i);

                Ok((package_index, target_index))
            })
            .map(|(package_index, target_index)| {
                let package = &cargo_metadata.packages[*package_index];
                let target = target_index.map(|v| &package.targets[v]);
                (package, target)
            })
    }

    pub fn _get_game_crate_dependencu_set(&self) -> anyhow::Result<&HashSet<usize>> {
        self._game_crate_dependency_set.get_or_try_init(|| {
            let cargo_metadata = self.get_cargo_metadata()?;
            let (package, _) = self._get_game_crate_and_target()?;
            let mut package_indices = HashSet::new();

            let mut package_stack = Vec::new();
            package_stack.push(package);
            while let Some(next_package) = package_stack.pop() {
                for dependency in next_package.dependencies.iter() {
                    // We only care about regular dependencies and not any other kind
                    if dependency.kind != DependencyKind::Normal {
                        continue;
                    }

                    let name = dependency.name.as_str();
                    let version_spec = &dependency.req;
                    let found_crate = self._find_matching_crate_index(name, version_spec)?;
                    if let Some(found_crate) = found_crate {
                        let not_found = package_indices.insert(found_crate);
                        if not_found {
                            package_stack.push(&cargo_metadata.packages[found_crate]);
                        }
                    }
                }
            }
            Ok(package_indices)
        })
    }

    /// Utility that
    pub fn find_tool(
        &self,
        mode: SearchMode,
        name: &Utf8Path,
    ) -> anyhow::Result<Option<Utf8PathBuf>> {
        let mut search_set = Vec::new();
        search_set.push(self.dxc_path().parent().unwrap());
        search_set.push(self.slang_path().parent().unwrap());
        search_set.push(self.ninja_path().parent().unwrap());

        // God help us all if this fails
        let path_env = std::env::var("PATH")?;

        if matches!(mode, SearchMode::IncludeSystemPath) {
            let sep = if target_platform().is_windows() {
                ";"
            } else {
                ":"
            };
            search_set.extend(path_env.split(sep).map(Utf8Path::new));
        }

        let entry = search_set.iter().find_map(|v| {
            let tool = v.join(name);
            if tool.is_file() { Some(tool) } else { None }
        });
        Ok(entry)
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
}

#[derive(Copy, Clone)]
pub enum SearchMode {
    #[allow(dead_code)]
    IncludeSystemPath,

    #[allow(dead_code)]
    OnlySdkPath,
}

fn sdk_platform_name() -> &'static str {
    match target_platform() {
        Platform::WindowsGNU => "windows",
        Platform::WindowsMSVC => "windows",
        Platform::Linux => "linux",
        Platform::MacOS => "macos",
        Platform::IOS => panic!("What are you doing???????"),
        Platform::Unknown => panic!("Unknown host platform"),
    }
}
