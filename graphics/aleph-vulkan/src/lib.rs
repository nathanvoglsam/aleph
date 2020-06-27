//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

pub extern crate spirv_reflect;

// Rename for brevity
extern crate aleph_vulkan_alloc as vulkan_alloc;
extern crate aleph_vulkan_core as vulkan_core;

pub use aleph_embedded_data::gltf;

pub mod alloc;
pub mod core;
pub mod embedded;
pub mod format;
pub mod image;
pub mod pipeline;
pub mod pipeline_cache;
pub mod pipeline_layout;
pub mod reflect;
pub mod shader;
