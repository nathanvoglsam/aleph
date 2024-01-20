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

use std::collections::HashSet;
use std::path::Path;

use crate::commands::ISubcommand;
use crate::project::AlephProject;
use crate::templates;
use crate::utils::BuildPlatform;
use aleph_shader_db::ShaderDatabase;
use aleph_shader_db::ShaderEntry;
use aleph_target::build::target_platform;
use aleph_target::Profile;
use anyhow::anyhow;
use camino::Utf8Path;
use camino::Utf8PathBuf;
use cargo_metadata::DependencyKind;
use cargo_metadata::Package;
use clap::{Arg, ArgMatches};
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use serde::Deserialize;
use serde::Serialize;
use std::io::Write;

pub struct Cook {}

impl ISubcommand for Cook {
    fn name(&self) -> &'static str {
        "cook"
    }

    fn description(&mut self) -> clap::Command {
        let platform = Arg::new("platform")
            .help("The platform to cook the game assets for.")
            .long_help("The platform to cook the game assets for. Supported values: native, uwp, android, windows, macos, linux.")
            .default_value("native")
            .required(false);
        let config = Arg::new("profile")
            .short('p')
            .long("profile")
            .help("The cook configuration to target.")
            .long_help(
                "The cook configuration to target. Supported values: debug, release, retail.",
            )
            .default_value("debug")
            .required(false);
        clap::Command::new(self.name())
            .about("Cooks the game assets for the requested platform/config")
            .long_about("Tool for cooking game assets for the requested platform/config.")
            .arg(platform)
            .arg(config)
    }

    fn exec(&mut self, project: &AlephProject, mut matches: ArgMatches) -> anyhow::Result<()> {
        let platform_arg: String = matches
            .remove_one("platform")
            .expect("platform should have a default");
        let profile_arg: String = matches
            .remove_one("profile")
            .expect("profile should have a default");

        let _platform = BuildPlatform::from_arg(&platform_arg)
            .ok_or(anyhow!("Unknown platform \"{}\"", &platform_arg))?;
        let _profile = Profile::from_name(&profile_arg.to_lowercase())
            .ok_or(anyhow!("Unknown profile \"{}\"", &profile_arg))?;

        // We need to compute a full list of all the dependencies of the game crate. This is
        // represented by a list of package indices. This may include several versions of the same
        // crate and is calculated by a full traversal of the crate graph starting from the game
        // crate.
        let dependencies = get_game_crate_dependencies(project)?;
        let cargo_metadata = project.get_cargo_metadata()?;
        let deps: Vec<_> = dependencies
            .iter()
            .map(|&v| &cargo_metadata.packages[v])
            .collect();
        // Filter out any dependencies that _don't_ have any shaders
        let deps = filter_dependencies_without_shaders(&deps);

        self.cook_shaders(project, &deps)?;
        self.run_shader_ninja_build(project)?;
        self.archive_shaders(project, &deps)?;

        Ok(())
    }
}

impl Cook {
    fn cook_shaders(&self, project: &AlephProject, deps: &[&Package]) -> anyhow::Result<()> {
        // Get our shader build dir and ensure it exists
        let shader_build_dir = Utf8PathBuf::try_from(project.shader_build_path().to_path_buf())?;
        std::fs::create_dir_all(&shader_build_dir)?;

        // Output the rules template
        let rules_path = shader_build_dir.join("rules.ninja");
        std::fs::write(rules_path, templates::SHADER_NINJA_RULES)?;

        // Walk through all the dependency packages that have shaders and create a build.ninja
        // file for them so we can compile the shaders using ninja
        let ctx = ShaderBuildContext {
            shader_build_dir: shader_build_dir.to_path_buf(),
        };
        deps.par_iter().for_each(|v| {
            log::info!("Generating Ninja File For: {} - {}", v.name, v.version);
            Self::build_shader_ninja_file_for_package(&ctx, v).unwrap()
        });

        let build_path = shader_build_dir.join("build.ninja");
        let mut build_file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&build_path)?;
        writeln!(&mut build_file, "include rules.ninja")?;
        writeln!(&mut build_file)?;

        write!(&mut build_file, "includes = ")?;
        for &dep in deps.iter() {
            let include_dir = dep
                .manifest_path
                .parent()
                .unwrap()
                .join("shaders")
                .join("include");
            write!(
                &mut build_file,
                "{} ",
                dunce::simplified(include_dir.as_std_path()).display()
            )?;
        }
        writeln!(&mut build_file)?;
        writeln!(&mut build_file)?;

        for &dep in deps.iter() {
            writeln!(
                &mut build_file,
                "include {}-{}/build.ninja",
                &dep.name, &dep.version
            )?;
            writeln!(&mut build_file)?;
        }

