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

use crate::{Architecture, Platform};

///
/// Takes a platform and architecture and produces a rust target triple
///
/// Returns None if the triple is not a valid rust target
///
pub fn recreate_triple(platform: Platform, arch: Architecture) -> Option<&'static str> {
    match arch {
        Architecture::X8664 => match platform {
            Platform::WindowsGNU => Some("x86_64-pc-windows-gnu"),
            Platform::WindowsMSVC => Some("x86_64-pc-windows-msvc"),
            Platform::Linux => Some("x86_64-unknown-linux-gnu"),
            Platform::Android => Some("x86_64-linux-android"),
        },
        Architecture::AARCH64 => match platform {
            Platform::WindowsGNU => None,
            Platform::WindowsMSVC => Some("aarch64-pc-windows-msvc"),
            Platform::Linux => Some("aarch64-unknown-linux-gnu"),
            Platform::Android => Some("aarch64-linux-android"),
        },
    }
}
