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
            let arch = match target_arch {
                Architecture::X8664 => "x86_64",
                Architecture::AARCH64 => "aarch64",
                Architecture::Unknown => panic!("Unknown architecture"),
            };

            let binary_path = format!("./thirdparty/out/{arch}/android/api-30");
            let binary_path = Path::new(&binary_path);
            let link_path = binary_path.join("lib").canonicalize().unwrap();

            println!("cargo:rustc-link-search=all={}", link_path.display());

            let source = binary_path.join("lib").join("libSDL2.so");
            compile::copy_file_to_artifacts_dir(&source)
                .expect("Failed to copy SDL2 .so to artifacts dir");
            compile::copy_file_to_target_dir(&source)
                .expect("Failed to copy SDL2 .so to target dir");
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
