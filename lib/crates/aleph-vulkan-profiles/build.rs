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

use std::path::Path;
use target::build::{target_build_type, target_platform};
use target::BuildType;

fn main() {
    let vk_header_inc = Path::new("../../../submodules/Vulkan-Headers/include");

    let mut build = cc::Build::new();
    build.cpp(true);
    build.flag_if_supported("-w");
    build.include(vk_header_inc);
    build.include("library/include");
    build.define("VK_NO_PROTOTYPES", "1");

    if target_platform().is_windows() {
        if target_platform().is_gnu() {
            build.flag("-std=c++17");
        } else {
            build.flag("/std:c++17");
        }
    } else {
        build.flag("-std=c++17");
    }

    if target_build_type() == BuildType::Development {
        build.file("library/source/debug/vulkan_profiles.cpp");
        build.file("library/source/debug_callback_slot.cpp");
        build.define("VP_DEBUG_MESSAGE_CALLBACK", "vpAlephDebugMessageTrampoline");
    } else {
        build.file("library/source/vulkan_profiles.cpp");
        build.file("library/source/debug_callback_slot.cpp");
    }

    // if target_platform().is_android() {
    //     let android_home = std::env::var("ANDROID_HOME").unwrap();
    //     let toolchain_file = format!(
    //         "{}/ndk-bundle/build/cmake/android.toolchain.cmake",
    //         &android_home
    //     );
    //
    //     build.define("CMAKE_TOOLCHAIN_FILE", &toolchain_file);
    //     build.define("ANDROID_PLATFORM", "android-24");
    //     build.define("ANDROID_STL", "c++_shared");
    //     build.define("ANDROID_ABI", target_architecture().ndk_name());
    //
    //     match target_architecture() {
    //         Architecture::X8664 => {
    //             build.target("x86_64-none-linux-android24");
    //         }
    //         Architecture::AARCH64 => {
    //             build.target("aarch64-none-linux-android24");
    //         }
    //         Architecture::Unknown => {
    //             panic!("Unsupported architecture");
    //         }
    //     }
    // }

    build.compile("vulkan_profiles");
}
