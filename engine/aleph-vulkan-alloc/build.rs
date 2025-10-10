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

use target::build::target_platform;

fn main() {
    let mut build = cc::Build::new();
    build.cpp(true);
    build.link_lib_modifier("-bundle");
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
