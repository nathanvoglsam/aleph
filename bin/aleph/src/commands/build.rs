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

use crate::commands::ISubcommand;
use crate::env::{project_file, project_root, target_project_root};
use crate::project::ProjectSchema;
use crate::utils::{
    architecture_from_arg, find_crate_and_target, get_cargo_metadata, profile_from_arg,
    BuildPlatform, Target,
};
use aleph_target::build::target_platform;
use aleph_target::Profile;
use anyhow::anyhow;
use clap::{Arg, ArgMatches};
use std::path::PathBuf;
use std::process::Command;

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
        clap::Command::new(self.name())
            .about("Builds the game for the requested platform/architecture/config")
            .long_about("Tool for building the game for the requested platform/architecture/config. Will also copy build artifacts into project directories, if they exist")
            .arg(platform)
            .arg(arch)
            .arg(config)
    }

    fn exec(&mut self, mut matches: ArgMatches) -> anyhow::Result<()> {
        let project_root = project_root()?;
        let project_toml = project_file()?;

        let toml = std::fs::read_to_string(&project_toml)?;
        let project: ProjectSchema = toml::from_str(&toml)?;

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
        let profile = profile_from_arg(&profile_arg)
            .ok_or(anyhow!("Unknown profile \"{}\"", &profile_arg))?;

        if platform_arg == "native" {
            if !platform.is_valid_native_platform() {
                return Err(anyhow!("Invalid native platform \"{}\"", platform.name()));
            }
        }

        let native_build_platform = BuildPlatform::from(target_platform());
        match platform {
            p @ (BuildPlatform::Windows | BuildPlatform::MacOS | BuildPlatform::Linux) => {
                if p != native_build_platform {
                    log::error!("Trying to build platform '{}' on host '{}'. This platform does not support cross-compiling", p.name(), native_build_platform.name());
                    return Err(anyhow!(
                        "Trying to build platform '{}' on host '{}'.",
                        p.name(),
                        native_build_platform.name()
                    ));
                }
            }
            _ => {}
        }

        let target = Target { arch, platform };

        match target.platform {
            BuildPlatform::Windows => self.windows(project, project_root, profile, &target),
            BuildPlatform::MacOS => self.macos(project, project_root, profile, &target),
            BuildPlatform::Linux => self.linux(project, project_root, profile, &target),
            BuildPlatform::UWP => self.uwp(project, project_root, profile, &target),
            BuildPlatform::Android => self.android(project, project_root, profile, &target),
        }
    }
}

impl Build {
    fn windows(
        &self,
        project: ProjectSchema,
        root: PathBuf,
        profile: Profile,
        target: &Target,
    ) -> anyhow::Result<()> {
        assert_eq!(
            BuildPlatform::from(target_platform()),
            BuildPlatform::Windows,
            "It is only valid to build windows on windows"
        );
        self.plain_cargo_build(project, root, profile, target)
    }

    fn macos(
        &self,
        project: ProjectSchema,
        root: PathBuf,
        profile: Profile,
        target: &Target,
    ) -> anyhow::Result<()> {
        assert_eq!(
            BuildPlatform::from(target_platform()),
            BuildPlatform::MacOS,
            "It is only valid to build macos on macos"
        );
        self.plain_cargo_build(project, root, profile, target)
    }

    fn linux(
        &self,
        project: ProjectSchema,
        root: PathBuf,
        profile: Profile,
        target: &Target,
    ) -> anyhow::Result<()> {
        assert_eq!(
            BuildPlatform::from(target_platform()),
            BuildPlatform::Linux,
            "It is only valid to build linux on linux"
        );
        self.plain_cargo_build(project, root, profile, target)
    }

    fn plain_cargo_build(
        &self,
        project: ProjectSchema,
        _root: PathBuf,
        profile: Profile,
        _target: &Target,
    ) -> anyhow::Result<()> {
        match profile {
            Profile::Debug => {
                let mut command = base_native_build(&project.game.crate_name)?;
                log::info!("{:?}", &command);
                command.status()?;
            }
            Profile::Release => {
                let mut command = release_native_build(&project.game.crate_name)?;
                log::info!("{:?}", &command);
                command.status()?;
            }
            Profile::Retail => {
                let mut command = retail_native_build(&project.game.crate_name)?;
                log::info!("{:?}", &command);
                command.status()?;
            }
        };
        Ok(())
    }

    fn uwp(
        &self,
        project: ProjectSchema,
        _root: PathBuf,
        profile: Profile,
        target: &Target,
    ) -> anyhow::Result<()> {
        match profile {
            Profile::Debug => {
                let mut command = base_uwp_build(target, &project.game.crate_name)?;
                log::info!("{:?}", &command);
                command.status()?;
            }
            Profile::Release => {
                let mut command = release_uwp_build(target, &project.game.crate_name)?;
                log::info!("{:?}", &command);
                command.status()?;
            }
            Profile::Retail => {
                let mut command = retail_uwp_build(target, &project.game.crate_name)?;
                log::info!("{:?}", &command);
                command.status()?;
            }
        };
        Ok(())
    }

