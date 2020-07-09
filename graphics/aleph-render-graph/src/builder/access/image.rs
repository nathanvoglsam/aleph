//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use crate::utils::{debug_check_image_access_type, debug_check_image_read_access_type};
use aleph_vulkan_core::erupt::vk1_0::{
    AccessFlagBits, AccessFlags, ImageLayout, PipelineStageFlagBits, PipelineStageFlags,
};

///
/// Describes a write to an image resource with the given `identifier` and with the image in the
/// given layout. There are further functions that must be called on the ImageWriteDescription in
/// order to specify the pipeline stages the image will be used in as well as the access flags to
/// state how it will be used.
///
pub struct ImageWriteDescription {
    /// The identifier for the image we're writing
    pub(crate) identifier: String,

    /// The layout we want the image to be in when writing to it
    pub(crate) layout: ImageLayout,

    /// The pipeline stages we'll be writing to the image
    pub(crate) stages: PipelineStageFlags,

    /// The types of access we will be writing to the image in
    pub(crate) access_types: AccessFlags,
}

impl ImageWriteDescription {
    ///
    /// Construct a new ImageWriteDescription for an image with the given `identifier` where the
    /// image will be in the given layout.
    ///
    pub fn new(identifier: &str, layout: ImageLayout) -> Self {
        assert_ne!(
            layout,
            ImageLayout::UNDEFINED,
            "Writing in layout undefined makes no sense"
        );
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
    pub fn in_stage(mut self, stage: PipelineStageFlagBits) -> Self {
        self.stages |= stage.bitmask();
        self
    }

    ///
    /// Add the given access type to the set of access types this image will be used
    ///
    /// # Panics
    ///
    /// `debug_assert!()` is used to check if nonsensical access flags are provided
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
    /// The identifier for the image we're reading
    pub(crate) identifier: String,

    /// The layout we want to read the image in
    pub(crate) layout: ImageLayout,

    /// The pipeline stages we'll be reading the image in
    pub(crate) stages: PipelineStageFlags,

    /// The set of access types this image will be read as
    pub(crate) access_types: AccessFlags,
}

impl ImageReadDescription {
    ///
    /// Construct a new ImageReadDescription for an image with the given `identifier` where the
    /// image will be in the given layout.
    ///
    pub fn new(identifier: &str, layout: ImageLayout) -> Self {
        assert_ne!(
            layout,
            ImageLayout::UNDEFINED,
            "Reading in layout undefined makes no sense"
        );
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
    pub fn in_stage(mut self, stage: PipelineStageFlagBits) -> Self {
        self.stages |= stage.bitmask();
        self
    }

    ///
    /// Add the given access type to the set of access types this image will be used
    ///
    /// # Panics
    ///
    /// `debug_assert!()` is used to check if nonsensical access flags are provided
    ///
    pub fn access_type(mut self, access: AccessFlagBits) -> Self {
        debug_check_image_access_type(access);
        debug_check_image_read_access_type(access);
        self.access_types |= access.bitmask();
        self
    }
}
