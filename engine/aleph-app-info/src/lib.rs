//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use std::ffi::CStr;

static ENGINE_NAME: &'static str = "AlephEngine\0";
const ENGINE_VERSION_STRING: &str = "0.1.0";
const ENGINE_VERSION_MAJOR: u32 = 0;
const ENGINE_VERSION_MINOR: u32 = 1;
const ENGINE_VERSION_PATCH: u32 = 0;

///
/// Returns the engine name string
///
pub fn engine_name() -> &'static str {
    &ENGINE_NAME[0..(ENGINE_NAME.len()-1)]
}

///
/// Returns the engine name string as a CStr
///
pub fn engine_name_cstr() -> &'static CStr {
    unsafe { CStr::from_ptr(ENGINE_NAME.as_ptr() as *const _) }
}

///
/// Returns the engine version string
///
pub fn engine_version_string() -> &'static str {
    ENGINE_VERSION_STRING
}

///
/// Returns the engine major version
///
pub fn engine_version_major() -> u32 {
    ENGINE_VERSION_MAJOR
}

///
/// Returns the engine minor version
///
pub fn engine_version_minor() -> u32 {
    ENGINE_VERSION_MINOR
}

///
/// Returns the engine patch version
///
pub fn engine_version_patch() -> u32 {
    ENGINE_VERSION_PATCH
}

///
/// A struct that holds information about the particular game the engine will be running
///
pub struct AppInfo {
    ///
    /// The name of the App.
    ///
    /// # Info
    ///
    /// This will be the window title
    ///
    pub name: String,

    ///
    /// A string to represent the author of the app
    ///
    pub author: String,

    ///
    /// The major version of the app, following semver conventions
    ///
    pub major: u32,

    ///
    /// The minor version of the app, following semver conventions
    ///
    pub minor: u32,

    ///
    /// The patch version of the app, following semver conventions
    ///
    pub patch: u32,
}

impl AppInfo {
    ///
    /// Get a string that represents the version number of the form {major}.{minor}.{patch}
    ///
    pub fn version_string(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }
}
