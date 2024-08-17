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

use std::borrow::Cow;

use aleph_target::Profile;
use anyhow::anyhow;
use anyhow::Context;
use bumpalo::Bump;
use camino::Utf8Path;
use clap::ArgMatches;
use rayon::prelude::*;

use crate::commands::{config_arg, ISubcommand};
use crate::haxe::{
    ClasspathBundle, HaxeCrateContext, HaxeHlDefinition, HaxeJsDefinition, HaxeModuleContext,
    HaxeModuleDefinitionFile, HaxeProjectContext, HaxeSubproject,
};
use crate::project::AlephProject;
use crate::templates::HAXE_VS_CODE_BUILD_HXML_PREFIX;
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

        // Generate the hxml for building the config override script
        generate_game_config_build_hxml(project, &library_classpaths)?;

        let all_classpaths = HaxeModuleJob::collect_all_classpaths(&jobs);
        generate_vscode_build_hxml(project, &all_classpaths, HaxeTarget::JavaScript)?;
        generate_vscode_build_hxml(project, &all_classpaths, HaxeTarget::HashLink)?;

        Ok(())
    }
}

struct HaxeModuleJob<'a> {
    module_ctx: &'a HaxeModuleContext<'a>,
    module_toml: Option<HaxeModuleDefinitionFile>,
}

