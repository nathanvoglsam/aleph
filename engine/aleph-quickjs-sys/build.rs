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

extern crate aleph_compile as compile;

use aleph_target::{BuildConfig, Platform};

///
/// Main driver for compiling luajit, handles switching between w/e implementation is needed for the
/// target platforms.
///
fn main() {
    println!("cargo::rerun-if-changed=src");
    println!("cargo::rerun-if-changed=cmake");

    let mut config = cmake::Config::new("cmake");

    // We can't build 'Debug' on Windows because we can't link with the debug C runtime
    if Platform::build_target().is_windows() && BuildConfig::build_target().is_debug() {
        config.profile("RelWithDebInfo");
    } else {
        config.profile("Release");
    }

    if let Platform::Android = Platform::build_target() {
        let android_config = compile::get_android_build_config_from_env();
        config.define("CMAKE_TOOLCHAIN_FILE", android_config.toolchain);
        config.define("ANDROID_ABI", android_config.abi);
        config.define(
            "ANDROID_PLATFORM",
            format!("android-{}", android_config.api),
        );
        config.define("ANDROID_STL", android_config.stl);
    }

    let dst = config.build();

    let mut lib_path = dst.join("lib");
    if !lib_path.exists() {
        // Because GNU likes to be difficult with their standard paths sometimes we get this
        // rubbish. I love making build logic more complicated!!!!
        lib_path = dst.join("lib64");
    }

    let target_platform = Platform::build_target();
    match target_platform {
        Platform::WindowsGNU
        | Platform::WindowsMSVC
        | Platform::MacOS
        | Platform::IOS
        | Platform::Android
        | Platform::Linux => {
            println!(
                "cargo:rustc-link-search=native={}",
                lib_path.canonicalize().unwrap().display()
            );
            println!("cargo::rustc-link-lib=static=qjs");
        }
        Platform::Unknown => {
            // Do nothing on 'unknown' as a safe default.
        }
    }
}
