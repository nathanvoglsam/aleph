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
    AccessFlagBits, AccessFlags, Extent2D, Format, ImageLayout, ImageView, PipelineStageFlagBits,
    PipelineStageFlags,
};

///
///
///
pub struct ImageImport {
    /// The image view for this image resource.
    pub(crate) image_view: ImageView,

    /// The initial layout this image will be in.
    pub(crate) current_layout: ImageLayout,

    /// The format this image is in
    pub(crate) format: Format,

    /// The size of the image
    pub(crate) extent: Extent2D,

    /// The pipeline stages we'll be using the image in
    pub(crate) stages: PipelineStageFlags,

    /// The set of access types this image will be used as
    pub(crate) access_types: AccessFlags,
}

impl ImageImport {
    ///
    /// Construct a new ImageReadDescription for an image with the given `identifier` where the
    /// image will be in the given layout and is of the provided format and extent.
    ///
    /// `current_layout` maps to `oldLayout` in `VkImageMemoryBarrier` for when the render graph
    /// injects a barrier for synchronizing with external usage of the image.
    ///
    /// `format` and `extent` refer to the format and extent of the `ImageView` provided and are
    /// provided for validation if a pass so wishes to do so. They come in handy during the compile
    /// phase when constructing pipelines and render passes.
    ///
    pub fn new(
        image_view: ImageView,
        current_layout: ImageLayout,
        format: Format,
        extent: Extent2D,
    ) -> Self {
        // Assert we aren't importing in ImageLayout::UNDEFINED as this makes no sense. If you're
        // importing an image into the render graph you obviously want to read data from it, but the
        // transition from UNDEFINED->VALID layout makes the content of the image undefined,
        // effectively destroying the image data so we treat it as an error.
        assert_ne!(
            current_layout,
            ImageLayout::UNDEFINED,
            "Importing in layout undefined makes no sense"
        );
        Self {
            image_view,
            current_layout,
            format,
            extent,
            stages: Default::default(),
            access_types: Default::default(),
        }
    }

    ///
    /// Add the given stage to the set of stages the image was used in prior to being imported into
    /// the graph. This maps to `srcStageMask` as used in `vkCmdPipelineBarrier` and
    /// `vkCmdWaitEvents` in what ever barrier is injected by the render graph to synchronize with
    /// the external usage of the image.
    ///
    pub fn in_stage(mut self, stage: PipelineStageFlagBits) -> Self {
        self.stages |= stage.bitmask();
        self
    }

    ///
    /// Add the given access type to the set of accesses that this image was used in prior to being
    /// imported into the graph. This maps to `srcAccessMask` in `VkImageMemoryBarrier` in whatever
    /// barrier is injected by the render graph to synchronize with the external usage of the image.
    ///
    pub fn access_type(mut self, access: AccessFlagBits) -> Self {
        debug_check_image_access_type(access);
        debug_check_image_read_access_type(access);
        self.access_types |= access.bitmask();
        self
    }
}

///
///
///
pub struct ImageExport {
    /// The layout we want the exported image to be in
    pub(crate) layout: ImageLayout,

    /// The pipeline stages we'll be using the image in
    pub(crate) stages: PipelineStageFlags,

    /// The set of access types this image will be used as
    pub(crate) access_types: AccessFlags,
}

impl ImageExport {
    ///
    /// Construct a new ImageReadDescription for an image with the given `identifier` where the
    /// image will be in the given layout.
    ///
    pub fn new(layout: ImageLayout) -> Self {
        // Assert we're not exporting into ImageLayout::UNDEFINED. This logically makes no sense as
        // you would simply be discarding the contents of the image before using them, But even more
        // importantly a layout transition to UNDEFINED is technically a write operation as it
        // destroys the contents of the image (VALID->UNDEFINED->VALID makes image data undefined).
        assert_ne!(
            layout,
            ImageLayout::UNDEFINED,
            "Exporting in layout undefined makes no sense"
        );
        Self {
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
    pub fn access_type(mut self, access: AccessFlagBits) -> Self {
        debug_check_image_access_type(access);
        debug_check_image_read_access_type(access);
        self.access_types |= access.bitmask();
        self
    }
}

///
/// Represents the set of supported resource types that can be imported
///
pub enum ResourceImport {
    Image(ImageImport),
}

///
/// Represents the set of supported resource types that can be exported
///
pub enum ResourceExport {
    Image(ImageExport),
}
