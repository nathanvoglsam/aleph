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

use std::collections::HashMap;

use anyhow::anyhow;
use camino::Utf8Path;
use clap::{ArgMatches, Command};

use crate::commands::{ISubcommand, arch_arg, platform_arg};
use crate::project::AlephProject;
use crate::project_schema::{AndroidBrandingSchema, AndroidSchema, GameSchema};
use crate::templates;
use crate::templates::android_project::ANDROID_ACTIVITY_SOURCE_TEMPLATE;
use crate::utils::{
    BuildPlatform, Target, architecture_from_arg, resolve_absolute_or_root_relative_path,
};

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

        let root = project.ensure_target_project_root(&target)?;

        match target.platform {
            BuildPlatform::Android => self.android(project, root, &target),
            _ => Err(anyhow!("Unsupported genproj platform '{platform_arg}'")),
        }
    }
}

impl GenProj {
    fn android(
        &self,
        project: &AlephProject,
        root: &Utf8Path,
        _target: &Target,
    ) -> anyhow::Result<()> {
        let project_schema = project.get_project_schema()?;
        let android = project_schema
            .android
            .as_ref()
            .ok_or(anyhow!("Android manifest info missing!"))?;

        if !is_valid_android_app_id(&android.app_id) {
            return Err(anyhow!(
                "App ID '{}' is invalid. It must contain at least two dot-separated segments; \
                 each segment must start with an ASCII letter and contain only ASCII letters, \
                 digits, or underscores",
                &android.app_id
            ));
        }

        let context = build_android_template_context(project)?;
        templates::android_project::write_android_project(root, &context)?;

        {
            // Create the path chain for the main AlephActivity
            let java_path = root.join("app/src/main/java");
            let package_path = android.app_id.replace('.', "/");
            let package_path = java_path.join(package_path);
            std::fs::create_dir_all(&package_path)?;

            let rendered =
                templates::android_project::render(ANDROID_ACTIVITY_SOURCE_TEMPLATE, &context)?;
            let java_file_name = templates::android_project::render(
                "${{ANDROID_GAME_NAME}}Activity.java",
                &context,
            )?;
            let java_file_path = package_path.join(java_file_name.as_ref());
            std::fs::write(java_file_path, rendered.as_ref())?;
        }

        if let Some(branding) = android.branding.as_ref() {
            let AndroidBrandingSchema {
                icon_mdpi,
                icon_hdpi,
                icon_xhdpi,
                icon_xxhdpi,
                icon_xxxhdpi,
            } = branding;

            let project_root = project.project_root();

            // Ensure the mipmap directories exists within the Android project folder
            let res_dir = root.join("app/src/main/res");
            std::fs::create_dir_all(res_dir.join("mipmap-mdpi"))?;
            std::fs::create_dir_all(res_dir.join("mipmap-hdpi"))?;
            std::fs::create_dir_all(res_dir.join("mipmap-xhdpi"))?;
            std::fs::create_dir_all(res_dir.join("mipmap-xxhdpi"))?;
            std::fs::create_dir_all(res_dir.join("mipmap-xxxhdpi"))?;

            let pairs = [
                (icon_mdpi.as_ref(), "mipmap-mdpi/ic_launcher.png"),
                (icon_hdpi.as_ref(), "mipmap-hdpi/ic_launcher.png"),
                (icon_xhdpi.as_ref(), "mipmap-xhdpi/ic_launcher.png"),
                (icon_xxhdpi.as_ref(), "mipmap-xxhdpi/ic_launcher.png"),
                (icon_xxxhdpi.as_ref(), "mipmap-xxxhdpi/ic_launcher.png"),
            ];
            let copy_branding_files = make_branding_file_copier(project_root, &res_dir);
            copy_branding_files(&pairs)?;
        }

        Ok(())
    }
}

fn is_valid_android_app_id(app_id: &str) -> bool {
    let segments: Vec<_> = app_id.split('.').collect();
    segments.len() >= 2
        && segments.iter().all(|segment| {
            let mut chars = segment.chars();
            chars
                .next()
                .is_some_and(|first| first.is_ascii_alphabetic())
                && chars.all(|ch| ch.is_ascii_alphanumeric() || ch == '_')
        })
}

#[cfg(test)]
mod tests {
    use super::is_valid_android_app_id;

    #[test]
    fn validates_android_app_ids() {
        for app_id in [
            "com.example.game",
            "com.example_1.game_2",
            "Com.Example.Game",
        ] {
            assert!(is_valid_android_app_id(app_id), "{app_id}");
        }

        for app_id in [
            "",
            "com",
            ".com.example",
            "com..example",
            "com.example.",
            "com.1example.game",
            "com._example.game",
            "com.exämple.game",
            "com.example-game",
        ] {
            assert!(!is_valid_android_app_id(app_id), "{app_id}");
        }
    }
}

fn make_branding_file_copier<'a>(
    project_root: &'a Utf8Path,
    root: &'a Utf8Path,
) -> impl (FnOnce(&[(&str, &str)]) -> anyhow::Result<()>) + 'a {
    let copy_branding_file = |src: &Utf8Path, dst: &str| -> anyhow::Result<()> {
        match src.extension() {
            None => return Err(anyhow!("Branding file \"{}\" is not a .png file", src)),
            Some(v) => {
                if v != "png" {
                    return Err(anyhow!("Branding file \"{}\" is not a .png file", src));
                }
            }
        }

        let from = resolve_absolute_or_root_relative_path(project_root, src);
        let to = root.join(dst);
        log::trace!("Copying '{} -> {}'", from, to);
        std::fs::copy(from, to)?;
        Ok(())
    };

    move |pairs: &[(&str, &str)]| -> anyhow::Result<()> {
        for (src, dst) in pairs {
            copy_branding_file(Utf8Path::new(src), dst)?
        }
        Ok(())
    }
}

fn build_android_template_context(
    project: &AlephProject,
) -> anyhow::Result<HashMap<String, String>> {
    let project_schema = project.get_project_schema()?;

    // Grab needed info from the project schema
    let AndroidSchema {
        app_id, version_id, ..
    } = project_schema.android.as_ref().ok_or(anyhow!(
        "Trying to generate an android project with missing android table in project.toml"
    ))?;
    let GameSchema {
        name,
        internal_name,
        ..
    } = &project_schema.game;

    // Fetch the cargo metadata from the current cargo workspace
    let (package, library_target) = project.get_game_crate_and_target()?;
    let library_target = library_target.unwrap();

    let app_version = format!(
        "{}.{}.{}",
        package.version.major, package.version.minor, package.version.patch
    );

    // Prepare our template context with information from the schema
    let mut context = HashMap::new();
    let mut set_var = |key: &str, val: &str| {
        log::info!("{key} = {val}");
        context.insert(key.to_string(), val.to_string());
    };

    log::info!("Project Template Variable Settings");
    set_var("ANDROID_GAME_PACKAGE", app_id);
    set_var("ANDROID_GAME_PRETTY_NAME", name);
    set_var("ANDROID_GAME_NAME", internal_name);
    set_var("ANDROID_GAME_CRATE", &package.name);
    set_var("ANDROID_GAME_TARGET", &library_target.name);
    set_var("ANDROID_GAME_VERSION_CODE", &version_id.to_string());
    set_var("ANDROID_GAME_VERSION_NAME", &app_version);
    set_var("ALEPH_WORKSPACE_ROOT", project.project_root().as_str());
    set_var("ANDROID_SDK_DIR", project.android_sdk_path().as_str());
    set_var("ANDROID_NDK_DIR", project.android_ndk_path().as_str());

    Ok(context)
}
