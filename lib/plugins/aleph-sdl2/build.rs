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

use std::path::PathBuf;

use target::Platform;

///
/// Main driver for compiling SDL2, handles switching between w/e implementation is needed for the
/// target platforms.
///
fn main() {
    let target_platform = target::build::target_platform();
    let target_arch = target::build::target_architecture();

    let mut binary_path = PathBuf::new();
    binary_path.push("thirdparty");
    binary_path.push("out");
    binary_path.push(compile::standard_binary_path_for(target_platform, target_arch).unwrap());

    let bin_path = binary_path.join("bin");
    let lib_path = binary_path.join("lib");

    // On windows the .dll is found in the 'bin' directory. On unix-likes the .so/.dylib is found in
    // the 'lib' directory. Handle it here so everything can be common code (except iOS).
    let dll_name = dll_name(target_platform);
    let dll_path = if target_platform.is_windows() {
        bin_path.join(dll_name)
    } else {
        lib_path.join(dll_name)
    };

    match target_platform {
        Platform::WindowsGNU
        | Platform::WindowsMSVC
        | Platform::UniversalWindowsMSVC
        | Platform::UniversalWindowsGNU
        | Platform::MacOS
        | Platform::Android => {
            println!(
                "cargo:rustc-link-search=all={}",
                lib_path.canonicalize().unwrap().display()
            );

            compile::copy_file_to_artifacts_dir(&dll_path)
                .expect("Failed to copy SDL2 dll to artifacts dir");
            compile::copy_file_to_target_dir(&dll_path)
                .expect("Failed to copy SDL2 dll to target dir");
        }
        Platform::Linux => {
            // Nothing has to be done on linux as the most sane choice is to use the system provided
            // SDL2 lest we wake the horrible demons of distributing your own libraries on linux.
            // If it's in the distro repository, get it from there as it will probably play much
            // nicer than compiling our own.
        }
        Platform::IOS => {
            // On iOS we link to a framework rather than directly to a binary. We just need to set
            // the search path, the sdl2 crate handles the linker directives.
            println!(
                "cargo:rustc-link-search=framework={}",
                binary_path.canonicalize().unwrap().display()
            );
        }
        Platform::Unknown => {
            // Do nothing on 'unknown' as a safe default.
        }
    }
}

///
/// Gets the name of the dll/so file that will need to be copied around
///
const fn dll_name(platform: Platform) -> &'static str {
    match platform {
        Platform::WindowsGNU
        | Platform::WindowsMSVC
        | Platform::UniversalWindowsGNU
        | Platform::UniversalWindowsMSVC => "SDL2.dll",
        Platform::Linux | Platform::Android => "libSDL2.so",
        Platform::MacOS => "libSDL2-2.0.0.dylib",
        Platform::IOS => "",
        Platform::Unknown => "",
    }
}