impl<'a> HaxeModuleJob<'a> {
    pub fn jobs_for_project(project_ctx: &'a HaxeProjectContext<'a>) -> Vec<HaxeModuleJob<'a>> {
        let mut jobs = Vec::new();
        for crate_ctx in project_ctx.crates {
            for module_ctx in crate_ctx.modules {
                jobs.push(Self {
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
                log::error!("{} {}", error, error.root_cause());
            }
            return Err(anyhow!("'load_toml_for_jobs' failed!'"));
        }

        Ok(())
    }

    pub fn collect_all_classpaths(jobs: &[HaxeModuleJob<'a>]) -> ClasspathBundle<'a> {
        let mut bundle = ClasspathBundle::default();
        for j in jobs {
            let module = &j.module_toml.as_ref().unwrap();

            let in_hl_classpath = module.hl.library || module.hl.package;
            let in_js_classpath = module.js.library || module.js.config_script;

            if in_hl_classpath {
                bundle.hl.push(j.module_ctx.meta.source_dir);
            }

            if in_js_classpath {
                bundle.js.push(j.module_ctx.meta.source_dir);
            }
        }
        bundle
    }

    pub fn collect_library_classpaths(jobs: &[HaxeModuleJob<'a>]) -> ClasspathBundle<'a> {
        let mut bundle = ClasspathBundle::default();
        for j in jobs {
            let module = &j.module_toml.as_ref().unwrap();

            if module.hl.library {
                bundle.hl.push(j.module_ctx.meta.source_dir);
            }
            if module.js.library {
                bundle.js.push(j.module_ctx.meta.source_dir);
            }
        }
        bundle
    }

    pub fn load_toml(&mut self) -> anyhow::Result<()> {
        let path = self.module_ctx.meta.toml_file;
        let module_toml = std::fs::read_to_string(path)
            .with_context(|| format!("Failed loading '{}'.", dunce_utf8::simplified(path)))?;
        let module_toml = toml::from_str(&module_toml)?;
        self.module_toml = Some(module_toml);
        Ok(())
    }
}

fn generate_project_build_hxmls(
    project_ctx: &HaxeProjectContext,
    classpaths: &ClasspathBundle,
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
    classpaths: &ClasspathBundle,
) -> anyhow::Result<()> {
    for module_ctx in crate_ctx.modules {
        generate_module_build_hxmls(crate_ctx, module_ctx, classpaths)?;
    }
    Ok(())
}

fn generate_module_build_hxmls(
    crate_ctx: &HaxeCrateContext,
    module_ctx: &HaxeModuleContext,
    classpaths: &ClasspathBundle,
) -> anyhow::Result<()> {
    let module_toml = std::fs::read_to_string(module_ctx.meta.toml_file)?;
    let HaxeModuleDefinitionFile { hl, js, .. } = toml::from_str(&module_toml)?;

    generate_module_build_hl_hxml(crate_ctx, module_ctx, classpaths, &hl)?;
    generate_module_build_js_hxml(crate_ctx, module_ctx, classpaths, &js)?;

    Ok(())
}

fn generate_module_build_hl_hxml(
    crate_ctx: &HaxeCrateContext,
    module_ctx: &HaxeModuleContext,
    classpaths: &ClasspathBundle,
    lua: &HaxeHlDefinition,
) -> anyhow::Result<()> {
    use std::fmt::Write;

    // Early exit if we shouldn't generate a package
    if !lua.package {
        return Ok(());
    }

    log::info!(
        "Generating lua build.hxml for '{}@{}'",
        crate_ctx.meta.output_name,
        module_ctx.module_name
    );

    let out_dir = module_ctx.meta.output_dir.join("hl");
    let out_file_name = out_dir.join("out.hl");
    let xml_file_name = out_dir.join("out.xml");

    let mut hxml = String::with_capacity(1024);

    for path in classpaths.hl.iter().copied() {
        writeln!(hxml, "--class-path \"{}\"", path_for_haxe(path))?;
    }
    if !lua.library {
        // If the module is not marked as a library then it won't already be present in the
        // 'classpaths' bundle. We'll have to add it ourselves. If we did this unconditionally
        // we would end up with the module added to the classpath twice
        writeln!(
            hxml,
            "--class-path \"{}\"",
            path_for_haxe(module_ctx.meta.source_dir)
        )?;
    }
    writeln!(hxml, "--dce std")?;
    writeln!(hxml, "-D hl-ver=1.14.0")?;
    {
        write!(hxml, "--macro include(\"\", true, [], [")?;
        write!(hxml, "\"{}\", ", path_for_haxe(module_ctx.meta.source_dir))?;
        writeln!(hxml, "])",)?;
    }
    writeln!(hxml, "--hl \"{}\"", path_for_haxe(&out_file_name))?;
    writeln!(hxml, "--xml \"{}\"", path_for_haxe(&xml_file_name))?;

    std::fs::write(module_ctx.meta.build_hl_file, hxml)?;

    Ok(())
}

fn generate_module_build_js_hxml(
    crate_ctx: &HaxeCrateContext,
    module_ctx: &HaxeModuleContext,
    classpaths: &ClasspathBundle,
    js: &HaxeJsDefinition,
) -> anyhow::Result<()> {
    use std::fmt::Write;

    // Early exit if we shouldn't generate a package
    if !js.config_script {
        return Ok(());
    }

    log::info!(
        "Generating js build.hxml for '{}@{}'",
        crate_ctx.meta.output_name,
        module_ctx.module_name
    );

    let mut hxml = String::with_capacity(1024);

    for path in classpaths.js.iter().copied() {
        writeln!(hxml, "--class-path \"{}\"", path_for_haxe(path))?;
    }
    if !js.library {
        // If the module is not marked as a library then it won't already be present in the
        // 'classpaths' bundle. We'll have to add it ourselves. If we did this unconditionally
        // we would end up with the module added to the classpath twice
        writeln!(
            hxml,
            "--class-path \"{}\"",
            path_for_haxe(module_ctx.meta.source_dir)
        )?;
    }
    writeln!(hxml, "--dce std")?;
    writeln!(hxml, "-D js-es=6")?;
    {
        write!(hxml, "--macro include(\"\", true, [], [")?;
        write!(hxml, "\"{}\", ", path_for_haxe(module_ctx.meta.source_dir))?;
        writeln!(hxml, "])",)?;
    }
    writeln!(
        hxml,
        "--js \"{}\"",
        path_for_haxe(module_ctx.meta.output_config_script)
    )?;

    std::fs::write(module_ctx.meta.build_js_file, hxml)?;

    Ok(())
}

enum HaxeTarget {
    JavaScript,
    HashLink,
}

impl HaxeTarget {
    fn select_classpath_from_bundle<'a>(
        &self,
        bundle: &'a ClasspathBundle<'a>,
    ) -> &'a [&'a Utf8Path] {
        match self {
            HaxeTarget::JavaScript => bundle.js.as_slice(),
            HaxeTarget::HashLink => bundle.hl.as_slice(),
        }
    }

    fn append_target_args_to_hxml(&self, target: &mut String) -> std::fmt::Result {
        use std::fmt::Write;

        match self {
            HaxeTarget::JavaScript => {
                writeln!(target, "-D js-es=6")?;
            }
            HaxeTarget::HashLink => {
                writeln!(target, "-D hl-ver=1.14.0")?;
            }
        }
        Ok(())
    }

    const fn out_file_ext(&self) -> &'static str {
        match self {
            HaxeTarget::JavaScript => "js",
            HaxeTarget::HashLink => "hl",
        }
    }

    const fn out_arg_name(&self) -> &'static str {
        match self {
            HaxeTarget::JavaScript => "js",
            HaxeTarget::HashLink => "hl",
        }
    }

    const fn vscode_hxml_name(&self) -> &'static str {
        match self {
            HaxeTarget::JavaScript => "build_js.hxml",
            HaxeTarget::HashLink => "build_hl.hxml",
        }
    }
}

