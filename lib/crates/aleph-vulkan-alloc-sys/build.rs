/*
 *
 * This file is a part of NovaEngine
 * https://gitlab.com/MindSpunk/NovaEngine
 *
 *
 * MIT License
 *
 * Copyright (c) 2020 Nathan Voglsam
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

extern crate aleph_compile as compile;
extern crate aleph_target_build as target;

#[cfg(feature = "generate_bindings")]
use bindgen;

use std::path::Path;
use target::build::target_architecture;
use target::build::target_platform;
use target::Architecture;

#[cfg(feature = "generate_bindings")]
fn generate_bindings_internal(path: &Path) {
    if !path.exists() {
        let bindings = bindgen::Builder::default()
            .clang_arg("-Iinclude")
            .header("lib/vk_mem_alloc.h")
            .rustified_enum("Vk.*")
            .rustfmt_bindings(true)
            .blacklist_type("size_t")
            .blacklist_type("__uint8_t")
            .blacklist_type("__uint16_t")
            .blacklist_type("__uint32_t")
            .blacklist_type("__uint64_t")
            .blacklist_type("__int8_t")
            .blacklist_type("__int16_t")
            .blacklist_type("__int32_t")
            .blacklist_type("__int64_t")
            .blacklist_type("__darwin_.*")
            .whitelist_type("PFN_vma.*")
            .whitelist_type("Vma.*")
            .whitelist_function(".*vma.*")
            .trust_clang_mangling(false)
            .layout_tests(false)
            .generate_comments(false)
            .generate()
            .expect("Unable to generate bindings!");

        bindings
            .write_to_file(path)
            .expect("Unable to write bindings!");
    }
}

#[cfg(not(feature = "generate_bindings"))]
fn generate_bindings_internal(_: &Path) {}

fn generate_bindings(path: &Path) {
    generate_bindings_internal(path)
}

fn build_lib() {
    let mut build = cmake::Config::new("library");

    if cfg!(feature = "corruption_detection") {
        build.define("FEATURE_VMA_DEBUG_DETECT_CORRUPTION", "1");
    }

    // Force to compile for release, we'll never need to debug this
    if target_platform().is_msvc() {
        build.profile("Release");
    }

    // Force link stdc++
    if target_platform().is_gnu() && target_platform().is_windows() {
        println!("cargo:rustc-link-lib=dylib=stdc++");
    }

    if target_platform().is_android() {
        let android_home = std::env::var("ANDROID_HOME").unwrap();
        let toolchain_file = format!(
            "{}/ndk-bundle/build/cmake/android.toolchain.cmake",
            &android_home
        );

        build.define("CMAKE_TOOLCHAIN_FILE", &toolchain_file);
        build.define("ANDROID_PLATFORM", "android-24");
        build.define("ANDROID_STL", "c++_shared");
        build.define("ANDROID_ABI", target_architecture().ndk_name());

        match target_architecture() {
            Architecture::X8664 => {
                build.target("x86_64-none-linux-android24");
            }
            Architecture::AARCH64 => {
                build.target("aarch64-none-linux-android24");
            }
            Architecture::Unknown => {
                panic!("Unsupported architecture");
            }
        }
    }

    build.generator("Ninja");

    let output_dir = build.build();

    println!("cargo:rustc-link-search=native={}", output_dir.display());
    //println!("cargo:rustc-link-lib=static=vma");
}

fn main() {
    let path = Path::new("raw.rs");

    generate_bindings(path);
    build_lib();
}
