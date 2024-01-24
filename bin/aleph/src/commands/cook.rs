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
use crate::shader_system::AlephCrateMetadata;
use crate::shader_system::CompilationParams;
use crate::shader_system::ProjectShaderContext;
use crate::shader_system::ShaderCrateContext;
use crate::shader_system::ShaderModuleContext;
use crate::shader_system::ShaderModuleDefinition;
use crate::shader_system::ShaderModuleDefinitionFile;
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

        let platform = BuildPlatform::from_arg(&platform_arg)
            .ok_or(anyhow!("Unknown platform \"{}\"", &platform_arg))?;
        let _profile = Profile::from_name(&profile_arg.to_lowercase())
            .ok_or(anyhow!("Unknown profile \"{}\"", &profile_arg))?;

        // Build the base level project context for our shader build system
        let project_ctx = ProjectShaderContext::new(project, platform)?;
        project_ctx.ensure_build_directories()?;
        project_ctx.ensure_build_files()?;

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

        // Filter out any dependencies that _definitely don't_ have any shaders. Could contain false
        // positives for crates with aleph metadata but no shaders but eh, this will filter the
        // _VAST_ majority of crates
        let deps = filter_dependencies_without_aleph_metadata(&project_ctx, &deps)?;
        let deps = load_dependencies_module_contexts(&deps)?;

        generate_shader_module_ninja_files(&project_ctx, &deps)?;
        run_shader_ninja_build(project)?;
        archive_shaders(&project_ctx, &deps)?;

        Ok(())
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

struct PartiallyLoadedCrateShaderProject<'a> {
    crate_ctx: ShaderCrateContext<'a>,
    metadata: AlephCrateMetadata<'a>,
}

struct LoadedCrateShaderProject<'a> {
    crate_ctx: ShaderCrateContext<'a>,
    module_contexts: Vec<ShaderModuleContext<'a>>,
}

fn filter_dependencies_without_aleph_metadata<'a, 'b: 'a>(
    project_ctx: &'b ProjectShaderContext,
    deps: &[&'a Package],
) -> anyhow::Result<Vec<PartiallyLoadedCrateShaderProject<'a>>> {
    let mut new_deps = Vec::with_capacity(deps.len());
    for &package in deps {
        let metadata = AlephCrateMetadata::load_for_package(package)?;
        if let Some(metadata) = metadata {
            let crate_ctx =
                ShaderCrateContext::new_with_project_ctx(project_ctx.get_borrowed(), package);
            crate_ctx.ensure_build_directories_no_parents()?;

            let v = PartiallyLoadedCrateShaderProject {
                crate_ctx,
                metadata,
            };

            new_deps.push(v);
        }
    }
    Ok(new_deps)
}

fn load_dependencies_module_contexts<'a>(
    deps: &'a [PartiallyLoadedCrateShaderProject<'a>],
) -> anyhow::Result<Vec<LoadedCrateShaderProject<'a>>> {
    let mut new_deps = Vec::with_capacity(deps.len());
    for v in deps.iter() {
        let module_contexts: Vec<_> = v
            .metadata
            .shaders
            .modules
            .iter()
            .map(|module_name| {
                ShaderModuleContext::new_with_crate_ctx(
                    v.crate_ctx.get_borrowed(),
                    module_name.clone(),
                )
            })
            .collect();

        let v = LoadedCrateShaderProject {
            crate_ctx: v.crate_ctx.get_borrowed(),
            module_contexts,
        };

        new_deps.push(v);
    }
    Ok(new_deps)
}

fn generate_shader_module_ninja_files(
    project_ctx: &ProjectShaderContext,
    deps: &[LoadedCrateShaderProject],
) -> anyhow::Result<()> {
    // Walk through all the dependency packages that have shaders and create a build.ninja
    // file for them so we can compile the shaders using ninja
    deps.par_iter().for_each(|v| {
        log::info!(
            "Generating Ninja Files For: {}",
            v.crate_ctx.crate_output_name.as_ref(),
        );
        build_shader_ninja_file_for_package(&v.module_contexts).unwrap();
    });

    let mut build_file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(project_ctx.root_ninja_file.as_ref())?;
    writeln!(&mut build_file, "include rules.ninja")?;
    writeln!(&mut build_file)?;

    write!(&mut build_file, "includes =")?;
    for dep in deps.iter() {
        for module in dep.module_contexts.iter() {
            let include_dir = module.module_include_dir.as_ref();
            write!(
                &mut build_file,
                " -I {}",
                dunce::simplified(include_dir.as_std_path()).display()
            )?;
        }
    }
    writeln!(&mut build_file)?;
    writeln!(&mut build_file)?;

    for v in deps.iter() {
        for m in v.module_contexts.iter() {
            writeln!(
                &mut build_file,
                "include {}/{}/build.ninja",
                &m.crate_ctx.crate_output_name, &m.module_name
            )?;
            writeln!(&mut build_file)?;
        }
    }

    Ok(())
}

fn build_shader_ninja_file_for_package(
    module_contexts: &[ShaderModuleContext],
) -> anyhow::Result<()> {
    let mut modules = Vec::new();
    for module_ctx in module_contexts {
        module_ctx.ensure_build_directories()?;

        let module_file = std::fs::read_to_string(module_ctx.module_toml_file.as_ref())?;
        let module_file: ShaderModuleDefinitionFile = toml::from_str(&module_file)?;
        modules.push((module_ctx, module_file.module));
    }

    for (module_ctx, module) in modules {
        log::info!(
            "Generating Ninja Files For: {}@{}",
            module_ctx.crate_ctx.crate_output_name,
            module_ctx.module_name
        );
        build_shader_ninja_file_for_shader_module(&module_ctx, &module)?;
    }

    Ok(())
}

