//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

//!
//! # Nova-Build
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
mod build_type;
mod platform;
mod triple;

pub mod build;

pub use triple::recreate_triple;

pub use platform::get_platform_from;
pub use platform::Platform;

pub use architecture::get_architecture_from;
pub use architecture::Architecture;

pub use build_type::get_build_type_from;
pub use build_type::BuildType;
