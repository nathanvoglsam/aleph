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

use aleph_target::build::{target_architecture, target_platform};
use aleph_target::Profile;
use anyhow::anyhow;
use camino::{Utf8Path, Utf8PathBuf};
use clap::{Arg, ArgAction, ArgMatches};

use crate::commands::ISubcommand;
use crate::project::AlephProject;
use crate::utils::{
    architecture_from_arg, resolve_absolute_or_root_relative_path, BuildPlatform, Target,
};

pub struct Build {}

impl ISubcommand for Build {
    fn name(&self) -> &'static str {
        "build"
    }

    fn description(&mut self) -> clap::Command {
        let platform = Arg::new("platform")
            .help("The platform to build the game for.")
            .long_help("The platform to generate a project for. Supported values: native, uwp, android, windows, macos, linux.")
            .default_value("native")
            .required(false);
        let arch = Arg::new("arch")
            .short('a')
            .long("arch")
            .help("The architecture to build the game for.")
            .default_value("native")
            .required(false);
        let config = Arg::new("profile")
            .short('p')
            .long("profile")
            .help("The build configuration to target.")
            .long_help(
                "The build configuration to target. Supported values: debug, release, retail.",
            )
            .default_value("debug")
            .required(false);
        let build_std = Arg::new("build-std")
            .long("build-std")
            .help("Force building the rust standard library with the build-std option")
            .long_help("Force building the rust standard library with the build-std option. Currently requires a nightly compiler and the rust-src component.")
            .action(ArgAction::SetTrue)
            .required(false);
        clap::Command::new(self.name())
            .about("Builds the game for the requested platform/architecture/config")
            .long_about("Tool for building the game for the requested platform/architecture/config. Will also copy build artifacts into project directories, if they exist")
            .arg(platform)
            .arg(arch)
            .arg(config)
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
                log::error!("Trying to build platform '{}' on host '{}'. This platform does not support cross-compiling", p, native_build_platform);
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
            log::warn!("--build-std flag specified for Android, but Android does not support this flag. --build-std will be ignored!");
        }
        if build_std && matches!(platform, BuildPlatform::Uwp) {
            log::warn!("--build-std flag specified for UWP, but UWP does not require this flag. UWP always builds with build-std");
        }

        match target.platform {
            BuildPlatform::Windows => self.windows(project, profile, &target, build_std),
            BuildPlatform::MacOS => self.macos(project, profile, &target, build_std),
            BuildPlatform::Linux => self.linux(project, profile, &target, build_std),
            BuildPlatform::Uwp => self.uwp(project, profile, &target),
            BuildPlatform::Android => self.android(project, profile, &target),
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

        self.copy_uwp_build_to_appx_folder(project, profile, target)?;

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

        self.copy_android_build_to_gradle_project(project, profile, target)?;

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

    fn copy_android_build_to_gradle_project(
        &self,
        project: &AlephProject,
        profile: Profile,
        target: &Target,
    ) -> anyhow::Result<()> {
        let android_project_root = project.target_project_root(target)?;
        let mut output_dir = android_project_root.join("app");
        output_dir.push("libs");
        output_dir.push(target.arch.ndk_name());
        let target_dir = project.cargo_build_dir_for_target(target, profile)?;

        if output_dir.exists() {
            let (_, library_target) = project.get_game_crate_and_target()?;
            let library_target = library_target.unwrap();

            let lib_name = format!("lib{}.so", &library_target.name);
            let target_lib = target_dir.join(&lib_name);
            let dst_lib = output_dir.join(&lib_name);
            log::trace!("Copying '{}' -> '{}'", target_lib, dst_lib);
            std::fs::copy(target_lib, dst_lib)?;

            Self::copy_artifacts_from_target_to_project(&target_dir, &output_dir)?;
        } else {
            log::warn!(
                "Skipping android build artifact copy as \"{}\" does not exist",
                output_dir
            );
        }

        Ok(())
    }

    fn copy_uwp_build_to_appx_folder(
        &self,
        project: &AlephProject,
        profile: Profile,
        target: &Target,
    ) -> anyhow::Result<()> {
        let uwp_project_root = project.target_project_root(target)?;
        let target_dir = project.cargo_build_dir_for_target(target, profile)?;

        if uwp_project_root.exists() {
            let (package, _) = project.get_game_crate_and_target()?;
            let exe_name = format!("{}.exe", &package.name);

            let src_exe = target_dir.join(&exe_name);
            let dst_exe = uwp_project_root.join(&exe_name);
            log::trace!("Copying '{}' -> '{}'", src_exe, dst_exe);
            std::fs::copy(src_exe, dst_exe)?;

            Self::copy_artifacts_from_target_to_project(&target_dir, uwp_project_root)?;
        } else {
            log::warn!(
                "Skipping uwp build artifact copy as \"{}\" does not exist",
                uwp_project_root
            );
        }

        Ok(())
    }

    fn copy_artifacts_from_target_to_project(
        target_dir: &Utf8Path,
        output_dir: &Utf8Path,
    ) -> anyhow::Result<()> {
        let target_artifacts_dir = target_dir.join("artifacts");
        for item in target_artifacts_dir.read_dir_utf8()? {
            let item = item?;
            if item.metadata()?.is_file() {
                let item_path = item.path();
                let dst = output_dir.join(item_path.file_name().unwrap());

                log::trace!("Copying '{}' -> '{}'", item_path, dst);
                std::fs::copy(item_path, dst)?;
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

    let target = format!("--target={}-uwp-windows-msvc", target.arch.name());
    bin_build(profile, Some(&target), package, true)
}

fn native_build(profile: Profile, package: &str, build_std: bool) -> Command {
    let target = build_std
        .then(|| aleph_target::recreate_triple(target_platform(), target_architecture()).unwrap());
    bin_build(profile, target, package, build_std)
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

    let ndk_home = std::env::var("ANDROID_NDK_HOME").map(Utf8PathBuf::from).or_else(|_err| {
        let ndk_path = project.ndk_path();
        if !ndk_path.exists() {
            log::error!("ANDROID_NDK_HOME is not set and .aleph/sdks/ndk is missing!");
            log::error!("Building for android requires either ANDROID_NDK_HOME to be set or for an NDK to be present at .aleph/sdks/ndk");
            Err(anyhow!("ANDROID_NDK_HOME is not set and .aleph/sdks/ndk is missing!"))
        } else {
            Ok(ndk_path.to_path_buf())
        }
    })?;
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
