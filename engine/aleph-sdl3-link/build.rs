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
/// Main driver for compiling SDL3, handles switching between w/e implementation is needed for the
/// target platforms.
///
fn main() {
    println!("cargo::rerun-if-changed=src");
    println!("cargo::rerun-if-changed=cmake");

    let target_platform = Platform::build_target();
    match target_platform {
        Platform::WindowsGNU | Platform::WindowsMSVC => {
            let mut config = cmake::Config::new("cmake");
            if let Some(toolchain) = compile::mingw_cross_toolchain_file() {
                config.define("CMAKE_TOOLCHAIN_FILE", toolchain);
            }

            // We can't build 'Debug' on Windows because we can't link with the debug C runtime
            if BuildConfig::build_target().is_debug() {
                config.profile("RelWithDebInfo");
            } else {
                config.profile("Release");
            }

            let dst = config.build();
            let dst = dst.join("lib");
            println!("cargo:rustc-link-search=native={}", dst.display());
            // The MSVC build of SDL3 produces 'SDL3-static.lib' while the mingw build produces
            // 'libSDL3.a'
            if target_platform.is_msvc() {
                println!("cargo:rustc-link-lib=static=SDL3-static");
            } else {
                println!("cargo:rustc-link-lib=static=SDL3");
            }

            // Hand unrolled from the generated pkg-config. These are the dependencies of
            // SDL3 that we must link to as well.
            println!("cargo:rustc-link-lib=kernel32");
            println!("cargo:rustc-link-lib=user32");
            println!("cargo:rustc-link-lib=gdi32");
            println!("cargo:rustc-link-lib=winmm");
            println!("cargo:rustc-link-lib=imm32");
            println!("cargo:rustc-link-lib=ole32");
            println!("cargo:rustc-link-lib=oleaut32");
            println!("cargo:rustc-link-lib=version");
            println!("cargo:rustc-link-lib=uuid");
            println!("cargo:rustc-link-lib=advapi32");
            println!("cargo:rustc-link-lib=setupapi");
            println!("cargo:rustc-link-lib=shell32");
            println!("cargo:rustc-link-lib=dinput8");
        }
        Platform::MacOS => {
            let dst = cmake::Config::new("cmake")
                .define("CMAKE_OSX_DEPLOYMENT_TARGET", "15.0")
                .build();
            let dst = dst.join("lib");
            println!("cargo:rustc-link-search=native={}", dst.display());
            println!("cargo:rustc-link-lib=static=SDL3");

            // Hand unrolled from the generated pkg-config. These are the dependencies of
            // SDL3 that we must link to as well.
            println!("cargo:rustc-link-lib=framework=CoreMedia");
            println!("cargo:rustc-link-lib=framework=CoreVideo");
            println!("cargo:rustc-link-lib=framework=Cocoa");
            println!("cargo:rustc-link-lib=framework=UniformTypeIdentifiers");
            println!("cargo:rustc-link-lib=framework=IOKit");
            println!("cargo:rustc-link-lib=framework=ForceFeedback");
            println!("cargo:rustc-link-lib=framework=Carbon");
            println!("cargo:rustc-link-lib=framework=CoreAudio");
            println!("cargo:rustc-link-lib=framework=AudioToolbox");
            println!("cargo:rustc-link-lib=framework=AVFoundation");
            println!("cargo:rustc-link-lib=framework=Foundation");
            println!("cargo:rustc-link-lib=framework=GameController");
            println!("cargo:rustc-link-lib=framework=Metal");
            println!("cargo:rustc-link-lib=framework=QuartzCore");
            println!("cargo:rustc-link-lib=framework=CoreHaptics");
        }
        Platform::Linux => {
            // Nothing has to be done on linux as the most sane choice is to use the system provided
            // SDL3.
            println!("cargo:rustc-link-lib=dylib=SDL3");
        }
        Platform::IOS => {
            let dst = cmake::Config::new("cmake").build();
            let dst = dst.join("lib");

            println!("cargo:rustc-link-search=native={}", dst.display());
            println!("cargo:rustc-link-lib=static=SDL3-static");
            todo!();
        }
        Platform::Android => {
            // Do nothing, we rely on the corrosion cmake build system to handle linking SDL3 on
            // Android.
        }
        Platform::Unknown => {
            // Do nothing on 'unknown' as a safe default.
        }
    }
}
