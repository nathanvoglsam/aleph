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

use anyhow::anyhow;
use clap::{ArgMatches, Command};

use crate::commands::{ISubcommand, arch_arg, platform_arg};
use crate::project::AlephProject;
use crate::utils::{BuildPlatform, Target, architecture_from_arg};

pub struct GenProj {}

impl ISubcommand for GenProj {
    fn name(&self) -> &'static str {
        "genproj"
    }

    fn description(&mut self) -> Command {
        Command::new(self.name())
            .about("Generate platform target projects")
            .long_about("Tool for generating platform specific projects for platforms that have specific bundling requirements. For example: iOS, which needs an XCode project to build an app.")
            .arg(platform_arg())
            .arg(arch_arg())
    }

    fn exec(&mut self, project: &AlephProject, mut matches: ArgMatches) -> anyhow::Result<()> {
        let platform_arg: String = matches
            .remove_one("platform")
            .expect("platform is required");
        let arch_arg: String = matches
            .remove_one("arch")
            .expect("arch should have a default");

        let platform = BuildPlatform::from_arg(&platform_arg)
            .ok_or(anyhow!("Unknown platform \"{}\"", &platform_arg))?;
        let arch = architecture_from_arg(&arch_arg)
            .ok_or(anyhow!("Unknown architecture \"{}\"", &arch_arg))?;

        if platform_arg == "native" && !platform.is_valid_native_platform() {
            return Err(anyhow!("Invalid native platform \"{}\"", platform.name()));
        }

        let target = Target { arch, platform };

        let _root = project.ensure_target_project_root(&target)?;

        match target.platform {
            _ => Err(anyhow!("Unsupported genproj platform '{platform_arg}'")),
        }
    }
}
