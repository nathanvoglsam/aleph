//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use aleph_vulkan_core::erupt::vk1_0::AccessFlagBits;

///
/// Internal function for asserting that a given access type is valid for a read only image
///
#[inline]
pub fn debug_check_image_read_access_type(access: AccessFlagBits) {
    // Debug check if passing write accesses to a read only description
    debug_assert!(access != AccessFlagBits::MEMORY_WRITE);
    debug_assert!(access != AccessFlagBits::COLOR_ATTACHMENT_WRITE);
    debug_assert!(access != AccessFlagBits::TRANSFER_WRITE);
    debug_assert!(access != AccessFlagBits::DEPTH_STENCIL_ATTACHMENT_WRITE);
    debug_assert!(access != AccessFlagBits::HOST_WRITE);
    debug_assert!(access != AccessFlagBits::SHADER_WRITE);
    debug_assert!(access != AccessFlagBits::TRANSFORM_FEEDBACK_COUNTER_WRITE_EXT);
    debug_assert!(access != AccessFlagBits::TRANSFORM_FEEDBACK_WRITE_EXT);
    debug_assert!(access != AccessFlagBits::ACCELERATION_STRUCTURE_WRITE_KHR);
    debug_assert!(access != AccessFlagBits::ACCELERATION_STRUCTURE_WRITE_NV);
    debug_assert!(access != AccessFlagBits::COMMAND_PREPROCESS_WRITE_NV);
}

///
/// Internal function for asserting that a given access type is valid for an image to be used as
///
#[inline]
pub fn debug_check_image_access_type(access: AccessFlagBits) {
    // Debug check if passing invalid access type in
    debug_assert!(access != AccessFlagBits::INDEX_READ);
    debug_assert!(access != AccessFlagBits::UNIFORM_READ);
    debug_assert!(access != AccessFlagBits::VERTEX_ATTRIBUTE_READ);
    debug_assert!(access != AccessFlagBits::INDIRECT_COMMAND_READ);
    debug_assert!(access != AccessFlagBits::ACCELERATION_STRUCTURE_READ_KHR);
    debug_assert!(access != AccessFlagBits::ACCELERATION_STRUCTURE_READ_NV);
    debug_assert!(access != AccessFlagBits::COMMAND_PREPROCESS_READ_NV);
    debug_assert!(access != AccessFlagBits::TRANSFORM_FEEDBACK_COUNTER_READ_EXT);

    // Debug check if passing unsupported access type in
    debug_assert!(access != AccessFlagBits::FRAGMENT_DENSITY_MAP_READ_EXT);
    debug_assert!(access != AccessFlagBits::SHADING_RATE_IMAGE_READ_NV);
    debug_assert!(access != AccessFlagBits::COLOR_ATTACHMENT_READ_NONCOHERENT_EXT);
}
