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

use std::process::Command;

use aleph_target::Profile;
use aleph_target::build::{target_architecture, target_platform};
use anyhow::anyhow;
use clap::{Arg, ArgAction, ArgMatches};

use crate::commands::{ISubcommand, arch_arg, config_arg, platform_arg};
use crate::project::AlephProject;
use crate::utils::{
    BuildPlatform, Target, architecture_from_arg, resolve_absolute_or_root_relative_path,
    resolve_ndk_from_proj_or_env,
};

pub struct Build {}

impl ISubcommand for Build {
    fn name(&self) -> &'static str {
        "build"
    }

    fn description(&mut self) -> clap::Command {
        let build_std = Arg::new("build-std")
            .long("build-std")
            .help("Force building the rust standard library with the build-std option")
            .long_help("Force building the rust standard library with the build-std option. Currently requires a nightly compiler and the rust-src component.")
            .action(ArgAction::SetTrue)
            .required(false);
        clap::Command::new(self.name())
            .about("Builds the game for the requested platform/architecture/config")
            .long_about("Tool for building the game for the requested platform/architecture/config. Will also copy build artifacts into project directories, if they exist")
            .arg(platform_arg())
            .arg(arch_arg())
            .arg(config_arg())
            .arg(build_std)
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

        let native_build_platform = BuildPlatform::from(target_platform());
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

        let build_std = matches.get_flag("build-std");

        if build_std && matches!(platform, BuildPlatform::Android) {
            log::warn!(
                "--build-std flag specified for Android, but Android does not support this flag. --build-std will be ignored!"
            );
        }
        if build_std && matches!(platform, BuildPlatform::Uwp) {
            log::warn!(
                "--build-std flag specified for UWP, but UWP does not require this flag. UWP always builds with build-std"
            );
        }

        match target.platform {
            BuildPlatform::Windows => self.windows(project, profile, &target, build_std),
            BuildPlatform::MacOS => self.macos(project, profile, &target, build_std),
            BuildPlatform::Linux => self.linux(project, profile, &target, build_std),
            BuildPlatform::Uwp => self.uwp(project, profile, &target),
            BuildPlatform::Android => self.android(project, profile, &target),
            BuildPlatform::IOS => self.ios(project, profile, &target, build_std),
        }
    }
}

impl Build {
    fn windows(
        &self,
        project: &AlephProject,
        profile: Profile,
        target: &Target,
        build_std: bool,
    ) -> anyhow::Result<()> {
        assert_eq!(
            BuildPlatform::from(target_platform()),
            BuildPlatform::Windows,
            "It is only valid to build windows on windows"
        );
        self.plain_cargo_build(project, profile, target, build_std)
    }

    fn macos(
        &self,
        project: &AlephProject,
        profile: Profile,
        target: &Target,
        build_std: bool,
    ) -> anyhow::Result<()> {
        assert_eq!(
            BuildPlatform::from(target_platform()),
            BuildPlatform::MacOS,
            "It is only valid to build macos on macos"
        );
        self.plain_cargo_build(project, profile, target, build_std)
    }

    fn linux(
        &self,
        project: &AlephProject,
        profile: Profile,
        target: &Target,
        build_std: bool,
    ) -> anyhow::Result<()> {
        assert_eq!(
            BuildPlatform::from(target_platform()),
            BuildPlatform::Linux,
            "It is only valid to build linux on linux"
        );
        self.plain_cargo_build(project, profile, target, build_std)
    }

    fn uwp(&self, project: &AlephProject, profile: Profile, target: &Target) -> anyhow::Result<()> {
        let project_schema = project.get_project_schema()?;

        let mut command = uwp_build(target, profile, &project_schema.game.crate_name);
        self.add_win32_branding_env_vars(project, target, &mut command)?;

        log::info!("{:?}", &command);
        let status = command.status()?;

        if !status.success() {
            log::error!("Cargo invocation failed! Terminating build.");
            return Err(anyhow!("cargo invocation failed!"));
        }

        Ok(())
    }

    fn android(
        &self,
        project: &AlephProject,
        profile: Profile,
        target: &Target,
    ) -> anyhow::Result<()> {
        let project_schema = project.get_project_schema()?;

        let mut command = android_build(project, target, profile, &project_schema.game.crate_name)?;
        log::info!("{:?}", &command);
        let status = command.status()?;

        if !status.success() {
            log::error!("Cargo invocation failed! Terminating build.");
            return Err(anyhow!("cargo invocation failed!"));
        }

        Ok(())
    }

