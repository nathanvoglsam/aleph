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

use aleph_target::Platform;
use anyhow::anyhow;
use clap::ArgMatches;

use crate::commands::{ISubcommand, arch_arg, config_arg, platform_arg};
use crate::project::AlephProject;
use crate::utils::{BuildPlatform, Profile, Target, architecture_from_arg};

pub struct Bundle {}

impl ISubcommand for Bundle {
    fn name(&self) -> &'static str {
        "bundle"
    }

    fn description(&mut self) -> clap::Command {
        const LONG: &str = "\
            Tool for building the game for the requested platform/architecture/config. This will \
            copy build artefacts into project dirs and generate a 'bundle' for the target \
            platform. On iOS an App Bundle.\
        ";
        clap::Command::new(self.name())
            .about("Bundles the game for the requested platform/architecture/config")
            .long_about(LONG)
            .arg(platform_arg())
            .arg(arch_arg())
            .arg(config_arg())
    }

    fn exec(&mut self, project: &AlephProject, mut matches: ArgMatches) -> anyhow::Result<()> {
        let platform_arg: String = matches
            .remove_one("platform")
            .expect("platform should have a default");
        let arch_arg: String = matches
            .remove_one("arch")
            .expect("arch should have a default");
        let profile_arg: String = matches
            .remove_one("profile")
            .expect("profile should have a default");

        let platform = BuildPlatform::from_arg(&platform_arg)
            .ok_or(anyhow!("Unknown platform \"{}\"", &platform_arg))?;
        let arch = architecture_from_arg(&arch_arg)
            .ok_or(anyhow!("Unknown architecture \"{}\"", &arch_arg))?;
        let profile = Profile::from_name(&profile_arg.to_lowercase())
            .ok_or(anyhow!("Unknown profile \"{}\"", &profile_arg))?;

        if platform_arg == "native" && !platform.is_valid_native_platform() {
            return Err(anyhow!("Invalid native platform \"{}\"", platform.name()));
        }

        let native_build_platform = BuildPlatform::from(Platform::host());
        match platform {
            p @ (BuildPlatform::Windows | BuildPlatform::MacOS | BuildPlatform::Linux)
                if p != native_build_platform =>
            {
                log::error!(
                    "Trying to build platform '{}' on host '{}'. This platform does not support cross-compiling",
                    p,
                    native_build_platform
                );
                return Err(anyhow!(
                    "Trying to build platform '{}' on host '{}'.",
                    p,
                    native_build_platform
                ));
            }
            _ => {}
        }

        let target = Target { arch, platform };

        match target.platform {
            BuildPlatform::Windows => self.windows(project, profile, &target),
            BuildPlatform::MacOS => self.macos(project, profile, &target),
            BuildPlatform::Linux => self.linux(project, profile, &target),
            BuildPlatform::IOS => self.ios(project, profile, &target),
        }
    }
}

impl Bundle {
    fn windows(
        &self,
        _project: &AlephProject,
        _profile: Profile,
        _target: &Target,
    ) -> anyhow::Result<()> {
        unimplemented!()
    }

    fn macos(
        &self,
        _project: &AlephProject,
        _profile: Profile,
        _target: &Target,
    ) -> anyhow::Result<()> {
        unimplemented!()
    }

    fn linux(
        &self,
        _project: &AlephProject,
        _profile: Profile,
        _target: &Target,
    ) -> anyhow::Result<()> {
        unimplemented!()
    }

    fn ios(
        &self,
        _project: &AlephProject,
        _profile: Profile,
        _target: &Target,
    ) -> anyhow::Result<()> {
        unimplemented!()
    }
}
