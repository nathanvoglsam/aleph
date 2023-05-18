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

use aleph_target_build::build::{target_architecture, target_platform};
use aleph_target_build::Architecture;
use std::path::{Path, PathBuf};

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
/// - `x86_64-pc-windows-gnu`
/// - `x86_64-uwp-windows-gnu`
/// - `i686-pc-windows-gnu`
/// - `i686-uwp-windows-gnu`
///
/// This function will do nothing on non windows platforms.
///
pub fn link_agility_symbol_def() {
    // Copy the file into a temp directory for the building exe crate so we don't lock on the
    // symbol file if multiple crates are trying to use it simultaneously.
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir);
    let def_file = out_dir.join("symbols.def");
    std::fs::copy(def_location(), &def_file).unwrap();

    // Add the linker flag
    if target_platform().is_windows() {
        if target_platform().is_msvc() {
            println!("cargo:rustc-link-arg=/DEF:{}", def_file.display());
        } else {
            println!("cargo:rustc-link-arg={}", def_file.display());
        }
    }
}

///
/// Internal function which returns the location of the .def file for giving to the linker
///
fn def_location() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("symbols.def")
}

///
/// Internal function which returns the location of the agility SDK .nupkg file.
///
fn pkg_location() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("thirdparty")
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
        let arch = match target_architecture() {
            Architecture::X8664 => "x64",
            Architecture::AARCH64 => "arm64",
            Architecture::Unknown => panic!("Unknown architecture"),
        };

        let bin_dir = pkg_location().join("bin").join(arch);

        let core_dll = bin_dir.join("D3D12Core.dll");
        aleph_compile::copy_file_to_artifacts_dir(&core_dll).unwrap();
        aleph_compile::copy_file_to_target_dir(&core_dll).unwrap();

        let layers_dll = bin_dir.join("d3d12SDKLayers.dll");
        aleph_compile::copy_file_to_artifacts_dir(&layers_dll).unwrap();
        aleph_compile::copy_file_to_target_dir(&layers_dll).unwrap();
    }
}
