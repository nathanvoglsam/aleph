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

use aleph_target::Platform;

fn main() {
    let target_platform = Platform::build_target();
    if target_platform.is_ios() {
        let mut framework_path = compile::deps_path();
        framework_path.extend(["build", "_deps", "moltenvk_ios-src", "MoltenVK", "static"]);

        println!("cargo::rerun-if-changed=src");
        println!("cargo::rerun-if-changed={}", framework_path.display());

        println!(
            "cargo:rustc-link-search=framework={}",
            framework_path.display()
        );
        println!(
            "cargo:rustc-link-search=framework={}/MoltenVK.xcframework/ios-arm64",
            framework_path.display(),
        );
    }
}
