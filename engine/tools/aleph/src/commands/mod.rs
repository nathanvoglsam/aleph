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

mod build;
mod bundle;
mod genconfigs;
mod genproj;
mod genvscode;
pub mod shaders;
mod uuid;

pub use build::Build;
pub use bundle::Bundle;
use clap::{Arg, ArgMatches, Command};
pub use genconfigs::GenConfigs;
pub use genproj::GenProj;
pub use genvscode::GenVsCode;
pub use uuid::Uuid;

use crate::project::AlephProject;

pub trait ISubcommand {
    fn name(&self) -> &'static str;

    fn description(&mut self) -> Command;

    fn exec(&mut self, project: &AlephProject, matches: ArgMatches) -> anyhow::Result<()>;

    /// Allows a subcommand to request the subcommand processing system to not log anything so as
    /// to not pollute stdout. Useful for tools that write to stdout as their primary output.
    fn dont_log(&self) -> bool {
        false
    }
}

pub struct SubcommandSet {
    name: &'static str,
    about: &'static str,
    subcommands: std::collections::HashMap<String, Box<dyn ISubcommand>>,
}

impl SubcommandSet {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            about: "",
            subcommands: Default::default(),
        }
    }

    pub fn about(mut self, about: &'static str) -> Self {
        self.about = about;
        self
    }

    pub fn register_subcommand(&mut self, subcommand: impl ISubcommand + 'static) {
        let name = subcommand.name();
        self.subcommands
            .insert(name.to_string(), Box::new(subcommand));
    }

    pub fn exec_as_root(&mut self) -> anyhow::Result<()> {
        use anyhow::Context;

        let command = self.description();
        let command = command
            .author(env!("CARGO_PKG_AUTHORS"))
            .version(env!("CARGO_PKG_VERSION"))
            .about("The aleph-engine meta tool for managing an aleph-engine project")
            .arg_required_else_help(true);
        let matches = command.get_matches();
        if let Some((subcommand_name, matches)) = matches.subcommand() {
            if let Some(subcommand) = self.subcommands.get_mut(subcommand_name) {
                if !subcommand.dont_log() {
                    // We only want to initialize the logger until _after_ we've started handling
                    // subcommands so we don't get people (somehow) logging in the middle of the
                    // command info dump.
                    //
                    // We also only want to enable the logger when the subcommand allows it in case
                    // the subcommand wants to output stuff to stdout.
                    env_logger::builder()
                        .filter_level(log::LevelFilter::Trace)
                        .init();
                }

                let arena = bumpalo::Bump::new();

                // Finds the 'aleph-project.toml' and deduces all the project directories against the
                // active project.
                let project = AlephProject::new(&arena).context("Loading project information")?;

                // Now we can run the command
                let result = subcommand.exec(&project, matches.clone());
                if result.is_ok() {
                    log::info!("Subcommand {subcommand_name} completed successfully!");
                }
                return result;
            }
        }
        Ok(())
    }
}

impl ISubcommand for SubcommandSet {
    fn name(&self) -> &'static str {
        self.name
    }

    fn description(&mut self) -> Command {
        let mut command = clap::Command::new(self.name)
            .about(self.about)
            .arg_required_else_help(true);

        for (name, subcommand) in self.subcommands.iter_mut() {
            let description = subcommand.description();

            assert_eq!(name.as_str(), subcommand.name());
            assert_eq!(name.as_str(), description.get_name());

            command = command.subcommand(description);
        }

        command
    }

    fn exec(&mut self, project: &AlephProject, matches: ArgMatches) -> anyhow::Result<()> {
        if let Some((subcommand_name, matches)) = matches.subcommand() {
            if let Some(subcommand) = self.subcommands.get_mut(subcommand_name) {
                let result = subcommand.exec(project, matches.clone());
                if result.is_ok() {
                    log::info!("Subcommand {subcommand_name} completed successfully!");
                }
                return result;
            }
        }
        Ok(())
    }
}

fn platform_arg() -> Arg {
    Arg::new("platform")
        .help("The platform to build shaders for.")
        .long_help("The platform to build shaders for. Supported values: native, uwp, android, ios, windows, macos, linux.")
        .default_value("native")
        .required(false)
}

fn config_arg() -> Arg {
    Arg::new("profile")
        .short('p')
        .long("profile")
        .help("The build configuration to target.")
        .long_help("The build configuration to target. Supported values: debug, release, retail.")
        .default_value("debug")
        .required(false)
}

fn arch_arg() -> Arg {
    Arg::new("arch")
        .short('a')
        .long("arch")
        .help("The target CPU architecture.")
        .long_help("The target CPU architecture. Can be 'native', 'x86_64', or 'aarch64'")
        .default_value("native")
        .required(false)
}
