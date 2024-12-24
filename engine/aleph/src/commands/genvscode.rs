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

use clap::{ArgMatches, Command};

use crate::commands::ISubcommand;
use crate::project::AlephProject;
use crate::vscode_settings::CodeWorkspace;

pub struct GenVsCode {}

impl ISubcommand for GenVsCode {
    fn name(&self) -> &'static str {
        "genvscode"
    }

    fn description(&mut self) -> Command {
        Command::new(self.name())
            .about("Generate vscode workspace for the project")
            .long_about("Tool for generating a vscode workspace for the project. This includes pre-filled settings and other options set for autocomplete for various langauges.")
    }

    fn exec(&mut self, project: &AlephProject, mut _matches: ArgMatches) -> anyhow::Result<()> {
        let file_path = project.vscode_workspace_file();
        let file = CodeWorkspace::from_project(project)?;

        log::info!("Generating workspace file");
        let file_text = serde_json::to_string_pretty(&file)?;

        log::info!("Writing workspace file '{}'", file_path.as_str());
        std::fs::write(file_path, file_text)?;

        Ok(())
    }
}
