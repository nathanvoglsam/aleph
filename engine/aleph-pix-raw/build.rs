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

cfg_select! {
    windows => {
        fn main() {
            use aleph_target::{Architecture, BuildConfig, Platform};

            if !Platform::build_target().is_windows() {
                // This script should do nothing if we're not building for windows
                return;
            }

            println!("cargo::rerun-if-changed=src");
            println!("cargo::rerun-if-changed=cmake");

            let mut config = cmake::Config::new("cmake");

            // We can't build 'Debug' on Windows because we can't link with the debug C runtime
            if BuildConfig::build_target().is_debug() {
                config.profile("RelWithDebInfo");
            } else {
                config.profile("Release");
            }

            let cmake_path = config.build();

            let mut pkg_path = cmake_path.clone();
            pkg_path.extend(["build", "_deps", "win_pix_event_runtime-src"]);

            if let Some(local_root) = aleph_compile::local_root() {
                let arch = match Architecture::build_target() {
                    Architecture::X8664 => "x64",
                    Architecture::AARCH64 => "ARM64",
                    Architecture::Unknown => panic!("Unknown architecture"),
                };

                let bin_dir = pkg_path.join("bin").join(arch);
                let dst_dir = local_root.join("pix").join(arch);

                std::fs::create_dir_all(&dst_dir).unwrap();

                let src = bin_dir.join("WinPixEventRuntime.dll");
                let dst = dst_dir.join("WinPixEventRuntime.dll");
                std::fs::copy(&src, &dst).unwrap();

                let src = cmake_path.join("bin").join("aleph_pix_shim.dll");
                let dst = dst_dir.join("aleph_pix_shim.dll");
                std::fs::copy(&src, &dst).unwrap();
            }
        }
    }
    _ => {
        fn main() {}
    }
}
