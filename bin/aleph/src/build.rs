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

use crate::utils::{BuildPlatform, Target};
use std::process::Command;

fn apply_release_args(cmd: &mut Command) {
    cmd.arg("--release");
}

fn apply_retail_args(cmd: &mut Command) {
    cmd.arg("--profile=retail");
    cmd.env("ALEPH_BUILD_TYPE", "Retail");
}

fn base_uwp_build(target: &Target) -> std::io::Result<Command> {
    assert_eq!(target.platform, BuildPlatform::UWP);

    // cargo +nightly-msvc build -Z build-std=std,panic_abort --target={arch}-uwp-windows-msvc
    let mut cmd = Command::new("cargo");
    cmd.arg("+nightly-msvc");
    cmd.arg("-Z");
    cmd.arg("build-std=std,panic_abort");
    cmd.arg(format!("--target={}-uwp-windows-msvc", target.arch.name()));
    cmd.arg("--package");
    cmd.arg("aleph-test");

    Ok(cmd)
}

fn release_uwp_build(target: &Target) -> std::io::Result<Command> {
    assert_eq!(target.platform, BuildPlatform::UWP);

    // cargo +nightly-msvc build -Z build-std=std,panic_abort --target={arch}-uwp-windows-msvc --release
    let mut cmd = base_uwp_build(target)?;
    apply_release_args(&mut cmd);

    Ok(cmd)
}

fn retail_uwp_build(target: &Target) -> std::io::Result<Command> {
    assert_eq!(target.platform, BuildPlatform::UWP);

    // cargo +nightly-msvc build -Z build-std=std,panic_abort --target={arch}-uwp-windows-msvc --release
    let mut cmd = base_uwp_build(target)?;
    apply_retail_args(&mut cmd);

    Ok(cmd)
}

fn base_android_build(target: &Target) -> std::io::Result<Command> {
    assert_eq!(target.platform, BuildPlatform::Android);

    // cargo ndk -t {arch} -p 30 build --target={arch}-linux-android --package aleph-test --lib
    let mut cmd = Command::new("cargo");
    cmd.arg("ndk");
    cmd.arg("-t");
    cmd.arg(target.arch.ndk_name());
    cmd.arg("-p");
    cmd.arg("30");
    cmd.arg("build");
    cmd.arg(format!("--target={}-linux-android", target.arch.name()));
    cmd.arg("--package");
    cmd.arg("aleph-test");
    cmd.arg("--lib");
    cmd.env("ANDROID_NDK_HOME", "TODO");

    Ok(cmd)
}

fn release_android_build(target: &Target) -> std::io::Result<Command> {
    assert_eq!(target.platform, BuildPlatform::Android);

    // cargo ndk -t {arch} -p 30 build --target={arch}-linux-android --package aleph-test --lib --release
    let mut cmd = base_android_build(target)?;
    apply_release_args(&mut cmd);

    Ok(cmd)
}

fn retail_android_build(target: &Target) -> std::io::Result<Command> {
    assert_eq!(target.platform, BuildPlatform::Android);

    // cargo ndk -t {arch} -p 30 build --profile=retail --target={arch}-linux-android --package aleph-test --lib
    let mut cmd = base_android_build(target)?;
    apply_retail_args(&mut cmd);

    Ok(cmd)
}
