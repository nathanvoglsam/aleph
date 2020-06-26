//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use crate::gpu::vk::core::VendorID;

///
/// Struct that holds information about the physical device (GPU) that a given device represents
///
pub struct GPUInfo {
    pub vendor_id: VendorID,
    pub device_name: String,
    pub api_version_major: u32,
    pub api_version_minor: u32,
    pub api_version_patch: u32,
}
