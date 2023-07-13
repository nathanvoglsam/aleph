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
use crate::env::{ensure_target_project_root, project_file, project_root};
use crate::project::{
    AndroidBrandingSchema, AndroidSchema, GameSchema, ProjectSchema, UwpBrandingSchema, UwpSchema,
};
use crate::templates::{
    android_project_bundle, uwp_project_bundle, ANDROID_ACTIVITY_SOURCE_TEMPLATE,
    LOCAL_PROPERTIES_TEMPLATE,
};
use crate::utils::{
    architecture_from_arg, extract_zip, find_crate_and_target, get_cargo_metadata, BuildPlatform,
    Target,
};
use aleph_target::Architecture;
use anyhow::anyhow;
use clap::{Arg, ArgMatches, Command};
use serde::Serialize;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use tera::{Context, Tera};

pub struct GenProj {}

impl ISubcommand for GenProj {
    fn name(&self) -> &'static str {
        "genproj"
    }

    fn description(&mut self) -> Command {
        let platform = Arg::new("platform")
            .help("The platform to generate a project for.")
            .long_help("The platform to generate a project for. Supported values: uwp, android")
            .required(true);
        let arch = Arg::new("arch")
            .help("The architecture to generate the project for.")
            .long_help("The architecture to generate the project for, if the target needs architecture specific projects.")
            .default_value("native")
            .required(false);
        Command::new(self.name())
            .about("Generate platform target projects")
            .long_about("Tool for generating platform specific projects for platforms that have specific bundling requirements. For example: Android, which needs an android project to build an apk.")
            .arg(platform)
            .arg(arch)
    }

    fn exec(&mut self, mut matches: ArgMatches) -> anyhow::Result<()> {
        // let project_root = project_root()?;
        let project_toml = project_file()?;

        let toml = std::fs::read_to_string(&project_toml)?;
        let project: ProjectSchema = toml::from_str(&toml)?;

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

        if platform_arg == "native" {
            if !platform.is_valid_native_platform() {
                return Err(anyhow!("Invalid native platform \"{}\"", platform.name()));
            }
        }

        let target = Target { arch, platform };

        let root = ensure_target_project_root(&target)?;

        match target.platform {
            BuildPlatform::UWP => self.uwp(project, root, &target),
            BuildPlatform::Android => self.android(project, root, &target),
            _ => Err(anyhow!("Unsupported genproj platform '{platform_arg}'")),
        }
    }
}

impl GenProj {
    fn uwp(&self, project: ProjectSchema, root: PathBuf, target: &Target) -> anyhow::Result<()> {
        let context = build_uwp_template_context(&project, target)?;

        // Extract the .zip file onto our target directory
        let mut bundle = uwp_project_bundle();
        extract_zip(&mut bundle, Some(&root))?;

        let manifest_path = root.join("AppxManifest.xml");
        let template_files = [manifest_path.as_path()];
        apply_template_context_to_files(&context, &template_files)?;

        let uwp = project.uwp.as_ref().unwrap();
        if let Some(branding) = uwp.branding.as_ref() {
            let UwpBrandingSchema {
                lock_screen_logo,
                splash_screen,
                square_44_x_44_logo,
                square_150_x_150_logo,
                store_logo,
                wide_310_x_150_logo,
            } = branding;

            let project_root = project_root()?;

            // Ensure the 'Assets' directory exists within the UWP project folder
            let asset_dir = root.join("Assets");
            std::fs::create_dir_all(&asset_dir)?;

            let copy_branding_files = make_branding_file_copier(&project_root, &asset_dir);
            copy_branding_files(&[
                (lock_screen_logo.as_ref(), "LockScreenLogo.png"),
                (splash_screen.as_ref(), "SplashScreen.png"),
                (square_44_x_44_logo.as_ref(), "Square44x44Logo.png"),
                (square_150_x_150_logo.as_ref(), "Square150x150Logo.png"),
                (store_logo.as_ref(), "StoreLogo.png"),
                (wide_310_x_150_logo.as_ref(), "Wide310x150Logo.png"),
            ])?;
        }

        Ok(())
    }

