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

use aleph_target::Profile;
use anyhow::anyhow;
use bumpalo::Bump;
use camino::Utf8Path;
use clap::ArgMatches;
use rayon::prelude::*;

use crate::commands::{config_arg, ISubcommand};
use crate::haxe::{
    HaxeCrateContext, HaxeModuleContext, HaxeModuleDefinitionFile, HaxeProjectContext,
    HaxeSubproject,
};
use crate::project::AlephProject;
use crate::templates::HAXE_LUA_BUILD_HXML_PREFIX;
use crate::utils::dunce_utf8;

pub struct GenHaxeProj {}

impl ISubcommand for GenHaxeProj {
    fn name(&self) -> &'static str {
        "genproj"
    }

    fn description(&mut self) -> clap::Command {
        clap::Command::new(self.name())
            .about("Generates the build system for the haxe projects within the crate graph.")
            .long_about("Generates the build system for the haxe projects within the crate graph.")
            .arg(config_arg())
    }

    fn exec(&mut self, project: &AlephProject, mut matches: ArgMatches) -> anyhow::Result<()> {
        let profile_arg: String = matches
            .remove_one("profile")
            .expect("profile should have a default");

        let _profile = Profile::from_name(&profile_arg.to_lowercase())
            .ok_or(anyhow!("Unknown profile \"{}\"", &profile_arg))?;

        let arena = Bump::new();

        let project_ctx = HaxeSubproject::load(&arena, project)?;
        HaxeSubproject::ensure_build_directories(&project_ctx)?;

        let mut jobs = HaxeModuleJob::jobs_for_project(&project_ctx);
        HaxeModuleJob::load_toml_for_jobs(&mut jobs)?;

        let library_classpaths = HaxeModuleJob::collect_library_classpaths(&jobs);
        generate_project_build_hxmls(&project_ctx, &library_classpaths)?;

        let all_classpaths = HaxeModuleJob::collect_all_module_classpaths(&jobs);
        generate_vscode_build_hxml(project, &all_classpaths)?;

        Ok(())
    }
}

struct HaxeModuleJob<'a> {
    crate_ctx: &'a HaxeCrateContext<'a>,
    module_ctx: &'a HaxeModuleContext<'a>,
    module_toml: Option<HaxeModuleDefinitionFile<'static>>,
}

