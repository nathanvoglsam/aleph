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
    if target_platform().is_windows() {
        const CORE_DLL: &str = "build/native/bin/x64/D3D12Core.dll";
        const CORE_NAME: &str = "D3D12Core.dll";
        const LAYERS_DLL: &str = "build/native/bin/x64/d3d12SDKLayers.dll";
        const LAYERS_NAME: &str = "d3d12SDKLayers.dll";

        let out_dir = std::env::var("OUT_DIR").unwrap();
        let out_dir = Path::new(&out_dir);

        let bytes = std::fs::read("pkg/microsoft.direct3d.d3d12.1.706.3-preview.nupkg").unwrap();
        let cursor = Cursor::new(bytes.as_slice());
        let mut package = ZipArchive::new(cursor).unwrap();

        write_zip_file(&mut package, out_dir, CORE_DLL, CORE_NAME);
        write_zip_file(&mut package, out_dir, LAYERS_DLL, LAYERS_NAME);

        let src = out_dir.join(CORE_NAME);
        aleph_compile::copy_file_to_artifacts_dir(&src).unwrap();
        aleph_compile::copy_file_to_target_dir(&src).unwrap();

        let src = out_dir.join(LAYERS_NAME);
        aleph_compile::copy_file_to_artifacts_dir(&src).unwrap();
        aleph_compile::copy_file_to_target_dir(&src).unwrap();
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
