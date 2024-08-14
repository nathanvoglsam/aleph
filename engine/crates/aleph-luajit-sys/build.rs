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
/// Main driver for compiling luajit, handles switching between w/e implementation is needed for the
/// target platforms.
///
fn main() {
    let target_platform = target::build::target_platform();
    let target_arch = target::build::target_architecture();

    let mut binary_path = PathBuf::new();
    binary_path.push("thirdparty");
    binary_path.push("out");
    binary_path.push(compile::standard_binary_path_for(target_platform, target_arch).unwrap());

    let lib_path = binary_path.join("lib");
    let lib_name = lib_name(target_platform);

    match target_platform {
        Platform::WindowsGNU
        | Platform::WindowsMSVC
        | Platform::UniversalWindowsMSVC
        | Platform::UniversalWindowsGNU
        | Platform::MacOS
        | Platform::Android
        | Platform::IOS
        | Platform::Linux => {
            println!(
                "cargo:rustc-link-search=all={}",
                lib_path.canonicalize().unwrap().display()
            );
            println!("cargo::rustc-link-lib=static={lib_name}");
        }
        Platform::Unknown => {
            // Do nothing on 'unknown' as a safe default.
        }
    }
}

///
/// Gets the name of the lib file that will need to be linked to
///
const fn lib_name(platform: Platform) -> &'static str {
    match platform {
        Platform::WindowsGNU
        | Platform::WindowsMSVC
        | Platform::UniversalWindowsGNU
        | Platform::UniversalWindowsMSVC
        | Platform::Linux
        | Platform::Android
        | Platform::MacOS => "luajit",
        Platform::IOS => "luajit-5.1",
        Platform::Unknown => "",
    }
}
