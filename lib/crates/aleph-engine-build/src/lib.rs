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

use aleph_target_build::build::target_platform;

pub fn add_platform_flags() {
    if target_platform().is_windows() {
        compile_and_link_windows_resource_file();
        aleph_dx12_agility_sdk::link_agility_symbol_def();
        aleph_dx12_agility_sdk::extract_agility_sdk_binaries();
    }

    aleph_target_build::build::target_build_config().print_target_cargo_cfg();
    aleph_target_build::build::target_build_type().print_target_cargo_cfg();

    if target_platform().is_macos() {
        println!("cargo:rustc-link-arg=-Wl,-rpath,@loader_path");
    }

    if target_platform().is_linux() {
        println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN");
    }

    if target_platform().is_android() {
        println!("cargo:rustc-link-lib=dylib=c++_shared");
    }
}

#[cfg(windows)]
fn compile_and_link_windows_resource_file() {
    use std::path::{Path, PathBuf};

    let icon_path = std::env::var("ALEPH_WIN32_ICON_FILE").unwrap_or_else(|_| {
        let v = Path::new(env!("CARGO_MANIFEST_DIR")).join("app_icon.ico");
        v.to_str().unwrap().to_string()
    });
    let icon_path = icon_path.replace('\\', "/");

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_dir = PathBuf::from(out_dir);
    let rc_file = out_dir.join("icon.rc");

    std::fs::write(&rc_file, format!("IDI_ICON1 ICON \"{}\"", icon_path)).unwrap();

    let name = std::env::var("CARGO_PKG_NAME").unwrap();
    embed_resource::compile_for(&rc_file, [&name], embed_resource::NONE);
}

#[cfg(not(windows))]
fn compile_and_link_windows_resource_file() {}
