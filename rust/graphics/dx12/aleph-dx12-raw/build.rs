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

#[cfg(target_os = "windows")]
fn main() {
    windows::build!(
        windows::win32::direct3d12::*
        windows::win32::dxgi::*
    );

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let src_file = "windows.rs";
    let src_file = format!("{}\\{}", out_dir, src_file);

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let dest_file = "windows.rs";
    let dest_file = format!("{}\\src\\{}", manifest_dir, dest_file);

    std::fs::copy(src_file, dest_file).unwrap();
}

#[cfg(not(target_os = "windows"))]
fn main() {}
