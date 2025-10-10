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

extern crate aleph_target_build as target;

use std::path::{Path, PathBuf};

use target::{Architecture, Platform};

///
/// Where to place build artifacts like .dll or .so files for this build. This will always be inside
/// cargo's `target` directory.
///
pub fn artifacts_dir() -> PathBuf {
    let mut out_dir = cargo_target_dir();
    out_dir.push("artifacts");
    out_dir
}

///
/// The location of the `Cargo.toml` for the current crate
///
pub fn manifest_dir() -> PathBuf {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    Path::new(&manifest_dir).to_path_buf()
}

///
/// An output directory that is specific to this crate inside the `target` directory.
///
pub fn cargo_out_dir() -> PathBuf {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    Path::new(&out_dir).to_path_buf()
}

///
/// The cargo output directory, usually `target/debug` or `target/release`
///
pub fn cargo_target_dir() -> PathBuf {
    let mut out_dir = cargo_out_dir();
    assert!(out_dir.pop());
    assert!(out_dir.pop());
    assert!(out_dir.pop());
    out_dir
}

///
/// Copy a file into the artifacts directory
///
pub fn copy_file_to_artifacts_dir(source: &Path) -> std::io::Result<()> {
    let mut out_artifact = artifacts_dir();

    // Create the artifacts dir if it doesn't already exist
    if !out_artifact.exists() {
        std::fs::create_dir_all(&out_artifact).unwrap();
    }

    // Make the output file name
    out_artifact.push(source.file_name().unwrap());

    // Do the copy
    std::fs::copy(source, &out_artifact)?;

    Ok(())
}

///
/// Copy a file into the target directory
///
pub fn copy_file_to_target_dir(source: &Path) -> std::io::Result<()> {
    let mut out_artifact = cargo_target_dir();

    // Make the output file name
    out_artifact.push(source.file_name().unwrap());

    // Do the copy
    std::fs::copy(source, &out_artifact)?;

    Ok(())
}

///
/// Copy a file into the artifacts directory
///
pub fn copy_file_to_artifacts_dir_with_name(source: &Path, name: &str) -> std::io::Result<()> {
    let mut out_artifact = artifacts_dir();

    // Create the artifacts dir if it doesn't already exist
    if !out_artifact.exists() {
        std::fs::create_dir_all(&out_artifact).unwrap();
    }

    // Make the output file name
    out_artifact.push(name);

    // Do the copy
    std::fs::copy(source, &out_artifact)?;

    Ok(())
}

///
/// Copy a file into the target directory
///
pub fn copy_file_to_target_dir_with_name(source: &Path, name: &str) -> std::io::Result<()> {
    let mut out_artifact = cargo_target_dir();

    // Make the output file name
    out_artifact.push(name);

    // Do the copy
    std::fs::copy(source, &out_artifact)?;

    Ok(())
}

pub fn standard_binary_path_for(platform: Platform, architecture: Architecture) -> Option<PathBuf> {
    assert!(!architecture.is_unknown());

    match platform {
        Platform::WindowsGNU | Platform::WindowsMSVC => {
            let vendor = "win32";
            let abi = if platform.is_gnu() { "gnu" } else { "msvc" };

            let mut out = PathBuf::new();
            out.push(architecture.name());
            out.push(vendor);
            out.push(abi);

            Some(out)
        }
        Platform::Linux => {
            // Note: Shipping binaries for linux sucks. In practice we'll probably never use this
            //       but for the sake of completness we include this anyway.
            let mut out = PathBuf::new();
            out.push(architecture.name());
            out.push("linux");
            Some(out)
        }
        Platform::Android => {
            let mut out = PathBuf::new();
            out.push(architecture.name());
            out.push("android");
            out.push("api-30"); // We assume api-30 for binaries we provide

            Some(out)
        }
        Platform::MacOS => {
            let mut out = PathBuf::new();
            out.push(architecture.name());
            out.push("macos");
            Some(out)
        }
        Platform::IOS => {
            let mut out = PathBuf::new();
            out.push(architecture.name());
            out.push("ios");
            Some(out)
        }
        Platform::Unknown => None,
    }
}
