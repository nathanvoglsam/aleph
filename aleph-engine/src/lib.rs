//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

// =================================================================================================
// Crate Imports
// =================================================================================================

// Re-export useful crates
pub extern crate log;
pub extern crate rayon;

extern crate num_cpus;
extern crate once_cell;
extern crate parking_lot;

// When on desktop we use env_logger
#[cfg(not(target_os = "android"))]
extern crate env_logger;

// When on android we use an android specific logger so things show up in android studio
#[cfg(target_os = "android")]
extern crate android_logger;

// =================================================================================================
// Public Modules
// =================================================================================================

pub mod app;

// =================================================================================================
// Internal Modules
// =================================================================================================

mod cpuid;
mod logger;