    fn ios(
        &self,
        project: &AlephProject,
        profile: Profile,
        target: &Target,
        build_std: bool,
    ) -> anyhow::Result<()> {
        let project_schema = project.get_project_schema()?;

        let mut command = ios_build(target, profile, &project_schema.game.crate_name, build_std);
        log::info!("{:?}", &command);
        let status = command.status()?;

        if !status.success() {
            log::error!("Cargo invocation failed! Terminating build.");
            return Err(anyhow!("cargo invocation failed!"));
        }

        // self.copy_android_build_to_gradle_project(project, profile, target)?;

        Ok(())
    }
}

impl Build {
    fn plain_cargo_build(
        &self,
        project: &AlephProject,
        profile: Profile,
        target: &Target,
        build_std: bool,
    ) -> anyhow::Result<()> {
        let project_schema = project.get_project_schema()?;

        let mut command = native_build(profile, &project_schema.game.crate_name, build_std);
        self.add_win32_branding_env_vars(project, target, &mut command)?;

        log::info!("{:?}", &command);
        let status = command.status()?;

        if !status.success() {
            log::error!("Cargo invocation failed! Terminating build.");
            return Err(anyhow!("cargo invocation failed!"));
        }

        Ok(())
    }

    fn add_win32_branding_env_vars(
        &self,
        project: &AlephProject,
        target: &Target,
        command: &mut Command,
    ) -> anyhow::Result<()> {
        let project_schema = project.get_project_schema()?;

        // For windows or uwp we need to add an environment var for configuring the executable icon
        if matches!(target.platform, BuildPlatform::Windows | BuildPlatform::Uwp) {
            if let Some(branding) = project_schema
                .windows
                .as_ref()
                .and_then(|v| v.branding.as_ref())
            {
                let icon = branding.icon.as_ref();
                let icon = resolve_absolute_or_root_relative_path(project.project_root(), icon);
                log::info!("ALEPH_WIN32_ICON_FILE = {:#?}", &icon);
                command.env("ALEPH_WIN32_ICON_FILE", icon);
            }
        }
        Ok(())
    }
}

fn bin_build(profile: Profile, target: Option<&str>, package: &str, build_std: bool) -> Command {
    let toolchain = if build_std { "nightly" } else { "stable" };

    let mut cmd = Command::new("rustup");
    cmd.arg("run");
    if cfg!(windows) {
        cmd.arg(format!("{}-msvc", toolchain));
    } else {
        cmd.arg(toolchain);
    }
    cmd.arg("cargo");
    cmd.arg("build");
    if build_std {
        cmd.arg("-Z");
        cmd.arg("build-std=std,panic_abort");
    }
    if let Some(target) = target {
        cmd.arg(format!("--target={}", target));
    }
    cmd.arg("--package");
    cmd.arg(package);
    cmd.arg("--bin");
    cmd.arg(package);

    profile_args(&mut cmd, profile);

    cmd
}

fn uwp_build(target: &Target, profile: Profile, package: &str) -> Command {
    assert_eq!(target.platform, BuildPlatform::Uwp);

    let target = format!("{}-uwp-windows-msvc", target.arch.name());
    bin_build(profile, Some(&target), package, true)
}

fn native_build(profile: Profile, package: &str, build_std: bool) -> Command {
    let target = build_std
        .then(|| aleph_target::recreate_triple(target_platform(), target_architecture()).unwrap());
    bin_build(profile, target, package, build_std)
}

fn ios_build(target: &Target, profile: Profile, package: &str, build_std: bool) -> Command {
    assert_eq!(target.platform, BuildPlatform::IOS);

    let target = format!("{}-apple-ios", target.arch.name());
    bin_build(profile, Some(&target), package, build_std)
}

fn android_build(
    project: &AlephProject,
    target: &Target,
    profile: Profile,
    package: &str,
) -> anyhow::Result<Command> {
    assert_eq!(target.platform, BuildPlatform::Android);

    let mut cmd = Command::new("cargo");
    cmd.arg("ndk");
    cmd.arg("-t");
    cmd.arg(target.arch.ndk_name());
    cmd.arg("-p");
    cmd.arg("30");
    cmd.arg("build");
    cmd.arg(format!("--target={}-linux-android", target.arch.name()));
    cmd.arg("--package");
    cmd.arg(package);
    cmd.arg("--lib");

    profile_args(&mut cmd, profile);

    let ndk_home = resolve_ndk_from_proj_or_env(project)?;
    cmd.env("ANDROID_NDK_HOME", ndk_home);

    Ok(cmd)
}

fn profile_args(cmd: &mut Command, profile: Profile) {
    match profile {
        Profile::Debug => {}
        Profile::Release => {
            cmd.arg("--release");
        }
        Profile::Retail => {
            cmd.arg("--profile=retail");
            cmd.env("ALEPH_BUILD_TYPE", "Retail");
        }
    }
}