impl<'a> HaxeModuleJob<'a> {
    pub fn jobs_for_project(project_ctx: &'a HaxeProjectContext<'a>) -> Vec<HaxeModuleJob<'a>> {
        let mut jobs = Vec::new();
        for crate_ctx in project_ctx.crates {
            for module_ctx in crate_ctx.modules {
                jobs.push(Self {
                    crate_ctx,
                    module_ctx,
                    module_toml: None,
                });
            }
        }
        jobs
    }

    pub fn load_toml_for_jobs(jobs: &mut [HaxeModuleJob<'a>]) -> anyhow::Result<()> {
        let (error_send, error_recv) = crossbeam::channel::bounded(jobs.len());
        jobs.par_iter_mut()
            .for_each_with(error_send, |error_channel, job| match job.load_toml() {
                Ok(_) => {}
                Err(v) => error_channel.send(v).unwrap(),
            });

        if !error_recv.is_empty() {
            while let Ok(error) = error_recv.try_recv() {
                log::error!("Error while loading haxe module toml!: {}", error);
            }
            return Err(anyhow!("'load_toml_for_jobs' failed!'"));
        }

        Ok(())
    }

    pub fn collect_all_module_classpaths(jobs: &[HaxeModuleJob<'a>]) -> Vec<&'a Utf8Path> {
        jobs.iter().map(|v| v.module_ctx.meta.source_dir).collect()
    }

    pub fn collect_library_classpaths(jobs: &[HaxeModuleJob<'a>]) -> Vec<&'a Utf8Path> {
        jobs.iter()
            .filter_map(|v| {
                if v.module_toml.as_ref().unwrap().module.lua.library {
                    Some(v.module_ctx.meta.source_dir)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn load_toml(&mut self) -> anyhow::Result<()> {
        let module_toml = std::fs::read_to_string(self.module_ctx.meta.toml_file)?;
        let module_toml = toml::from_str(&module_toml)?;
        self.module_toml = Some(module_toml);
        Ok(())
    }
}

fn generate_project_build_hxmls(
    project_ctx: &HaxeProjectContext,
    classpaths: &[&Utf8Path],
) -> anyhow::Result<()> {
    let (error_send, error_recv) = crossbeam::channel::bounded(project_ctx.crates.len());
    project_ctx
        .crates
        .par_iter()
        .for_each_with(error_send, |error_channel, crate_ctx| {
            log::info!(
                "Generating build.hxml file for: {}",
                crate_ctx.meta.output_name
            );
            match generate_crate_build_hxmls(crate_ctx, classpaths) {
                Ok(_) => {}
                Err(v) => error_channel.send(v).unwrap(),
            }
        });

    if !error_recv.is_empty() {
        while let Ok(error) = error_recv.try_recv() {
            log::error!("Error while generating haxe project!: {}", error);
        }
        return Err(anyhow!("'generate_project_build_hxmls' failed!'"));
    }

    Ok(())
}

fn generate_crate_build_hxmls(
    crate_ctx: &HaxeCrateContext,
    classpaths: &[&Utf8Path],
) -> anyhow::Result<()> {
    for module_ctx in crate_ctx.modules {
        generate_module_build_hxmls(crate_ctx, module_ctx, classpaths)?;
    }
    Ok(())
}

fn generate_module_build_hxmls(
    crate_ctx: &HaxeCrateContext,
    module_ctx: &HaxeModuleContext,
    classpaths: &[&Utf8Path],
) -> anyhow::Result<()> {
    use std::fmt::Write;

    let module_toml = std::fs::read_to_string(module_ctx.meta.toml_file)?;
    let HaxeModuleDefinitionFile { module } = toml::from_str(&module_toml)?;

    if module.lua.package {
        log::info!(
            "Generating lua build.hxml for '{}@{}'",
            crate_ctx.meta.output_name,
            module_ctx.module_name
        );

        let out_file_name = module_ctx.meta.output_dir.join("out.lua");

        let mut hxml = String::with_capacity(1024);

        for path in classpaths {
            writeln!(hxml, "--class-path \"{}\"", dunce_utf8::simplified(path))?;
        }
        if !module.lua.library {
            // If the module is not marked as a library then it won't already be present in the
            // 'classpaths' bundle. We'll have to add it ourselves. If we did this unconditionally
            // we would end up with the module added to the classpath twice
            writeln!(
                hxml,
                "--class-path \"{}\"",
                dunce_utf8::simplified(module_ctx.meta.source_dir)
            )?;
        }
        writeln!(hxml, "--dce std")?;
        writeln!(hxml, "-D lua-jit")?;
        writeln!(hxml, "-D lua-ver=5.1")?;
        writeln!(
            hxml,
            "--macro include(\"{}\", true)",
            module_ctx.module_name
        )?;
        writeln!(hxml, "--lua \"{}\"", dunce_utf8::simplified(&out_file_name))?;

        std::fs::write(module_ctx.meta.build_xml_file, hxml)?;
    }

    Ok(())
}

fn generate_vscode_build_hxml(
    project: &AlephProject,
    classpaths: &[&Utf8Path],
) -> anyhow::Result<()> {
    use std::fmt::Write;

    log::info!("Generating 'build.xml' for language server");

    let dummy_output_file = project.haxe_build_path().join("dummy.lua");
    let hxml_file_name = project.project_root().join("build.hxml");

    let mut hxml = String::with_capacity(1024);

    writeln!(hxml, "{}", HAXE_LUA_BUILD_HXML_PREFIX)?;
    for path in classpaths {
        writeln!(hxml, "--class-path \"{}\"", dunce_utf8::simplified(path))?;
    }
    writeln!(
        hxml,
        "--lua \"{}\"",
        dunce_utf8::simplified(&dummy_output_file)
    )?;

    std::fs::write(hxml_file_name, hxml)?;

    Ok(())
}
