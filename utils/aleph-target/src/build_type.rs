//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

///
/// Enumeration of all supported build types
///
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum BuildType {
    Release,
    Debug,
}

impl BuildType {
    pub fn print_host_cargo_cfg(self) {
        match self {
            BuildType::Release => {
                println!("cargo:rustc-cfg=ALEPH_BUILD_PROFILE_HOST_is_release");
            }
            BuildType::Debug => {
                println!("cargo:rustc-cfg=ALEPH_BUILD_PROFILE_HOST_is_debug");
            }
        }
    }

    pub fn print_target_cargo_cfg(self) {
        match self {
            BuildType::Release => {
                println!("cargo:rustc-cfg=ALEPH_BUILD_PROFILE_TARGET_is_release");
            }
            BuildType::Debug => {
                println!("cargo:rustc-cfg=ALEPH_BUILD_PROFILE_TARGET_is_debug");
            }
        }
    }

    ///
    /// Get the build type name
    ///
    #[inline]
    pub fn name(self) -> &'static str {
        match self {
            BuildType::Release => "release",
            BuildType::Debug => "debug",
        }
    }

    #[inline]
    pub fn pretty_name(self) -> &'static str {
        match self {
            BuildType::Release => "Release",
            BuildType::Debug => "Debug",
        }
    }

    #[inline]
    pub fn is_release(self) -> bool {
        self == BuildType::Release
    }

    #[inline]
    pub fn is_debug(self) -> bool {
        self == BuildType::Debug
    }
}

#[inline]
pub fn get_build_type_from(profile: &str) -> BuildType {
    if profile == "release" {
        BuildType::Release
    } else if profile == "debug" {
        BuildType::Debug
    } else {
        panic!("Unsupported Build Profile")
    }
}
