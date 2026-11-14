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

use std::path::PathBuf;

pub fn build_root() -> Option<PathBuf> {
    std::env::var_os("ALEPH_BUILD_ROOT")
        .map(PathBuf::from)
        .inspect(|_| println!("cargo:rerun-if-env-changed=ALEPH_BUILD_ROOT"))
}

pub fn rerun_if_android_env_changed() {
    println!("cargo::rerun-if-env-changed=ANDROID_NDK_HOME");
    println!("cargo::rerun-if-env-changed=ANDROID_NDK_ROOT");
    println!("cargo::rerun-if-env-changed=ANDROID_NDK");
    println!("cargo::rerun-if-env-changed=CARGO_NDK_ANDROID_PLATFORM");
    println!("cargo::rerun-if-env-changed=ANDROID_PLATFORM");
    println!("cargo::rerun-if-env-changed=ANDROID_STL");
}

pub struct AndroidBuildConfig {
    pub api: u32,
    pub abi: &'static str,
    pub toolchain: PathBuf,
    pub stl: String,
}

pub fn get_android_build_config_from_env() -> AndroidBuildConfig {
    use std::env;

    use aleph_target::Architecture;

    rerun_if_android_env_changed();

    let (abi, default_api) = match Architecture::build_target() {
        Architecture::X8664 => ("x86_64", 21),
        Architecture::AARCH64 => ("arm64-v8a", 21),
        Architecture::Unknown => panic!("Unknown architecture"),
    };

    let ndk = ["ANDROID_NDK_HOME", "ANDROID_NDK_ROOT", "ANDROID_NDK"]
        .into_iter()
        .find_map(|name| env::var_os(name))
        .map(PathBuf::from)
        .expect(
            "Android SDL3 build requires the Android NDK; set ANDROID_NDK_HOME or ANDROID_NDK_ROOT",
        );
    let toolchain = ndk.join("build/cmake/android.toolchain.cmake");
    assert!(
        toolchain.is_file(),
        "Android NDK toolchain not found at '{}'",
        toolchain.display()
    );

    let api = ["CARGO_NDK_ANDROID_PLATFORM", "ANDROID_PLATFORM"]
        .into_iter()
        .find_map(|name| env::var(name).ok())
        .map(|value| {
            value
                .strip_prefix("android-")
                .unwrap_or(&value)
                .parse::<u32>()
                .unwrap_or_else(|_| panic!("Invalid Android API level: {value}"))
        })
        .unwrap_or(default_api);

    let stl = env::var("ANDROID_STL").expect("ANDROID_STL environment variable not set");

    AndroidBuildConfig {
        api,
        abi,
        toolchain,
        stl,
    }
}