    fn android(
        &self,
        project: ProjectSchema,
        root: PathBuf,
        _target: &Target,
    ) -> anyhow::Result<()> {
        let context = build_android_template_context(&project)?;

        // Extract the .zip file onto our target directory
        let mut bundle = android_project_bundle();
        extract_zip(&mut bundle, Some(&root))?;

        let manifest_path = root.join("app/src/main/AndroidManifest.xml");
        let gradle_path = root.join("app/build.gradle");
        let strings_path = root.join("app/src/main/res/values/strings.xml");
        let template_files = [
            manifest_path.as_path(),
            gradle_path.as_path(),
            strings_path.as_path(),
        ];
        apply_template_context_to_files(&context, &template_files)?;

        let android = project.android.as_ref().unwrap();

        // Check if we have the empty string, which is not a valid App ID
        if android.app_id.is_empty() {
            return Err(anyhow!("An empty string is not a valid app ID"));
        }

        // Check if we have something other than a sequence of path.segments.like.this with only
        // alpha numeric ascii characters.
        if android
            .app_id
            .contains(|v: char| !v.is_ascii_alphanumeric() && v != '.')
        {
            return Err(anyhow!(
                "App ID '{}' is invalid. Must only be alpha-numeric or . characters",
                &android.app_id
            ));
        }

        create_activity_from_template(android, &root, &context)?;

        let local_properties = root.join("local.properties");
        log::trace!("Writing template file '{:?}'", &local_properties);
        std::fs::write(&local_properties, LOCAL_PROPERTIES_TEMPLATE)?;

        if let Some(branding) = android.branding.as_ref() {
            let AndroidBrandingSchema {
                icon_mdpi,
                icon_hdpi,
                icon_xhdpi,
                icon_xxhdpi,
                icon_xxxhdpi,
            } = branding;

            let project_root = project_root()?;

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
            let copy_branding_files = make_branding_file_copier(&project_root, &res_dir);
            copy_branding_files(&pairs)?;
        }

        Ok(())
    }
}

fn create_activity_from_template(
    android: &AndroidSchema,
    root: &PathBuf,
    context: &Context,
) -> anyhow::Result<()> {
    // Create the path chain for the main AlephActivity
    let java_path = root.join("app/src/main/java");
    let package_path = android.app_id.replace('.', "/");
    let package_path = java_path.join(package_path);
    std::fs::create_dir_all(&package_path)?;

    let mut tera = Tera::default();
    tera.add_raw_template("a", ANDROID_ACTIVITY_SOURCE_TEMPLATE)?;
    let rendered_activity = tera.render("a", &context)?;

    let java_file_path = package_path.join("AlephActivity.java");
    log::trace!("Writing template file '{:?}'", &java_file_path);
    std::fs::write(java_file_path, rendered_activity)?;

    Ok(())
}

fn make_branding_file_copier<'a>(
    project_root: &'a Path,
    root: &'a Path,
) -> impl (FnOnce(&[(&str, &str)]) -> anyhow::Result<()>) + 'a {
    let copy_branding_file = |src: &Path, dst: &str| -> anyhow::Result<()> {
        match src.extension() {
            None => return Err(anyhow!("Branding file \"{:?}\" is not a .png file", src)),
            Some(v) => {
                if v != OsStr::new("png") {
                    return Err(anyhow!("Branding file \"{:?}\" is not a .png file", src));
                }
            }
        }

        if src.is_absolute() {
            let from = src;
            let to = root.join(dst);
            log::trace!("Copying '{:?} -> {:?}'", from, &to);
            std::fs::copy(from, to)?;
        } else {
            let from = project_root.join(src);
            let to = root.join(dst);
            log::trace!("Copying '{:?} -> {:?}'", from, &to);
            std::fs::copy(from, to)?;
        }
        Ok(())
    };

    move |pairs: &[(&str, &str)]| -> anyhow::Result<()> {
        for (src, dst) in pairs {
            copy_branding_file(Path::new(src), dst)?
        }
        Ok(())
    }
}

