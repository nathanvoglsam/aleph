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

use anyhow::anyhow;
use blink_alloc::Blink;
use clap::ArgMatches;
use tera::{Context, Tera};

use crate::commands::ISubcommand;
use crate::config_subproject::ConfigSubproject;
use crate::project::AlephProject;
use crate::templates::JS_CONFIG_TEMPLATE;

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

    fn exec(&mut self, project: &AlephProject, mut _matches: ArgMatches) -> anyhow::Result<()> {
        let arena = Blink::new();

        let project_ctx = ConfigSubproject::load(&arena, project)?;

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
            if c.meta.defs_only {
                continue;
            }

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

        for c in project_ctx.crates.iter() {
            if c.meta.defs_only {
                continue;
            }

            for config in c.meta.configs {
                log::info!("Creating Symlink {} -> {}", config.src, config.dst);
                // Symlink the config so we don't need to call this command every time we change a
                // config
                #[cfg(windows)]
                {
                    std::os::windows::fs::symlink_file(config.src, config.dst)?;
                }

                #[cfg(not(windows))]
                {
                    std::os::unix::fs::symlink(config.src, config.dst)?;
                }
            }
        }

        let mut includes = String::new();
        let mut crates = project_ctx.crates.iter().peekable();
        while let Some(c) = crates.next() {
            if !c.meta.configs.is_empty() || c.meta.defs_only {
                let include = c.meta.config_dir.join("*");
                let include = include.as_str().replace('\\', "\\\\");
                use std::fmt::Write;
                if crates.peek().is_some() {
                    writeln!(&mut includes, "    \"{}\",", include)?;
                } else {
                    write!(&mut includes, "    \"{}\"", include)?;
                }
            }
        }

        let mut context = Context::new();
        context.insert("ALEPH_INCLUDES", &includes);

        let mut tera = Tera::default();
        tera.add_raw_template("a", JS_CONFIG_TEMPLATE)?;
        let rendered = tera.render("a", &context)?;

        log::info!("Outputting 'jsconfig.json'");
        std::fs::write(project.project_root().join("jsconfig.json"), rendered)?;

        Ok(())
    }
}
