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

use target::build::{target_architecture, target_platform};
use target::Platform;

fn main() {
    if target_platform().is_android() {
        let arch = target_architecture();

        let ndk_build_dir = get_ndk_build_file();
        let mut ndk_build = std::process::Command::new(&ndk_build_dir);
        ndk_build.current_dir("./library");
        ndk_build.stdout(std::process::Stdio::inherit());
        ndk_build.stderr(std::process::Stdio::inherit());

        let out_dir = compile::cargo_out_dir();
        let obj_dir = out_dir.join("obj");
        let lib_dir = out_dir.join("lib");

        let abi = format!("APP_ABI={}", arch.ndk_name());
        let obj_dir_arg = format!("NDK_OUT={}", obj_dir.display());
        let lib_dir_arg = format!("NDK_LIBS_OUT={}", lib_dir.display());

        ndk_build.arg("NDK_PROJECT_PATH=null");
        ndk_build.arg("APP_BUILD_SCRIPT=Android.mk");
        ndk_build.arg("APP_PLATFORM=android-30");
        ndk_build.arg("APP_STL=c++_shared");
        ndk_build.arg("APP_MODULES=vma");
        ndk_build.arg(&obj_dir_arg);
        ndk_build.arg(&lib_dir_arg);
        ndk_build.arg(&abi);

        println!("ndk-build: {}", &ndk_build_dir);
        let exit_status = ndk_build
            .spawn()
            .expect("Failed to start ndk-build")
            .wait()
            .expect("ndk-build failed unexpectedly");

        if !exit_status.success() {
            panic!("ndk-build failed");
        }

        // Copy the output archive to the OUT_DIR so we don't get the copied libc++_shared messing
        // our linking up
        let archive_dir = obj_dir.join("local").join(arch.ndk_name());
        let lib = archive_dir.join("libvma.a");
        std::fs::copy(lib, out_dir.join("libvma.a")).unwrap();

        println!("cargo:rustc-link-search=all={}", out_dir.display());
        println!("cargo:rustc-link-lib=static=vma");
    } else {
        let mut build = cc::Build::new();
        build.cpp(true);
        build.debug(true);
        build.flag_if_supported("-w");
        build.define("VMA_STATIC_VULKAN_FUNCTIONS", "0");
        build.define("VMA_USE_STL_SHARED_MUTEX", "1");
        build.define("VMA_STATS_STRING_ENABLED", "0");
        build.file("VulkanMemoryAllocator/src/VmaUsage.cpp");
        build.include("Vulkan-Headers/include");
        build.include("VulkanMemoryAllocator/include");
        build.include("VulkanMemoryAllocator/src");

        if target_platform().is_windows() {
            if target_platform().is_gnu() {
                build.flag("-std=c++17");
            } else {
                build.flag("/std:c++17");
            }
        } else {
            build.flag("-std=c++17");
        }

        build.compile("vma");
    }
}

///
/// Gets the name of the ndk-build driver
///
fn get_ndk_build_file() -> String {
    let ndk_build = std::env::var("ANDROID_NDK_HOME").unwrap();
    match target::build::host_platform() {
        Platform::WindowsGNU | Platform::WindowsMSVC => format!("{ndk_build}\\ndk-build.cmd"),
        Platform::Linux | Platform::MacOS => format!("{ndk_build}/ndk-build"),
        Platform::Android => panic!("Unsupported host"),
        Platform::UniversalWindowsGNU => panic!("Unsupported host"),
        Platform::UniversalWindowsMSVC => panic!("Unsupported host"),
        Platform::Unknown => panic!("Unsupported host"),
    }
}
