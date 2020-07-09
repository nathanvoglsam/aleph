//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use aleph_vulkan_core::erupt::vk1_0::{
    AccessFlagBits, AccessFlags, ImageLayout, PipelineStageFlags,
};

///
/// Internal function for asserting that a given access type is valid for a read only image
///
#[inline]
fn debug_check_image_read_access_type(access: AccessFlagBits) {
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
fn debug_check_image_access_type(access: AccessFlagBits) {
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

///
/// Describes a write to an image resource with the given `identifier` and with the image in the
/// given layout. There are further functions that must be called on the ImageWriteDescription in
/// order to specify the pipeline stages the image will be used in as well as the access flags to
/// state how it will be used.
///
pub struct ImageWriteDescription {
    pub(crate) identifier: String,
    pub(crate) layout: ImageLayout,
    pub(crate) stages: PipelineStageFlags,
    pub(crate) access_types: AccessFlags,
}

impl ImageWriteDescription {
    ///
    /// Construct a new ImageWriteDescription for an image with the given `identifier` where the
    /// image will be in the given layout.
    ///
    pub fn new(identifier: &str, layout: ImageLayout) -> Self {
        Self {
            identifier: identifier.to_string(),
            layout,
            stages: Default::default(),
            access_types: Default::default(),
        }
    }

    ///
    /// Add the given stage to the set of stages the image will be used in
    ///
    pub fn in_stage(mut self, stage: PipelineStageFlags) -> Self {
        self.stages |= stage;
        self
    }

    ///
    /// Add the given access type to the
    ///
    pub fn access_type(mut self, access: AccessFlagBits) -> Self {
        debug_check_image_access_type(access);
        self.access_types |= access.bitmask();
        self
    }
}

///
/// Describes a read to an image resource with the given `identifier` and with the image in the
/// given layout. There are further functions that must be called on the ImageReadDescription in
/// order to specify the pipeline stages the image will be used in as well as the access flags to
/// state how it will be used.
///
pub struct ImageReadDescription {
    pub(crate) identifier: String,
    pub(crate) layout: ImageLayout,
    pub(crate) stages: PipelineStageFlags,
    pub(crate) access_types: AccessFlags,
}

impl ImageReadDescription {
    ///
    /// Construct a new ImageReadDescription for an image with the given `identifier` where the
    /// image will be in the given layout.
    ///
    pub fn new(identifier: &str, layout: ImageLayout) -> Self {
        Self {
            identifier: identifier.to_string(),
            layout,
            stages: Default::default(),
            access_types: Default::default(),
        }
    }

    ///
    /// Add the given stage to the set of stages the image will be used in
    ///
    pub fn in_stage(mut self, stage: PipelineStageFlags) -> Self {
        self.stages |= stage;
        self
    }

    ///
    /// Add the given access type to the
    ///
    pub fn access_type(mut self, access: AccessFlagBits) -> Self {
        debug_check_image_access_type(access);
        debug_check_image_read_access_type(access);
        self.access_types |= access.bitmask();
        self
    }
}

///
/// Struct passed into `register_access` for describing the resources accessed by the pass
///
pub struct ResourceAccess {
    pub(crate) image_reads: Vec<ImageReadDescription>,
    pub(crate) image_writes: Vec<ImageWriteDescription>,
}

impl ResourceAccess {
    ///
    ///
    /// Register that the given image resource will be read in this pass
    ///
    pub fn read_image(&mut self, read: ImageReadDescription) {
        self.image_reads.push(read);
    }

    ///
    ///
    /// Register that the given image resource will be written in this pass
    ///
    pub fn write_image(&mut self, write: ImageWriteDescription) {
        self.image_writes.push(write);
    }
}
