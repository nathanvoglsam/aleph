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

use std::io::IsTerminal;

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
use crate::utils::dunce_utf8;

pub struct BuildHaxeProj {}

impl ISubcommand for BuildHaxeProj {
    fn name(&self) -> &'static str {
        "build"
    }

    fn description(&mut self) -> clap::Command {
        clap::Command::new(self.name())
            .about("Builds all haxe modules in the crate graph.")
            .long_about("Builds all haxe modules in the crate graph.")
            .arg(config_arg())
    }

    fn exec(&mut self, project: &AlephProject, mut matches: ArgMatches) -> anyhow::Result<()> {
        let profile_arg: String = matches
            .remove_one("profile")
            .expect("profile should have a default");

        let _profile = Profile::from_name(&profile_arg.to_lowercase())
            .ok_or(anyhow!("Unknown profile \"{}\"", &profile_arg))?;

        // Build the base level project context for our shader build system
        let arena = Bump::new();
        let project_ctx = HaxeSubproject::load(&arena, project)?;

        std::fs::create_dir_all(project.config_build_path())?;
        HaxeSubproject::ensure_build_directories(&project_ctx)?;

        build_project_hxmls(project.haxe_path(), &project_ctx)?;
        build_config_override_script_hxml(project)?;

        Ok(())
    }
}

fn build_project_hxmls(
    haxe_path: &Utf8Path,
    project_ctx: &HaxeProjectContext,
) -> anyhow::Result<()> {
    let (error_send, error_recv) = crossbeam::channel::bounded(project_ctx.crates.len());
    project_ctx
        .crates
        .par_iter()
        .for_each_with(
            error_send,
            |error_channel, crate_ctx| match build_crate_hxmls(haxe_path, crate_ctx) {
                Ok(_) => {}
                Err(v) => error_channel.send(v).unwrap(),
            },
        );

    if !error_recv.is_empty() {
        while let Ok(error) = error_recv.try_recv() {
            log::error!("Error while building haxe project!: {}", error);
        }
        return Err(anyhow!("'build_project_hxmls' failed!'"));
    }

    Ok(())
}

fn build_crate_hxmls(haxe_path: &Utf8Path, crate_ctx: &HaxeCrateContext) -> anyhow::Result<()> {
    for module_ctx in crate_ctx.modules {
        build_module_hxmls(haxe_path, crate_ctx, module_ctx)?;
    }
    Ok(())
}

fn build_module_hxmls(
    haxe_path: &Utf8Path,
    crate_ctx: &HaxeCrateContext,
    module_ctx: &HaxeModuleContext,
) -> anyhow::Result<()> {
    let module_toml = std::fs::read_to_string(module_ctx.meta.toml_file)?;
    let HaxeModuleDefinitionFile { hl, js, .. } = toml::from_str(&module_toml)?;

    if hl.package {
        log::info!(
            "Build hl module for '{}@{}'",
            crate_ctx.meta.output_name,
            module_ctx.module_name
        );
        build_hxml(haxe_path, module_ctx.meta.build_hl_file)?;
    }

    if js.config_script {
        log::info!(
            "Build js module for '{}@{}'",
            crate_ctx.meta.output_name,
            module_ctx.module_name
        );
        build_hxml(haxe_path, module_ctx.meta.build_js_file)?;
    }

    Ok(())
}

fn build_config_override_script_hxml(project: &AlephProject) -> anyhow::Result<()> {
    let hxml = project.haxe_build_path().join("build_cfg_overrides.hxml");

    log::info!("Build js module for game config override script");
    build_hxml(project.haxe_path(), &hxml)
}

fn build_hxml(haxe_path: &Utf8Path, path: &Utf8Path) -> anyhow::Result<()> {
    let mut command = std::process::Command::new(haxe_path);
    command.arg(dunce_utf8::simplified(path));
    command.arg("-D");
    command.arg("message.reporting=pretty");

    // If we're not writing to a terminal we want to skip color codes.
    //
    // I'm not sure if haxe does this itself, but just to be sure.
    if !std::io::stdout().is_terminal() {
        command.arg("-D");
        command.arg("message.no-color");
    }

    let status = command.status()?;
    if !status.success() {
        log::error!("Haxe invocation failed! Terminating build.");
        return Err(anyhow!("haxe invocation failed!"));
    }

    Ok(())
}
