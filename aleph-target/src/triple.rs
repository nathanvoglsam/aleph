//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
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
