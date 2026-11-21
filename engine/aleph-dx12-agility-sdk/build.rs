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

use std::path::{Path, PathBuf};

use aleph_target::{Architecture, Platform};

pub fn main() {
    if Platform::build_target().is_windows() {
        println!("cargo::rerun-if-changed=src");
        println!("cargo::rerun-if-changed=cmake");

        let mut config = cmake::Config::new("cmake");
        config.no_build_target(true);
        if let Some(toolchain) = aleph_compile::mingw_cross_toolchain_file() {
            config.define("CMAKE_TOOLCHAIN_FILE", toolchain);
        }
        let mut pkg_location = config.build();
        pkg_location.extend(["build", "_deps", "d3d12_agility_sdk-src", "build", "native"]);

        if let Some(local_root) = aleph_compile::local_root() {
            let arch = match Architecture::build_target() {
                Architecture::X8664 => "x64",
                Architecture::AARCH64 => "arm64",
                Architecture::Unknown => panic!("Unknown architecture"),
            };

            let dst_dir = local_root.join("D3D12").join(arch);
            std::fs::create_dir_all(&dst_dir).unwrap();

            let bin_dir = pkg_location.join("bin").join(arch);

            let src = bin_dir.join("D3D12Core.dll");
            let dst = dst_dir.join("D3D12Core.dll");
            std::fs::copy(&src, &dst).unwrap();

            let src = bin_dir.join("d3d12SDKLayers.dll");
            let dst = dst_dir.join("d3d12SDKLayers.dll");
            std::fs::copy(&src, &dst).unwrap();

            let src = bin_dir.join("D3D12Core.pdb");
            let dst = dst_dir.join("D3D12Core.pdb");
            std::fs::copy(&src, &dst).unwrap();

            let src = bin_dir.join("d3d12SDKLayers.pdb");
            let dst = dst_dir.join("d3d12SDKLayers.pdb");
            std::fs::copy(&src, &dst).unwrap();
        }

        // Copy the file into a temp directory for the building exe crate so we don't lock on the
        // symbol file if multiple crates are trying to use it simultaneously.
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let out_dir = Path::new(&out_dir);
        let def_file = out_dir.join("symbols.def");
        std::fs::copy(def_location().unwrap(), &def_file).unwrap();

        // Add the linker flag
        if Platform::build_target().is_msvc() {
            println!("cargo:rustc-link-arg=/DEF:{}", def_file.display());
        } else {
            println!("cargo:rustc-link-arg={}", def_file.display());
        }
    }
}

///
/// Internal function which returns the location of the .def file for giving to the linker
///
fn def_location() -> Option<PathBuf> {
    std::env::var_os("CARGO_MANIFEST_DIR")
        .as_deref()
        .map(Path::new)
        .map(|v| v.join("symbols.def"))
}
