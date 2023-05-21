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
extern crate aleph_target_build as target;

use std::path::Path;
use target::{Architecture, Platform};

///
/// Main driver for compiling SDL2, handles switching between w/e implementation is needed for the
/// target platforms.
///
fn main() {
    let target_platform = target::build::target_platform();
    let target_arch = target::build::target_architecture();
    match target_platform {
        Platform::WindowsGNU | Platform::WindowsMSVC | Platform::UniversalWindowsMSVC => {
            let arch = match target_arch {
                Architecture::X8664 => "x86_64",
                Architecture::AARCH64 => "aarch64",
                Architecture::Unknown => panic!("Unknown architecture"),
            };
            let vendor = if target_platform.is_uwp() {
                "winrt"
            } else {
                "win32"
            };
            let abi = if target_platform.is_gnu() {
                "gnu"
            } else {
                "msvc"
            };

            let binary_path = format!("./thirdparty/out/{arch}/{vendor}/{abi}");
            let binary_path = Path::new(&binary_path);
            let link_path = binary_path.join("lib").canonicalize().unwrap();

            println!("cargo:rustc-link-search=all={}", link_path.display());

            let source = binary_path.join("bin").join(dll_name());
            compile::copy_file_to_artifacts_dir(&source)
                .expect("Failed to copy SDL2 dll to artifacts dir");
            compile::copy_file_to_target_dir(&source)
                .expect("Failed to copy SDL2 dll to target dir");
        }
        Platform::UniversalWindowsGNU => {
            // We can't compile SDL2 for this platform as it requires msvc
            panic!("Unsupported platform")
        }
        Platform::Linux => {
            // Nothing has to be done on linux as the most sane choice is to use the system provided
            // SDL2 lest we wake the horrible demons of distributing your own libraries on linux.
            // If it's in the distro repository, get it from there as it will probably play much
            // nicer than compiling our own.
        }
        Platform::Android => {
            // On android we need to compile with ndk-build so it will play nicely with all the
            // wacky things android has done for building.

            // Driver function for doing the android compile
            android_compile_sdl2(target::build::target_architecture());

            // Get the location of the link files and add it to rustc's search path
            let mut lib_dir = compile::artifacts_dir();
            lib_dir.push("obj");
            lib_dir.push("local");
            lib_dir.push(target::build::target_architecture().ndk_name());
            println!("cargo:rustc-link-search=all={}", lib_dir.display());
        }
        Platform::MacOS => {
            let arch = match target_arch {
                Architecture::X8664 => panic!("Unsupported architecture+platform"),
                Architecture::AARCH64 => "aarch64",
                Architecture::Unknown => panic!("Unknown architecture"),
            };

            let binary_path = format!("./thirdparty/out/{arch}/macos");
            let binary_path = Path::new(&binary_path);
            let link_path = binary_path.join("lib").canonicalize().unwrap();

            println!("cargo:rustc-link-search=all={}", link_path.display());

            let source = binary_path.join("lib").join("libSDL2-2.0.0.dylib");
            compile::copy_file_to_artifacts_dir(&source)
                .expect("Failed to copy SDL2 dylib to artifacts dir");
            compile::copy_file_to_target_dir(&source)
                .expect("Failed to copy SDL2 dylib to target dir");
        }
        Platform::Unknown => {
            // Do nothing on 'unknown' as a safe default.
        }
    }
}

///
/// Gets the name of the dll/so file that will need to be copied around
///
fn dll_name() -> &'static str {
    match target::build::target_platform() {
        Platform::WindowsGNU
        | Platform::WindowsMSVC
        | Platform::UniversalWindowsGNU
        | Platform::UniversalWindowsMSVC => "SDL2.dll",
        Platform::Linux | Platform::Android => "libSDL2.so",
        Platform::MacOS => "libSDL2-2.0.dylib",
        Platform::Unknown => panic!("Unsupported Platform"),
    }
}

///
/// Gets the name of the ndk-build driver
///
fn get_ndk_build_file() -> String {
    let ndk_build = std::env::var("ANDROID_HOME").unwrap();
    match target::build::host_platform() {
        Platform::WindowsGNU | Platform::WindowsMSVC => {
            format!("{}\\ndk-bundle\\ndk-build.cmd", &ndk_build)
        }
        Platform::Linux => format!("{}/ndk-bundle/ndk-build", &ndk_build),
        Platform::UniversalWindowsGNU => panic!("Unsupported host"),
        Platform::UniversalWindowsMSVC => panic!("Unsupported host"),
        Platform::Android => panic!("Unsupported host"),
        Platform::MacOS => unimplemented!(),
        Platform::Unknown => panic!("Unsupported host"),
    }
}

///
/// Driver for compiling SDL2 for android, handles all the pain of dealing with ndk-build and
/// the different architectures
///
fn android_compile_sdl2(arch: Architecture) {
    let ndk_build_dir = get_ndk_build_file();
    let mut ndk_build = std::process::Command::new(&ndk_build_dir);

    let out_dir = compile::artifacts_dir();

    let mut obj_dir = out_dir.clone();
    obj_dir.push("obj");
    let obj_dir = format!("NDK_OUT={}", obj_dir.display());

    let mut lib_dir = out_dir;
    lib_dir.push("lib");
    let lib_dir = format!("NDK_LIBS_OUT={}", lib_dir.display());

    ndk_build.arg("NDK_PROJECT_PATH=null");
    ndk_build.arg("APP_BUILD_SCRIPT=Android.mk");
    ndk_build.arg("APP_PLATFORM=android-24");
    ndk_build.arg("APP_STL=c++_shared");
    ndk_build.arg("APP_MODULES=SDL2 SDL2_main");
    ndk_build.arg(&obj_dir);
    ndk_build.arg(&lib_dir);

    let abi = format!("APP_ABI={}", arch.ndk_name());
    ndk_build.arg(&abi);

    ndk_build.current_dir("./thirdparty/sdl2");

    ndk_build.stdout(std::process::Stdio::inherit());
    ndk_build.stderr(std::process::Stdio::inherit());

    println!("ndk-build: {}", &ndk_build_dir);
    let exit_status = ndk_build
        .spawn()
        .expect("Failed to start ndk-build")
        .wait()
        .expect("ndk-build failed unexpectedly");

    if !exit_status.success() {
        panic!("ndk-build failed");
    }
}
