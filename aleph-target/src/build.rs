//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use crate::{
    get_architecture_from, get_build_type_from, get_platform_from, Architecture, BuildType,
    Platform,
};
use std::env;

///
/// Returns the host architecture
///
/// # Warning
///
/// Only works in a build script
///
#[inline]
pub fn host_architecture() -> Architecture {
    get_architecture_from(&env::var("HOST").unwrap())
}

///
/// Returns the target architecture
///
/// # Warning
///
/// Only works in a build script
///
#[inline]
pub fn target_architecture() -> Architecture {
    get_architecture_from(&env::var("TARGET").unwrap())
}

///
/// Returns the host build profile
///
/// # Warning
///
/// Only works in a build script
///
#[inline]
pub fn host_build_type() -> BuildType {
    target_build_type()
}

///
/// Returns the target build profile
///
/// # Warning
///
/// Only works in a build script
///
#[inline]
pub fn target_build_type() -> BuildType {
    get_build_type_from(&env::var("PROFILE").unwrap())
}

///
/// Returns the host platform (operating system)
///
/// # Warning
///
/// Only works in a build script
///
#[inline]
pub fn host_platform() -> Platform {
    get_platform_from(&env::var("HOST").unwrap())
}

///
/// Returns the target platform (operating system)
///
/// # Warning
///
/// Only works in a build script
///
#[inline]
pub fn target_platform() -> Platform {
    get_platform_from(&env::var("TARGET").unwrap())
}