        Ok(())
    }

    fn build_shader_ninja_file_for_package(
        ctx: &ShaderBuildContext,
        package: &Package,
    ) -> anyhow::Result<()> {
        let output_dir = get_package_shader_output_dir(package, &ctx.shader_build_dir);
        std::fs::create_dir_all(&output_dir)?;

        let output_file_path = output_dir.join("build.ninja");
        let mut output_file = std::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&output_file_path)?;

        // We use a single-threaded walker as we intend to parallelise at the package level instead
        let shader_dir = package.manifest_path.parent().unwrap().join("shaders");
        let source_dir = shader_dir.join("source");
        let walker = ignore::WalkBuilder::new(source_dir).build();

        for entry in walker {
            if let Ok(entry) = entry {
                // We will only process utf-8 paths because we are sane little crewmates. If it's
                // not utf-8 we're in for sadness
                let entry_path =
                    if let Some(v) = Utf8PathBuf::from_path_buf(entry.path().to_path_buf()).ok() {
                        v
                    } else {
                        log::trace!(
                            "Skipping file '{}' because of non-unicode path",
                            entry.path().display()
                        );
                        continue;
                    };

                let f_type = entry.file_type().unwrap();
                if f_type.is_file() {
                    let file_name = entry_path.file_name().expect("File with a name");

                    // Split out the last two dot segments of the file name. For something like
                    // shader.frag.hlsl we should get a file_ext = hlsl and shader_type = frag with
                    // name_segment = shader.
                    //
                    // We need to know part of the rest of the name so we can reject files like
                    // 'frag.hlsl' as it is effectively a nameless shader.
                    let mut dot_segments = file_name.split('.').rev();
                    let file_ext = dot_segments.next();
                    let shader_type = dot_segments.next();
                    let name_segment = dot_segments.next();

                    // Extract our shader type and file extension from the segments of the file
                    // name.
                    //
                    // We want file names like <name>.<stype>.hlsl or <name>.<name>.<stype>.hlsl
                    let (_, shader_type) = match (file_ext, shader_type, name_segment) {
                        (Some(a), Some(b), Some(_)) => (a, b),
                        (Some(_), Some(_), None) => {
                            // If there's no file name then the shader has no name, we should log
                            // to warn that we're skipping the shader and skip the file.
                            log::warn!("Skipping nameless shader - '{}'", entry.path().display());
                            continue;
                        }
                        _ => continue,
                    };

                    let file_name_no_ext = entry_path.file_stem().unwrap();
                    let out_file = output_dir.join(file_name_no_ext);
                    output_build_statement_for_shader(
                        &mut output_file,
                        &out_file,
                        &entry_path,
                        shader_type,
                        "spirv",
                    )?;
                    output_build_statement_for_shader(
                        &mut output_file,
                        &out_file,
                        &entry_path,
                        shader_type,
                        "dxil",
                    )?;
                }
            }
        }

        Ok(())
    }

    fn run_shader_ninja_build(&self, project: &AlephProject) -> anyhow::Result<()> {
        // If we have a bundled ninja exe use that, otherwise just rely on what's in the path
        let ninja = project.ninja_path();
        let ninja = if ninja.exists() {
            dunce::simplified(ninja)
        } else {
            Path::new("ninja")
        };

        let mut command = std::process::Command::new(ninja);
        command.current_dir(project.shader_build_path());

        let dxc = project.dxc_path();
        if dxc.exists() {
            let dxc_directory = dunce::simplified(dxc.parent().unwrap());
            let path_string = std::env::var("PATH")?;
            let sep = if target_platform().is_windows() {
                ";"
            } else {
                ":"
            };
            let new_path = format!("{}{}{}", dxc_directory.display(), sep, path_string);
            command.env("PATH", new_path);
        }

        log::info!("{:#?}", &command);
        let status = command.status()?;

        if !status.success() {
            log::error!("Ninja invocation failed! Terminating cook.");
            return Err(anyhow!("ninja invocation failed!"));
        }

        Ok(())
    }

    fn archive_shaders(&self, project: &AlephProject, deps: &[&Package]) -> anyhow::Result<()> {
        let shader_build_dir = Utf8PathBuf::try_from(project.shader_build_path().to_path_buf())?;
        let out_path = shader_build_dir.join("shaders.shaderdb");

        log::info!(
            "Compiling ShaderDatabase '{}'",
            dunce::simplified(out_path.as_std_path()).to_str().unwrap()
        );
        let mut shader_db = ShaderDatabase::default();

        deps.iter().for_each(|&package| {
            log::info!(
                "Collecting shaders for package '{}' - {}",
                &package.name,
                &package.version
            );
            archive_shaders_for_package(&mut shader_db, &shader_build_dir, package).unwrap();
        });

        let bytes = rkyv::to_bytes::<_, 1_048_576>(&shader_db).unwrap();

        std::fs::write(out_path, bytes)?;

        Ok(())
    }
}

