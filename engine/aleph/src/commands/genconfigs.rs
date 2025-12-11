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
use std::process::Command;

use aleph_alloc::{BVec, Blink};
use anyhow::anyhow;
use camino::Utf8Path;
use clap::ArgMatches;

use crate::commands::ISubcommand;
use crate::config_subproject::{ConfigProjectContext, ConfigSubproject};
use crate::project::AlephProject;
use crate::templates::{TypescriptReference, configs_tsconfig_template, root_tsconfig_template};
use crate::utils::dunce_utf8;

pub struct GenConfigs {}

impl ISubcommand for GenConfigs {
    fn name(&self) -> &'static str {
        "genconfigs"
    }

    fn description(&mut self) -> clap::Command {
        clap::Command::new(self.name())
            .about("Generates the configs into the output format the engine can consume")
            .long_about("Generates the configs into the output format the engine can consume. Prepares each crate's config.js into a specific folder and file structure inside .aleph")
    }

    fn exec(&mut self, project: &AlephProject, _matches: ArgMatches) -> anyhow::Result<()> {
        let arena = Blink::new();
        let project_ctx = ConfigSubproject::load(&arena, project)?;

        // Generate tsconfig.json files (root config referencing generated configs in .aleph)
        generate_tsconfigs(&arena, project, &project_ctx)?;

        // Build everything with tsc
        run_tsc_with_node(project)?;

        Ok(())
    }
}

fn run_tsc_with_node(project: &AlephProject) -> anyhow::Result<()> {
    // If we have a bundled node exe use that, otherwise just rely on what's in the path
    let node = project.node_path();
    let node = if node.exists() {
        log::trace!("Found 'node' in our SDK at '{node}'.");
        dunce_utf8::simplified(node)
    } else {
        log::warn!("Failed to find 'node' in our SDK! Relying on system path instead");
        Utf8Path::new("node")
    };

    // We _do_ need to use our bundled tsc though, that's not going to be in the path
    let tsc = project.tsc_path();
    let tsc = dunce_utf8::simplified(tsc);
    if !tsc.exists() {
        log::error!("tsc.js not found!");
        return Err(anyhow!("tsc.js not found!"));
    }

    // We specify the tsconfig directly so we aren't ambiguous about which one we're building
    let root_tsconfig_path = project.project_root().join("tsconfig.json");

    let mut command = Command::new(node);
    command.arg(tsc);
    command.arg("--build");
    command.arg(&root_tsconfig_path);

    log::info!("Running TSC with {:?}", &command);
    let status = command.status()?;
    if !status.success() {
        log::error!("tsc invocation failed! Terminating.");
        return Err(anyhow!("tsc invocation failed!"));
    }

    Ok(())
}

fn render_config_tsconfig(
    configs: &[(&str, &Utf8Path)],
    overrides: &[(&str, &Utf8Path)],
    out_file: &Utf8Path,
) -> anyhow::Result<String> {
    let mut config = configs_tsconfig_template();

    let includes = Vec::from_iter(
        configs
            .iter()
            .chain(overrides)
            .map(|(_, c)| dunce_utf8::simplified(c).to_string()),
    );
    config.include = Some(includes);
    config.compiler_options.out_file = Some(dunce_utf8::simplified(out_file).to_string());

    let rendered = serde_json::to_string_pretty(&config)?;
    Ok(rendered)
}

fn render_root_tsconfig(projects: &[&Utf8Path]) -> anyhow::Result<String> {
    let mut config = root_tsconfig_template();

    let references = Vec::from_iter(projects.iter().map(|p| TypescriptReference {
        path: dunce_utf8::simplified(p).to_string(),
    }));
    config.references = Some(references);

    let rendered = serde_json::to_string_pretty(&config)?;
    Ok(rendered)
}

fn generate_tsconfigs(
    arena: &Blink,
    project: &AlephProject,
    project_ctx: &ConfigProjectContext,
) -> anyhow::Result<()> {
    // Nuke the './aleph/configs' directory to get a clean rebuild
    match std::fs::remove_dir_all(project_ctx.meta.configs_root) {
        Ok(_) => {}
        Err(e) => {
            if e.kind() != std::io::ErrorKind::NotFound {
                return Err(e.into());
            }
        }
    }

    ConfigSubproject::ensure_build_directories(&project_ctx)?;

    let mut declared_configs = HashSet::new();
    for c in project_ctx.crates.iter() {
        for config in c.meta.config_names {
            if !declared_configs.insert(*config) {
                log::error!("Config name '{}' declared by multiple crates!", config);
                return Err(anyhow!(
                    "Config name '{}' declared by multiple crates!",
                    config
                ));
            }
        }
    }

    // Collect the plain configs and sorted configs into separate groups as they must be
    // arranged in a specific order in the output tsconfig
    let mut plain_configs = BVec::with_capacity_in(16, arena.allocator());
    let mut sorted_overrides = BVec::with_capacity_in(16, arena.allocator());
    for c in project_ctx.crates.iter() {
        let src = c
            .meta
            .config_names
            .iter()
            .copied()
            .zip(c.meta.configs.iter().copied());
        plain_configs.extend(src);

        let src = c
            .meta
            .override_names
            .iter()
            .copied()
            .zip(c.meta.overrides.iter().copied());
        sorted_overrides.extend(src);
    }

    // Overrides are sorted alphabetically to produce a controllable order in which they run.
    // They are always run _after_ all plain configs.
    sorted_overrides.sort_by_key(|(name, _)| *name);

    // Render the config build tsconfig.json file
    let out_file = project.configs_build_path().join("bundled.js");
    let rendered = render_config_tsconfig(&plain_configs, &sorted_overrides, &out_file)?;

    // And write the rendered config
    let config_tsconfig_path = project.configs_build_path().join("tsconfig.json");
    log::info!("Outputting '{config_tsconfig_path}'");
    std::fs::write(&config_tsconfig_path, rendered)?;

    // And finally, render the root tsconfig that references all our tsconfig subprojects.
    let rendered = render_root_tsconfig(&[&config_tsconfig_path])?;
    let root_tsconfig_path = project.project_root().join("tsconfig.json");
    log::info!("Outputting '{root_tsconfig_path}'");
    std::fs::write(root_tsconfig_path, rendered)?;

    Ok(())
}
