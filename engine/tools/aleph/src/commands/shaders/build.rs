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

use aleph_target::build::target_platform;
use aleph_target::Profile;
use anyhow::anyhow;
use bumpalo::Bump;
use camino::{Utf8Path, Utf8PathBuf};
use clap::ArgMatches;

use crate::commands::{config_arg, platform_arg, ISubcommand};
use crate::project::AlephProject;
use crate::shader_system::{
    ShaderCrateContext, ShaderFile, ShaderProjectContext, ShaderSubproject,
};
use crate::utils::{dunce_utf8, BuildPlatform};

pub struct BuildShaderProj {}

impl ISubcommand for BuildShaderProj {
    fn name(&self) -> &'static str {
        "build"
    }

    fn description(&mut self) -> clap::Command {
        clap::Command::new(self.name())
            .about("Cooks the game assets for the requested platform/config")
            .long_about("Tool for cooking game assets for the requested platform/config.")
            .arg(platform_arg())
            .arg(config_arg())
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

        // Build the base level project context for our shader build system
        let arena = Bump::new();
        let project_ctx = ShaderSubproject::load(&arena, project)?;

        ShaderSubproject::ensure_build_directories(&project_ctx)?;

        run_shader_ninja_build(project)?;
        archive_shaders(&project_ctx)?;

        Ok(())
    }
}

fn run_shader_ninja_build(project: &AlephProject) -> anyhow::Result<()> {
    fn push_path_if_tool_exists(v: &mut String, tool: &Utf8Path) {
        if tool.exists() {
            let dir = dunce_utf8::simplified(tool.parent().unwrap());
            log::trace!("Tool found!: '{}'", tool);
            push_path_str(v, dir.as_str());
        } else {
            log::trace!("Tool is missing!: '{}'", tool);
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
        dunce_utf8::simplified(ninja)
    } else {
        Utf8Path::new("ninja")
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

fn archive_shaders(project_ctx: &ShaderProjectContext) -> anyhow::Result<()> {
    let shader_db_file = project_ctx.meta.output_root.join("shaders.shaderdb");
    log::info!(
        "Compiling ShaderDatabase '{}'",
        dunce_utf8::simplified(&shader_db_file)
    );

    let mut shader_db = aleph_shader_db::ShaderDatabase::default();

    for crate_ctx in project_ctx.crates.iter() {
        archive_shaders_for_package(&mut shader_db, crate_ctx)?;
    }

    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&shader_db).unwrap();

    std::fs::write(shader_db_file, bytes)?;

    Ok(())
}

fn archive_shaders_for_package(
    shader_db: &mut aleph_shader_db::ShaderDatabase,
    crate_ctx: &ShaderCrateContext,
) -> anyhow::Result<()> {
    log::info!(
        "Collecting shaders for package {}",
        crate_ctx.meta.output_name
    );

    for module_ctx in crate_ctx.modules.iter() {
        log::info!(
            "Collecting shaders for package {}@{}",
            crate_ctx.meta.output_name,
            module_ctx.module_name,
        );

        let walker = ignore::WalkBuilder::new(module_ctx.meta.output_dir).build();
        for entry in walker.flatten() {
            // We will only process utf-8 paths because we are sane little crewmates. If it's
            // not utf-8 we're in for sadness
            let entry_path = match Utf8PathBuf::from_path_buf(entry.path().to_path_buf()) {
                Ok(v) => v,
                Err(_) => continue,
            };

            let file_type = entry.file_type().unwrap();
            if !file_type.is_file() {
                continue;
            }

            let shader_file = match ShaderFile::new_binary(&entry_path) {
                Some(v) => v,
                None => continue,
            };

            let shader_name = super::shader_name_for_bin_file_in_module(module_ctx, &shader_file)?;

            match shader_file.file_ext {
                crate::shader_system::ShaderFileFormat::Hlsl => unreachable!(),
                crate::shader_system::ShaderFileFormat::Slang => unreachable!(),
                crate::shader_system::ShaderFileFormat::Dxil => {
                    log::trace!("Collecting DXIL for shader '{shader_name}'");
                    let file_data = std::fs::read(&entry_path)?;
                    if let Some(db_entry) = shader_db.shaders.get_mut(&shader_name) {
                        db_entry.dxil = file_data;
                    } else {
                        shader_db.shaders.insert(
                            shader_name,
                            aleph_shader_db::ShaderEntry {
                                shader_type: shader_file.shader_type.into(),
                                spirv: Vec::new(),
                                dxil: file_data,
                            },
                        );
                    }
                }
                crate::shader_system::ShaderFileFormat::Spirv => {
                    log::trace!("Collecting SPIRV for shader '{shader_name}'");
                    let file_data = std::fs::read(&entry_path)?;
                    if let Some(db_entry) = shader_db.shaders.get_mut(&shader_name) {
                        db_entry.spirv = file_data;
                    } else {
                        shader_db.shaders.insert(
                            shader_name,
                            aleph_shader_db::ShaderEntry {
                                shader_type: shader_file.shader_type.into(),
                                spirv: file_data,
                                dxil: Vec::new(),
                            },
                        );
                    }
                }
            }
        }
    }

    Ok(())
}
