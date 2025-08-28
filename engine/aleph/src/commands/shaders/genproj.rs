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

use std::io::Write;

use aleph_target::Profile;
use anyhow::anyhow;
use blink_alloc::Blink;
use camino::{Utf8Path, Utf8PathBuf};
use clap::ArgMatches;
use rayon::prelude::*;

use crate::commands::{ISubcommand, config_arg, platform_arg};
use crate::project::AlephProject;
use crate::shader_system::{
    ShaderCompilationParams, ShaderCrateContext, ShaderFile, ShaderModuleContext,
    ShaderModuleDefinition, ShaderModuleDefinitionFile, ShaderProjectContext, ShaderSubproject,
    ShaderTargetLanguage,
};
use crate::utils::{BuildPlatform, dunce_utf8, ninja};

pub struct GenShaderProj {}

impl ISubcommand for GenShaderProj {
    fn name(&self) -> &'static str {
        "genproj"
    }

    fn description(&mut self) -> clap::Command {
        clap::Command::new(self.name())
            .about("Generates the build system for compiling our shader database")
            .long_about("Generates the build system for compiling our shader database")
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

        let platform = BuildPlatform::from_arg(&platform_arg)
            .ok_or(anyhow!("Unknown platform \"{}\"", &platform_arg))?;
        let _profile = Profile::from_name(&profile_arg.to_lowercase())
            .ok_or(anyhow!("Unknown profile \"{}\"", &profile_arg))?;

        let arena = Blink::new();
        let project_ctx = ShaderSubproject::load(&arena, project)?;

        ShaderSubproject::ensure_build_directories(&project_ctx)?;
        ShaderSubproject::ensure_build_files(&project_ctx)?;

        generate_shader_module_ninja_files(platform, &project_ctx)?;
        generate_shader_name_bindings(&project_ctx)?;

        Ok(())
    }
}

fn generate_shader_module_ninja_files(
    platform: BuildPlatform,
    project_ctx: &ShaderProjectContext,
) -> anyhow::Result<()> {
    // Walk through all the dependency packages that have shaders and create a build.ninja
    // file for them so we can compile the shaders using ninja
    let (error_send, error_recv) = crossbeam::channel::bounded(project_ctx.crates.len());
    project_ctx
        .crates
        .par_iter()
        .for_each_with(error_send, |error_channel, crate_ctx| {
            log::info!("Generating Ninja Files For: {}", crate_ctx.meta.output_name,);
            match build_shader_ninja_file_for_package(platform, crate_ctx) {
                Ok(_) => {}
                Err(v) => error_channel.send(v).unwrap(),
            }
        });

    if !error_recv.is_empty() {
        while let Ok(error) = error_recv.try_recv() {
            log::error!("Error while generating shader project!: {:#?}", error);
        }
        return Err(anyhow!("'generate_shader_module_ninja_files' failed!'"));
    }

    let mut build_file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(project_ctx.meta.root_ninja_file)?;
    writeln!(&mut build_file, "include rules.ninja")?;
    writeln!(&mut build_file)?;

    write!(&mut build_file, "includes =")?;
    for crate_ctx in project_ctx.crates.iter() {
        for module_ctx in crate_ctx.modules.iter() {
            write!(
                &mut build_file,
                " -I {}",
                dunce_utf8::simplified(module_ctx.meta.include_dir)
            )?;
        }
    }
    writeln!(&mut build_file)?;
    writeln!(&mut build_file)?;

    for crate_ctx in project_ctx.crates.iter() {
        for module_ctx in crate_ctx.modules.iter() {
            let ninja_file = ninja::prepare_path_for_build_statement(module_ctx.meta.ninja_file);
            writeln!(&mut build_file, "include {}", ninja_file)?;
            writeln!(&mut build_file)?;
        }
    }

    Ok(())
}

fn build_shader_ninja_file_for_package(
    platform: BuildPlatform,
    crate_ctx: &ShaderCrateContext,
) -> anyhow::Result<()> {
    let mut modules = Vec::new();
    for module_ctx in crate_ctx.modules.iter() {
        let module_file = std::fs::read_to_string(module_ctx.meta.toml_file)?;
        let module_file = ShaderModuleDefinitionFile::from_str(&module_file)?;
        modules.push((module_ctx, module_file.module));
    }

    for (module_ctx, module) in modules {
        log::info!(
            "Generating Ninja Files For: {}@{}",
            crate_ctx.meta.output_name,
            module_ctx.module_name
        );
        build_shader_ninja_file_for_shader_module(platform, module_ctx, &module)?;
    }

    Ok(())
}

