//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

///
/// Enumeration of all supported architectures
///
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Architecture {
    X8664,
    AARCH64,
}

impl Architecture {
    pub fn print_host_cargo_cfg(&self) {
        match self {
            Architecture::X8664 => {
                println!("cargo:rustc-cfg=ALEPH_BUILD_ARCH_HOST_is_x86_64");
            }
            Architecture::AARCH64 => {
                println!("cargo:rustc-cfg=ALEPH_BUILD_ARCH_HOST_is_aarch64");
            }
        }
    }

    pub fn print_target_cargo_cfg(&self) {
        match self {
            Architecture::X8664 => {
                println!("cargo:rustc-cfg=ALEPH_BUILD_ARCH_TARGET_is_x86_64");
            }
            Architecture::AARCH64 => {
                println!("cargo:rustc-cfg=ALEPH_BUILD_ARCH_TARGET_is_aarch64");
            }
        }
    }

    #[inline]
    pub fn name(&self) -> &'static str {
        match self {
            Architecture::X8664 => "x86_64",
            Architecture::AARCH64 => "aarch64",
        }
    }

    #[inline]
    pub fn ndk_name(&self) -> &'static str {
        match self {
            Architecture::X8664 => "x86_64",
            Architecture::AARCH64 => "arm64-v8a",
        }
    }

    ///
    /// Are we building for x86-64
    ///
    #[inline]
    pub fn is_x86_64(&self) -> bool {
        *self == Architecture::X8664
    }

    ///
    /// Are we building for aarch64 (ARM 64bit)
    ///
    #[inline]
    pub fn is_aarch64(&self) -> bool {
        *self == Architecture::AARCH64
    }
}

#[inline]
pub fn get_architecture_from(triple: &str) -> Architecture {
    let target = triple;

    if target.contains("x86_64") {
        Architecture::X8664
    } else if target.contains("aarch64") {
        Architecture::AARCH64
    } else {
        panic!("Unsupported Architecture");
    }
}
