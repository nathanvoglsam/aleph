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
pub extern crate imgui;
pub extern crate log;
pub extern crate rayon;
//pub extern crate aleph_math as math;

extern crate aleph_target_crate as target;
extern crate aleph_vma_sys as vma_sys;
extern crate console;
extern crate gltf;
extern crate num_cpus;
extern crate once_cell;
extern crate palette;
extern crate parking_lot;
extern crate raw_cpuid;
extern crate raw_window_handle;
extern crate spirv_reflect;

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
pub mod gpu;

// =================================================================================================
// Internal Modules
// =================================================================================================

pub(crate) mod cpuid;
pub(crate) mod logger;
mod macros;