fn build_shader_ninja_file_for_shader_module(
    module_ctx: &ShaderModuleContext,
    module: &ShaderModuleDefinition,
) -> anyhow::Result<()> {
    let mut ninja_file = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(module_ctx.module_ninja_file.as_ref())?;

    // Compute the compilation params from the module file
    let compilation_params = CompilationParams::new(module_ctx, module)?;

    // We use a single-threaded walker as we intend to parallelise at the package level instead
    let walker = ignore::WalkBuilder::new(module_ctx.module_source_dir.as_ref()).build();
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
                let (file_ext, shader_type) = match (file_ext, shader_type, name_segment) {
                    (Some(a), Some(b), Some(_)) => (a, b),
                    (Some(_), Some(_), None) => {
                        // If there's no file name then the shader has no name, we should log
                        // to warn that we're skipping the shader and skip the file.
                        log::warn!("Skipping nameless shader - '{}'", entry.path().display());
                        continue;
                    }
                    _ => continue,
                };

                // Skip any file that isn't a HLSL or Slang file.
                match file_ext {
                    "slang" | "hlsl" => {}
                    _ => continue,
                }

                let file_name_no_ext = entry_path.file_stem().unwrap();
                let out_file = module_ctx.module_output_dir.join(file_name_no_ext);

                // Only build dxil on windows, where the full dxil pipeline will be available.
                if module_ctx.platform() == BuildPlatform::Windows {
                    output_build_statement_for_shader(
                        &mut ninja_file,
                        &compilation_params,
                        &out_file,
                        &entry_path,
                        shader_type,
                        "dxil",
                    )?;
                }
                output_build_statement_for_shader(
                    &mut ninja_file,
                    &compilation_params,
                    &out_file,
                    &entry_path,
                    shader_type,
                    "spirv",
                )?;
            }
        }
    }

    Ok(())
}

fn output_build_statement_for_shader(
    ninja_file: &mut std::fs::File,
    compilation_params: &CompilationParams,
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
            ninja_file,
            "build {out_file}.{backend}: {rule}_{backend} {in_file}"
        )?;
    } else {
        writeln!(
            ninja_file,
            "build {out_file}.{backend}: {rule}_{backend} {in_file}"
        )?;
    }
    compilation_params.write_ninja_overrides(ninja_file)?;
    writeln!(ninja_file)?;
    Ok(())
}

fn run_shader_ninja_build(project: &AlephProject) -> anyhow::Result<()> {
    fn push_path_if_tool_exists(v: &mut String, tool: &Path) {
        if tool.exists() {
            let dir = dunce::simplified(tool.parent().unwrap());
            log::trace!("Tool found!: '{}'", tool.display());
            push_path_str(v, dir.to_str().unwrap());
        } else {
            log::trace!("Tool is missing!: '{}'", tool.display());
        }
    }

    fn push_path_str(v: &mut String, s: &str) {
        let sep = if target_platform().is_windows() {
            ";"
        } else {
            ":"
        };

        v.push_str(s);
        v.push_str(sep);
    }

    // If we have a bundled ninja exe use that, otherwise just rely on what's in the path
    let ninja = project.ninja_path();
    let ninja = if ninja.exists() {
        dunce::simplified(ninja)
    } else {
        Path::new("ninja")
    };

    let mut command = std::process::Command::new(ninja);
    command.current_dir(project.shader_build_path());

    let mut path_string = String::new();
    let dxc = project.dxc_path();
    let slang = project.slang_path();

    push_path_if_tool_exists(&mut path_string, dxc);
    push_path_if_tool_exists(&mut path_string, slang);

    let inherit_path = std::env::var("PATH")?;
    push_path_str(&mut path_string, &inherit_path);

    command.env("PATH", path_string);

    log::info!("Running Ninja!");
    let status = command.status()?;

    if !status.success() {
        log::error!("Ninja invocation failed! Terminating cook.");
        return Err(anyhow!("ninja invocation failed!"));
    }

    Ok(())
}

fn archive_shaders(
    project_ctx: &ProjectShaderContext,
    deps: &[LoadedCrateShaderProject],
) -> anyhow::Result<()> {
    let shader_db_file = project_ctx.shaders_output_root_dir.join("shaders.shaderdb");
    log::info!(
        "Compiling ShaderDatabase '{}'",
        dunce::simplified(shader_db_file.as_std_path())
            .to_str()
            .unwrap()
    );

    let mut shader_db = ShaderDatabase::default();

    deps.iter().for_each(|loaded| {
        archive_shaders_for_package(&mut shader_db, loaded).unwrap();
    });

    let bytes = rkyv::to_bytes::<_, 1_048_576>(&shader_db).unwrap();

    std::fs::write(shader_db_file, bytes)?;

    Ok(())
}

fn archive_shaders_for_package(
    shader_db: &mut ShaderDatabase,
    loaded: &LoadedCrateShaderProject,
) -> anyhow::Result<()> {
    log::info!(
        "Collecting shaders for package {}",
        loaded.crate_ctx.crate_output_name
    );

    for module in loaded.module_contexts.iter() {
        log::info!(
            "Collecting shaders for package {}@{}",
            loaded.crate_ctx.crate_output_name,
            module.module_name,
        );
        let read_dir = module.module_output_dir.read_dir_utf8()?;
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
    }

    Ok(())
}
