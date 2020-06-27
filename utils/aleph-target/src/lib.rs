//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

extern crate aleph_target_build as aleph_target;

pub use aleph_target::recreate_triple;

pub use aleph_target::Architecture;
pub use aleph_target::BuildType;
pub use aleph_target::Platform;

pub use aleph_target::get_architecture_from;
pub use aleph_target::get_build_type_from;
pub use aleph_target::get_platform_from;

pub mod build {

    ///
    /// Returns the host platform
    ///
    #[inline]
    pub fn host_platform() -> aleph_target::Platform {
        if cfg!(ALEPH_BUILD_PLATFORM_HOST_is_windows) {
            if cfg!(ALEPH_BUILD_PLATFORM_HOST_is_msvc) {
                aleph_target::Platform::WindowsMSVC
            } else if cfg!(ALEPH_BUILD_PLATFORM_HOST_is_gnu) {
                aleph_target::Platform::WindowsGNU
            } else {
                panic!("Unsupported Platform")
            }
        } else if cfg!(ALEPH_BUILD_PLATFORM_HOST_is_android) {
            aleph_target::Platform::Android
        } else if cfg!(ALEPH_BUILD_PLATFORM_HOST_is_linux) {
            aleph_target::Platform::Linux
        } else {
            panic!("Unsupported Platform")
        }
    }

    ///
    /// Returns the host architecture
    ///
    #[inline]
    pub fn host_architecture() -> aleph_target::Architecture {
        if cfg!(ALEPH_BUILD_ARCH_HOST_is_x86_64) {
            aleph_target::Architecture::X8664
        } else if cfg!(ALEPH_BUILD_ARCH_HOST_is_aarch64) {
            aleph_target::Architecture::AARCH64
        } else {
            panic!("Unsupported Architecture")
        }
    }

    ///
    /// Returns the host build type
    ///
    #[inline]
    pub fn host_build_type() -> aleph_target::BuildType {
        if cfg!(ALEPH_BUILD_PROFILE_HOST_is_release) {
            aleph_target::BuildType::Release
        } else if cfg!(ALEPH_BUILD_PROFILE_HOST_is_debug) {
            aleph_target::BuildType::Debug
        } else {
            panic!("Unsupported Build Profile")
        }
    }

    ///
    /// Returns whether we are compiling for an x86 platform. Useful for guarding use of x86 intrinsics
    ///
    #[cfg(any(HOST_is_x86_64, HOST_is_i686))]
    const fn internal_host_is_x86() -> bool {
        true
    }

    ///
    /// Returns whether we are compiling for an x86 platform. Useful for guarding use of x86 intrinsics
    ///
    #[cfg(not(any(HOST_is_x86_64, HOST_is_i686)))]
    const fn internal_host_is_x86() -> bool {
        false
    }

    ///
    /// Returns whether we are compiling for an x86 platform. Useful for guarding use of x86 intrinsics
    ///
    pub const fn host_is_x86() -> bool {
        internal_host_is_x86()
    }

    ///
    /// Returns the target platform
    ///
    #[inline]
    pub fn target_platform() -> aleph_target::Platform {
        if cfg!(ALEPH_BUILD_PLATFORM_TARGET_is_windows) {
            if cfg!(ALEPH_BUILD_PLATFORM_TARGET_is_msvc) {
                aleph_target::Platform::WindowsMSVC
            } else if cfg!(ALEPH_BUILD_PLATFORM_TARGET_is_gnu) {
                aleph_target::Platform::WindowsGNU
            } else {
                panic!("Unsupported Platform")
            }
        } else if cfg!(ALEPH_BUILD_PLATFORM_TARGET_is_android) {
            aleph_target::Platform::Android
        } else if cfg!(ALEPH_BUILD_PLATFORM_TARGET_is_linux) {
            aleph_target::Platform::Linux
        } else {
            panic!("Unsupported Platform")
        }
    }

    ///
    /// Returns the target architecture
    ///
    #[inline]
    pub fn target_architecture() -> aleph_target::Architecture {
        if cfg!(ALEPH_BUILD_ARCH_TARGET_is_x86_64) {
            aleph_target::Architecture::X8664
        } else if cfg!(ALEPH_BUILD_ARCH_TARGET_is_aarch64) {
            aleph_target::Architecture::AARCH64
        } else {
            panic!("Unsupported Architecture")
        }
    }

    ///
    /// Returns the target build type
    ///
    #[inline]
    pub fn target_build_type() -> aleph_target::BuildType {
        if cfg!(ALEPH_BUILD_PROFILE_TARGET_is_release) {
            aleph_target::BuildType::Release
        } else if cfg!(ALEPH_BUILD_PROFILE_TARGET_is_debug) {
            aleph_target::BuildType::Debug
        } else {
            panic!("Unsupported Build Profile")
        }
    }

    ///
    /// Returns whether we are compiling for an x86 platform. Useful for guarding use of x86 intrinsics
    ///
    #[cfg(any(TARGET_is_x86_64, TARGET_is_i686))]
    const fn internal_target_is_x86() -> bool {
        true
    }

    ///
    /// Returns whether we are compiling for an x86 platform. Useful for guarding use of x86 intrinsics
    ///
    #[cfg(not(any(TARGET_is_x86_64, TARGET_is_i686)))]
    const fn internal_target_is_x86() -> bool {
        false
    }

    ///
    /// Returns whether we are compiling for an x86 platform. Useful for guarding use of x86 intrinsics
    ///
    pub const fn target_is_x86() -> bool {
        internal_target_is_x86()
    }
}
