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
    &ENGINE_NAME[0..(ENGINE_NAME.len() - 1)]
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
#[derive(Clone, Debug)]
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

impl Default for AppInfo {
    fn default() -> Self {
        Self {
            name: "Default AlephEngine Game".to_string(),
            author: "AlephEngine".to_string(),
            major: 0,
            minor: 1,
            patch: 0,
        }
    }
}

impl AppInfo {
    ///
    /// Get a string that represents the version number of the form {major}.{minor}.{patch}
    ///
    pub fn version_string(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }
}
