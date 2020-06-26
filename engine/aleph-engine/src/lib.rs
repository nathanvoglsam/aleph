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
pub extern crate aleph_vulkan as gpu;
pub extern crate aleph_app_info as app_info;

extern crate aleph_target_crate as target;
extern crate aleph_logger as logger;
extern crate aleph_cpu_info as cpu_info;
extern crate once_cell;
extern crate palette;
extern crate parking_lot;
extern crate clap;


// =================================================================================================
// Public Modules
// =================================================================================================

pub mod app;