fn generate_vscode_build_hxml(
    project: &AlephProject,
    classpaths: &ClasspathBundle,
    target: HaxeTarget,
) -> anyhow::Result<()> {
    use std::fmt::Write;

    log::info!(
        "Generating '{}' for language server",
        target.vscode_hxml_name()
    );

    // Stamp out the prefix and any target specific args
    let mut hxml = HAXE_VS_CODE_BUILD_HXML_PREFIX.to_string();
    target.append_target_args_to_hxml(&mut hxml)?;

    // Write out the library class paths first
    let target_classpaths = target.select_classpath_from_bundle(classpaths);
    for path in target_classpaths.iter().copied() {
        writeln!(hxml, "--class-path \"{}\"", path_for_haxe(path))?;
    }

    // Write out the class path for the game's config override
    let (game_crate, _) = project.get_game_crate_and_target()?;
    let cfg_classpath = game_crate.manifest_path.parent().unwrap();
    let cfg_classpath = cfg_classpath.join("config");
    writeln!(hxml, "--class-path \"{}\"", path_for_haxe(&cfg_classpath))?;

    // Write out the dummy target output declaration. This is how the language server knows what
    // haxe target we're using
    let dummy_output_file = project
        .haxe_build_path()
        .join("dummy")
        .with_extension(target.out_file_ext());
    let dummy_output_file = path_for_haxe(&dummy_output_file);
    let dummy_target = target.out_arg_name();
    writeln!(hxml, "--{dummy_target} \"{dummy_output_file}\"")?;

    // And finally write the file out
    let hxml_file_name = project.project_root().join(target.vscode_hxml_name());
    std::fs::write(hxml_file_name, hxml)?;

    Ok(())
}

fn generate_game_config_build_hxml(
    project: &AlephProject,
    classpaths: &ClasspathBundle,
) -> anyhow::Result<()> {
    use std::fmt::Write;

    let (game_crate, _) = project.get_game_crate_and_target()?;

    let cfg_classpath = game_crate.manifest_path.parent().unwrap();
    let cfg_classpath = cfg_classpath.join("config");

    let out_script_file = project.config_build_path().join("@overrides.js");
    let out_hxml_file = project.haxe_build_path().join("build_cfg_overrides.hxml");

    log::info!(
        "Generating build.hxml for '{}' config override script",
        &game_crate.name
    );

    let mut hxml = String::with_capacity(1024);

    for path in classpaths.js.iter().copied() {
        writeln!(hxml, "--class-path \"{}\"", path_for_haxe(path))?;
    }
    writeln!(hxml, "--class-path \"{}\"", path_for_haxe(&cfg_classpath))?;
    writeln!(hxml, "--dce std")?;
    writeln!(hxml, "-D js-es=6")?;
    {
        write!(hxml, "--macro include(\"\", true, [], [")?;
        write!(hxml, "\"{}\", ", path_for_haxe(&cfg_classpath))?;
        writeln!(hxml, "])",)?;
    }
    writeln!(hxml, "--js \"{}\"", path_for_haxe(&out_script_file))?;

    std::fs::write(out_hxml_file, hxml)?;

    Ok(())
}

/// Does the whole 'UNC' discard + convers '\' to '/' for haxe. It's not required on windows, but it
/// does make haxe output better error messages.
fn path_for_haxe(path: &Utf8Path) -> Cow<str> {
    let path = dunce_utf8::simplified(path);
    if cfg!(windows) {
        let path = path.as_str().replace('\\', "/");
        Cow::Owned(path)
    } else {
        Cow::Borrowed(path.as_str())
    }
}