fn build_uwp_template_context(project: &ProjectSchema, target: &Target) -> anyhow::Result<Context> {
    // Grab needed info from the project schema
    let UwpSchema {
        identity_name,
        identity_publisher,
        ..
    } = project.uwp.as_ref().ok_or(anyhow!(
        "Trying to generate a uwp project with missing uwp table in project.toml"
    ))?;
    let GameSchema {
        name,
        crate_name,
        author,
        ..
    } = &project.game;

    // Fetch the cargo metadata from the current cargo workspace
    let cargo_metadata = get_cargo_metadata()?;
    let (package, _) = find_crate_and_target(&cargo_metadata, &crate_name, None)?;

    // Produce the uwp version string, leaving the last item as 0. We could in the future make
    // the last value a build number
    let uwp_version = format!(
        "{}.{}.{}.{}",
        package.version.major, package.version.minor, package.version.patch, 0
    );

    // UWP has its own naming for architectures (because of course it does).
    let uwp_arch = match target.arch {
        Architecture::X8664 => "x64",
        Architecture::AARCH64 => "arm64",
        Architecture::Unknown => panic!("Unknown architecture"),
    };

    // Grab the description from the cargo package, or use the default of "No Description" if
    // the crate lacks a description
    let uwp_description = package
        .description
        .as_ref()
        .map(|v| v.as_str())
        .unwrap_or("No Description");

    // Prepare our template context with information from the schema
    let mut context = Context::new();
    let mut set_var = context_var_logger(&mut context);
    log::info!("Project Template Variable Settings");
    set_var("UWP_GAME_IDENTITY_NAME", identity_name);
    set_var("UWP_GAME_IDENTITY_PUBLISHER", identity_publisher);
    set_var("UWP_GAME_VERSION", &uwp_version);
    set_var("UWP_GAME_ARCH", uwp_arch);
    set_var("UWP_GAME_EXECUTABLE", &format!("{crate_name}.exe"));
    set_var("UWP_GAME_DISPLAY_NAME", name);
    set_var("UWP_GAME_PUBLISHER_DISPLAY_NAME", author);
    set_var("UWP_GAME_DESCRIPTION", uwp_description);

    drop(set_var);
    Ok(context)
}

fn build_android_template_context(project: &ProjectSchema) -> anyhow::Result<Context> {
    // Grab needed info from the project schema
    let AndroidSchema {
        app_id, version_id, ..
    } = project.android.as_ref().ok_or(anyhow!(
        "Trying to generate an android project with missing android table in project.toml"
    ))?;
    let GameSchema {
        name, crate_name, ..
    } = &project.game;

    // Fetch the cargo metadata from the current cargo workspace
    let cargo_metadata = get_cargo_metadata()?;
    let (package, library_target) =
        find_crate_and_target(&cargo_metadata, &crate_name, Some("cdylib"))?;
    let library_target = library_target.unwrap();

    let app_version = format!(
        "{}.{}.{}",
        package.version.major, package.version.minor, package.version.patch
    );

    // Prepare our template context with information from the schema
    let mut context = Context::new();
    let mut set_var = context_var_logger(&mut context);
    log::trace!("Project Template Variable Settings");
    set_var("ANDROID_GAME_APPLICATION_ID", &app_id);
    set_var("ANDROID_GAME_NAME", &name);
    set_var("ANDROID_GAME_VERSION_CODE", &version_id.to_string());
    set_var("ANDROID_GAME_VERSION_NAME", &app_version);
    set_var("ANDROID_GAME_LIBRARY", &library_target.name);

    drop(set_var);
    Ok(context)
}

fn context_var_logger<'a>(context: &'a mut Context) -> impl FnMut(&str, &str) + 'a {
    |key: &str, val: &str| {
        log::info!("{key} = {val}");
        context.insert(key, &val)
    }
}

fn apply_template_context_to_files(context: &Context, files: &[&Path]) -> anyhow::Result<()> {
    let mut tera = Tera::default();
    for file in files.iter().copied() {
        let path_string = file.to_string_lossy();
        let file_content = std::fs::read_to_string(file)?;
        tera.add_raw_template(&path_string, &file_content)?;
        let rendered_content = tera.render(&path_string, context)?;

        log::trace!("Writing template file '{:?}'", file);
        std::fs::write(file, rendered_content)?;
    }
    Ok(())
}
