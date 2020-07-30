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

use crate::{
    get_architecture_from, get_build_type_from, get_platform_from, Architecture, BuildType,
    Platform,
};
use std::env;

///
/// Returns the host architecture
///
/// # Warning
///
/// Only works in a build script
///
#[inline]
pub fn host_architecture() -> Architecture {
    get_architecture_from(&env::var("HOST").unwrap())
}

///
/// Returns the target architecture
///
/// # Warning
///
/// Only works in a build script
///
#[inline]
pub fn target_architecture() -> Architecture {
    get_architecture_from(&env::var("TARGET").unwrap())
}

///
/// Returns the host build profile
///
/// # Warning
///
/// Only works in a build script
///
#[inline]
pub fn host_build_type() -> BuildType {
    target_build_type()
}

///
/// Returns the target build profile
///
/// # Warning
///
/// Only works in a build script
///
#[inline]
pub fn target_build_type() -> BuildType {
    get_build_type_from(&env::var("PROFILE").unwrap())
}

///
/// Returns the host platform (operating system)
///
/// # Warning
///
/// Only works in a build script
///
#[inline]
pub fn host_platform() -> Platform {
    get_platform_from(&env::var("HOST").unwrap())
}

///
/// Returns the target platform (operating system)
///
/// # Warning
///
/// Only works in a build script
///
#[inline]
pub fn target_platform() -> Platform {
    get_platform_from(&env::var("TARGET").unwrap())
}