    fn android(
        &self,
        project: ProjectSchema,
        root: PathBuf,
        profile: Profile,
        target: &Target,
    ) -> anyhow::Result<()> {
        match profile {
            Profile::Debug => {
                let mut command = base_android_build(target, &project.game.crate_name)?;
                log::info!("{:?}", &command);
                command.status()?;
            }
            Profile::Release => {
                let mut command = release_android_build(target, &project.game.crate_name)?;
                log::info!("{:?}", &command);
                command.status()?;
            }
            Profile::Retail => {
                let mut command = retail_android_build(target, &project.game.crate_name)?;
                log::info!("{:?}", &command);
                command.status()?;
            }
        };

        let project_root = target_project_root(target)?;
        if project_root.exists() {
            let mut output_dir = project_root.join("app");
            output_dir.push("libs");
            output_dir.push(target.arch.ndk_name());

            let mut target_dir = root.join("target");
            target_dir.push(format!("{}-linux-android", target.arch.name()));
            target_dir.push(profile.name());

            let cargo_metadata = get_cargo_metadata()?;
            let (_, library_target) =
                find_crate_and_target(&cargo_metadata, &project.game.crate_name, Some("cdylib"))?;
            let library_target = library_target.unwrap();

            let lib_name = format!("lib{}.so", &library_target.name);
            let target_lib = target_dir.join(&lib_name);
            let dst_lib = output_dir.join(&lib_name);
            log::trace!("Copying '{:?}' -> '{:?}'", target_lib, dst_lib);
            std::fs::copy(target_lib, dst_lib)?;

            let target_artifacts_dir = target_dir.join("artifacts");
            for item in target_artifacts_dir.read_dir()? {
                let item = item?;
                if item.metadata()?.is_file() {
                    let item_path = item.path();
                    let dst = output_dir.join(item_path.file_name().unwrap());

                    log::trace!("Copying '{:?}' -> '{:?}'", &item_path, &dst);
                    std::fs::copy(item_path, dst)?;
                }
            }
        }

        Ok(())
    }
}

fn apply_release_args(cmd: &mut Command) {
    cmd.arg("--release");
}

fn apply_retail_args(cmd: &mut Command) {
    cmd.arg("--profile=retail");
    cmd.env("ALEPH_BUILD_TYPE", "Retail");
}

fn base_uwp_build(target: &Target, package: &str) -> std::io::Result<Command> {
    assert_eq!(target.platform, BuildPlatform::UWP);

    // cargo +nightly-msvc build -Z build-std=std,panic_abort --target={arch}-uwp-windows-msvc --package {package} --bin
    let mut cmd = Command::new("rustup");
    cmd.arg("run");
    cmd.arg("nightly-msvc");
    cmd.arg("cargo");
    cmd.arg("build");
    cmd.arg("-Z");
    cmd.arg("build-std=std,panic_abort");
    cmd.arg(format!("--target={}-uwp-windows-msvc", target.arch.name()));
    cmd.arg("--package");
    cmd.arg(package);
    cmd.arg("--bin");
    cmd.arg(package);

    Ok(cmd)
}

fn release_uwp_build(target: &Target, package: &str) -> std::io::Result<Command> {
    assert_eq!(target.platform, BuildPlatform::UWP);

    // cargo +nightly-msvc build -Z build-std=std,panic_abort --target={arch}-uwp-windows-msvc --package {package} --bin --release
    let mut cmd = base_uwp_build(target, package)?;
    apply_release_args(&mut cmd);

    Ok(cmd)
}

fn retail_uwp_build(target: &Target, package: &str) -> std::io::Result<Command> {
    assert_eq!(target.platform, BuildPlatform::UWP);

    // cargo +nightly-msvc build -Z build-std=std,panic_abort --target={arch}-uwp-windows-msvc --package {package} --bin --profile="retail"
    let mut cmd = base_uwp_build(target, package)?;
    apply_retail_args(&mut cmd);

    Ok(cmd)
}

fn base_android_build(target: &Target, package: &str) -> std::io::Result<Command> {
    assert_eq!(target.platform, BuildPlatform::Android);

    // cargo ndk -t {arch} -p 30 build --target={arch}-linux-android --package {package} --lib
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
    // cmd.env("ANDROID_NDK_HOME", "TODO");

    Ok(cmd)
}

fn release_android_build(target: &Target, package: &str) -> std::io::Result<Command> {
    assert_eq!(target.platform, BuildPlatform::Android);

    // cargo ndk -t {arch} -p 30 build --target={arch}-linux-android --package {package} --lib --release
    let mut cmd = base_android_build(target, package)?;
    apply_release_args(&mut cmd);

    Ok(cmd)
}

fn retail_android_build(target: &Target, package: &str) -> std::io::Result<Command> {
    assert_eq!(target.platform, BuildPlatform::Android);

    // cargo ndk -t {arch} -p 30 build --profile=retail --target={arch}-linux-android --package {package} --lib
    let mut cmd = base_android_build(target, package)?;
    apply_retail_args(&mut cmd);

    Ok(cmd)
}

fn base_native_build(package: &str) -> std::io::Result<Command> {
    // cargo build --package aleph-test --bin
    let mut cmd = Command::new("rustup");
    cmd.arg("run");
    cmd.arg("stable-msvc");
    cmd.arg("cargo");
    cmd.arg("build");
    cmd.arg("--package");
    cmd.arg(package);
    cmd.arg("--bin");
    cmd.arg(package);

    Ok(cmd)
}

fn release_native_build(package: &str) -> std::io::Result<Command> {
    // cargo build --package aleph-test --bin --release
    let mut cmd = base_native_build(package)?;
    apply_release_args(&mut cmd);

    Ok(cmd)
}

fn retail_native_build(package: &str) -> std::io::Result<Command> {
    // cargo build --profile=retail --package aleph-test --bin
    let mut cmd = base_native_build(package)?;
    apply_retail_args(&mut cmd);

    Ok(cmd)
}