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

use std::ffi::OsString;
use std::process::Command;

use anyhow::{Context, anyhow};
use clap::{Arg, ArgMatches};

use crate::commands::ISubcommand;
use crate::project::AlephProject;

pub struct Node {}

impl ISubcommand for Node {
    fn name(&self) -> &'static str {
        "node"
    }

    fn description(&mut self) -> clap::Command {
        clap::Command::new(self.name())
            .about("Run the project's bundled Node.js executable")
            .disable_help_flag(true)
            .arg(
                Arg::new("args")
                    .value_parser(clap::builder::OsStringValueParser::new())
                    .num_args(0..)
                    .allow_hyphen_values(true)
                    .trailing_var_arg(true),
            )
    }

    fn exec(&mut self, project: &AlephProject, matches: ArgMatches) -> anyhow::Result<()> {
        let args: Vec<OsString> = matches
            .get_many::<OsString>("args")
            .into_iter()
            .flatten()
            .cloned()
            .collect();

        let status = Command::new(project.node_path())
            .args(args)
            .status()
            .context("Failed to run the project's bundled Node.js executable")?;

        if !status.success() {
            return Err(anyhow!("Node.js exited with status {status}"));
        }

        Ok(())
    }

    fn dont_log(&self) -> bool {
        true
    }
}
