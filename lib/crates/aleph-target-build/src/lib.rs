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

//!
//! # Aleph-Build
//!
//! This crate provides a unified and simplified API for detecting the host and target platform and
//! build profile. It provides a structure for representing a target or host platform and some
//! additional utilities for checking the host and target platform and build profile.
//!
//! ## Build Script vs Library Usage
//!
//! The crate has 3 basic parts, the enums and types used to represent the platform, the functions
//! for detecting the platform in a build script and the functions for detecting the platform in an
//! end user crate.
//!
//! ### Build Script
//!
//! Rust build.rs scripts provide the current HOST (the platform we are compiling ON) and TARGET
//! (the platform we are compiling FOR) through environment variables. The build script side of this
//! crate works by querying the environment variables to produce a Platform, Architecture or
//! BuildType.
//!
//! There are also an associated functions with each of those 3 types `print_cargo_cfg` that will
//! add some build cfg flags that will be used by code in the library portion (not in the build.rs)
//! of the crate to provide the same interface to all the information in the build script.
//!
//! ### Library Usage
//!
//! To correctly use this crate inside your own crate, you must provide a build script that calls
//!

#![allow(clippy::suspicious_else_formatting)]
#![deny(bare_trait_objects)]

mod architecture;
mod build_config;
mod build_type;
mod platform;
mod triple;

pub mod build {
    use crate::{
        get_architecture_from, get_platform_from, Architecture, BuildConfig, BuildType, Platform,
    };
    use std::env;

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
    /// Returns the target build profile
    ///
    /// # Warning
    ///
    /// Only works in a build script
    ///
    #[inline]
    pub fn target_build_type() -> BuildType {
        match env::var("ALEPH_BUILD_TYPE")
            .unwrap_or("Development".to_string())
            .as_str()
        {
            "Development" => BuildType::Development,
            "Retail" => BuildType::Retail,
            _ => BuildType::Development,
        }
    }

    ///
    /// Returns the target build config
    ///
    /// # Warning
    ///
    /// Only works in a build script
    ///
    pub fn target_build_config() -> BuildConfig {
        BuildConfig {
            debug: env::var("DEBUG").unwrap() == "true",
            optimized: env::var("OPT_LEVEL").unwrap() != "0",
        }
    }
}

pub use architecture::get_architecture_from;
pub use architecture::Architecture;
pub use build_config::BuildConfig;
pub use build_type::BuildType;
pub use platform::get_platform_from;
pub use platform::Platform;
pub use triple::recreate_triple;
