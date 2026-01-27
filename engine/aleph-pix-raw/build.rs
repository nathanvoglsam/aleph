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

#[cfg(not(windows))]
fn main() {}

#[cfg(windows)]
fn main() {
    use std::path::{Path, PathBuf};

    use aleph_compile::deps_path;
    use aleph_target::{Architecture, Platform};

    if !Platform::build_target().is_windows() {
        // This script should do nothing if we're not building for windows
        return;
    }

    let mut pkg_path = deps_path();
    pkg_path.extend(["build", "_deps", "win_pix_event_runtime-src"]);

    println!("cargo::rerun-if-changed={}", pkg_path.display());

    let include_dir = pkg_path.join("Include").join("WinPixEventRuntime");

    // Whether we should link to WinPixEventRuntime depends on whether we used the no-op shim or not
    let should_link = if Platform::build_target().is_gnu() {
        cc::Build::new()
            .cpp(true)
            .link_lib_modifier("-bundle")
            .file("cpp/shim_noop.cpp")
            .flag("-w")
            .include(&include_dir)
            .compile("winpix_shim");
        false
    } else {
        cc::Build::new()
            .cpp(true)
            .link_lib_modifier("-bundle")
            .file("cpp/shim.cpp")
            .include(&include_dir)
            .compile("winpix_shim");
        true
    };

    if should_link {
        let arch = match Architecture::build_target() {
            Architecture::X8664 => "x64",
            Architecture::AARCH64 => "ARM64",
            Architecture::Unknown => panic!("Unknown architecture"),
        };

        let bin_dir = pkg_path.join("bin").join(arch);

        let dll_path = bin_dir.join("WinPixEventRuntime.dll");
        aleph_compile::copy_file_to_artifacts_dir(&dll_path).unwrap();
        aleph_compile::copy_file_to_target_dir(&dll_path).unwrap();

        let link_path_full = bin_dir.canonicalize().unwrap().display();
        println!("cargo:rustc-link-search=native={link_path_full}");
        println!("cargo:rustc-link-lib=dylib=WinPixEventRuntime");
    }
}
