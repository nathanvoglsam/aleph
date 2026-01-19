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

use std::path::Path;

fn main() {
    use std::fmt::Write;

    let dst = std::env::var("OUT_DIR").unwrap();
    let dst = Path::new(&dst).join("generated.rs");

    let mut generated = String::new();

    // output the whole target triple
    let target = std::env::var("TARGET").unwrap();

    // output the target architecture
    let arch = if target.contains("x86_64") {
        "X8664"
    } else if target.contains("aarch64") {
        "AARCH64"
    } else {
        "Unknown"
    };
    writeln!(
        &mut generated,
        "pub const ARCH: crate::Architecture = crate::Architecture::{};",
        &arch
    )
    .unwrap();

    // output the target platform
    let platform = if target.contains("pc-windows") {
        if target.contains("msvc") {
            "WindowsMSVC"
        } else if target.contains("gnu") {
            "WindowsGNU"
        } else {
            "Unknown"
        }
    } else if target.contains("linux") {
        "Linux"
    } else if target.contains("apple-darwin") {
        "MacOS"
    } else if target.contains("apple-ios") {
        "IOS"
    } else {
        "Unknown"
    };
    writeln!(
        &mut generated,
        "pub const PLATFORM: crate::Platform = crate::Platform::{};",
        &platform
    )
    .unwrap();

    // output the build configuration
    let debug = std::env::var("DEBUG").unwrap() == "true";
    let optimized = std::env::var("OPT_LEVEL").unwrap() != "0";
    writeln!(
        &mut generated,
        "pub const CONFIG: crate::BuildConfig = crate::BuildConfig::new({debug}, {optimized});"
    )
    .unwrap();

    std::fs::write(&dst, &generated).unwrap();

    println!("cargo::rerun-if-changed=src");
}
