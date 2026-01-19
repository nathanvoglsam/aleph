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
    use std::path::Path;

    use aleph_target::{Architecture, Platform};

    if !Platform::build_target().is_windows() {
        // This script should do nothing if we're building for windows
        return;
    }

    // Whether we should link to WinPixEventRuntime depends on whether we used the no-op shim or not
    let should_link = if Platform::build_target().is_gnu() {
        cc::Build::new()
            .cpp(true)
            .link_lib_modifier("-bundle")
            .file("cpp/shim_noop.cpp")
            .flag("-w")
            .include("WinPixEventRuntime/Include/WinPixEventRuntime")
            .compile("winpix_shim");
        false
    } else {
        cc::Build::new()
            .cpp(true)
            .link_lib_modifier("-bundle")
            .file("cpp/shim.cpp")
            .include("WinPixEventRuntime/Include/WinPixEventRuntime")
            .compile("winpix_shim");
        true
    };

    if should_link {
        let arch = match Architecture::build_target() {
            Architecture::X8664 => "x64",
            Architecture::AARCH64 => "ARM64",
            Architecture::Unknown => panic!("Unknown architecture"),
        };
        let win32_dll = format!("./WinPixEventRuntime/bin/{arch}/WinPixEventRuntime.dll");
        let link_path = format!("./WinPixEventRuntime/bin/{arch}");
        let link_path = Path::new(&link_path).canonicalize().unwrap();
        let link_path_full = link_path.display();

        let src = Path::new(&win32_dll);
        aleph_compile::copy_file_to_artifacts_dir(src).unwrap();
        aleph_compile::copy_file_to_target_dir(src).unwrap();
        println!("cargo:rustc-link-search=native={link_path_full}");
        println!("cargo:rustc-link-lib=dylib=WinPixEventRuntime");
    }
}