fn build_shader_ninja_file_for_shader_module(
    platform: BuildPlatform,
    module_ctx: &ShaderModuleContext,
    module: &ShaderModuleDefinition,
) -> anyhow::Result<()> {
    let mut ninja_file = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(module_ctx.meta.ninja_file)?;

    // Compute the compilation params from the module file
    let compilation_params = ShaderCompilationParams::new(module_ctx, module)?;

    // We use a single-threaded walker as we intend to parallelise at the package level instead.
    //
    // We disable any ignore file filtering as that shouldn't have any affect on our shader build
    // system.
    let walker = ignore::WalkBuilder::new(module_ctx.meta.source_dir)
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

        let entry_path_tail = entry_path.strip_prefix(module_ctx.meta.source_dir)?;

        let f_type = entry.file_type().unwrap();
        if f_type.is_file() {
            let shader_file = match ShaderFile::new(&entry_path) {
                Some(v) => v,
                None => continue,
            };

            // Only build dxil on windows, where the full dxil pipeline will be available.
            if platform == BuildPlatform::Windows {
                output_build_statement_for_shader(
                    &mut ninja_file,
                    module_ctx,
                    &compilation_params,
                    &shader_file,
                    ShaderTargetLanguage::Dxil,
                )?;
            }

            output_build_statement_for_shader(
                &mut ninja_file,
                module_ctx,
                &compilation_params,
                &shader_file,
                ShaderTargetLanguage::Spirv,
            )?;

            output_build_statement_for_shader(
                &mut ninja_file,
                module_ctx,
                &compilation_params,
                &shader_file,
                ShaderTargetLanguage::Msl,
            )?;
        }
        if f_type.is_dir() {
            let v = module_ctx.meta.output_dir.join(entry_path_tail);
            std::fs::create_dir_all(v)?;
        }
    }

    Ok(())
}

fn output_build_statement_for_shader(
    ninja_file: &mut std::fs::File,
    module_ctx: &ShaderModuleContext,
    compilation_params: &ShaderCompilationParams,
    shader_file: &ShaderFile,
    target_ir: ShaderTargetLanguage,
) -> anyhow::Result<()> {
    let rule = shader_file.ninja_rule();

    let in_file = shader_file.path;
    let in_file_relative_to_src = in_file.strip_prefix(module_ctx.meta.source_dir)?;
    let in_file_dir_relative_to_src = in_file_relative_to_src.parent().unwrap();

    let out_file = module_ctx
        .meta
        .output_dir
        .join(in_file_dir_relative_to_src)
        .join(shader_file.name_with_type);

    let out_file = ninja::prepare_path_for_build_statement(&out_file);
    let in_file = ninja::prepare_path_for_build_statement(in_file);

    match target_ir {
        crate::shader_system::ShaderTargetLanguage::Dxil => {
            writeln!(ninja_file, "build {out_file}.dxil: {rule}_dxil {in_file}")?;
            compilation_params.write_ninja_overrides(ninja_file)?;
            writeln!(ninja_file)?;
        }
        crate::shader_system::ShaderTargetLanguage::Spirv => {
            writeln!(ninja_file, "build {out_file}.spirv: {rule}_spirv {in_file}")?;
            compilation_params.write_ninja_overrides(ninja_file)?;
            writeln!(ninja_file)?;
        }
        crate::shader_system::ShaderTargetLanguage::Msl => {
            writeln!(ninja_file, "build {out_file}.msl: {rule}_msl {in_file}")?;
            compilation_params.write_ninja_overrides(ninja_file)?;
            writeln!(ninja_file)?;
        }
    }

    Ok(())
}

fn generate_shader_name_bindings(project_ctx: &ShaderProjectContext) -> anyhow::Result<()> {
    let (error_send, error_recv) = crossbeam::channel::bounded(project_ctx.crates.len());
    project_ctx
        .crates
        .par_iter()
        .for_each_with(error_send, |error_channel, crate_ctx| {
            log::info!("Generating Ninja Files For: {}", crate_ctx.meta.output_name);
            match generate_shader_name_bindings_for_package(crate_ctx) {
                Ok(_) => {}
                Err(v) => error_channel.send(v).unwrap(),
            }
        });

    if !error_recv.is_empty() {
        while let Ok(error) = error_recv.try_recv() {
            log::error!("Error while generating shader name bindings!: {:#?}", error);
        }
        return Err(anyhow!("'generate_shader_name_bindings' failed!'"));
    }

    Ok(())
}

fn generate_shader_name_bindings_for_package(crate_ctx: &ShaderCrateContext) -> anyhow::Result<()> {
    for module_ctx in crate_ctx.modules.iter() {
        log::info!(
            "Generating Shader Name Bindings For: {}@{}",
            crate_ctx.meta.output_name,
            module_ctx.module_name
        );
        generate_shader_name_bindings_for_module(crate_ctx, module_ctx)?;
    }
    Ok(())
}

