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
pub extern crate aleph_app_info as app_info;
pub extern crate aleph_imgui as imgui;
pub extern crate aleph_log as log;
pub extern crate aleph_platform as platform;
pub extern crate aleph_render as render;
pub extern crate aleph_vulkan as vulkan;
pub extern crate aleph_vulkan_alloc as vulkan_alloc;
pub extern crate aleph_vulkan_core as vulkan_core;
pub extern crate rayon;

// =================================================================================================
// Modules
// =================================================================================================

mod app_logic;
mod engine;
mod frame_rate;
mod thread_pools;

pub use self::app_logic::AppLogic;
pub use self::engine::Engine;
pub use self::frame_rate::FrameRate;
