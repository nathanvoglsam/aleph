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
use std::path::{Path, PathBuf};
use zip::ZipArchive;

///
/// This function will perform the necessary work to allow rust to export the `D3D12SDKVersion` and
/// `D3D12SDKPath` symbols, as defined in rust code.
///
/// For the `.def` file and link argument to work this function *must* be called in a build.rs
/// script directly in the final 'bin' crate. Rust will not propagate linker arguments from library
/// dependencies to binary crates.
///
/// This function works by defining and linking in a .def file that tells the linker that the two
/// symbols *must* be exported.
///
/// # Important
///
/// Using this utility and not defining `D3D12SDKVersion` and `D3D12SDKPath` somewhere in your code
/// *will* cause a linker error for undefined symbols. You must define these symbols.
///
/// This currently works for the following targets:
///
/// - `x86_64-pc-windows-msvc`
/// - `x86_64-uwp-windows-msvc`
/// - `i686-pc-windows-msvc`
/// - `i686-uwp-windows-msvc`
///
/// This function will do nothing on non windows platforms.
///
pub fn link_agility_symbol_def() {
    if target_platform().is_msvc() {
        println!("cargo:rustc-link-arg=/DEF:{}", def_location().display());
    }
}

///
/// Internal function which returns the location of the .def file for giving to the linker
///
fn def_location() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("pkg")
        .join("symbols.def")
}

///
/// Internal function which returns the location of the agility SDK .nupkg file.
///
fn pkg_location() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("pkg")
        .join("microsoft.direct3d.d3d12.1.706.3-preview.nupkg")
}

///
/// Macro that will define the `D3D12SDKVersion` and `D3D12SDKPath` symbols.
///
/// They will be defined with the following values
///
/// - `D3D12SDKVersion` = 706
///     - This is the version identifier of the SDK version bundled with this crate
/// - `D3D12SDKPath` = ".\"
///     - This allows 'D3D12Core.dll' to be next to the app executable and is compatible with the
///       results of the [extract_agility_sdk_binaries] function, which will copy the SDK binaries
///       into the 'target' directory next to the output .exe file.
///
/// To use custom values declare the symbols manually like this:
///
/// ```{skip}
/// #[used]
/// #[no_mangle]
/// #[allow(non_upper_case_globals)]
/// /// Replace 706 with your minimum required SDK version
/// pub static D3D12SDKVersion: u32 = 706;
///
/// #[used]
/// #[no_mangle]
/// #[allow(non_upper_case_globals)]
/// /// Replace b".\\\0" with your directory to search for D3D12Core.dll
/// pub static D3D12SDKPath: &[u8; 3] = b".\\\0";
/// ```
///
/// See the D3D12 Agility SDK documentation for information on what these values mean and can be
/// set to.
///
#[macro_export]
macro_rules! export_standard_agility_sdk_symbols {
    () => {
        #[used]
        #[no_mangle]
        #[allow(non_upper_case_globals)]
        pub static D3D12SDKVersion: u32 = 706;

        #[used]
        #[no_mangle]
        #[allow(non_upper_case_globals)]
        pub static D3D12SDKPath: &[u8; 3] = b".\\\0";
    };
}

///
/// This function will extract `D3D12Core.dll and `D3D12SDKLayers.dll` from a bundled distribution
/// of the Agility SDK .nupkg file. The bundled version will match this crate's 'major' version.
///
/// The files will be copied to two places.
///
/// - `target/{build_type}` (In the same folder as any output .exe files)
/// - `target/{build_type}/artifacts` (All required .dll files linked to are placed here)
///
/// This function will do nothing on any platform that is not windows.
///
/// For custom locations, either copy from the above two output locations or implement this function
/// manually.
///
/// This currently works for the following targets:
///
/// - `x86_64-pc-windows-msvc`
/// - `x86_64-uwp-windows-msvc`
///
pub fn extract_agility_sdk_binaries() {
    if target_platform().is_windows() {
        const CORE_DLL: &str = "build/native/bin/x64/D3D12Core.dll";
        const CORE_NAME: &str = "D3D12Core.dll";
        const LAYERS_DLL: &str = "build/native/bin/x64/d3d12SDKLayers.dll";
        const LAYERS_NAME: &str = "d3d12SDKLayers.dll";

        let out_dir = std::env::var("OUT_DIR").unwrap();
        let out_dir = Path::new(&out_dir);

        let bytes = std::fs::read(pkg_location()).unwrap();
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