fn generate_shader_name_bindings_for_module(
    crate_ctx: &ShaderCrateContext,
    module_ctx: &ShaderModuleContext,
) -> anyhow::Result<()> {
    use std::fmt::Write;
    let mut output = String::new();

    writeln!(&mut output, "// Do not edit manually! File is GENERATED!")?;
    writeln!(&mut output)?;

    write_imports(&mut output, "")?;

    let mut indent = String::with_capacity(32);
    let mut stack = Vec::new();
    stack.push(sorted_dir_listing(module_ctx.meta.source_dir)?);
    'outer: while let Some(mut item) = stack.pop() {
        while let Some((item_type, item_path)) = item.next() {
            let file_name = item_path.file_name().unwrap();
            if item_type.is_file() {
                // Just skip anything that we don't identify as a valid shader file
                let shader_file = match ShaderFile::new(item_path.as_path()) {
                    Some(v) => v,
                    None => continue,
                };

                // Output the function declaration
                let file_stem_no_dots = shader_file.name_with_type.replace(&['.', '-'], "_");
                writeln!(&mut output, "{indent}#[allow(unused, non_snake_case)]")?;
                writeln!(
                    &mut output,
                    "{indent}pub const fn {file_stem_no_dots}() -> {} {{",
                    shader_file.shader_type.shader_db_name_type()
                )?;

                // Output the function body
                let shader_name =
                    super::shader_name_for_src_file_in_module(module_ctx, &shader_file)?;
                writeln!(
                    &mut output,
                    "{indent}    unsafe {{ {}(\"{shader_name}\") }} // Safety guaranteed by code-gen",
                    shader_file.shader_type.shader_db_name_constructor(),
                )?;

                // Close the function block
                writeln!(&mut output, "{indent}}}")?;

                log::trace!("{}", shader_name);
            } else if item_type.is_dir() {
                // If we find another nested directory we open a new module and start iterating the
                // new directory.

                // Open the new module in the file
                let sanitized_dir_name = file_name.replace(&['.', '-'], "_");
                writeln!(&mut output, "{indent}#[allow(unused, non_snake_case)]")?;
                writeln!(&mut output, "{indent}pub mod {sanitized_dir_name} {{")?;

                // Increase our indent level
                indent.push_str("    ");

                write_imports(&mut output, &indent)?;

                // Store our progress through the current directory and then push the newly found
                // child directory as the next element to process onto the stack.
                stack.push(item);
                stack.push(sorted_dir_listing(item_path.as_path())?);

                // Exit from the inner loop so we can iterate the child directory
                continue 'outer;
            }
        }

        // We don't close the module scope on the final item
        if !stack.is_empty() {
            // If we hit here we've finished iterating a dir, so for every level except the very
            // bottom level we close an (assumed to be open) module and decrease the indent level.
            indent.truncate(indent.len() - 4);
            writeln!(&mut output, "{indent}}}",)?;
        }
    }

    let module_name = module_ctx.module_name.replace(&['.', '-'], "_");
    let module_output_file = crate_ctx
        .meta
        .shader_dir
        .join(module_name)
        .with_extension("rs")
        .to_string();
    match std::fs::read_to_string(&module_output_file) {
        Ok(existing_text) => {
            // Only write the file if the new output is different than the existing contents of the
            // file. This prevents the mtime from being updated. If we re-write the contents of the
            // file it will trigger cargo to rebuild the crate, which we want to avoid if we know
            // the file is unchanged.
            if existing_text != output {
                std::fs::write(&module_output_file, output)?;
            }
        }
        Err(e) => {
            // If there's no file we just create it, otherwise throw the error out to the caller.
            if e.kind() == std::io::ErrorKind::NotFound {
                std::fs::write(&module_output_file, output)?;
            } else {
                Err(e)?;
            }
        }
    }

    Ok(())
}

type SortedDirListingIter = std::vec::IntoIter<(std::fs::FileType, Utf8PathBuf)>;

fn sorted_dir_listing(item: &Utf8Path) -> std::io::Result<SortedDirListingIter> {
    // We want our read dirs to be sorted (stably) so we can guarantee that different platforms will
    // generate the same code. Not all platforms will yield the same order of elements for a
    // read_dir walk (Windows is already sorted, unixes aren't) so we sort them ourselves.
    //
    // This adds some overhead, but is worth it for avoiding redundant diffs when generating
    // bindings on different platforms.
    let iter = item.read_dir_utf8()?;
    let mut dir_items: Vec<(std::fs::FileType, Utf8PathBuf)> = Vec::new();
    for i in iter {
        let i = i?;
        let file_type = i.file_type()?;
        let path_buf = i.path().to_path_buf();
        dir_items.push((file_type, path_buf));
    }
    dir_items.sort_by(|l, r| l.1.cmp(&r.1));
    Ok(dir_items.into_iter())
}

fn write_imports(output: &mut impl std::fmt::Write, indent: &str) -> std::fmt::Result {
    writeln!(output, "{indent}#[allow(unused)]")?;
    writeln!(
        output,
        "{indent}use aleph_shader_db::{{ Amplification, Compute, Domain, Fragment, Geometry, Hull, Mesh, ShaderName, Vertex }};"
    )?;
    writeln!(output)?;
    Ok(())
}
