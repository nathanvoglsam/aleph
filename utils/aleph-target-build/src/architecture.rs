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

///
/// Enumeration of all supported architectures
///
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Architecture {
    X8664,
    AARCH64,
}

impl Architecture {
    pub fn print_host_cargo_cfg(self) {
        match self {
            Architecture::X8664 => {
                println!("cargo:rustc-cfg=ALEPH_BUILD_ARCH_HOST_is_x86_64");
            }
            Architecture::AARCH64 => {
                println!("cargo:rustc-cfg=ALEPH_BUILD_ARCH_HOST_is_aarch64");
            }
        }
    }

    pub fn print_target_cargo_cfg(self) {
        match self {
            Architecture::X8664 => {
                println!("cargo:rustc-cfg=ALEPH_BUILD_ARCH_TARGET_is_x86_64");
            }
            Architecture::AARCH64 => {
                println!("cargo:rustc-cfg=ALEPH_BUILD_ARCH_TARGET_is_aarch64");
            }
        }
    }

    #[inline]
    pub fn name(self) -> &'static str {
        match self {
            Architecture::X8664 => "x86_64",
            Architecture::AARCH64 => "aarch64",
        }
    }

    #[inline]
    pub fn ndk_name(self) -> &'static str {
        match self {
            Architecture::X8664 => "x86_64",
            Architecture::AARCH64 => "arm64-v8a",
        }
    }

    ///
    /// Are we building for x86-64
    ///
    #[inline]
    pub fn is_x86_64(self) -> bool {
        self == Architecture::X8664
    }

    ///
    /// Are we building for aarch64 (ARM 64bit)
    ///
    #[inline]
    pub fn is_aarch64(self) -> bool {
        self == Architecture::AARCH64
    }
}

#[inline]
pub fn get_architecture_from(triple: &str) -> Architecture {
    let target = triple;

    if target.contains("x86_64") {
        Architecture::X8664
    } else if target.contains("aarch64") {
        Architecture::AARCH64
    } else {
        panic!("Unsupported Architecture");
    }
}
