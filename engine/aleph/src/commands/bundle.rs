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
use aleph_target::build::target_platform;
use anyhow::anyhow;
use blink_alloc::Blink;
use camino::Utf8Path;
use clap::ArgMatches;

use crate::commands::{ISubcommand, arch_arg, config_arg, platform_arg};
use crate::project::AlephProject;
use crate::shader_system::ShaderSubproject;
use crate::utils::dunce_utf8::simplified;
use crate::utils::{
    BuildPlatform, Target, architecture_from_arg, get_gradlew_name, resolve_ndk_from_proj_or_env,
};

pub struct Bundle {}

impl ISubcommand for Bundle {
    fn name(&self) -> &'static str {
        "bundle"
    }

    fn description(&mut self) -> clap::Command {
        const LONG: &str = "\
            Tool for building the game for the requested platform/architecture/config. This will \
            copy build artefacts into project dirs and generate a 'bundle' for the target \
            platform. On Android you get an APK, and iOS an App Bundle.\
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

        match target.platform {
            BuildPlatform::Windows => self.windows(project, profile, &target),
            BuildPlatform::MacOS => self.macos(project, profile, &target),
            BuildPlatform::Linux => self.linux(project, profile, &target),
            BuildPlatform::Android => self.android(project, profile, &target),
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

    fn android(
        &self,
        project: &AlephProject,
        profile: Profile,
        target: &Target,
    ) -> anyhow::Result<()> {
        // TODO: we need to handle assets _way_ better eventually
        self.copy_android_build_to_gradle_project(project, profile, target)?;
        self.copy_shader_db_to_gradle_project(project, target)?;
        self.android_build_apk(project, target)?;

        Ok(())
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

impl Bundle {
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

    fn copy_shader_db_to_gradle_project(
        &self,
        project: &AlephProject,
        target: &Target,
    ) -> anyhow::Result<()> {
        let android_project_root = project.target_project_root(target)?;

        let mut output_dir = android_project_root.join("app");
        output_dir.push("src");
        output_dir.push("main");
        output_dir.push("assets");

        // Ensure the assets directory exists
        std::fs::create_dir_all(&output_dir)?;

        if output_dir.exists() {
            // Build the base level project context for our shader build system
            let arena = Blink::new();
            let project_ctx = ShaderSubproject::load(&arena, project)?;

            let src_file = project_ctx.meta.output_root.join("shaders.shaderdb");
            let dst_file = output_dir.join("shaders.shaderdb");
            log::trace!("Copying '{}' -> '{}'", src_file, dst_file);
            std::fs::copy(src_file, dst_file)?;
        } else {
            log::warn!(
                "Skipping android shaderdb copy as \"{}\" does not exist",
                output_dir
            );
        }

        Ok(())
    }

    fn android_build_apk(&self, project: &AlephProject, target: &Target) -> anyhow::Result<()> {
        let android_project_root = project.target_project_root(target)?;
        let ndk_home = resolve_ndk_from_proj_or_env(project)?;

        if android_project_root.exists() {
            let android_project_root = simplified(android_project_root);

            let gradlew = android_project_root.join(get_gradlew_name());
            let mut command = Command::new(gradlew);
            command.arg("assembleDebug");
            command.current_dir(android_project_root);
            command.env("ANDROID_NDK_HOME", ndk_home);

            // TODO: where do we get a JRE for gradle, where do we get the android SDK from?

            log::info!("Building APK for {}", &android_project_root);

            let status = command.status()?;

            if !status.success() {
                log::error!("gradlew invocation failed! Terminating build.");
                return Err(anyhow!("gradlew invocation failed!"));
            }
        } else {
            log::warn!(
                "Skipping android packaging as \"{}\" does not exist",
                android_project_root
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
