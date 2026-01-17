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

use target::Platform;

///
/// Main driver for compiling luajit, handles switching between w/e implementation is needed for the
/// target platforms.
///
fn main() {
    let deps_install_path = compile::deps_path().join("install");

    println!("cargo::rerun-if-changed=src");
    println!("cargo::rerun-if-changed={}", deps_install_path.display());

    let mut lib_path = deps_install_path.join("lib");
    if !lib_path.exists() {
        // Because GNU likes to be difficult with their standard paths sometimes we get this
        // rubbish. I love making build logic more complicated!!!!
        lib_path = deps_install_path.join("lib64");
    }

    let target_platform = target::build::target_platform();
    match target_platform {
        Platform::WindowsGNU
        | Platform::WindowsMSVC
        | Platform::MacOS
        | Platform::IOS
        | Platform::Linux => {
            println!(
                "cargo:rustc-link-search=all={}",
                lib_path.canonicalize().unwrap().display()
            );
            println!("cargo::rustc-link-lib=static=qjs");
        }
        Platform::Unknown => {
            // Do nothing on 'unknown' as a safe default.
        }
    }
}
