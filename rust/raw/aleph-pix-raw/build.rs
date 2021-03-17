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

use aleph_target_build::build::target_platform;
use std::io::{Cursor, Read};
use std::path::Path;
use zip::ZipArchive;

fn main() {
    const HEADER_1: &str = "Include/WinPixEventRuntime/pix3.h";
    const HEADER_1_NAME: &str = "pix3.h";
    const HEADER_2: &str = "Include/WinPixEventRuntime/pix3_win.h";
    const HEADER_2_NAME: &str = "pix3_win.h";
    const HEADER_3: &str = "Include/WinPixEventRuntime/PIXEvents.h";
    const HEADER_3_NAME: &str = "PIXEvents.h";
    const HEADER_4: &str = "Include/WinPixEventRuntime/PIXEventsCommon.h";
    const HEADER_4_NAME: &str = "PIXEventsCommon.h";

    const WIN32_DLL: &str = "bin/x64/WinPixEventRuntime.dll";
    const WIN32_LIB: &str = "bin/x64/WinPixEventRuntime.lib";
    const WINRT_DLL: &str = "bin/x64/WinPixEventRuntime_UAP.dll";
    const WINRT_LIB: &str = "bin/x64/WinPixEventRuntime_UAP.lib";

    const WIN32_DLL_NAME: &str = "WinPixEventRuntime.dll";
    const WIN32_LIB_NAME: &str = "WinPixEventRuntime.lib";
    const WINRT_DLL_NAME: &str = "WinPixEventRuntime_UAP.dll";
    const WINRT_LIB_NAME: &str = "WinPixEventRuntime_UAP.lib";

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir);

    let bytes = std::fs::read("pkg/winpixeventruntime.1.0.200127001.nupkg").unwrap();
    let cursor = Cursor::new(bytes.as_slice());
    let mut package = zip::ZipArchive::new(cursor).unwrap();

    write_zip_file(&mut package, out_dir, HEADER_1, HEADER_1_NAME);
    write_zip_file(&mut package, out_dir, HEADER_2, HEADER_2_NAME);
    write_zip_file(&mut package, out_dir, HEADER_3, HEADER_3_NAME);
    write_zip_file(&mut package, out_dir, HEADER_4, HEADER_4_NAME);
    write_zip_file(&mut package, out_dir, WIN32_DLL, WIN32_DLL_NAME);
    write_zip_file(&mut package, out_dir, WIN32_LIB, WIN32_LIB_NAME);
    write_zip_file(&mut package, out_dir, WINRT_DLL, WINRT_DLL_NAME);
    write_zip_file(&mut package, out_dir, WINRT_LIB, WINRT_LIB_NAME);

    if target_platform().is_gnu() || !target_platform().is_windows() {
        cc::Build::new()
            .cpp(true)
            .file("cpp/shim_noop.cpp")
            .flag("-w")
            .include(out_dir)
            .compile("winpix_shim");
    } else {
        cc::Build::new()
            .cpp(true)
            .file("cpp/shim.cpp")
            .include(out_dir)
            .compile("winpix_shim");
    };

    if target_platform().is_uwp() {
        let src = out_dir.join(WINRT_DLL_NAME);
        aleph_compile::copy_file_to_artifacts_dir(&src).unwrap();
        aleph_compile::copy_file_to_target_dir(&src).unwrap();
        println!("cargo:rustc-link-lib=dylib=WinPixEventRuntime_UAP");
    } else if target_platform().is_windows() {
        let src = out_dir.join(WIN32_DLL_NAME);
        aleph_compile::copy_file_to_artifacts_dir(&src).unwrap();
        aleph_compile::copy_file_to_target_dir(&src).unwrap();
        println!("cargo:rustc-link-lib=dylib=WinPixEventRuntime");
    }
}

fn write_zip_file<R: Read + std::io::Seek>(
    package: &mut ZipArchive<R>,
    out_dir: &Path,
    zip_path: &str,
    name: &str,
) {
    // Try to get the file, asserting it is in fact a file
    let mut file = package.by_name(zip_path).unwrap();
    assert!(file.is_file());

    // Read the bytes into a buffer
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).unwrap();

    // Output the bytes into the file
    let out = out_dir.join(name);
    std::fs::write(out, bytes).unwrap();
}