fn archive_shaders_for_package(
    shader_db: &mut ShaderDatabase,
    shader_build_dir: &Utf8Path,
    package: &Package,
) -> anyhow::Result<()> {
    let package_shader_dir = get_package_shader_output_dir(package, &shader_build_dir);

    let read_dir = package_shader_dir.read_dir_utf8()?;
    for file in read_dir {
        let file = file?;

        let file_type = file.file_type()?;
        if !file_type.is_file() {
            continue;
        }

        let file_name = Utf8Path::new(file.file_name());
        let file_stem = file_name.file_stem();
        match (file_name.extension(), file_stem) {
            (Some("spirv"), Some(stem)) => {
                let file_data = std::fs::read(file.path())?;
                if let Some(db_entry) = shader_db.shaders.get_mut(stem) {
                    db_entry.spirv = file_data;
                } else {
                    shader_db.shaders.insert(
                        stem.to_string(),
                        ShaderEntry {
                            spirv: file_data,
                            dxil: Vec::new(),
                        },
                    );
                }
            }
            (Some("dxil"), Some(stem)) => {
                let file_data = std::fs::read(file.path())?;
                if let Some(db_entry) = shader_db.shaders.get_mut(stem) {
                    db_entry.dxil = file_data;
                } else {
                    shader_db.shaders.insert(
                        stem.to_string(),
                        ShaderEntry {
                            spirv: Vec::new(),
                            dxil: file_data,
                        },
                    );
                }
            }
            _ => continue,
        }
    }

    Ok(())
}

fn filter_dependencies_without_shaders<'a>(deps: &[&'a Package]) -> Vec<&'a Package> {
    deps.iter()
        .filter(|&&v| {
            let aleph_metadata = AlephCrateMetadata::load_for_package(v).unwrap();
            if let Some(aleph_metadata) = aleph_metadata {
                aleph_metadata.has_shaders()
            } else {
                false
            }
        })
        .copied()
        .collect()
}

fn output_build_statement_for_shader(
    output_file: &mut std::fs::File,
    out_file: &Utf8Path,
    in_file: &Utf8Path,
    shader_type: &str,
    backend: &str,
) -> anyhow::Result<()> {
    // Select which ninja rule to use based on the declared shader type
    let rule = match shader_type {
        // Fragment/Pixel shaders
        "frag" | "fragment" | "pix" | "pixel" | "ps" | "fg" => "fragment_shader",

        // Vertex shaders
        "vert" | "vertex" | "vs" => "vertex_shader",

        // Compute shaders
        "comp" | "compute" | "cs" => "compute_shader",

        // Geometry shaders
        "geom" | "geometry" | "gs" => "geometry_shader",

        _ => {
            return Ok(());
        }
    };

    if target_platform().is_windows() {
        // We have to replace bare colons with '$:' as ninja doesn't handle them
        let out_file = dunce::simplified(out_file.as_std_path()).to_str().unwrap();
        let out_file = out_file.replace(':', "$:");
        let in_file = dunce::simplified(in_file.as_std_path()).to_str().unwrap();
        let in_file = in_file.replace(':', "$:");

        writeln!(
            output_file,
            "build {out_file}.{backend}: {rule}_{backend} {in_file}"
        )?;
    } else {
        writeln!(
            output_file,
            "build {out_file}.{backend}: {rule}_{backend} {in_file}"
        )?;
    }
    writeln!(output_file)?;
    Ok(())
}

struct ShaderBuildContext {
    /// Path to shader build directory
    pub shader_build_dir: Utf8PathBuf,
}

fn get_package_shader_output_dir(v: &Package, base: &Utf8Path) -> Utf8PathBuf {
    let output_dir = format!("{}-{}", &v.name, &v.version);
    base.join(output_dir)
}

#[derive(Default, Serialize, Deserialize)]
struct AlephCrateMetadata {
    /// A flag whether a crate includes shaders that should be cooked for any game including this
    /// package
    pub shaders: Option<bool>,
}

impl AlephCrateMetadata {
    pub fn load_for_package(package: &Package) -> anyhow::Result<Option<Self>> {
        if let Some(metadata) = package
            .metadata
            .as_object()
            .map(|v| v.get("aleph"))
            .flatten()
        {
            Ok(serde_json::from_value(metadata.clone())?)
        } else {
            Ok(None)
        }
    }

    pub fn has_shaders(&self) -> bool {
        self.shaders.unwrap_or_default()
    }
}

fn get_game_crate_dependencies(project: &AlephProject) -> anyhow::Result<HashSet<usize>> {
    let cargo_metadata = project.get_cargo_metadata()?;
    let (package, _) = project.get_game_crate_and_target()?;

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
            let found_crate = project.find_matching_crate_index(name, version_spec)?;
            if let Some(found_crate) = found_crate {
                let not_found = package_indices.insert(found_crate);
                if not_found {
                    package_stack.push(&cargo_metadata.packages[found_crate]);
                }
            }
        }
    }
    Ok(package_indices)
}
