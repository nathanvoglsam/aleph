//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use std::path::{Path, PathBuf};

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
    let manifest_dir = Path::new(&manifest_dir).to_path_buf();
    manifest_dir
}

///
/// An output directory that is specific to this crate inside the `target` directory.
///
pub fn cargo_out_dir() -> PathBuf {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir).to_path_buf();
    out_dir
}

///
/// The cargo output directory, usually `target/debug` or `target/release`
///
pub fn cargo_target_dir() -> PathBuf {
    let mut out_dir = cargo_out_dir();
    out_dir.push("..");
    out_dir.push("..");
    out_dir.push("..");
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
