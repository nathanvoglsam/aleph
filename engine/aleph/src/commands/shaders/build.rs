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

use std::collections::hash_map::Entry;

use aleph_alloc::Blink;
use aleph_shader_db::{ParameterBlockDesc, ShaderDatabase, ShaderEntry};
use anyhow::anyhow;
use camino::{Utf8Path, Utf8PathBuf};
use clap::ArgMatches;

use crate::commands::{ISubcommand, config_arg, platform_arg};
use crate::project::AlephProject;
use crate::shader_system::{
    ShaderCrateContext, ShaderFile, ShaderProjectContext, ShaderSubproject,
};
use crate::utils::{BuildPlatform, Profile, dunce_utf8};

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
        let arena = Blink::new();
        let project_ctx = ShaderSubproject::load(&arena, project)?;

        ShaderSubproject::ensure_build_directories(&project_ctx)?;

        run_shader_ninja_build(project)?;
        archive_shaders(&project_ctx)?;

        Ok(())
    }
}

fn run_shader_ninja_build(project: &AlephProject) -> anyhow::Result<()> {
    // If we have a bundled ninja exe use that, otherwise just rely on what's in the path
    let ninja = project.ninja_path();
    let ninja = if ninja.exists() {
        dunce_utf8::simplified(ninja)
    } else {
        Utf8Path::new("ninja")
    };

    let mut command = std::process::Command::new(ninja);
    command.current_dir(project.shader_build_path());

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

    let mut shader_db = ShaderDatabase::default();

    for crate_ctx in project_ctx.crates.iter() {
        archive_shaders_for_package(&mut shader_db, crate_ctx)?;
    }

    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&shader_db).unwrap();

    std::fs::write(shader_db_file, bytes)?;

    Ok(())
}

fn archive_shaders_for_package(
    shader_db: &mut ShaderDatabase,
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

        // We disable any ignore file filtering as that shouldn't have any affect on our shader build
        // system.
        let walker = ignore::WalkBuilder::new(module_ctx.meta.output_dir)
            .ignore(false)
            .git_ignore(false)
            .git_global(false)
            .git_exclude(false)
            .build();
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

            log::trace!(
                "Collecting Reflection for '{}' shader: '{}'",
                shader_file.file_ext,
                shader_name,
            );
            let refl_data = std::fs::read_to_string(&shader_file.reflection_path)?;
            let mut refl = serde_json::from_str::<aleph_slang_reflection::Root>(&refl_data)?;
            refl.normalize();
            let (parameter_blocks, push_constants) =
                crate::shader_system::reflection::build_shader_db_reflection(&refl).unwrap();

            let [cx, cy, cz] = refl.entry_points[0].thread_group_size.unwrap_or([0; 3]);

            log::trace!(
                "Collecting Data for '{}' shader:  '{}'",
                shader_file.file_ext,
                shader_name,
            );
            let file_data = std::fs::read(shader_file.path)?;
            let entry = insert_or_init_entry(
                shader_db,
                &shader_file,
                shader_name,
                parameter_blocks,
                push_constants,
                (cx, cy, cz),
            );

            match shader_file.file_ext {
                crate::shader_system::ShaderFileFormat::Hlsl => unreachable!(),
                crate::shader_system::ShaderFileFormat::Slang => unreachable!(),
                crate::shader_system::ShaderFileFormat::Dxil => entry.dxil = Some(file_data),
                crate::shader_system::ShaderFileFormat::Spirv => entry.spirv = Some(file_data),
                crate::shader_system::ShaderFileFormat::Msl => entry.msl = Some(file_data),
            }
        }
    }

    Ok(())
}

fn insert_or_init_entry<'a>(
    shader_db: &'a mut ShaderDatabase,
    shader_file: &ShaderFile,
    shader_name: String,
    parameter_blocks: Vec<ParameterBlockDesc>,
    push_constants: Option<u64>,
    compute_workgroup_size: (u32, u32, u32),
) -> &'a mut ShaderEntry {
    let entry = match shader_db.shaders.entry(shader_name) {
        Entry::Occupied(entry) => {
            let entry = entry.into_mut();
            assert_eq!(
                &entry.push_constants, &push_constants,
                "Push constants mismatch"
            );
            assert_eq!(
                &entry.parameter_blocks, &parameter_blocks,
                "Parameter blocks mismatch"
            );
            entry
        }
        Entry::Vacant(entry) => entry.insert(ShaderEntry {
            shader_type: shader_file.shader_type.into(),
            parameter_blocks,
            push_constants,
            compute_workgroup_size,
            ..Default::default()
        }),
    };
    entry
}
